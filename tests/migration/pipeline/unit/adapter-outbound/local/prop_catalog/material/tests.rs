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

use std::collections::BTreeSet;
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
    resolve_source_material, shader_consumer_provenance,
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
fn missing_analysis_default_shader_fails_closed() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "pipeline-missing-analysis-default-{}",
        std::process::id()
    ));
    let scratch = root.join("scratch");
    let source = DecodedComponentSource::new(&root, &scratch);
    let authority = SharedTextureAuthority::from_occurrences_for_tests(&[]);
    let result = resolve_source_material(
        &source,
        "lambert1",
        None,
        Some(&authority),
        None,
        "terrain-world/level-01/terrain-mesh",
    );
    let Err(error) = result else {
        return Err(String::from(
            "missing analysis shader invented an untextured material",
        ));
    };
    if !error.to_string().contains("prop material lambert1 failed: Read") {
        return Err(format!(
            "missing analysis shader changed failure boundary: {error}"
        ));
    }
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
