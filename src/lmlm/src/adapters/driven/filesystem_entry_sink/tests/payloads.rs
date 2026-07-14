// File:
//   - payloads.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/tests/payloads.rs
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
//   - Regression coverage for payload preflight before publication.
// - Must-Not:
//   - Read private archives or retain temporary output directories.
// - Allows:
//   - Synthetic entry ranges and process-local filesystem roots.
// - Split-When:
//   - Another payload publication invariant needs independent fixtures.
// - Merge-When:
//   - The parent materialization tests remain below the file-size boundary.
// - Summary:
//   - Proves malformed payloads fail before filesystem mutation.
// - Description:
//   - Uses a malformed later entry to detect partial output.
// - Usage:
//   - Compiled only by the LMLM filesystem sink test module.
// - Defaults:
//   - Every temporary root is removed before and after the scenario.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Payload preflight regressions for LMLM materialization.
//!
//! Malformed later ranges must fail before any earlier file is created.

use std::io;
use std::path::Path;

use super::{materialize_entries, remove_test_root, test_root};
use crate::FileEntry;

#[test]
fn invalid_payload_errors_include_the_declared_range() {
    let entries = [
        FileEntry {
            path: "invalid.bin".to_owned(),
            offset: 2,
            size: 3,
        },
    ];
    let result = materialize_entries(
        b"a",
        &entries,
        Path::new("unused-output-root"),
    );

    assert!(
        matches!(
            result,
            Err(error)
                if error
                    .to_string()
                    .contains("offset 2")
                    && error
                        .to_string()
                        .contains("size 3")
        ),
        "invalid payload errors must retain offset and size evidence"
    );
}

#[test]
fn rejects_invalid_later_payload_before_writing() -> Result<(), String> {
    let root = test_root("invalid-later-payload");
    remove_test_root(&root)?;
    let entries = [
        FileEntry {
            path: "first.bin".to_owned(),
            offset: 0,
            size: 1,
        },
        FileEntry {
            path: "invalid.bin".to_owned(),
            offset: 2,
            size: 1,
        },
    ];
    let result = materialize_entries(
        b"a", &entries, &root,
    );
    let first_exists = root
        .join("first.bin")
        .exists();
    remove_test_root(&root)?;
    match result {
        Err(error)
            if error.kind() == io::ErrorKind::InvalidData && !first_exists =>
        {
            Ok(())
        }
        other => Err(
            format!(
                "invalid later payload must fail before writes, got {other:?} \
                 and first_exists={first_exists}"
            ),
        ),
    }
}
