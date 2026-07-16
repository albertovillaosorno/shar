// File:
//   - filesystem_batch_exporter_tests.rs
// Path:
//   - src/p3d/src/adapters/driven/filesystem_batch_exporter_tests.rs
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
//   - Focused regressions for the local P3D batch exporter.
// - Must-Not:
//   - Create persistent files or implement batch export behavior.
// - Allows:
//   - Exercise deterministic report and output-path helper contracts.
// - Split-When:
//   - One helper family requires an independent fixture boundary.
// - Merge-When:
//   - Batch exporter tests no longer obscure production behavior.
// - Summary:
//   - Local P3D batch exporter regression coverage.
// - Description:
//   - Verifies JSON report identity and output path helpers.
// - Usage:
//   - Included by filesystem_batch_exporter.rs under cfg(test).
// - Defaults:
//   - Tests use only deterministic in-memory values.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Regression tests for the local P3D batch exporter.
//!
//! These cases protect JSON report identity and output path rules without
//! creating persistent local state.

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
    let row = report_line(
        "ok", root, input, output, &error,
    );
    let escaped = escape_json(&error);
    let contains = row.contains(&escaped);
    assert!(contains);
}

#[cfg(windows)]
#[test]
fn report_rows_preserve_unpaired_utf16_path_units() {
    let path = PathBuf::from(
        OsString::from_wide(
            &[
                u16::from(b'a'),
                0xd800,
                u16::from(b'b'),
            ],
        ),
    );
    let row = report_line(
        "failed",
        &path,
        &path,
        &path,
        "read failure",
    );
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
    assert_ne!(
        nested,
        underscored
    );
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
