// File:
//   - prop_catalog.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog.rs
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
//   - Complete non-world model-prop batch.
// - Must-Not:
//   - Export non-model evidence to FBX or publish partial catalog directories.
// - Allows:
//   - Re-extraction, discovery, semantic deduplication, verification, and
//     atomic
//   - local publication.
// - Split-When:
//   - Batch orchestration gains a second independently publishable transaction.
// - Merge-When:
//   - A generic complete model catalog owns the same staging lifecycle.
// - Summary:
//   - Publishes card and mission model props in one deterministic run.
// - Description:
//   - Leaves physics, placement, cameras, effects, and gameplay for Phase 6.
// - Usage:
//   - Called by the `fbx-export-props` pipeline command.
// - Defaults:
//   - Output and sibling hidden staging paths must not already exist.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md
//
// Large file:
//   - false
//

//! Complete non-world model-prop batch.

use std::fs;
use std::path::{Path, PathBuf};

use self::catalog::{catalog_counts, inventory, write_catalog};
use self::export::export_unique_props;
use self::source::extract_and_discover;
use crate::domain::package::PhaseThreePackageIndex;
use crate::domain::{PipelineError, StageReport};

mod canonical;
mod catalog;
mod export;
mod extraction;
mod inventory_common;
mod material;
mod model;
mod non_world_inventory;
mod prepare;
mod prepared;
mod source;
mod texture_authority;
mod world;
mod world_catalog;
mod world_export;
mod world_inventory;
mod world_ledger;
mod world_level;
mod world_model;

/// Stable complete prop batch stage name.
const STAGE: &str = "fbx-export-props";

/// Export every model-bearing card and mission prop in one batch.
///
/// # Errors
///
/// Returns an error when staged extraction, discovery, export, catalog
/// rendering, cleanup, or atomic publication fails.
pub(super) fn export_prop_catalog(
    index_path: &Path,
    game_root: &Path,
    output_dir: &Path,
) -> Result<StageReport, PipelineError> {
    ensure_missing(
        output_dir,
        "prop catalog output",
    )?;
    let staging = staging_path(output_dir)?;
    ensure_missing(
        &staging,
        "prop catalog staging",
    )?;
    fs::create_dir_all(&staging).map_err(
        |error| {
            PipelineError::new(format!("prop catalog staging failed: {error}"))
        },
    )?;
    let result = build_catalog(
        index_path, game_root, &staging,
    )
    .and_then(
        |(files, bytes, counts)| {
            fs::rename(
                &staging, output_dir,
            )
            .map_err(
                |error| {
                    PipelineError::new(
                        format!("prop catalog publication failed: {error}"),
                    )
                },
            )?;
            Ok(
                StageReport {
                    name: STAGE,
                    files,
                    bytes,
                    note: format!(
                        concat!(
                            "published {} unique non-world model props from ",
                            "{} occurrences across {} source packages: {} ",
                            "cards, {} missions, {} static, {} rigid animated"
                        ),
                        counts.assets,
                        counts.occurrences,
                        counts.source_packages,
                        counts.card_assets,
                        counts.mission_assets,
                        counts.static_assets,
                        counts.animated_assets
                    ),
                },
            )
        },
    );
    if result.is_err() {
        let _cleanup_result = fs::remove_dir_all(&staging);
    }
    result
}

/// Build the complete verified staging catalog.
fn build_catalog(
    index_path: &Path,
    game_root: &Path,
    staging: &Path,
) -> Result<
    (
        usize,
        u64,
        model::PropCatalogCounts,
    ),
    PipelineError,
> {
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let work = staging.join(".work");
    let normalized = work.join("normalized");
    let scratch = work.join("materials");
    fs::create_dir_all(&normalized).map_err(
        |error| {
            PipelineError::new(
                format!("prop normalized staging failed: {error}"),
            )
        },
    )?;
    let (source_packages, candidates) = extract_and_discover(
        &index,
        game_root,
        &normalized,
    )?;
    let assets = export_unique_props(
        &candidates,
        &normalized,
        &scratch,
        staging,
    )?;
    let counts = catalog_counts(
        source_packages,
        candidates.len(),
        &assets,
    );
    write_catalog(
        staging, counts, &assets,
    )?;
    fs::remove_dir_all(&work).map_err(
        |error| {
            PipelineError::new(format!("prop work cleanup failed: {error}"))
        },
    )?;
    let (files, bytes) = inventory(staging)?;
    Ok(
        (
            files, bytes, counts,
        ),
    )
}

pub(super) use world::export_world_prop_catalog;
pub(super) use world_level::export_world_master;

/// Require one output or staging path not to exist.
fn ensure_missing(
    path: &Path,
    label: &str,
) -> Result<(), PipelineError> {
    if path.exists() {
        return Err(
            PipelineError::new(
                format!(
                    "{label} already exists: {}",
                    path.display()
                ),
            ),
        );
    }
    Ok(())
}

/// Build one sibling hidden staging directory path.
fn staging_path(output_dir: &Path) -> Result<PathBuf, PipelineError> {
    let name = output_dir
        .file_name()
        .ok_or_else(
            || {
                PipelineError::new(
                    "prop catalog output has no final path segment",
                )
            },
        )?;
    let staging_name = format!(
        ".{}.staging",
        name.to_string_lossy()
    );
    Ok(
        output_dir
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(staging_name),
    )
}
