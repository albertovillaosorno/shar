// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Decoded component source test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded component source test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded component source test module.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource, read_indexed_mesh,
};
use fbx::ports::component_source::ComponentSource;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn temp_root(label: &str) -> PathBuf {
    std::env::temp_dir()
        .join(format!("fbx-decoded-{label}-{}", std::process::id()))
}

const fn valid_mesh_json() -> &'static str {
    concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    )
}

#[test]
fn indexed_mesh_preserves_authored_identity() -> Result<(), String> {
    let root = temp_root("indexed-mesh-path");
    let mesh_path = root.join("renamed-duplicate_000.json");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let json = valid_mesh_json()
        .replace("\"name\":\"mesh\"", "\"name\":\"authored-mesh\"");
    fs::write(&mesh_path, json).map_err(|error| error.to_string())?;

    let result = read_indexed_mesh(&mesh_path)
        .map_err(|error| format!("indexed mesh failed: {error:?}"));
    let _cleanup_result = fs::remove_dir_all(&root);
    let mesh = result?;
    if mesh.name != "authored-mesh" {
        return Err(format!(
            "indexed mesh lost authored identity: {}",
            mesh.name
        ));
    }
    Ok(())
}

#[test]
fn indexed_material_uses_published_shader_path() -> Result<(), String> {
    let root = temp_root("indexed-material-path");
    let shader_path = root.join("material-logical-uuid.json");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    fs::write(
        &shader_path,
        r#"{"schema":"shader","name":"authoredShader","params":[]}"#,
    )
    .map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(&root, root.join("textures"));

    let result = source
        .resolve_indexed_material(&shader_path)
        .map_err(|error| format!("indexed material failed: {error:?}"));
    let _cleanup_result = fs::remove_dir_all(&root);
    let material = result?;
    if material.material_name != "authoredShader" {
        return Err(format!(
            "indexed material lost authored identity: {}",
            material.material_name
        ));
    }
    Ok(())
}

#[test]
fn indexed_material_accepts_exact_external_texture() -> Result<(), String> {
    let root = temp_root("indexed-material-external");
    let shader_path = root.join("shader-logical-uuid.json");
    let texture_path = root.join("shared.png");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    fs::write(
        &shader_path,
        r#"{"schema":"shader","name":"sharedShader","params":[{"kind":"texture","param":"TEX","value":"shared.bmp"}]}"#,
    )
    .map_err(|error| error.to_string())?;
    fs::write(&texture_path, b"png-payload")
        .map_err(|error| error.to_string())?;
    let output_dir = root.join("staged");
    let source = DecodedComponentSource::new(&root, &output_dir);

    let result = source
        .resolve_indexed_material_with_external_texture(
            &shader_path,
            &texture_path,
        )
        .map_err(|error| {
            format!("indexed external material failed: {error:?}")
        });
    let material = result?;
    let staged = output_dir.join("shared.png");
    if material.material_name != "sharedShader"
        || material.texture_file_name.as_deref() != Some("shared.png")
        || fs::read(&staged).map_err(|error| error.to_string())?
            != b"png-payload"
    {
        let _cleanup_result = fs::remove_dir_all(&root);
        return Err(
            "indexed external material lost exact payload evidence".to_owned()
        );
    }
    let _cleanup_result = fs::remove_dir_all(&root);
    Ok(())
}

#[test]
fn rejects_duplicate_texture_parameters() {
    let root = temp_root("duplicate-texture-parameter");
    let shader_dir = root.join("components").join("shader");
    let texture_dir = root.join("components").join("texture");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","value":"a.bmp"},{"#,
        r#""kind":"texture","param":"TEX","value":"b.bmp"}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| fs::write(shader_dir.join("shader.json"), shader_json))
        .and_then(|()| fs::write(texture_dir.join("a.png"), b"a"))
        .and_then(|()| fs::write(texture_dir.join("b.png"), b"b"));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::DuplicateTextureParameter {
            shader: "shader".to_owned(),
        })
    );
}

#[test]
fn rejects_non_string_texture_parameters() {
    let root = temp_root("non-string-texture");
    let shader_dir = root.join("components").join("shader");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","value":123}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::write(shader_dir.join("shader.json"), shader_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidTextureParameter {
            shader: "shader".to_owned(),
        })
    );
}

#[test]
fn rejects_unsupported_decoded_uv_channels() {
    let root = temp_root("unsupported-uv-channel");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2],"uvs":[{"#,
        r#""channel":1,"coords":[[0,0],[1,0],[0,1]]}]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("mesh.json"), mesh_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::UnsupportedUvChannel {
            group: 0,
            channel: 1,
        })
    );
}

#[test]
fn rejects_duplicate_decoded_uv_channels() {
    let root = temp_root("duplicate-uv-channel");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2],"uvs":[{"#,
        r#""channel":0,"coords":[[0,0],[1,0],[0,1]]},{"#,
        r#""channel":0,"coords":[[0,0],[1,0],[0,1]]}]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("mesh.json"), mesh_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::DuplicateUvChannel { group: 0, channel: 0 })
    );
}

#[test]
fn rejects_unknown_decoded_json_fields() {
    let root = temp_root("unknown-fields");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","extra":1,"#,
        r#""prim_groups":[{"shader":"shader","#,
        r#""positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("mesh.json"), mesh_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert!(matches!(result, Err(DecodedComponentError::Parse { .. })));
}

#[test]
fn rejects_component_ids_that_escape_the_package_root() {
    let root = temp_root("path-traversal");
    let components = root.join("components");
    let setup_result = fs::create_dir_all(&components)
        .and_then(|()| {
            fs::write(components.join("escape-mesh.json"), valid_mesh_json())
        })
        .and_then(|()| {
            fs::write(
                components.join("escape-shader.json"),
                r#"{"name":"shader","params":[]}"#,
            )
        });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let mesh_result = source.load_mesh("../escape-mesh");
    let material_result = source.resolve_material("../escape-shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        mesh_result,
        Err(DecodedComponentError::InvalidMemberId(
            "../escape-mesh".to_owned()
        ))
    );
    assert_eq!(
        material_result,
        Err(DecodedComponentError::InvalidMemberId(
            "../escape-shader".to_owned()
        ))
    );
}
