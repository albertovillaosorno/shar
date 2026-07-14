// File:
//   - diagnostic_path.rs
// Path:
//   - src/game-manifest/src/application/diagnostic_path.rs
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
//   - Control-safe rendering of application-owned diagnostic paths.
// - Must-Not:
//   - Access storage, normalize path identity, or choose command wording.
// - Allows:
//   - Deterministic printable and hexadecimal escaping of native path bytes.
// - Split-When:
//   - Split when another diagnostic value needs a distinct encoding contract.
// - Merge-When:
//   - Another application module owns the same native-path rendering boundary.
// - Summary:
//   - Prevents raw path controls from reaching operator diagnostics.
// - Description:
//   - Preserves visible path context while escaping non-printable bytes.
// - Usage:
//   - Used by game-manifest application errors and invalid-path diagnostics.
// - Defaults:
//   - Printable ASCII remains readable and all other bytes use lowercase hex.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Control-safe path rendering for application diagnostics.
//!
//! Native path bytes remain identifiable without emitting terminal controls.

use std::path::Path;

/// Renders one path without raw controls or lossy Unicode replacement.
#[must_use]
pub(super) fn escaped_path(path: &Path) -> String {
    let encoded = path
        .as_os_str()
        .as_encoded_bytes();
    let mut output = String::with_capacity(encoded.len());
    for byte in encoded {
        let value = *byte;
        if value == b'\\' {
            output.push('\\');
            output.push('\\');
        } else if value == b' ' || value.is_ascii_graphic() {
            output.push(char::from(value));
        } else {
            append_hex_byte(
                &mut output,
                value,
            );
        }
    }
    output
}

/// Appends one byte as a deterministic lowercase hexadecimal escape.
fn append_hex_byte(
    output: &mut String,
    byte: u8,
) {
    output.push('\\');
    output.push('x');
    output.push(hex_digit(byte >> 4_u32));
    output.push(hex_digit(byte & 0x0f_u8));
}

/// Converts one hexadecimal nibble into its lowercase display digit.
fn hex_digit(value: u8) -> char {
    char::from_digit(
        u32::from(value),
        16_u32,
    )
    .unwrap_or('?')
}
