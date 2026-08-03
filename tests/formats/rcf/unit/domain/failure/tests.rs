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
use std::ffi::OsString;
use std::io;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt as _;
#[cfg(windows)]
use std::path::PathBuf;

use super::{ArchiveError, IoFailure};

#[test]
fn io_error_escapes_source_control_characters() {
    let error =
        ArchiveError::io("archive.rcf", io::Error::other("read\nfailure"));

    let rendered = error.to_string();

    assert!(
        !rendered.chars().any(char::is_control),
        "diagnostic contains a control character: {rendered:?}"
    );
    assert!(rendered.contains(r"read\nfailure"));
    assert_eq!(
        error.io_failure().map(IoFailure::message),
        Some("read\nfailure")
    );
}

#[test]
fn unsafe_entry_path_error_escapes_control_characters() {
    let error = ArchiveError::unsafe_entry_path("bad\npath");

    let rendered = error.to_string();

    assert!(
        !rendered.chars().any(char::is_control),
        "diagnostic contains a control character: {rendered:?}"
    );
    assert!(rendered.contains(r"bad\npath"));
}

#[cfg(windows)]
#[test]
fn io_error_preserves_unpaired_utf16_path_unit() {
    let path = PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800,
        u16::from(b'b'),
    ]));
    let error = ArchiveError::io(path, io::Error::other("read failure"));

    let rendered = error.to_string();

    assert!(
        rendered.contains(r"a\u{D800}b"),
        "diagnostic lost the native path unit: {rendered:?}"
    );
    assert!(!rendered.contains('\u{fffd}'));
}
