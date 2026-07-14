// File:
//   - format.rs
// Path:
//   - src/rmv/src/domain/format.rs
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
//   - Pure rmv domain rules for domain format.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when format contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Lightweight movie container identification from magic bytes.
// - Description:
//   - Defines format data and behavior for rmv domain.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Lightweight movie container identification from magic bytes.
//!
//! This boundary keeps lightweight movie container identification from magic
//! bytes explicit and returns deterministic results to rmv callers.
/// Difference between the stored Bink size field and complete file length.
const BINK_FILE_SIZE_BIAS: u64 = 8;
/// Maximum accepted Bink frame count.
const BINK_MAX_FRAMES: u32 = 1_000_000;
/// Maximum supported Bink frame width.
const BINK_MAX_WIDTH: u32 = 7_680;
/// Maximum supported Bink frame height.
const BINK_MAX_HEIGHT: u32 = 4_800;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// Moviekind.
pub enum MovieKind {
    /// Item.
    BinkV1,
    /// Item.
    BinkV2,
    /// Item.
    OggNamedRmv,
    /// Item.
    XboxXmvLike,
    /// Item.
    RadicalMovieHeader,
    /// Item.
    Unknown,
}

impl MovieKind {
    /// Number of bytes required to validate every supported Bink header field.
    pub const HEADER_PROBE_LEN: usize = 36;

    #[must_use]
    /// Classifies complete movie bytes and validates available container size
    /// fields.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let Ok(actual_size) = u64::try_from(bytes.len()) else {
            return Self::Unknown;
        };
        Self::from_sized_header(
            bytes,
            actual_size,
        )
    }

    #[must_use]
    /// Classifies a movie header against the complete file length.
    pub fn from_sized_header(
        header: &[u8],
        actual_size: u64,
    ) -> Self {
        let kind = Self::from_prefix(header);
        if kind != Self::BinkV1 && kind != Self::BinkV2 {
            return kind;
        }
        if !bink_header_is_valid(
            header,
            actual_size,
        ) {
            return Self::Unknown;
        }
        kind
    }

    #[must_use]
    /// From prefix.
    pub fn from_prefix(prefix: &[u8]) -> Self {
        let bink = bink_kind(prefix);
        if bink != Self::Unknown {
            bink
        } else if prefix.starts_with(b"OggS") {
            Self::OggNamedRmv
        } else if prefix.get(12..16) == Some(b"xobX") {
            Self::XboxXmvLike
        } else if prefix.starts_with(b"rmv") {
            Self::RadicalMovieHeader
        } else {
            Self::Unknown
        }
    }

    #[must_use]
    /// Label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::BinkV1 => "bink-v1",
            Self::BinkV2 => "bink-v2",
            Self::OggNamedRmv => "ogg-named-rmv",
            Self::XboxXmvLike => "xbox-xmv-like",
            Self::RadicalMovieHeader => "radical-movie-header",
            Self::Unknown => "unknown",
        }
    }
}

/// Returns the supported Bink kind encoded by a four-byte signature.
fn bink_kind(prefix: &[u8]) -> MovieKind {
    let Some(signature_bytes) = prefix.get(..4) else {
        return MovieKind::Unknown;
    };
    let Ok(signature) = <[u8; 4]>::try_from(signature_bytes) else {
        return MovieKind::Unknown;
    };
    match signature {
        [
            b'B',
            b'I',
            b'K',
            b'b' | b'f' | b'g' | b'h' | b'i' | b'k',
        ] => MovieKind::BinkV1,
        [
            b'K',
            b'B',
            b'2',
            b'a' | b'd' | b'f' | b'g' | b'h' | b'i' | b'j' | b'k',
        ] => MovieKind::BinkV2,
        _ => MovieKind::Unknown,
    }
}

/// Verifies mandatory Bink header fields shared by supported revisions.
fn bink_header_is_valid(
    header: &[u8],
    actual_size: u64,
) -> bool {
    if header.len() < MovieKind::HEADER_PROBE_LEN {
        return false;
    }
    let Some(declared_size) = read_header_u32(
        header, 4,
    ) else {
        return false;
    };
    let Some(frame_count) = read_header_u32(
        header, 8,
    ) else {
        return false;
    };
    let Some(largest_frame_size) = read_header_u32(
        header, 12,
    ) else {
        return false;
    };
    let Some(width) = read_header_u32(
        header, 20,
    ) else {
        return false;
    };
    let Some(height) = read_header_u32(
        header, 24,
    ) else {
        return false;
    };
    let Some(frame_rate_numerator) = read_header_u32(
        header, 28,
    ) else {
        return false;
    };
    let Some(frame_rate_denominator) = read_header_u32(
        header, 32,
    ) else {
        return false;
    };
    let declared_size_matches = u64::from(declared_size)
        .checked_add(BINK_FILE_SIZE_BIAS)
        == Some(actual_size);
    declared_size_matches
        && frame_count > 0
        && frame_count <= BINK_MAX_FRAMES
        && u64::from(largest_frame_size) <= actual_size
        && width > 0
        && width <= BINK_MAX_WIDTH
        && height > 0
        && height <= BINK_MAX_HEIGHT
        && frame_rate_numerator > 0
        && frame_rate_denominator > 0
}

/// Reads one little-endian 32-bit Bink header field.
fn read_header_u32(
    bytes: &[u8],
    offset: usize,
) -> Option<u32> {
    let end = offset.checked_add(4)?;
    let field = bytes.get(offset..end)?;
    let field_bytes = <[u8; 4]>::try_from(field).ok()?;
    let value = u32::from_le_bytes(field_bytes);
    Some(value)
}

#[cfg(test)]
#[path = "format_tests.rs"]
mod tests;
