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
    Ok(())
}
