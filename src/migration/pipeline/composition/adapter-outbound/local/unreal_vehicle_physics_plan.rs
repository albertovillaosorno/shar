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
//   - Editor-facing projection of verified vehicle collision recipes.
// - Must-Not:
//   - Read source catalogs, create Unreal objects, or approximate source
//   - shapes.
// - Allows:
//   - Project exact sphere and oriented-box recipes into imported bone-local
//   - coordinates and report unsupported native shape families as blockers.
// - Split-When:
//   - Physics Asset construction gains an independent publication lifecycle.
// - Merge-When:
//   - Another adapter owns the same verified vehicle-physics projection.
// - Summary:
//   - Verified vehicle Physics Asset plan-evidence renderer.
// - Description:
//   - Converts source bone-local coordinates to the current skeletal-import
//   - local frame without applying scene-unit scale a second time.
// - Usage:
//   - Published by prepare-unreal as a revision-bound semantic artifact.
// - Defaults:
//   - Unsupported primitives block their rig instead of being approximated.
//

//! Verified vehicle Physics Asset plan-evidence renderer.

use serde_json::{Value, json};

use super::unreal_vehicle_catalog::{
    VerifiedVehicleFbxArtifact, VerifiedVehiclePhysicsPrimitive,
};
use crate::domain::{PipelineError, PipelineOutcome};

pub(super) const VEHICLE_PHYSICS_PLAN_SCHEMA: &str =
    "shar-schoenwald.unreal-vehicle-physics-evidence.v1";
const SOURCE_SCHEMA: &str = "shar.vehicle-catalog.v8";
const CYLINDER_BLOCKER: &str =
    "source-cylinder-has-no-exact-aggregate-geometry";
const RIG_BINDING_BLOCKER: &str =
    "source-rig-is-not-imported-render-root";
const DIMENSION_POLICY: &str =
    "retain-source-magnitude-under-bone-scale";

/// Render verified source physics into exact native-construction requests.
pub(super) fn render_vehicle_physics_plan(
    catalog: Option<&[VerifiedVehicleFbxArtifact]>,
) -> PipelineOutcome<String> {
    let Some(catalog) = catalog else {
        return serialize_plan(&plan_value(&[], &[], Counts::default()));
    };

    let mut requests = Vec::new();
    let mut blockers = Vec::new();
    let mut counts = Counts {
        vehicles: catalog.len(),
        ..Counts::default()
    };
    for vehicle in catalog {
        for rig in &vehicle.physics_rigs {
            counts.rigs = counts.rigs.saturating_add(1);
            counts.primitives = counts
                .primitives
                .saturating_add(rig.primitives.len());
            let mut shapes = Vec::with_capacity(rig.primitives.len());
            let mut rig_blockers = Vec::new();
            for primitive in &rig.primitives {
                match primitive {
                    VerifiedVehiclePhysicsPrimitive::Sphere {
                        bone_name,
                        center_m,
                        radius_m,
                    } => {
                        counts.spheres = counts.spheres.saturating_add(1);
                        shapes.push(json!({
                            "kind": "sphere",
                            "bone_name": bone_name,
                            "center": vehicle_chaos_vector(*center_m),
                            "radius": radius_m
                        }));
                    },
                    VerifiedVehiclePhysicsPrimitive::OrientedBox {
                        bone_name,
                        center_m,
                        axes,
                        half_extents_m,
                    } => {
                        counts.oriented_boxes =
                            counts.oriented_boxes.saturating_add(1);
                        let target_axes = target_box_axes(*axes);
                        validate_target_basis(&target_axes)?;
                        let [extent0, extent1, extent2] = *half_extents_m;
                        shapes.push(json!({
                            "kind": "box",
                            "bone_name": bone_name,
                            "center": vehicle_chaos_vector(*center_m),
                            "axes": target_axes,
                            "extents": [
                                extent0 * 2.0,
                                extent1 * 2.0,
                                extent2 * 2.0
                            ]
                        }));
                    },
                    VerifiedVehiclePhysicsPrimitive::Cylinder { .. } => {
                        counts.cylinders = counts.cylinders.saturating_add(1);
                        if !rig_blockers.contains(&CYLINDER_BLOCKER) {
                            rig_blockers.push(CYLINDER_BLOCKER);
                        }
                    },
                }
            }
            if rig.identity != vehicle.render_root_bone {
                rig_blockers.push(RIG_BINDING_BLOCKER);
            }
            if rig_blockers.is_empty() {
                counts.native_ready_rigs =
                    counts.native_ready_rigs.saturating_add(1);
                counts.native_shapes =
                    counts.native_shapes.saturating_add(shapes.len());
                requests.push(json!({
                    "package_id": vehicle.evidence.package_id,
                    "source_fbx": vehicle.evidence.path,
                    "subcategory": vehicle.subcategory,
                    "rig_identity": rig.identity,
                    "joint_count": rig.joint_count,
                    "shapes": shapes
                }));
            } else {
                counts.native_blocked_rigs =
                    counts.native_blocked_rigs.saturating_add(1);
                blockers.push(json!({
                    "package_id": vehicle.evidence.package_id,
                    "source_fbx": vehicle.evidence.path,
                    "rig_identity": rig.identity,
                    "primitive_count": rig.primitives.len(),
                    "blockers": rig_blockers
                }));
            }
        }
    }
    if counts.rigs
        != counts
            .native_ready_rigs
            .saturating_add(counts.native_blocked_rigs)
    {
        return Err(PipelineError::new(
            "vehicle physics native readiness counts drifted",
        ));
    }
    serialize_plan(&plan_value(&requests, &blockers, counts))
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Counts {
    vehicles: usize,
    rigs: usize,
    primitives: usize,
    spheres: usize,
    oriented_boxes: usize,
    cylinders: usize,
    native_ready_rigs: usize,
    native_blocked_rigs: usize,
    native_shapes: usize,
}

fn plan_value(requests: &[Value], blockers: &[Value], counts: Counts) -> Value {
    json!({
        "schema": VEHICLE_PHYSICS_PLAN_SCHEMA,
        "source_schema": SOURCE_SCHEMA,
        "target_policy": {
            "source_coordinate_space": "source-bone-local",
            "source_unit": "meter",
            "skeletal_import_unit_policy": "scene-unit-converted",
            "local_axis_conversion": "source-x-y-z-to-target-z-x-y",
            "native_dimension_policy": DIMENSION_POLICY,
            "box_extent_policy": "source-half-to-native-full",
            "rig_binding_policy": "match-imported-render-root",
            "self_collision_policy": "source-empty-disable-all",
            "secondary_body_policy": "kinematic-until-joints-translated",
            "unsupported_shape_policy": "block-rig"
        },
        "counts": {
            "vehicles": counts.vehicles,
            "rigs": counts.rigs,
            "primitives": counts.primitives,
            "spheres": counts.spheres,
            "oriented_boxes": counts.oriented_boxes,
            "cylinders": counts.cylinders,
            "native_ready_rigs": counts.native_ready_rigs,
            "native_blocked_rigs": counts.native_blocked_rigs,
            "native_shapes": counts.native_shapes
        },
        "native_construction": {
            "requests": requests,
            "blockers": blockers
        }
    })
}

/// Convert SHAR lateral/up/longitudinal into Chaos forward/right/up.
const fn vehicle_chaos_vector(vector: [f32; 3]) -> [f32; 3] {
    [vector[2], vector[0], vector[1]]
}

/// Re-express one authored box basis in the same proper target basis.
fn target_box_axes(axes: [[f32; 3]; 3]) -> [[f32; 3]; 3] {
    axes.map(vehicle_chaos_vector)
}

fn validate_target_basis(axes: &[[f32; 3]; 3]) -> PipelineOutcome<()> {
    const TOLERANCE: f32 = 1.0e-5;
    let [axis0, axis1, axis2] = axes;
    let dot = |left: &[f32; 3], right: &[f32; 3]| {
        left.iter()
            .zip(right)
            .map(|(left, right)| left * right)
            .sum::<f32>()
    };
    for axis in axes {
        if (dot(axis, axis) - 1.0).abs() > TOLERANCE {
            return Err(PipelineError::new(
                "target vehicle box axis is not unit length",
            ));
        }
    }
    if dot(axis0, axis1).abs() > TOLERANCE
        || dot(axis0, axis2).abs() > TOLERANCE
        || dot(axis1, axis2).abs() > TOLERANCE
    {
        return Err(PipelineError::new(
            "target vehicle box axes are not orthogonal",
        ));
    }
    let cross = [
        axis0[1] * axis1[2] - axis0[2] * axis1[1],
        axis0[2] * axis1[0] - axis0[0] * axis1[2],
        axis0[0] * axis1[1] - axis0[1] * axis1[0],
    ];
    if dot(&cross, axis2) <= 0.0 {
        return Err(PipelineError::new(
            "target vehicle box basis is reflected",
        ));
    }
    Ok(())
}

fn serialize_plan(value: &Value) -> PipelineOutcome<String> {
    let mut text = serde_json::to_string(value).map_err(|_error| {
        PipelineError::new("serialize vehicle physics evidence failed")
    })?;
    text.push('\n');
    Ok(text)
}

#[cfg(test)]
// jig-ignore-next-line: repository test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_vehicle_physics_plan/tests.rs"]
mod tests;
