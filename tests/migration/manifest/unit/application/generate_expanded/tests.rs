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

use std::path::{Path, PathBuf};

use super::{collect_game_files, escape, portable_relative, validate_paths};

#[test]
fn overlapping_roots_fail_before_output_read() {
    let result = validate_paths(
        Path::new("game"),
        Path::new("game"),
        Path::new("output/result.jsonl"),
    );

    assert!(result.is_err());
}

#[test]
fn escape_preserves_control_character_identity() {
    assert_eq!(escape("line\nfield\t\u{0001}"), "line\\nfield\\t\\u0001");
}

#[cfg(unix)]
#[test]
fn portable_relative_rejects_invalid_utf8() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt as _;

    let component =
        PathBuf::from(OsString::from_vec(vec![b'b', 0xff_u8, b'x']));
    let path = PathBuf::from("game").join(component).join("asset.p3d");

    assert!(portable_relative(Path::new("game"), &path,).is_err());
}

#[cfg(unix)]
#[test]
fn game_collection_rejects_non_unicode_file_name() {
    use std::ffi::OsString;
    use std::os::unix::ffi::OsStringExt as _;

    let file_name =
        OsString::from_vec(vec![b'b', 0xff_u8, b'x', b'.', b'p', b'3', b'd']);
    let path = PathBuf::from("game").join(file_name);
    let mut records = Vec::new();
    let result = collect_game_files(
        Path::new("game"),
        Path::new("output/expanded.jsonl"),
        &[path],
        &mut records,
    );

    assert!(result.is_err());
    assert!(records.is_empty());
}

#[cfg(windows)]
#[test]
fn game_collection_rejects_non_unicode_file_name() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;

    let file_name = OsString::from_wide(&[
        u16::from(b'b'),
        0xd800_u16,
        u16::from(b'x'),
        u16::from(b'.'),
        u16::from(b'p'),
        u16::from(b'3'),
        u16::from(b'd'),
    ]);
    let path = PathBuf::from("game").join(file_name);
    let mut records = Vec::new();
    let result = collect_game_files(
        Path::new("game"),
        Path::new("output/expanded.jsonl"),
        &[path],
        &mut records,
    );

    assert!(result.is_err());
    assert!(records.is_empty());
}

#[cfg(windows)]
#[test]
fn portable_relative_rejects_unpaired_utf16() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;

    let component = PathBuf::from(OsString::from_wide(&[
        u16::from(b'b'),
        0xd800_u16,
        u16::from(b'x'),
    ]));
    let path = PathBuf::from("game").join(component).join("asset.p3d");

    assert!(portable_relative(Path::new("game"), &path,).is_err());
}
