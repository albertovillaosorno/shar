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
use fbx::domain::texture::MaterialSemantics;

use super::super::model::{
    EffectAnimationRecord, EffectControllerRecord,
    EffectTextureOccurrenceRecord, EffectTextureReferenceRecord,
    GroundingRecord, HeadlightBillboardMaterialRecord,
    HeadlightBillboardSidecarRecord, NormalizedModelArtifactRecord,
    PhysicsPrimitiveRecord, PhysicsRigRecord,
    PhysicsSidecarRecord, VehicleRecord,
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
        normalized_model: NormalizedModelArtifactRecord {
            path: "model.normalized.json".to_owned(),
            bytes: 1234,
            sha256: "a".repeat(64),
            parts: 7,
            bones: 9,
            animations: 2,
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
        headlight_billboard_sidecars: vec![HeadlightBillboardSidecarRecord {
            path: "presentation/headlights/headlight.json".to_owned(),
            identity: "headlightShape".to_owned(),
            shader_identity: "headlight_m".to_owned(),
            bones: vec!["hll".to_owned(), "hlr".to_owned()],
            material: HeadlightBillboardMaterialRecord {
                source_material_name: "headlight_m".to_owned(),
                base_color_rgba8: [255, 220, 128, 255],
                semantics: MaterialSemantics::default()
                    .with_transparent(true)
                    .with_light_emitter(true),
                shader_path: "shaders/headlight-m.json".to_owned(),
                shader_bytes: 32,
                shader_sha256: "4".repeat(64),
                texture_path: Some("textures/headlight.png".to_owned()),
                texture_bytes: Some(48),
                texture_sha256: Some("5".repeat(64)),
            },
            bytes: 64,
            sha256: "3".repeat(64),
        }],
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
                    package_member_id: "image-occurrence-50".to_owned(),
                    member_id: "light-frame__ordinal_50".to_owned(),
                    source_ordinal: 50,
                    sha256: "1".repeat(64),
                }],
            }],
        }],
        textures: Vec::new(),
        shaders: Vec::new(),
        material_slots: Vec::new(),
        physics_sidecars: vec![PhysicsSidecarRecord {
            path: "physics/collision__ordinal_000321.json".to_owned(),
            package_member_id: "physics-collision".to_owned(),
            source_path: "pkg/components/collision.json".to_owned(),
            kind: "p3d-collision".to_owned(),
            source_chunk_kind: "simulation_collision_object".to_owned(),
            source_ordinal: 321,
            bytes: 42,
            sha256: "2".repeat(64),
        }],
        physics_rigs: vec![PhysicsRigRecord {
            identity: "family-sedan".to_owned(),
            joint_count: 2,
            primitives: vec![PhysicsPrimitiveRecord::Sphere {
                bone_name: "w0".to_owned(),
                center_m: [0.0, 0.0, 0.0],
                radius_m: 0.5,
            }],
        }],
    };

    let value = vehicle_json(&record);
    let normalized = value
        .get("normalized_model")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "vehicle catalog omitted normalized model".to_owned())?;
    if normalized.get("path") != Some(&"model.normalized.json".into())
        || normalized.get("bytes") != Some(&1234.into())
        || normalized.get("sha256") != Some(&"a".repeat(64).into())
        || normalized.get("parts") != Some(&7.into())
        || normalized.get("bones") != Some(&9.into())
        || normalized.get("animations") != Some(&2.into())
    {
        return Err("normalized model evidence is incomplete".to_owned());
    }
    if value
        .get("fbx")
        .and_then(|fbx| fbx.get("deprecated"))
        != Some(&true.into())
    {
        return Err("FBX compatibility artifact is not deprecated".to_owned());
    }
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
    let physics = value
        .get("physics_sidecars")
        .and_then(serde_json::Value::as_array)
        .and_then(|sidecars| sidecars.first())
        .ok_or_else(|| "vehicle physics sidecar is missing".to_owned())?;
    if physics.get("source_ordinal") != Some(&321.into())
        || physics.get("package_member_id") != Some(&"physics-collision".into())
        || physics.get("bytes") != Some(&42.into())
    {
        return Err("vehicle physics provenance is incomplete".to_owned());
    }

    let headlight = value
        .get("headlight_billboard_sidecars")
        .and_then(serde_json::Value::as_array)
        .and_then(|sidecars| sidecars.first())
        .ok_or_else(|| {
            "vehicle headlight billboard sidecar is missing".to_owned()
        })?;
    if headlight.get("identity") != Some(&"headlightShape".into())
        || headlight.get("shader_identity") != Some(&"headlight_m".into())
        || headlight.get("bones")
            != Some(&serde_json::json!(["hll", "hlr"]))
        || headlight.get("bytes") != Some(&64.into())
        || headlight
            .get("material")
            .and_then(|material| material.get("source_material_name"))
            != Some(&"headlight_m".into())
        || headlight
            .get("material")
            .and_then(|material| material.get("base_color_rgba8"))
            != Some(&serde_json::json!([255, 220, 128, 255]))
        || headlight
            .get("material")
            .and_then(|material| material.get("texture"))
            .and_then(|texture| texture.get("bytes"))
            != Some(&48.into())
    {
        return Err(
            "vehicle headlight billboard evidence is incomplete".to_owned(),
        );
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
        || effect
            .get("texture_references")
            .and_then(serde_json::Value::as_array)
            .and_then(|references| references.first())
            .and_then(|reference| reference.get("occurrences"))
            .and_then(serde_json::Value::as_array)
            .and_then(|occurrences| occurrences.first())
            .and_then(|occurrence| occurrence.get("package_member_id"))
            != Some(&"image-occurrence-50".into())
    {
        return Err("vehicle effect relationship is incomplete".to_owned());
    }
    let rig = value
        .get("physics_rigs")
        .and_then(serde_json::Value::as_array)
        .and_then(|rigs| rigs.first())
        .ok_or_else(|| "vehicle physics rig is missing".to_owned())?;
    if rig.get("coordinate_space") != Some(&"source-bone-local".into())
        || rig.get("unit") != Some(&"meter".into())
        || rig.get("joint_count") != Some(&2.into())
    {
        return Err("physics rig handoff contract changed".to_owned());
    }
    let primitive = rig
        .get("primitives")
        .and_then(serde_json::Value::as_array)
        .and_then(|primitives| primitives.first())
        .ok_or_else(|| "vehicle physics primitive is missing".to_owned())?;
    if primitive.get("kind") != Some(&"sphere".into())
        || primitive.get("bone_name") != Some(&"w0".into())
        || primitive.get("radius_m") != Some(&serde_json::json!(0.5))
    {
        return Err("physics primitive handoff contract changed".to_owned());
    }
    Ok(())
}
