// File:
//   - catalog.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/vehicle_catalog/catalog.rs
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
//   - Vehicle-local and root catalog serialization plus publication totals.
// - Must-Not:
//   - Select packages, decode P3D, or assemble FBX geometry.
// - Allows:
//   - Deterministic JSON, recursive inventory, and create-new writes.
// - Summary:
//   - Vehicle publication catalog writer.
//
// Large file:
//   - false
//

//! Vehicle publication catalog writer.

use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};

use super::model::VehicleRecord;
use crate::domain::PipelineError;

/// Write one deterministic vehicle-local catalog.
pub(super) fn write_vehicle_catalog(
    vehicle_dir: &Path,
    record: &VehicleRecord,
) -> Result<(), PipelineError> {
    let bytes = serde_json::to_vec_pretty(&vehicle_json(record))
        .map_err(|error| PipelineError::new(error.to_string()))?;
    write_new(
        &vehicle_dir.join("vehicle.catalog.json"),
        &bytes,
    )
}

/// Write the deterministic root vehicle catalog.
pub(super) fn write_root_catalog(
    staging: &Path,
    records: &[VehicleRecord],
    extracted_packages: usize,
) -> Result<(), PipelineError> {
    let value = json!({
        "schema": "shar.vehicle-catalog.v1",
        "boundary": {
            "source": concat!(
                "original game P3D packages selected by the generated package ",
                "index"
            ),
            "fbx": [
                "render geometry split into semantic objects",
                "authored render skeleton and component pivots",
                "package-local skeletal animation clips",
                "external normal-state texture bindings"
            ],
            "sidecars": [
                "decoded shader parameters",
                "damage textures",
                "alternate appearance textures",
                "semantic part roles and pivot bones"
            ],
            "excluded": [
                "collision and physics",
                "cameras and follow-camera data",
                "locators and triggers",
                "quad-group flares and particles",
                "gameplay state and tuning"
            ]
        },
        "counts": {
            "vehicles": records.len(),
            "freshly_extracted_car_packages": extracted_packages,
            "parts": records
                .iter()
                .map(|record| record.parts.len())
                .sum::<usize>(),
            "deferred_geometry": records
                .iter()
                .map(|record| record.deferred_geometry.len())
                .sum::<usize>(),
            "skeletal_animations": records
                .iter()
                .map(|record| record.animations.len())
                .sum::<usize>(),
            "effect_animation_sidecars": records
                .iter()
                .map(|record| record.effect_animation_sidecars.len())
                .sum::<usize>(),
            "textures": records
                .iter()
                .map(|record| record.textures.len())
                .sum::<usize>(),
            "shaders": records
                .iter()
                .map(|record| record.shaders.len())
                .sum::<usize>()
        },
        "vehicles": records.iter().map(vehicle_json).collect::<Vec<_>>()
    });
    let bytes = serde_json::to_vec_pretty(&value)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    write_new(
        &staging.join("vehicles.catalog.json"),
        &bytes,
    )
}

/// Render one vehicle record to stable JSON.
fn vehicle_json(record: &VehicleRecord) -> Value {
    json!({
        "vehicle": record.vehicle,
        "package_id": record.package_id,
        "subcategory": record.subcategory,
        "fbx": {
            "path": record.fbx_path,
            "bytes": record.fbx_bytes,
            "sha256": record.fbx_sha256,
            "geometries": record.summary.geometries,
            "bones": record.summary.bones,
            "clusters": record.summary.clusters,
            "materials": record.summary.materials,
            "textures": record.summary.textures,
            "animations": record.summary.animations
        },
        "parts": record.parts.iter().map(|part| json!({
            "name": part.name,
            "source_mesh": part.source_mesh,
            "role": part.role,
            "shader": part.shader,
            "bones": part.bones
        })).collect::<Vec<_>>(),
        "deferred_geometry": record.deferred_geometry,
        "animations": record.animations,
        "effect_animation_sidecars": record.effect_animation_sidecars,
        "textures": record.textures.iter().map(|texture| json!({
            "path": texture.path,
            "role": texture.role,
            "bytes": texture.bytes,
            "sha256": texture.sha256
        })).collect::<Vec<_>>(),
        "shaders": record.shaders
    })
}

/// Return every file recursively below one optional root.
pub(super) fn recursive_files(
    root: &Path
) -> Result<Vec<PathBuf>, PipelineError> {
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory)
            .map_err(|error| PipelineError::new(error.to_string()))?
        {
            let path = entry
                .map_err(|error| PipelineError::new(error.to_string()))?
                .path();
            if path.is_dir() {
                pending.push(path);
            } else if path.is_file() {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

/// Return complete file and byte totals below one publication root.
pub(super) fn tree_totals(
    root: &Path
) -> Result<
    (
        usize,
        u64,
    ),
    PipelineError,
> {
    let files = recursive_files(root)?;
    let mut bytes = 0_u64;
    for path in &files {
        bytes = bytes
            .checked_add(
                fs::metadata(path)
                    .map_err(|error| PipelineError::new(error.to_string()))?
                    .len(),
            )
            .ok_or_else(
                || PipelineError::new("vehicle byte total overflowed"),
            )?;
    }
    Ok(
        (
            files.len(),
            bytes,
        ),
    )
}

/// Write one create-new deterministic artifact.
pub(super) fn write_new(
    path: &Path,
    bytes: &[u8],
) -> Result<(), PipelineError> {
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "write {} failed: {error}",
                        path.display()
                    ),
                )
            },
        )?;
    file.write_all(bytes)
        .map_err(|error| PipelineError::new(error.to_string()))
}
