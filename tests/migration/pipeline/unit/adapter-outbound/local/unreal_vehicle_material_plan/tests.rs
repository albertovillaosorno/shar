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
//   - Vehicle material plan-evidence renderer regression tests.
// - Must-Not:
//   - Depend on source packages or Unreal Editor state.
// - Allows:
//   - Synthetic verified vehicle material evidence.
// - Split-When:
//   - Vehicle material target families gain independent lifecycles.
// - Merge-When:
//   - The renderer no longer has a distinct projection contract.
// - Summary:
//   - Vehicle material projection tests.
// - Description:
//   - Proves exact PDDI retention and explicit native blockers.
// - Usage:
//   - Included by the local vehicle material plan adapter in test builds.
// - Defaults:
//   - Native construction remains blocked until a vehicle toolset exists.
//

//! Vehicle material projection tests.

use serde_json::Value;

use super::{
    VEHICLE_MATERIAL_PLAN_SCHEMA, is_simple_unlit_graph_candidate,
    render_vehicle_material_plan,
};
use crate::adapters::driven::local::unreal_vehicle_catalog::{
    VerifiedVehicleFbxArtifact, VerifiedVehicleHeadlightBillboardArtifact,
    VerifiedVehicleHeadlightMaterialArtifact, VerifiedVehicleMaterialArtifact,
    VerifiedVehicleMaterialRaster, VerifiedVehicleMaterialSemantics,
    VerifiedVehiclePresentationPart,
};
use crate::domain::UnrealFbxArtifactEvidence;

fn headlight_sidecar(
    raster: VerifiedVehicleMaterialRaster,
    semantics: VerifiedVehicleMaterialSemantics,
) -> VerifiedVehicleHeadlightBillboardArtifact {
    VerifiedVehicleHeadlightBillboardArtifact {
        path: concat!(
            "vehicle-assets/sedana/presentation/headlights/",
            "glow.json"
        )
        .to_owned(),
        identity: "glowShape".to_owned(),
        shader_identity: "glow_m".to_owned(),
        bones: vec!["hll".to_owned(), "hlr".to_owned()],
        material: VerifiedVehicleHeadlightMaterialArtifact {
            source_material_name: "glow_m".to_owned(),
            base_color_rgba8: [255, 224, 128, 255],
            semantics,
            raster,
            shader_path: "vehicle-assets/sedana/shaders/glow-m.json".to_owned(),
            shader_size_bytes: 40,
            shader_sha256: "d".repeat(64),
            texture_path: Some("textures/glow.png".to_owned()),
            texture_size_bytes: Some(50),
            texture_sha256: Some("e".repeat(64)),
        },
        size_bytes: 60,
        sha256: "f".repeat(64),
    }
}

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
        material_slots: vec![VerifiedVehicleMaterialArtifact {
            slot_name: "sedanA_m".to_owned(),
            source_material_name: "sedanA_m".to_owned(),
            base_color_rgba8: [255, 255, 255, 255],
            semantics: VerifiedVehicleMaterialSemantics {
                transparent: false,
                glass: false,
                mirror: false,
                reflective: false,
                light_emitter: false,
                visual_effect: false,
            },
            raster: VerifiedVehicleMaterialRaster {
                shader_family: "simple".to_owned(),
                has_translucency: false,
                blend_mode: 0,
                alpha_test: false,
                alpha_compare: 4,
                alpha_reference_bits: Some(0.5_f32.to_bits()),
                two_sided: true,
                lit: true,
                diffuse_rgba8: [255, 255, 255, 255],
                ambient_rgba8: [0, 0, 0, 255],
                emissive_rgba8: [0, 0, 0, 255],
                specular_rgba8: [0, 0, 0, 255],
                shininess_bits: 10.0_f32.to_bits(),
                texture_reference: Some("sedanA.bmp".to_owned()),
            },
            shader_path: concat!(
                "vehicle-assets/sedana/shaders/",
                "sedana-m.json"
            )
            .to_owned(),
            shader_size_bytes: 20,
            shader_sha256: "b".repeat(64),
            texture_path: Some(
                "textures/sedanA.png".to_owned(),
            ),
            texture_size_bytes: Some(30),
            texture_sha256: Some("c".repeat(64)),
        }],
        presentation_parts: vec![],
        headlight_billboard_sidecars: Vec::new(),
        physics_sidecars: Vec::new(),
        physics_rigs: Vec::new(),
    }
}

#[test]
fn renders_exact_vehicle_material_projection_and_blockers()
-> Result<(), String> {
    let vehicle = vehicle();
    let text = render_vehicle_material_plan(Some(
        std::slice::from_ref(&vehicle),
    ))
        .map_err(|error| error.to_string())?;
    let value = serde_json::from_str::<Value>(&text)
        .map_err(|error| error.to_string())?;
    if value.get("schema").and_then(Value::as_str)
        != Some(VEHICLE_MATERIAL_PLAN_SCHEMA)
        || value.pointer("/counts/vehicles").and_then(Value::as_u64) != Some(1)
        || value.pointer("/counts/slots").and_then(Value::as_u64) != Some(1)
        || value
            .pointer("/counts/source_projection_ready_slots")
            .and_then(Value::as_u64)
            != Some(1)
        || value
            .pointer("/counts/native_graph_ready_slots")
            .and_then(Value::as_u64)
            != Some(0)
        || value
            .pointer("/counts/native_ready_slots")
            .and_then(Value::as_u64)
            != Some(0)
    {
        return Err("vehicle material plan counts drifted".to_owned());
    }
    let slot = value
        .pointer("/vehicles/0/slots/0")
        .ok_or_else(|| "vehicle material slot is missing".to_owned())?;
    if slot.pointer("/pddi/shader_family").and_then(Value::as_str)
        != Some("simple")
        || slot.pointer("/pddi/blend_mode").and_then(Value::as_u64) != Some(0)
        || slot.pointer("/pddi/lit").and_then(Value::as_bool) != Some(true)
        || slot.pointer("/pddi/diffuse_rgba8")
            != Some(&serde_json::json!([255, 255, 255, 255]))
        || slot.pointer("/pddi/ambient_rgba8")
            != Some(&serde_json::json!([0, 0, 0, 255]))
        || slot.pointer("/pddi/shininess_bits").and_then(Value::as_u64)
            != Some(u64::from(10.0_f32.to_bits()))
        || slot.get("native_status").and_then(Value::as_str) != Some("blocked")
    {
        return Err("vehicle PDDI projection drifted".to_owned());
    }
    let blockers = slot
        .get("native_blockers")
        .and_then(Value::as_array)
        .ok_or_else(|| "vehicle material blockers are missing".to_owned())?;
    let blocker_names = blockers
        .iter()
        .filter_map(Value::as_str)
        .collect::<Vec<_>>();
    if blocker_names
        != [
            "vehicle-native-material-construction-not-applied",
            "vehicle-material-slot-application-not-reviewed",
            "vehicle-native-material-graph-not-reviewed",
            "vehicle-lit-master-not-reviewed",
        ]
    {
        return Err(format!(
            "vehicle material blockers drifted: {blocker_names:?}"
        ));
    }
    Ok(())
}

#[test]
fn simple_unlit_graph_candidate_stays_separate_from_presentation_readiness()
-> Result<(), String> {
    let mut vehicle = vehicle();
    {
        let [slot] = vehicle.material_slots.as_mut_slice() else {
            return Err(
                "vehicle material fixture cardinality drifted".to_owned(),
            );
        };
        slot.raster.lit = false;
        slot.raster.blend_mode = 2;
        slot.semantics.light_emitter = true;
        slot.semantics.transparent = true;
        if !is_simple_unlit_graph_candidate(slot) {
            return Err(
                "reviewed simple-unlit graph candidate was lost".to_owned(),
            );
        }
    }
    let text = render_vehicle_material_plan(Some(
        std::slice::from_ref(&vehicle),
    ))
        .map_err(|error| error.to_string())?;
    let value = serde_json::from_str::<Value>(&text)
        .map_err(|error| error.to_string())?;
    if value
        .pointer("/counts/native_graph_ready_slots")
        .and_then(Value::as_u64)
        != Some(1)
        || value
            .pointer("/counts/native_construction_ready_slots")
            .and_then(Value::as_u64)
            != Some(1)
        || value
            .pointer("/counts/native_instance_requests")
            .and_then(Value::as_u64)
            != Some(1)
        || value
            .pointer("/counts/native_ready_slots")
            .and_then(Value::as_u64)
            != Some(0)
        || value
            .pointer("/vehicles/0/slots/0/native_construction_status")
            .and_then(Value::as_str)
            != Some("ready")
        || value
            .pointer("/native_construction/instance_requests/0/slot_index")
            .and_then(Value::as_u64)
            != Some(0)
        || value
            .pointer("/vehicles/0/slots/0/native_status")
            .and_then(Value::as_str)
            != Some("blocked")
    {
        return Err("graph readiness became native readiness".to_owned());
    }
    let [slot] = vehicle.material_slots.as_mut_slice() else {
        return Err("vehicle material fixture cardinality drifted".to_owned());
    };
    slot.raster.blend_mode = 3;
    if is_simple_unlit_graph_candidate(slot) {
        return Err(
            "unreviewed subtract blend became a graph candidate".to_owned(),
        );
    }
    Ok(())
}


#[test]
fn renders_slot_independent_headlight_sidecar_bindings() -> Result<(), String> {
    let mut vehicle = vehicle();
    let [slot] = vehicle.material_slots.as_slice() else {
        return Err("vehicle material fixture cardinality drifted".to_owned());
    };
    let mut semantics = slot.semantics;
    semantics.light_emitter = true;
    let mut raster = slot.raster.clone();
    raster.lit = false;
    vehicle.headlight_billboard_sidecars = vec![headlight_sidecar(
        raster,
        semantics,
    )];
    let text = render_vehicle_material_plan(Some(
        std::slice::from_ref(&vehicle),
    ))
        .map_err(|error| error.to_string())?;
    let value = serde_json::from_str::<Value>(&text)
        .map_err(|error| error.to_string())?;
    let bindings = value
        .pointer("/vehicles/0/dynamic_light_bindings")
        .and_then(Value::as_array)
        .ok_or_else(|| "vehicle headlight bindings are missing".to_owned())?;
    let [left, right] = bindings.as_slice() else {
        return Err(format!("unexpected headlight bindings: {bindings:?}"));
    };
    if left.get("semantic_role").and_then(Value::as_str)
        != Some("headlight-left")
        || left.get("bone_name").and_then(Value::as_str) != Some("hll")
        || left.get("material_slot_indices") != Some(&serde_json::json!([]))
        || left.get("slot_join_status").and_then(Value::as_str)
            != Some("not-applicable-native")
        || right.get("semantic_role").and_then(Value::as_str)
            != Some("headlight-right")
        || value
            .pointer("/counts/slot_independent_headlight_bindings")
            .and_then(Value::as_u64)
            != Some(2)
        || value
            .pointer("/counts/unique_slot_light_bindings")
            .and_then(Value::as_u64)
            != Some(0)
        || value
            .pointer("/counts/ambiguous_slot_light_bindings")
            .and_then(Value::as_u64)
            != Some(0)
    {
        return Err("slot-independent headlight binding drifted".to_owned());
    }
    Ok(())
}

#[test]
fn renders_verified_dynamic_light_part_binding() -> Result<(), String> {
    let mut vehicle = vehicle();
    let [slot] = vehicle.material_slots.as_mut_slice() else {
        return Err("vehicle material fixture cardinality drifted".to_owned());
    };
    slot.raster.lit = false;
    slot.raster.blend_mode = 2;
    slot.semantics.transparent = true;
    slot.semantics.light_emitter = true;
    vehicle.presentation_parts.push(VerifiedVehiclePresentationPart {
        name: "brake1shape__light-emitter".to_owned(),
        source_mesh: "brake1Shape__joint_06__instance_00".to_owned(),
        role: "light-emitter".to_owned(),
        shader: "sedanA_m".to_owned(),
        surface_semantics: vec![
            "transparent".to_owned(),
            "light-emitter".to_owned(),
        ],
        bones: vec!["brake1".to_owned()],
    });
    let text = render_vehicle_material_plan(Some(
        std::slice::from_ref(&vehicle),
    ))
        .map_err(|error| error.to_string())?;
    let value = serde_json::from_str::<Value>(&text)
        .map_err(|error| error.to_string())?;
    let binding = value
        .pointer("/vehicles/0/dynamic_light_bindings/0")
        .ok_or_else(|| "vehicle dynamic light binding is missing".to_owned())?;
    if binding
        .get("semantic_role")
        .and_then(Value::as_str)
        != Some("brake")
        || binding.get("bone_name").and_then(Value::as_str) != Some("brake1")
        || binding.get("material_slot_indices")
            != Some(&serde_json::json!([0]))
        || binding
            .get("slot_join_status")
            .and_then(Value::as_str)
            != Some("unique")
        || value
            .pointer("/counts/dynamic_light_bindings")
            .and_then(Value::as_u64)
            != Some(1)
        || value
            .pointer("/counts/unique_slot_light_bindings")
            .and_then(Value::as_u64)
            != Some(1)
    {
        return Err("vehicle dynamic light binding drifted".to_owned());
    }
    Ok(())
}

#[test]
fn absent_vehicle_catalog_renders_explicit_empty_material_evidence()
-> Result<(), String> {
    let text = render_vehicle_material_plan(None)
        .map_err(|error| error.to_string())?;
    let value = serde_json::from_str::<Value>(&text)
        .map_err(|error| error.to_string())?;
    if value.pointer("/counts/vehicles").and_then(Value::as_u64) != Some(0)
        || value.pointer("/counts/slots").and_then(Value::as_u64) != Some(0)
        || value
            .get("vehicles")
            .and_then(Value::as_array)
            .is_none_or(|rows| !rows.is_empty())
    {
        return Err("absent vehicle material evidence drifted".to_owned());
    }
    Ok(())
}
