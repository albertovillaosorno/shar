// File:
//   - movement.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     movement.rs
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
//   - Package selection and adapter application for authored world movement.
// - Must-Not:
//   - Infer transforms at runtime, mutate source files, or serialize catalogs.
// - Allows:
//   - Apply one domain movement to render, collision, and decoded coordinates.
// - Summary:
//   - First authored coordinate movement: Kwik-E-Mart interior into Level 1.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Applies authored package movement above geometry-specific conversion.

use std::path::Path;

use fbx::domain::mesh::MeshAsset;

use super::layout::collection_bounds;
use super::movement_model::WorldCoordinateMovementRecord;
use super::movement_records::collect_moved_records;
use super::transform::bake_mesh;
use crate::domain::PipelineError;
use crate::domain::coordinate_movement::{
    CoordinateMovement, CoordinateSubject,
};

/// Canonical package containing the Kwik-E-Mart interior.
const KWIK_E_MART_PACKAGE: &str = "extracted-art-l1i01";
/// Stable movement identity preserved in generated catalogs.
const KWIK_E_MART_MOVEMENT_ID: &str = "level-01-kwik-e-mart-interior-to-world";
/// Coordinate families that must move together when runtime translation lands.
const KWIK_E_MART_SUBJECTS: &[CoordinateSubject] = &[
    CoordinateSubject::Geometry,
    CoordinateSubject::Collision,
    CoordinateSubject::Door,
    CoordinateSubject::ObjectPlacement,
    CoordinateSubject::CharacterSpawn,
    CoordinateSubject::ObjectSpawn,
    CoordinateSubject::MissionPlacement,
    CoordinateSubject::Trigger,
    CoordinateSubject::Camera,
    CoordinateSubject::Locator,
    CoordinateSubject::Light,
];
/// Source-space transform derived from the operator-authored FBX alignment.
///
/// The FBX root maps source `(x, y, z)` to Blender `(-x, z, y)`. The authored
/// Blender movement preserves X and Z, reflects Blender Y, and translates by
/// approximately `(285.4401, -609.0188, 25.1905)`. Conjugating that movement
/// through the root conversion yields this source-space matrix.
const KWIK_E_MART_MOVEMENT: CoordinateMovement = CoordinateMovement::new(
    KWIK_E_MART_MOVEMENT_ID,
    [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0,
        -285.440_1, 25.190_514, -609.018_8, 1.0,
    ],
    KWIK_E_MART_SUBJECTS,
);

/// Apply one package movement to every currently decoded coordinate family.
///
/// # Errors
///
/// Returns an error when movement validation, mesh transformation, or decoded
/// coordinate evidence fails.
pub(super) fn apply_package_movement(
    package_id: &str,
    package_root: &Path,
    render_meshes: &mut [MeshAsset],
    collision_meshes: &mut [MeshAsset],
) -> Result<Option<WorldCoordinateMovementRecord>, PipelineError> {
    let Some(movement) = movement_for_package(package_id) else {
        return Ok(None);
    };
    movement
        .validate()
        .map_err(
            |error| {
                PipelineError::new(
                    format!("world coordinate movement is invalid: {error}"),
                )
            },
        )?;
    let source_render_bounds = collection_bounds(render_meshes).map(
        |bounds| {
            (
                bounds.low,
                bounds.high,
            )
        },
    );
    let expected_moved_bounds = source_render_bounds
        .map(
            |(low, high)| {
                movement
                    .transform_bounds(
                        low, high,
                    )
                    .map_err(
                        |error| {
                            PipelineError::new(
                                format!(
                                    "world movement bounds failed: {error}"
                                ),
                            )
                        },
                    )
            },
        )
        .transpose()?;
    apply_to_meshes(
        render_meshes,
        movement,
    )?;
    let moved_render_bounds = collection_bounds(render_meshes).map(
        |bounds| {
            (
                bounds.low,
                bounds.high,
            )
        },
    );
    validate_moved_bounds(
        expected_moved_bounds,
        moved_render_bounds,
    )?;
    apply_to_meshes(
        collision_meshes,
        movement,
    )?;
    let records = collect_moved_records(
        package_root,
        movement,
    )?;
    Ok(
        Some(
            WorldCoordinateMovementRecord {
                id: movement
                    .id()
                    .to_owned(),
                package_id: package_id.to_owned(),
                matrix: movement.matrix(),
                subjects: movement
                    .subjects()
                    .iter()
                    .map(
                        |subject| {
                            subject
                                .as_str()
                                .to_owned()
                        },
                    )
                    .collect(),
                moved_render_meshes: render_meshes.len(),
                moved_collision_meshes: collision_meshes.len(),
                source_render_bounds,
                moved_render_bounds,
                records,
            },
        ),
    )
}

/// Verify actual mesh movement against the pure bound projection.
fn validate_moved_bounds(
    expected_bounds: Option<(
        [f32; 3],
        [f32; 3],
    )>,
    actual_bounds: Option<(
        [f32; 3],
        [f32; 3],
    )>,
) -> Result<(), PipelineError> {
    let ((expected_low, expected_high), (actual_low, actual_high)) = match (
        expected_bounds,
        actual_bounds,
    ) {
        (Some(projected), Some(observed)) => (
            projected, observed,
        ),
        (None, None) => return Ok(()),
        _ => {
            return Err(
                PipelineError::new("world movement bounds disappeared"),
            );
        }
    };
    if !coordinates_close(
        expected_low,
        actual_low,
        0.001,
    ) || !coordinates_close(
        expected_high,
        actual_high,
        0.001,
    ) {
        return Err(
            PipelineError::new(
                format!(
                    concat!(
                        "world movement bound mismatch: expected ",
                        "{:?}..{:?}; actual {:?}..{:?}"
                    ),
                    expected_low, expected_high, actual_low, actual_high,
                ),
            ),
        );
    }
    Ok(())
}

/// Return whether every coordinate component is within one tolerance.
fn coordinates_close(
    left: [f32; 3],
    right: [f32; 3],
    tolerance: f32,
) -> bool {
    left.into_iter()
        .zip(right)
        .all(
            |(left_value, right_value)| {
                (left_value - right_value).abs() <= tolerance
            },
        )
}

/// Return the statically reviewed movement for one package.
fn movement_for_package(package_id: &str) -> Option<CoordinateMovement> {
    (package_id == KWIK_E_MART_PACKAGE).then_some(KWIK_E_MART_MOVEMENT)
}

/// Bake one movement into every mesh while preserving stable mesh identities.
fn apply_to_meshes(
    meshes: &mut [MeshAsset],
    movement: CoordinateMovement,
) -> Result<(), PipelineError> {
    let matrix = movement.matrix();
    for mesh in meshes {
        let name = mesh
            .name
            .clone();
        bake_mesh(
            mesh, &matrix, name,
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{coordinates_close, movement_for_package};

    #[test]
    fn kwik_movement_matches_authored_basis() -> Result<(), String> {
        let movement = movement_for_package("extracted-art-l1i01")
            .ok_or_else(|| String::from("Kwik-E-Mart movement is missing"))?;
        if movement.id() != "level-01-kwik-e-mart-interior-to-world" {
            return Err(String::from("Kwik-E-Mart movement identity changed"));
        }
        let moved = movement
            .transform_point(
                [
                    500.0, -20.0, -300.0,
                ],
            )
            .map_err(|error| error.to_string())?;
        let expected = [
            214.559_9,
            5.190_513_6,
            -309.018_8,
        ];
        if !coordinates_close(
            moved, expected, 0.000_2,
        ) {
            return Err(
                format!(
                    "Kwik-E-Mart InteriorOrigin moved incorrectly: {moved:?}"
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn movement_is_package_specific() {
        assert!(movement_for_package("extracted-art-l1i01").is_some());
        assert!(movement_for_package("extracted-art-l1i00").is_none());
        assert!(movement_for_package("extracted-art-l1z2").is_none());
    }
}
