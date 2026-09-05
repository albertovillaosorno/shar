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

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::package::PhaseThreePackageRow;

use super::{
    VehicleTextureAuthority, common_headlight_quad_groups, decoded_name,
    texture_key, unique_vehicle_component_paths, vehicle_animation_paths,
    vehicle_mesh_paths, vehicle_package_texture_sources,
    vehicle_quad_group_paths,
};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(label: &str) -> Result<Self, String> {
        let path = std::env::temp_dir().join(format!(
            "shar-vehicle-source-{label}-{}",
            std::process::id()
        ));
        if path.exists() {
            fs::remove_dir_all(&path).map_err(|error| error.to_string())?;
        }
        fs::create_dir_all(&path).map_err(|error| error.to_string())?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.0));
    }
}

fn vehicle_row(chunk_kind: &str) -> Result<PhaseThreePackageRow, String> {
    let json = format!(
        concat!(
            "{{\"package_id\":\"pkg-car\",",
            "\"package_root\":\"pkg-car\",",
            "\"package_category\":\"cars\",",
            "\"package_subcategory\":\"cars/test/car\",",
            "\"unit_count\":2,\"text_key_count\":0,",
            "\"unit_ids\":[\"model-a\",\"model-z\"],",
            "\"world_ids\":[],\"texture_ids\":[],",
            "\"material_ids\":[],",
            "\"model_ids\":[\"model-a\",\"model-z\"],",
            "\"physics_ids\":[],\"animation_ids\":[],",
            "\"scene_ids\":[],\"locator_ids\":[],",
            "\"camera_ids\":[],\"light_ids\":[],",
            "\"particle_ids\":[],\"controller_ids\":[],",
            "\"audio_ids\":[],\"movie_ids\":[],",
            "\"script_ids\":[],\"text_ids\":[],",
            "\"ui_ids\":[],\"metadata_ids\":[],",
            "\"error_ids\":[],\"source_unit_ids\":[],",
            "\"text_key_ids\":[],\"members\":[",
            "{{\"id\":\"model-a\",\"role\":\"model\",",
            "\"path\":\"extracted/a.json\",",
            "\"type\":\"model\",\"kind\":\"p3d-mesh\",",
            "\"source_chunk_kind\":\"{}\",",
            "\"source_chunk_ordinal\":\"20\"}},",
            "{{\"id\":\"model-z\",\"role\":\"model\",",
            "\"path\":\"extracted/z.json\",",
            "\"type\":\"model\",\"kind\":\"p3d-mesh\",",
            "\"source_chunk_kind\":\"{}\",",
            "\"source_chunk_ordinal\":\"10\"}}],",
            "\"text_keys\":[]}}"
        ),
        chunk_kind, chunk_kind,
    );
    PhaseThreePackageRow::from_json_line(&json)
        .map_err(|error| error.to_string())
}

fn create_component_files(
    root: &Path,
    component_kind: &str,
) -> Result<(), String> {
    let directory = root.join("components").join(component_kind);
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    for name in ["a.json", "z.json"] {
        fs::write(directory.join(name), b"{}")
            .map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[test]
fn common_headlights_follow_source_chunk_ordinals() -> Result<(), String> {
    let root = TestDirectory::new("common-headlight-order")?;
    let common = root.path().join("cars/common");
    let directory = common.join("components/quad_group");
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    for (file, name) in [
        ("headlightShape8.json", "headlightShape8"),
        ("headlight2Shape.json", "headlight2Shape"),
        ("glowGroupShape2.json", "glowGroupShape2"),
    ] {
        fs::write(directory.join(file), format!(r#"{{"name":"{name}"}}"#))
            .map_err(|error| error.to_string())?;
    }
    fs::write(
        common.join("components.jsonl"),
        concat!(
            r#"{"ordinal":100,"name":"glowGroupShape2","#,
            r#""path":"quad_group/glowGroupShape2.json","kind":"quad_group"}"#,
            "\n",
            r#"{"ordinal":92,"name":"headlight2Shape","#,
            r#""path":"quad_group/headlight2Shape.json","kind":"quad_group"}"#,
            "\n",
            r#"{"ordinal":96,"name":"headlightShape8","#,
            r#""path":"quad_group/headlightShape8.json","kind":"quad_group"}"#,
            "\n",
        ),
    )
    .map_err(|error| error.to_string())?;
    let (_common_root, paths) = common_headlight_quad_groups(root.path())
        .map_err(|error| error.to_string())?;
    assert_eq!(
        paths,
        [
            directory.join("headlight2Shape.json"),
            directory.join("headlightShape8.json"),
            directory.join("glowGroupShape2.json"),
        ]
    );
    Ok(())
}

#[test]
fn decoded_name_rejects_surrounding_space() -> Result<(), String> {
    let root = TestDirectory::new("space-padded-name")?;
    let path = root.path().join("component.json");
    fs::write(&path, br#"{"name":" shared"}"#)
        .map_err(|error| error.to_string())?;
    if decoded_name(&path).is_ok() {
        return Err("space-padded vehicle identity was repaired".to_owned());
    }
    Ok(())
}

#[test]
fn texture_key_removes_extension_case_and_fixed_width_padding() {
    assert_eq!(
        texture_key("WindsheildT.bmp\0\0").ok().as_deref(),
        Some("windsheildt")
    );
    assert_eq!(
        texture_key("homer_vWheel.PNG").ok().as_deref(),
        Some("homer_vwheel")
    );
}

#[test]
fn texture_key_rejects_surrounding_space() {
    assert!(texture_key(" WindsheildT.bmp").is_err());
    assert!(texture_key("WindsheildT.bmp ").is_err());
}

#[test]
fn texture_authority_groups_collision_by_ledger_name() -> Result<(), String> {
    let root = TestDirectory::new("texture-ledger-collision")?;
    let texture_dir = root.path().join("components/texture");
    fs::create_dir_all(&texture_dir).map_err(|error| error.to_string())?;
    fs::write(texture_dir.join("shared.png"), b"canonical-payload")
        .and_then(|()| {
            fs::write(
                texture_dir.join("shared__ordinal_10.png"),
                b"collision-payload",
            )
        })
        .map_err(|error| error.to_string())?;
    fs::write(
        root.path().join("components.jsonl"),
        concat!(
            r#"{"ordinal":20,"name":"shared","#,
            r#""path":"texture/shared.png","kind":"texture"}"#,
            "\n",
            r#"{"ordinal":10,"name":"shared","#,
            r#""path":"texture/shared__ordinal_10.png","kind":"texture"}"#,
            "\n"
        ),
    )
    .map_err(|error| error.to_string())?;
    let package = vehicle_row("mesh")?;
    let sources = vehicle_package_texture_sources(&package, root.path())
        .map_err(|error| error.to_string())?;
    let mut grouped = BTreeMap::new();
    for (key, source) in sources {
        grouped.entry(key).or_insert_with(Vec::new).push(source);
    }
    for entries in grouped.values_mut() {
        entries.sort_by(|left, right| {
            (&left.subcategory, &left.path)
                .cmp(&(&right.subcategory, &right.path))
        });
    }
    if grouped.keys().map(String::as_str).collect::<Vec<_>>() != ["shared"]
        || grouped.get("shared").map(Vec::len) != Some(2)
    {
        return Err(
            "ledger identity did not group physical texture peers".to_owned(),
        );
    }
    let authority = VehicleTextureAuthority { sources: grouped };
    let occurrences = authority
        .preferred_occurrences("shared.bmp", &package.subcategory)
        .map_err(|error| error.to_string())?;
    let ordinals = occurrences
        .iter()
        .map(|occurrence| occurrence.source_ordinal)
        .collect::<Vec<_>>();
    let members = occurrences
        .iter()
        .map(|occurrence| occurrence.member_id.as_str())
        .collect::<Vec<_>>();
    if ordinals != [10, 20]
        || members != ["shared__ordinal_10", "shared"]
        || occurrences
            .iter()
            .any(|occurrence| occurrence.package_id != package.package_id)
        || occurrences
            .iter()
            .any(|occurrence| occurrence.subcategory != package.subcategory)
        || occurrences
            .first()
            .zip(occurrences.get(1))
            .is_none_or(|(left, right)| left.sha256 == right.sha256)
    {
        return Err(format!(
            "texture occurrence provenance changed: {occurrences:?}"
        ));
    }
    if authority.resolve("shared.bmp", &package.subcategory).is_ok() {
        return Err("differing collision payloads were selected".to_owned());
    }
    let runtime_visible = authority
        .resolve_runtime_visible("shared.bmp", &package.subcategory)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| {
            "runtime-visible collision was not selected".to_owned()
        })?;
    let visible_name = runtime_visible
        .file_name()
        .and_then(|value| value.to_str());
    if visible_name != Some("shared__ordinal_10.png") {
        return Err(format!(
            "runtime-visible collision did not follow source order: {}",
            runtime_visible.display()
        ));
    }
    Ok(())
}

#[test]
fn texture_authority_resolves_identical_collision_payloads()
-> Result<(), String> {
    let root = TestDirectory::new("texture-identical-collision")?;
    let texture_dir = root.path().join("components/texture");
    fs::create_dir_all(&texture_dir).map_err(|error| error.to_string())?;
    for file_name in ["shared.png", "shared__ordinal_10.png"] {
        fs::write(texture_dir.join(file_name), b"canonical-payload")
            .map_err(|error| error.to_string())?;
    }
    fs::write(
        root.path().join("components.jsonl"),
        concat!(
            r#"{"ordinal":20,"name":"shared","#,
            r#""path":"texture/shared.png","kind":"texture"}"#,
            "\n",
            r#"{"ordinal":10,"name":"shared","#,
            r#""path":"texture/shared__ordinal_10.png","kind":"texture"}"#,
            "\n"
        ),
    )
    .map_err(|error| error.to_string())?;
    let package = vehicle_row("mesh")?;
    let sources = vehicle_package_texture_sources(&package, root.path())
        .map_err(|error| error.to_string())?;
    let mut grouped = BTreeMap::new();
    for (key, source) in sources {
        grouped.entry(key).or_insert_with(Vec::new).push(source);
    }
    let authority = VehicleTextureAuthority { sources: grouped };
    let resolved = authority
        .resolve("shared.bmp", &package.subcategory)
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "identical collision did not resolve".to_owned())?;
    let resolved_name =
        resolved.file_name().and_then(|value| value.to_str());
    if resolved_name != Some("shared.png") {
        return Err(format!(
            "identical collision selected an unstable member: {}",
            resolved.display()
        ));
    }
    let occurrences = authority
        .preferred_occurrences("shared.bmp", &package.subcategory)
        .map_err(|error| error.to_string())?;
    if occurrences.len() != 2
        || occurrences
            .first()
            .zip(occurrences.get(1))
            .is_none_or(|(left, right)| left.sha256 != right.sha256)
    {
        return Err(format!(
            "identical collision provenance changed: {occurrences:?}"
        ));
    }
    Ok(())
}

#[test]
fn animation_paths_follow_source_chunk_ordinals() -> Result<(), String> {
    let root = TestDirectory::new("animation-order")?;
    create_component_files(root.path(), "animation")?;
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
    let package = PhaseThreePackageRow::from_json_line(json)
        .map_err(|error| error.to_string())?;
    let paths = vehicle_animation_paths(&package, root.path())
        .map_err(|error| error.to_string())?;
    assert_eq!(
        paths,
        [
            root.path().join("components/animation/z.json"),
            root.path().join("components/animation/a.json"),
        ]
    );
    Ok(())
}

#[test]
fn mesh_paths_follow_source_chunk_ordinals() -> Result<(), String> {
    let root = TestDirectory::new("mesh-order")?;
    create_component_files(root.path(), "mesh")?;
    let package = vehicle_row("mesh")?;
    let paths = vehicle_mesh_paths(&package, root.path())
        .map_err(|error| error.to_string())?;
    assert_eq!(
        paths,
        [
            root.path().join("components/mesh/z.json"),
            root.path().join("components/mesh/a.json"),
        ]
    );
    Ok(())
}

#[test]
fn quad_group_paths_follow_source_chunk_ordinals() -> Result<(), String> {
    let root = TestDirectory::new("quad-order")?;
    create_component_files(root.path(), "quad_group")?;
    let package = vehicle_row("quad_group")?;
    let paths = vehicle_quad_group_paths(&package, root.path())
        .map_err(|error| error.to_string())?;
    assert_eq!(
        paths,
        [
            root.path().join("components/quad_group/z.json"),
            root.path().join("components/quad_group/a.json"),
        ]
    );
    Ok(())
}

#[test]
fn source_order_projection_rejects_missing_ordinals() -> Result<(), String> {
    let root = TestDirectory::new("missing-order")?;
    create_component_files(root.path(), "mesh")?;
    let json = concat!(
            "{\"package_id\":\"pkg-car\",",
            "\"package_root\":\"pkg-car\",",
            "\"package_category\":\"cars\",",
            "\"package_subcategory\":\"cars/test/car\",",
            "\"unit_count\":1,\"text_key_count\":0,",
            "\"unit_ids\":[\"model-a\"],",
            "\"world_ids\":[],\"texture_ids\":[],",
            "\"material_ids\":[],\"model_ids\":[\"model-a\"],",
            "\"physics_ids\":[],\"animation_ids\":[],",
            "\"scene_ids\":[],\"locator_ids\":[],",
            "\"camera_ids\":[],\"light_ids\":[],",
            "\"particle_ids\":[],\"controller_ids\":[],",
            "\"audio_ids\":[],\"movie_ids\":[],",
            "\"script_ids\":[],\"text_ids\":[],",
            "\"ui_ids\":[],\"metadata_ids\":[],",
            "\"error_ids\":[],\"source_unit_ids\":[],",
            "\"text_key_ids\":[],\"members\":[",
            "{\"id\":\"model-a\",\"role\":\"model\",",
            "\"path\":\"extracted/a.json\",",
            "\"type\":\"model\",\"kind\":\"p3d-mesh\",",
            "\"source_chunk_kind\":\"mesh\"}],",
            "\"text_keys\":[]}"
    );
    let package = PhaseThreePackageRow::from_json_line(json)
        .map_err(|error| error.to_string())?;
    let Err(error) = vehicle_mesh_paths(&package, root.path()) else {
        return Err(
            "historical mirror without source ordinal was accepted".to_owned(),
        );
    };
    if !error.to_string().contains("has no source chunk ordinal") {
        return Err(format!("unexpected missing-ordinal error: {error}"));
    }
    Ok(())
}

#[test]
fn package_intake_rejects_duplicate_ordinals_before_projection()
-> Result<(), String> {
    let json = concat!(
            "{\"package_id\":\"pkg-car\",",
            "\"package_root\":\"pkg-car\",",
            "\"package_category\":\"cars\",",
            "\"package_subcategory\":\"cars/test/car\",",
            "\"unit_count\":2,\"text_key_count\":0,",
            "\"unit_ids\":[\"model-a\",\"model-z\"],",
            "\"world_ids\":[],\"texture_ids\":[],",
            "\"material_ids\":[],",
            "\"model_ids\":[\"model-a\",\"model-z\"],",
            "\"physics_ids\":[],\"animation_ids\":[],",
            "\"scene_ids\":[],\"locator_ids\":[],",
            "\"camera_ids\":[],\"light_ids\":[],",
            "\"particle_ids\":[],\"controller_ids\":[],",
            "\"audio_ids\":[],\"movie_ids\":[],",
            "\"script_ids\":[],\"text_ids\":[],",
            "\"ui_ids\":[],\"metadata_ids\":[],",
            "\"error_ids\":[],\"source_unit_ids\":[],",
            "\"text_key_ids\":[],\"members\":[",
            "{\"id\":\"model-a\",\"role\":\"model\",",
            "\"path\":\"extracted/a.json\",",
            "\"type\":\"model\",\"kind\":\"p3d-mesh\",",
            "\"source_chunk_kind\":\"mesh\",",
            "\"source_chunk_ordinal\":\"10\"},",
            "{\"id\":\"model-z\",\"role\":\"model\",",
            "\"path\":\"extracted/z.json\",",
            "\"type\":\"model\",\"kind\":\"p3d-mesh\",",
            "\"source_chunk_kind\":\"mesh\",",
            "\"source_chunk_ordinal\":\"10\"}],",
            "\"text_keys\":[]}"
    );
    let Err(error) = PhaseThreePackageRow::from_json_line(json) else {
        return Err(
            "duplicate source ordinals passed package intake".to_owned(),
        );
    };
    if !error.to_string().contains("repeats a source chunk ordinal") {
        return Err(format!("unexpected duplicate-ordinal error: {error}"));
    }
    Ok(())
}

#[test]
fn projected_component_paths_preserve_supplied_order() -> Result<(), String> {
    let paths = unique_vehicle_component_paths(
        [PathBuf::from("mesh/z.json"), PathBuf::from("mesh/a.json")],
        "mesh",
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(
        paths,
        [PathBuf::from("mesh/z.json"), PathBuf::from("mesh/a.json")]
    );
    Ok(())
}

#[test]
fn projected_component_path_collision_fails_closed() -> Result<(), String> {
    let result = unique_vehicle_component_paths(
        [
            PathBuf::from("components/mesh/shared.json"),
            PathBuf::from("components/mesh/shared.json"),
        ],
        "mesh",
    );
    let Err(error) = result else {
        return Err("duplicate projected vehicle path was accepted".to_owned());
    };
    assert_eq!(
        error.to_string(),
        concat!(
            "vehicle package projects duplicate mesh path: ",
            "components/mesh/shared.json"
        )
    );
    Ok(())
}
