// File:
//   - filesystem.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/filesystem.rs
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
//   - The filesystem contract for pipeline phase.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute filesystem.
// - Split-When:
//   - Split when filesystem contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Shared filesystem traversal for pipeline phases.
// - Description:
//   - Defines filesystem data and behavior for pipeline phase.
// - Usage:
//   - Used by pipeline phase code that needs filesystem.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Shared filesystem traversal for pipeline phases.
//!
//! This boundary keeps shared filesystem traversal for pipeline phases
//! explicit and returns deterministic results to pipeline callers.

use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local::{
    path_kind, regular_files,
};

use crate::domain::PipelineError;

/// Walks files with one ordering-independent implementation so extraction,
/// audit, and straggler phases cannot drift in hidden-file behavior.
///
/// # Errors
///
/// Returns an error when a directory cannot be listed or an entry cannot be
/// inspected.
pub(super) fn collect_files(
    root: &Path
) -> Result<Vec<PathBuf>, PipelineError> {
    match path_kind(root).map_err(
        |error| {
            PipelineError::new(
                format!(
                    "failed to inspect {}: {error}",
                    root.display()
                ),
            )
        },
    )? {
        PathKind::Directory => regular_files(root).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "failed to list {}: {error}",
                        root.display()
                    ),
                )
            },
        ),
        PathKind::Missing | PathKind::File | PathKind::Other => Ok(Vec::new()),
    }
}

#[cfg(test)]
#[path = "filesystem_tests.rs"]
mod tests;
