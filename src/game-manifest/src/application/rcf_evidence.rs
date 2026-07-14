// File:
//   - rcf_evidence.rs
// Path:
//   - src/game-manifest/src/application/rcf_evidence.rs
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
//   - Extracted RCF evidence acquisition and completeness invariants.
// - Must-Not:
//   - Render expanded rows, classify payloads, or publish artifacts.
// - Allows:
//   - Inspect the configured root and normalize its file snapshot.
// - Split-When:
//   - Split when archive-to-extraction matching gains an independent model.
// - Merge-When:
//   - Another application module owns the same RCF evidence contract.
// - Summary:
//   - Application boundary for extracted RCF evidence.
// - Description:
//   - Fails closed when source and extracted evidence are inconsistent.
// - Usage:
//   - Called by expanded-manifest generation before record rendering.
// - Defaults:
//   - Source archives require a non-empty extracted directory.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Extracted RCF evidence loading and completeness validation.
//!
//! Source archives and extracted snapshots must remain mutually consistent.

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
    let kind = tree
        .kind(root)
        .map_err(
            |error| {
                ManifestError::io(
                    "inspect",
                    root.to_path_buf(),
                    error,
                )
            },
        )?;
    match kind {
        PathKind::Directory => {}
        PathKind::Missing if !game_has_rcf => return Ok(Vec::new()),
        PathKind::Missing => {
            return Err(
                ManifestError::Invalid(
                    "RCF archives require an extracted RCF directory"
                        .to_owned(),
                ),
            );
        }
        PathKind::File | PathKind::Other => {
            return Err(
                ManifestError::Invalid(
                    "extracted RCF path must be a directory".to_owned(),
                ),
            );
        }
    }
    let mut files = deduplicate_paths(
        tree.files(root)
            .map_err(
                |error| {
                    ManifestError::io(
                        "scan",
                        root.to_path_buf(),
                        error,
                    )
                },
            )?,
    );
    files.retain(|path| extension_of(path) != BACKUP_EXTENSION);
    if game_has_rcf && files.is_empty() {
        return Err(
            ManifestError::Invalid(
                "RCF archives require extracted RCF files".to_owned(),
            ),
        );
    }
    if !game_has_rcf && !files.is_empty() {
        return Err(
            ManifestError::Invalid(
                "extracted RCF files require source RCF archives".to_owned(),
            ),
        );
    }
    Ok(files)
}
