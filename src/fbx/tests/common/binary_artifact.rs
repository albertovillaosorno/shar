// File:
//   - binary_artifact.rs
// Path:
//   - src/fbx/tests/common/binary_artifact.rs
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
//   - Shared filesystem evidence helpers for binary FBX integration tests.
// - Must-Not:
//   - Construct domain fixtures, inspect private assets, or invoke Blender.
// - Allows:
//   - Reading paired temporary artifacts and removing them after assertions.
// - Split-When:
//   - Another test helper gains an independent lifecycle or evidence contract.
// - Merge-When:
//   - Binary FBX tests no longer share filesystem artifact handling.
// - Summary:
//   - Deduplicates deterministic paired-artifact reads for FBX regressions.
// - Description:
//   - Reads two generated files, reports read failures, and cleans both paths.
// - Usage:
//   - Imported by binary character and animation integration tests.
// - Defaults:
//   - Returns no bytes after any failed read and still attempts cleanup.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Shared filesystem evidence helpers for binary FBX integration tests.
//!
//! The helper keeps paired deterministic artifact reads and cleanup behavior
//! identical across static-character and animated-character regressions.

use std::fs;
use std::path::Path;

/// Read two generated artifacts and remove both temporary files.
#[must_use]
pub(super) fn read_binary_pair(
    first_path: &Path,
    second_path: &Path,
    label: &str,
) -> Option<(
    Vec<u8>,
    Vec<u8>,
)> {
    let first_result = fs::read(first_path);
    assert!(
        first_result.is_ok(),
        "first {label} should be readable: {first_result:?}"
    );
    let first = first_result.ok()?;
    let second_result = fs::read(second_path);
    assert!(
        second_result.is_ok(),
        "second {label} should be readable: {second_result:?}"
    );
    let second = second_result.ok()?;
    assert!(
        fs::remove_file(first_path).is_ok(),
        "first {label} temporary file should be removable"
    );
    assert!(
        fs::remove_file(second_path).is_ok(),
        "second {label} temporary file should be removable"
    );
    Some(
        (
            first, second,
        ),
    )
}
