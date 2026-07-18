// File:
//   - vehicle_catalog.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/vehicle_catalog.rs
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
//   - Atomic publication of one semantically separated FBX per vehicle package.
// - Must-Not:
//   - Export collision, physics, cameras, locators, particles, or gameplay state.
// - Allows:
//   - Fresh extraction, vehicle assembly, semantic separation, and catalogs.
// - Split-When:
//   - Another model family shares the same publication transaction.
// - Merge-When:
//   - A shared rigid-model catalog owns identical selection and output rules.
// - Summary:
//   - Complete deterministic vehicle FBX catalog publisher.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Complete deterministic vehicle FBX catalog publication.

use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::package::{PhaseThreePackageIndex, PhaseThreePackageRow};
use crate::domain::{PipelineError, StageReport};

mod catalog;
mod model;
mod prepare;
mod source;

use model::VehicleRecord;
use source::VehicleTextureAuthority;

/// Stable stage identity for the vehicle catalog.
const STAGE: &str = "fbx-export-vehicles";
/// Generated package category containing vehicle models.
pub(super) const VEHICLE_CATEGORY: &str = "cars";
/// Shared car package used as dependency evidence, not a standalone vehicle.
pub(super) const VEHICLE_COMMON_SUBCATEGORY: &str = "cars/runtime-base/common";

/// Export every real vehicle package through one atomic root transaction.
///
/// # Errors
///
/// Returns an error when selection, extraction, assembly, serialization,
/// verification, or publication fails.
pub(super) fn export_vehicle_catalog(
    index_path: &Path,
    game_root: &Path,
    output_dir: &Path,
) -> Result<StageReport, PipelineError> {
    ensure_missing(output_dir, "vehicle catalog output")?;
    let staging = staging_path(output_dir)?;
    ensure_missing(&staging, "vehicle catalog staging")?;
    fs::create_dir_all(&staging).map_err(
        |error| PipelineError::new(format!("vehicle staging failed: {error}")),
    )?;
    let result = build_catalog(index_path, game_root, &staging).and_then(
        |(vehicles, files, bytes)| {
            fs::rename(&staging, output_dir).map_err(
                |error| {
                    PipelineError::new(
                        format!("vehicle catalog publication failed: {error}"),
                    )
                },
            )?;
            Ok(StageReport {
                name: STAGE,
                files,
                bytes,
                note: format!(
                    "published {vehicles} semantically separated vehicle FBX files"
                ),
            })
        },
    );
    if result.is_err() {
        drop(fs::remove_dir_all(&staging));
    }
    result
}

/// Build the complete vehicle catalog below one hidden staging root.
fn build_catalog(
    index_path: &Path,
    game_root: &Path,
    staging: &Path,
) -> Result<(usize, usize, u64), PipelineError> {
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let mut packages = index
        .packages()
        .iter()
        .filter(|package| is_vehicle_package(package))
        .collect::<Vec<_>>();
    packages.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    if packages.is_empty() {
        return Err(PipelineError::new("vehicle catalog selection is empty"));
    }
    let work = staging.join(".work");
    let normalized = work.join("normalized");
    fs::create_dir_all(&normalized).map_err(
        |error| PipelineError::new(format!("vehicle work creation failed: {error}")),
    )?;
    let extracted = source::extract_vehicle_packages(
        &index,
        game_root,
        &normalized,
    )?;
    let authority = VehicleTextureAuthority::build(&index, &normalized)?;
    let mut records = Vec::<VehicleRecord>::with_capacity(packages.len());
    for package in packages {
        records.push(prepare::export_vehicle(
            package,
            &normalized,
            staging,
            &authority,
        )?);
    }
    catalog::write_root_catalog(staging, &records, extracted)?;
    fs::remove_dir_all(&work).map_err(
        |error| PipelineError::new(format!("vehicle work cleanup failed: {error}")),
    )?;
    let (files, bytes) = catalog::tree_totals(staging)?;
    Ok((records.len(), files, bytes))
}

/// Return whether one generated row represents a standalone vehicle artifact.
fn is_vehicle_package(package: &PhaseThreePackageRow) -> bool {
    package.category == VEHICLE_CATEGORY
        && package.subcategory != VEHICLE_COMMON_SUBCATEGORY
        && package.members().iter().any(
            |member| {
                member.kind == "p3d-composite-drawable"
                    && member.source_chunk_kind == "composite_drawable"
            },
        )
        && package.members().iter().any(
            |member| {
                member.kind == "p3d-mesh" && member.source_chunk_kind == "mesh"
            },
        )
}

/// Reject one pre-existing output or hidden staging path.
fn ensure_missing(path: &Path, label: &str) -> Result<(), PipelineError> {
    if path.exists() {
        return Err(PipelineError::new(format!(
            "{label} already exists: {}",
            path.display()
        )));
    }
    Ok(())
}

/// Derive one hidden sibling staging path for atomic publication.
fn staging_path(output_dir: &Path) -> Result<PathBuf, PipelineError> {
    let parent = output_dir
        .parent()
        .ok_or_else(|| PipelineError::new("vehicle output has no parent"))?;
    let name = output_dir
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| PipelineError::new("vehicle output has no UTF-8 name"))?;
    Ok(parent.join(format!(".{name}.staging")))
}
