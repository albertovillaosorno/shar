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
//   - Unreal import-manifest domain tests.
// - Must-Not:
//   - Perform filesystem access or execute Unreal Editor.
// - Allows:
//   - Pure package-index fixtures and source evidence assertions.
// - Split-When:
//   - Split when another manifest schema needs independent fixtures.
// - Merge-When:
//   - Merge when another test module owns identical evidence.
// - Summary:
//   - Unreal import-manifest domain tests.
// - Description:
//   - Proves deterministic direct-import planning and content validation.
// - Usage:
//   - Included only by the owning domain module under cfg(test).
// - Defaults:
//   - Invalid source evidence fails closed.
//

//! Unreal import-manifest domain tests.

use shar_sha256::digest_hex;
use shar_unreal_conversion::domain::PlanFamily;

use super::{
    UnrealFbxArtifactEvidence, UnrealImportManifest, UnrealSourceEvidence,
};
use crate::domain::package::PhaseThreePackageIndex;

fn index() -> Result<PhaseThreePackageIndex, String> {
    index_with_member_path("extracted/ui/icon.png")
}

fn index_with_member_path(
    member_path: &str,
) -> Result<PhaseThreePackageIndex, String> {
    let row = concat!(
        "{\"package_id\":\"extracted-ui-icon\",",
        "\"package_root\":\"extracted/ui/icon\",",
        "\"package_category\":\"ui-images\",",
        "\"package_subcategory\":\"ui-images/icon\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"texture-a\"],\"world_ids\":[],",
        "\"texture_ids\":[\"texture-a\"],\"material_ids\":[],",
        "\"model_ids\":[],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[],",
        "\"metadata_ids\":[],\"error_ids\":[],",
        "\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[{\"id\":\"texture-a\",",
        "\"role\":\"texture\",",
        "\"path\":\"extracted/ui/icon.png\",",
        "\"type\":\"texture\",\"kind\":\"runtime-asset\",",
        "\"source_chunk_kind\":\"image\"}],",
        "\"text_keys\":[]}",
    );
    let row = row.replace("extracted/ui/icon.png", member_path);
    PhaseThreePackageIndex::from_jsonl(&format!("{row}\n"))
        .map_err(|error| error.to_string())
}

fn model_index() -> Result<PhaseThreePackageIndex, String> {
    let row = concat!(
        "{\"package_id\":\"extracted-art-cars-model\",",
        "\"package_root\":\"extracted/art/cars/model\",",
        "\"package_category\":\"cars\",",
        "\"package_subcategory\":\"cars/traffic/model\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"model-a\"],\"world_ids\":[],",
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
        "\"members\":[{\"id\":\"model-a\",",
        "\"role\":\"model\",",
        "\"path\":\"extracted/art/cars/model/model.json\",",
        "\"type\":\"model\",\"kind\":\"p3d-mesh\",",
        "\"source_chunk_kind\":\"mesh\"}],",
        "\"text_keys\":[]}",
    );
    PhaseThreePackageIndex::from_jsonl(&format!("{row}\n"))
        .map_err(|error| error.to_string())
}

fn skeletal_model_index() -> Result<PhaseThreePackageIndex, String> {
    let row = concat!(
        "{\"package_id\":\"pkg\",\"package_root\":\"pkg\",",
        "\"package_category\":\"characters\",",
        "\"package_subcategory\":\"characters/test/base-model\",",
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
        "\"path\":\"extracted/skeleton.json\",",
        "\"type\":\"animation\",\"kind\":\"p3d-skeleton\",",
        "\"source_chunk_kind\":\"skeleton\"}],\"text_keys\":[]}",
    );
    PhaseThreePackageIndex::from_jsonl(&format!("{row}\n"))
        .map_err(|error| error.to_string())
}

fn skeletal_model_evidence() -> Vec<UnrealSourceEvidence> {
    [
        (
            "composite-a",
            "extracted/composite.json",
            "model",
            "composite_drawable",
            "p3d-composite-drawable",
        ),
        ("skin-a", "extracted/skin.json", "model", "skin", "p3d-skin"),
        (
            "skeleton-a",
            "extracted/skeleton.json",
            "animation",
            "skeleton",
            "p3d-skeleton",
        ),
    ]
    .into_iter()
    .map(|(id, path, unit_type, subtype, kind)| UnrealSourceEvidence {
        id: id.to_owned(),
        path: path.to_owned(),
        file_extension: "json".to_owned(),
        unit_type: unit_type.to_owned(),
        subtype: subtype.to_owned(),
        kind: kind.to_owned(),
        function: "skeletal model evidence".to_owned(),
        schema: subtype.to_owned(),
        origin: "p3d-package".to_owned(),
        source_path: "extracted/character/model.p3d".to_owned(),
        source_chunk_kind: subtype.to_owned(),
        size_bytes: 4,
        sha256: "9".repeat(64),
        unreal_import_relation: "import-after-conversion".to_owned(),
        future_normalization: "model-to-fbx".to_owned(),
    })
    .collect()
}

fn verified_skeletal_fbx() -> UnrealFbxArtifactEvidence {
    UnrealFbxArtifactEvidence {
        package_id: "pkg".to_owned(),
        path: "fbx-assets/packages/pkg/pkg.fbx".to_owned(),
        size_bytes: 31,
        sha256: "8".repeat(64),
        fbx_version: 7700,
    }
}

fn model_evidence() -> UnrealSourceEvidence {
    UnrealSourceEvidence {
        id: "model-a".to_owned(),
        path: "extracted/art/cars/model/model.json".to_owned(),
        file_extension: "json".to_owned(),
        unit_type: "model".to_owned(),
        subtype: "mesh".to_owned(),
        kind: "p3d-mesh".to_owned(),
        function: "model evidence".to_owned(),
        schema: "model-v1".to_owned(),
        origin: "p3d-package".to_owned(),
        source_path: "extracted/art/cars/model/model.p3d".to_owned(),
        source_chunk_kind: "mesh".to_owned(),
        size_bytes: 4,
        sha256: "b".repeat(64),
        unreal_import_relation: "import-after-conversion".to_owned(),
        future_normalization: "model-to-fbx".to_owned(),
    }
}

fn composite_model_index() -> Result<PhaseThreePackageIndex, String> {
    let row = concat!(
        "{\"package_id\":\"extracted-art-ui-model\",",
        "\"package_root\":\"extracted/art/ui/model\",",
        "\"package_category\":\"ui-resources\",",
        "\"package_subcategory\":\"ui-resources/frontend-scenes/model\",",
        "\"unit_count\":2,\"text_key_count\":0,",
        "\"unit_ids\":[\"model-a\",\"camera-a\"],\"world_ids\":[],",
        "\"texture_ids\":[],\"material_ids\":[],",
        "\"model_ids\":[\"model-a\"],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[\"camera-a\"],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[],",
        "\"metadata_ids\":[],\"error_ids\":[],",
        "\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[",
        "{\"id\":\"model-a\",\"role\":\"model\",",
        "\"path\":\"extracted/art/ui/model/model.json\",",
        "\"type\":\"model\",\"kind\":\"p3d-mesh\",",
        "\"source_chunk_kind\":\"mesh\"},",
        "{\"id\":\"camera-a\",\"role\":\"camera\",",
        "\"path\":\"extracted/art/ui/model/camera.json\",",
        "\"type\":\"camera\",\"kind\":\"p3d-camera\",",
        "\"source_chunk_kind\":\"camera\"}],\"text_keys\":[]}",
    );
    PhaseThreePackageIndex::from_jsonl(&format!("{row}\n"))
        .map_err(|error| error.to_string())
}

fn composite_model_evidence() -> Vec<UnrealSourceEvidence> {
    vec![
        UnrealSourceEvidence {
            id: "model-a".to_owned(),
            path: "extracted/art/ui/model/model.json".to_owned(),
            file_extension: "json".to_owned(),
            unit_type: "model".to_owned(),
            subtype: "mesh".to_owned(),
            kind: "p3d-mesh".to_owned(),
            function: "model evidence".to_owned(),
            schema: "mesh".to_owned(),
            origin: "p3d-package".to_owned(),
            source_path: "extracted/art/ui/model/model.p3d".to_owned(),
            source_chunk_kind: "mesh".to_owned(),
            size_bytes: 4,
            sha256: "b".repeat(64),
            unreal_import_relation: "import-after-conversion".to_owned(),
            future_normalization: "model-to-fbx".to_owned(),
        },
        UnrealSourceEvidence {
            id: "camera-a".to_owned(),
            path: "extracted/art/ui/model/camera.json".to_owned(),
            file_extension: "json".to_owned(),
            unit_type: "camera".to_owned(),
            subtype: "camera".to_owned(),
            kind: "p3d-camera".to_owned(),
            function: "camera evidence".to_owned(),
            schema: "camera".to_owned(),
            origin: "p3d-package".to_owned(),
            source_path: "extracted/art/ui/model/model.p3d".to_owned(),
            source_chunk_kind: "camera".to_owned(),
            size_bytes: 4,
            sha256: "d".repeat(64),
            unreal_import_relation: "semantic-companion".to_owned(),
            future_normalization: "camera-native".to_owned(),
        },
    ]
}

fn verified_model_fbx() -> UnrealFbxArtifactEvidence {
    UnrealFbxArtifactEvidence {
        package_id: "extracted-art-cars-model".to_owned(),
        path: concat!(
            "fbx-assets/packages/extracted_art_cars_model/",
            "extracted_art_cars_model.fbx"
        )
        .to_owned(),
        size_bytes: 27,
        sha256: "c".repeat(64),
        fbx_version: 7700,
    }
}

fn evidence() -> UnrealSourceEvidence {
    UnrealSourceEvidence {
        id: "texture-a".to_owned(),
        path: "extracted/ui/icon.png".to_owned(),
        file_extension: "png".to_owned(),
        unit_type: "texture".to_owned(),
        subtype: "png-texture".to_owned(),
        kind: "runtime-asset".to_owned(),
        function: "UI texture".to_owned(),
        schema: "png".to_owned(),
        origin: "p3d-package".to_owned(),
        source_path: "extracted/ui/icon.png".to_owned(),
        source_chunk_kind: "image".to_owned(),
        size_bytes: 4,
        sha256: "a".repeat(64),
        unreal_import_relation: "import-after-conversion".to_owned(),
        future_normalization: "png-to-texture2d".to_owned(),
    }
}

#[test]
fn builds_deterministic_direct_texture_import() -> Result<(), String> {
    let manifest = UnrealImportManifest::build(&index()?, vec![evidence()])?;
    let first = manifest.to_jsonl();
    let second = manifest.to_jsonl();
    if first != second {
        return Err("manifest serialization is not deterministic".to_owned());
    }
    for expected in [
        "shar-schoenwald.unreal-import-manifest.v2",
        "\"direct_import_count\":1",
        "\"target_class\":\"Texture2D\"",
        "/Game/Generated/SHAR/ui_images/extracted_ui_icon/texture_a.texture_a",
    ] {
        if !first.contains(expected) {
            return Err(format!("manifest is missing {expected}"));
        }
    }
    if manifest.package_count() != 1 || manifest.source_count() != 1 {
        return Err("manifest counts do not match the fixture".to_owned());
    }
    Ok(())
}

#[test]
fn emits_complete_deterministic_plan_bundle() -> Result<(), String> {
    let manifest = UnrealImportManifest::build(&index()?, vec![evidence()])?;
    let manifest_jsonl = manifest.to_jsonl();
    let revision = digest_hex(manifest_jsonl.as_bytes());
    let first = manifest.plan_bundle(&revision)?;
    let second = manifest.plan_bundle(&revision)?;
    if first != second {
        return Err("plan bundle is not deterministic".to_owned());
    }
    if first.artifacts().len() != PlanFamily::all().len() {
        return Err("plan bundle is missing canonical families".to_owned());
    }
    let import = first
        .artifacts()
        .iter()
        .find(|artifact| artifact.family == PlanFamily::AssetImport)
        .ok_or_else(|| "asset import plan is missing".to_owned())?;
    if import.operation_count != 1
        || !import.json.contains(r#""source_format":"image""#)
        || !import.json.contains(r#""readiness":"ready""#)
    {
        return Err("direct texture evidence did not become one ready import"
            .to_owned());
    }
    for family in PlanFamily::all() {
        if !first
            .artifacts()
            .iter()
            .any(|artifact| artifact.family == family)
        {
            return Err(format!("missing plan family: {}", family.plan_id()));
        }
    }
    Ok(())
}

#[test]
fn composite_geometry_reserves_no_false_static_mesh() -> Result<(), String> {
    let manifest = UnrealImportManifest::build(
        &composite_model_index()?,
        composite_model_evidence(),
    )?;
    let json = manifest.to_jsonl();
    let summary = manifest.summary_json();
    for expected in [
        "\"disposition\":\"requires-semantic-conversion\"",
        "\"target_kind\":\"CompositeModel\"",
        "\"expected_staged_files\":[]",
        "\"expected_unreal_objects\":[]",
    ] {
        if !json.contains(expected) {
            return Err(format!("composite model manifest lost: {expected}"));
        }
    }
    if !summary.contains("\"requires_fbx\":0")
        || !summary.contains("\"requires_semantic_conversion\":1")
    {
        return Err(format!("composite summary is wrong: {summary}"));
    }
    Ok(())
}

#[test]
fn complete_fbx_catalog_promotes_model_import_to_ready() -> Result<(), String> {
    let manifest =
        UnrealImportManifest::build(&model_index()?, vec![model_evidence()])?;
    let revision = digest_hex(manifest.to_jsonl().as_bytes());
    let pending = manifest.plan_bundle(&revision)?;
    let verified = manifest
        .plan_bundle_with_complete_fbx_catalog(&revision, &[
            verified_model_fbx(),
        ])?;
    let pending_json = &pending
        .artifacts()
        .iter()
        .find(|artifact| artifact.family == PlanFamily::AssetImport)
        .ok_or_else(|| "pending model import plan is missing".to_owned())?
        .json;
    let verified_json = &verified
        .artifacts()
        .iter()
        .find(|artifact| artifact.family == PlanFamily::AssetImport)
        .ok_or_else(|| "verified model import plan is missing".to_owned())?
        .json;
    if !pending_json.contains(r#""readiness":"requires-conversion""#)
        || !verified_json.contains(r#""readiness":"ready""#)
        || !verified_json
            .contains(&format!(r#""source_revision":"{}""#, "c".repeat(64)))
        || !verified_json.contains(r#""target_engine_version":"5.8.1""#)
    {
        return Err(
            "verified FBX evidence did not promote the model plan".to_owned()
        );
    }
    Ok(())
}


#[test]
fn complete_fbx_catalog_promotes_skeletal_import_with_companion_reservation()
-> Result<(), String> {
    let manifest = UnrealImportManifest::build(
        &skeletal_model_index()?,
        skeletal_model_evidence(),
    )?;
    let manifest_json = manifest.to_jsonl();
    for expected in [
        "\"disposition\":\"requires-fbx\"",
        "\"target_kind\":\"SkeletalMesh\"",
        "\"import_profile\":\"shar-fbx-skeletal-v1\"",
        "/Game/Generated/SHAR/characters/pkg/pkg.pkg",
        "/Game/Generated/SHAR/characters/pkg/pkg_Skeleton.pkg_Skeleton",
    ] {
        if !manifest_json.contains(expected) {
            return Err(format!("skeletal manifest lost contract: {expected}"));
        }
    }
    let revision = digest_hex(manifest_json.as_bytes());
    let pending = manifest.plan_bundle(&revision)?;
    let verified = manifest.plan_bundle_with_complete_fbx_catalog(
        &revision,
        &[verified_skeletal_fbx()],
    )?;
    let import_json = |bundle: &shar_unreal_conversion::domain::PlanBundle| {
        bundle
            .artifacts()
            .iter()
            .find(|artifact| artifact.family == PlanFamily::AssetImport)
            .map(|artifact| artifact.json.clone())
            .ok_or_else(|| "skeletal import plan is missing".to_owned())
    };
    let pending_json = import_json(&pending)?;
    let verified_json = import_json(&verified)?;
    if !pending_json.contains(r#""readiness":"requires-conversion""#)
        || !verified_json.contains(r#""readiness":"ready""#)
        || !verified_json.contains(r#""target_class":"SkeletalMesh""#)
        || !verified_json.contains(r#""import_profile":"shar-fbx-skeletal-v1""#)
        || !verified_json.contains(&format!(
            r#""source_revision":"{}""#,
            "8".repeat(64)
        ))
    {
        return Err(
            "verified skeletal FBX did not promote companion-aware import"
                .to_owned(),
        );
    }
    Ok(())
}


#[test]
fn complete_fbx_catalog_rejects_missing_and_unclaimed_packages()
-> Result<(), String> {
    let manifest =
        UnrealImportManifest::build(&model_index()?, vec![model_evidence()])?;
    let revision = digest_hex(manifest.to_jsonl().as_bytes());
    if manifest
        .plan_bundle_with_complete_fbx_catalog(&revision, &[])
        .is_ok()
    {
        return Err("partial FBX catalog was accepted".to_owned());
    }
    let mut extra = verified_model_fbx();
    extra.package_id = "unclaimed-model".to_owned();
    if manifest
        .plan_bundle_with_complete_fbx_catalog(&revision, &[
            verified_model_fbx(),
            extra,
        ])
        .is_ok()
    {
        return Err("unclaimed FBX catalog package was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn complete_fbx_catalog_rejects_stale_artifact_contract() -> Result<(), String>
{
    let manifest =
        UnrealImportManifest::build(&model_index()?, vec![model_evidence()])?;
    let revision = digest_hex(manifest.to_jsonl().as_bytes());
    for mutate in ["path", "digest", "version", "size"] {
        let mut artifact = verified_model_fbx();
        match mutate {
            "path" => {
                artifact.path =
                    "fbx-assets/packages/other/other.fbx".to_owned();
            },
            "digest" => artifact.sha256 = "C".repeat(64),
            "version" => artifact.fbx_version = 7400,
            "size" => artifact.size_bytes = 26,
            _ => return Err("unknown FBX mutation".to_owned()),
        }
        if manifest
            .plan_bundle_with_complete_fbx_catalog(&revision, &[artifact])
            .is_ok()
        {
            return Err(format!("stale FBX {mutate} was accepted"));
        }
    }
    Ok(())
}

#[test]
fn rejects_uppercase_source_hash() -> Result<(), String> {
    let mut source = evidence();
    source.sha256 = "A".repeat(64);
    let result = UnrealImportManifest::build(&index()?, vec![source]);
    let Err(error) = result else {
        return Err("uppercase hashes must fail".to_owned());
    };
    if !error.contains("invalid SHA-256") {
        return Err(format!("unexpected hash failure: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_invalid_source_hash() -> Result<(), String> {
    let mut source = evidence();
    source.sha256 = "not-a-digest".to_owned();
    let result = UnrealImportManifest::build(&index()?, vec![source]);
    let Err(error) = result else {
        return Err("invalid hashes must fail".to_owned());
    };
    if !error.contains("invalid SHA-256") {
        return Err(format!("unexpected hash failure: {error}"));
    }
    Ok(())
}

#[test]
fn direct_policy_without_compatible_source_requires_factory()
-> Result<(), String> {
    let mut policy = super::native_policy(Some(
        crate::domain::package::UnrealTargetKind::Texture,
    ));
    super::resolve_effective_policy(
        crate::domain::package::ConversionFamily::UnrealNative,
        false,
        &mut policy,
    );
    if policy.disposition != "requires-editor-factory" {
        return Err(format!(
            "unexpected fallback disposition: {}",
            policy.disposition
        ));
    }
    if policy.reason.is_none() {
        return Err(
            "factory fallback must explain why direct import failed".to_owned()
        );
    }
    Ok(())
}

#[test]
fn skeletal_fbx_policy_uses_companion_aware_fbx_transaction()
-> Result<(), String> {
    let policy =
        super::fbx_policy(crate::domain::package::FbxTargetKind::SkeletalMesh);
    if policy.disposition != "requires-fbx"
        || policy.target_kind != "SkeletalMesh"
        || policy.importer != "asset-tools-fbx"
        || policy.import_profile != "shar-fbx-skeletal-v1"
    {
        return Err(
            "skeletal FBX did not use companion-aware import".to_owned()
        );
    }
    Ok(())
}

#[test]
fn semantic_source_policy_requires_upstream_compilation() -> Result<(), String>
{
    let policy = super::native_policy(Some(
        crate::domain::package::UnrealTargetKind::SemanticSource,
    ));
    if policy.disposition != "requires-semantic-conversion"
        || policy.target_kind != "SemanticSource"
        || policy.importer != "semantic-converter"
        || policy.reason.is_none()
    {
        return Err("unexpected semantic-source policy".to_owned());
    }
    Ok(())
}

#[test]
fn semantic_source_reserves_no_unreal_object() -> Result<(), String> {
    use std::collections::BTreeSet;

    let policy = super::native_policy(Some(
        crate::domain::package::UnrealTargetKind::SemanticSource,
    ));
    let mut staged_files = Vec::new();
    let mut unreal_objects = Vec::new();
    let mut staged_paths = BTreeSet::new();
    let mut object_paths = BTreeSet::new();
    let mut summary = super::UnrealImportSummary::default();
    super::add_package_outputs(
        crate::domain::package::ConversionFamily::UnrealNative,
        policy.disposition,
        policy.target_kind,
        "package",
        "/Game/Generated/SHAR/missions/package",
        &mut staged_files,
        &mut unreal_objects,
        &mut staged_paths,
        &mut object_paths,
        &mut summary,
    )?;
    if !staged_files.is_empty()
        || !unreal_objects.is_empty()
        || summary.requires_semantic_conversion != 1
        || summary.requires_editor_factory != 0
    {
        return Err(
            "semantic source incorrectly reserved an Unreal asset".to_owned()
        );
    }
    Ok(())
}

#[test]
fn factory_policy_keeps_primary_object_with_direct_companion()
-> Result<(), String> {
    use std::collections::BTreeSet;

    let policy = super::native_policy(Some(
        crate::domain::package::UnrealTargetKind::UserInterface,
    ));
    let package_path = "/Game/Generated/SHAR/ui/package";
    let companion = format!("{package_path}/icon.icon");
    let mut staged_files = Vec::new();
    let mut unreal_objects = vec![companion.clone()];
    let mut staged_paths = BTreeSet::new();
    let mut object_paths = BTreeSet::from([companion.to_ascii_lowercase()]);
    let mut summary = super::UnrealImportSummary::default();
    super::add_package_outputs(
        crate::domain::package::ConversionFamily::UnrealNative,
        policy.disposition,
        policy.target_kind,
        "package",
        package_path,
        &mut staged_files,
        &mut unreal_objects,
        &mut staged_paths,
        &mut object_paths,
        &mut summary,
    )?;
    if unreal_objects.len() != 2 || summary.requires_editor_factory != 1 {
        return Err(format!(
            concat!(
                "factory object was suppressed by direct companion: ",
                "objects={} factories={}"
            ),
            unreal_objects.len(),
            summary.requires_editor_factory
        ));
    }
    Ok(())
}

#[test]
fn rejects_source_extension_that_disagrees_with_path() -> Result<(), String> {
    let mut source = evidence();
    source.file_extension = "json".to_owned();
    let result = UnrealImportManifest::build(&index()?, vec![source]);
    let Err(error) = result else {
        return Err("source extension mismatch must fail".to_owned());
    };
    if !error.contains("extension disagrees") {
        return Err(format!("unexpected extension failure: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_nonportable_source_and_provenance_paths() -> Result<(), String> {
    for unsafe_path in [
        "/private/icon.png",
        "C:/private/icon.png",
        "extracted/../private/icon.png",
        r"extracted\private\icon.png",
    ] {
        let mut source = evidence();
        source.path = unsafe_path.to_owned();
        let result = UnrealImportManifest::build(&index()?, vec![source]);
        let Err(error) = result else {
            return Err(format!(
                "unsafe source path was accepted: {unsafe_path}"
            ));
        };
        if error != "unsafe Unreal source path" || error.contains(unsafe_path) {
            return Err(format!("unexpected source-path failure: {error}"));
        }
    }

    for unsafe_path in [
        "/private/source.p3d",
        "D:/private/source.p3d",
        "source/../../private/source.p3d",
        r"source\private\source.p3d",
    ] {
        let mut source = evidence();
        source.source_path = unsafe_path.to_owned();
        let result = UnrealImportManifest::build(&index()?, vec![source]);
        let Err(error) = result else {
            return Err(format!(
                "unsafe provenance path was accepted: {unsafe_path}"
            ));
        };
        if error != "unsafe Unreal source provenance path"
            || error.contains(unsafe_path)
        {
            return Err(format!("unexpected provenance-path failure: {error}"));
        }
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_source_identity_without_echoing_it()
-> Result<(), String> {
    let private_id = "C:/private/source";
    let mut source = evidence();
    source.id = private_id.to_owned();
    let Err(error) = UnrealImportManifest::build(&index()?, vec![source])
    else {
        return Err("path-shaped source identity was accepted".to_owned());
    };
    if error.contains(private_id)
        || error != "Unreal source evidence id is not canonical"
    {
        return Err(format!("source identity diagnostic leaked: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_source_extension() -> Result<(), String> {
    for extension in ["PNG", "p.ng", "png/extra", ""] {
        let mut source = evidence();
        source.file_extension = extension.to_owned();
        let Err(error) = UnrealImportManifest::build(&index()?, vec![source])
        else {
            return Err(format!(
                "noncanonical source extension was accepted: {extension:?}"
            ));
        };
        if error != "Unreal source evidence extension is not canonical" {
            return Err(format!("unexpected extension failure: {error}"));
        }
    }
    Ok(())
}

#[test]
fn rejects_uppercase_path_extension_even_when_evidence_matches()
-> Result<(), String> {
    let mut source = evidence();
    source.path = "extracted/ui/icon.PNG".to_owned();
    source.file_extension = "png".to_owned();
    let uppercase_index = index_with_member_path("extracted/ui/icon.PNG")?;
    let Err(error) =
        UnrealImportManifest::build(&uppercase_index, vec![source])
    else {
        return Err("uppercase source path extension was accepted".to_owned());
    };
    if !error.contains("extension disagrees") {
        return Err(format!("unexpected path-extension failure: {error}"));
    }
    Ok(())
}

#[test]
fn object_path_validation_covers_asset_package_segment() -> Result<(), String> {
    let invalid = "/Game/Generated/SHAR/ui/bad name/bad name.bad name";
    let Err(error) = super::validate_unreal_object_path(invalid) else {
        return Err("invalid asset package segment was accepted".to_owned());
    };
    if error.contains(invalid) || error != "unsafe Unreal package segment" {
        return Err(format!("object-path diagnostic leaked: {error}"));
    }
    Ok(())
}

#[test]
fn object_path_validation_does_not_echo_rejected_paths() -> Result<(), String> {
    for (invalid, expected) in [
        ("C:/private/object.asset", "unsafe Unreal package path"),
        (
            "/Game/Generated/SHAR/ui/good.bad",
            "Unreal object name does not match package",
        ),
    ] {
        let Err(error) = super::validate_unreal_object_path(invalid) else {
            return Err(format!("invalid object path was accepted: {invalid}"));
        };
        if error.contains(invalid) || error != expected {
            return Err(format!("object-path diagnostic leaked: {error}"));
        }
    }
    Ok(())
}

#[test]
fn collision_diagnostic_does_not_echo_rejected_path() -> Result<(), String> {
    use std::collections::BTreeSet;

    let private_path = "C:/private/collision";
    let mut paths = BTreeSet::from([private_path.to_ascii_lowercase()]);
    let Err(error) =
        super::claim_path(&mut paths, private_path, "Unreal object")
    else {
        return Err("case-insensitive collision was accepted".to_owned());
    };
    if error.contains(private_path)
        || error != "case-insensitive Unreal object collision"
    {
        return Err(format!("collision diagnostic leaked: {error}"));
    }
    Ok(())
}

#[test]
fn skeletal_fbx_reserves_primary_and_skeleton_outputs() -> Result<(), String> {
    use std::collections::BTreeSet;

    let policy =
        super::fbx_policy(crate::domain::package::FbxTargetKind::SkeletalMesh);
    let mut staged_files = Vec::new();
    let mut unreal_objects = Vec::new();
    let mut staged_paths = BTreeSet::new();
    let mut object_paths = BTreeSet::new();
    let mut summary = super::UnrealImportSummary::default();
    super::add_package_outputs(
        crate::domain::package::ConversionFamily::FbxModel,
        policy.disposition,
        policy.target_kind,
        "character_package",
        "/Game/Generated/SHAR/characters/character_package",
        &mut staged_files,
        &mut unreal_objects,
        &mut staged_paths,
        &mut object_paths,
        &mut summary,
    )?;
    let expected = vec![
                // jig-ignore-next-line: literal
                "/Game/Generated/SHAR/characters/character_package/character_package.character_package".to_owned(),
                // jig-ignore-next-line: literal
                "/Game/Generated/SHAR/characters/character_package/character_package_Skeleton.character_package_Skeleton".to_owned(),
    ];
    if unreal_objects != expected {
        // jig-ignore-next-line: literal
        return Err(format!("unexpected skeletal object inventory: {unreal_objects:?}"));
    }
    if staged_files
        != vec!["fbx-assets/packages/character_package/character_package.fbx"]
        || summary.requires_fbx != 1
        || summary.requires_semantic_conversion != 0
    {
        // jig-ignore-next-line: literal
        return Err("skeletal FBX outputs were not reserved atomically".to_owned());
    }
    Ok(())
}
