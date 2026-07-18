// File:
//   - world_level.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level.rs
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
//   - Transactional separated master-world FBX publication.
// - Must-Not:
//   - Publish private reference geometry or replace later runtime world
//     assembly.
// - Allows:
//   - Fresh original extraction, coordinate-only joins, collision review,
//   - similarity-overlaid definitions, catalogs, and atomic output.
// - Summary:
//   - Builds one separated connected-coordinate master-world FBX.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Transactional separated master-world FBX publication.

use std::fs;
use std::path::Path;

use self::catalog::{counts, write_catalog};
use self::export::export_master_scene;
use self::inventory::packages_by_level;
use super::catalog::inventory;
use super::extraction::{
    extract_world_level_coordinate_packages, extract_world_level_packages,
};
use super::texture_authority::SharedTextureAuthority;
use super::{ensure_missing, staging_path};
use crate::domain::package::PhaseThreePackageIndex;
use crate::domain::{PipelineError, StageReport};

mod catalog;
mod collision;
mod coordinate;
mod export;
mod inventory;
mod model;
mod scenegraph;
mod transform;

/// Stable master-world stage identity.
const STAGE: &str = "fbx-export-world";

/// Export one separated static master FBX for all seven main game levels.
///
/// # Errors
///
/// Returns an error when extraction, coordinate joining, assembly, review-layer
/// placement, verification, cleanup, or atomic publication fails.
pub(in crate::adapters::driven::local) fn export_world_master(
    index_path: &Path,
    game_root: &Path,
    coordinate_root: &Path,
    output_dir: &Path,
) -> Result<StageReport, PipelineError> {
    ensure_missing(
        output_dir,
        "world master output",
    )?;
    let staging = staging_path(output_dir)?;
    ensure_missing(
        &staging,
        "world master staging",
    )?;
    fs::create_dir_all(&staging).map_err(
        |error| {
            PipelineError::new(format!("world master staging failed: {error}"))
        },
    )?;
    let result = build(
        index_path,
        game_root,
        coordinate_root,
        &staging,
    )
    .and_then(
        |(files, bytes, counts)| {
            fs::rename(
                &staging, output_dir,
            )
            .map_err(
                |error| {
                    PipelineError::new(
                        format!("world master publication failed: {error}"),
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
                            "published one separated master-world FBX from {} \
                             levels ",
                            "and {} packages: {} coordinate references, {} \
                             fallbacks, ",
                            "{} interiors, {} source meshes, {} authored \
                             placements, ",
                            "{} review definitions in {} similarity groups, \
                             {} ",
                            "collision meshes, and {} reference-position \
                             collisions"
                        ),
                        counts.source_levels,
                        counts.source_packages,
                        counts.coordinate_reference_packages,
                        counts.coordinate_fallback_packages,
                        counts.interior_packages,
                        counts.source_meshes,
                        counts.authored_placements,
                        counts.review_definitions,
                        counts.review_similarity_groups,
                        counts.collision_meshes,
                        counts.reference_collision_meshes,
                    ),
                },
            )
        },
    );
    if result.is_err() {
        drop(fs::remove_dir_all(&staging));
    }
    result
}

/// Build and verify the complete staging publication.
fn build(
    index_path: &Path,
    game_root: &Path,
    coordinate_root: &Path,
    staging: &Path,
) -> Result<
    (
        usize,
        u64,
        model::WorldMasterCounts,
    ),
    PipelineError,
> {
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let work = staging.join(".work");
    let canonical = work.join("canonical");
    let coordinates = work.join("coordinates");
    let scratch = work.join("materials");
    for directory in [
        &canonical,
        &coordinates,
    ] {
        fs::create_dir_all(directory).map_err(
            |error| {
                PipelineError::new(
                    format!("world master normalized staging failed: {error}"),
                )
            },
        )?;
    }
    let source_packages = extract_world_level_packages(
        &index, game_root, &canonical,
    )?;
    let reference_packages = extract_world_level_coordinate_packages(
        &index,
        coordinate_root,
        &coordinates,
    )?;
    let packages = packages_by_level(&index)?;
    let authority = SharedTextureAuthority::build(
        &index, &canonical,
    )?;
    let master = export_master_scene(
        &packages,
        &canonical,
        &coordinates,
        &reference_packages,
        &scratch,
        staging,
        &authority,
    )?;
    let counts = counts(
        source_packages,
        &master,
    );
    write_catalog(
        staging, counts, &master,
    )?;
    fs::remove_dir_all(&work).map_err(
        |error| {
            PipelineError::new(format!("world master cleanup failed: {error}"))
        },
    )?;
    let (files, bytes) = inventory(staging)?;
    Ok(
        (
            files, bytes, counts,
        ),
    )
}
