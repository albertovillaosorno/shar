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

use fbx::domain::texture::MaterialBinding;

use crate::domain::package::{
    PackageRole, PhaseThreePackageIndex, PhaseThreePackageMember,
    PhaseThreePackageRow,
};

use super::{
    GENERAL_CHARACTER_ANIMATION_SUBCATEGORY, animation_member_paths,
    animation_subcategory_candidates, body_texture_file_name, classify_members,
    deferred_material_identity, fbx_io_error,
    normalized_texture_png_file_name, order_character_mesh_members,
    ordered_shader_names, resolve_shared_texture_member, shared_eye_frame_paths,
    single_package_staging_path,
};

#[test]
fn body_texture_requires_each_shader_to_resolve_its_own_texture()
-> Result<(), String> {
    let unresolved = MaterialBinding::new("char_swatches_m", None)
        .map_err(|error| format!("unresolved binding failed: {error:?}"))?;
    let bindings = BTreeMap::from([(
        unresolved.material_name.as_str(),
        &unresolved,
    )]);
    let result = body_texture_file_name("char_swatches_m", &bindings);
    let Err(error) = result else {
        return Err(
            "unresolved body shader borrowed another texture".to_owned(),
        );
    };
    if !error
        .to_string()
        .contains("has no resolved source texture")
    {
        return Err(format!("body texture failure changed: {error}"));
    }
    Ok(())
}

#[test]
fn deferred_material_preserves_decoded_shader_identity() {
    assert_eq!(
        deferred_material_identity(
            "char_swatches_lit_m_",
            "char_swatches_lit_m",
        ),
        "char_swatches_lit_m"
    );
}

#[test]
fn normalizes_trailing_nul_padded_texture_reference() {
    let result = normalized_texture_png_file_name(
        "char_swatches_lit.bmp\u{0}\u{0}\u{0}",
    );

    assert!(
        result.is_ok(),
        "fixed-width texture padding should normalize: {result:?}"
    );
    assert_eq!(result.ok().as_deref(), Some("char_swatches_lit.png"));
}

#[test]
fn shared_texture_member_requires_exact_source_identity()
-> Result<(), String> {
    let row = concat!(
        r#"{"package_id":"extracted-art-chars-global","#,
        r#""package_root":"extracted/art/chars/global","#,
        r#""package_category":"characters","#,
        r#""package_subcategory":"characters/rig/common","unit_count":1,"#,
        r#""text_key_count":0,"unit_ids":["swatch-lit"],"world_ids":[],"#,
        r#""texture_ids":["swatch-lit"],"material_ids":[],"model_ids":[],"#,
        r#""physics_ids":[],"animation_ids":[],"scene_ids":[],"#,
        r#""locator_ids":[],"camera_ids":[],"light_ids":[],"#,
        r#""particle_ids":[],"controller_ids":[],"audio_ids":[],"#,
        r#""movie_ids":[],"script_ids":[],"text_ids":[],"ui_ids":[],"#,
        r#""metadata_ids":[],"error_ids":[],"source_unit_ids":[],"#,
        r#""text_key_ids":[],"members":[{"id":"swatch-lit","#,
        r#""role":"texture","#,
        r#""path":"components/texture/char_swatches_lit.png","#,
        r#""type":"image","kind":"p3d-texture","#,
        r#""source_chunk_kind":"texture","source_chunk_ordinal":"1"}],"#,
        r#""text_keys":[]}"#,
    );
    let index = PhaseThreePackageIndex::from_jsonl(row)
        .map_err(|error| error.to_string())?;
    if resolve_shared_texture_member(&index, "char_swatches.bmp")
        .map_err(|error| error.to_string())?
        .is_some()
    {
        return Err(String::from(
            "plain swatch reference resolved the distinct lit source",
        ));
    }
    let resolved =
        resolve_shared_texture_member(&index, "char_swatches_lit.bmp")
            .map_err(|error| error.to_string())?
            .ok_or_else(|| {
                "exact lit swatch identity was not resolved".to_owned()
            })?;
    assert_eq!(
        std::path::Path::new(&resolved.1.path)
            .file_name()
            .and_then(|value| value.to_str()),
        Some("char_swatches_lit.png")
    );
    Ok(())
}

#[test]
fn character_animation_candidates_prefer_identity_specific_banks() {
    assert_eq!(
        animation_subcategory_candidates("characters/apu/base-model"),
        vec![
            "characters/apu/animation-set".to_owned(),
            GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned(),
        ]
    );
    assert_eq!(
        animation_subcategory_candidates("characters/lisa/costume/cool"),
        vec![
            "characters/lisa/animation-set".to_owned(),
            GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned(),
        ]
    );
}

#[test]
fn character_animation_candidates_use_general_bank_for_other_models() {
    assert_eq!(
        animation_subcategory_candidates("characters/krusty/base-model"),
        vec![
            "characters/krusty/animation-set".to_owned(),
            GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned(),
        ]
    );
    assert_eq!(
        animation_subcategory_candidates("characters/boy1/crowd-model"),
        vec![GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned()]
    );
    assert_eq!(
        animation_subcategory_candidates("characters/homer/base-model"),
        vec![GENERAL_CHARACTER_ANIMATION_SUBCATEGORY.to_owned()]
    );
}

#[test]
fn package_staging_is_a_hidden_sibling_of_the_final_package() {
    let root = std::path::Path::new("generated/fbx");
    assert_eq!(
        single_package_staging_path(root, "extracted-art-h2h-flag"),
        root.join(".extracted-art-h2h-flag.fbx-staging")
    );
}

#[test]
fn fbx_io_diagnostics_hide_physical_error_text() {
    let private_fragment = "private-workstation/fbx/staging/file.fbx";
    let error = std::io::Error::other(private_fragment);
    let rendered =
        fbx_io_error("read canonical FBX source", &error).to_string();
    assert_eq!(rendered, "read canonical FBX source failed (Other)");
    assert!(!rendered.contains(private_fragment));
}

#[test]
fn shader_names_preserve_package_member_order() -> Result<(), String> {
    let names = ordered_shader_names([
        "zebra".to_owned(),
        "alpha".to_owned(),
        "middle".to_owned(),
    ])
    .map_err(|error| error.to_string())?;
    assert_eq!(names, ["zebra", "alpha", "middle"]);
    Ok(())
}

#[test]
fn duplicate_shader_identity_fails_closed() -> Result<(), String> {
    let result = ordered_shader_names([
        "shared".to_owned(),
        "shared".to_owned(),
    ]);
    let Err(error) = result else {
        return Err("duplicate shader identity was accepted".to_owned());
    };
    assert_eq!(
        error.to_string(),
        "package material list repeats shader identity shared"
    );
    Ok(())
}


#[test]
fn character_mesh_members_follow_source_chunk_ordinals() -> Result<(), String> {
    let json = concat!(
        r#"{"package_id":"character-order","package_root":"character-order","#,
        r#""package_category":"characters","#,
        r#""package_subcategory":"characters/test/base-model","#,
        r#""unit_count":5,"text_key_count":0,"#,
        r#""unit_ids":["skeleton","skin","mesh-181","mesh-170","mesh-192"],"#,
        r#""world_ids":[],"texture_ids":[],"material_ids":[],"#,
        r#""model_ids":["skeleton","skin","mesh-181","mesh-170","mesh-192"],"#,
        r#""physics_ids":[],"animation_ids":[],"scene_ids":[],"#,
        r#""locator_ids":[],"camera_ids":[],"light_ids":[],"#,
        r#""particle_ids":[],"controller_ids":[],"audio_ids":[],"#,
        r#""movie_ids":[],"script_ids":[],"text_ids":[],"ui_ids":[],"#,
        r#""metadata_ids":[],"error_ids":[],"source_unit_ids":[],"#,
        r#""text_key_ids":[],"members":["#,
        r#"{"id":"skeleton","role":"model","path":"a-skeleton.json","#,
        r#""type":"model","kind":"p3d-skeleton","#,
        r#""source_chunk_kind":"skeleton","source_chunk_ordinal":"1"},"#,
        r#"{"id":"skin","role":"model","path":"b-skin.json","#,
        r#""type":"model","kind":"p3d-skin","#,
        r#""source_chunk_kind":"skin","source_chunk_ordinal":"203"},"#,
        r#"{"id":"mesh-181","role":"model","path":"c-mesh.json","#,
        r#""type":"model","kind":"p3d-mesh","#,
        r#""source_chunk_kind":"mesh","source_chunk_ordinal":"181"},"#,
        r#"{"id":"mesh-170","role":"model","path":"d-mesh.json","#,
        r#""type":"model","kind":"p3d-mesh","#,
        r#""source_chunk_kind":"mesh","source_chunk_ordinal":"170"},"#,
        r#"{"id":"mesh-192","role":"model","path":"e-mesh.json","#,
        r#""type":"model","kind":"p3d-mesh","#,
        r#""source_chunk_kind":"mesh","source_chunk_ordinal":"192"}],"#,
        r#""text_keys":[]}"#,
    );
    let package = PhaseThreePackageRow::from_json_line(json)
        .map_err(|error| error.to_string())?;
    let members = classify_members(&package)
        .map_err(|error| error.to_string())?;
    let ids = members
        .meshes
        .iter()
        .map(|member| member.id.as_str())
        .collect::<Vec<_>>();
    assert_eq!(ids, ["mesh-170", "mesh-181", "mesh-192"]);
    Ok(())
}


fn character_mesh_member(
    id: &str,
    source_chunk_ordinal: Option<usize>,
) -> PhaseThreePackageMember {
    PhaseThreePackageMember {
        id: id.to_owned(),
        role: PackageRole::Model,
        path: format!("{id}.json"),
        unit_type: "model".to_owned(),
        kind: "p3d-mesh".to_owned(),
        source_chunk_kind: "mesh".to_owned(),
        source_chunk_ordinal,
    }
}

#[test]
fn character_mesh_order_rejects_missing_source_ordinal() -> Result<(), String> {
    let first = character_mesh_member("first", None);
    let mut meshes = vec![&first];
    let Err(error) = order_character_mesh_members(&mut meshes) else {
        return Err(
            "character mesh without source ordinal was accepted".to_owned(),
        );
    };
    assert!(error.to_string().contains("has no source chunk ordinal"));
    Ok(())
}

#[test]
fn character_mesh_order_rejects_duplicate_source_ordinals(
) -> Result<(), String> {
    let first = character_mesh_member("first", Some(10));
    let second = character_mesh_member("second", Some(10));
    let mut meshes = vec![&first, &second];
    let Err(error) = order_character_mesh_members(&mut meshes) else {
        return Err(
            "duplicate character mesh source ordinals were accepted".to_owned(),
        );
    };
    assert!(
        error
            .to_string()
            .contains("repeats source mesh ordinal 10")
    );
    Ok(())
}


#[test]
fn character_animation_paths_follow_source_chunk_ordinals(
) -> Result<(), String> {
    let json = concat!(
        r#"{"package_id":"animation-order","package_root":"animation-order","#,
        r#""package_category":"characters","#,
        r#""package_subcategory":"characters/test/animation-set","#,
        r#""unit_count":2,"text_key_count":0,"#,
        r#""unit_ids":["animation-a","animation-z"],"#,
        r#""world_ids":[],"texture_ids":[],"material_ids":[],"model_ids":[],"#,
        r#""physics_ids":[],"animation_ids":["animation-a","animation-z"],"#,
        r#""scene_ids":[],"locator_ids":[],"camera_ids":[],"light_ids":[],"#,
        r#""particle_ids":[],"controller_ids":[],"audio_ids":[],"#,
        r#""movie_ids":[],"script_ids":[],"text_ids":[],"ui_ids":[],"#,
        r#""metadata_ids":[],"error_ids":[],"source_unit_ids":[],"#,
        r#""text_key_ids":[],"members":["#,
        r#"{"id":"animation-a","role":"animation","path":"a.json","#,
        r#""type":"animation","kind":"p3d-animation","#,
        r#""source_chunk_kind":"animation","source_chunk_ordinal":"20"},"#,
        r#"{"id":"animation-z","role":"animation","path":"z.json","#,
        r#""type":"animation","kind":"p3d-animation","#,
        r#""source_chunk_kind":"animation","source_chunk_ordinal":"10"}],"#,
        r#""text_keys":[]}"#,
    );
    let package = PhaseThreePackageRow::from_json_line(json)
        .map_err(|error| error.to_string())?;
    let paths = animation_member_paths(
        Some(&package),
        std::path::Path::new("normalized"),
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(
        paths,
        [
            std::path::Path::new("normalized").join("z.json"),
            std::path::Path::new("normalized").join("a.json"),
        ]
    );
    Ok(())
}


#[test]
fn shared_eye_frames_reject_duplicate_frame_identity() -> Result<(), String> {
    let row = concat!(
        r#"{"package_id":"extracted-test-global","#,
        r#""package_root":"extracted/test/global","#,
        r#""package_category":"characters","#,
        r#""package_subcategory":"characters/rig/common","unit_count":5,"#,
        r#""text_key_count":0,"unit_ids":["eye-zero-a","eye-one","eye-two","#,
        r#""eye-three","eye-zero-z"],"world_ids":[],"texture_ids":["#,
        r#""eye-zero-a","eye-one","eye-two","eye-three","eye-zero-z"],"#,
        r#""material_ids":[],"model_ids":[],"physics_ids":[],"#,
        r#""animation_ids":[],"#,
        r#""scene_ids":[],"locator_ids":[],"camera_ids":[],"light_ids":[],"#,
        r#""particle_ids":[],"controller_ids":[],"audio_ids":[],"#,
        r#""movie_ids":[],"#,
        r#""script_ids":[],"text_ids":[],"ui_ids":[],"metadata_ids":[],"#,
        r#""error_ids":[],"source_unit_ids":[],"text_key_ids":[],"members":["#,
        r#"{"id":"eye-zero-a","role":"texture","path":"a/eyeball.bmp.0.png","#,
        r#""type":"image","kind":"p3d-texture","source_chunk_kind":"texture","#,
        r#""source_chunk_ordinal":"1"},{"id":"eye-one","role":"texture","#,
        r#""path":"frames/eyeball.bmp.1.png","type":"image","#,
        r#""kind":"p3d-texture","#,
        r#""source_chunk_kind":"texture","source_chunk_ordinal":"2"},"#,
        r#"{"id":"eye-two","#,
        r#""role":"texture","path":"frames/eyeball.bmp.2.png","type":"image","#,
        r#""kind":"p3d-texture","source_chunk_kind":"texture","#,
        r#""source_chunk_ordinal":"3"},{"id":"eye-three","role":"texture","#,
        r#""path":"frames/eyeball.bmp.3.png","type":"image","#,
        r#""kind":"p3d-texture","#,
        r#""source_chunk_kind":"texture","source_chunk_ordinal":"4"},"#,
        r#"{"id":"eye-zero-z","#,
        r#""role":"texture","path":"z/eyeball.bmp.0.png","type":"image","#,
        r#""kind":"p3d-texture","source_chunk_kind":"texture","#,
        r#""source_chunk_ordinal":"5"}],"text_keys":[]}"#,
    );
    let index = PhaseThreePackageIndex::from_jsonl(row)
        .map_err(|error| error.to_string())?;
    let result = shared_eye_frame_paths(
        &index,
        std::path::Path::new("normalized"),
    );
    let Err(error) = result else {
        return Err(
            "duplicate shared eye frame identity was accepted".to_owned(),
        );
    };
    assert!(error.to_string().contains("shared eye frame is ambiguous"));
    Ok(())
}
