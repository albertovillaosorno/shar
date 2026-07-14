// File:
//   - decoded_material_source.rs
// Path:
//   - src/fbx/tests/decoded_material_source.rs
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
//   - Regression coverage for decoded FBX material resolution boundaries.
// - Must-Not:
//   - Read private assets, discover packages, or use fixed machine paths.
// - Allows:
//   - Synthetic shader JSON and process-unique temporary directories.
// - Split-When:
//   - Shader translation introduces independently testable channel behavior.
// - Merge-When:
//   - Material regressions move into shared adapter conformance tests.
// - Summary:
//   - Protects shader identity and material evidence before scene building.
// - Description:
//   - Exercises decoded shader resolution with synthetic evidence.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Temporary directories are process-unique and removed by each regression.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: Synthetic shader identity, schema, version, parameter, and flag
//   - regressions share one decoded material evidence boundary.
//

//! Regression coverage for decoded FBX material resolution boundaries.
//!
//! Synthetic shader evidence verifies material identity without private assets
//! or machine-local dependency routes.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource,
};
use fbx::ports::component_source::ComponentSource;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;

fn temp_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-decoded-material-{label}-{}",
            std::process::id()
        ),
    )
}

#[test]
fn accepts_utf8_bom_in_decoded_json() {
    let root = temp_root("utf8-bom");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("shader.json"),
                concat!(
                    "\u{feff}",
                    r#"{"name":"shader","params":[]}"#
                ),
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
        Ok(
            fbx::domain::texture::MaterialBinding {
                material_name: "shader".to_owned(),
                texture_file_name: None,
            }
        )
    );
}

#[test]
fn rejects_shader_identity_mismatches() {
    let root = temp_root("identity-mismatch");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("requested.json"),
                r#"{"name":"decoded","params":[]}"#,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.resolve_material("requested");
    let _cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Err(
            DecodedComponentError::ShaderIdentityMismatch {
                requested: "requested".to_owned(),
                decoded: "decoded".to_owned(),
            }
        )
    );
}

#[test]
fn accepts_trailing_nul_padding_sanitized_in_shader_member_path() {
    let root = temp_root("nul-padded-identity");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("char_swatches_lit_m_.json"),
                r#"{"name":"char_swatches_lit_m\u0000","params":[]}"#,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );
    let result = source.resolve_material("char_swatches_lit_m_");
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(
            fbx::domain::texture::MaterialBinding {
                material_name: "char_swatches_lit_m".to_owned(),
                texture_file_name: None,
            }
        )
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn accepts_trailing_nul_padding_in_numbered_texture_reference() {
    let root = temp_root("nul-padded-numbered-texture");
    let shader_dir = root
        .join("package")
        .join("components")
        .join("shader");
    let shared_dir = root.join("shared");
    let output_dir = root.join("output");
    let external_texture = shared_dir.join("shared.bmp.0.png");
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&shared_dir))
        .and_then(
            |()| {
                fs::write(
                    shader_dir.join("skin.json"),
                    concat!(
                        r#"{"name":"skin","params":[{"kind":"texture","#,
                        r#""param":"TEX","value":"shared.bmp.0\u0000\u0000"}]}"#
                    ),
                )
            },
        )
        .and_then(
            |()| {
                fs::write(
                    &external_texture,
                    b"synthetic-png",
                )
            },
        );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        root.join("package"),
        &output_dir,
    );
    let result = source.resolve_material_with_external_texture(
        "skin",
        &external_texture,
    );
    let cleanup_result = fs::remove_dir_all(&root);

    assert_eq!(
        result,
        Ok(
            fbx::domain::texture::MaterialBinding {
                material_name: "skin".to_owned(),
                texture_file_name: Some("shared.bmp.0.png".to_owned()),
            }
        )
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn stages_exact_index_published_external_texture() {
    let root = temp_root("external-texture");
    let shader_dir = root
        .join("package")
        .join("components")
        .join("shader");
    let shared_dir = root.join("shared");
    let output_dir = root.join("output");
    let external_texture = shared_dir.join("shared.png");
    let setup_result = fs::create_dir_all(&shader_dir)
        .and_then(|()| fs::create_dir_all(&shared_dir))
        .and_then(
            |()| {
                fs::write(
                    shader_dir.join("skin.json"),
                    concat!(
                        r#"{"name":"skin","params":[{"kind":"texture","#,
                        r#""param":"TEX","value":"shared.bmp"}]}"#
                    ),
                )
            },
        )
        .and_then(
            |()| {
                fs::write(
                    &external_texture,
                    b"synthetic-png",
                )
            },
        );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        root.join("package"),
        &output_dir,
    );

    let result = source.resolve_material_with_external_texture(
        "skin",
        &external_texture,
    );
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
        Ok(
            fbx::domain::texture::MaterialBinding {
                material_name: "skin".to_owned(),
                texture_file_name: Some("shared.png".to_owned()),
            }
        )
    );
    assert_eq!(
        staged,
        b"synthetic-png".to_vec()
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_shader_parameter_count_mismatch() {
    let root = temp_root("parameter-count");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("shader.json"),
                r#"{"name":"shader","num_params":1,"params":[]}"#,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(
            DecodedComponentError::ShaderParameterCountMismatch {
                shader: "shader".to_owned(),
                declared: 1,
                actual: 0,
            }
        )
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_explicit_shader_schema_mismatch() {
    let root = temp_root("schema-mismatch");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("shader.json"),
                r#"{"schema":"texture","name":"shader","params":[]}"#,
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(
            DecodedComponentError::ShaderSchemaMismatch {
                shader: "shader".to_owned(),
                schema: "texture".to_owned(),
            }
        )
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_unsupported_shader_version() {
    let root = temp_root("version-mismatch");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("shader.json"),
                concat!(
                    r#"{"schema":"shader","name":"shader","version":1,"#,
                    r#""params":[]}"#,
                ),
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(
            DecodedComponentError::UnsupportedShaderVersion {
                shader: "shader".to_owned(),
                version: 1,
            }
        )
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_blank_platform_shader_name() {
    let root = temp_root("blank-platform-shader");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("shader.json"),
                concat!(
                    r#"{"schema":"shader","name":"shader","version":0,"#,
                    r#""pddi_shader_name":"","params":[]}"#,
                ),
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(
            DecodedComponentError::BlankPlatformShaderName {
                shader: "shader".to_owned(),
            }
        )
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_invalid_shader_translucency_flag() {
    let root = temp_root("invalid-translucency");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("shader.json"),
                concat!(
                    r#"{"schema":"shader","name":"shader","version":0,"#,
                    r#""pddi_shader_name":"simple","has_translucency":2,"#,
                    r#""params":[]}"#,
                ),
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert_eq!(
        result,
        Err(
            DecodedComponentError::InvalidShaderTranslucency {
                shader: "shader".to_owned(),
                value: 2,
            }
        )
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_non_numeric_shader_vertex_needs() {
    let root = temp_root("non-numeric-vertex-needs");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("shader.json"),
                concat!(
                    r#"{"schema":"shader","name":"shader","version":0,"#,
                    r#""vertex_needs":"33","params":[]}"#,
                ),
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert!(
        matches!(
            result,
            Err(DecodedComponentError::Parse { .. })
        )
    );
    assert!(cleanup_result.is_ok());
}

#[test]
fn rejects_non_numeric_shader_vertex_mask() {
    let root = temp_root("non-numeric-vertex-mask");
    let shader_dir = root
        .join("components")
        .join("shader");
    let setup_result = fs::create_dir_all(&shader_dir).and_then(
        |()| {
            fs::write(
                shader_dir.join("shader.json"),
                concat!(
                    r#"{"schema":"shader","name":"shader","version":0,"#,
                    r#""vertex_mask":"4294721505","params":[]}"#,
                ),
            )
        },
    );
    assert!(setup_result.is_ok());
    let source = DecodedComponentSource::new(
        &root,
        root.join("textures"),
    );

    let result = source.resolve_material("shader");

    let cleanup_result = fs::remove_dir_all(&root);
    assert!(
        matches!(
            result,
            Err(DecodedComponentError::Parse { .. })
        )
    );
    assert!(cleanup_result.is_ok());
}
