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
//   - Vehicle Physics Asset plan-evidence renderer regression tests.
// - Must-Not:
//   - Depend on external source trees or Unreal Editor state.
// - Allows:
//   - Synthetic verified vehicle physics evidence.
// - Split-When:
//   - Shape-family fixtures gain independent lifecycle.
// - Merge-When:
//   - The renderer no longer has a distinct native-projection contract.
// - Summary:
//   - Vehicle physics native-projection tests.
// - Description:
//   - Proves axis conversion, source-unit retention, and fail-closed blockers.
// - Usage:
//   - Included by the local vehicle physics plan adapter in test builds.
// - Defaults:
//   - Unexpected projection drift fails explicitly.
//

//! Vehicle physics native-projection tests.

use serde_json::Value;

use super::{VEHICLE_PHYSICS_PLAN_SCHEMA, render_vehicle_physics_plan};
use crate::adapters::driven::local::unreal_vehicle_catalog::{
    VerifiedVehicleFbxArtifact, VerifiedVehiclePhysicsPrimitive,
    VerifiedVehiclePhysicsRig,
};
use crate::domain::UnrealFbxArtifactEvidence;

fn vehicle() -> VerifiedVehicleFbxArtifact {
    VerifiedVehicleFbxArtifact {
        evidence: UnrealFbxArtifactEvidence {
            package_id: "extracted-art-cars-sedana".to_owned(),
            path: "vehicle-assets/sedana/sedana.fbx".to_owned(),
            size_bytes: 10,
            sha256: "a".repeat(64),
            fbx_version: 7_700,
        },
        subcategory: "cars/road".to_owned(),
        material_slots: Vec::new(),
        presentation_parts: vec![],
        headlight_billboard_sidecars: Vec::new(),
        physics_sidecars: Vec::new(),
        physics_rigs: vec![
            VerifiedVehiclePhysicsRig {
                identity: "sedanA".to_owned(),
                joint_count: 19,
                primitives: vec![
                    VerifiedVehiclePhysicsPrimitive::OrientedBox {
                        bone_name: "sedanA".to_owned(),
                        center_m: [1.0, 2.0, 3.0],
                        axes: [
                            [1.0, 0.0, 0.0],
                            [0.0, 1.0, 0.0],
                            [0.0, 0.0, 1.0],
                        ],
                        half_extents_m: [2.0, 3.0, 4.0],
                    },
                    VerifiedVehiclePhysicsPrimitive::Sphere {
                        bone_name: "w0".to_owned(),
                        center_m: [0.25, -0.5, 0.75],
                        radius_m: 0.4,
                    },
                ],
            },
            VerifiedVehiclePhysicsRig {
                identity: "blocked".to_owned(),
                joint_count: 2,
                primitives: vec![VerifiedVehiclePhysicsPrimitive::Cylinder {
                    bone_name: "root".to_owned(),
                    center_m: [0.0, 0.0, 0.0],
                    axis: [0.0, 0.0, 1.0],
                    half_length_m: 1.0,
                    radius_m: 0.5,
                    flat_end: true,
                }],
            },
        ],
    }
}

#[test]
fn renders_exact_native_vehicle_physics_requests() -> Result<(), String> {
    let vehicle = vehicle();
    let text = render_vehicle_physics_plan(Some(std::slice::from_ref(&vehicle)))
        .map_err(|error| error.to_string())?;
    let value = serde_json::from_str::<Value>(&text)
        .map_err(|error| error.to_string())?;
    if value.get("schema").and_then(Value::as_str)
        != Some(VEHICLE_PHYSICS_PLAN_SCHEMA)
    {
        return Err("vehicle physics plan schema drifted".to_owned());
    }
    let counts = value
        .get("counts")
        .and_then(Value::as_object)
        .ok_or_else(|| "vehicle physics counts are missing".to_owned())?;
    for (field, expected) in [
        ("vehicles", 1_u64),
        ("rigs", 2),
        ("primitives", 3),
        ("oriented_boxes", 1),
        ("spheres", 1),
        ("cylinders", 1),
        ("native_ready_rigs", 1),
        ("native_blocked_rigs", 1),
        ("native_shapes", 2),
    ] {
        if counts.get(field).and_then(Value::as_u64) != Some(expected) {
            return Err(format!("vehicle physics count {field} drifted"));
        }
    }
    let native = value
        .get("native_construction")
        .and_then(Value::as_object)
        .ok_or_else(|| "native vehicle construction is missing".to_owned())?;
    let request = native
        .get("requests")
        .and_then(Value::as_array)
        .and_then(|rows| rows.first())
        .and_then(Value::as_object)
        .ok_or_else(|| "ready vehicle physics request is missing".to_owned())?;
    let shapes = request
        .get("shapes")
        .and_then(Value::as_array)
        .ok_or_else(|| "ready vehicle shapes are missing".to_owned())?;
    let [box_shape, sphere_shape] = shapes.as_slice() else {
        return Err("ready vehicle shape count drifted".to_owned());
    };
    if box_shape.get("center") != Some(&serde_json::json!([1.0, -2.0, 3.0]))
        || box_shape.get("extents")
            != Some(&serde_json::json!([4.0, 6.0, 8.0]))
        || box_shape.get("axes")
            != Some(&serde_json::json!([
                [1.0, 0.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, 0.0, -1.0]
            ]))
    {
        return Err("oriented-box target projection drifted".to_owned());
    }
    if sphere_shape.get("center")
        != Some(&serde_json::json!([0.25, 0.5, 0.75]))
        || sphere_shape.get("radius")
            != Some(
                &serde_json::to_value(0.4_f32)
                    .map_err(|error| error.to_string())?
            )
    {
        return Err("sphere target projection drifted".to_owned());
    }
    let blocker = native
        .get("blockers")
        .and_then(Value::as_array)
        .and_then(|rows| rows.first())
        .ok_or_else(|| "cylinder blocker is missing".to_owned())?;
    if blocker
        .get("blockers")
        .and_then(Value::as_array)
        .and_then(|rows| rows.first())
        .and_then(Value::as_str)
        != Some("source-cylinder-has-no-exact-aggregate-geometry")
    {
        return Err("cylinder blocker contract drifted".to_owned());
    }
    Ok(())
}

#[test]
fn absent_vehicle_catalog_renders_empty_native_evidence()
-> Result<(), String> {
    let text = render_vehicle_physics_plan(None)
        .map_err(|error| error.to_string())?;
    let value = serde_json::from_str::<Value>(&text)
        .map_err(|error| error.to_string())?;
    if value
        .pointer("/counts/vehicles")
        .and_then(Value::as_u64)
        != Some(0)
        || value
            .pointer("/native_construction/requests")
            .and_then(Value::as_array)
            .is_none_or(|rows| !rows.is_empty())
    {
        return Err("absent vehicle catalog did not stay explicit".to_owned());
    }
    Ok(())
}
