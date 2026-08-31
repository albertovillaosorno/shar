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
//   - Decoded mesh source test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded mesh source test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded mesh source test module.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource, read_mesh_for_analysis,
};
use fbx::ports::component_source::ComponentSource;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn temp_root(label: &str) -> PathBuf {
    std::env::temp_dir()
        .join(format!("fbx-decoded-mesh-{label}-{}", std::process::id()))
}

#[test]
fn rejects_windows_device_member_ids() {
    let root = temp_root("windows-device-name");
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("CON");

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidMemberId("CON".to_owned()))
    );
}

#[test]
fn rejects_member_ids_with_nonportable_file_characters() {
    let root = temp_root("nonportable-character");
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("mesh:stream");

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidMemberId(
            "mesh:stream".to_owned()
        ))
    );
}

#[test]
fn rejects_member_ids_with_trailing_dots() {
    let root = temp_root("trailing-dot");
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("mesh.");

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidMemberId("mesh.".to_owned()))
    );
}

#[test]
fn rejects_member_ids_with_surrounding_whitespace() {
    let root = temp_root("member-whitespace");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":" mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join(" mesh.json"), mesh_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh(" mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidMemberId(" mesh".to_owned()))
    );
}

#[test]
fn rejects_decoded_mesh_identity_with_surrounding_whitespace() {
    let root = temp_root("decoded-name-whitespace");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":" mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("mesh.json"), mesh_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::MeshIdentityMismatch {
            requested: "mesh".to_owned(),
            decoded: " mesh".to_owned(),
        })
    );
}

#[test]
fn rejects_decoded_group_shader_with_surrounding_whitespace() {
    let root = temp_root("decoded-shader-whitespace");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":" shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("mesh.json"), mesh_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::Mesh(
            fbx::domain::mesh::MeshError::NonCanonicalShader,
        ))
    );
}

#[test]
fn rejects_non_boolean_cast_shadow_status() {
    let root = temp_root("invalid-cast-shadow");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","render_status":2,"#,
        r#""prim_groups":[{"shader":"shader","#,
        r#""positions":[[0,0,0],[1,0,0],[0,1,0]],"indices":[0,1,2]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("mesh.json"), mesh_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidCastShadow {
            mesh: "mesh".to_owned(),
            value: 2,
        })
    );
}

#[test]
fn preserves_disabled_cast_shadow_status() -> Result<(), String> {
    let root = temp_root("disabled-cast-shadow");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","render_status":0,"#,
        r#""prim_groups":[{"shader":"shader","#,
        r#""positions":[[0,0,0],[1,0,0],[0,1,0]],"indices":[0,1,2]}]}"#,
    );
    fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("mesh.json"), mesh_json))
        .map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let mesh = source
        .load_mesh("mesh")
        .map_err(|error| format!("CastShadow decode failed: {error:?}"))?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;

    if mesh.cast_shadow == Some(false) {
        Ok(())
    } else {
        Err(format!("CastShadow source flag changed: {mesh:?}"))
    }
}

#[test]
fn rejects_unsupported_decoded_uv_channels() {
    let root = temp_root("unsupported-uv-channel");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2],"uvs":[{"channel":1,"coords":["#,
        r#"[0,0],[1,0],[0,1]]}]}]}"#,
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
fn rejects_declared_uv_channels_without_coordinates() {
    let root = temp_root("empty-uv-channel");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2],"uvs":[{"channel":0,"coords":[]}]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("mesh.json"), mesh_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::EmptyUvChannel { group: 0, channel: 0 })
    );
}

#[test]
fn rejects_mesh_identity_mismatches() {
    let root = temp_root("identity-mismatch");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"decoded","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("requested.json"), mesh_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.load_mesh("requested");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::MeshIdentityMismatch {
            requested: "requested".to_owned(),
            decoded: "decoded".to_owned(),
        })
    );
}

#[test]
fn analysis_loader_rejects_repeated_index_triangle_lists() -> Result<(), String>
{
    let root = temp_root("analysis-degenerate-triangle");
    let mesh_dir = root.join("components").join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0],"#,
        r#"[1,1,0]],"indices":[0,1,2,2,2,3]}]}"#,
    );
    fs::create_dir_all(&mesh_dir)
        .and_then(|()| fs::write(mesh_dir.join("mesh.json"), mesh_json))
        .map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    if source.load_mesh("mesh").is_ok() {
        return Err("strict mesh loading accepted repeated indices".to_owned());
    }
    let analysis_result = read_mesh_for_analysis(&root, "mesh");
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if analysis_result.is_ok() {
        return Err(
            "analysis mesh altered repeated-index source topology".to_owned()
        );
    }
    Ok(())
}
