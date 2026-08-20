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
//   - Observed manifest application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Observed manifest application service.
// - Description:
//   - Implements the declared application service responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Observed manifest application service.

use std::path::Path;

use super::ManifestError;
use super::generate_manifest::load_counts;
use crate::domain::{DirCount, classify_manifest_bucket};
use crate::ports::{GameTree, PathKind};

/// Public-safe observed count rows from one read-only source scan.
#[derive(Debug)]
pub struct ObserveManifestReport {
    /// Deterministically ordered structural count rows.
    pub rows: Vec<DirCount>,
    /// Sum of observed countable files across rows.
    pub total_files: usize,
}

/// Stateless read-only source count observation use case.
#[derive(Debug, Clone, Copy)]
pub struct ObserveManifest;

impl ObserveManifest {
    /// Observes one lawful source tree without publishing into that tree.
    ///
    /// # Errors
    ///
    /// Returns a typed path, required-evidence, or classification failure.
    pub fn execute(
        tree: &impl GameTree,
        game_dir: &Path,
    ) -> Result<ObserveManifestReport, ManifestError> {
        let game_kind = tree.kind(game_dir).map_err(|error| {
            ManifestError::io("inspect", game_dir.to_path_buf(), error)
        })?;
        if game_kind != PathKind::Directory {
            return Err(ManifestError::Invalid(
                "game directory is unavailable for observation".to_owned(),
            ));
        }
        let counts = load_counts(tree, game_dir)?;
        let mut rows = Vec::with_capacity(counts.len());
        let mut total_files = 0_usize;
        for ((dir, extension), count) in counts {
            let kind = classify_manifest_bucket(&dir, &extension);
            if kind == "error" {
                return Err(ManifestError::Invalid(
                    "source observation contains an unclassified bucket"
                        .to_owned(),
                ));
            }
            total_files = total_files.saturating_add(count);
            rows.push(DirCount {
                dir,
                extension,
                min_count: count,
                kind,
            });
        }
        Ok(ObserveManifestReport { rows, total_files })
    }
}
