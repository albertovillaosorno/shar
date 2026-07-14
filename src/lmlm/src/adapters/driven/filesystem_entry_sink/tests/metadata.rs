// File:
//   - metadata.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/tests/metadata.rs
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
//   - Output-root metadata error context regressions.
// - Must-Not:
//   - Read private archives or create filesystem state.
// - Allows:
//   - Invalid synthetic paths that fail before publication.
// - Split-When:
//   - Another metadata inspection family needs independent fixtures.
// - Merge-When:
//   - Metadata errors no longer carry output-root context.
// - Summary:
//   - Proves metadata failures identify the inspected output path.
// - Description:
//   - Exercises native path rejection before any directory is created.
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

//! Output-root metadata error context regressions.
//!
//! Native inspection failures must retain escaped caller path evidence.

use std::io;
use std::path::Path;

use super::materialize_entries;
use crate::FileEntry;

#[test]
fn metadata_errors_include_the_output_root() {
    let entries = [
        FileEntry {
            path: "file.bin".to_owned(),
            offset: 0,
            size: 1,
        },
    ];
    let result = materialize_entries(
        b"x",
        &entries,
        Path::new("bad\0root"),
    );

    assert!(
        matches!(
            result,
            Err(error)
                if error.kind() == io::ErrorKind::InvalidInput
                    && error
                        .to_string()
                        .contains("bad")
                    && error
                        .to_string()
                        .contains("root")
                    && !error
                        .to_string()
                        .chars()
                        .any(char::is_control)
        ),
        "metadata errors must identify the inspected output root"
    );
}
