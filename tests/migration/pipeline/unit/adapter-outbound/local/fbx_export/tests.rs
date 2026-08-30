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

use crate::domain::package::{
    PackageRole, PhaseThreePackageMember, PhaseThreePackageRow,
};

use super::{
    GENERAL_CHARACTER_ANIMATION_SUBCATEGORY, animation_subcategory_candidates,
    classify_members, deferred_material_identity, fbx_io_error,
    normalized_texture_png_file_name, order_character_mesh_members,
    ordered_shader_names,
    single_package_staging_path,
};

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
