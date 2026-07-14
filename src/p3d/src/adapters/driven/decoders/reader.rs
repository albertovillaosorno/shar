// File:
//   - reader.rs
// Path:
//   - src/p3d/src/adapters/driven/decoders/reader.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - The p3d adapter boundary for adapters driven decoders reader.
// - Must-Not:
//   - Change domain semantics or bypass application and port contracts.
// - Allows:
//   - Filesystem, JSON, CLI, Blender, or serialization work behind explicit
//   - ports.
// - Split-When:
//   - Split when reader contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same adapters boundary with no distinct
//   - invariant.
// - Summary:
//   - Shared byte-reader primitives for the per-kind Pure3D chunk decoders.
// - Description:
//   - Defines reader data and behavior for p3d adapters driven decoders.
// - Usage:
//   - Constructed by composition roots or tests and passed behind port traits.
// - Defaults:
//   - Adapter defaults stay local to the adapter constructor or config.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Shared byte-reader primitives for the per-kind Pure3D chunk decoders.
//!
//! The on-disk layout is little-endian. Every chunk begins with a 12-byte
//! header: `id: u32`, `header_size: u32`, `total_size: u32`. A chunk's own
//! fields live in the header region `[offset + 12, offset + header_size)`;
//! its child chunks fill `[offset + header_size, offset + total_size)`. Leaf
//! data chunks (the geometry lists) keep their `count + array` payload inside
//! their own header region and have no children.
//!
//! Pure3D length-prefixed strings ("PString") are a single length byte
//! followed by exactly that many bytes, where the stored length already
//! includes any trailing null padding. There is no additional 4-byte
//! alignment after the string — verified by round-tripping vertex/index counts.
// The reader mirrors verified on-disk offsets and cursor movement, so the
// temporary arithmetic and indexing expectations stay local to this parser.
#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::as_conversions,
    clippy::must_use_candidate,
    clippy::missing_const_for_fn,
    clippy::doc_markdown,
    reason = "P3D binary parser code mirrors fixed on-disk offsets and \
              generated chunk taxonomy; follow-up tranche work replaces stubs \
              with typed decoders."
)]
#[derive(Debug)]
/// A forward cursor over a chunk's bytes.
pub struct Reader<'a> {
    /// Buf.
    buf: &'a [u8],
    /// Pos.
    pos: usize,
}

impl<'a> Reader<'a> {
    /// Start reading `buf` at `pos`.
    pub fn new(
        buf: &'a [u8],
        pos: usize,
    ) -> Self {
        Self {
            buf,
            pos,
        }
    }

    /// Current absolute offset within the backing slice.
    pub fn pos(&self) -> usize {
        self.pos
    }

    /// Read a little-endian `u32` and advance four bytes.
    pub fn u32(&mut self) -> Option<u32> {
        let end = self
            .pos
            .checked_add(4)?;
        let slice = self
            .buf
            .get(self.pos..end)?;
        self.pos = end;
        Some(
            u32::from_le_bytes(
                [
                    slice[0], slice[1], slice[2], slice[3],
                ],
            ),
        )
    }

    /// Read a little-endian `f32` and advance four bytes.
    pub fn f32(&mut self) -> Option<f32> {
        let end = self
            .pos
            .checked_add(4)?;
        let slice = self
            .buf
            .get(self.pos..end)?;
        self.pos = end;
        Some(
            f32::from_le_bytes(
                [
                    slice[0], slice[1], slice[2], slice[3],
                ],
            ),
        )
    }

    /// Read one raw byte and advance the cursor.
    pub fn byte(&mut self) -> Option<u8> {
        let value = *self
            .buf
            .get(self.pos)?;
        self.pos = self
            .pos
            .checked_add(1)?;
        Some(value)
    }

    /// Move the cursor forward after an external fixed-width read.
    pub fn skip(
        &mut self,
        count: usize,
    ) -> Option<()> {
        let end = self
            .pos
            .checked_add(count)?;
        if end
            > self
                .buf
                .len()
        {
            return None;
        }
        self.pos = end;
        Some(())
    }

    /// Read a Pure3D PString (length byte + `length` bytes) and advance.
    /// Declared trailing null data is preserved in the returned value.
    pub fn pstring(&mut self) -> Option<String> {
        let length = usize::from(
            *self
                .buf
                .get(self.pos)?,
        );
        let start = self.pos + 1;
        let raw = self
            .buf
            .get(start..start + length)?;
        let value = std::str::from_utf8(raw).ok()?;
        self.pos = start + length;
        Some(value.to_owned())
    }
}

#[derive(Debug, Clone, Copy)]
/// A single child chunk located inside a parent's child region.
pub struct SubChunk {
    /// Id.
    pub id: u32,
    /// Absolute offset of this chunk's 12-byte header.
    pub offset: usize,
    /// Header size.
    pub header_size: usize,
    /// Total size.
    pub total_size: usize,
}

impl SubChunk {
    /// Absolute offset where this chunk's own field/data region begins
    /// (immediately after the 12-byte header).
    pub fn data_offset(&self) -> usize {
        self.offset + 12
    }

    /// Absolute end of this chunk's header region.
    pub fn header_end(&self) -> usize {
        self.offset + self.header_size
    }

    /// Absolute end of this chunk including its children.
    pub fn end(&self) -> usize {
        self.offset + self.total_size
    }
}

/// Enumerate the child chunks in `[start, end)`, returning `None` if any
/// header is malformed so callers fail closed rather than emit partial data.
pub fn subchunks(
    buf: &[u8],
    start: usize,
    end: usize,
) -> Option<Vec<SubChunk>> {
    if start > end || end > buf.len() {
        return None;
    }
    let mut chunks = Vec::new();
    let mut cursor = start;
    while cursor + 12 <= end {
        let id = read_u32(
            buf, cursor,
        )?;
        let header_size = read_u32(
            buf,
            cursor + 4,
        )? as usize;
        let total_size = read_u32(
            buf,
            cursor + 8,
        )? as usize;
        if header_size < 12
            || total_size < header_size
            || cursor + total_size > end
        {
            return None;
        }
        chunks.push(
            SubChunk {
                id,
                offset: cursor,
                header_size,
                total_size,
            },
        );
        cursor += total_size;
    }
    (cursor == end).then_some(chunks)
}

/// Read a little-endian `u32` at an absolute offset without a cursor.
pub fn read_u32(
    buf: &[u8],
    offset: usize,
) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let slice = buf.get(offset..end)?;
    Some(
        u32::from_le_bytes(
            [
                slice[0], slice[1], slice[2], slice[3],
            ],
        ),
    )
}

/// Read an instance header that can appear in extended or legacy form.
pub fn read_instances_header(
    chunk: &[u8],
    child: &SubChunk,
) -> Option<(
    u32,
    u32,
    String,
)> {
    let mut extended = Reader::new(
        chunk,
        child.data_offset(),
    );
    let extended_header = (|| {
        let version = extended.u32()?;
        let flags = extended.u32()?;
        let name = extended.pstring()?;
        (extended.pos() == child.header_end()).then_some(
            (
                version, flags, name,
            ),
        )
    })();
    if let Some(header) = extended_header {
        return Some(header);
    }
    let mut legacy = Reader::new(
        chunk,
        child.data_offset(),
    );
    let legacy_name = legacy.pstring()?;
    if legacy.pos() == child.header_end() {
        return Some(
            (
                0,
                0,
                legacy_name,
            ),
        );
    }
    None
}
