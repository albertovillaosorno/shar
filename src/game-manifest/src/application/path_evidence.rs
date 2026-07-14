// File:
//   - path_evidence.rs
// Path:
//   - src/game-manifest/src/application/path_evidence.rs
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
//   - Deterministic normalization and rooted validation of path snapshots.
// - Must-Not:
//   - Traverse filesystems, classify files, or select evidence roots.
// - Allows:
//   - Reject unsafe coordinates and remove repeated lexical paths.
// - Split-When:
//   - Split when another normalization invariant becomes independently used.
// - Merge-When:
//   - Another application module owns identical snapshot normalization.
// - Summary:
//   - Shared application path-evidence normalization.
// - Description:
//   - Validates and canonicalizes caller-supplied paths without external IO.
// - Usage:
//   - Used by manifest commands before classification and rendering.
// - Defaults:
//   - Lexically identical paths represent one physical evidence item.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Deterministic path-snapshot validation for application use cases.
//!
//! Unsafe coordinates fail closed and repeated lexical paths collapse.

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
        let Ok(resolved) = resolve_under(
            root, relative,
        ) else {
            return Err(UNSAFE_PATH.to_owned());
        };
        if resolved != *path {
            return Err(NON_NORMALIZED_PATH.to_owned());
        }
    }
    Ok(())
}
