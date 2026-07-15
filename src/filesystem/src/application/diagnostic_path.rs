// File:
//   - diagnostic_path.rs
// Path:
//   - src/filesystem/src/application/diagnostic_path.rs
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
//   - Exact, control-safe rendering of untrusted filesystem paths.
// - Must-Not:
//   - Read filesystem state, normalize path identity, or choose error policy.
// - Allows:
//   - Platform-aware encoding traversal and reversible invalid-unit escaping.
// - Split-When:
//   - Another diagnostic transport requires a distinct escaping grammar.
// - Merge-When:
//   - Another filesystem application module owns the same path rendering.
// - Summary:
//   - Lossless diagnostic wrapper for shared filesystem paths.
// - Description:
//   - Preserves path identity without allowing controls or native invalid units
//   - to alter terminal diagnostics.
// - Usage:
//   - Used by shared filesystem application errors before text reaches callers.
// - Defaults:
//   - Printable ASCII stays readable; other units use reversible escapes.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Lossless, control-safe rendering for shared filesystem diagnostics.
use std::path::Path;

/// Wraps one untrusted path without normalizing its native identity.
pub(super) struct DiagnosticPath<'a>(&'a Path);

impl<'a> DiagnosticPath<'a> {
    /// Creates one borrowed diagnostic path renderer.
    #[must_use]
    pub(super) const fn new(path: &'a Path) -> Self {
        Self(path)
    }
}

impl core::fmt::Display for DiagnosticPath<'_> {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        write_path(
            formatter, self.0,
        )
    }
}

/// Writes printable ASCII directly and escapes every other scalar.
fn write_character(
    formatter: &mut core::fmt::Formatter<'_>,
    character: char,
) -> core::fmt::Result {
    if character.is_ascii() && !character.is_control() {
        return write!(
            formatter,
            "{character}"
        );
    }
    for escaped in character.escape_default() {
        write!(
            formatter,
            "{escaped}"
        )?;
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

    for decoded in char::decode_utf16(
        path.as_os_str()
            .encode_wide(),
    ) {
        match decoded {
            Ok(character) => write_character(
                formatter, character,
            )?,
            Err(error) => write!(
                formatter,
                r"\u{{{:04X}}}",
                error.unpaired_surrogate()
            )?,
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

    let mut remaining = path
        .as_os_str()
        .as_bytes();
    while !remaining.is_empty() {
        match core::str::from_utf8(remaining) {
            Ok(text) => {
                for character in text.chars() {
                    write_character(
                        formatter, character,
                    )?;
                }
                break;
            }
            Err(error) => {
                let valid_length = error.valid_up_to();
                let valid_bytes = remaining
                    .get(..valid_length)
                    .ok_or(core::fmt::Error)?;
                let valid_text = core::str::from_utf8(valid_bytes)
                    .map_err(|_utf8_error| core::fmt::Error)?;
                for character in valid_text.chars() {
                    write_character(
                        formatter, character,
                    )?;
                }
                let invalid_length = error
                    .error_len()
                    .unwrap_or_else(|| remaining.len() - valid_length);
                let invalid_end = valid_length
                    .checked_add(invalid_length)
                    .ok_or(core::fmt::Error)?;
                let invalid_bytes = remaining
                    .get(valid_length..invalid_end)
                    .ok_or(core::fmt::Error)?;
                for byte in invalid_bytes {
                    write!(
                        formatter,
                        r"\x{byte:02X}"
                    )?;
                }
                remaining = remaining
                    .get(invalid_end..)
                    .ok_or(core::fmt::Error)?;
            }
        }
    }
    Ok(())
}

/// Falls back to scalar escaping on targets without native encoding access.
#[cfg(
    not(
        any(
            unix, windows
        )
    )
)]
fn write_path(
    formatter: &mut core::fmt::Formatter<'_>,
    path: &Path,
) -> core::fmt::Result {
    for character in path
        .to_string_lossy()
        .chars()
    {
        write_character(
            formatter, character,
        )?;
    }
    Ok(())
}
