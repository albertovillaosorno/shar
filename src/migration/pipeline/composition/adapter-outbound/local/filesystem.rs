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
//   - Filesystem outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem outbound adapter.

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
    root: &Path,
) -> Result<Vec<PathBuf>, PipelineError> {
    match path_kind(root).map_err(|error| {
        PipelineError::new(format!(
            "failed to inspect {}: {error}",
            schoenwald_filesystem::DiagnosticPath::new(root)
        ))
    })? {
        PathKind::Directory => regular_files(root).map_err(|error| {
            PipelineError::new(format!(
                "failed to list {}: {error}",
                schoenwald_filesystem::DiagnosticPath::new(root)
            ))
        }),
        PathKind::Missing | PathKind::File | PathKind::Other => Ok(Vec::new()),
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/filesystem_tests.rs"]
mod tests;
