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
//   - World catalog unit tests.
// - Must-Not:
//   - Own production behavior or artifact publication.
// - Allows:
//   - Focused serialization assertions for deferred source relationships.
// - Split-When:
//   - Another world-catalog contract gains independent test ownership.
// - Merge-When:
//   - The world catalog no longer owns deferred relationship rendering.
// - Summary:
//   - World catalog unit tests.
// - Description:
//   - Locks source-backed deferred composite relationship serialization.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Relationship fields must remain explicit and non-inferred.
//

//! World catalog unit tests.

use super::deferred_binding_value;
use crate::adapters::driven::local::prop_catalog::model::{
    DeferredBillboardBinding, DeferredBillboardQuadBinding,
    DeferredControllerBinding, DeferredRenderBinding,
    DeferredShaderOccurrenceBinding, DeferredShaderParameterBinding,
    DeferredTextureOccurrenceBinding, DeferredTextureReferenceBinding,
};

#[test]
fn deferred_binding_serialization_keeps_exact_source_relationship(
) -> Result<(), String> {
    let value = deferred_binding_value(&DeferredRenderBinding {
        composite_prop_index: 2,
        source_identity: "light-beam".to_owned(),
        skeleton_joint_id: 7,
        is_translucent: true,
        component_kind: Some("quad_group".to_owned()),
        component_member_id: Some("light-beam__ordinal_42".to_owned()),
        source_ordinal: Some(42),
        billboard: Some(DeferredBillboardBinding {
            version: 0,
            shader_identity: "glow_m".to_owned(),
            shader_occurrences: vec![
                DeferredShaderOccurrenceBinding {
                    package_member_id: "shader-member-3".to_owned(),
                    member_id: "glow_m__ordinal_3".to_owned(),
                    source_ordinal: 3,
                    schema: Some("shader".to_owned()),
                    identity: "glow_m".to_owned(),
                    version: 0,
                    platform_shader_name: Some("simple\0".to_owned()),
                    translucency: Some(1),
                    vertex_needs: Some(33),
                    vertex_mask: Some(3),
                    parameter_count: Some(2),
                    texture_reference: Some("glow.bmp".to_owned()),
                    params: vec![
                        DeferredShaderParameterBinding {
                            kind: "texture".to_owned(),
                            param: "TEX".to_owned(),
                            value: "glow.bmp\0".into(),
                        },
                        DeferredShaderParameterBinding {
                            kind: "int".to_owned(),
                            param: "BLMD".to_owned(),
                            value: 2.into(),
                        },
                    ],
                },
                DeferredShaderOccurrenceBinding {
                    package_member_id: "shader-member-7".to_owned(),
                    member_id: "glow_m__ordinal_7".to_owned(),
                    source_ordinal: 7,
                    schema: Some("shader".to_owned()),
                    identity: "glow_m".to_owned(),
                    version: 0,
                    platform_shader_name: Some("simple\0".to_owned()),
                    translucency: Some(1),
                    vertex_needs: Some(33),
                    vertex_mask: Some(7),
                    parameter_count: Some(2),
                    texture_reference: Some("glow.bmp".to_owned()),
                    params: vec![
                        DeferredShaderParameterBinding {
                            kind: "texture".to_owned(),
                            param: "TEX".to_owned(),
                            value: "glow.bmp\0".into(),
                        },
                        DeferredShaderParameterBinding {
                            kind: "int".to_owned(),
                            param: "BLMD".to_owned(),
                            value: 3.into(),
                        },
                    ],
                },
            ],
            texture_references: vec![DeferredTextureReferenceBinding {
                identity: "glow.bmp".to_owned(),
                occurrences: vec![
                    DeferredTextureOccurrenceBinding {
                        package_id: "level-three-terrain".to_owned(),
                        subcategory: "terrain-world/level-03/terrain-mesh"
                            .to_owned(),
                        package_member_id: "texture-member-182".to_owned(),
                        member_id: "glow".to_owned(),
                        source_ordinal: 182,
                        sha256: "one".to_owned(),
                    },
                    DeferredTextureOccurrenceBinding {
                        package_id: "level-three-terrain".to_owned(),
                        subcategory: "terrain-world/level-03/terrain-mesh"
                            .to_owned(),
                        package_member_id: "texture-member-10591".to_owned(),
                        member_id: "glow__ordinal_10591".to_owned(),
                        source_ordinal: 10_591,
                        sha256: "two".to_owned(),
                    },
                ],
            }],
            z_test: 1,
            z_write: 0,
            fog: 0,
            quads: vec![DeferredBillboardQuadBinding {
                identity: "light-beam-child".to_owned(),
                version: 2,
                billboard_mode: "YAX".to_owned(),
                translation_bits: [2.5_f32, -3., 4.].map(f32::to_bits),
                colour: 305_419_896,
                uv_bits: [[0.1_f32, 0.2], [0.9, 0.2], [0.9, 0.8], [0.1, 0.8]]
                    .map(|uv| uv.map(f32::to_bits)),
                width_bits: 2.25_f32.to_bits(),
                height_bits: 4.5_f32.to_bits(),
                distance_bits: (-0.35_f32).to_bits(),
                uv_offset_bits: [0.25_f32, -0.5].map(f32::to_bits),
                rotation_wxyz_bits: [1_f32, 0., 0., 0.].map(f32::to_bits),
                cutoff_mode: "DBL".to_owned(),
                uv_offset_range_bits: [0.5_f32, 0.75].map(f32::to_bits),
                source_range_bits: 1.25_f32.to_bits(),
                edge_range_bits: 0.625_f32.to_bits(),
                perspective: false,
            }],
        }),
        controller: Some(DeferredControllerBinding {
            controller_identity: "BQG_light-beam".to_owned(),
            controller_kind: "frame_controller".to_owned(),
            controller_member_id: "BQG_light-beam".to_owned(),
            controller_source_ordinal: 43,
            controller_version: 0,
            controller_type: "BQG".to_owned(),
            frame_offset_bits: 0_f32.to_bits(),
            animation_identity: "BQG_light-beam".to_owned(),
            animation_member_id: Some("animation_0001".to_owned()),
            animation_source_ordinal: Some(41),
            animation_version: Some(0),
            animation_type: Some("BQG_".to_owned()),
            animation_source: Some(serde_json::json!({
                "identity": "BQG_light-beam",
                "version": 0,
                "frame_count": 25.0,
                "frame_rate": 30.0,
                "cyclic": false,
                "sizes": [{
                    "version": 1,
                    "pc": 152,
                    "ps2": 160,
                    "xbox": 152,
                    "gc": 152
                }],
                "group_lists": [{
                    "version": 0,
                    "declared_group_count": 1,
                    "groups": [{
                        "version": 0,
                        "identity": "light-beam-child",
                        "group_id": 0,
                        "declared_channel_count": 2,
                        "channels": [{
                            "kind": "float1",
                            "version": 0,
                            "param": "\x57\x44\x54\x5f",
                            "declared_key_count": 1,
                            "frames": [0],
                            "values": [[2.25]],
                            "metadata": []
                        }, {
                            "kind": "bool",
                            "version": 0,
                            "param": "VIS_",
                            "start_state": 1,
                            "declared_key_count": 2,
                            "raw_values": [3, 8],
                            "metadata": []
                        }]
                    }]
                }]
            })),
        }),
    });
    assert_eq!(value.get("composite_prop_index"), Some(&2.into()));
    assert_eq!(value.get("source_identity"), Some(&"light-beam".into()));
    assert_eq!(value.get("skeleton_joint_id"), Some(&7.into()));
    assert_eq!(value.get("is_translucent"), Some(&true.into()));
    assert_eq!(value.get("component_kind"), Some(&"quad_group".into()));
    assert_eq!(
        value.get("component_member_id"),
        Some(&"light-beam__ordinal_42".into())
    );
    assert_eq!(value.get("source_ordinal"), Some(&42.into()));
    let billboard = value
        .get("billboard")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| {
            "deferred billboard did not serialize as an object".to_owned()
        })?;
    assert_eq!(billboard.get("shader_identity"), Some(&"glow_m".into()));
    let shader_occurrences = billboard
        .get("shader_occurrences")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "deferred shader occurrences are missing".to_owned())?;
    assert_eq!(shader_occurrences.len(), 2);
    let first_shader = shader_occurrences
        .first()
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "first deferred shader is missing".to_owned())?;
    let second_shader = shader_occurrences
        .get(1)
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "second deferred shader is missing".to_owned())?;
    assert_eq!(
        first_shader.get("package_member_id"),
        Some(&"shader-member-3".into())
    );
    assert_eq!(
        first_shader.get("source_ordinal"),
        Some(&3.into())
    );
    assert_eq!(
        first_shader.get("texture_reference"),
        Some(&"glow.bmp".into())
    );
    let first_params = first_shader
        .get("params")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "first shader params are missing".to_owned())?;
    let second_params = second_shader
        .get("params")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "second shader params are missing".to_owned())?;
    assert_eq!(
        first_params.get(1).and_then(|value| value.get("value")),
        Some(&2.into())
    );
    assert_eq!(
        second_shader.get("source_ordinal"),
        Some(&7.into())
    );
    assert_eq!(
        second_params.get(1).and_then(|value| value.get("value")),
        Some(&3.into())
    );
    let texture_references = billboard
        .get("texture_references")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "deferred texture references are missing".to_owned())?;
    assert_eq!(texture_references.len(), 1);
    let texture_reference = texture_references
        .first()
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "deferred texture reference is missing".to_owned())?;
    assert_eq!(texture_reference.get("identity"), Some(&"glow.bmp".into()));
    let texture_occurrences = texture_reference
        .get("occurrences")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "deferred texture occurrences are missing".to_owned())?;
    assert_eq!(texture_occurrences.len(), 2);
    assert_eq!(
        texture_occurrences
            .first()
            .and_then(|value| value.get("package_member_id")),
        Some(&"texture-member-182".into())
    );
    assert_eq!(
        texture_occurrences
            .first()
            .and_then(|value| value.get("source_ordinal")),
        Some(&182.into())
    );
    assert_eq!(
        texture_occurrences
            .get(1)
            .and_then(|value| value.get("member_id")),
        Some(&"glow__ordinal_10591".into())
    );
    assert_eq!(
        texture_occurrences
            .get(1)
            .and_then(|value| value.get("sha256")),
        Some(&"two".into())
    );
    assert_eq!(billboard.get("z_test"), Some(&1.into()));
    let quads = billboard
        .get("quads")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "deferred billboard quads are missing".to_owned())?;
    let quad = quads
        .first()
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "deferred billboard child is missing".to_owned())?;
    assert_eq!(quad.get("identity"), Some(&"light-beam-child".into()));
    assert_eq!(quad.get("billboard_mode"), Some(&"YAX".into()));
    assert_eq!(
        quad.get("distance").and_then(serde_json::Value::as_f64),
        Some(f64::from(-0.35_f32))
    );
    assert_eq!(quad.get("cutoff_mode"), Some(&"DBL".into()));
    assert_eq!(quad.get("perspective"), Some(&false.into()));
    let controller = value
        .get("controller")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| {
            "deferred controller did not serialize as an object".to_owned()
        })?;
    assert_eq!(controller.get("controller_source_ordinal"), Some(&43.into()));
    assert_eq!(controller.get("controller_type"), Some(&"BQG".into()));
    assert_eq!(controller.get("frame_offset"), Some(&0.0.into()));
    assert_eq!(controller.get("animation_source_ordinal"), Some(&41.into()));
    assert_eq!(controller.get("animation_type"), Some(&"BQG_".into()));
    let animation_source = controller
        .get("animation_source")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "validated BQG source payload is missing".to_owned())?;
    assert_eq!(animation_source.get("frame_count"), Some(&25.0.into()));
    let animation_groups = animation_source
        .get("group_lists")
        .and_then(serde_json::Value::as_array)
        .and_then(|lists| lists.first())
        .and_then(|list| list.get("groups"))
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "validated BQG groups are missing".to_owned())?;
    assert_eq!(
        animation_groups
            .first()
            .and_then(|group| group.get("identity")),
        Some(&"light-beam-child".into())
    );
    assert_eq!(
        animation_groups
            .first()
            .and_then(|group| group.get("channels"))
            .and_then(|channels| channels.get(1))
            .and_then(|channel| channel.get("raw_values")),
        Some(&serde_json::json!([3, 8]))
    );
    Ok(())
}
