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

use super::super::model::AtlasAssignment;
use super::{atlas_uv, validate_world_fbx_bounds};

fn assignment(repeat: f32) -> AtlasAssignment {
    AtlasAssignment {
        offset: [0.1, 0.2],
        scale: [0.3, 0.4],
        repeat,
        approximated_vertex_color: false,
    }
}

#[test]
fn imported_uv_zero_bakes_repeating_atlas_mapping() {
    let mapped = atlas_uv([-0.25, 1.25], assignment(1.));
    assert!((mapped[0] - 0.325).abs() <= f32::EPSILON);
    assert!((mapped[1] - 0.3).abs() <= f32::EPSILON);
}

#[test]
fn imported_uv_zero_clamps_to_its_atlas_tile() {
    let mapped = atlas_uv([-0.5, 1.5], assignment(0.));
    assert!((mapped[0] - 0.1).abs() <= f32::EPSILON);
    assert!((mapped[1] - 0.6).abs() <= f32::EPSILON);
}

#[test]
fn source_world_bounds_reject_non_finite_first_position() -> Result<(), String>
{
    let result = validate_world_fbx_bounds(&[[f32::NAN, 0., 0.]]);
    let Err(error) = result else {
        return Err("non-finite first positions must fail".to_owned());
    };
    if !error.to_string().contains("non-finite") {
        return Err(format!("unexpected bounds failure: {error}"));
    }
    Ok(())
}

#[test]
fn source_world_bounds_are_validated_without_extent_or_height_policy()
-> Result<(), String> {
    let positions =
        [[-20_000., -500., 30_000.], [40_000., 12_000., -10_000.], [
            0., 0., 0.,
        ]];
    validate_world_fbx_bounds(&positions).map_err(|error| error.to_string())?;
    assert_eq!(positions[0], [-20_000., -500., 30_000.]);
    Ok(())
}
