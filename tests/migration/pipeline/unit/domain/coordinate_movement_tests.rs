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
//   - Coordinate movement tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Coordinate movement tests test module.
// - Description:
//   - Implements the declared test module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Coordinate movement tests test module.

use super::{
    CoordinateMovement, CoordinateSubject, MovementError, identity_matrix,
    translation_matrix,
};

/// Complete representative subject contract.
const SUBJECTS: &[CoordinateSubject] = &[
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

/// Return whether two coordinates are equal within test precision.
fn coordinates_close(left: [f32; 3], right: [f32; 3]) -> bool {
    left.into_iter()
        .zip(right)
        .all(|(left_value, right_value)| {
            (left_value - right_value).abs() <= f32::EPSILON
        })
}

/// Build one mirrored Z movement used by the Kwik-E-Mart placement shape.
const fn mirrored_movement() -> CoordinateMovement {
    CoordinateMovement::new(
        "test-movement",
        [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., -1., 0., -10., 20., -30.,
            1.,
        ],
        SUBJECTS,
    )
}

#[test]
fn point_direction_and_bounds_share_one_affine_basis() -> Result<(), String> {
    let movement = mirrored_movement();
    movement.validate().map_err(|error| error.to_string())?;
    let point = movement
        .transform_point([12., 4., -8.])
        .map_err(|error| error.to_string())?;
    if !coordinates_close(point, [2., 24., -22.]) {
        return Err(String::from("point movement was incorrect"));
    }
    let direction = movement
        .transform_direction([0., 0., 1.])
        .map_err(|error| error.to_string())?;
    if !coordinates_close(direction, [0., 0., -1.]) {
        return Err(String::from("direction included invalid translation"));
    }
    let bounds = movement
        .transform_bounds([10., 0., -5.], [20., 10., 5.])
        .map_err(|error| error.to_string())?;
    if !coordinates_close(bounds.0, [0., 20., -35.])
        || !coordinates_close(bounds.1, [10., 30., -25.])
    {
        return Err(String::from("movement bounds were incorrect"));
    }
    Ok(())
}

#[test]
fn authored_placement_is_composed_before_world_movement() -> Result<(), String>
{
    let movement = mirrored_movement();
    let authored = translation_matrix([1., 2., 3.]);
    let transformed = movement.transform_placement(&authored);
    let expected = [-9., 22., -33.];
    let matched = transformed.into_iter().skip(12).take(3).zip(expected).all(
        |(actual, expected_value)| {
            (actual - expected_value).abs() <= f32::EPSILON
        },
    );
    matched
        .then_some(())
        .ok_or_else(|| String::from("placement translation was incorrect"))
}

#[test]
fn movement_declares_every_future_coordinate_family() -> Result<(), String> {
    let movement = mirrored_movement();
    let labels = movement
        .subjects()
        .iter()
        .map(|subject| subject.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    if labels.len() != SUBJECTS.len() {
        return Err(String::from("coordinate subject identities repeated"));
    }
    for required in [
        "geometry",
        "collision",
        "door",
        "object-placement",
        "character-spawn",
        "object-spawn",
        "mission-placement",
        "trigger",
        "camera",
        "locator",
        "light",
    ] {
        if !labels.contains(required) {
            return Err(format!("coordinate subject is missing: {required}"));
        }
    }
    Ok(())
}

#[test]
fn invalid_affine_state_fails_closed() {
    let empty = CoordinateMovement::new("", identity_matrix(), SUBJECTS);
    assert_eq!(empty.validate(), Err(MovementError::MissingIdentity));
    let degenerate = CoordinateMovement::new("degenerate", [0.; 16], SUBJECTS);
    assert!(degenerate.validate().is_err());
    assert!(
        mirrored_movement()
            .transform_bounds([1., 0., 0.,], [0., 1., 1.,],)
            .is_err()
    );
}
