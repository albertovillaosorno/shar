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
//   - Root identity outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Root identity outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Root identity outbound adapter.

use std::path::{Path, PathBuf};

/// Raw identity bytes stored in each portable path component.
const IDENTITY_COMPONENT_BYTES: usize = 32;
/// Stable prefix for the first root-identity encoding contract.
const ROOT_IDENTITY_VERSION: &str = "root-v1";

/// Encodes one input root as a versioned portable relative identity path.
pub(super) fn root_identity_path(path: &Path) -> PathBuf {
    let bytes = path_identity_bytes(path);
    let mut identity = PathBuf::from(ROOT_IDENTITY_VERSION);
    if bytes.is_empty() {
        identity.push("empty");
        return identity;
    }
    for chunk in bytes.chunks(IDENTITY_COMPONENT_BYTES) {
        identity.push(hex_component(chunk));
    }
    identity
}

/// Encodes raw bytes as lowercase hexadecimal path content.
fn hex_component(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len().saturating_mul(2));
    for byte in bytes {
        output.push(hex_digit(byte >> 4));
        output.push(hex_digit(byte & 15));
    }
    output
}

/// Maps one four-bit value to stable lowercase hexadecimal text.
const fn hex_digit(value: u8) -> char {
    match value {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => '?',
    }
}

/// Returns raw Windows UTF-16 path units as little-endian bytes.
#[cfg(windows)]
fn path_identity_bytes(path: &Path) -> Vec<u8> {
    use std::os::windows::ffi::OsStrExt as _;

    path.as_os_str()
        .encode_wide()
        .flat_map(u16::to_le_bytes)
        .collect()
}

/// Returns raw Unix path bytes without Unicode replacement.
#[cfg(unix)]
fn path_identity_bytes(path: &Path) -> Vec<u8> {
    use std::os::unix::ffi::OsStrExt as _;

    path.as_os_str().as_bytes().to_vec()
}
