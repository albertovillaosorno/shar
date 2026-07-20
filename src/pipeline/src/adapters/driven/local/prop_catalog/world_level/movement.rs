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
//   - Family-level world movement selected from reviewed operator placement.
// - Must-Not:
//   - Infer transforms at runtime, move interiors, mutate source files, or
//     serialize catalogs.
// - Allows:
//   - Apply one reviewed affine movement to render, collision, and decoded
//     coordinates for every exterior package in one recurring map family.
// - Summary:
//   - Applies the reviewed Zone 2 and Zone 3 horizontal placements.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Applies reviewed family-level exterior movement above geometry conversion.

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

/// Stable movement identity for the Levels 2 and 5 map family.
const ZONE_2_MOVEMENT_ID: &str = "zone-02-levels-02-05-operator-placement";
/// Stable movement identity for the Levels 3 and 6 map family.
const ZONE_3_MOVEMENT_ID: &str = "zone-03-levels-03-06-operator-placement";
/// Coordinate families that must move with one exterior zone placement.
const ZONE_SUBJECTS: &[CoordinateSubject] = &[
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

/// Source-space transform derived from the reviewed Zone 2 placement.
///
/// The solved horizontal transform is composed after the former
/// `[8192, 0, 0]` audit spacing. Source height remains unchanged.
const ZONE_2_MOVEMENT: CoordinateMovement = CoordinateMovement::new(
    ZONE_2_MOVEMENT_ID,
    [
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -1.0,
        0.0,
        0.0,
        0.0,
        989.247_3,
        0.0,
        -360.133_76,
        1.0,
    ],
    ZONE_SUBJECTS,
);

/// Source-space transform derived from vertex-matched Zone 3 placement.
///
/// The reviewed object changed its local origin, so the rigid transform was
/// solved by matching stable vertex indices against the untouched Level 3
/// general FBX. The maximum residual was below 0.00016 Blender units. The
/// solved movement is composed after the former `[16384, 0, 0]` audit spacing,
/// while source height remains unchanged.
const ZONE_3_MOVEMENT: CoordinateMovement = CoordinateMovement::new(
    ZONE_3_MOVEMENT_ID,
    [
        0.0, 0.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        745.360_84, 0.0, 296.963_32, 1.0,
    ],
    ZONE_SUBJECTS,
);

/// Apply one reviewed exterior-family movement to every decoded coordinate
/// family owned by a package.
///
/// Interiors remain separate until the next operator-authored placement pass.
///
/// # Errors
///
/// Returns an error when movement validation, mesh transformation, or decoded
/// coordinate evidence fails.
pub(super) fn apply_package_movement(
    scope: &str,
    interior: bool,
    package_id: &str,
    package_root: &Path,
    render_meshes: &mut [MeshAsset],
    collision_meshes: &mut [MeshAsset],
) -> Result<Option<WorldCoordinateMovementRecord>, PipelineError> {
    let Some(movement) = movement_for_scope(
        scope, interior,
    ) else {
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
fn coordinates_close(left: [f32; 3], right: [f32; 3], tolerance: f32) -> bool {
    left.into_iter()
        .zip(right)
        .all(
            |(left_value, right_value)| {
                (left_value - right_value).abs() <= tolerance
            },
        )
}

/// Return the reviewed movement for one exterior package scope.
fn movement_for_scope(
    scope: &str,
    interior: bool,
) -> Option<CoordinateMovement> {
    if interior {
        return None;
    }
    match scope {
        "level-02" | "level-05" => Some(ZONE_2_MOVEMENT),
        "level-03" | "level-06" => Some(ZONE_3_MOVEMENT),
        _ => None,
    }
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
    use super::{coordinates_close, movement_for_scope};

    #[test]
    fn zone_two_movement_matches_reviewed_horizontal_placement()
    -> Result<(), String> {
        let movement = movement_for_scope(
            "level-02", false,
        )
        .ok_or_else(|| String::from("Zone 2 movement is missing"))?;
        if movement.id() != "zone-02-levels-02-05-operator-placement" {
            return Err(String::from("Zone 2 movement identity changed"));
        }
        let moved = movement
            .transform_point(
                [
                    100.0, 20.0, 300.0,
                ],
            )
            .map_err(|error| error.to_string())?;
        if !coordinates_close(
            moved,
            [
                689.247_3,
                20.0,
                -260.133_76,
            ],
            0.001,
        ) {
            return Err(format!("Zone 2 placement changed: {moved:?}"));
        }
        Ok(())
    }

    #[test]
    fn zone_three_movement_matches_vertex_solved_placement()
    -> Result<(), String> {
        let movement = movement_for_scope(
            "level-03", false,
        )
        .ok_or_else(|| String::from("Zone 3 movement is missing"))?;
        if movement.id() != "zone-03-levels-03-06-operator-placement" {
            return Err(String::from("Zone 3 movement identity changed"));
        }
        let moved = movement
            .transform_point(
                [
                    100.0, 20.0, 300.0,
                ],
            )
            .map_err(|error| error.to_string())?;
        if !coordinates_close(
            moved,
            [
                1_045.360_8,
                20.0,
                196.963_32,
            ],
            0.001,
        ) {
            return Err(format!("Zone 3 placement changed: {moved:?}"));
        }
        Ok(())
    }

    #[test]
    fn zone_one_and_interiors_remain_unmoved() {
        for scope in [
            "level-01", "level-04", "level-07",
        ] {
            assert!(
                movement_for_scope(
                    scope, false
                )
                .is_none()
            );
        }
        assert!(
            movement_for_scope(
                "level-02", true
            )
            .is_none()
        );
        assert!(
            movement_for_scope(
                "level-03", true
            )
            .is_none()
        );
    }
}
