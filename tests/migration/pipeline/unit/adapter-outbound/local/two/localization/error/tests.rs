// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

#[test]
fn io_error_escapes_source_controls_and_preserves_chain() {
    let error = super::Error::io(
        std::path::PathBuf::from("language.bin"),
        std::io::Error::other("read\ninjected"),
    );

    assert_eq!(error.to_string(), r"language.bin: read\ninjected");
    assert!(std::error::Error::source(&error).is_some());
}

#[test]
fn invalid_source_escapes_control_characters() {
    let error = super::Error::invalid("invalid\nsource");

    assert_eq!(error.to_string(), r"invalid\nsource");
}

#[cfg(windows)]
#[test]
fn io_error_preserves_unpaired_utf16_path_unit() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;

    let path = std::path::PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800_u16,
        u16::from(b'b'),
    ]));
    let error = super::Error::io(path, std::io::Error::other("read failure"));

    assert_eq!(error.to_string(), r"a\u{D800}b: read failure");
}
