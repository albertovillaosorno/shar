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

use std::collections::BTreeMap;

use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};

use super::super::model::{
    AtlasAssignment, AtlasBuild, AtlasLayout, SurfaceKey,
};
use super::super::super::export::MasterContent;
use super::{atlas_uv, build, validate_world_fbx_bounds};

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

fn ordered_mesh(
    name: &str,
    shader: &str,
    x: f32,
) -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        shader,
        vec![[x, 0., 0.], [x + 0.5, 0., 0.], [x, 0.5, 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("ordered group failed: {error:?}"))?;
    MeshAsset::new(name, vec![group])
        .map_err(|error| format!("ordered mesh failed: {error:?}"))
}

fn ordered_atlas() -> AtlasBuild {
    let assignment = AtlasAssignment {
        offset: [0., 0.],
        scale: [1., 1.],
        repeat: 0.,
        approximated_vertex_color: false,
    };
    AtlasBuild {
        png_bytes: Vec::new(),
        layout: AtlasLayout {
            schema_version: 1,
            atlas_width: 1,
            atlas_height: 1,
            padding_pixels: 0,
            rotation_allowed: false,
            entries: Vec::new(),
        },
        assignments: BTreeMap::from([
            (
                SurfaceKey {
                    material: "z-source".to_owned(),
                    repeat: false,
                },
                assignment,
            ),
            (
                SurfaceKey {
                    material: "a-source".to_owned(),
                    repeat: false,
                },
                assignment,
            ),
        ]),
    }
}

#[test]
fn structural_guide_preserves_master_mesh_order() -> Result<(), String> {
    let mut content = MasterContent::default();
    content.meshes = vec![
        ordered_mesh("z-mesh", "z-source", 10.)?,
        ordered_mesh("a-mesh", "a-source", 1.)?,
    ];
    let (guide, _counts) =
        build(&content, &ordered_atlas()).map_err(|error| error.to_string())?;
    let first = guide
        .positions
        .first()
        .copied()
        .ok_or_else(|| "structural guide lost source positions".to_owned())?;
    if first != [10., 0., 0.] {
        return Err(format!("structural guide mesh order changed: {first:?}"));
    }
    Ok(())
}
