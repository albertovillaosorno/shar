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
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt as _;

use super::*;

#[cfg(windows)]
#[test]
fn archive_stem_error_preserves_unpaired_utf16_path_unit() {
    let path = PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800,
        u16::from(b'b'),
        u16::from(b'.'),
        u16::from(b'r'),
        u16::from(b'c'),
        u16::from(b'f'),
    ]));
    let source = FileArchiveSource::new(path);

    let result = source.archive_stem();
    assert!(
        result.is_err(),
        "non-Unicode archive stem unexpectedly succeeded"
    );
    let Err(error) = result else {
        return;
    };
    let rendered = error.to_string();

    assert!(
        rendered.contains(r"a\u{D800}b.rcf"),
        "diagnostic lost the native path unit: {rendered:?}"
    );
    assert!(!rendered.contains('\u{fffd}'));
}

#[test]
fn rejects_parent_traversal() {
    let result = safe_relative_path("sound/../escape.rsd");

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "parent traversal must be rejected before writing entries"
    );
}

#[test]
fn converts_backslashes_to_relative_path() {
    let expected = PathBuf::from("sound").join("scripts").join("apu.spt");
    let result = safe_relative_path(r"sound\scripts\apu.spt");

    assert!(
        result.as_ref().is_ok_and(|path| path == &expected),
        "backslash-separated archive names must stay relative"
    );
}

#[test]
fn rejects_console_device_aliases() {
    let result = safe_relative_path("sound/CONIN$");

    assert!(
        matches!(result, Err(ArchiveError::UnsafeEntryPath(_))),
        "console device aliases must be rejected before filesystem IO"
    );
}
