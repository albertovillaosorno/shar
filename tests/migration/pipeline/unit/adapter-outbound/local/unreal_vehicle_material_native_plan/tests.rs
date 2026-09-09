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
//   - Unit coverage for native vehicle texture, master, and instance requests.
// - Must-Not:
//   - Read production source files, contact Unreal Editor, or assign mesh
//   - slots.
// - Allows:
//   - Synthetic verified vehicle slots and exact generated-path assertions.
// - Split-When:
//   - Construction request families gain independent fixtures.
// - Merge-When:
//   - Another test module owns identical vehicle native-planning coverage.
// - Summary:
//   - Native vehicle-material request projection tests.
// - Description:
//   - Proves reviewed slots produce deterministic create-only dependencies.
// - Usage:
//   - Runs with pipeline library tests.
// - Defaults:
//   - Unsupported graph slots emit no native construction requests.
//

//! Native vehicle-material request projection tests.

use serde_json::Value;

use super::plan_vehicle_material_native_construction;
use crate::adapters::driven::local::unreal_vehicle_catalog::{
    VerifiedVehicleFbxArtifact, VerifiedVehicleMaterialArtifact,
    VerifiedVehicleMaterialRaster, VerifiedVehicleMaterialSemantics,
};
use crate::domain::UnrealFbxArtifactEvidence;

fn vehicle(
    slots: Vec<VerifiedVehicleMaterialArtifact>,
) -> VerifiedVehicleFbxArtifact {
    VerifiedVehicleFbxArtifact {
        evidence: UnrealFbxArtifactEvidence {
            package_id: "cars/sedana".to_owned(),
            path: "vehicle-assets/sedana/sedana.fbx".to_owned(),
            sha256: "a".repeat(64),
            size_bytes: 10,
            fbx_version: 7_700,
        },
        subcategory: "sedanA".to_owned(),
        material_slots: slots,
        presentation_parts: Vec::new(),
        physics_sidecars: Vec::new(),
        physics_rigs: Vec::new(),
    }
}

fn slot(
    blend: u32,
    lit: bool,
    texture: bool,
) -> VerifiedVehicleMaterialArtifact {
    VerifiedVehicleMaterialArtifact {
        slot_name: "glow2_m__transparent-light-emitter".to_owned(),
        source_material_name: "glow2_m".to_owned(),
        base_color_rgba8: [255, 128, 0, 64],
        semantics: VerifiedVehicleMaterialSemantics {
            transparent: true,
            glass: false,
            mirror: false,
            reflective: false,
            light_emitter: true,
            visual_effect: false,
        },
        raster: VerifiedVehicleMaterialRaster {
            shader_family: "simple".to_owned(),
            has_translucency: true,
            blend_mode: blend,
            alpha_test: false,
            alpha_compare: 4,
            alpha_reference_bits: Some(0.5_f32.to_bits()),
            two_sided: false,
            lit,
            diffuse_rgba8: [255; 4],
            ambient_rgba8: [255; 4],
            emissive_rgba8: [0, 0, 0, 255],
            specular_rgba8: [0, 0, 0, 255],
            shininess_bits: 10.0_f32.to_bits(),
            texture_reference: texture.then(|| "glow2.bmp".to_owned()),
        },
        shader_path: "shaders/glow2-m.json".to_owned(),
        shader_size_bytes: 20,
        shader_sha256: "b".repeat(64),
        texture_path: texture.then(|| "textures/glow2.png".to_owned()),
        texture_size_bytes: texture.then_some(30),
        texture_sha256: texture.then(|| "c".repeat(64)),
    }
}

#[test]
fn plans_vehicle_texture_master_and_per_slot_instance() -> Result<(), String> {
    let plan = plan_vehicle_material_native_construction(&[vehicle(vec![
        slot(2, false, true),
    ])])
    .map_err(|error| error.to_string())?;
    let [texture] = plan.texture_requests.as_slice() else {
        return Err("vehicle texture request cardinality drifted".to_owned());
    };
    let [master] = plan.master_requests.as_slice() else {
        return Err("vehicle master request cardinality drifted".to_owned());
    };
    let [instance] = plan.instance_requests.as_slice() else {
        return Err("vehicle instance request cardinality drifted".to_owned());
    };
    if texture.get("source_path")
        != Some(&Value::String(
            "vehicle-assets/sedana/textures/glow2.png".to_owned(),
        ))
        || texture.get("folder_path")
            != Some(&Value::String(
                "/Game/Generated/SHAR/Textures/Vehicles".to_owned(),
            ))
        || master.get("blend_mode").and_then(Value::as_u64) != Some(2)
        || master.get("lit").and_then(Value::as_bool) != Some(false)
        || instance.get("slot_index").and_then(Value::as_u64) != Some(0)
        || instance.get("set_alpha_reference").and_then(Value::as_bool)
            != Some(false)
    {
        return Err("vehicle native construction request drifted".to_owned());
    }
    let tint = instance
        .get("base_color_tint")
        .and_then(Value::as_array)
        .ok_or_else(|| "vehicle instance tint is missing".to_owned())?;
    let [red, green, blue, alpha] = tint.as_slice() else {
        return Err("vehicle instance tint cardinality drifted".to_owned());
    };
    let component = |value: &Value| {
        value
            .as_f64()
            .ok_or_else(|| {
                "vehicle instance tint component is invalid".to_owned()
            })
    };
    if (component(red)? - 1.0).abs() > f64::EPSILON
        || (component(green)? - 128.0 / 255.0).abs() > f64::EPSILON
        || component(blue)?.abs() > f64::EPSILON
        || (component(alpha)? - 64.0 / 255.0).abs() > f64::EPSILON
        || instance
            .get("base_color_texture_path")
            .and_then(Value::as_str)
            .is_none_or(|path| !path.contains("T_Vehicle_cccc"))
    {
        return Err("vehicle instance parameter projection drifted".to_owned());
    }
    Ok(())
}

#[test]
fn omits_slots_outside_reviewed_graph_subset() -> Result<(), String> {
    let plan = plan_vehicle_material_native_construction(&[vehicle(vec![
        slot(0, true, true),
    ])])
    .map_err(|error| error.to_string())?;
    if !plan.texture_requests.is_empty()
        || !plan.master_requests.is_empty()
        || !plan.instance_requests.is_empty()
    {
        return Err("blocked vehicle slot emitted native requests".to_owned());
    }
    Ok(())
}
