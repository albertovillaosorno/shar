// File:
//   - generate_manifest.rs
// Path:
//   - src/game-manifest/src/application/generate_manifest.rs
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
//   - Minimum manifest generation and complete publication sequence.
// - Must-Not:
//   - Traverse filesystems, print diagnostics, or select concrete adapters.
// - Allows:
//   - Count supplied tree evidence, classify records, and publish one artifact.
// - Split-When:
//   - Split when rendering and publication become independent commands.
// - Merge-When:
//   - Another use case owns the same minimum-manifest command.
// - Summary:
//   - Application command for minimum manifest generation.
// - Description:
//   - Produces canonical JSONL from explicit game tree evidence.
// - Usage:
//   - Invoked by driving adapters with tree and text-store ports.
// - Defaults:
//   - Optional root LMLM and PNG coordinates have zero minimums.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application command for canonical minimum manifest generation.
//!
//! Tree evidence is classified completely before one artifact is published.
use std::path::{Path, PathBuf};

use super::ManifestError;
use super::path_evidence::require_rooted_paths;
use crate::domain::{
    DirCount, DirExtCounts, GENERATED_IMAGE_EXTENSION, MANIFEST_FILE_NAME,
    OPTIONAL_EXTENSION, classify_manifest_bucket, count_by_dir_ext_paths,
    kind_taxonomy_jsonl,
};
use crate::ports::{GameTree, PathKind, TextArtifactStore};

/// Evidence returned after one successful manifest generation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateManifestReport {
    /// Number of folder/type records written.
    pub record_count: usize,
    /// Sum of required files across records.
    pub total_files: usize,
    /// Explicit manifest output path.
    pub manifest_path: PathBuf,
}

/// Stateless minimum-manifest generation use case.
#[derive(Debug, Clone, Copy)]
pub struct GenerateManifest;

impl GenerateManifest {
    /// Generates and publishes one canonical minimum manifest.
    ///
    /// # Errors
    ///
    /// Returns a typed path or classification failure.
    pub fn execute(
        tree: &impl GameTree,
        store: &impl TextArtifactStore,
        game_dir: &Path,
    ) -> Result<GenerateManifestReport, ManifestError> {
        let game_kind = tree
            .kind(game_dir)
            .map_err(
                |error| {
                    ManifestError::io(
                        "inspect",
                        game_dir.to_path_buf(),
                        error,
                    )
                },
            )?;
        if game_kind != PathKind::Directory {
            return Err(
                ManifestError::Invalid(
                    format!(
                        "game directory not found: {}",
                        super::diagnostic_path::escaped_path(game_dir)
                    ),
                ),
            );
        }
        let mut counts = load_counts(
            tree, game_dir,
        )?;
        require_required_evidence(&counts)?;
        let _optional_previous = counts.insert(
            (
                String::new(),
                OPTIONAL_EXTENSION.to_owned(),
            ),
            0,
        );
        let _generated_previous = counts.insert(
            (
                String::new(),
                GENERATED_IMAGE_EXTENSION.to_owned(),
            ),
            0,
        );
        let mut text = kind_taxonomy_jsonl();
        text.push('\n');
        let mut total_files = 0_usize;
        for ((dir, extension), min_count) in &counts {
            total_files = total_files.saturating_add(*min_count);
            let record_kind = classify_manifest_bucket(
                dir, extension,
            );
            if record_kind == "error" {
                return Err(
                    ManifestError::Invalid(
                        format!("unclassified bucket: {dir} .{extension}"),
                    ),
                );
            }
            text.push_str(
                &DirCount {
                    dir: dir.clone(),
                    extension: extension.clone(),
                    min_count: *min_count,
                    kind: record_kind,
                }
                .to_jsonl(),
            );
            text.push('\n');
        }
        let manifest_path = game_dir.join(MANIFEST_FILE_NAME);
        store
            .write(
                &manifest_path,
                &text,
            )
            .map_err(
                |error| {
                    ManifestError::io(
                        "write",
                        manifest_path.clone(),
                        error,
                    )
                },
            )?;
        Ok(
            GenerateManifestReport {
                record_count: counts.len(),
                total_files,
                manifest_path,
            },
        )
    }
}

/// Loads, validates, and counts one complete game snapshot.
fn load_counts(
    tree: &impl GameTree,
    game_dir: &Path,
) -> Result<DirExtCounts, ManifestError> {
    let files = tree
        .files(game_dir)
        .map_err(
            |error| {
                ManifestError::io(
                    "scan",
                    game_dir.to_path_buf(),
                    error,
                )
            },
        )?;
    require_rooted_paths(
        game_dir, &files,
    )
    .map_err(ManifestError::Invalid)?;
    Ok(
        count_by_dir_ext_paths(
            game_dir, &files,
        ),
    )
}

/// Rejects manifests that would contain only synthetic optional coordinates.
fn require_required_evidence(
    counts: &DirExtCounts
) -> Result<(), ManifestError> {
    if counts.is_empty() {
        return Err(
            ManifestError::Invalid(
                "game directory contains no required files".to_owned(),
            ),
        );
    }
    Ok(())
}
