// File:
//   - world.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world.rs
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
//   - Complete terrain-world model-prop batch orchestration.
// - Must-Not:
//   - Publish terrain placement or native Unreal assets.
// - Summary:
//   - Builds the hash-free props/world catalog transactionally.
//
// Large file:
//   - false

//! Complete world model-prop batch.

use std::fs;
use std::path::Path;

use super::catalog::inventory;
use super::extraction::extract_world_packages;
use super::texture_authority::SharedTextureAuthority;
use super::world_catalog::{world_counts, write_world_catalog};
use super::world_export::export_world_props;
use super::world_inventory::discover_world_candidates;
use crate::domain::package::PhaseThreePackageIndex;
use crate::domain::{PipelineError, StageReport};

/// Stable complete world-prop batch stage name.
const STAGE: &str = "fbx-export-world-props";

/// Export every standalone terrain-world model prop under hash-free names.
///
/// # Errors
///
/// Returns an error when staging, extraction, export, or publication fails.
pub(in crate::adapters::driven::local) fn export_world_prop_catalog(
    index_path: &Path,
    game_root: &Path,
    output_dir: &Path,
) -> Result<StageReport, PipelineError> {
    super::ensure_missing(
        output_dir,
        "world prop catalog output",
    )?;
    let staging = super::staging_path(output_dir)?;
    super::ensure_missing(
        &staging,
        "world prop catalog staging",
    )?;
    fs::create_dir_all(&staging).map_err(
        |error| {
            PipelineError::new(format!("world prop staging failed: {error}"))
        },
    )?;
    let result = build(
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
                        format!("world prop publication failed: {error}"),
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
                            "published {} hash-free world props from {} ",
                            "occurrences: {} static, {} animated, {} merged ",
                            "variants, {} omitted visual variants"
                        ),
                        counts.assets,
                        counts.occurrences,
                        counts.static_assets,
                        counts.animated_assets,
                        counts.merged_variants,
                        counts.omitted_variants
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

/// Build one complete verified world-prop staging catalog.
///
/// # Errors
///
/// Returns an error when extraction, discovery, export, cataloging, or cleanup
/// fails.
fn build(
    index_path: &Path,
    game_root: &Path,
    staging: &Path,
) -> Result<
    (
        usize,
        u64,
        super::world_model::WorldCatalogCounts,
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
                format!("world prop normalization failed: {error}"),
            )
        },
    )?;
    let source_packages = extract_world_packages(
        &index,
        game_root,
        &normalized,
    )?;
    let candidates = discover_world_candidates(
        &index,
        &normalized,
    )?;
    let authority = SharedTextureAuthority::build(
        &index,
        &normalized,
    )?;
    let assets = export_world_props(
        &candidates,
        &normalized,
        &scratch,
        &authority,
        staging,
    )?;
    let counts = world_counts(
        source_packages,
        candidates.len(),
        &assets,
    );
    write_world_catalog(
        staging, counts, &assets,
    )?;
    fs::remove_dir_all(&work).map_err(
        |error| {
            PipelineError::new(
                format!("world prop work cleanup failed: {error}"),
            )
        },
    )?;
    let (files, bytes) = inventory(staging)?;
    Ok(
        (
            files, bytes, counts,
        ),
    )
}
