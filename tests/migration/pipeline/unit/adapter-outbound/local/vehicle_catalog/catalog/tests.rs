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
//   - Vehicle catalog grounding-evidence regression tests.
// - Must-Not:
//   - Read private source packages or write generated vehicle artifacts.
// - Allows:
//   - Constructing deterministic in-memory vehicle records.
// - Split-When:
//   - Grounding evidence gains an independent schema lifecycle.
// - Merge-When:
//   - Vehicle catalog evidence becomes one inseparable contract.
// - Summary:
//   - Guards source-backed vehicle grounding evidence.
// - Description:
//   - Verifies catalog JSON records why a vehicle receives vertical grounding.
// - Usage:
//   - Runs through the pipeline crate unit-test boundary.
// - Defaults:
//   - Uses zero-size synthetic FBX metadata and no external files.
//

//! Vehicle catalog grounding-evidence regression tests.

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;

use super::super::model::{
    EffectAnimationRecord, EffectControllerRecord,
    EffectTextureOccurrenceRecord, EffectTextureReferenceRecord,
    GroundingRecord, VehicleRecord,
};
use super::vehicle_json;

#[test]
fn vehicle_catalog_records_source_backed_grounding() -> Result<(), String> {
    let record = VehicleRecord {
        vehicle: "family-sedan".to_owned(),
        package_id: "art/cars/family-sedan".to_owned(),
        subcategory: "cars/family-sedan".to_owned(),
        fbx_path: "family-sedan/family-sedan.fbx".to_owned(),
        fbx_bytes: 0,
        fbx_sha256: "0".repeat(64),
        summary: CharacterBinaryFbxSummary {
            geometries: 0,
            bones: 0,
            clusters: 0,
            materials: 0,
            textures: 0,
            animations: 0,
        },
        grounding: GroundingRecord {
            source: "road-wheel-surfaces",
            offset_y: 0.75,
            root_bone: "root".to_owned(),
        },
        parts: Vec::new(),
        deferred_geometry: Vec::new(),
        hidden_wheel_proxies: 0,
        animations: Vec::new(),
        effect_animation_sidecars: vec![EffectAnimationRecord {
            path: "animations/effects/light.json".to_owned(),
            identity: "light-animation".to_owned(),
            animation_type: "BQG_".to_owned(),
            source_ordinal: 20,
            controller: Some(EffectControllerRecord {
                controller_identity: "light-controller".to_owned(),
                controller_kind: "frame_controller".to_owned(),
                controller_source_ordinal: 30,
                controller_version: 0,
                controller_type: "BQG".to_owned(),
                frame_offset_bits: 0_f32.to_bits(),
                target_kind: "quad_group".to_owned(),
                target_identity: "light-billboard".to_owned(),
                target_source_ordinal: 40,
            }),
            texture_references: vec![EffectTextureReferenceRecord {
                identity: "light-frame".to_owned(),
                occurrences: vec![EffectTextureOccurrenceRecord {
                    member_id: "light-frame__ordinal_50".to_owned(),
                    source_ordinal: 50,
                    sha256: "1".repeat(64),
                }],
            }],
        }],
        textures: Vec::new(),
        shaders: Vec::new(),
    };

    let value = vehicle_json(&record);
    let grounding = value
        .get("grounding")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| {
            "vehicle catalog omitted grounding evidence".to_owned()
        })?;
    let expected_source = serde_json::json!("road-wheel-surfaces");
    if grounding.get("source") != Some(&expected_source)
        || grounding.get("offset_y") != Some(&serde_json::json!(0.75))
        || grounding.get("root_bone") != Some(&serde_json::json!("root"))
    {
        return Err(String::from("grounding evidence is incomplete"));
    }
    let effect = value
        .get("effect_animation_sidecars")
        .and_then(serde_json::Value::as_array)
        .and_then(|sidecars| sidecars.first())
        .ok_or_else(|| "vehicle effect relationship is missing".to_owned())?;
    if effect.get("source_ordinal") != Some(&20.into())
        || effect
            .get("controller")
            .and_then(|controller| controller.get("source_ordinal"))
            != Some(&30.into())
        || effect
            .get("controller")
            .and_then(|controller| controller.get("type"))
            != Some(&"BQG".into())
        || effect
            .get("controller")
            .and_then(|controller| controller.get("frame_offset"))
            != Some(&0.0.into())
        || effect
            .get("controller")
            .and_then(|controller| controller.get("target_kind"))
            != Some(&"quad_group".into())
        || effect
            .get("controller")
            .and_then(|controller| controller.get("target_source_ordinal"))
            != Some(&40.into())
        || effect
            .get("texture_references")
            .and_then(serde_json::Value::as_array)
            .and_then(|references| references.first())
            .and_then(|reference| reference.get("occurrences"))
            .and_then(serde_json::Value::as_array)
            .and_then(|occurrences| occurrences.first())
            .and_then(|occurrence| occurrence.get("source_ordinal"))
            != Some(&50.into())
    {
        return Err("vehicle effect relationship is incomplete".to_owned());
    }
    Ok(())
}
