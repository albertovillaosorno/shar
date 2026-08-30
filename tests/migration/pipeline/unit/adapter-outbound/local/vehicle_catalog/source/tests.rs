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

use crate::domain::package::PhaseThreePackageRow;

use super::{
    texture_key, unique_vehicle_component_paths, vehicle_mesh_paths,
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
fn texture_key_removes_extension_case_and_fixed_width_padding() {
    assert_eq!(texture_key("WindsheildT.bmp\0\0"), "windsheildt");
    assert_eq!(texture_key("homer_vWheel.PNG"), "homer_vwheel");
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
    let error = vehicle_mesh_paths(&package, root.path())
        .expect_err("historical mirror without source ordinal was accepted");
    if !error.to_string().contains("has no source chunk ordinal") {
        return Err(format!("unexpected missing-ordinal error: {error}"));
    }
    Ok(())
}

#[test]
fn source_order_projection_rejects_duplicate_ordinals() -> Result<(), String> {
    let root = TestDirectory::new("duplicate-order")?;
    create_component_files(root.path(), "mesh")?;
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
    let package = PhaseThreePackageRow::from_json_line(json)
        .map_err(|error| error.to_string())?;
    let error = vehicle_mesh_paths(&package, root.path())
        .expect_err("duplicate source ordinals were accepted");
    if !error.to_string().contains("repeats source mesh ordinal 10") {
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
