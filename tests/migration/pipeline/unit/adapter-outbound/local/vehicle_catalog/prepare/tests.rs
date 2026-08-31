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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::fs;
use std::path::{Path, PathBuf};

use fbx::domain::character::{
    CharacterAsset, CharacterSourceProvenance, SkinnedPart,
};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;
use fbx::domain::texture::{MaterialBinding, MaterialSemantics};

use crate::domain::package::PhaseThreePackageRow;

use super::{
    is_wheel_identity, load_vehicle_animations, partition_vehicle_billboards,
    separate_vehicle_parts, texture_state_role, vehicle_animation_name,
    vehicle_part_role, vehicle_part_semantics,
};

fn role(mesh: &str, shader: &str) -> &'static str {
    let semantics =
        vehicle_part_semantics(mesh, shader, MaterialSemantics::default());
    vehicle_part_role(mesh, shader, semantics)
}

fn ordered_vehicle_part(
    name: &str,
    shader: &str,
) -> Result<SkinnedPart, String> {
    let group = PrimitiveGroup::new(
        0,
        shader,
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("vehicle part group failed: {error:?}"))?;
    let mesh = MeshAsset::new(name, vec![group])
        .map_err(|error| format!("vehicle part mesh failed: {error:?}"))?;
    let influences = (0_u32..3)
        .map(|vertex_index| SkinInfluence {
            vertex_index,
            bone_id: "root".to_owned(),
            weight: 1.,
        })
        .collect();
    Ok(SkinnedPart {
        mesh,
        group_influences: vec![influences],
    })
}

#[test]
fn semantic_part_records_preserve_fbx_part_order() -> Result<(), String> {
    let root = Bone {
        id: "root".to_owned(),
        parent_id: None,
        rest_matrix: [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 0., 0., 1.,
        ],
        source_identity: None,
        source_rig: None,
    };
    let asset = CharacterAsset::new(
        "vehicle",
        vec![root],
        vec![
            ordered_vehicle_part("zShape", "z_m")?,
            ordered_vehicle_part("aShape", "a_m")?,
        ],
    )
    .map_err(|error| format!("vehicle fixture failed: {error:?}"))?;
    let materials = ["z_m", "a_m"]
        .into_iter()
        .map(|name| {
            MaterialBinding::new(name, None)
                .map_err(|error| format!("material failed: {error:?}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let (separated, records) = separate_vehicle_parts(asset, &materials)
        .map_err(|error| error.to_string())?;
    let part_names = separated
        .parts
        .iter()
        .map(|part| part.mesh.name.as_str())
        .collect::<Vec<_>>();
    let record_names = records
        .iter()
        .map(|record| record.name.as_str())
        .collect::<Vec<_>>();
    if record_names != part_names {
        return Err(format!(
            concat!(
                "vehicle catalog part order diverged from FBX parts: ",
                "records={:?} fbx={:?}"
            ),
            record_names,
            part_names
        ));
    }
    Ok(())
}

#[test]
fn semantic_part_split_preserves_source_cast_shadow() -> Result<(), String> {
    let root = Bone {
        id: "root".to_owned(),
        parent_id: None,
        rest_matrix: [
            1., 0., 0., 0., 0., 1., 0., 0., 0., 1., 0., 0., 0., 0., 0., 1.,
        ],
        source_identity: None,
        source_rig: None,
    };
    let mut source_part = ordered_vehicle_part("bodyShape", "body_m")?;
    source_part.mesh = source_part
        .mesh
        .with_source_identity("bodyShape")
        .map_err(|error| format!("source identity failed: {error:?}"))?
        .with_cast_shadow(Some(false));
    let provenance = CharacterSourceProvenance::new(
        "vehicle-skeleton",
        vec!["vehicle-composite".to_owned()],
    )
    .map_err(|error| format!("vehicle provenance failed: {error:?}"))?;
    let asset = CharacterAsset::new("vehicle", vec![root], vec![source_part])
        .map(|asset| asset.with_source_provenance(provenance.clone()))
        .map_err(|error| format!("vehicle fixture failed: {error:?}"))?;
    let material = MaterialBinding::new("body_m", None)
        .map_err(|error| format!("material failed: {error:?}"))?;
    let (separated, _records) = separate_vehicle_parts(asset, &[material])
        .map_err(|error| error.to_string())?;
    if separated.source_provenance.as_ref() != Some(&provenance) {
        return Err(format!(
            "vehicle source relationships changed: {:?}",
            separated.source_provenance
        ));
    }
    let mesh = separated
        .parts
        .first()
        .map(|part| &part.mesh)
        .ok_or_else(|| "vehicle semantic split lost its mesh".to_owned())?;
    if mesh.source_identity.as_deref() != Some("bodyShape") {
        return Err(format!("vehicle source mesh identity changed: {mesh:?}"));
    }
    if mesh.cast_shadow != Some(false) {
        return Err(format!("vehicle CastShadow changed: {mesh:?}"));
    }
    Ok(())
}

#[test]
fn semantic_roles_keep_moving_and_glass_parts_separate() {
    assert_eq!(role("TrunkRotShape", "trunk_m"), "trunk");
    assert_eq!(role("DoorDRotShape", "door_m"), "driver-door");
    assert_eq!(role("homer_vShape", "WindsheildT_m"), "glass");
    assert_eq!(role("w0Shape", "wheel_m"), "wheel");
}

#[test]
fn wheel_identity_does_not_capture_unrelated_body_names() {
    assert!(is_wheel_identity("wshape3"));
    assert!(is_wheel_identity("w2shape"));
    assert!(!is_wheel_identity("windowshape"));
}

#[test]
fn damage_textures_receive_a_distinct_sidecar_role() {
    assert_eq!(texture_state_role("homer_vDoorDDam.png"), "damage");
    assert_eq!(texture_state_role("homer_vSideFL.png"), "alternates");
}

struct EffectTestDirectory(PathBuf);

impl EffectTestDirectory {
    fn new(label: &str) -> Result<Self, String> {
        let path = std::env::temp_dir().join(format!(
            "shar-vehicle-effect-{label}-{}",
            std::process::id()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).map_err(|error| error.to_string())?;
        }
        fs::create_dir_all(path.join("components/animation"))
            .map_err(|error| error.to_string())?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for EffectTestDirectory {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.0));
    }
}

fn effect_animation_package() -> Result<PhaseThreePackageRow, String> {
    let json = concat!(
        r#"{"package_id":"pkg-car","package_root":"pkg-car","#,
        r#""package_category":"cars","#,
        r#""package_subcategory":"cars/test/car","#,
        r#""unit_count":2,"text_key_count":0,"#,
        r#""unit_ids":["animation-a","animation-z"],"#,
        r#""world_ids":[],"texture_ids":[],"material_ids":[],"#,
        r#""model_ids":[],"physics_ids":[],"#,
        r#""animation_ids":["animation-a","animation-z"],"#,
        r#""scene_ids":[],"locator_ids":[],"camera_ids":[],"#,
        r#""light_ids":[],"particle_ids":[],"controller_ids":[],"#,
        r#""audio_ids":[],"movie_ids":[],"script_ids":[],"#,
        r#""text_ids":[],"ui_ids":[],"metadata_ids":[],"#,
        r#""error_ids":[],"source_unit_ids":[],"text_key_ids":[],"#,
        r#""members":[{"id":"animation-a","role":"animation","#,
        r#""path":"extracted/a.json","type":"animation","#,
        r#""kind":"p3d-animation","source_chunk_kind":"animation","#,
        r#""source_chunk_ordinal":"20"},{"id":"animation-z","#,
        r#""role":"animation","path":"extracted/z.json","#,
        r#""type":"animation","kind":"p3d-animation","#,
        r#""source_chunk_kind":"animation","#,
        r#""source_chunk_ordinal":"10"}],"text_keys":[]}"#
    );
    PhaseThreePackageRow::from_json_line(json)
        .map_err(|error| error.to_string())
}

fn effect_test_asset() -> Result<CharacterAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "material",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map_err(|error| format!("effect fixture group failed: {error:?}"))?;
    let mesh = MeshAsset::new("body", vec![group])
        .map_err(|error| format!("effect fixture mesh failed: {error:?}"))?;
    let influences = (0_u32..3)
        .map(|vertex_index| SkinInfluence {
            vertex_index,
            bone_id: "root".to_owned(),
            weight: 1.,
        })
        .collect();
    CharacterAsset::new(
        "vehicle",
        vec![Bone {
            id: "root".to_owned(),
            parent_id: None,
            rest_matrix: [
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
            source_identity: None,
            source_rig: None,
        }],
        vec![SkinnedPart {
            mesh,
            group_influences: vec![influences],
        }],
    )
    .map_err(|error| format!("effect fixture asset failed: {error:?}"))
}

fn billboard_json(name: &str) -> String {
    format!(
        concat!(
            r#"{{"schema":"quad_group","version":0,"name":"{name}","#,
            r#""shader":"material","z_test":1,"z_write":1,"fog":0,"#,
            r#""num_quads":1,"quads":[{{"name":"quad","version":2,"#,
            r#""billboard_mode":"none","translation":[0,0,0],"#,
            r#""colour":4294967295,"uvs":[[0,0],[1,0],[1,1],[0,1]],"#,
            r#""width":1,"height":1,"distance":0,"uv_offset":[0,0],"#,
            r#""rotation_wxyz":[1,0,0,0],"cutoff_mode":"none","#,
            r#""uv_offset_range":[0,0],"source_range":0,"edge_range":0,"#,
            r#""perspective":false}}]}}"#
        ),
        name = name
    )
}

#[test]
fn partitioned_billboards_preserve_source_order() -> Result<(), String> {
    let root = EffectTestDirectory::new("billboard-order")?;
    let source = root.path().join("billboards");
    fs::create_dir_all(&source).map_err(|error| error.to_string())?;
    let z = source.join("z.json");
    let a = source.join("a.json");
    fs::write(&z, billboard_json("Zed")).map_err(|error| error.to_string())?;
    fs::write(&a, billboard_json("Alpha")).map_err(|error| error.to_string())?;
    let paths = [z.clone(), a.clone()];
    let output = root.path().join("output");
    let (retained, deferred) = partition_vehicle_billboards(&paths, &output)
        .map_err(|error| error.to_string())?;
    if !deferred.is_empty() {
        return Err(format!("valid billboards were deferred: {deferred:?}"));
    }
    if retained != [z, a] {
        return Err(format!("billboard source order changed: {retained:?}"));
    }
    Ok(())
}

#[test]
fn vehicle_animation_identity_requires_source_name() {
    assert!(
        vehicle_animation_name(&serde_json::json!({"type": "effect"})).is_err()
    );
    assert!(
        vehicle_animation_name(&serde_json::json!({
            "name": "\0\0",
            "type": "effect"
        }))
        .is_err()
    );
}

#[test]
fn vehicle_animation_identity_accepts_fixed_width_padding() {
    let value = serde_json::json!({
        "name": "Zebra\0\0",
        "type": "effect"
    });
    assert_eq!(vehicle_animation_name(&value).ok(), Some("Zebra"));
}

#[test]
fn effect_sidecars_reject_repaired_source_identity() -> Result<(), String> {
    let root = EffectTestDirectory::new("identity")?;
    fs::write(
        root.path().join("components/animation/z.json"),
        r#"{"name":" Zebra","type":"effect"}"#,
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.path().join("components/animation/a.json"),
        r#"{"name":"Alpha","type":"effect"}"#,
    )
    .map_err(|error| error.to_string())?;
    let package = effect_animation_package()?;
    let vehicle_dir = root.path().join("output");
    let asset = effect_test_asset()?;
    let result = load_vehicle_animations(
        &package,
        root.path(),
        &vehicle_dir,
        &asset,
    );
    if result.is_ok() {
        return Err(
            "space-padded vehicle animation identity was repaired".to_owned()
        );
    }
    Ok(())
}

#[test]
fn effect_sidecars_preserve_source_chunk_order() -> Result<(), String> {
    let root = EffectTestDirectory::new("order")?;
    fs::write(
        root.path().join("components/animation/z.json"),
        r#"{"name":"Zebra","type":"effect"}"#,
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.path().join("components/animation/a.json"),
        r#"{"name":"Alpha","type":"effect"}"#,
    )
    .map_err(|error| error.to_string())?;
    let package = effect_animation_package()?;
    let vehicle_dir = root.path().join("output");
    let asset = effect_test_asset()?;
    let (clips, sidecars) = load_vehicle_animations(
        &package,
        root.path(),
        &vehicle_dir,
        &asset,
    )
    .map_err(|error| error.to_string())?;
    if !clips.is_empty() {
        return Err("effect-only fixture produced skeletal clips".to_owned());
    }
    if sidecars
        != [
            "animations/effects/zebra.json",
            "animations/effects/alpha.json",
        ]
    {
        return Err(format!(
            "effect sidecar source order changed: {sidecars:?}"
        ));
    }
    Ok(())
}
