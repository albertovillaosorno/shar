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
//   - Transactional globally aligned world-package FBX publication.
// - Must-Not:
//   - Publish private reference geometry or merge incompatible level variants.
// - Allows:
//   - Fresh original extraction, coordinate-only joins, package FBX files,
//   - collision inspection, isolated review galleries, and transform manifests.
// - Summary:
//   - Builds independently importable world FBX files at one shared origin.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Transactional globally aligned world-package FBX publication.

use std::fs;
use std::path::Path;

use self::catalog::{counts, write_catalogs};
use self::export::export_world_collection;
use self::inventory::world_packages;
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
mod islands;
mod layout;
mod model;
mod movement;
mod movement_catalog;
mod movement_model;
mod movement_records;
mod scenegraph;
mod transform;

/// Stable world-package stage identity.
const STAGE: &str = "fbx-export-world";

/// Export globally aligned package FBX files for every terrain-world source.
///
/// # Errors
///
/// Returns an error when extraction, coordinate joining, package assembly,
/// verification, cleanup, or atomic publication fails.
pub(in crate::adapters::driven::local) fn export_world_master(
    index_path: &Path,
    game_root: &Path,
    coordinate_root: &Path,
    output_dir: &Path,
) -> Result<StageReport, PipelineError> {
    ensure_missing(
        output_dir,
        "world package output",
    )?;
    let staging = staging_path(output_dir)?;
    ensure_missing(
        &staging,
        "world package staging",
    )?;
    fs::create_dir_all(&staging).map_err(
        |error| {
            PipelineError::new(format!("world package staging failed: {error}"))
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
                        format!("world package publication failed: {error}"),
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
                            "published {} FBX files across {} connected zone ",
                            "families and {} isolated review FBX files from \
                             {} ",
                            "main-level packages; {} horizontally mirrored ",
                            "interiors, {} authored placements, and {} \
                             excluded ",
                            "collision meshes"
                        ),
                        counts.normal_world_fbx_files,
                        counts.narrative_map_groups,
                        counts.review_fbx_files,
                        counts.source_packages,
                        counts.interior_packages,
                        counts.authored_placements,
                        counts.excluded_collision_meshes,
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
        model::WorldCollectionCounts,
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
                    format!("world package normalized staging failed: {error}"),
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
    let packages = world_packages(&index);
    let authority = SharedTextureAuthority::build(
        &index, &canonical,
    )?;
    let collection = export_world_collection(
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
        &collection,
    );
    write_catalogs(
        staging,
        counts,
        &collection,
    )?;
    fs::remove_dir_all(&work).map_err(
        |error| {
            PipelineError::new(format!("world package cleanup failed: {error}"))
        },
    )?;
    let (files, bytes) = inventory(staging)?;
    Ok(
        (
            files, bytes, counts,
        ),
    )
}
