// File:
//   - file_len_destination.rs
// Path:
//   - src/filesystem/tests/file_len_destination.rs
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
//   - Regression coverage for explicit file-length destinations.
// - Must-Not:
//   - Depend on concrete storage or native metadata behavior.
// - Allows:
//   - Supply a permissive inspector and assert application path rejection.
// - Split-When:
//   - Split when another inspection invariant needs unrelated fixtures.
// - Merge-When:
//   - Another test target owns the same file-length destination contract.
// - Summary:
//   - File-length destination regression tests.
// - Description:
//   - Prevents directory syntax from reaching file metadata ports.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - File-length queries require explicit file syntax.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for explicit file-length destinations.
//!
//! Directory syntax must fail before a permissive inspector port is called.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::application::InspectPath;
use schoenwald_filesystem::ports::PathInspector;

struct PermissiveInspector;

impl PathInspector for PermissiveInspector {
    fn path_kind(
        &self,
        _path: &Path,
    ) -> io::Result<PathKind> {
        Ok(PathKind::File)
    }

    fn file_len(
        &self,
        _path: &Path,
    ) -> io::Result<u64> {
        Ok(7)
    }

    fn canonicalize(
        &self,
        path: &Path,
    ) -> io::Result<PathBuf> {
        Ok(path.to_path_buf())
    }
}

#[test]
fn directory_syntax_file_length_is_rejected() -> Result<(), String> {
    let result = InspectPath::len(
        &PermissiveInspector,
        Path::new("report/"),
    );

    if result.is_ok() {
        return Err(
            "directory syntax unexpectedly returned a length".to_owned(),
        );
    }
    Ok(())
}
