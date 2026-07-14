// File:
//   - read_destination.rs
// Path:
//   - src/filesystem/tests/read_destination.rs
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
//   - Regression coverage for explicit complete-file read destinations.
// - Must-Not:
//   - Depend on concrete storage or localized native failures.
// - Allows:
//   - Supply a permissive reader and assert application path rejection.
// - Split-When:
//   - Split when another read invariant needs unrelated fixtures.
// - Merge-When:
//   - Another test target owns the same complete-file destination contract.
// - Summary:
//   - Complete-file read destination regression tests.
// - Description:
//   - Prevents directory syntax from reaching file reader ports.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Complete-file reads require explicit file syntax.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for complete-file read destinations.
//!
//! Directory syntax must fail before a permissive reader port is called.
use std::io;
use std::path::Path;

use schoenwald_filesystem::application::ReadFile;
use schoenwald_filesystem::ports::FileReader;

struct PermissiveReader;

impl FileReader for PermissiveReader {
    fn read_bytes(
        &self,
        _path: &Path,
    ) -> io::Result<Vec<u8>> {
        Ok(Vec::new())
    }
}

#[test]
fn directory_syntax_read_destination_is_rejected() -> Result<(), String> {
    let result = ReadFile::bytes(
        &PermissiveReader,
        Path::new("report/"),
    );

    if result.is_ok() {
        return Err("directory syntax unexpectedly returned bytes".to_owned());
    }
    Ok(())
}
