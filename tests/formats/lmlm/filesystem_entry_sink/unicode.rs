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
//   - Unicode test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Unicode test module.
// - Description:
//   - Implements the declared test module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Unicode test module.

use std::io;

use super::{materialize_entries, remove_test_root, test_root};
use crate::FileEntry;

#[test]
fn rejects_unicode_case_collisions_before_writing() -> Result<(), String> {
    let root = test_root("unicode-case-collision");
    remove_test_root(&root)?;
    let entries = [
        FileEntry {
            path: "Σ.bin".to_owned(),
            offset: 0,
            size: 1,
        },
        FileEntry {
            path: "ς.bin".to_owned(),
            offset: 1,
            size: 1,
        },
    ];
    let result = materialize_entries(b"ab", &entries, &root);
    let first_exists = root.join("Σ.bin").exists();
    let second_exists = root.join("ς.bin").exists();
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::AlreadyExists
                && !first_exists
                && !second_exists =>
        {
            Ok(())
        },
        other => Err(format!(
            "Unicode case-colliding destinations must fail before writes, \
                 got {other:?}, first_exists={first_exists}, \
                 second_exists={second_exists}"
        )),
    }
}
