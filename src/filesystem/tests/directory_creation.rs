// File:
//   - directory_creation.rs
// Path:
//   - src/filesystem/tests/directory_creation.rs
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
//   - Regression coverage for explicit directory-creation inputs.
// - Must-Not:
//   - Test caller policy or depend on machine-specific paths.
// - Allows:
//   - Assert that false-success directory requests fail closed.
// - Split-When:
//   - Split when another filesystem capability needs independent coverage.
// - Merge-When:
//   - Another test file owns the same directory-creation contract.
// - Summary:
//   - Directory creation regression tests.
// - Description:
//   - Protects public local composition from reporting no-op creation success.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Invalid requests must not be accepted as successful work.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for explicit directory-creation inputs.
//!
//! Invalid no-op requests must fail instead of reporting created state.
use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

#[test]
fn empty_directory_path_is_rejected() -> Result<(), String> {
    let error = match local::create_dir_all(Path::new("")) {
        Ok(()) => {
            return Err(
                "an empty path reported directory creation success".to_owned(),
            );
        }
        Err(error) => error,
    };

    if error.kind() != io::ErrorKind::InvalidInput {
        return Err(
            format!(
                "unexpected empty-path error kind: {:?}",
                error.kind()
            ),
        );
    }
    Ok(())
}
