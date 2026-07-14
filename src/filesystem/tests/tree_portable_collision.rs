// File:
//   - tree_portable_collision.rs
// Path:
//   - src/filesystem/tests/tree_portable_collision.rs
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
//   - Regression coverage for portable tree identity collisions.
// - Must-Not:
//   - Depend on host case sensitivity or concrete filesystem storage.
// - Allows:
//   - Supply colliding port paths and assert fail-closed snapshots.
// - Split-When:
//   - Split when another portable identity rule needs unrelated fixtures.
// - Merge-When:
//   - Another test target owns the same tree collision contract.
// - Summary:
//   - Portable tree collision regression tests.
// - Description:
//   - Prevents snapshots that cannot coexist on case-insensitive hosts.
// - Usage:
//   - Runs through the filesystem crate test target.
// - Defaults:
//   - Locale-independent uppercase identity defines collisions.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for portable tree identity collisions.
//!
//! Case aliases must not escape a host-independent tree snapshot.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::application::CollectRegularFiles;
use schoenwald_filesystem::ports::TreeReader;

struct CaseCollidingTree;

impl TreeReader for CaseCollidingTree {
    fn regular_files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(
            vec![
                PathBuf::from("root/File.bin"),
                PathBuf::from("root/file.bin"),
            ],
        )
    }
}

#[test]
fn case_colliding_tree_entries_are_rejected() -> Result<(), String> {
    let result = CollectRegularFiles::execute(
        &CaseCollidingTree,
        Path::new("root"),
    );
    let Err(error) = result else {
        return Err("case-colliding tree entries were accepted".to_owned());
    };
    if error.kind() != io::ErrorKind::InvalidData {
        return Err(
            format!(
                "unexpected collision error kind: {:?}",
                error.kind()
            ),
        );
    }
    Ok(())
}
