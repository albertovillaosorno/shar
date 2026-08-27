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


use super::{
    ConversionFamily, FbxTargetKind, PhaseThreePackagePlanner, UnrealTargetKind,
};
use crate::domain::package::index::PhaseThreePackageRow;

fn row(
    category: &str,
    subcategory: &str,
    role_field: &str,
) -> Result<PhaseThreePackageRow, String> {
    row_with_kind(category, subcategory, role_field, "test")
}

fn row_with_kind(
    category: &str,
    subcategory: &str,
    role_field: &str,
    kind: &str,
) -> Result<PhaseThreePackageRow, String> {
    let mut json = concat!(
        "{\"package_id\":\"pkg\",\"package_root\":\"pkg\",",
        "\"package_category\":\"CATEGORY\",",
        "\"package_subcategory\":\"SUBCATEGORY\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"unit-a\"],\"world_ids\":[],",
        "\"texture_ids\":[],\"material_ids\":[],",
        "\"model_ids\":[],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[],",
        "\"metadata_ids\":[],\"error_ids\":[],",
        "\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[],\"text_keys\":[]}",
    )
    .replace("SUBCATEGORY", subcategory)
    .replace("CATEGORY", category);
    let empty_field = format!("\"{role_field}\":[]");
    let filled_field = format!("\"{role_field}\":[\"unit-a\"]");
    json = json.replace(&empty_field, &filled_field);
    let role = role_field
        .strip_suffix("_ids")
        .ok_or_else(|| format!("invalid role field: {role_field}"))?;
    let member = format!(
        concat!(
            "\"members\":[{{",
            "\"id\":\"unit-a\",",
            "\"role\":\"{}\",",
            "\"path\":\"extracted/unit-a.bin\",",
            "\"type\":\"test\",",
            "\"kind\":\"{}\",",
            "\"source_chunk_kind\":\"test\"}}]"
        ),
        role, kind,
    );
    json = json.replace("\"members\":[]", &member);
    PhaseThreePackageRow::from_json_line(&json)
        .map_err(|error| error.to_string())
}

fn row_with_exact_member(
    category: &str,
    subcategory: &str,
    role_field: &str,
    kind: &str,
    source_chunk_kind: &str,
) -> Result<PhaseThreePackageRow, String> {
    let mut json = concat!(
        "{\"package_id\":\"pkg\",\"package_root\":\"pkg\",",
        "\"package_category\":\"CATEGORY\",",
        "\"package_subcategory\":\"SUBCATEGORY\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"unit-a\"],\"world_ids\":[],",
        "\"texture_ids\":[],\"material_ids\":[],",
        "\"model_ids\":[],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[],",
        "\"metadata_ids\":[],\"error_ids\":[],",
        "\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[],\"text_keys\":[]}",
    )
    .replace("SUBCATEGORY", subcategory)
    .replace("CATEGORY", category);
    let empty_field = format!("\"{role_field}\":[]");
    let filled_field = format!("\"{role_field}\":[\"unit-a\"]");
    json = json.replace(&empty_field, &filled_field);
    let role = role_field
        .strip_suffix("_ids")
        .ok_or_else(|| format!("invalid role field: {role_field}"))?;
    let member = format!(
        concat!(
            "\"members\":[{{",
            "\"id\":\"unit-a\",\"role\":\"{}\",",
            "\"path\":\"extracted/unit-a.json\",",
            "\"type\":\"test\",\"kind\":\"{}\",",
            "\"source_chunk_kind\":\"{}\"}}]"
        ),
        role, kind, source_chunk_kind,
    );
    json = json.replace("\"members\":[]", &member);
    PhaseThreePackageRow::from_json_line(&json)
        .map_err(|error| error.to_string())
}

fn row_with_two_static_meshes() -> Result<PhaseThreePackageRow, String> {
    let row = concat!(
        "{\"package_id\":\"pkg\",\"package_root\":\"pkg\",",
        "\"package_category\":\"ui-resources\",",
        "\"package_subcategory\":\"ui-resources/test\",",
        "\"unit_count\":2,\"text_key_count\":0,",
        "\"unit_ids\":[\"mesh-a\",\"mesh-b\"],",
        "\"world_ids\":[],\"texture_ids\":[],",
        "\"material_ids\":[],",
        "\"model_ids\":[\"mesh-a\",\"mesh-b\"],",
        "\"physics_ids\":[],\"animation_ids\":[],",
        "\"scene_ids\":[],\"locator_ids\":[],",
        "\"camera_ids\":[],\"light_ids\":[],",
        "\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],",
        "\"script_ids\":[],\"text_ids\":[],",
        "\"ui_ids\":[],\"metadata_ids\":[],",
        "\"error_ids\":[],\"source_unit_ids\":[],",
        "\"text_key_ids\":[],\"members\":[",
        "{\"id\":\"mesh-a\",\"role\":\"model\",",
        "\"path\":\"extracted/mesh-a.json\",",
        "\"type\":\"model\",\"kind\":\"p3d-mesh\",",
        "\"source_chunk_kind\":\"mesh\"},",
        "{\"id\":\"mesh-b\",\"role\":\"model\",",
        "\"path\":\"extracted/mesh-b.json\",",
        "\"type\":\"model\",\"kind\":\"p3d-mesh\",",
        "\"source_chunk_kind\":\"mesh\"}],",
        "\"text_keys\":[]}",
    );
    PhaseThreePackageRow::from_json_line(row).map_err(|error| error.to_string())
}

fn row_with_texture_path(
    category: &str,
    kind: &str,
    path: &str,
) -> Result<PhaseThreePackageRow, String> {
    let json = format!(
        concat!(
            "{{\"package_id\":\"pkg\",\"package_root\":\"pkg\",",
            "\"package_category\":\"{}\",",
            "\"package_subcategory\":\"{}/sample\",",
            "\"unit_count\":1,\"text_key_count\":0,",
            "\"unit_ids\":[\"unit-a\"],\"world_ids\":[],",
            "\"texture_ids\":[\"unit-a\"],\"material_ids\":[],",
            "\"model_ids\":[],\"physics_ids\":[],",
            "\"animation_ids\":[],\"scene_ids\":[],",
            "\"locator_ids\":[],\"camera_ids\":[],",
            "\"light_ids\":[],\"particle_ids\":[],",
            "\"controller_ids\":[],\"audio_ids\":[],",
            "\"movie_ids\":[],\"script_ids\":[],",
            "\"text_ids\":[],\"ui_ids\":[],",
            "\"metadata_ids\":[],\"error_ids\":[],",
            "\"source_unit_ids\":[],\"text_key_ids\":[],",
            "\"members\":[{{\"id\":\"unit-a\",",
            "\"role\":\"texture\",\"path\":\"{}\",",
            "\"type\":\"image\",\"kind\":\"{}\",",
            "\"source_chunk_kind\":\"none\"}}],",
            "\"text_keys\":[]}}"
        ),
        category,
        category,
        path,
        kind,
    );
    PhaseThreePackageRow::from_json_line(&json)
        .map_err(|error| error.to_string())
}

fn single_skeletal_row() -> Result<PhaseThreePackageRow, String> {
    let row = concat!(
        "{\"package_id\":\"pkg\",\"package_root\":\"pkg\",",
        "\"package_category\":\"characters\",",
        "\"package_subcategory\":\"characters/cletus/base-model\",",
        "\"unit_count\":3,\"text_key_count\":0,",
        "\"unit_ids\":[\"composite-a\",\"skin-a\",\"skeleton-a\"],",
        "\"world_ids\":[],\"texture_ids\":[],\"material_ids\":[],",
        "\"model_ids\":[\"composite-a\",\"skin-a\"],",
        "\"physics_ids\":[],\"animation_ids\":[\"skeleton-a\"],",
        "\"scene_ids\":[],\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[],\"metadata_ids\":[],",
        "\"error_ids\":[],\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[",
        "{\"id\":\"composite-a\",\"role\":\"model\",",
        "\"path\":\"extracted/composite.json\",\"type\":\"model\",",
        "\"kind\":\"p3d-composite-drawable\",",
        "\"source_chunk_kind\":\"composite_drawable\"},",
        "{\"id\":\"skin-a\",\"role\":\"model\",",
        "\"path\":\"extracted/skin.json\",\"type\":\"model\",",
        "\"kind\":\"p3d-skin\",\"source_chunk_kind\":\"skin\"},",
        "{\"id\":\"skeleton-a\",\"role\":\"animation\",",
        "\"path\":\"extracted/skeleton.json\",\"type\":\"animation\",",
        "\"kind\":\"p3d-skeleton\",",
        "\"source_chunk_kind\":\"skeleton\"}],\"text_keys\":[]}",
    );
    PhaseThreePackageRow::from_json_line(row).map_err(|error| error.to_string())
}

fn row_with_duplicate_skeleton() -> Result<PhaseThreePackageRow, String> {
    let row = concat!(
        "{\"package_id\":\"pkg\",\"package_root\":\"pkg\",",
        "\"package_category\":\"characters\",",
        "\"package_subcategory\":\"characters/test/base-model\",",
        "\"unit_count\":4,\"text_key_count\":0,",
        "\"unit_ids\":[\"composite-a\",\"skin-a\",",
        "\"skeleton-a\",\"skeleton-b\"],",
        "\"world_ids\":[],\"texture_ids\":[],",
        "\"material_ids\":[],",
        "\"model_ids\":[\"composite-a\",\"skin-a\"],",
        "\"physics_ids\":[],",
        "\"animation_ids\":[\"skeleton-a\",\"skeleton-b\"],",
        "\"scene_ids\":[],\"locator_ids\":[],",
        "\"camera_ids\":[],\"light_ids\":[],",
        "\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],",
        "\"script_ids\":[],\"text_ids\":[],",
        "\"ui_ids\":[],\"metadata_ids\":[],",
        "\"error_ids\":[],\"source_unit_ids\":[],",
        "\"text_key_ids\":[],\"members\":[",
        "{\"id\":\"composite-a\",\"role\":\"model\",",
        "\"path\":\"extracted/composite.json\",",
        "\"type\":\"model\",",
        "\"kind\":\"p3d-composite-drawable\",",
        "\"source_chunk_kind\":\"composite_drawable\"},",
        "{\"id\":\"skin-a\",\"role\":\"model\",",
        "\"path\":\"extracted/skin.json\",",
        "\"type\":\"model\",\"kind\":\"p3d-skin\",",
        "\"source_chunk_kind\":\"skin\"},",
        "{\"id\":\"skeleton-a\",\"role\":\"animation\",",
        "\"path\":\"extracted/skeleton-a.json\",",
        "\"type\":\"animation\",\"kind\":\"p3d-skeleton\",",
        "\"source_chunk_kind\":\"skeleton\"},",
        "{\"id\":\"skeleton-b\",\"role\":\"animation\",",
        "\"path\":\"extracted/skeleton-b.json\",",
        "\"type\":\"animation\",\"kind\":\"p3d-skeleton\",",
        "\"source_chunk_kind\":\"skeleton\"}],",
        "\"text_keys\":[]}",
    );
    PhaseThreePackageRow::from_json_line(row).map_err(|error| error.to_string())
}

fn row_with_model_and_companion(
    companion_field: &str,
) -> Result<PhaseThreePackageRow, String> {
    let companion_role = companion_field
        .strip_suffix("_ids")
        .ok_or_else(|| format!("invalid role field: {companion_field}"))?;
    let mut json = concat!(
        "{\"package_id\":\"pkg\",\"package_root\":\"pkg\",",
        "\"package_category\":\"cars\",",
        "\"package_subcategory\":\"cars/character-rigs/homer-v\",",
        "\"unit_count\":2,\"text_key_count\":0,",
        "\"unit_ids\":[\"model-a\",\"companion-a\"],\"world_ids\":[],",
        "\"texture_ids\":[],\"material_ids\":[],",
        "\"model_ids\":[\"model-a\"],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[],",
        "\"metadata_ids\":[],\"error_ids\":[],",
        "\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[",
        "{\"id\":\"model-a\",\"role\":\"model\",",
        "\"path\":\"extracted/model-a.bin\",\"type\":\"test\",",
        "\"kind\":\"test\",\"source_chunk_kind\":\"test\"},",
        "{\"id\":\"companion-a\",\"role\":\"COMPANION_ROLE\",",
        "\"path\":\"extracted/companion-a.bin\",\"type\":\"test\",",
        "\"kind\":\"test\",\"source_chunk_kind\":\"test\"}],",
        "\"text_keys\":[]}",
    )
    .replace("COMPANION_ROLE", companion_role);
    let empty_field = format!("\"{companion_field}\":[]");
    let filled_field = format!("\"{companion_field}\":[\"companion-a\"]");
    json = json.replace(&empty_field, &filled_field);
    PhaseThreePackageRow::from_json_line(&json)
        .map_err(|error| error.to_string())
}

#[test]
fn preserves_scene_assembly_roles_with_fbx_geometry() -> Result<(), String> {
    for role_field in ["scene_ids", "locator_ids", "camera_ids"] {
        let package = row_with_model_and_companion(role_field)?;
        let plan = PhaseThreePackagePlanner::plan(&package);
        let Some(fbx) = plan.fbx else {
            return Err(format!(
                "model plus {role_field} should produce an FBX plan"
            ));
        };
        let retained = [
            fbx.model_ids,
            fbx.world_ids,
            fbx.scene_ids,
            fbx.locator_ids,
            fbx.camera_ids,
            fbx.animation_ids,
            fbx.texture_ids,
            fbx.material_ids,
            fbx.physics_ids,
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        if retained != ["model-a".to_owned(), "companion-a".to_owned()] {
            return Err(format!("FBX plan dropped {role_field}: {retained:?}"));
        }
    }
    Ok(())
}

#[test]
fn routes_non_geometry_scene_evidence_to_semantic_conversion()
-> Result<(), String> {
    for (category, subcategory, role_field) in [
        (
            "ui-resources",
            "ui-resources/backend/hazards/bomb",
            "scene_ids",
        ),
        ("missions", "missions/level-02/locators", "locator_ids"),
        ("characters", "characters/homer/animations", "animation_ids"),
    ] {
        let package = row(category, subcategory, role_field)?;
        let plan = PhaseThreePackagePlanner::plan(&package);
        if plan.family != ConversionFamily::UnrealNative || plan.fbx.is_some() {
            return Err(format!(
                "non-geometry {role_field} incorrectly entered FBX planning"
            ));
        }
        let Some(unreal) = plan.unreal else {
            return Err("semantic conversion plan is missing".to_owned());
        };
        if unreal.target_kind != UnrealTargetKind::SemanticSource {
            return Err(format!(
                "non-geometry {role_field} should remain semantic source"
            ));
        }
    }
    Ok(())
}

#[test]
fn classifies_exact_single_static_mesh_target() -> Result<(), String> {
    let package = row_with_exact_member(
        "ui-resources",
        "ui-resources/frontend-scenes/hud-maps/l1",
        "model_ids",
        "p3d-mesh",
        "mesh",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(fbx) = plan.fbx else {
        return Err("single static mesh should produce an FBX plan".to_owned());
    };
    if fbx.target_kind != FbxTargetKind::StaticMesh {
        return Err(format!("unexpected static target: {:?}", fbx.target_kind));
    }
    Ok(())
}

#[test]
fn multiple_static_meshes_require_semantic_split() -> Result<(), String> {
    let plan = PhaseThreePackagePlanner::plan(&row_with_two_static_meshes()?);
    let Some(fbx) = plan.fbx else {
        return Err("multi-mesh package should produce an FBX plan".to_owned());
    };
    if fbx.target_kind != FbxTargetKind::SemanticSplit {
        return Err(format!(
            "multi-mesh package bypassed semantic split: {:?}",
            fbx.target_kind
        ));
    }
    Ok(())
}

#[test]
fn classifies_exact_single_skeletal_mesh_target() -> Result<(), String> {
    let plan = PhaseThreePackagePlanner::plan(&single_skeletal_row()?);
    let Some(fbx) = plan.fbx else {
        return Err(
            "single skeletal mesh should produce an FBX plan".to_owned()
        );
    };
    if fbx.target_kind != FbxTargetKind::SkeletalMesh {
        return Err(format!(
            "unexpected skeletal target: {:?}",
            fbx.target_kind
        ));
    }
    Ok(())
}

#[test]
fn multiple_skeletons_require_semantic_split() -> Result<(), String> {
    let plan = PhaseThreePackagePlanner::plan(&row_with_duplicate_skeleton()?);
    let Some(fbx) = plan.fbx else {
        return Err(
            "multi-skeleton package should produce an FBX plan".to_owned(),
        );
    };
    if fbx.target_kind != FbxTargetKind::SemanticSplit {
        return Err(format!(
            "multi-skeleton package bypassed semantic split: {:?}",
            fbx.target_kind
        ));
    }
    Ok(())
}

#[test]
fn requires_semantic_split_for_geometry_companions() -> Result<(), String> {
    let package = row_with_model_and_companion("camera_ids")?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(fbx) = plan.fbx else {
        return Err("geometry companion should retain an FBX plan".to_owned());
    };
    if fbx.target_kind != FbxTargetKind::SemanticSplit {
        return Err(format!(
            "geometry companion bypassed split: {:?}",
            fbx.target_kind
        ));
    }
    Ok(())
}

#[test]
fn routes_model_packages_to_fbx() -> Result<(), String> {
    let package = row("cars", "cars/character-rigs/homer-v", "model_ids")?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    if plan.family != ConversionFamily::FbxModel {
        return Err("car model package should route to FBX".to_owned());
    }
    let Some(fbx) = plan.fbx else {
        return Err("fbx plan should exist".to_owned());
    };
    if fbx.model_ids != ["unit-a".to_owned()] {
        return Err("fbx plan should carry model ids".to_owned());
    }
    Ok(())
}

#[test]
fn excludes_provenance_sources_from_unreal_inputs() -> Result<(), String> {
    let json = concat!(
        "{\"package_id\":\"derived-language\",",
        "\"package_root\":\"derived/language\",",
        "\"package_category\":\"language\",",
        "\"package_subcategory\":\"language/objectives\",",
        "\"unit_count\":0,\"text_key_count\":1,",
        "\"unit_ids\":[],\"world_ids\":[],",
        "\"texture_ids\":[],\"material_ids\":[],",
        "\"model_ids\":[],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[],",
        "\"metadata_ids\":[],\"error_ids\":[],",
        "\"source_unit_ids\":[\"source-a\"],",
        "\"text_key_ids\":[\"text-a\"],",
        "\"members\":[],",
        "\"text_keys\":[{",
        "\"id\":\"text-a\",",
        "\"key\":\"HELLO\",",
        "\"source_unit_id\":\"source-a\",",
        "\"subcategory\":\"language/objectives\"}]}",
    );
    let package = PhaseThreePackageRow::from_json_line(json)
        .map_err(|error| error.to_string())?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let unreal = plan.unreal.ok_or_else(|| {
        "derived text package should produce an Unreal plan".to_owned()
    })?;
    if unreal.input_ids != ["text-a".to_owned()] {
        return Err(format!(
            "provenance sources leaked into Unreal inputs: {:?}",
            unreal.input_ids,
        ));
    }
    Ok(())
}

#[test]
fn routes_dialog_voice_to_unreal_sound_waves() -> Result<(), String> {
    let package = row(
        "dialog",
        "dialog/homer/ad-lib/free-roam/default",
        "audio_ids",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    if plan.family != ConversionFamily::UnrealNative {
        return Err("dialog should route to Unreal-native data".to_owned());
    }
    let Some(unreal) = plan.unreal else {
        return Err("unreal plan should exist".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SoundWave {
        return Err("dialog voice should target sound waves".to_owned());
    }
    Ok(())
}

#[test]
fn routes_metadata_only_packages_to_do_not_import() -> Result<(), String> {
    let package = row(
        "ui-images",
        "ui-images/source-metadata/root",
        "metadata_ids",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    if plan.family != ConversionFamily::DoNotImport {
        return Err("metadata-only package should not import".to_owned());
    }
    let Some(unreal) = plan.unreal else {
        return Err("metadata plan should exist".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::Metadata {
        return Err("metadata package should target metadata".to_owned());
    }
    Ok(())
}

#[test]
fn routes_movie_payloads_to_media_sources() -> Result<(), String> {
    let package = row("movies", "movies/story/fmv4", "movie_ids")?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("movie package should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::MediaSource {
        return Err("movie payload should target a media source".to_owned());
    }
    Ok(())
}

#[test]
fn keeps_video_less_movie_metadata_non_media() -> Result<(), String> {
    let package = row("movies", "movies/logos/gvuglogo", "text_ids")?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("movie metadata should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::Metadata {
        return Err(
            "movie metadata without video should remain traceability metadata"
                .to_owned(),
        );
    }
    Ok(())
}

#[test]
fn routes_derived_language_keys_to_string_tables() -> Result<(), String> {
    let json = concat!(
        "{\"package_id\":\"derived-language\",",
        "\"package_root\":\"derived/language\",",
        "\"package_category\":\"language\",",
        "\"package_subcategory\":\"language/text/system\",",
        "\"unit_count\":0,\"text_key_count\":1,",
        "\"unit_ids\":[],\"world_ids\":[],",
        "\"texture_ids\":[],\"material_ids\":[],",
        "\"model_ids\":[],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[],",
        "\"metadata_ids\":[],\"error_ids\":[],",
        "\"source_unit_ids\":[\"source-a\"],",
        "\"text_key_ids\":[\"text-a\"],",
        "\"members\":[],",
        "\"text_keys\":[{",
        "\"id\":\"text-a\",",
        "\"key\":\"HELLO\",",
        "\"source_unit_id\":\"source-a\",",
        "\"subcategory\":\"language/text/system\"}]}"
    );
    let package = PhaseThreePackageRow::from_json_line(json)
        .map_err(|error| error.to_string())?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err(
            "derived language package should produce an Unreal plan".to_owned()
        );
    };
    if unreal.target_kind != UnrealTargetKind::StringTable {
        return Err(
            "derived language keys should target StringTable".to_owned()
        );
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn defers_physical_language_layouts_for_semantic_compilation() -> Result<(), String> {
    let package = row("language", "language/ui-text/scene-layouts", "ui_ids")?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("language layout should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SemanticSource {
        // jig-ignore-next-line: literal
        return Err("physical language layout bypassed semantic compilation".to_owned());
    }
    Ok(())
}

#[test]
fn defers_mission_scripts_for_semantic_compilation() -> Result<(), String> {
    let package =
        row("mission-scripts", "missions/level-01/scripts", "script_ids")?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err(
            "mission script bundle should produce an Unreal plan".to_owned()
        );
    };
    if unreal.target_kind != UnrealTargetKind::SemanticSource {
        return Err(
            "mission script bundle must await semantic compilation".to_owned()
        );
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn defers_texture_font_headers_for_semantic_compilation() -> Result<(), String> {
    let package = row_with_kind(
        "ui-resources",
        "ui-resources/fonts/fonts/font0-16",
        "ui_ids",
        "p3d-texture-font",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("font resource should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SemanticSource {
        // jig-ignore-next-line: literal
        return Err("texture-font header bypassed semantic compilation".to_owned());
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn defers_ui_text_bible_headers_for_semantic_compilation() -> Result<(), String> {
    let package = row_with_kind(
        "ui-resources",
        "ui-resources/language/art-assets/sprite-layouts/txtbible-srr2",
        "text_ids",
        "p3d-text-bible",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("text bible should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SemanticSource {
        // jig-ignore-next-line: literal
        return Err("text-bible header bypassed semantic compilation".to_owned());
    }
    Ok(())
}

#[test]
fn defers_ui_layout_bundles_for_semantic_compilation() -> Result<(), String> {
    let package = row_with_kind(
        "ui-screens",
        "ui-screens/layout-index/pages",
        "ui_ids",
        "ui-layout",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("UI layout bundle should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SemanticSource {
        return Err("UI layout bundle bypassed semantic compilation".to_owned());
    }
    Ok(())
}

#[test]
fn defers_normalized_tuning_for_semantic_compilation() -> Result<(), String> {
    let package = row(
        "vehicle-tuning",
        "vehicle-tuning/level-01/chase",
        "script_ids",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("vehicle tuning should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SemanticSource {
        return Err("vehicle tuning bypassed semantic compilation".to_owned());
    }
    Ok(())
}

#[test]
fn routes_cinematic_audio_to_sound_waves() -> Result<(), String> {
    let package =
        row("cinematics", "cinematics/nis-audio/spanish", "audio_ids")?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("cinematic audio should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SoundWave {
        return Err("cinematic audio should target SoundWave".to_owned());
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn sprite_layout_json_does_not_fabricate_texture_payload() -> Result<(), String> {
    let package = row_with_texture_path(
        "ui-images",
        "p3d-sprite",
        "extracted/ui/components/sprite/icon.json",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("sprite layout should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SemanticSource {
        return Err("sprite JSON fabricated a Texture2D target".to_owned());
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn embedded_sprite_dds_remains_semantic_until_compiler_support() -> Result<(), String> {
    let package = row_with_texture_path(
        "ui-images",
        "p3d-texture",
        "extracted/ui/components/image/icon.dds",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("embedded DDS should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SemanticSource {
        return Err(
            "embedded DDS bypassed its semantic compiler gate".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn game_icon_category_without_pixels_remains_semantic() -> Result<(), String> {
    let package = row("game-icons", "game-icons", "text_ids")?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("game metadata should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::SemanticSource {
        return Err("game metadata fabricated a Texture2D target".to_owned());
    }
    Ok(())
}

#[test]
fn physical_ui_png_targets_texture() -> Result<(), String> {
    let package = row_with_texture_path(
        "ui-images",
        "png-image",
        "game/art/frontend/icon.png",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("physical PNG should produce an Unreal plan".to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::Texture {
        return Err("physical PNG did not target Texture2D".to_owned());
    }
    Ok(())
}

#[test]
fn routes_character_texture_companions_to_textures() -> Result<(), String> {
    let package = row_with_texture_path(
        "characters",
        "png-image",
        "extracted/characters/homer.png",
    )?;
    let plan = PhaseThreePackagePlanner::plan(&package);
    let Some(unreal) = plan.unreal else {
        return Err("character texture package should produce an Unreal plan"
            .to_owned());
    };
    if unreal.target_kind != UnrealTargetKind::Texture {
        return Err(
            "character texture companion should target Texture2D".to_owned()
        );
    }
    Ok(())
}
