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
//   - Error display test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Error display test module.
// - Description:
//   - Implements the declared test module responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Error display test module.

use rsd::RsdError;
use same_file as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn unsupported_encoding_escapes_control_bytes() {
    let rendered =
        RsdError::UnsupportedEncoding([0x1b_u8, b'[', b'2', b'J']).to_string();

    assert!(
        !rendered.contains('\u{1b}'),
        "untrusted codec bytes must not emit terminal escape controls"
    );
    assert_eq!(rendered, r"unsupported RSD encoding: \x1B\x5B\x32\x4A");
}

#[test]
fn path_errors_escape_control_characters() {
    let rendered =
        RsdError::InvalidPath(std::path::PathBuf::from("\u{1b}[2Jbad.rsd"))
            .to_string();

    assert!(
        !rendered.contains('\u{1b}'),
        "untrusted paths must not emit terminal escape controls"
    );
    assert!(
        rendered.contains(r"\u{1b}"),
        "escaped diagnostics must retain the original control code"
    );
}

#[cfg(windows)]
#[test]
fn path_errors_preserve_unpaired_utf16_units() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;

    let path = std::path::PathBuf::from(OsString::from_wide(&[
        u16::from(b'b'),
        0xd800_u16,
        u16::from(b'x'),
    ]));
    let rendered = RsdError::InvalidPath(path).to_string();

    assert!(
        rendered.contains(r"\u{D800}"),
        "diagnostics must preserve the exact unpaired UTF-16 unit"
    );
    assert!(
        !rendered.contains('\u{fffd}'),
        "diagnostics must not replace invalid path units"
    );
}

#[cfg(unix)]
#[test]
fn path_errors_preserve_invalid_utf8_bytes() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt as _;

    let path =
        std::path::PathBuf::from(OsString::from_vec(vec![b'b', 0xff_u8, b'x']));
    let rendered = RsdError::InvalidPath(path).to_string();

    assert!(
        rendered.contains(r"\xFF"),
        "diagnostics must preserve the exact invalid UTF-8 byte"
    );
    assert!(
        !rendered.contains('\u{fffd}'),
        "diagnostics must not replace invalid path bytes"
    );
}
