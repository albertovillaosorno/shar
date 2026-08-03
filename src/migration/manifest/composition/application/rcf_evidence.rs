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
//   - Rcf evidence application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Rcf evidence application service.
// - Description:
//   - Implements the declared application service responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Rcf evidence application service.

use std::path::{Path, PathBuf};

use super::ManifestError;
use super::path_evidence::deduplicate_paths;
use crate::domain::{BACKUP_EXTENSION, extension_of};
use crate::ports::{GameTree, PathKind};

/// Loads and validates the extracted RCF snapshot for source archives.
pub(super) fn load_extracted_rcf_files(
    tree: &impl GameTree,
    root: &Path,
    game_has_rcf: bool,
) -> Result<Vec<PathBuf>, ManifestError> {
    let kind = tree.kind(root).map_err(|error| {
        ManifestError::io("inspect", root.to_path_buf(), error)
    })?;
    match kind {
        PathKind::Directory => {},
        PathKind::Missing if !game_has_rcf => return Ok(Vec::new()),
        PathKind::Missing => {
            return Err(ManifestError::Invalid(
                "RCF archives require an extracted RCF directory".to_owned(),
            ));
        },
        PathKind::File | PathKind::Other => {
            return Err(ManifestError::Invalid(
                "extracted RCF path must be a directory".to_owned(),
            ));
        },
    }
    let mut files = deduplicate_paths(tree.files(root).map_err(|error| {
        ManifestError::io("scan", root.to_path_buf(), error)
    })?);
    files.retain(|path| extension_of(path) != BACKUP_EXTENSION);
    if game_has_rcf && files.is_empty() {
        return Err(ManifestError::Invalid(
            "RCF archives require extracted RCF files".to_owned(),
        ));
    }
    if !game_has_rcf && !files.is_empty() {
        return Err(ManifestError::Invalid(
            "extracted RCF files require source RCF archives".to_owned(),
        ));
    }
    Ok(files)
}
