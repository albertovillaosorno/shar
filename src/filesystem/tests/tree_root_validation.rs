// File:
//   - tree_root_validation.rs
// Path:
//   - src/filesystem/tests/tree_root_validation.rs
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
//   - Regression coverage for tree-root containment authorities.
// - Must-Not:
//   - Depend on concrete storage or returned tree entries.
// - Allows:
//   - Supply an empty reader and assert root validation before port access.
// - Split-When:
//   - Split when another tree-root invariant needs unrelated fixtures.
// - Merge-When:
//   - Another test target owns the same tree-root authority contract.
// - Summary:
//   - Tree-root validation regression tests.
// - Description:
//   - Prevents empty snapshots from bypassing root traversal validation.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Tree roots must not contain parent traversal.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for tree-root containment authorities.
//!
//! Empty reader output must not bypass validation of the requested root.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct EmptyTree;

impl TreeReader for EmptyTree {
    fn regular_files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }
}

#[test]
fn parent_traversal_tree_root_is_rejected() -> Result<(), String> {
    let result = CollectRegularFiles::execute(
        &EmptyTree,
        Path::new("root/.."),
    );

    if result.is_ok() {
        return Err(
            "traversing tree root unexpectedly returned a snapshot".to_owned(),
        );
    }
    Ok(())
}
