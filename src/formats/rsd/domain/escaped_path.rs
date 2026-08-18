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
//   - Escaped path domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Escaped path domain module.
// - Description:
//   - Implements the declared domain module responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Escaped path domain module.
use std::path::Path;

/// Wraps one untrusted path without normalizing its native identity.
#[derive(Debug)]
#[expect(
    clippy::redundant_pub_crate,
    reason = "The parent module re-exports this renderer."
)]
pub(crate) struct EscapedPath<'value>(&'value Path);

impl<'value> EscapedPath<'value> {
    /// Creates one borrowed diagnostic path renderer.
    #[must_use]
    pub(crate) const fn new(path: &'value Path) -> Self {
        Self(path)
    }
}

impl core::fmt::Display for EscapedPath<'_> {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write_path(formatter, self.0)
    }
}

/// Writes one scalar through Rust's stable reversible escaping grammar.
fn write_character(
    formatter: &mut core::fmt::Formatter<'_>,
    character: char,
) -> core::fmt::Result {
    for escaped in character.escape_default() {
        write!(formatter, "{escaped}")?;
    }
    Ok(())
}

/// Preserves Windows path identity, including unpaired UTF-16 units.
#[cfg(windows)]
fn write_path(
    formatter: &mut core::fmt::Formatter<'_>,
    path: &Path,
) -> core::fmt::Result {
    use std::os::windows::ffi::OsStrExt as _;

    for decoded in char::decode_utf16(path.as_os_str().encode_wide()) {
        match decoded {
            Ok(character) => write_character(formatter, character)?,
            Err(error) => {
                write!(formatter, r"\u{{{:04X}}}", error.unpaired_surrogate())?;
            },
        }
    }
    Ok(())
}

/// Preserves Unix path identity, including invalid UTF-8 bytes.
#[cfg(unix)]
fn write_path(
    formatter: &mut core::fmt::Formatter<'_>,
    path: &Path,
) -> core::fmt::Result {
    use std::os::unix::ffi::OsStrExt as _;

    let mut remaining = path.as_os_str().as_bytes();
    while !remaining.is_empty() {
        match core::str::from_utf8(remaining) {
            Ok(text) => {
                for character in text.chars() {
                    write_character(formatter, character)?;
                }
                break;
            },
            Err(error) => {
                let valid_length = error.valid_up_to();
                let valid_bytes =
                    remaining.get(..valid_length).ok_or(core::fmt::Error)?;
                let valid_text = core::str::from_utf8(valid_bytes)
                    .map_err(|_utf8_error| core::fmt::Error)?;
                for character in valid_text.chars() {
                    write_character(formatter, character)?;
                }
                let invalid_length = match error.error_len() {
                    Some(length) => length,
                    None => remaining
                        .len()
                        .checked_sub(valid_length)
                        .ok_or(core::fmt::Error)?,
                };
                let invalid_end = valid_length
                    .checked_add(invalid_length)
                    .ok_or(core::fmt::Error)?;
                let invalid_bytes = remaining
                    .get(valid_length..invalid_end)
                    .ok_or(core::fmt::Error)?;
                for byte in invalid_bytes {
                    write!(formatter, r"\x{byte:02X}")?;
                }
                remaining =
                    remaining.get(invalid_end..).ok_or(core::fmt::Error)?;
            },
        }
    }
    Ok(())
}

/// Falls back to scalar escaping on targets without native encoding access.
#[cfg(not(any(unix, windows)))]
fn write_path(
    formatter: &mut core::fmt::Formatter<'_>,
    path: &Path,
) -> core::fmt::Result {
    for character in path.to_string_lossy().chars() {
        write_character(formatter, character)?;
    }
    Ok(())
}
