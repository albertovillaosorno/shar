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
//   - Filesystem batch exporter tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem batch exporter tests test module.
// - Description:
//   - Implements the declared test module responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem batch exporter tests test module.

#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt as _;
use std::path::Path;
#[cfg(windows)]
use std::path::PathBuf;

#[cfg(windows)]
use schoenwald_filesystem::DiagnosticPath;

use super::{
    escape_json, path_without_extension, report_line, root_identity_path,
};

#[test]
fn report_rows_preserve_json_string_identity() {
    let root = Path::new("root");
    let input = Path::new("input");
    let output = Path::new("output");
    let mut error = String::from("quote");
    error.push('"');
    error.push(char::from(92));
    error.push(char::from(10));
    error.push(char::from(9));
    let row = report_line("ok", root, input, output, &error);
    let escaped = escape_json(&error);
    let contains = row.contains(&escaped);
    assert!(contains);
}

#[cfg(windows)]
#[test]
fn report_rows_preserve_unpaired_utf16_path_units() {
    let path = PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800,
        u16::from(b'b'),
    ]));
    let row = report_line("failed", &path, &path, &path, "read failure");
    let expected = escape_json(&DiagnosticPath::new(&path).to_string());

    assert!(
        row.contains(&expected),
        "report row lost native path identity: {row:?}"
    );
    assert!(!row.contains('\u{fffd}'));
}

#[test]
fn distinct_input_roots_keep_distinct_output_identities() {
    let nested_root = Path::new("a/b");
    let underscored_root = Path::new("a_b");
    let nested = root_identity_path(nested_root);
    let underscored = root_identity_path(underscored_root);
    assert_ne!(nested, underscored);
}

#[test]
fn removes_only_p3d_leaf_extensions() {
    let input = Path::new("folder.p3d/nested/model.P3D");
    assert_eq!(
        path_without_extension(input),
        Path::new("folder.p3d/nested/model")
    );
    assert_ne!(
        path_without_extension(input),
        path_without_extension(Path::new("folder/nested/model.P3D"))
    );
}
