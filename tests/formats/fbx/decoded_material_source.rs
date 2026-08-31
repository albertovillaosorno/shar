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
//   - Decoded material source test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Decoded material source test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Decoded material source test module.

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
        "fbx-decoded-material-{label}-{}",
        std::process::id()
    ))
}

#[test]
fn accepts_utf8_bom_in_decoded_json() {
    let root = temp_root("utf8-bom");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("shader.json"),
            concat!("\u{feff}", r#"{"name":"shader","params":[]}"#),
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("shader");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "shader".to_owned(),
            texture_file_name: None,
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
}

#[test]
fn empty_texture_parameter_is_an_untextured_material() {
    let root = temp_root("empty-texture-reference");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("lambert1.json"),
            concat!(
                r#"{"name":"lambert1","params":[{"kind":"texture","#,
                r#""param":"TEX","value":""}]}"#
            ),
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("lambert1");
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "lambert1".to_owned(),
            texture_file_name: None,
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_shader_identity_mismatches() {
    let root = temp_root("identity-mismatch");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("requested.json"),
            r#"{"name":"decoded","params":[]}"#,
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("requested");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::ShaderIdentityMismatch {
            requested: "requested".to_owned(),
            decoded: "decoded".to_owned(),
        })
    );
}

#[test]
fn accepts_trailing_nul_padding_sanitized_in_shader_member_path() {
    let root = temp_root("nul-padded-identity");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("char_swatches_lit_m_.json"),
            r#"{"name":"char_swatches_lit_m\u0000","params":[]}"#,
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("char_swatches_lit_m_");
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "char_swatches_lit_m".to_owned(),
            texture_file_name: None,
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn accepts_trailing_nul_padding_in_numbered_texture_reference() {
    let root = temp_root("nul-padded-numbered-texture");
    let shader_dir = root.join("package").join("components").join("shader");
    let shared_dir = root.join("shared");
    let output_dir = root.join("output");
    let external_texture = shared_dir.join("shared.bmp.0.png");
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&shared_dir))
        .and_then(|()| {
            fs::write(
                shader_dir.join("skin.json"),
                concat!(
                    r#"{"name":"skin","params":[{"kind":"texture","#,
                    r#""param":"TEX","value":"shared.bmp.0\u0000\u0000"}]}"#
                ),
            )
        })
        .and_then(|()| fs::write(&external_texture, b"synthetic-png"));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(root.join("package"), &output_dir);
    let result = source
        .resolve_material_with_external_texture("skin", &external_texture);
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "skin".to_owned(),
            texture_file_name: Some("shared.bmp.0.png".to_owned()),
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_ambiguous_shader_ledger_even_when_direct_member_exists() {
    let root = temp_root("ambiguous-ledger-direct-shader");
    let package = root.join("package");
    let shader_dir = package.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| {
            fs::write(
                shader_dir.join("shared.json"),
                r#"{"schema":"shader","name":"shared","params":[]}"#,
            )
        })
        .and_then(|()| {
            fs::write(
                shader_dir.join("shared__ordinal_2.json"),
                concat!(
                    r#"{"schema":"shader","name":"shared","params":[],"#,
                    r#""has_translucency":1}"#,
                ),
            )
        })
        .and_then(|()| {
            fs::write(
                package.join("components.jsonl"),
                concat!(
                    r#"{"schema":"p3d.package.v1"}"#,
                    "\n",
                    r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"#,
                    r#""container_ordinal":1,"name":"shared","#,
                    r#""path":"shader/shared.json","kind":"shader"}"#,
                    "\n",
                    r#"{"ordinal":2,"depth":1,"parent_ordinal":0,"#,
                    r#""container_ordinal":2,"name":"shared","#,
                    r#""path":"shader/shared__ordinal_2.json","#,
                    r#""kind":"shader"}"#,
                    "\n",
                ),
            )
        });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&package, root.join("textures"));
    let result = source.resolve_material("shared");
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::AmbiguousShaderMember {
            shader: "shared".to_owned(),
            candidates: vec![
                "shared.json".to_owned(),
                "shared__ordinal_2.json".to_owned(),
            ],
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn resolves_sanitized_local_texture_through_package_ledger() {
    let root = temp_root("ledger-texture-identity");
    let package = root.join("package");
    let shader_dir = package.join("components").join("shader");
    let texture_dir = package.join("components").join("texture");
    let output_dir = root.join("output");
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| {
            fs::write(
                shader_dir.join("glass.json"),
                concat!(
                    r#"{"name":"glass","params":[{"kind":"texture","#,
                    r#""param":"TEX","value":"Krusty_ HumanCola.bmp"}]}"#
                ),
            )
        })
        .and_then(|()| {
            fs::write(
                texture_dir.join("Krusty__HumanCola.png"),
                b"synthetic-png",
            )
        })
        .and_then(|()| {
            fs::write(
                package.join("components.jsonl"),
                concat!(
                    r#"{"schema":"p3d.package.v1"}"#,
                    "\n",
                    r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"#,
                    r#""container_ordinal":1,"#,
                    r#""name":"Krusty_ HumanCola.bmp","#,
                    r#""path":"texture/Krusty__HumanCola.png","#,
                    r#""kind":"texture","payload_format":"image/png","#,
                    r#""schema_ref":"texture","#,
                    r#""recovery_status":"#,
                    r#""recovered_embedded_image_payload"}"#,
                    "\n"
                ),
            )
        });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&package, &output_dir);
    let result = source.resolve_material("glass");
    let staged = fs::read(output_dir.join("Krusty__HumanCola.png"));
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "glass".to_owned(),
            texture_file_name: Some("Krusty__HumanCola.png".to_owned()),
            semantics: fbx::domain::texture::MaterialSemantics::default()
                .with_glass(true),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
    assert!(
        staged.is_ok(),
        "staged ledger texture should be readable: {staged:?}"
    );
    assert_eq!(staged.ok(), Some(b"synthetic-png".to_vec()));
    assert!(cleanup_result.is_ok());
}

#[test]
fn resolves_case_only_texture_ledger_identity() {
    let root = temp_root("ledger-texture-case-identity");
    let package = root.join("package");
    let shader_dir = package.join("components").join("shader");
    let texture_dir = package.join("components").join("texture");
    let output_dir = root.join("output");
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| {
            fs::write(
                shader_dir.join("tree.json"),
                concat!(
                    r#"{"name":"tree","params":[{"kind":"texture","#,
                    r#""param":"TEX","value":"tree.bmp"}]}"#,
                ),
            )
        })
        .and_then(|()| {
            fs::write(texture_dir.join("tree.BMP.png"), b"synthetic-png")
        })
        .and_then(|()| {
            fs::write(
                package.join("components.jsonl"),
                concat!(
                    r#"{"schema":"p3d.package.v1"}"#,
                    "\n",
                    r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"#,
                    r#""container_ordinal":1,"name":"tree.BMP","#,
                    r#""path":"texture/tree.BMP.png","#,
                    r#""kind":"texture"}"#,
                    "\n",
                ),
            )
        });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&package, &output_dir);
    let result = source.resolve_material("tree");
    let staged = fs::read(output_dir.join("tree.BMP.png"));
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "tree".to_owned(),
            texture_file_name: Some("tree.BMP.png".to_owned()),
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
    assert!(staged.is_ok_and(|bytes| bytes == b"synthetic-png"));
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_duplicate_texture_ledger_relationships() {
    let root = temp_root("duplicate-ledger-texture-identity");
    let package = root.join("package");
    let shader_dir = package.join("components").join("shader");
    let texture_dir = package.join("components").join("texture");
    let output_dir = root.join("output");
    let texture_row = concat!(
        r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"#,
        r#""container_ordinal":1,"name":"shared.bmp","#,
        r#""path":"texture/shared_.png","kind":"texture","#,
        r#""payload_format":"image/png","schema_ref":"texture","#,
        r#""recovery_status":"recovered_embedded_image_payload"}"#,
    );
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| {
            fs::write(
                shader_dir.join("glass.json"),
                concat!(
                    r#"{"name":"glass","params":[{"kind":"texture","#,
                    r#""param":"TEX","value":"shared.bmp"}]}"#
                ),
            )
        })
        .and_then(|()| {
            fs::write(texture_dir.join("shared_.png"), b"synthetic-png")
        })
        .and_then(|()| {
            fs::write(
                package.join("components.jsonl"),
                format!(
                    "{}\n{}\n{}\n",
                    r#"{"schema":"p3d.package.v1"}"#, texture_row, texture_row,
                ),
            )
        });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&package, &output_dir);
    let result = source.resolve_material("glass");
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::AmbiguousTextureMember {
            texture: "shared.bmp".to_owned(),
            candidates: vec![
                "shared_.png".to_owned(),
                "shared_.png".to_owned(),
            ],
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_ambiguous_texture_ledger_even_when_direct_member_exists() {
    let root = temp_root("ambiguous-ledger-direct-texture");
    let package = root.join("package");
    let shader_dir = package.join("components").join("shader");
    let texture_dir = package.join("components").join("texture");
    let output_dir = root.join("output");
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| {
            fs::write(
                shader_dir.join("glass.json"),
                concat!(
                    r#"{"name":"glass","params":[{"kind":"texture","#,
                    r#""param":"TEX","value":"shared.bmp"}]}"#
                ),
            )
        })
        .and_then(|()| {
            fs::write(texture_dir.join("shared.png"), b"first-payload")
        })
        .and_then(|()| {
            fs::write(
                texture_dir.join("shared__ordinal_2.png"),
                b"second-payload",
            )
        })
        .and_then(|()| {
            fs::write(
                package.join("components.jsonl"),
                concat!(
                    r#"{"schema":"p3d.package.v1"}"#,
                    "\n",
                    r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"#,
                    r#""container_ordinal":1,"name":"shared.bmp","#,
                    r#""path":"texture/shared.png","kind":"texture"}"#,
                    "\n",
                    r#"{"ordinal":2,"depth":1,"parent_ordinal":0,"#,
                    r#""container_ordinal":2,"name":"shared.bmp","#,
                    r#""path":"texture/shared__ordinal_2.png","#,
                    r#""kind":"texture"}"#,
                    "\n",
                ),
            )
        });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&package, &output_dir);
    let result = source.resolve_material("glass");
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::AmbiguousTextureMember {
            texture: "shared.bmp".to_owned(),
            candidates: vec![
                "shared.png".to_owned(),
                "shared__ordinal_2.png".to_owned(),
            ],
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_space_padded_texture_ledger_identity() {
    let root = temp_root("ledger-texture-space-padding");
    let package = root.join("package");
    let shader_dir = package.join("components").join("shader");
    let texture_dir = package.join("components").join("texture");
    let output_dir = root.join("output");
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&texture_dir))
        .and_then(|()| {
            fs::write(
                shader_dir.join("glass.json"),
                concat!(
                    r#"{"name":"glass","params":[{"kind":"texture","#,
                    r#""param":"TEX","value":"shared.bmp"}]}"#
                ),
            )
        })
        .and_then(|()| {
            fs::write(texture_dir.join("shared_.png"), b"synthetic-png")
        })
        .and_then(|()| {
            fs::write(
                package.join("components.jsonl"),
                concat!(
                    r#"{"schema":"p3d.package.v1"}"#,
                    "\n",
                    r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"#,
                    r#""container_ordinal":1,"name":" shared.bmp","#,
                    r#""path":"texture/shared_.png","kind":"texture","#,
                    r#""payload_format":"image/png","schema_ref":"texture","#,
                    r#""recovery_status":"recovered_embedded_image_payload"}"#,
                    "\n"
                ),
            )
        });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&package, &output_dir);
    let result = source.resolve_material("glass");
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidTextureReference(
            " shared.bmp".to_owned()
        ))
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn stages_exact_index_published_external_texture() {
    let root = temp_root("external-texture");
    let shader_dir = root.join("package").join("components").join("shader");
    let shared_dir = root.join("shared");
    let output_dir = root.join("output");
    let external_texture = shared_dir.join("shared.png");
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&shared_dir))
        .and_then(|()| {
            fs::write(
                shader_dir.join("skin.json"),
                concat!(
                    r#"{"name":"skin","params":[{"kind":"texture","#,
                    r#""param":"TEX","value":"shared.bmp"}]}"#
                ),
            )
        })
        .and_then(|()| fs::write(&external_texture, b"synthetic-png"));
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(root.join("package"), &output_dir);

    let result = source
        .resolve_material_with_external_texture("skin", &external_texture);
    let staged_result = fs::read(output_dir.join("shared.png"));
    assert!(
        staged_result.is_ok(),
        "staged external texture should be readable: {staged_result:?}"
    );
    let Some(staged) = staged_result.ok() else {
        let _cleanup_result = fs::remove_dir_all(&root);
        return;
    };
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "skin".to_owned(),
            texture_file_name: Some("shared.png".to_owned()),
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [u8::MAX; 4],
        })
    );
    assert_eq!(staged, b"synthetic-png".to_vec());
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_shader_parameter_count_mismatch() {
    let root = temp_root("parameter-count");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("shader.json"),
            r#"{"name":"shader","num_params":1,"params":[]}"#,
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(DecodedComponentError::ShaderParameterCountMismatch {
            shader: "shader".to_owned(),
            declared: 1,
            actual: 0,
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_explicit_shader_schema_mismatch() {
    let root = temp_root("schema-mismatch");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("shader.json"),
            r#"{"schema":"texture","name":"shader","params":[]}"#,
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(DecodedComponentError::ShaderSchemaMismatch {
            shader: "shader".to_owned(),
            schema: "texture".to_owned(),
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_unsupported_shader_version() {
    let root = temp_root("version-mismatch");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("shader.json"),
            concat!(
                r#"{"schema":"shader","name":"shader","version":1,"#,
                r#""params":[]}"#,
            ),
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(DecodedComponentError::UnsupportedShaderVersion {
            shader: "shader".to_owned(),
            version: 1,
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_blank_platform_shader_name() {
    let root = temp_root("blank-platform-shader");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("shader.json"),
            concat!(
                r#"{"schema":"shader","name":"shader","version":0,"#,
                r#""pddi_shader_name":"","params":[]}"#,
            ),
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(DecodedComponentError::BlankPlatformShaderName {
            shader: "shader".to_owned(),
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_invalid_shader_translucency_flag() {
    let root = temp_root("invalid-translucency");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("shader.json"),
            concat!(
                r#"{"schema":"shader","name":"shader","version":0,"#,
                r#""pddi_shader_name":"simple","has_translucency":2,"#,
                r#""params":[]}"#,
            ),
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(DecodedComponentError::InvalidShaderTranslucency {
            shader: "shader".to_owned(),
            value: 2,
        })
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_non_numeric_shader_vertex_needs() {
    let root = temp_root("non-numeric-vertex-needs");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("shader.json"),
            concat!(
                r#"{"schema":"shader","name":"shader","version":0,"#,
                r#""vertex_needs":"33","params":[]}"#,
            ),
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert!(matches!(result, Err(DecodedComponentError::Parse { .. })));
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_non_numeric_shader_vertex_mask() {
    let root = temp_root("non-numeric-vertex-mask");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("shader.json"),
            concat!(
                r#"{"schema":"shader","name":"shader","version":0,"#,
                r#""vertex_mask":"4294721505","params":[]}"#,
            ),
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert!(matches!(result, Err(DecodedComponentError::Parse { .. })));
    assert!(cleanup_result.is_ok());
}

#[test]
fn preserves_zero_alpha_from_diffuse_colour_parameter() {
    let root = temp_root("diffuse-zero-alpha");
    let shader_dir = root.join("components").join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(|()| {
        fs::write(
            shader_dir.join("zero_alpha.json"),
                        // jig-ignore-next-line: literal
            r#"{"name":"zero_alpha","params":[{"kind":"colour","param":"DIFF","value":1122867}]}"#,
        )
    });
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(&root, root.join("textures"));
    let result = source.resolve_material("zero_alpha");
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(fbx::domain::texture::MaterialBinding {
            material_name: "zero_alpha".to_owned(),
            texture_file_name: None,
            semantics: fbx::domain::texture::MaterialSemantics::default(),
            base_color_rgba8: [0x11, 0x22, 0x33, 0x00],
        })
    );
    assert!(cleanup_result.is_ok());
}
