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
//   - Decoded texture source test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded texture source test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded texture source test module.

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
    std::env::temp_dir().join(format!(
        "fbx-decoded-texture-{label}-{}",
        std::process::id()
    ))
}

#[test]
fn resolves_already_normalized_png_texture_extensions() {
    let root = temp_root("normalized-png");
    let shader_dir = root.join("components").join("shader");
    let texture_dir = root.join("components").join("texture");
    let output_dir = root.join("staged-textures");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","#,
        r#""value":"ready.png"}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| fs::write(shader_dir.join("shader.json"), shader_json))
        .and_then(|()| fs::write(texture_dir.join("ready.png"), b"png"));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, &output_dir);
    let result = source.resolve_material("shader");
    let staged = fs::read(output_dir.join("ready.png"));
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "shader".to_owned(),
            texture_file_name: Some("ready.png".to_owned()),
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
    assert!(staged.is_ok_and(|bytes| bytes == b"png"));
}

#[test]
fn rejects_decoded_shader_identity_with_surrounding_whitespace() {
    let root = temp_root("shader-name-whitespace");
    let shader_dir = root.join("components").join("shader");
    let shader_json = r#"{"name":" shader","params":[]}"#;
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::write(shader_dir.join("shader.json"), shader_json));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::ShaderIdentityMismatch {
            requested: "shader".to_owned(),
            decoded: " shader".to_owned(),
        })
    );
}

#[test]
fn rejects_texture_references_with_surrounding_whitespace() {
    let root = temp_root("texture-reference-whitespace");
    let shader_dir = root.join("components").join("shader");
    let texture_dir = root.join("components").join("texture");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","#,
        r#""value":" ready.bmp "}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| fs::write(shader_dir.join("shader.json"), shader_json))
        .and_then(|()| fs::write(texture_dir.join("ready.png"), b"png"));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidTextureReference(
            " ready.bmp ".to_owned()
        ))
    );
}

#[test]
fn rejects_texture_references_without_a_file_stem() {
    let root = temp_root("missing-texture-stem");
    let shader_dir = root.join("components").join("shader");
    let texture_dir = root.join("components").join("texture");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","#,
        r#""value":".bmp"}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| fs::write(shader_dir.join("shader.json"), shader_json))
        .and_then(|()| fs::write(texture_dir.join(".png"), b"png"));
    assert!(setup_result.is_ok());
    let source =
        DecodedComponentSource::new(&root, root.join("staged-textures"));
    let result = source.resolve_material("shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidTextureReference(
            ".bmp".to_owned()
        ))
    );
}

#[test]
fn accepts_textures_already_in_the_staging_directory() {
    let root = temp_root("already-staged");
    let shader_dir = root.join("components").join("shader");
    let texture_dir = root.join("components").join("texture");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","#,
        r#""value":"ready.bmp"}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| fs::write(shader_dir.join("shader.json"), shader_json))
        .and_then(|()| fs::write(texture_dir.join("ready.png"), b"png"));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, &texture_dir);
    let result = source.resolve_material("shader");
    let retained = fs::read(texture_dir.join("ready.png"));
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "shader".to_owned(),
            texture_file_name: Some("ready.png".to_owned()),
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
    assert!(retained.is_ok_and(|bytes| bytes == b"png"));
}

#[test]
fn resolves_mixed_case_bmp_texture_extensions() {
    let root = temp_root("mixed-case-bmp");
    let shader_dir = root.join("components").join("shader");
    let texture_dir = root.join("components").join("texture");
    let output_dir = root.join("staged-textures");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","#,
        r#""value":"mixed.BmP"}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| fs::write(shader_dir.join("shader.json"), shader_json))
        .and_then(|()| fs::write(texture_dir.join("mixed.png"), b"png"));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, &output_dir);
    let result = source.resolve_material("shader");
    let staged = fs::read(output_dir.join("mixed.png"));
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "shader".to_owned(),
            texture_file_name: Some("mixed.png".to_owned()),
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
    assert!(staged.is_ok_and(|bytes| bytes == b"png"));
}

#[test]
fn rejects_texture_references_that_escape_the_texture_directory() {
    let root = temp_root("texture-path-traversal");
    let shader_dir = root.join("components").join("shader");
    let escaped_texture = root.join("components").join("escape.png");
    let shader_json = concat!(
        r#"{"name":"shader","params":[{"#,
        r#""kind":"texture","param":"TEX","#,
        r#""value":"../escape.bmp"}]}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::write(shader_dir.join("shader.json"), shader_json))
        .and_then(|()| fs::write(&escaped_texture, b"outside"));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidTextureReference(
            "../escape.bmp".to_owned()
        ))
    );
}
