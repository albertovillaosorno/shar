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
//   - Normalized skeletal-model projection regression tests.
// - Must-Not:
//   - Exercise FBX serialization or Unreal importer behavior.
// - Allows:
//   - Synthetic geometry, rig, skin, animation, and grounding fixtures.
// - Split-When:
//   - Split when another normalized model family needs independent fixtures.
// - Merge-When:
//   - Merge when another test module owns identical projection evidence.
// - Summary:
//   - Normalized skeletal-model projection tests.
// - Description:
//   - Proves source basis and complete deforming-model evidence survive JSON.
// - Usage:
//   - Included only by the normalized-model adapter under cfg(test).
// - Defaults:
//   - Target-basis compensation must remain absent.
//

//! Normalized skeletal-model projection regression tests.

use fbx::domain::animation::{
    AnimationClip, BoneAnimationTrack, LocalTransformSample,
};
use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::{Bone, BoneSourceRig};
use fbx::domain::skin::SkinInfluence;

use super::super::model::GroundingRecord;
use super::{NORMALIZED_SKELETAL_MODEL_SCHEMA, normalized_model_value};

#[test]
fn projection_preserves_source_basis_geometry_rig_skin_and_animation()
-> Result<(), String> {
    let group = PrimitiveGroup::new(
        4,
        "body-m",
        vec![[1., 2., 3.], [-4., 5., -6.], [7., -8., 9.]],
        vec![[0., 0.], [1., 0.], [0., 1.]],
        &[0, 1, 2],
    )
    .map_err(|error| format!("group fixture failed: {error:?}"))?
    .with_source_identity("body-shape")
    .map_err(|error| format!("group identity failed: {error:?}"))?
    .with_source_ordinal(42)
    .with_normals(vec![[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]])
    .map_err(|error| format!("normal fixture failed: {error:?}"))?
    .with_colors(vec![
        [1., 0., 0., 1.],
        [0., 1., 0., 1.],
        [0., 0., 1., 1.],
    ])
    .map_err(|error| format!("color fixture failed: {error:?}"))?;
    let mesh = MeshAsset::new("body", vec![group])
        .map_err(|error| format!("mesh fixture failed: {error:?}"))?
        .with_source_identity("body-source")
        .map_err(|error| format!("mesh identity failed: {error:?}"))?
        .with_cast_shadow(Some(false));
    let influences = (0_u32..3)
        .map(|vertex_index| SkinInfluence {
            vertex_index,
            bone_id: "root".to_owned(),
            weight: 1.,
        })
        .collect::<Vec<_>>();
    let root = Bone {
        id: "root".to_owned(),
        source_identity: Some("source-root".to_owned()),
        parent_id: None,
        rest_matrix: [
            1., 0., 0., 0.,
            0., 1., 0., 0.,
            0., 0., 1., 0.,
            0.25, 0.5, 0.75, 1.,
        ],
        source_rig: Some(BoneSourceRig {
            dof: 3,
            free_axes: 5,
            primary_axis: 1,
            secondary_axis: 2,
            twist_axis: 4,
            mirror_map: None,
            fix_flags: Some(7),
        }),
    };
    let asset = CharacterAsset::new(
        "vehicle",
        vec![root],
        vec![SkinnedPart {
            mesh,
            group_influences: vec![influences],
        }],
    )
    .map_err(|error| format!("character fixture failed: {error:?}"))?;
    let animation = AnimationClip::new(
        "idle",
        30.,
        true,
        1,
        vec![BoneAnimationTrack {
            bone_id: "root".to_owned(),
            samples: vec![LocalTransformSample {
                translation: [10., 20., 30.],
                rotation_wxyz: [1., 0., 0., 0.],
            }],
        }],
        Vec::new(),
    )
    .map_err(|error| format!("animation fixture failed: {error:?}"))?;
    let grounding = GroundingRecord {
        source: "synthetic-wheel-surfaces",
        offset_y: 0.125,
        root_bone: "root".to_owned(),
    };

    let value = normalized_model_value(&asset, &[animation], &grounding);
    if value.get("schema") != Some(&NORMALIZED_SKELETAL_MODEL_SCHEMA.into()) {
        return Err("normalized model schema drifted".to_owned());
    }
    let coordinates = value
        .get("coordinate_system")
        .ok_or_else(|| "coordinate contract is missing".to_owned())?;
    if coordinates.get("right_axis") != Some(&"+X".into())
        || coordinates.get("up_axis") != Some(&"+Y".into())
        || coordinates.get("forward_axis") != Some(&"+Z".into())
        || coordinates.get("unit") != Some(&"meter".into())
    {
        return Err("source coordinate contract drifted".to_owned());
    }
    let expected = [
        ("/normalization/target_basis_applied", serde_json::json!(false)),
        (
            "/parts/0/primitive_groups/0/positions/0",
            serde_json::json!([1., 2., 3.]),
        ),
        (
            "/parts/0/primitive_groups/0/triangles/0",
            serde_json::json!([0, 1, 2]),
        ),
        (
            "/parts/0/primitive_groups/0/skin_influences/2/bone_id",
            serde_json::json!("root"),
        ),
        ("/bones/0/rest_matrix_row_major/12", serde_json::json!(0.25)),
        ("/bones/0/source_rig/dof", serde_json::json!(3)),
        (
            "/animations/0/tracks/0/samples/0/translation",
            serde_json::json!([10., 20., 30.]),
        ),
    ];
    for (pointer, expected_value) in expected {
        let actual = value
            .pointer(pointer)
            .ok_or_else(|| format!("normalized model omitted {pointer}"))?;
        if actual != &expected_value {
            return Err(format!(
                "normalized model drifted at {pointer}: {actual:?}"
            ));
        }
    }
    Ok(())
}
