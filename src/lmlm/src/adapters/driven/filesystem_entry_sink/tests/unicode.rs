// File:
//   - unicode.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/tests/unicode.rs
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
//   - Unicode portable-identity regressions for filesystem publication.
// - Must-Not:
//   - Read private archives or retain temporary output directories.
// - Allows:
//   - Synthetic entries and process-local filesystem roots.
// - Split-When:
//   - Another Unicode destination invariant needs independent fixtures.
// - Merge-When:
//   - The parent test module remains below its file-size boundary.
// - Summary:
//   - Proves Unicode case collisions fail before publication begins.
// - Description:
//   - Exercises portable identities whose uppercase mappings converge.
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

//! Unicode collision regressions for LMLM materialization.
//!
//! Equivalent portable identities must fail before either file is created.

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
    let result = materialize_entries(
        b"ab", &entries, &root,
    );
    let first_exists = root
        .join("Σ.bin")
        .exists();
    let second_exists = root
        .join("ς.bin")
        .exists();
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::AlreadyExists
                && !first_exists
                && !second_exists =>
        {
            Ok(())
        }
        other => Err(
            format!(
                "Unicode case-colliding destinations must fail before writes, \
                 got {other:?}, first_exists={first_exists}, \
                 second_exists={second_exists}"
            ),
        ),
    }
}
