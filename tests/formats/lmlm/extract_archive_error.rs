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
//   - Extract archive error test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Extract archive error test module.
// - Description:
//   - Implements the declared test module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Extract archive error test module.

#[cfg(windows)]
use std::ffi::OsString;
use std::io;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt as _;
use std::path::PathBuf;

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
            !rendered.chars().any(char::is_control),
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
