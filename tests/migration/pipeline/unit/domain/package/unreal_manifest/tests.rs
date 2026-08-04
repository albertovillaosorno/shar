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

use super::{UnrealImportManifest, UnrealSourceEvidence};
use crate::domain::package::PhaseThreePackageIndex;

fn index() -> Result<PhaseThreePackageIndex, String> {
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
    PhaseThreePackageIndex::from_jsonl(&format!("{row}\n"))
        .map_err(|error| error.to_string())
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
        "shar-schoenwald.unreal-import-manifest.v1",
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
