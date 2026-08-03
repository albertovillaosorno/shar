// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
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

use super::{ConversionFamily, PhaseThreePackagePlanner, UnrealTargetKind};
use crate::domain::package::index::PhaseThreePackageRow;

fn row(
    category: &str,
    subcategory: &str,
    role_field: &str,
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
            "\"kind\":\"test\",",
            "\"source_chunk_kind\":\"test\"}}]"
        ),
        role,
    );
    json = json.replace("\"members\":[]", &member);
    PhaseThreePackageRow::from_json_line(&json)
        .map_err(|error| error.to_string())
}

#[test]
fn preserves_scene_assembly_roles_in_fbx_plans() -> Result<(), String> {
    for role_field in ["scene_ids", "locator_ids", "camera_ids"] {
        let package = row("cars", "cars/character-rigs/homer-v", role_field)?;
        let plan = PhaseThreePackagePlanner::plan(&package);
        let Some(fbx) = plan.fbx else {
            return Err(format!(
                "{role_field} package should produce an FBX plan"
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
        if retained != ["unit-a".to_owned()] {
            return Err(format!("FBX plan dropped {role_field}: {retained:?}"));
        }
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
