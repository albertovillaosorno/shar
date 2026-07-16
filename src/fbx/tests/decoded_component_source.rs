// File:
//   - decoded_component_source.rs
// Path:
//   - src/fbx/tests/decoded_component_source.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Regression coverage for decoded FBX component adapter boundaries.
// - Must-Not:
//   - Read private assets, discover packages, or use machine-local fixed paths.
// - Allows:
//   - Synthetic decoded JSON and process-unique temporary directories.
// - Split-When:
//   - Texture conversion requires an independent external-process boundary.
// - Merge-When:
//   - Decoded component regressions move into shared adapter conformance tests.
// - Summary:
//   - Protects decoded mesh and material loading before application planning.
// - Description:
//   - Exercises the adapter with deterministic synthetic component data.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Temporary directories are process-unique and removed by each regression.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Regression coverage for decoded FBX component adapter boundaries.
//!
//! Synthetic JSON verifies path and schema contracts without private game data
//! or machine-local dependency routes.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource,
};
use fbx::ports::component_source::ComponentSource;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn temp_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-decoded-{label}-{}",
            std::process::id()
        ),
    )
}

const fn valid_mesh_json() -> &'static str {
    concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    )
}

#[test]
fn rejects_duplicate_texture_parameters() {
    let root = temp_root("duplicate-texture-parameter");
    let shader_dir = root
        .join("components")
        .join("shader");
    let texture_dir = root
        .join("components")
        .join("texture");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","value":"a.bmp"},{"#,
        r#""kind":"texture","param":"TEX","value":"b.bmp"}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(
            |()| {
                fs::write(
                    shader_dir.join("shader.json"),
                    shader_json,
                )
            },
        )
        .and_then(
            |()| {
                fs::write(
                    texture_dir.join("a.png"),
                    b"a",
                )
            },
        )
        .and_then(
            |()| {
                fs::write(
                    texture_dir.join("b.png"),
                    b"b",
                )
            },
        );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.resolve_material("shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(
            DecodedComponentError::DuplicateTextureParameter {
                shader: "shader".to_owned(),
            }
        )
    );
}

#[test]
fn rejects_non_string_texture_parameters() {
    let root = temp_root("non-string-texture");
    let shader_dir = root
        .join("components")
        .join("shader");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","value":123}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("shader.json"),
                shader_json,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.resolve_material("shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(
            DecodedComponentError::InvalidTextureParameter {
                shader: "shader".to_owned(),
            }
        )
    );
}

#[test]
fn rejects_unsupported_decoded_uv_channels() {
    let root = temp_root("unsupported-uv-channel");
    let mesh_dir = root
        .join("components")
        .join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2],"uvs":[{"#,
        r#""channel":1,"coords":[[0,0],[1,0],[0,1]]}]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir).and_then(
        |()| {
            fs::write(
                mesh_dir.join("mesh.json"),
                mesh_json,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(
            DecodedComponentError::UnsupportedUvChannel {
                group: 0,
                channel: 1,
            }
        )
    );
}

#[test]
fn rejects_duplicate_decoded_uv_channels() {
    let root = temp_root("duplicate-uv-channel");
    let mesh_dir = root
        .join("components")
        .join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","prim_groups":[{"#,
        r#""shader":"shader","positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2],"uvs":[{"#,
        r#""channel":0,"coords":[[0,0],[1,0],[0,1]]},{"#,
        r#""channel":0,"coords":[[0,0],[1,0],[0,1]]}]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir).and_then(
        |()| {
            fs::write(
                mesh_dir.join("mesh.json"),
                mesh_json,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(
            DecodedComponentError::DuplicateUvChannel {
                group: 0,
                channel: 0,
            }
        )
    );
}

#[test]
fn rejects_unknown_decoded_json_fields() {
    let root = temp_root("unknown-fields");
    let mesh_dir = root
        .join("components")
        .join("mesh");
    let mesh_json = concat!(
        r#"{"schema":"mesh","name":"mesh","extra":1,"#,
        r#""prim_groups":[{"shader":"shader","#,
        r#""positions":[[0,0,0],[1,0,0],[0,1,0]],"#,
        r#""indices":[0,1,2]}]}"#,
    );
    let setup_result = fs::create_dir_all(&mesh_dir).and_then(
        |()| {
            fs::write(
                mesh_dir.join("mesh.json"),
                mesh_json,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.load_mesh("mesh");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert!(
        matches!(
            result,
            Err(DecodedComponentError::Parse { .. })
        )
    );
}

#[test]
fn rejects_component_ids_that_escape_the_package_root() {
    let root = temp_root("path-traversal");
    let components = root.join("components");
    let setup_result = fs::create_dir_all(&components)
        .and_then(
            |()| {
                fs::write(
                    components.join("escape-mesh.json"),
                    valid_mesh_json(),
                )
            },
        )
        .and_then(
            |()| {
                fs::write(
                    components.join("escape-shader.json"),
                    r#"{"name":"shader","params":[]}"#,
                )
            },
        );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let mesh_result = source.load_mesh("../escape-mesh");
    let material_result = source.resolve_material("../escape-shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        mesh_result,
        Err(
            DecodedComponentError::InvalidMemberId("../escape-mesh".to_owned())
        )
    );
    assert_eq!(
        material_result,
        Err(
            DecodedComponentError::InvalidMemberId(
                "../escape-shader".to_owned()
            )
        )
    );
}
