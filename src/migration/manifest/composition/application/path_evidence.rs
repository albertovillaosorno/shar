// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Path evidence application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Path evidence application service.
// - Description:
//   - Implements the declared application service responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Path evidence application service.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::resolve_under;

/// Diagnostic for evidence outside its declared root.
const OUTSIDE_ROOT: &str = "tree snapshot contains a path outside its root";
/// Diagnostic for traversing or otherwise unsafe evidence.
const UNSAFE_PATH: &str = "tree snapshot contains an unsafe path";
/// Diagnostic for aliases that are not normalized coordinates.
const NON_NORMALIZED_PATH: &str =
    "tree snapshot contains a non-normalized path";

/// Removes repeated lexical path evidence while retaining deterministic order.
pub(super) fn deduplicate_paths(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Rejects path evidence outside or non-normalized beneath one root.
///
/// # Errors
///
/// Returns a stable diagnostic when any supplied coordinate is unsafe.
pub(super) fn require_rooted_paths(
    root: &Path,
    files: &[PathBuf],
) -> Result<(), String> {
    for path in files {
        let Ok(relative) = path.strip_prefix(root) else {
            return Err(OUTSIDE_ROOT.to_owned());
        };
        let Ok(resolved) = resolve_under(root, relative) else {
            return Err(UNSAFE_PATH.to_owned());
        };
        if resolved != *path {
            return Err(NON_NORMALIZED_PATH.to_owned());
        }
    }
    Ok(())
}
