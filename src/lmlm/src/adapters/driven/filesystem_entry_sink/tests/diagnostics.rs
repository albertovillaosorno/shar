// File:
//   - diagnostics.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/tests/diagnostics.rs
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
//   - Public materialization-error diagnostic regressions.
// - Must-Not:
//   - Read private archives or create filesystem state.
// - Allows:
//   - Synthetic entries and nonexisting output paths.
// - Split-When:
//   - Another sink error family needs independent fixtures.
// - Merge-When:
//   - Materialization errors no longer contain path evidence.
// - Summary:
//   - Proves sink diagnostics escape untrusted path text.
// - Description:
//   - Exercises entry-validation and destination-collision errors.
// - Usage:
//   - Compiled only by the LMLM filesystem sink test module.
// - Defaults:
//   - Every scenario fails before filesystem mutation.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Public materialization-error diagnostic regressions.
//!
//! Direct callers must receive single-line path evidence.

use std::path::Path;

use super::materialize_entries;
use crate::FileEntry;

#[test]
fn materialization_errors_escape_untrusted_paths() {
    let entries = [
        FileEntry {
            path: "unsafe\nname.bin".to_owned(),
            offset: 0,
            size: 1,
        },
    ];
    let result = materialize_entries(
        b"x",
        &entries,
        Path::new("output"),
    );

    assert!(
        matches!(
            result,
            Err(error)
                if !error
                    .to_string()
                    .chars()
                    .any(char::is_control)
        ),
        "materialization errors must escape untrusted path text"
    );
}
