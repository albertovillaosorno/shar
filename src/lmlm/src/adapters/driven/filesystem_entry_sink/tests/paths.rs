// File:
//   - paths.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/tests/paths.rs
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
//   - Parser-equivalent path safety regressions for filesystem publication.
// - Must-Not:
//   - Read private archives or retain temporary output directories.
// - Allows:
//   - Synthetic entries and process-local filesystem roots.
// - Split-When:
//   - Another portable path family needs independent fixtures.
// - Merge-When:
//   - The parent test module remains below its file-size boundary.
// - Summary:
//   - Proves parser-unsafe paths fail before publication begins.
// - Description:
//   - Exercises Unicode modifiers that filesystems can otherwise persist.
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

//! Portable path safety regressions for LMLM materialization.
//!
//! The public sink must enforce the same path rules as the parser.

use std::io;

use super::{materialize_entries, remove_test_root, test_root};
use crate::FileEntry;

#[test]
fn rejects_unicode_path_modifiers_before_writing() -> Result<(), String> {
    let root = test_root("unsafe-unicode-path");
    remove_test_root(&root)?;
    let path = "report\u{202e}cod.exe";
    let entries = [
        FileEntry {
            path: path.to_owned(),
            offset: 0,
            size: 1,
        },
    ];
    let result = materialize_entries(
        b"x", &entries, &root,
    );
    let destination_exists = root
        .join(path)
        .exists();
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::InvalidInput
                && !destination_exists =>
        {
            Ok(())
        }
        other => Err(
            format!(
                "unsafe path must fail before writing, got {other:?}, \
                 destination_exists={destination_exists}"
            ),
        ),
    }
}
