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
//   - Vehicle catalog outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Vehicle catalog outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Vehicle catalog outbound adapter.

use std::fs;
use std::path::{Path, PathBuf};

use crate::adapters::driven::check_cancellation;
use crate::adapters::driven::local::progress::StageProgress;
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
    fs::create_dir_all(&staging).map_err(|error| {
        PipelineError::new(format!("vehicle staging failed: {error}"))
    })?;
    let result = build_catalog(index_path, game_root, &staging).and_then(
        |(vehicles, files, bytes)| {
            fs::rename(&staging, output_dir).map_err(|error| {
                PipelineError::new(format!(
                    "vehicle catalog publication failed: {error}"
                ))
            })?;
            Ok(StageReport {
                name: STAGE,
                files,
                bytes,
                note: format!(
                    "published {vehicles} semantically separated vehicle \
                         FBX files"
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
    check_cancellation()?;
    let mut index_progress =
        StageProgress::begin("vehicle catalog index load", 1);
    index_progress.advance(&index_path.to_string_lossy());
    let index = PhaseThreePackageIndex::read(index_path)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    index_progress.finish();
    check_cancellation()?;
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
    fs::create_dir_all(&normalized).map_err(|error| {
        PipelineError::new(format!("vehicle work creation failed: {error}"))
    })?;
    let extracted =
        source::extract_vehicle_packages(&index, game_root, &normalized)?;
    let authority = VehicleTextureAuthority::build(&index, &normalized)?;
    let mut records = Vec::<VehicleRecord>::with_capacity(packages.len());
    let mut progress =
        StageProgress::begin("vehicle FBX assembly", packages.len());
    for package in packages {
        check_cancellation()?;
        progress.advance(&package.package_id);
        records.push(prepare::export_vehicle(
            package,
            &normalized,
            staging,
            &authority,
        )?);
    }
    progress.finish();
    catalog::write_root_catalog(staging, &records, extracted)?;
    fs::remove_dir_all(&work).map_err(|error| {
        PipelineError::new(format!("vehicle work cleanup failed: {error}"))
    })?;
    let (files, bytes) = catalog::tree_totals(staging)?;
    Ok((records.len(), files, bytes))
}

/// Return whether one generated row represents a standalone vehicle artifact.
fn is_vehicle_package(package: &PhaseThreePackageRow) -> bool {
    package.category == VEHICLE_CATEGORY
        && package.subcategory != VEHICLE_COMMON_SUBCATEGORY
        && package.members().iter().any(|member| {
            member.kind == "p3d-composite-drawable"
                && member.source_chunk_kind == "composite_drawable"
        })
        && package.members().iter().any(|member| {
            member.kind == "p3d-mesh" && member.source_chunk_kind == "mesh"
        })
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
        .ok_or_else(|| {
            PipelineError::new("vehicle output has no UTF-8 name")
        })?;
    Ok(parent.join(format!(".{name}.staging")))
}
