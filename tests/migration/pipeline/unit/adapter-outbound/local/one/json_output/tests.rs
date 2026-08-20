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

use super::{validate_document, validate_lines};

#[cfg(windows)]
#[test]
fn non_unicode_json_path_error_is_reversible() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;

    let path = std::path::PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800_u16,
        u16::from(b'b'),
        u16::from(b'.'),
        u16::from(b'j'),
        u16::from(b's'),
        u16::from(b'o'),
        u16::from(b'n'),
    ]));
    let result = super::validate_generated_text_file(&path);
    assert!(
        result.is_err(),
        "missing non-Unicode JSON unexpectedly passed"
    );
    let Err(error) = result else {
        return;
    };
    let rendered = error.to_string();
    let prefix = r"failed to read generated JSON a\u{D800}b.json: ";
    assert!(
        rendered.strip_prefix(prefix).is_some(),
        "diagnostic lost native path: {rendered:?}"
    );
}

#[test]
fn rejects_raw_control_characters() {
    let quote = char::from(34);
    let nul = char::from(0);
    let invalid = format!("{{{quote}name{quote}:{quote}bad{nul}value{quote}}}");
    assert!(validate_document(invalid.as_bytes(), "raw-control",).is_err());
}

#[test]
fn accepts_escaped_controls_and_valid_jsonl_rows() {
    let quote = char::from(34);
    let slash = char::from(92);
    let escaped =
        format!("{{{quote}name{quote}:{quote}good{slash}u0000value{quote}}}");
    let rows = format!("{{{quote}row{quote}:1}}\n{{{quote}row{quote}:2}}\n");
    assert!(validate_document(escaped.as_bytes(), "escaped-control",).is_ok());
    assert!(validate_lines(&rows, "rows",).is_ok());
}
