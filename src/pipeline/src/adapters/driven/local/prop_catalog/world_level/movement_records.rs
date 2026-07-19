// File:
//   - movement_records.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     movement_records.rs
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
//   - Deterministic intake of decoded non-mesh coordinate records.
// - Must-Not:
//   - Modify source JSON, select movement policy, or serialize final catalogs.
// - Allows:
//   - Read locators, lights, and static physics into typed movement evidence.
// - Summary:
//   - Applies one movement to decoded world-coordinate evidence.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Decoded coordinate-record intake for one authored package movement.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use self::light::collect_lights;
use self::locator::collect_locators;
use self::physics::collect_physics;
use super::movement_model::WorldMovedCoordinateRecord;
use crate::domain::PipelineError;
use crate::domain::coordinate_movement::{
    CoordinateMatrix, CoordinateMovement, MovementError,
};

mod light;
mod locator;
mod physics;

#[cfg(test)]
#[path = "movement_records_tests.rs"]
mod tests;

/// One decoded source component and its stable package-relative path.
pub(super) struct SourceComponent {
    /// Stable slash-separated source path.
    pub(super) relative_path: String,
    /// Parsed decoded JSON value.
    pub(super) value: Value,
}

/// Collect all currently decoded non-mesh coordinate records.
///
/// # Errors
///
/// Returns an error when source enumeration, JSON parsing, or coordinate
/// transformation fails.
pub(super) fn collect_moved_records(
    package_root: &Path,
    movement: CoordinateMovement,
) -> Result<Vec<WorldMovedCoordinateRecord>, PipelineError> {
    let mut records = Vec::new();
    collect_locators(
        &components(
            package_root,
            "srr_locator",
        )?,
        movement,
        &mut records,
    )?;
    collect_lights(
        &components(
            package_root,
            "light",
        )?,
        movement,
        &mut records,
    )?;
    collect_physics(
        &components(
            package_root,
            "srr_static_phys_dsg",
        )?,
        movement,
        &mut records,
    )?;
    records.sort_by(
        |left, right| {
            left.source_path
                .cmp(&right.source_path)
                .then_with(
                    || {
                        left.identity
                            .cmp(&right.identity)
                    },
                )
                .then_with(
                    || {
                        left.subject
                            .cmp(&right.subject)
                    },
                )
        },
    );
    Ok(records)
}

/// Read one decoded component family in stable path order.
fn components(
    package_root: &Path,
    family: &str,
) -> Result<Vec<SourceComponent>, PipelineError> {
    let root = package_root
        .join("components")
        .join(family);
    if !root.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths = fs::read_dir(&root)
        .map_err(
            |error| {
                PipelineError::new(
                    format!("movement component directory failed: {error}"),
                )
            },
        )?
        .map(
            |entry| {
                entry
                    .map(|value| value.path())
                    .map_err(|error| PipelineError::new(error.to_string()))
            },
        )
        .collect::<Result<Vec<PathBuf>, _>>()?;
    paths.retain(
        |path| {
            path.is_file()
                && path
                    .extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(|value| value.eq_ignore_ascii_case("json"))
        },
    );
    paths.sort();
    paths
        .into_iter()
        .map(
            |path| {
                let relative_path = path
                    .strip_prefix(package_root)
                    .map_err(
                        |error| {
                            PipelineError::new(
                                format!(
                                    "movement component path failed: {error}"
                                ),
                            )
                        },
                    )?
                    .to_string_lossy()
                    .replace(
                        '\\', "/",
                    );
                let bytes = fs::read(&path)
                    .map_err(|error| PipelineError::new(error.to_string()))?;
                let value = serde_json::from_slice(&bytes).map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "movement component JSON failed for {}: \
                                 {error}",
                                path.display()
                            ),
                        )
                    },
                )?;
                Ok(
                    SourceComponent {
                        relative_path,
                        value,
                    },
                )
            },
        )
        .collect()
}

/// Return one decoded identity without fixed-width NUL padding.
pub(super) fn identity(
    value: &Value,
    fallback: &str,
) -> String {
    value
        .get("name")
        .and_then(Value::as_str)
        .map(
            |name| {
                name.trim_end_matches('\0')
                    .to_owned()
            },
        )
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| fallback.to_owned())
}

/// Parse one exact three-component floating-point vector.
pub(super) fn vector3(
    value: &Value,
    label: &str,
) -> Result<[f32; 3], PipelineError> {
    let components = value
        .as_array()
        .ok_or_else(
            || PipelineError::new(format!("{label} is not an array")),
        )?;
    if components.len() != 3 {
        return Err(PipelineError::new(format!("{label} has invalid length")));
    }
    let values = components
        .iter()
        .map(json_f32)
        .collect::<Result<Vec<_>, _>>()?;
    values
        .try_into()
        .map_err(
            |_values: Vec<f32>| {
                PipelineError::new(format!("{label} conversion failed"))
            },
        )
}

/// Parse one nested four-by-four row-vector matrix.
pub(super) fn matrix4(
    value: &Value,
    label: &str,
) -> Result<CoordinateMatrix, PipelineError> {
    let rows = value
        .as_array()
        .ok_or_else(
            || PipelineError::new(format!("{label} is not an array")),
        )?;
    if rows.len() != 4 {
        return Err(PipelineError::new(format!("{label} has invalid rows")));
    }
    let values = rows
        .iter()
        .flat_map(
            |row| {
                row.as_array()
                    .into_iter()
                    .flatten()
            },
        )
        .map(json_f32)
        .collect::<Result<Vec<_>, _>>()?;
    if values.len() != 16 {
        return Err(PipelineError::new(format!("{label} has invalid columns")));
    }
    values
        .try_into()
        .map_err(
            |_values: Vec<f32>| {
                PipelineError::new(format!("{label} conversion failed"))
            },
        )
}

/// Convert one movement failure into the pipeline error boundary.
pub(super) fn movement_error(
    identity: &str
) -> impl FnOnce(MovementError) -> PipelineError + '_ {
    move |error| {
        PipelineError::new(
            format!("coordinate movement failed for {identity}: {error}"),
        )
    }
}

/// Parse one finite JSON number as an exact pipeline coordinate.
fn json_f32(value: &Value) -> Result<f32, PipelineError> {
    let number = value
        .as_number()
        .ok_or_else(|| PipelineError::new("coordinate value is not numeric"))?;
    let parsed = number
        .to_string()
        .parse::<f32>()
        .map_err(
            |error| {
                PipelineError::new(
                    format!("coordinate value conversion failed: {error}"),
                )
            },
        )?;
    parsed
        .is_finite()
        .then_some(parsed)
        .ok_or_else(|| PipelineError::new("coordinate value is non-finite"))
}
