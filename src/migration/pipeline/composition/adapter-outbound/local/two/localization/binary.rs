// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Binary outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Binary outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Binary outbound adapter.

use super::{Error, Outcome};

/// Cursor constrained to one declared chunk range.
pub(super) struct ByteCursor<'a> {
    /// Complete immutable source bytes shared by the bounded cursor.
    bytes: &'a [u8],
    /// Next source byte owned by this cursor.
    position: usize,
    /// Exclusive end of the declared chunk range.
    end: usize,
}

impl<'a> ByteCursor<'a> {
    /// Create a cursor whose complete range already fits the source bytes.
    pub(super) fn new(
        bytes: &'a [u8],
        start: usize,
        end: usize,
    ) -> Outcome<Self> {
        if start > end || bytes.get(start..end).is_none() {
            return Err(Error::invalid(
                "localization chunk range is out of bounds",
            ));
        }
        Ok(Self {
            bytes,
            position: start,
            end,
        })
    }

    /// Read one byte and advance the bounded cursor.
    pub(super) fn read_u8(&mut self) -> Outcome<u8> {
        self.read_bytes(1)?.first().copied().ok_or_else(|| {
            Error::invalid("unexpected end of localization byte")
        })
    }

    /// Read one little-endian scalar and advance the bounded cursor.
    pub(super) fn read_u32(&mut self) -> Outcome<u32> {
        let raw = self.read_bytes(4)?;
        let array: [u8; 4] = raw.try_into().map_err(|error| {
            Error::invalid(format!("invalid u32 width: {error}"))
        })?;
        Ok(u32::from_le_bytes(array))
    }

    /// Read an exact byte range and advance the bounded cursor.
    pub(super) fn read_bytes(&mut self, length: usize) -> Outcome<&'a [u8]> {
        let end = self
            .position
            .checked_add(length)
            .ok_or_else(|| Error::invalid("localization range overflowed"))?;
        if end > self.end {
            return Err(Error::invalid("localization range exceeds its chunk"));
        }
        let value = self.bytes.get(self.position..end).ok_or_else(|| {
            Error::invalid("localization range is out of bounds")
        })?;
        self.position = end;
        Ok(value)
    }

    /// Read one padded length-prefixed UTF-8 string.
    pub(super) fn read_pstring(&mut self) -> Outcome<String> {
        let length = usize::from(self.read_u8()?);
        let raw = self.read_bytes(length)?;
        let content_len =
            raw.iter().position(|byte| *byte == 0).unwrap_or(raw.len());
        let padding = raw.get(content_len..).ok_or_else(|| {
            Error::invalid("PString padding is out of bounds")
        })?;
        if padding.iter().any(|byte| *byte != 0) {
            return Err(Error::invalid(
                "PString contains nonzero data after null padding",
            ));
        }
        let content = raw.get(..content_len).ok_or_else(|| {
            Error::invalid("PString content is out of bounds")
        })?;
        std::str::from_utf8(content)
            .map(str::to_owned)
            .map_err(|error| {
                Error::invalid(format!("PString is not valid UTF-8: {error}"))
            })
    }
}

/// Decode one aligned, zero-terminated UTF-16LE string from a shared buffer.
pub(super) fn read_utf16z(buffer: &[u8], offset: usize) -> Outcome<String> {
    if !buffer.len().is_multiple_of(2) {
        return Err(Error::invalid(
            "language string buffer is not UTF-16 aligned",
        ));
    }
    if !offset.is_multiple_of(2) {
        return Err(Error::invalid(format!(
            "language string offset {offset} is not UTF-16 aligned"
        )));
    }
    let payload = buffer.get(offset..).ok_or_else(|| {
        Error::invalid(format!(
            "language string offset {offset} is out of bounds"
        ))
    })?;
    let mut units = Vec::new();
    let mut terminated = false;
    for pair in payload.chunks(2) {
        let [low, high] = pair else {
            return Err(Error::invalid(
                "language string has an incomplete UTF-16 code unit",
            ));
        };
        let value = u16::from_le_bytes([*low, *high]);
        if value == 0 {
            terminated = true;
            break;
        }
        units.push(value);
    }
    if !terminated {
        return Err(Error::invalid(
            "language string is missing its zero terminator",
        ));
    }
    String::from_utf16(&units).map_err(|error| {
        Error::invalid(format!("language string is not valid UTF-16: {error}"))
    })
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/localization/binary/tests.rs"]
mod tests;
