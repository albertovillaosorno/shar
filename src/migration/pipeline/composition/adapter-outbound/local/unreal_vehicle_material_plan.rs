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
//   - Editor-facing projection of verified vehicle material-slot evidence.
// - Must-Not:
//   - Create Unreal materials, reuse world-only material assumptions, or infer
//   - missing runtime shader behavior.
// - Allows:
//   - Preserve exact PDDI raster state and expose explicit native blockers.
// - Split-When:
//   - Native vehicle material construction gains an independent lifecycle.
// - Merge-When:
//   - Another adapter owns the same verified vehicle material projection.
// - Summary:
//   - Verified vehicle material plan-evidence renderer.
// - Description:
//   - Separates exact source projection readiness from native Unreal material
//   - construction readiness without approximating unsupported runtime state.
// - Usage:
//   - Consumed after vehicle-catalog verification and before editor mutation.
// - Defaults:
//   - Reviewed graph subsets may construct assets; slot publication remains
//   - blocked until its transaction validates them.
//

//! Verified vehicle material plan-evidence renderer.

use serde_json::{Value, json};

use super::unreal_vehicle_catalog::{
    VerifiedVehicleFbxArtifact, VerifiedVehicleMaterialArtifact,
};
use super::unreal_vehicle_material_native_plan::{
    VehicleMaterialNativeConstructionPlan,
    plan_vehicle_material_native_construction,
};
use crate::domain::{PipelineError, PipelineOutcome};

pub(super) const VEHICLE_MATERIAL_PLAN_SCHEMA: &str =
    "shar-schoenwald.unreal-vehicle-material-evidence.v5";
const SOURCE_SCHEMA: &str = "shar.vehicle-catalog.v8";
const NATIVE_GRAPH_BLOCKER: &str =
    "vehicle-native-material-graph-not-reviewed";
const NATIVE_CONSTRUCTION_BLOCKER: &str =
    "vehicle-native-material-construction-not-applied";
const SLOT_APPLICATION_BLOCKER: &str =
    "vehicle-material-slot-application-not-reviewed";
const LIGHT_PRESENTATION_BLOCKER: &str =
    "vehicle-light-material-application-not-reviewed";
const HEADLIGHT_GLOW_BLOCKER: &str =
    "vehicle-headlight-native-glow-not-applied";
const CONTEXT_AMBIENT_BLOCKER: &str =
    "vehicle-context-ambient-light-not-reviewed";

/// Render exact verified vehicle material state and native-readiness blockers.
pub(super) fn render_vehicle_material_plan(
    catalog: Option<&[VerifiedVehicleFbxArtifact]>,
) -> PipelineOutcome<String> {
    let native = plan_vehicle_material_native_construction(
        catalog.unwrap_or_default(),
    )?;
    let mut counts = Counts::default();
    let mut vehicles = Vec::new();
    if let Some(catalog) = catalog {
        counts.vehicles = catalog.len();
        vehicles.reserve(catalog.len());
        for vehicle in catalog {
            let mut slots = Vec::with_capacity(vehicle.material_slots.len());
            for (slot_index, slot) in
                vehicle.material_slots.iter().enumerate()
            {
                counts.record(slot);
                slots.push(slot_value(slot_index, slot));
            }
            let light_bindings = dynamic_light_bindings(vehicle)?;
            counts.record_light_bindings(&light_bindings);
            vehicles.push(json!({
                "package_id": vehicle.evidence.package_id,
                "source_fbx": vehicle.evidence.path,
                "subcategory": vehicle.subcategory,
                "slots": slots,
                "dynamic_light_bindings": light_bindings,
            }));
        }
    }
    counts.native_construction_ready_slots = native.instance_requests.len();
    counts.native_texture_requests = native.texture_requests.len();
    counts.native_master_requests = native.master_requests.len();
    counts.native_instance_requests = native.instance_requests.len();
    if counts.slots
        != counts.native_ready_slots.saturating_add(counts.native_blocked_slots)
        || counts.native_construction_ready_slots
            != counts.native_graph_ready_slots
    {
        return Err(PipelineError::new(
            "vehicle material native readiness counts drifted",
        ));
    }
    let value = json!({
        "schema": VEHICLE_MATERIAL_PLAN_SCHEMA,
        "source_schema": SOURCE_SCHEMA,
        "target_policy": {
            "source_projection": "reviewed-pddi-render-state",
            "source_projection_status": "ready",
            "world_material_policy_reuse": "forbidden",
            "native_construction":
                "simple-unlit-and-opaque-lit-texture-master-instance-ready",
            "mesh_slot_application": "blocked-pending-reviewed-transaction",
            "dynamic_light_binding":
                "headlight-sidecar-plus-slot-bound-rear-lights",
            "runtime_shader_mutation": "preserve-separately"
        },
        "counts": counts.value(),
        "vehicles": vehicles,
        "native_construction": native_construction_value(&native),
    });
    let mut text = serde_json::to_string(&value).map_err(|_error| {
        PipelineError::new("serialize vehicle material evidence failed")
    })?;
    text.push('\n');
    Ok(text)
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct Counts {
    vehicles: usize,
    slots: usize,
    simple_slots: usize,
    spheremap_slots: usize,
    environment_slots: usize,
    lit_slots: usize,
    translucent_slots: usize,
    alpha_test_slots: usize,
    two_sided_slots: usize,
    simple_unlit_graph_candidates: usize,
    simple_lit_opaque_graph_candidates: usize,
    simple_lit_source_alpha_glass_candidates: usize,
    context_ambient_light_slots: usize,
    presentation_special_slots: usize,
    native_graph_ready_slots: usize,
    native_construction_ready_slots: usize,
    native_texture_requests: usize,
    native_master_requests: usize,
    native_instance_requests: usize,
    native_ready_slots: usize,
    native_blocked_slots: usize,
    dynamic_light_bindings: usize,
    unique_slot_light_bindings: usize,
    ambiguous_slot_light_bindings: usize,
    slot_independent_headlight_bindings: usize,
}

impl Counts {
    fn record(&mut self, slot: &VerifiedVehicleMaterialArtifact) {
        self.slots = self.slots.saturating_add(1);
        match slot.raster.shader_family.as_str() {
            "simple" => self.simple_slots = self.simple_slots.saturating_add(1),
            "spheremap" => {
                self.spheremap_slots = self.spheremap_slots.saturating_add(1);
            },
            "environment" => {
                self.environment_slots =
                    self.environment_slots.saturating_add(1);
            },
            _ => {},
        }
        self.lit_slots = self
            .lit_slots
            .saturating_add(usize::from(slot.raster.lit));
        self.translucent_slots = self.translucent_slots.saturating_add(
            usize::from(slot.raster.has_translucency),
        );
        self.alpha_test_slots = self
            .alpha_test_slots
            .saturating_add(usize::from(slot.raster.alpha_test));
        self.two_sided_slots = self
            .two_sided_slots
            .saturating_add(usize::from(slot.raster.two_sided));
        self.simple_unlit_graph_candidates = self
            .simple_unlit_graph_candidates
            .saturating_add(usize::from(is_simple_unlit_graph_candidate(slot)));
        self.simple_lit_opaque_graph_candidates = self
            .simple_lit_opaque_graph_candidates
            .saturating_add(usize::from(
                is_simple_lit_opaque_graph_candidate(slot),
            ));
        self.simple_lit_source_alpha_glass_candidates = self
            .simple_lit_source_alpha_glass_candidates
            .saturating_add(usize::from(
                is_simple_lit_source_alpha_glass_candidate(slot),
            ));
        self.context_ambient_light_slots = self
            .context_ambient_light_slots
            .saturating_add(usize::from(
                slot.raster.lit
                    && slot.raster.ambient_rgba8 != [0, 0, 0, 255],
            ));
        self.presentation_special_slots = self
            .presentation_special_slots
            .saturating_add(usize::from(has_special_presentation(slot)));
        if is_reviewed_native_graph_candidate(slot) {
            self.native_graph_ready_slots =
                self.native_graph_ready_slots.saturating_add(1);
        }
        self.native_blocked_slots = self.native_blocked_slots.saturating_add(1);
    }

    fn record_light_bindings(&mut self, bindings: &[Value]) {
        self.dynamic_light_bindings = self
            .dynamic_light_bindings
            .saturating_add(bindings.len());
        for binding in bindings {
            let unique = binding
                .get("slot_join_status")
                .and_then(Value::as_str)
                == Some("unique");
            if unique {
                self.unique_slot_light_bindings =
                    self.unique_slot_light_bindings.saturating_add(1);
            } else if binding
                .get("slot_join_status")
                .and_then(Value::as_str)
                == Some("ambiguous")
            {
                self.ambiguous_slot_light_bindings =
                    self.ambiguous_slot_light_bindings.saturating_add(1);
            } else {
                self.slot_independent_headlight_bindings = self
                    .slot_independent_headlight_bindings
                    .saturating_add(1);
            }
        }
    }

    fn value(self) -> Value {
        json!({
            "vehicles": self.vehicles,
            "slots": self.slots,
            "simple_slots": self.simple_slots,
            "spheremap_slots": self.spheremap_slots,
            "environment_slots": self.environment_slots,
            "lit_slots": self.lit_slots,
            "translucent_slots": self.translucent_slots,
            "alpha_test_slots": self.alpha_test_slots,
            "two_sided_slots": self.two_sided_slots,
            "simple_unlit_graph_candidates": self.simple_unlit_graph_candidates,
            "simple_lit_opaque_graph_candidates":
                self.simple_lit_opaque_graph_candidates,
            "simple_lit_source_alpha_glass_candidates":
                self.simple_lit_source_alpha_glass_candidates,
            "context_ambient_light_slots": self.context_ambient_light_slots,
            "presentation_special_slots": self.presentation_special_slots,
            "source_projection_ready_slots": self.slots,
            "native_graph_ready_slots": self.native_graph_ready_slots,
            "native_construction_ready_slots":
                self.native_construction_ready_slots,
            "native_texture_requests": self.native_texture_requests,
            "native_master_requests": self.native_master_requests,
            "native_instance_requests": self.native_instance_requests,
            "native_ready_slots": self.native_ready_slots,
            "native_blocked_slots": self.native_blocked_slots,
            "dynamic_light_bindings": self.dynamic_light_bindings,
            "unique_slot_light_bindings": self.unique_slot_light_bindings,
            "ambiguous_slot_light_bindings": self.ambiguous_slot_light_bindings,
            "slot_independent_headlight_bindings":
                self.slot_independent_headlight_bindings,
        })
    }
}

fn slot_value(
    slot_index: usize,
    slot: &VerifiedVehicleMaterialArtifact,
) -> Value {
    json!({
        "slot_index": slot_index,
        "slot_name": slot.slot_name,
        "source_material_name": slot.source_material_name,
        "base_color_rgba8": slot.base_color_rgba8,
        "pddi": {
            "shader_family": slot.raster.shader_family,
            "has_translucency": slot.raster.has_translucency,
            "blend_mode": slot.raster.blend_mode,
            "alpha_test": slot.raster.alpha_test,
            "alpha_compare": slot.raster.alpha_compare,
            "alpha_reference_bits": slot.raster.alpha_reference_bits,
            "two_sided": slot.raster.two_sided,
            "lit": slot.raster.lit,
            "diffuse_rgba8": slot.raster.diffuse_rgba8,
            "ambient_rgba8": slot.raster.ambient_rgba8,
            "emissive_rgba8": slot.raster.emissive_rgba8,
            "specular_rgba8": slot.raster.specular_rgba8,
            "shininess_bits": slot.raster.shininess_bits,
            "texture_reference": slot.raster.texture_reference,
        },
        "surface_semantics": {
            "transparent": slot.semantics.transparent,
            "glass": slot.semantics.glass,
            "mirror": slot.semantics.mirror,
            "reflective": slot.semantics.reflective,
            "light_emitter": slot.semantics.light_emitter,
            "visual_effect": slot.semantics.visual_effect,
        },
        "shader": {
            "path": slot.shader_path,
            "bytes": slot.shader_size_bytes,
            "sha256": slot.shader_sha256,
        },
        "texture": slot.texture_path.as_ref().map(|path| json!({
            "path": path,
            "bytes": slot.texture_size_bytes,
            "sha256": slot.texture_sha256,
        })),
        "source_projection_status": "ready",
        "native_graph_review": {
            "simple_unlit_candidate": is_simple_unlit_graph_candidate(slot),
            "simple_lit_opaque_candidate":
                is_simple_lit_opaque_graph_candidate(slot),
            "simple_lit_source_alpha_glass_candidate":
                is_simple_lit_source_alpha_glass_candidate(slot),
            "presentation_special": has_special_presentation(slot),
        },
        "native_construction_status": if is_reviewed_native_graph_candidate(
            slot,
        ) {
            "ready"
        } else {
            "blocked"
        },
        "native_status": "blocked",
        "native_blockers": native_blockers(slot),
    })
}

pub(super) fn is_simple_unlit_graph_candidate(
    slot: &VerifiedVehicleMaterialArtifact,
) -> bool {
    slot.raster.shader_family == "simple"
        && !slot.raster.lit
        && matches!(slot.raster.blend_mode, 0..=2)
        && slot.raster.alpha_compare == 4
        && (!slot.raster.alpha_test
            || slot.raster.alpha_reference_bits.is_some())
}

pub(super) fn is_simple_lit_opaque_graph_candidate(
    slot: &VerifiedVehicleMaterialArtifact,
) -> bool {
    let shininess = f32::from_bits(slot.raster.shininess_bits);
    slot.raster.shader_family == "simple"
        && slot.raster.lit
        && !slot.raster.has_translucency
        && slot.raster.blend_mode == 0
        && !slot.raster.alpha_test
        && slot.raster.alpha_compare == 4
        && slot.raster.ambient_rgba8 == [0, 0, 0, 255]
        && slot.raster.specular_rgba8 == [0, 0, 0, 255]
        && slot.raster.emissive_rgba8 == [0, 0, 0, 255]
        && slot.raster.texture_reference.is_some()
        && slot.texture_path.is_some()
        && shininess.is_finite()
        && (0.0..=128.0).contains(&shininess)
        && !has_special_presentation(slot)
}

pub(super) fn is_simple_lit_source_alpha_glass_candidate(
    slot: &VerifiedVehicleMaterialArtifact,
) -> bool {
    let shininess = f32::from_bits(slot.raster.shininess_bits);
    slot.raster.shader_family == "simple"
        && slot.raster.lit
        && slot.raster.has_translucency
        && slot.raster.blend_mode == 1
        && !slot.raster.alpha_test
        && slot.raster.alpha_compare == 4
        && slot.raster.diffuse_rgba8 == [255, 255, 255, 255]
        && slot.raster.specular_rgba8 == [0, 0, 0, 255]
        && slot.raster.emissive_rgba8 == [0, 0, 0, 255]
        && slot.raster.texture_reference.is_some()
        && slot.texture_path.is_some()
        && shininess.is_finite()
        && (0.0..=128.0).contains(&shininess)
        && slot.semantics.transparent
        && slot.semantics.glass
        && !slot.semantics.mirror
        && !slot.semantics.reflective
        && !slot.semantics.light_emitter
        && !slot.semantics.visual_effect
}

fn is_reviewed_native_graph_candidate(
    slot: &VerifiedVehicleMaterialArtifact,
) -> bool {
    is_simple_unlit_graph_candidate(slot)
        || is_simple_lit_opaque_graph_candidate(slot)
}

const fn has_special_presentation(
    slot: &VerifiedVehicleMaterialArtifact,
) -> bool {
    slot.semantics.transparent
        || slot.semantics.glass
        || slot.semantics.mirror
        || slot.semantics.reflective
        || slot.semantics.light_emitter
        || slot.semantics.visual_effect
}


fn native_construction_value(
    native: &VehicleMaterialNativeConstructionPlan,
) -> Value {
    json!({
        "texture_requests": native.texture_requests,
        "master_requests": native.master_requests,
        "instance_requests": native.instance_requests,
    })
}

fn dynamic_light_bindings(
    vehicle: &VerifiedVehicleFbxArtifact,
) -> PipelineOutcome<Vec<Value>> {
    let mut result = headlight_sidecar_bindings(vehicle)?;
    for part in &vehicle.presentation_parts {
        if !part
            .surface_semantics
            .iter()
            .any(|semantic| semantic == "light-emitter")
        {
            continue;
        }
        let slot_indices = vehicle
            .material_slots
            .iter()
            .enumerate()
            .filter_map(|(index, slot)| {
                (slot.source_material_name == part.shader).then_some(index)
            })
            .collect::<Vec<_>>();
        if slot_indices.is_empty() {
            return Err(PipelineError::new(
                "vehicle light part lost its verified material-slot join",
            ));
        }
        for bone in &part.bones {
            let Some(semantic_role) = semantic_light_role(bone) else {
                continue;
            };
            if semantic_role.starts_with("headlight-") {
                continue;
            }
            let join_status = if slot_indices.len() == 1 {
                "unique"
            } else {
                "ambiguous"
            };
            let mut blockers = vec![LIGHT_PRESENTATION_BLOCKER];
            if slot_indices.len() != 1 {
                blockers.push("vehicle-light-material-slot-join-ambiguous");
            }
            result.push(json!({
                "binding_id": format!("{}::{}", part.name, bone),
                "semantic_role": semantic_role,
                "bone_name": bone,
                "source_part_name": part.name,
                "source_mesh": part.source_mesh,
                "source_shader": part.shader,
                "material_slot_indices": slot_indices,
                "slot_join_status": join_status,
                "native_status": "blocked",
                "native_blockers": blockers,
            }));
        }
    }
    Ok(result)
}

fn headlight_sidecar_bindings(
    vehicle: &VerifiedVehicleFbxArtifact,
) -> PipelineOutcome<Vec<Value>> {
    let mut result = Vec::new();
    for sidecar in &vehicle.headlight_billboard_sidecars {
        if sidecar.shader_identity != sidecar.material.source_material_name {
            return Err(PipelineError::new(
                "vehicle headlight sidecar material identity drifted",
            ));
        }
        for bone in &sidecar.bones {
            let Some(semantic_role) = semantic_light_role(bone) else {
                return Err(PipelineError::new(
                    "vehicle headlight sidecar has unsupported hardpoint",
                ));
            };
            if !semantic_role.starts_with("headlight-") {
                return Err(PipelineError::new(
                    "vehicle headlight sidecar escaped headlight hardpoints",
                ));
            }
            result.push(json!({
                "binding_id": format!("{}::{}", sidecar.identity, bone),
                "semantic_role": semantic_role,
                "bone_name": bone,
                "source_billboard_identity": sidecar.identity,
                "source_shader": sidecar.shader_identity,
                "material_slot_indices": [],
                "slot_join_status": "not-applicable-native",
                "native_status": "blocked",
                "native_blockers": [HEADLIGHT_GLOW_BLOCKER],
            }));
        }
    }
    Ok(result)
}

fn semantic_light_role(bone: &str) -> Option<&'static str> {
    match bone {
        "hll" => Some("headlight-left"),
        "hlr" => Some("headlight-right"),
        "brake1" | "brake2" | "brake3" | "brake4" => Some("brake"),
        "rev1" | "rev2" | "rev3" | "rev4" => Some("reverse"),
        _ => None,
    }
}

fn native_blockers(
    slot: &VerifiedVehicleMaterialArtifact,
) -> Vec<&'static str> {
    let mut blockers = vec![
        NATIVE_CONSTRUCTION_BLOCKER,
        SLOT_APPLICATION_BLOCKER,
    ];
    if !is_reviewed_native_graph_candidate(slot) {
        blockers.push(NATIVE_GRAPH_BLOCKER);
    }
    match slot.raster.shader_family.as_str() {
        "spheremap" => blockers.push("vehicle-spheremap-master-not-reviewed"),
        "environment" => {
            blockers.push("vehicle-environment-master-not-reviewed");
        },
        _ => {},
    }
    if slot.raster.lit && !is_simple_lit_opaque_graph_candidate(slot) {
        blockers.push("vehicle-lit-master-not-reviewed");
    }
    if slot.raster.lit && slot.raster.ambient_rgba8 != [0, 0, 0, 255] {
        blockers.push(CONTEXT_AMBIENT_BLOCKER);
    }
    if slot.semantics.glass {
        blockers.push("vehicle-glass-policy-not-reviewed");
    }
    if slot.semantics.mirror {
        blockers.push("vehicle-mirror-policy-not-reviewed");
    }
    if slot.semantics.reflective {
        blockers.push("vehicle-reflection-policy-not-reviewed");
    }
    if slot.semantics.light_emitter {
        blockers.push(LIGHT_PRESENTATION_BLOCKER);
    }
    if slot.semantics.visual_effect {
        blockers.push("vehicle-vfx-material-policy-not-reviewed");
    }
    blockers
}

#[cfg(test)]
// jig-ignore-next-line: repository test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_vehicle_material_plan/tests.rs"]
mod tests;
