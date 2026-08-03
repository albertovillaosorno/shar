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
//   - Prefix test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Prefix test module.
// - Description:
//   - Implements the declared test module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Prefix test module.

use std::io;

use super::{materialize_entries, remove_test_root, test_root};
use crate::FileEntry;

#[test]
fn rejects_file_directory_prefix_collisions_before_writing()
-> Result<(), String> {
    for parent_first in [true, false] {
        let root = test_root(if parent_first {
            "prefix-parent-first"
        } else {
            "prefix-child-first"
        });
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
            vec![parent, child]
        } else {
            vec![child, parent]
        };
        let result = materialize_entries(b"ab", &entries, &root);
        let parent_exists = root.join("node").exists();
        let child_exists = root.join("node/child.bin").exists();
        remove_test_root(&root)?;
        match result {
            Err(error)
                if error.kind() == io::ErrorKind::AlreadyExists
                    && !parent_exists
                    && !child_exists => {},
            other => {
                return Err(format!(
                    "file/directory prefix collisions must fail before \
                         writes, got {other:?}, \
                         parent_exists={parent_exists}, \
                         child_exists={child_exists}"
                ));
            },
        }
    }
    Ok(())
}
