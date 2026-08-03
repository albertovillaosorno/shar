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

#[cfg(windows)]
#[test]
fn provider_path_keeps_one_native_escape_layer() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;

    let path = std::path::PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800_u16,
        u16::from(b'b'),
    ]));
    let result = super::inspect_path_kind(&path);
    assert!(result.is_err(), "non-Unicode path unexpectedly inspected");
    let Err(error) = result else {
        return;
    };

    assert!(error.to_string().contains(r"a\u{D800}b"));
    assert!(!error.to_string().contains(r"a\\u{D800}b"));
    assert!(std::error::Error::source(&error).is_some());
}
