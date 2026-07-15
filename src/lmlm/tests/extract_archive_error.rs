// File:
//   - extract_archive_error.rs
// Path:
//   - src/lmlm/tests/extract_archive_error.rs
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
//   - Public extraction-error diagnostic regressions.
// - Must-Not:
//   - Read archives, write outputs, or expose operator paths.
// - Allows:
//   - Synthetic paths and in-memory error sources.
// - Split-When:
//   - Another public error family needs independent fixtures.
// - Merge-When:
//   - Application errors no longer have a public display contract.
// - Summary:
//   - Proves extraction diagnostics remain single-line and escaped.
// - Description:
//   - Exercises read, parse, and materialization error variants.
// - Usage:
//   - Compiled as an LMLM integration test.
// - Defaults:
//   - No filesystem state is created.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Public extraction-error diagnostic regressions.
//!
//! Untrusted paths and source messages must not inject terminal lines.

use std::io;
use std::path::PathBuf;

#[cfg(windows)]
use std::ffi::OsString;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt as _;

use lmlm::{ExtractArchiveError, LmlmError};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn extraction_errors_escape_control_characters() {
    let errors = [
        ExtractArchiveError::Read {
            path: PathBuf::from("read\npath"),
            source: io::Error::other("read\rsource"),
        },
        ExtractArchiveError::Parse {
            path: PathBuf::from("parse\npath"),
            source: LmlmError::UnsafePath("entry\rpath".to_owned()),
        },
        ExtractArchiveError::Materialize {
            path: PathBuf::from("write\npath"),
            source: io::Error::other("write\rsource"),
        },
    ];
    for error in errors {
        let rendered = error.to_string();
        assert!(
            !rendered
                .chars()
                .any(char::is_control),
            "diagnostic contains a control character: {rendered:?}"
        );
    }
}

#[cfg(windows)]
#[test]
fn extraction_error_preserves_unpaired_utf16_path_unit() {
    let path = PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800,
        u16::from(b'b'),
    ]));
    let error = ExtractArchiveError::Read {
        path,
        source: io::Error::other("read failure"),
    };

    let rendered = error.to_string();

    assert!(
        rendered.contains(r"a\u{D800}b"),
        "diagnostic lost the native path unit: {rendered:?}"
    );
    assert!(!rendered.contains(r"\u{fffd}"));
}
