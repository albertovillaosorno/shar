// File:
//   - tree_uniqueness.rs
// Path:
//   - src/filesystem/tests/tree_uniqueness.rs
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
//   - Regression coverage for unique tree snapshot entries.
// - Must-Not:
//   - Depend on local storage or caller-specific deduplication.
// - Allows:
//   - Supply duplicate port output and assert one public path identity.
// - Split-When:
//   - Split when another identity rule needs unrelated fixtures.
// - Merge-When:
//   - Another test file owns the same uniqueness contract.
// - Summary:
//   - Tree uniqueness regression tests.
// - Description:
//   - Ensures duplicate adapter rows cannot multiply downstream work.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - One path appears at most once in a snapshot.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for unique tree snapshot entries.
//!
//! Duplicate provider rows must collapse before leaving the application layer.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct DuplicateTree;

impl TreeReader for DuplicateTree {
    fn regular_files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(
            vec![
                PathBuf::from("root/file.bin"),
                PathBuf::from("root/file.bin"),
            ],
        )
    }
}

#[test]
fn application_removes_duplicate_port_rows() -> Result<(), String> {
    let actual = CollectRegularFiles::execute(
        &DuplicateTree,
        Path::new("root"),
    )
    .map_err(|error| error.to_string())?;
    let expected = vec![PathBuf::from("root/file.bin")];

    if actual != expected {
        return Err(
            format!("duplicate paths escaped the use case: {actual:?}"),
        );
    }
    Ok(())
}
