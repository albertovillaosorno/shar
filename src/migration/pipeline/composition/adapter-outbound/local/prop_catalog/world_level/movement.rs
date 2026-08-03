// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Movement outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Movement outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Movement outbound adapter.

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
const ZONE_1_MOVEMENT_ID: &str = "zone-01-levels-01-04-07-height";
/// Stable movement identity for the Levels 2 and 5 map family.
const ZONE_2_MOVEMENT_ID: &str = "zone-02-levels-02-05-placement-and-height";
/// Stable movement identity for the Levels 3 and 6 map family.
const ZONE_3_MOVEMENT_ID: &str = "zone-03-levels-03-06-placement-and-height";
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
pub(super) const WORLD_HEIGHT_OFFSET_METERS: f32 = 80.;

/// Orientation-preserving Zone 1 placement with the canonical height.
const ZONE_1_MOVEMENT: CoordinateMovement = CoordinateMovement::new(
    ZONE_1_MOVEMENT_ID,
    [
        1.,
        0.,
        0.,
        0.,
        0.,
        1.,
        0.,
        0.,
        0.,
        0.,
        1.,
        0.,
        0.,
        WORLD_HEIGHT_OFFSET_METERS,
        0.,
        1.,
    ],
    ZONE_SUBJECTS,
);

/// Reviewed orientation-preserving Zone 2 placement.
///
/// The family retains the operator-authored connection and exact global height
/// without applying a horizontal reflection to exterior geometry or records.
const ZONE_2_MOVEMENT: CoordinateMovement = CoordinateMovement::new(
    ZONE_2_MOVEMENT_ID,
    [
        0.,
        0.,
        -1.,
        0.,
        0.,
        1.,
        0.,
        0.,
        1.,
        0.,
        0.,
        0.,
        -989.247_3,
        WORLD_HEIGHT_OFFSET_METERS,
        -360.133_76,
        1.,
    ],
    ZONE_SUBJECTS,
);

/// Vertex-solved orientation-preserving Zone 3 placement.
///
/// The reviewed object changed its local origin, so the rigid transform was
/// solved by matching stable vertex indices against the untouched Level 3
/// general FBX. The maximum residual was below 0.00016 Blender units. The basis
/// preserves handedness and adds the exact canonical 80-meter world datum.
const ZONE_3_MOVEMENT: CoordinateMovement = CoordinateMovement::new(
    ZONE_3_MOVEMENT_ID,
    [
        0.,
        0.,
        1.,
        0.,
        0.,
        1.,
        0.,
        0.,
        -1.,
        0.,
        0.,
        0.,
        -745.360_84,
        WORLD_HEIGHT_OFFSET_METERS,
        296.963_32,
        1.,
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
    let Some(movement) = movement_for_package(scope, interior, package_id)
    else {
        return Ok(None);
    };
    movement.validate().map_err(|error| {
        PipelineError::new(format!(
            "world coordinate movement is invalid: {error}"
        ))
    })?;
    let source_render_bounds = collection_bounds(render_meshes)
        .map(|bounds| (bounds.low, bounds.high));
    let expected_moved_bounds = source_render_bounds
        .map(|(low, high)| {
            movement.transform_bounds(low, high).map_err(|error| {
                PipelineError::new(format!(
                    "world movement bounds failed: {error}"
                ))
            })
        })
        .transpose()?;
    apply_to_meshes(render_meshes, movement)?;
    let moved_render_bounds = collection_bounds(render_meshes)
        .map(|bounds| (bounds.low, bounds.high));
    validate_moved_bounds(expected_moved_bounds, moved_render_bounds)?;
    apply_to_meshes(collision_meshes, movement)?;
    let records = collect_moved_records(package_root, movement)?;
    Ok(Some(WorldCoordinateMovementRecord {
        id: movement.id().to_owned(),
        package_id: package_id.to_owned(),
        matrix: movement.matrix(),
        subjects: movement
            .subjects()
            .iter()
            .map(|subject| subject.as_str().to_owned())
            .collect(),
        moved_render_meshes: render_meshes.len(),
        moved_collision_meshes: collision_meshes.len(),
        source_render_bounds,
        moved_render_bounds,
        records,
    }))
}

/// Transform one interior ownership snapshot in the exact reviewed datum.
///
/// The snapshot exists only for fused-interior duplicate decisions. Final mesh,
/// collision, and decoded-coordinate evidence still receive the complete
/// 80-meter movement through [`apply_package_movement`].
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
        .ok_or_else(|| {
            PipelineError::new(format!(
                "interior ownership movement is missing: {package_id}"
            ))
        })?;
    let movement = CoordinateMovement::new(id, matrix, ZONE_SUBJECTS);
    movement.validate().map_err(|error| {
        PipelineError::new(format!(
            "interior ownership movement is invalid: {error}"
        ))
    })?;
    apply_to_meshes(render_meshes, movement)
}

/// Verify actual mesh movement against the pure bound projection.
fn validate_moved_bounds(
    expected_bounds: Option<([f32; 3], [f32; 3])>,
    actual_bounds: Option<([f32; 3], [f32; 3])>,
) -> Result<(), PipelineError> {
    let ((expected_low, expected_high), (actual_low, actual_high)) =
        match (expected_bounds, actual_bounds) {
            (Some(projected), Some(observed)) => (projected, observed),
            (None, None) => return Ok(()),
            _ => {
                return Err(PipelineError::new(
                    "world movement bounds disappeared",
                ));
            },
        };
    if !coordinates_close(expected_low, actual_low, 0.001)
        || !coordinates_close(expected_high, actual_high, 0.001)
    {
        return Err(PipelineError::new(format!(
            concat!(
                "world movement bound mismatch: expected ",
                "{:?}..{:?}; actual {:?}..{:?}"
            ),
            expected_low, expected_high, actual_low, actual_high,
        )));
    }
    Ok(())
}

/// Return whether every coordinate component is within one tolerance.
fn coordinates_close(left: [f32; 3], right: [f32; 3], tolerance: f32) -> bool {
    left.into_iter()
        .zip(right)
        .all(|(left_value, right_value)| {
            (left_value - right_value).abs() <= tolerance
        })
}

/// Return the final movement for one world package.
fn movement_for_package(
    scope: &str,
    interior: bool,
    package_id: &str,
) -> Option<CoordinateMovement> {
    if interior {
        return interior_movement_for_package(package_id).map(
            |(id, matrix)| CoordinateMovement::new(id, matrix, ZONE_SUBJECTS),
        );
    }
    let level = scope
        .strip_prefix("level-")
        .and_then(|value| value.parse::<u8>().ok());
    level.and_then(exterior_movement_for_level)
}

/// Return the reviewed exterior-family movement for one narrative level.
#[must_use]
pub(super) const fn exterior_movement_for_level(
    level: u8,
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
        let name = mesh.name.clone();
        bake_mesh(mesh, &matrix, name)?;
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/movement/tests.rs"]
mod tests;
