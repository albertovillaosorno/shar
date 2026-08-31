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

use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;
use fbx::domain::texture::MaterialSemantics;

use crate::domain::package::PhaseThreePackageRow;

use super::{
    is_wheel_identity, load_vehicle_animations, texture_state_role,
    vehicle_part_role, vehicle_part_semantics,
};

fn role(mesh: &str, shader: &str) -> &'static str {
    let semantics =
        vehicle_part_semantics(mesh, shader, MaterialSemantics::default());
    vehicle_part_role(mesh, shader, semantics)
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
    fn new() -> Result<Self, String> {
        let path = std::env::temp_dir().join(format!(
            "shar-vehicle-effect-order-{}",
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
            source_rig: None,
        }],
        vec![SkinnedPart {
            mesh,
            group_influences: vec![influences],
        }],
    )
    .map_err(|error| format!("effect fixture asset failed: {error:?}"))
}

#[test]
fn effect_sidecars_preserve_source_chunk_order() -> Result<(), String> {
    let root = EffectTestDirectory::new()?;
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
