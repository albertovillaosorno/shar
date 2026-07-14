// File:
//   - tree_ordering.rs
// Path:
//   - src/filesystem/tests/tree_ordering.rs
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
//   - Regression coverage for application-owned tree ordering.
// - Must-Not:
//   - Depend on the standard filesystem adapter or caller policy.
// - Allows:
//   - Supply adversarial port output and assert stable public ordering.
// - Split-When:
//   - Split when another tree invariant needs unrelated fixtures.
// - Merge-When:
//   - Another test file owns the same ordering contract.
// - Summary:
//   - Tree ordering regression tests.
// - Description:
//   - Ensures the application use case enforces its ordering promise.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Port output order is not trusted.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for application-owned tree ordering.
//!
//! The use case must not inherit nondeterministic provider ordering.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct UnsortedTree;

impl TreeReader for UnsortedTree {
    fn regular_files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(
            vec![
                PathBuf::from("root/z.bin"),
                PathBuf::from("root/a.bin"),
            ],
        )
    }
}

#[test]
fn application_sorts_adversarial_port_output() -> Result<(), String> {
    let actual = CollectRegularFiles::execute(
        &UnsortedTree,
        Path::new("root"),
    )
    .map_err(|error| error.to_string())?;
    let expected = vec![
        PathBuf::from("root/a.bin"),
        PathBuf::from("root/z.bin"),
    ];

    if actual != expected {
        return Err(format!("tree snapshot was not sorted: {actual:?}"));
    }
    Ok(())
}
