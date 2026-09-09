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
//   - construction readiness without approximating lit or runtime state.
// - Usage:
//   - Consumed after vehicle-catalog verification and before editor mutation.
// - Defaults:
//   - Every slot remains native-blocked until the vehicle material toolset is
//   - reviewed and implemented.
//

//! Verified vehicle material plan-evidence renderer.

use serde_json::{Value, json};

use super::unreal_vehicle_catalog::{
    VerifiedVehicleFbxArtifact, VerifiedVehicleMaterialArtifact,
};
use crate::domain::{PipelineError, PipelineOutcome};

pub(super) const VEHICLE_MATERIAL_PLAN_SCHEMA: &str =
    "shar-schoenwald.unreal-vehicle-material-evidence.v1";
const SOURCE_SCHEMA: &str = "shar.vehicle-catalog.v8";
const NATIVE_TOOLSET_BLOCKER: &str =
    "vehicle-native-material-toolset-not-reviewed";

/// Render exact verified vehicle material state and native-readiness blockers.
pub(super) fn render_vehicle_material_plan(
    catalog: Option<&[VerifiedVehicleFbxArtifact]>,
) -> PipelineOutcome<String> {
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
            vehicles.push(json!({
                "package_id": vehicle.evidence.package_id,
                "source_fbx": vehicle.evidence.path,
                "subcategory": vehicle.subcategory,
                "slots": slots,
            }));
        }
    }
    if counts.slots != counts.native_blocked_slots
        || counts.native_ready_slots != 0
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
            "native_construction": "blocked-until-vehicle-toolset-reviewed",
            "runtime_shader_mutation": "preserve-separately"
        },
        "counts": counts.value(),
        "vehicles": vehicles,
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
    presentation_special_slots: usize,
    native_ready_slots: usize,
    native_blocked_slots: usize,
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
        self.presentation_special_slots = self
            .presentation_special_slots
            .saturating_add(usize::from(has_special_presentation(slot)));
        self.native_blocked_slots = self.native_blocked_slots.saturating_add(1);
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
            "presentation_special_slots": self.presentation_special_slots,
            "source_projection_ready_slots": self.slots,
            "native_ready_slots": self.native_ready_slots,
            "native_blocked_slots": self.native_blocked_slots,
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
            "presentation_special": has_special_presentation(slot),
        },
        "native_status": "blocked",
        "native_blockers": native_blockers(slot),
    })
}

fn is_simple_unlit_graph_candidate(
    slot: &VerifiedVehicleMaterialArtifact,
) -> bool {
    slot.raster.shader_family == "simple"
        && !slot.raster.lit
        && matches!(slot.raster.blend_mode, 0..=2)
        && slot.raster.alpha_compare == 4
        && (!slot.raster.alpha_test
            || slot.raster.alpha_reference_bits.is_some())
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

fn native_blockers(
    slot: &VerifiedVehicleMaterialArtifact,
) -> Vec<&'static str> {
    let mut blockers = vec![NATIVE_TOOLSET_BLOCKER];
    match slot.raster.shader_family.as_str() {
        "spheremap" => blockers.push("vehicle-spheremap-master-not-reviewed"),
        "environment" => {
            blockers.push("vehicle-environment-master-not-reviewed");
        },
        _ => {},
    }
    if slot.raster.lit {
        blockers.push("vehicle-lit-master-not-reviewed");
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
        blockers.push("vehicle-runtime-light-material-policy-not-reviewed");
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
