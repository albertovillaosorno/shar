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
//   - Reviewed exterior-family and package-specific interior movements plus the
//     exact global height offset.
// - Must-Not:
//   - Infer transforms at runtime, mutate source files, fuse geometry, or
//     serialize catalogs.
// - Allows:
//   - Apply one reviewed affine movement to render, collision, and decoded
//     coordinates for every world package.
// - Summary:
//   - Applies reviewed world placement, reflection, and global height.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Applies reviewed family placement plus one global exterior X reflection.

use std::path::Path;

use fbx::domain::mesh::MeshAsset;

use super::interior::{
    movement_for_package as interior_movement_for_package,
    reviewed_movement_for_package as reviewed_interior_movement_for_package,
};
use super::layout::collection_bounds;
use super::movement_model::WorldCoordinateMovementRecord;
use super::movement_records::collect_moved_records;
use super::transform::bake_mesh;
use crate::domain::PipelineError;
use crate::domain::coordinate_movement::{
    CoordinateMovement, CoordinateSubject,
};

/// Stable movement identity for the Levels 1, 4, and 7 map family.
const ZONE_1_MOVEMENT_ID: &str =
    "zone-01-levels-01-04-07-global-horizontal-mirror-and-height";
/// Stable movement identity for the Levels 2 and 5 map family.
const ZONE_2_MOVEMENT_ID: &str =
    "zone-02-levels-02-05-placement-global-mirror-and-height";
/// Stable movement identity for the Levels 3 and 6 map family.
const ZONE_3_MOVEMENT_ID: &str =
    "zone-03-levels-03-06-placement-global-mirror-and-height";
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

/// Superseded reference height retained only to normalize reviewed interiors.
pub(super) const LEGACY_REVIEWED_HEIGHT_OFFSET_METERS: f32 = 43.396;
/// Canonical portable height baked into every generated FBX and coordinate.
pub(super) const WORLD_HEIGHT_OFFSET_METERS: f32 = 41.046;

/// Source-space transform canceling the shared FBX horizontal X reversal.
const ZONE_1_MOVEMENT: CoordinateMovement = CoordinateMovement::new(
    ZONE_1_MOVEMENT_ID,
    [
        -1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        WORLD_HEIGHT_OFFSET_METERS,
        0.0,
        1.0,
    ],
    ZONE_SUBJECTS,
);

/// Reviewed Zone 2 placement followed by the global exterior X reflection.
///
/// The family retains the operator-authored connection, mirrors source X so the
/// shared FBX export-root reversal no longer swaps the world's left and right
/// sides, and adds the exact global source-height translation.
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
        1.0,
        0.0,
        0.0,
        0.0,
        -989.247_3,
        WORLD_HEIGHT_OFFSET_METERS,
        -360.133_76,
        1.0,
    ],
    ZONE_SUBJECTS,
);

/// Vertex-solved Zone 3 placement followed by the global X reflection.
///
/// The reviewed object changed its local origin, so the rigid transform was
/// solved by matching stable vertex indices against the untouched Level 3
/// general FBX. The maximum residual was below 0.00016 Blender units. The final
/// source X reflection cancels the shared FBX export-root reversal, and the
/// source-height row adds the exact canonical 41.046-meter world datum.
const ZONE_3_MOVEMENT: CoordinateMovement = CoordinateMovement::new(
    ZONE_3_MOVEMENT_ID,
    [
        0.0,
        0.0,
        -1.0,
        0.0,
        0.0,
        1.0,
        0.0,
        0.0,
        -1.0,
        0.0,
        0.0,
        0.0,
        -745.360_84,
        WORLD_HEIGHT_OFFSET_METERS,
        296.963_32,
        1.0,
    ],
    ZONE_SUBJECTS,
);

/// Apply one reviewed exterior-family or interior movement to every decoded
/// coordinate family owned by a package.
///
/// Interior packages use their own reviewed full-XYZ matrices rather than the
/// recurring exterior-family placement.
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
    let Some(movement) = movement_for_package(
        scope, interior, package_id,
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

/// Transform one interior ownership snapshot in the exact reviewed datum.
///
/// The snapshot exists only for fused-interior duplicate decisions. Final mesh,
/// collision, and decoded-coordinate evidence still receive the complete
/// 41.046-meter movement through [`apply_package_movement`].
///
/// # Errors
///
/// Returns an error when the package has no reviewed movement or the ownership
/// mesh cannot be transformed.
pub(super) fn apply_interior_ownership_movement(
    package_id: &str,
    render_meshes: &mut [MeshAsset],
) -> Result<(), PipelineError> {
    let (id, matrix) = reviewed_interior_movement_for_package(package_id)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "interior ownership movement is missing: {package_id}"
                    ),
                )
            },
        )?;
    let movement = CoordinateMovement::new(
        id,
        matrix,
        ZONE_SUBJECTS,
    );
    movement
        .validate()
        .map_err(
            |error| {
                PipelineError::new(
                    format!("interior ownership movement is invalid: {error}"),
                )
            },
        )?;
    apply_to_meshes(
        render_meshes,
        movement,
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

/// Return the final movement for one world package.
fn movement_for_package(
    scope: &str,
    interior: bool,
    package_id: &str,
) -> Option<CoordinateMovement> {
    if interior {
        return interior_movement_for_package(package_id).map(
            |(id, matrix)| {
                CoordinateMovement::new(
                    id,
                    matrix,
                    ZONE_SUBJECTS,
                )
            },
        );
    }
    let level = scope
        .strip_prefix("level-")
        .and_then(
            |value| {
                value
                    .parse::<u8>()
                    .ok()
            },
        );
    level.and_then(exterior_movement_for_level)
}

/// Return the reviewed exterior-family movement for one narrative level.
#[must_use]
pub(super) const fn exterior_movement_for_level(
    level: u8
) -> Option<CoordinateMovement> {
    match level {
        1 | 4 | 7 => Some(ZONE_1_MOVEMENT),
        2 | 5 => Some(ZONE_2_MOVEMENT),
        3 | 6 => Some(ZONE_3_MOVEMENT),
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
    use super::{
        LEGACY_REVIEWED_HEIGHT_OFFSET_METERS, WORLD_HEIGHT_OFFSET_METERS,
        coordinates_close, movement_for_package,
    };

    fn moved_point(scope: &str) -> Result<[f32; 3], String> {
        movement_for_package(
            scope,
            false,
            "exterior-test",
        )
        .ok_or_else(|| format!("movement is missing for {scope}"))?
        .transform_point(
            [
                100.0, 20.0, 300.0,
            ],
        )
        .map_err(|error| error.to_string())
    }

    #[test]
    fn canonical_world_height_excludes_the_legacy_reference_offset() {
        assert_eq!(
            LEGACY_REVIEWED_HEIGHT_OFFSET_METERS,
            43.396,
        );
        assert_eq!(
            WORLD_HEIGHT_OFFSET_METERS,
            41.046,
        );
    }

    #[test]
    fn zone_one_cancels_the_global_horizontal_reversal() -> Result<(), String> {
        let movement = movement_for_package(
            "level-01",
            false,
            "exterior-test",
        )
        .ok_or_else(|| String::from("Zone 1 movement is missing"))?;
        if movement.id()
            != "zone-01-levels-01-04-07-global-horizontal-mirror-and-height"
        {
            return Err(String::from("Zone 1 movement identity changed"));
        }
        let moved = moved_point("level-01")?;
        if !coordinates_close(
            moved,
            [
                -100.0, 61.046, 300.0,
            ],
            0.001,
        ) {
            return Err(format!("Zone 1 mirror changed: {moved:?}"));
        }
        Ok(())
    }

    #[test]
    fn zone_two_places_then_mirrors_the_exterior() -> Result<(), String> {
        let movement = movement_for_package(
            "level-02",
            false,
            "exterior-test",
        )
        .ok_or_else(|| String::from("Zone 2 movement is missing"))?;
        if movement.id()
            != "zone-02-levels-02-05-placement-global-mirror-and-height"
        {
            return Err(String::from("Zone 2 movement identity changed"));
        }
        let moved = moved_point("level-02")?;
        if !coordinates_close(
            moved,
            [
                -689.247_3,
                61.046,
                -260.133_76,
            ],
            0.001,
        ) {
            return Err(format!("Zone 2 placement changed: {moved:?}"));
        }
        Ok(())
    }

    #[test]
    fn zone_three_places_then_mirrors_the_exterior() -> Result<(), String> {
        let movement = movement_for_package(
            "level-03",
            false,
            "exterior-test",
        )
        .ok_or_else(|| String::from("Zone 3 movement is missing"))?;
        if movement.id()
            != "zone-03-levels-03-06-placement-global-mirror-and-height"
        {
            return Err(String::from("Zone 3 movement identity changed"));
        }
        let moved = moved_point("level-03")?;
        if !coordinates_close(
            moved,
            [
                -1_045.360_8,
                61.046,
                196.963_32,
            ],
            0.001,
        ) {
            return Err(format!("Zone 3 placement changed: {moved:?}"));
        }
        Ok(())
    }

    #[test]
    fn every_interior_has_reviewed_placement_and_global_height()
    -> Result<(), String> {
        for (scope, package) in [
            (
                "level-01",
                "extracted-art-l1i00",
            ),
            (
                "level-02",
                "extracted-art-l2i04",
            ),
            (
                "level-03",
                "extracted-art-l3i06",
            ),
            (
                "level-04",
                "extracted-art-l4i07",
            ),
            (
                "level-05",
                "extracted-art-l5i03",
            ),
            (
                "level-06",
                "extracted-art-l6i05",
            ),
            (
                "level-07",
                "extracted-art-l7i01",
            ),
        ] {
            let movement = movement_for_package(
                scope, true, package,
            )
            .ok_or_else(
                || format!("interior movement is missing: {package}"),
            )?;
            let moved = movement
                .transform_point(
                    [
                        0.0, 0.0, 0.0,
                    ],
                )
                .map_err(|error| error.to_string())?;
            let [
                _,
                height,
                _,
            ] = moved;
            if height < WORLD_HEIGHT_OFFSET_METERS {
                return Err(
                    format!("interior height is missing: {package}:{height}"),
                );
            }
        }
        Ok(())
    }
}
