// File:
//   - prefix.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/tests/prefix.rs
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
//   - Regression coverage for file/directory destination prefix collisions.
// - Must-Not:
//   - Read private archives or retain temporary output directories.
// - Allows:
//   - Synthetic entries and process-local filesystem roots.
// - Split-When:
//   - Another destination identity family needs independent fixtures.
// - Merge-When:
//   - The parent materialization tests remain below the file-size boundary.
// - Summary:
//   - Proves prefix collisions fail before publication begins.
// - Description:
//   - Exercises both archive entry orders for one file/directory conflict.
// - Usage:
//   - Compiled only by the LMLM filesystem sink test module.
// - Defaults:
//   - Every temporary root is removed before and after each scenario.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Prefix-collision regressions for LMLM materialization.
//!
//! Both entry orders must fail before creating either destination.

use std::io;

use super::{materialize_entries, remove_test_root, test_root};
use crate::FileEntry;

#[test]
fn rejects_file_directory_prefix_collisions_before_writing()
-> Result<(), String> {
    for parent_first in [
        true, false,
    ] {
        let root = test_root(
            if parent_first {
                "prefix-parent-first"
            } else {
                "prefix-child-first"
            },
        );
        remove_test_root(&root)?;
        let parent = FileEntry {
            path: "node".to_owned(),
            offset: 0,
            size: 1,
        };
        let child = FileEntry {
            path: "node/child.bin".to_owned(),
            offset: 1,
            size: 1,
        };
        let entries = if parent_first {
            vec![
                parent, child,
            ]
        } else {
            vec![
                child, parent,
            ]
        };
        let result = materialize_entries(
            b"ab", &entries, &root,
        );
        let parent_exists = root
            .join("node")
            .exists();
        let child_exists = root
            .join("node/child.bin")
            .exists();
        remove_test_root(&root)?;
        match result {
            Err(error)
                if error.kind() == io::ErrorKind::AlreadyExists
                    && !parent_exists
                    && !child_exists => {}
            other => {
                return Err(
                    format!(
                        "file/directory prefix collisions must fail before \
                         writes, got {other:?}, \
                         parent_exists={parent_exists}, \
                         child_exists={child_exists}"
                    ),
                );
            }
        }
    }
    Ok(())
}
