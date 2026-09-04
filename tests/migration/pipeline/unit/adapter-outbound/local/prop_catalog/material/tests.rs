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

use std::collections::{BTreeMap, BTreeSet};
use std::fs;

use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, ShaderMemberOccurrence,
};
use fbx::domain::mesh::PrimitiveGroup;
use fbx::domain::texture::MaterialSemantics;

use crate::domain::package::PhaseThreePackageRow;

use super::{
    DecodedComponentSource, ShaderConsumerProvenance, SharedTextureAuthority,
    WorldMeshSourceCoordinate, canonical_material_identity,
    material_resolution_error, model_package_member_id, prepare_source_texture,
    resolve_materials, resolve_source_material, shader_consumer_provenance,
};

fn phase_three_shader_package() -> Result<PhaseThreePackageRow, String> {
    PhaseThreePackageRow::from_json_line(concat!(
        r#"{"package_id":"extracted-art-l1-terra","#,
        r#""package_root":"extracted/art/L1_TERRA","#,
        r#""package_category":"terrain-world","#,
        r#""package_subcategory":"terrain-world/level-01/terrain-mesh","#,
        r#""unit_count":3,"text_key_count":0,"#,
        r#""unit_ids":["material-three","material-nine","model-forty"],"#,
        r#""world_ids":[],"#,
        r#""texture_ids":[],"#,
        r#""material_ids":["material-three","material-nine"],"#,
        r#""model_ids":["model-forty"],"physics_ids":[],"animation_ids":[],"#,
        r#""scene_ids":[],"locator_ids":[],"camera_ids":[],"#,
        r#""light_ids":[],"particle_ids":[],"controller_ids":[],"#,
        r#""audio_ids":[],"movie_ids":[],"script_ids":[],"#,
        r#""text_ids":[],"ui_ids":[],"metadata_ids":[],"error_ids":[],"#,
        r#""source_unit_ids":[],"text_key_ids":[],"members":[{"#,
        r#""id":"material-three","role":"material","#,
        r#""path":"extracted/art/L1_TERRA/components/shader/first.json","#,
        r#""type":"material","kind":"p3d-shader","#,
        r#""source_chunk_kind":"shader","source_chunk_ordinal":"3"},{"#,
        r#""id":"material-nine","role":"material","#,
        r#""path":"extracted/art/L1_TERRA/components/shader/second.json","#,
        r#""type":"material","kind":"p3d-shader","#,
        r#""source_chunk_kind":"shader","source_chunk_ordinal":"9"},{"#,
        r#""id":"model-forty","role":"model","#,
        r#""path":"extracted/art/L1_TERRA/components/mesh/body.json","#,
        r#""type":"model","kind":"p3d-mesh","#,
        r#""source_chunk_kind":"mesh","source_chunk_ordinal":"40"}],"#,
        r#""text_keys":[]}"#
    ))
    .map_err(|error| error.to_string())
}

#[test]
fn ambiguous_shader_error_retains_exact_consumer_ordinals()
-> Result<(), String> {
    let left = PrimitiveGroup::new(
        0,
        "shared",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map(|group| group.with_source_ordinal(42))
    .map_err(|error| format!("left primitive group failed: {error:?}"))?;
    let right = PrimitiveGroup::new(
        1,
        "shared",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map(|group| group.with_source_ordinal(7))
    .map_err(|error| format!("right primitive group failed: {error:?}"))?;
    let meshes = [fbx::domain::mesh::MeshAsset::new("body", vec![left, right])
        .map_err(|error| format!("fixture mesh failed: {error:?}"))?];
    let sources = shader_consumer_provenance(&meshes, None, None);
    let expected = BTreeSet::from([7_usize, 42]);
    if sources.get("shared").map(|source| &source.source_ordinals)
        != Some(&expected)
    {
        return Err(format!("shader source coordinates changed: {sources:?}"));
    }
    let error = DecodedComponentError::AmbiguousShaderMember {
        shader: "shared".to_owned(),
        occurrences: vec![
            ShaderMemberOccurrence {
                member: "first.json".to_owned(),
                source_ordinal: Some(3),
            },
            ShaderMemberOccurrence {
                member: "second.json".to_owned(),
                source_ordinal: Some(9),
            },
        ],
    };
    let rendered =
        material_resolution_error("shared", sources.get("shared"), &error, None)
            .to_string();
    if !rendered.contains("primitive-group source ordinals {7, 42}")
        || !rendered.contains("AmbiguousShaderMember")
    {
        return Err(format!(
            "ambiguous shader lost consumer evidence: {rendered}"
        ));
    }
    Ok(())
}

#[test]
fn ambiguous_shader_error_retains_phase_three_model_id()
-> Result<(), String> {
    let package = phase_three_shader_package()?;
    let group = PrimitiveGroup::new(
        0,
        "shared",
        vec![[0., 0., 0.], [1., 0., 0.], [0., 1., 0.]],
        Vec::new(),
        &[0, 1, 2],
    )
    .map(|group| group.with_source_ordinal(42))
    .map_err(|error| format!("primitive group failed: {error:?}"))?;
    let meshes = [fbx::domain::mesh::MeshAsset::new("body", vec![group])
        .map_err(|error| format!("fixture mesh failed: {error:?}"))?];
    let coordinates = [WorldMeshSourceCoordinate {
        member_id: "body",
        source_ordinal: 40,
    }];
    let sources =
        shader_consumer_provenance(&meshes, Some(&coordinates), Some(&package));
    let error = DecodedComponentError::AmbiguousShaderMember {
        shader: "shared".to_owned(),
        occurrences: vec![
            ShaderMemberOccurrence {
                member: "first.json".to_owned(),
                source_ordinal: Some(3),
            },
            ShaderMemberOccurrence {
                member: "second.json".to_owned(),
                source_ordinal: Some(9),
            },
        ],
    };
    let rendered = material_resolution_error(
        "shared",
        sources.get("shared"),
        &error,
        Some(&package),
    )
    .to_string();
    if !rendered.contains(
        r#"phase-three model members {"model-forty"}"#,
    ) || !rendered.contains(
        r#"phase-three material members ["material-three", "material-nine"]"#,
    ) {
        return Err(format!(
            "ambiguous shader lost model provenance: {rendered}"
        ));
    }
    Ok(())
}

#[test]
fn wrong_mesh_coordinate_does_not_invent_phase_three_model_id()
-> Result<(), String> {
    let package = phase_three_shader_package()?;
    let coordinate = WorldMeshSourceCoordinate {
        member_id: "body",
        source_ordinal: 41,
    };
    if model_package_member_id(&package, coordinate).is_some() {
        return Err("wrong mesh coordinate resolved a model member".to_owned());
    }
    Ok(())
}

#[test]
fn ambiguous_shader_error_retains_phase_three_material_ids()
-> Result<(), String> {
    let package = phase_three_shader_package()?;
    let error = DecodedComponentError::AmbiguousShaderMember {
        shader: "shared".to_owned(),
        occurrences: vec![
            ShaderMemberOccurrence {
                member: "first.json".to_owned(),
                source_ordinal: Some(3),
            },
            ShaderMemberOccurrence {
                member: "second.json".to_owned(),
                source_ordinal: Some(9),
            },
        ],
    };
    let provenance = ShaderConsumerProvenance {
        source_ordinals: BTreeSet::from([42_usize]),
        model_member_ids: BTreeSet::new(),
    };
    let rendered = material_resolution_error(
        "shared",
        Some(&provenance),
        &error,
        Some(&package),
    )
    .to_string();
    if !rendered.contains(
        r#"phase-three material members ["material-three", "material-nine"]"#,
    ) {
        return Err(format!(
            "ambiguous shader lost phase-three member evidence: {rendered}"
        ));
    }
    Ok(())
}

#[test]
fn ambiguous_shader_member_mapping_falls_back_on_wrong_coordinate()
-> Result<(), String> {
    let package = phase_three_shader_package()?;
    let error = DecodedComponentError::AmbiguousShaderMember {
        shader: "shared".to_owned(),
        occurrences: vec![ShaderMemberOccurrence {
            member: "first.json".to_owned(),
            source_ordinal: Some(8),
        }],
    };
    let provenance = ShaderConsumerProvenance {
        source_ordinals: BTreeSet::from([42_usize]),
        model_member_ids: BTreeSet::new(),
    };
    let rendered = material_resolution_error(
        "shared",
        Some(&provenance),
        &error,
        Some(&package),
    )
    .to_string();
    if rendered.contains("phase-three material members")
        || !rendered.contains("source_ordinal: Some(8)")
    {
        return Err(format!(
            "wrong shader coordinate did not fail back to exact \
             evidence: {rendered}"
        ));
    }
    Ok(())
}

#[test]
fn non_ambiguous_material_error_keeps_existing_shape() {
    let provenance = ShaderConsumerProvenance {
        source_ordinals: BTreeSet::from([42_usize]),
        model_member_ids: BTreeSet::new(),
    };
    let error = DecodedComponentError::InvalidMemberId("bad".to_owned());
    assert_eq!(
        material_resolution_error("shared", Some(&provenance), &error, None)
            .to_string(),
        "prop material shared failed: InvalidMemberId(\"bad\")"
    );
}

#[test]
fn shared_texture_fallback_preserves_decoded_shader_evidence()
-> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-shared-material-evidence-{}",
        std::process::id()
    ));
    let package = root.join("package");
    let shader_dir = package.join("components").join("shader");
    let shared_dir = root.join("shared");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&shared_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&scratch).map_err(|error| error.to_string())?;
    fs::write(
        shader_dir.join("road_m.json"),
        concat!(
            r#"{"name":"road_m","has_translucency":1,"num_params":2,"#,
            r#""params":[{"kind":"texture","param":"TEX","#,
            r#""value":"shared.bmp"},{"kind":"colour","#,
            r#""param":"DIFF","value":287454020}]}"#
        ),
    )
    .map_err(|error| error.to_string())?;
    let external = shared_dir.join("shared.png");
    fs::write(&external, b"source-png").map_err(|error| error.to_string())?;
    let external_text = external
        .to_str()
        .ok_or_else(|| "external fixture path is not UTF-8".to_owned())?;
    let authority = SharedTextureAuthority::from_occurrences_for_tests(&[
        super::super::texture_authority::TextureOccurrenceFixture {
            logical: "shared.bmp",
            package_id: "level-one-terrain",
            subcategory: "terrain-world/level-01/terrain-mesh",
            package_member_id: "texture-member-1",
            member_id: "shared",
            source_ordinal: 1,
            path: external_text,
            sha256: "fixture-digest",
        },
    ]);
    let source = DecodedComponentSource::new(&package, &scratch);
    let result = resolve_source_material(
        &source,
        &package,
        "road_m",
        None,
        Some(&authority),
        None,
        "terrain-world/level-01/terrain-mesh",
    );
    let cleanup = fs::remove_dir_all(&root);
    let binding = result.map_err(|error| error.to_string())?;
    if binding.base_color_rgba8 != [0x22, 0x33, 0x44, 0x11]
        || !binding.semantics.is_transparent()
    {
        return Err(format!(
            "shared fallback discarded decoded shader evidence: {binding:?}"
        ));
    }
    cleanup.map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn world_missing_shader_uses_runtime_error_material() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-missing-world-shader-{}",
        std::process::id()
    ));
    let shader_dir = root.join("components").join("shader");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(&root, &scratch);
    let authority = SharedTextureAuthority::from_occurrences_for_tests(&[]);
    let package = phase_three_shader_package()?;
    let provenance = ShaderConsumerProvenance {
        source_ordinals: BTreeSet::from([42]),
        model_member_ids: BTreeSet::from(["model-forty".to_owned()]),
    };
    let result = resolve_source_material(
        &source,
        &root,
        "lambert1",
        Some(&provenance),
        Some(&authority),
        Some(&package),
        "terrain-world/level-01/terrain-mesh",
    );
    let cleanup_result = fs::remove_dir_all(&root);
    let binding = result.map_err(|error| error.to_string())?;
    assert_eq!(binding.material_name, "error");
    assert_eq!(binding.texture_file_name, None);
    assert_eq!(binding.semantics, MaterialSemantics::default());
    assert_eq!(binding.base_color_rgba8, [u8::MAX; 4]);
    cleanup_result.map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn world_missing_shader_without_package_consumer_proof_stays_fail_closed()
-> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-unproven-world-missing-shader-{}",
        std::process::id()
    ));
    let shader_dir = root.join("components").join("shader");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(&root, &scratch);
    let authority = SharedTextureAuthority::from_occurrences_for_tests(&[]);
    let package = phase_three_shader_package()?;
    let provenance = ShaderConsumerProvenance {
        source_ordinals: BTreeSet::from([42]),
        model_member_ids: BTreeSet::from(["unknown-model".to_owned()]),
    };
    let result = resolve_source_material(
        &source,
        &root,
        "lambert1",
        Some(&provenance),
        Some(&authority),
        Some(&package),
        "terrain-world/level-01/terrain-mesh",
    );
    let cleanup_result = fs::remove_dir_all(&root);
    let Err(error) = result else {
        return Err(String::from("unproven world shader used runtime fallback"));
    };
    assert!(error.to_string().contains("MissingShaderMember"));
    cleanup_result.map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn non_world_missing_shader_stays_fail_closed() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-missing-non-world-shader-{}",
        std::process::id()
    ));
    let shader_dir = root.join("components").join("shader");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    let source = DecodedComponentSource::new(&root, &scratch);
    let result = resolve_source_material(
        &source,
        &root,
        "lambert1",
        None,
        None,
        None,
        "",
    );
    let cleanup_result = fs::remove_dir_all(&root);
    let Err(error) = result else {
        return Err(String::from(
            "non-world missing shader used runtime fallback",
        ));
    };
    assert!(error.to_string().contains("MissingShaderMember"));
    cleanup_result.map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn canonical_material_identity_separates_surface_semantics() {
    let opaque = canonical_material_identity(
        Some("abc123"),
        MaterialSemantics::default(),
    );
    let glass = canonical_material_identity(
        Some("abc123"),
        MaterialSemantics::default().with_glass(true),
    );
    let emitter = canonical_material_identity(
        Some("abc123"),
        MaterialSemantics::default()
            .with_transparent(true)
            .with_light_emitter(true),
    );
    assert_eq!(opaque, "material-abc123");
    assert_eq!(glass, "material-abc123-glass");
    assert_eq!(emitter, "material-abc123-transparent-light-emitter");
    assert_ne!(opaque, glass);
    assert_ne!(glass, emitter);
}

#[test]
fn canonical_material_preserves_decoded_diffuse_color() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-canonical-material-color-{}",
        std::process::id()
    ));
    let shader_dir = root.join("components").join("shader");
    let texture_dir = root.join("components").join("texture");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&texture_dir).map_err(|error| error.to_string())?;
    fs::write(
        shader_dir.join("road_m.json"),
        concat!(
            r#"{"name":"road_m","num_params":2,"params":[{"#,
            r#""kind":"texture","param":"TEX","value":"road.bmp"},{"#,
            r#""kind":"colour","param":"DIFF","value":287454020}]}"#
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(texture_dir.join("road.png"), b"source-png")
        .map_err(|error| error.to_string())?;
    let result = resolve_materials(
        BTreeSet::from(["road_m".to_owned()]),
        &BTreeMap::new(),
        &root,
        &scratch,
        None,
        None,
        "",
    );
    let cleanup = fs::remove_dir_all(&root);
    let (_renames, materials, _textures) =
        result.map_err(|error| error.to_string())?;
    let material = materials
        .first()
        .ok_or_else(|| "canonical material is missing".to_owned())?;
    if material.base_color_rgba8 != [0x22, 0x33, 0x44, 0x11] {
        return Err(format!(
            "canonical material discarded decoded DIFF: {material:?}"
        ));
    }
    cleanup.map_err(|error| error.to_string())?;
    Ok(())
}

#[test]
fn prepared_texture_preserves_exact_source_bytes() -> Result<(), String> {
    let bytes = vec![10_u8, 20, 30, 40];
    let prepared = prepare_source_texture(bytes.clone());
    if prepared.bytes != bytes {
        return Err("source texture bytes were rewritten".to_owned());
    }
    let expected_digest = shar_sha256::digest_hex(&bytes);
    if prepared.sha256 != expected_digest
        || prepared.file_name != format!("texture-{expected_digest}.png")
    {
        return Err("source texture content identity changed".to_owned());
    }
    Ok(())
}

#[test]
fn runtime_first_duplicate_shader_and_texture_follow_load_order()
-> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-runtime-first-material-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    let package_root = root.join("package");
    let shader_dir = package_root.join("components/shader");
    let texture_dir = package_root.join("components/texture");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&texture_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&scratch).map_err(|error| error.to_string())?;
    fs::write(
        shader_dir.join("first.json"),
        concat!(
            r#"{"name":"shared","has_translucency":0,"num_params":1,"#,
            r#""params":[{"kind":"texture","param":"TEX","#,
            r#""value":"shared.bmp"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        shader_dir.join("second.json"),
        concat!(
            r#"{"name":"shared","has_translucency":1,"num_params":1,"#,
            r#""params":[{"kind":"texture","param":"TEX","#,
            r#""value":"shared.bmp"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(texture_dir.join("shared.png"), b"first-texture")
        .map_err(|error| error.to_string())?;
    fs::write(
        texture_dir.join("shared__ordinal_2.png"),
        b"second-texture",
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        package_root.join("components.jsonl"),
        concat!(
            r#"{"schema":"p3d.package.v1","component_count":4}"#,
            "\n",
            r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"#,
            r#""kind":"texture","name":"shared.bmp","#,
            r#""path":"texture/shared.png"}"#,
            "\n",
            r#"{"ordinal":2,"depth":1,"parent_ordinal":0,"#,
            r#""kind":"texture","name":"shared.bmp","#,
            r#""path":"texture/shared__ordinal_2.png"}"#,
            "\n",
            r#"{"ordinal":3,"depth":1,"parent_ordinal":0,"#,
            r#""kind":"shader","name":"shared","#,
            r#""path":"shader/first.json"}"#,
            "\n",
            r#"{"ordinal":9,"depth":1,"parent_ordinal":0,"#,
            r#""kind":"shader","name":"shared","#,
            r#""path":"shader/second.json"}"#,
            "\n",
        ),
    )
    .map_err(|error| error.to_string())?;
    let package = phase_three_shader_package()?;
    let provenance = ShaderConsumerProvenance {
        source_ordinals: BTreeSet::from([42]),
        model_member_ids: BTreeSet::from(["model-forty".to_owned()]),
    };
    let source = DecodedComponentSource::new(&package_root, &scratch);
    let result = resolve_source_material(
        &source,
        &package_root,
        "shared",
        Some(&provenance),
        None,
        Some(&package),
        "terrain-world/level-01/terrain-mesh",
    );
    let binding = result.map_err(|error| error.to_string())?;
    let staged = fs::read(scratch.join("shared.png"))
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if binding.semantics.is_transparent() || staged != b"first-texture" {
        return Err(format!(
            "runtime first-store material authority drifted: {binding:?}"
        ));
    }
    Ok(())
}

#[test]
fn duplicate_shader_before_consumer_remains_ambiguous() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-runtime-first-order-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    let shader_dir = root.join("components/shader");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    fs::write(shader_dir.join("first.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    fs::write(shader_dir.join("second.json"), r#"{"name":"shared"}"#)
        .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components.jsonl"),
        concat!(
            r#"{"schema":"p3d.package.v1","component_count":2}"#,
            "\n",
            r#"{"ordinal":3,"depth":1,"parent_ordinal":0,"#,
            r#""kind":"shader","name":"shared","#,
            r#""path":"shader/first.json"}"#,
            "\n",
            r#"{"ordinal":9,"depth":1,"parent_ordinal":0,"#,
            r#""kind":"shader","name":"shared","#,
            r#""path":"shader/second.json"}"#,
            "\n",
        ),
    )
    .map_err(|error| error.to_string())?;
    let package = phase_three_shader_package()?;
    let provenance = ShaderConsumerProvenance {
        source_ordinals: BTreeSet::from([2]),
        model_member_ids: BTreeSet::new(),
    };
    let source = DecodedComponentSource::new(&root, &scratch);
    let result = resolve_source_material(
        &source,
        &root,
        "shared",
        Some(&provenance),
        None,
        Some(&package),
        "terrain-world/level-01/terrain-mesh",
    );
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "consumer before first shader invented runtime authority".into(),
        );
    };
    if !error.to_string().contains("AmbiguousShaderMember") {
        return Err(format!("unexpected ordering failure: {error}"));
    }
    Ok(())
}

#[test]
fn runtime_first_duplicate_texture_binds_unique_shader() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-runtime-first-unique-shader-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    let shader_dir = root.join("components/shader");
    let texture_dir = root.join("components/texture");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&texture_dir).map_err(|error| error.to_string())?;
    fs::write(
        shader_dir.join("first.json"),
        concat!(
            r#"{"name":"shared","num_params":1,"params":[{"#,
            r#""kind":"texture","param":"TEX","value":"shared.bmp"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(texture_dir.join("shared.png"), b"first-texture")
        .map_err(|error| error.to_string())?;
    fs::write(
        texture_dir.join("shared__ordinal_2.png"),
        b"second-texture",
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        root.join("components.jsonl"),
        concat!(
            r#"{"schema":"p3d.package.v1","component_count":3}"#,
            "\n",
            r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"#,
            r#""kind":"texture","name":"shared.bmp","#,
            r#""path":"texture/shared.png"}"#,
            "\n",
            r#"{"ordinal":2,"depth":1,"parent_ordinal":0,"#,
            r#""kind":"texture","name":"shared.bmp","#,
            r#""path":"texture/shared__ordinal_2.png"}"#,
            "\n",
            r#"{"ordinal":3,"depth":1,"parent_ordinal":0,"#,
            r#""kind":"shader","name":"shared","#,
            r#""path":"shader/first.json"}"#,
            "\n",
        ),
    )
    .map_err(|error| error.to_string())?;
    let package = phase_three_shader_package()?;
    let provenance = ShaderConsumerProvenance {
        source_ordinals: BTreeSet::from([42]),
        model_member_ids: BTreeSet::from(["model-forty".to_owned()]),
    };
    let source = DecodedComponentSource::new(&root, &scratch);
    let _binding = resolve_source_material(
        &source,
        &root,
        "shared",
        Some(&provenance),
        None,
        Some(&package),
        "terrain-world/level-01/terrain-mesh",
    )
    .map_err(|error| error.to_string())?;
    let staged = fs::read(scratch.join("shared.png"))
        .map_err(|error| error.to_string())?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if staged != b"first-texture" {
        return Err(
            "unique shader did not bind first runtime texture".to_owned(),
        );
    }
    Ok(())
}


#[test]
fn shared_texture_authority_preserves_authored_name_over_cache_name()
-> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-shared-normalized-texture-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    let package = root.join("package");
    let shader_dir = package.join("components/shader");
    let shared_dir = root.join("shared");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&shared_dir).map_err(|error| error.to_string())?;
    fs::write(
        shader_dir.join("glass.json"),
        concat!(
            r#"{"name":"glass","num_params":1,"params":[{"#,
            r#""kind":"texture","param":"TEX","#,
            r#""value":"Krusty_ HumanCola_ Glass_ 8bitt_64x64.bmp"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let external = shared_dir.join(
        "Krusty__HumanCola__Glass__8bitt_64x64.png",
    );
    fs::write(&external, b"source-png").map_err(|error| error.to_string())?;
    let external_text = external
        .to_str()
        .ok_or_else(|| "shared fixture path is not UTF-8".to_owned())?;
    let authority = SharedTextureAuthority::from_occurrences_for_tests(&[
        super::super::texture_authority::TextureOccurrenceFixture {
            logical: "Krusty_ HumanCola_ Glass_ 8bitt_64x64.bmp",
            package_id: "level-one-terrain",
            subcategory: "terrain-world/level-01/terrain-mesh",
            package_member_id: "texture-member-1",
            member_id: "normalized-cache-member",
            source_ordinal: 1,
            path: external_text,
            sha256: "fixture-digest",
        },
    ]);
    let source = DecodedComponentSource::new(&package, &scratch);
    let _binding = resolve_source_material(
        &source,
        &package,
        "glass",
        None,
        Some(&authority),
        None,
        "terrain-world/level-01/regions/l1r1",
    )
    .map_err(|error| error.to_string())?;
    let staged = scratch.join(
        "Krusty_ HumanCola_ Glass_ 8bitt_64x64.png",
    );
    let bytes = fs::read(&staged).map_err(|error| error.to_string())?;
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    if bytes != b"source-png" {
        return Err(
            "shared authored texture identity was not staged".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn independent_shaders_isolate_same_named_texture_staging(
) -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-independent-shader-staging-{}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    let package = root.join("package");
    let shader_dir = package.join("components/shader");
    let first_dir = root.join("first");
    let second_dir = root.join("second");
    let scratch = root.join("scratch");
    fs::create_dir_all(&shader_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&first_dir).map_err(|error| error.to_string())?;
    fs::create_dir_all(&second_dir).map_err(|error| error.to_string())?;
    fs::write(
        shader_dir.join("first_m.json"),
        concat!(
            r#"{"name":"first_m","num_params":1,"params":[{"#,
            r#""kind":"texture","param":"TEX","value":"first.bmp"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(
        shader_dir.join("second_m.json"),
        concat!(
            r#"{"name":"second_m","num_params":1,"params":[{"#,
            r#""kind":"texture","param":"TEX","value":"second.bmp"}]}"#,
        ),
    )
    .map_err(|error| error.to_string())?;
    let first = first_dir.join("shared.png");
    let second = second_dir.join("shared.png");
    fs::write(&first, b"first-payload").map_err(|error| error.to_string())?;
    fs::write(&second, b"second-payload").map_err(|error| error.to_string())?;
    let first_text = first
        .to_str()
        .ok_or_else(|| "first fixture path is not UTF-8".to_owned())?;
    let second_text = second
        .to_str()
        .ok_or_else(|| "second fixture path is not UTF-8".to_owned())?;
    let first_digest = shar_sha256::digest_hex(b"first-payload");
    let second_digest = shar_sha256::digest_hex(b"second-payload");
    let authority = SharedTextureAuthority::from_occurrences_for_tests(&[
        super::super::texture_authority::TextureOccurrenceFixture {
            logical: "first.bmp",
            package_id: "first-package",
            subcategory: "terrain-world/level-01/terrain-mesh",
            package_member_id: "first-member",
            member_id: "first",
            source_ordinal: 1,
            path: first_text,
            sha256: &first_digest,
        },
        super::super::texture_authority::TextureOccurrenceFixture {
            logical: "second.bmp",
            package_id: "second-package",
            subcategory: "terrain-world/level-01/terrain-mesh",
            package_member_id: "second-member",
            member_id: "second",
            source_ordinal: 2,
            path: second_text,
            sha256: &second_digest,
        },
    ]);
    let result = resolve_materials(
        BTreeSet::from(["first_m".to_owned(), "second_m".to_owned()]),
        &BTreeMap::new(),
        &package,
        &scratch,
        Some(&authority),
        None,
        "terrain-world/level-01/terrain-mesh",
    );
    let cleanup = fs::remove_dir_all(&root);
    let (_renames, materials, textures) =
        result.map_err(|error| error.to_string())?;
    if materials.len() != 2 || textures.len() != 2 {
        return Err(format!(
            "independent textures collapsed: materials={} textures={}",
            materials.len(),
            textures.len()
        ));
    }
    let payloads = textures
        .iter()
        .map(|texture| texture.bytes.as_slice())
        .collect::<BTreeSet<_>>();
    if payloads != BTreeSet::from([
        b"first-payload".as_slice(),
        b"second-payload".as_slice(),
    ]) {
        return Err(format!("independent texture bytes changed: {payloads:?}"));
    }
    cleanup.map_err(|error| error.to_string())?;
    Ok(())
}
