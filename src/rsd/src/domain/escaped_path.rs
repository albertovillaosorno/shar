// File:
//   - escaped_path.rs
// Path:
//   - src/rsd/src/domain/escaped_path.rs
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
//   - Read filesystem state, normalize path identity, or choose diagnostics.
// - Allows:
//   - Platform-aware encoding traversal and reversible invalid-unit escaping.
// - Split-When:
//   - Another diagnostic transport requires a distinct escaping grammar.
// - Merge-When:
//   - Another RSD domain module owns the same exact path-rendering contract.
// - Summary:
//   - Lossless diagnostic wrapper for RSD filesystem paths.
// - Description:
//   - Preserves valid text, control characters, invalid UTF-8 bytes, and
//   - unpaired UTF-16 units without terminal injection or replacement loss.
// - Usage:
//   - Used by RSD errors and command reports before text reaches stderr.
// - Defaults:
//   - Valid characters remain readable and invalid units use uppercase escapes.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Lossless, control-safe rendering for untrusted RSD paths.
//! Valid text stays readable while invalid platform units remain reversible.
use std::path::Path;

/// Renders one valid scalar through Rust's stable control escaping grammar.
fn write_character(
    formatter: &mut core::fmt::Formatter<'_>,
    character: char,
) -> core::fmt::Result {
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
pub(super) fn write_path(
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
pub(super) fn write_path(
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

/// Falls back to scalar escaping on targets without byte or UTF-16 access.
#[cfg(
    not(
        any(
            unix, windows
        )
    )
)]
pub(super) fn write_path(
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
