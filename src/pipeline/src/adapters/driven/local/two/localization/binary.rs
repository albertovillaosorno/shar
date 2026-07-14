// File:
//   - binary.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/localization/binary.rs
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
//   - Bounded scalar, PString, and UTF-16 string-buffer decoding.
// - Must-Not:
//   - Read outside declared ranges, replace malformed text, or infer package
//   - ids.
// - Allows:
//   - Checked cursor movement and strict string framing.
// - Split-When:
//   - A binary primitive gains an independent format contract.
// - Merge-When:
//   - Another localization reader owns identical checked byte operations.
// - Summary:
//   - Checked binary primitives for TextBible decoding.
// - Description:
//   - Centralizes range and framing invariants used by language chunks.
// - Usage:
//   - Used only by the TextBible source adapter.
// - Defaults:
//   - Malformed source data fails closed without implicit output.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - true
//   - Reason: Checked binary primitives for TextBible decoding keeps tightly
//   - coupled validation, ordering, and deterministic transformation
//   - invariants together; split when a stable independently testable sub-
//   - boundary is identified.
//

//! Checked binary primitives prevent malformed `TextBible` fields from
//! escaping.

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
        if start > end
            || bytes
                .get(start..end)
                .is_none()
        {
            return Err(
                Error::invalid("localization chunk range is out of bounds"),
            );
        }
        Ok(
            Self {
                bytes,
                position: start,
                end,
            },
        )
    }

    /// Read one byte and advance the bounded cursor.
    pub(super) fn read_u8(&mut self) -> Outcome<u8> {
        self.read_bytes(1)?
            .first()
            .copied()
            .ok_or_else(
                || Error::invalid("unexpected end of localization byte"),
            )
    }

    /// Read one little-endian scalar and advance the bounded cursor.
    pub(super) fn read_u32(&mut self) -> Outcome<u32> {
        let raw = self.read_bytes(4)?;
        let array: [u8; 4] = raw
            .try_into()
            .map_err(
                |error| Error::invalid(format!("invalid u32 width: {error}")),
            )?;
        Ok(u32::from_le_bytes(array))
    }

    /// Read an exact byte range and advance the bounded cursor.
    pub(super) fn read_bytes(
        &mut self,
        length: usize,
    ) -> Outcome<&'a [u8]> {
        let end = self
            .position
            .checked_add(length)
            .ok_or_else(|| Error::invalid("localization range overflowed"))?;
        if end > self.end {
            return Err(Error::invalid("localization range exceeds its chunk"));
        }
        let value = self
            .bytes
            .get(self.position..end)
            .ok_or_else(
                || Error::invalid("localization range is out of bounds"),
            )?;
        self.position = end;
        Ok(value)
    }

    /// Read one padded length-prefixed UTF-8 string.
    pub(super) fn read_pstring(&mut self) -> Outcome<String> {
        let length = usize::from(self.read_u8()?);
        let raw = self.read_bytes(length)?;
        let content_len = raw
            .iter()
            .position(|byte| *byte == 0)
            .unwrap_or(raw.len());
        let padding = raw
            .get(content_len..)
            .ok_or_else(
                || Error::invalid("PString padding is out of bounds"),
            )?;
        if padding
            .iter()
            .any(|byte| *byte != 0)
        {
            return Err(
                Error::invalid(
                    "PString contains nonzero data after null padding",
                ),
            );
        }
        let content = raw
            .get(..content_len)
            .ok_or_else(
                || Error::invalid("PString content is out of bounds"),
            )?;
        std::str::from_utf8(content)
            .map(str::to_owned)
            .map_err(
                |error| {
                    Error::invalid(
                        format!("PString is not valid UTF-8: {error}"),
                    )
                },
            )
    }
}

/// Decode one aligned, zero-terminated UTF-16LE string from a shared buffer.
pub(super) fn read_utf16z(
    buffer: &[u8],
    offset: usize,
) -> Outcome<String> {
    if !buffer
        .len()
        .is_multiple_of(2)
    {
        return Err(
            Error::invalid("language string buffer is not UTF-16 aligned"),
        );
    }
    if !offset.is_multiple_of(2) {
        return Err(
            Error::invalid(
                format!(
                    "language string offset {offset} is not UTF-16 aligned"
                ),
            ),
        );
    }
    let payload = buffer
        .get(offset..)
        .ok_or_else(
            || {
                Error::invalid(
                    format!("language string offset {offset} is out of bounds"),
                )
            },
        )?;
    let mut units = Vec::new();
    let mut terminated = false;
    for pair in payload.chunks(2) {
        let [
            low,
            high,
        ] = pair
        else {
            return Err(
                Error::invalid(
                    "language string has an incomplete UTF-16 code unit",
                ),
            );
        };
        let value = u16::from_le_bytes(
            [
                *low, *high,
            ],
        );
        if value == 0 {
            terminated = true;
            break;
        }
        units.push(value);
    }
    if !terminated {
        return Err(
            Error::invalid("language string is missing its zero terminator"),
        );
    }
    String::from_utf16(&units).map_err(
        |error| {
            Error::invalid(
                format!("language string is not valid UTF-16: {error}"),
            )
        },
    )
}

#[cfg(test)]
mod tests {
    use super::{ByteCursor, read_utf16z};

    #[test]
    fn failed_byte_read_preserves_bounded_position() -> Result<(), String> {
        let bytes = [
            10, 20,
        ];
        let mut cursor = ByteCursor::new(
            &bytes, 1, 1,
        )
        .map_err(|error| error.to_string())?;
        if cursor
            .read_u8()
            .is_ok()
        {
            return Err("byte beyond declared chunk was returned".to_owned());
        }
        if cursor.position != 1 {
            return Err(
                format!(
                    "failed byte read advanced cursor to {}",
                    cursor.position
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_invalid_pstring_utf8() -> Result<(), String> {
        let mut cursor = ByteCursor::new(
            &[
                1, 0xff,
            ],
            0,
            2,
        )
        .map_err(|error| error.to_string())?;
        if cursor
            .read_pstring()
            .is_err()
        {
            Ok(())
        } else {
            Err("invalid PString UTF-8 was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_nonzero_pstring_padding() -> Result<(), String> {
        let mut cursor = ByteCursor::new(
            &[
                3, b'A', 0, b'B',
            ],
            0,
            4,
        )
        .map_err(|error| error.to_string())?;
        if cursor
            .read_pstring()
            .is_err()
        {
            Ok(())
        } else {
            Err("invalid PString padding was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_unterminated_utf16_string() -> Result<(), String> {
        if read_utf16z(
            &[
                b'A', 0,
            ],
            0,
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("unterminated UTF-16 string was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_unpaired_utf16_surrogate() -> Result<(), String> {
        if read_utf16z(
            &[
                0, 0xd8, 0, 0,
            ],
            0,
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("unpaired UTF-16 surrogate was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_odd_buffer_and_offset() -> Result<(), String> {
        if read_utf16z(
            &[
                0, 0, 0xff,
            ],
            0,
        )
        .is_ok()
        {
            return Err("odd UTF-16 buffer was accepted".to_owned());
        }
        if read_utf16z(
            &[
                0xff, b'A', 0, 0,
            ],
            1,
        )
        .is_ok()
        {
            return Err("odd UTF-16 offset was accepted".to_owned());
        }
        Ok(())
    }
}
