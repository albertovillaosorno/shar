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
//   - Binary character writer loose unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Binary character writer loose unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Assertions fail explicitly.
//

//! Binary character writer loose unit tests.

use super::*;
use crate::domain::skeleton::Bone;
use crate::domain::skin::SkinInfluence;

#[test]
fn near_standard_rate_uses_custom_time_mode() {
    assert_eq!(frame_rate_time_mode(30.000_000_000_5_f64), 14_i32);
}

#[test]
fn cluster_bones_preserve_skeleton_ordinal_order() -> Result<(), String> {
    let group = PrimitiveGroup::new(
        0,
        "skin",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("primitive group failed: {error:?}"))?;
    let mesh = MeshAsset::new("body", vec![group])
        .map_err(|error| format!("mesh failed: {error:?}"))?;
    let influences = (0_u32..3)
        .flat_map(|vertex_index| {
            [
                SkinInfluence {
                    vertex_index,
                    bone_id: "z_root".to_owned(),
                    weight: 0.5,
                },
                SkinInfluence {
                    vertex_index,
                    bone_id: "a_child".to_owned(),
                    weight: 0.5,
                },
            ]
        })
        .collect();
    let rest_matrix = [
        1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
    ];
    let character = CharacterAsset::new(
        "ordered-bones",
        vec![
            Bone {
                id: "z_root".to_owned(),
                parent_id: None,
                rest_matrix,
                source_identity: None,
                source_rig: None,
            },
            Bone {
                id: "a_child".to_owned(),
                parent_id: Some("z_root".to_owned()),
                rest_matrix,
                source_identity: None,
                source_rig: None,
            },
        ],
        vec![SkinnedPart {
            mesh,
            group_influences: vec![influences],
        }],
    )
    .map_err(|error| format!("character failed: {error:?}"))?;
    let materials = vec![
        MaterialBinding::new("skin", None)
            .map_err(|error| format!("material failed: {error:?}"))?,
    ];
    let material_plan = material_slots(&character, &materials)
        .map_err(|error| format!("material plan failed: {error:?}"))?;
    let groups = binary_groups(&character, &material_plan)
        .map_err(|error| format!("binary groups failed: {error:?}"))?;
    let first = groups.first().ok_or("binary group is missing")?;

    assert_eq!(first.used_bones, ["z_root", "a_child"]);
    Ok(())
}

#[test]
fn static_material_slot_plan_preserves_geometry_semantic_variants()
-> Result<(), String> {
    let group = || {
        PrimitiveGroup::new(
            0,
            "surface",
            vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
            Vec::new(),
            &[0, 1, 2],
        )
        .map_err(|error| format!("primitive group failed: {error:?}"))
    };
    let opaque = MeshAsset::new("body", vec![group()?])
        .map_err(|error| format!("opaque mesh failed: {error:?}"))?;
    let glass = MeshAsset::new("window_glass", vec![group()?])
        .map_err(|error| format!("glass mesh failed: {error:?}"))?;
    let materials = vec![
        MaterialBinding::new("surface", None)
            .map_err(|error| format!("material failed: {error:?}"))?,
    ];
    let slots = static_model_material_slots(
        "semantic-variants",
        &[opaque, glass],
        &materials,
    )
    .map_err(|error| format!("slot planning failed: {error:?}"))?;
    assert_eq!(slots.len(), 2);
    let mut slots = slots.iter();
    let opaque_slot = slots.next().ok_or("opaque slot is missing")?;
    let glass_slot = slots.next().ok_or("glass slot is missing")?;
    assert!(slots.next().is_none());
    assert_eq!(opaque_slot.material_name, "surface");
    assert_eq!(opaque_slot.source_material_name, "surface");
    assert!(!opaque_slot.semantics.is_transparent());
    assert_eq!(glass_slot.material_name, "surface__glass");
    assert_eq!(glass_slot.source_material_name, "surface");
    assert!(glass_slot.semantics.is_glass());
    Ok(())
}
