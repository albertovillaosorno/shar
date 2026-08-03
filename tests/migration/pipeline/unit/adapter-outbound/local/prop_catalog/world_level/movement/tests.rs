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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use super::{
    LEGACY_REVIEWED_HEIGHT_OFFSET_METERS, WORLD_HEIGHT_OFFSET_METERS,
    coordinates_close, movement_for_package,
    reviewed_interior_movement_for_package,
};

fn moved_point(scope: &str) -> Result<[f32; 3], String> {
    movement_for_package(scope, false, "exterior-test")
        .ok_or_else(|| format!("movement is missing for {scope}"))?
        .transform_point([100., 20., 300.])
        .map_err(|error| error.to_string())
}

fn basis_determinant(matrix: [f32; 16]) -> f32 {
    matrix[0] * matrix[5].mul_add(matrix[10], -matrix[6] * matrix[9])
        - matrix[1] * matrix[4].mul_add(matrix[10], -matrix[6] * matrix[8])
        + matrix[2] * matrix[4].mul_add(matrix[9], -matrix[5] * matrix[8])
}

#[test]
fn exterior_movements_preserve_handedness() -> Result<(), String> {
    for scope in ["level-01", "level-02", "level-03"] {
        let movement = movement_for_package(scope, false, "exterior-test")
            .ok_or_else(|| format!("movement is missing for {scope}"))?;
        let determinant = basis_determinant(movement.matrix());
        if determinant <= 0. {
            return Err(format!(
                "exterior movement changed handedness for {scope}: \
                     {determinant}"
            ));
        }
        if movement.id().contains("mirror") {
            return Err(format!(
                "exterior movement still declares a mirror: {scope}"
            ));
        }
    }
    Ok(())
}

#[test]
fn interior_movement_keeps_its_explicit_x_basis_correction()
-> Result<(), String> {
    let (_, matrix) =
        reviewed_interior_movement_for_package("extracted-art-l1i00")
            .ok_or_else(|| String::from("interior movement is missing"))?;
    if matrix[0] != -1. {
        return Err(format!(
            "interior X-basis correction changed: {}",
            matrix[0]
        ));
    }
    Ok(())
}

#[test]
fn canonical_world_height_excludes_the_legacy_reference_offset() {
    assert_eq!(LEGACY_REVIEWED_HEIGHT_OFFSET_METERS, 43.396,);
    assert_eq!(WORLD_HEIGHT_OFFSET_METERS, 80.,);
}

#[test]
fn zone_one_preserves_exterior_orientation() -> Result<(), String> {
    let movement = movement_for_package("level-01", false, "exterior-test")
        .ok_or_else(|| String::from("Zone 1 movement is missing"))?;
    if movement.id() != "zone-01-levels-01-04-07-height" {
        return Err(String::from("Zone 1 movement identity changed"));
    }
    let moved = moved_point("level-01")?;
    if !coordinates_close(
        moved,
        [100., 20. + WORLD_HEIGHT_OFFSET_METERS, 300.],
        0.001,
    ) {
        return Err(format!("Zone 1 orientation changed: {moved:?}"));
    }
    Ok(())
}

#[test]
fn zone_two_places_without_mirroring_exterior() -> Result<(), String> {
    let movement = movement_for_package("level-02", false, "exterior-test")
        .ok_or_else(|| String::from("Zone 2 movement is missing"))?;
    if movement.id() != "zone-02-levels-02-05-placement-and-height" {
        return Err(String::from("Zone 2 movement identity changed"));
    }
    let moved = moved_point("level-02")?;
    if !coordinates_close(
        moved,
        [-689.247_3, 20. + WORLD_HEIGHT_OFFSET_METERS, -460.133_76],
        0.001,
    ) {
        return Err(format!("Zone 2 placement changed: {moved:?}"));
    }
    Ok(())
}

#[test]
fn zone_three_places_without_mirroring_exterior() -> Result<(), String> {
    let movement = movement_for_package("level-03", false, "exterior-test")
        .ok_or_else(|| String::from("Zone 3 movement is missing"))?;
    if movement.id() != "zone-03-levels-03-06-placement-and-height" {
        return Err(String::from("Zone 3 movement identity changed"));
    }
    let moved = moved_point("level-03")?;
    if !coordinates_close(
        moved,
        [-1_045.360_8, 20. + WORLD_HEIGHT_OFFSET_METERS, 396.963_32],
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
        ("level-01", "extracted-art-l1i00"),
        ("level-02", "extracted-art-l2i04"),
        ("level-03", "extracted-art-l3i06"),
        ("level-04", "extracted-art-l4i07"),
        ("level-05", "extracted-art-l5i03"),
        ("level-06", "extracted-art-l6i05"),
        ("level-07", "extracted-art-l7i01"),
    ] {
        let movement =
            movement_for_package(scope, true, package).ok_or_else(|| {
                format!("interior movement is missing: {package}")
            })?;
        let moved = movement
            .transform_point([0., 0., 0.])
            .map_err(|error| error.to_string())?;
        let [_, height, _] = moved;
        if height < WORLD_HEIGHT_OFFSET_METERS {
            return Err(format!(
                "interior height is missing: {package}:{height}"
            ));
        }
    }
    Ok(())
}
