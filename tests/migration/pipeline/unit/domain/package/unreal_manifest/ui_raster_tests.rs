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
//   - UI sprite raster promotion tests for the Unreal import manifest.
// - Must-Not:
//   - Perform filesystem access or accept partial semantic package coverage.
// - Allows:
//   - Pure package-index fixtures and generated raster evidence.
// - Split-When:
//   - Another generated UI artifact family gains independent planning rules.
// - Merge-When:
//   - The parent manifest test module owns the same promotion boundary.
// - Summary:
//   - Pins fail-closed UI raster promotion into canonical Unreal plans.
// - Description:
//   - Separates generated raster fixtures from older FBX manifest fixtures.
// - Usage:
//   - Included only by the Unreal import-manifest test module.
// - Defaults:
//   - Bookkeeping metadata is allowed; additional semantics remain blocked.
//

//! UI raster promotion tests for the Unreal import manifest.

use shar_sha256::digest_hex;
use shar_unreal_conversion::domain::PlanFamily;

use super::super::{
    UnrealImportManifest, UnrealSourceEvidence, UnrealUiRasterArtifactEvidence,
};
use crate::domain::package::PhaseThreePackageIndex;

fn ui_sprite_index() -> Result<PhaseThreePackageIndex, String> {
    let row = concat!(
        "{\"package_id\":\"extracted-ui-sprite\",",
        "\"package_root\":\"extracted/ui/sprite\",",
        "\"package_category\":\"ui-images\",",
        "\"package_subcategory\":\"ui-images/sprite\",",
        "\"unit_count\":3,\"text_key_count\":0,",
        "\"unit_ids\":[\"image-a\",\"sprite-a\",\"manifest-a\"],",
        "\"world_ids\":[],\"texture_ids\":[\"image-a\"],",
        "\"material_ids\":[],\"model_ids\":[],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[\"sprite-a\"],",
        "\"metadata_ids\":[\"manifest-a\"],\"error_ids\":[],",
        "\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[",
        "{\"id\":\"image-a\",\"role\":\"texture\",",
        "\"path\":\"extracted/ui/sprite/components/image/tile.dds\",",
        "\"type\":\"texture\",\"kind\":\"p3d-image\",",
        "\"source_chunk_kind\":\"image\"},",
        "{\"id\":\"sprite-a\",\"role\":\"ui\",",
        "\"path\":\"extracted/ui/sprite/components/sprite/main.json\",",
        "\"type\":\"ui\",\"kind\":\"p3d-sprite\",",
        "\"source_chunk_kind\":\"sprite\"},",
        "{\"id\":\"manifest-a\",\"role\":\"metadata\",",
        "\"path\":\"extracted/ui/sprite/components.jsonl\",",
        "\"type\":\"metadata\",\"kind\":\"package-manifest\",",
        "\"source_chunk_kind\":\"none\"}],\"text_keys\":[]}",
    );
    PhaseThreePackageIndex::from_jsonl(&format!("{row}\n"))
        .map_err(|error| error.to_string())
}

fn ui_sprite_with_extra_index() -> Result<PhaseThreePackageIndex, String> {
    let row = concat!(
        "{\"package_id\":\"extracted-ui-sprite-extra\",",
        "\"package_root\":\"extracted/ui/sprite/extra\",",
        "\"package_category\":\"ui-images\",",
        "\"package_subcategory\":\"ui-images/sprite\",",
        "\"unit_count\":4,\"text_key_count\":0,",
        "\"unit_ids\":[\"image-a\",\"sprite-a\",",
        "\"manifest-a\",\"metadata-a\"],",
        "\"world_ids\":[],\"texture_ids\":[\"image-a\"],",
        "\"material_ids\":[],\"model_ids\":[],\"physics_ids\":[],",
        "\"animation_ids\":[],\"scene_ids\":[],",
        "\"locator_ids\":[],\"camera_ids\":[],",
        "\"light_ids\":[],\"particle_ids\":[],",
        "\"controller_ids\":[],\"audio_ids\":[],",
        "\"movie_ids\":[],\"script_ids\":[],",
        "\"text_ids\":[],\"ui_ids\":[\"sprite-a\"],",
        "\"metadata_ids\":[\"manifest-a\",\"metadata-a\"],",
        "\"error_ids\":[],\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[",
        "{\"id\":\"image-a\",\"role\":\"texture\",",
        "\"path\":\"extracted/ui/sprite/extra/components/image/tile.dds\",",
        "\"type\":\"texture\",\"kind\":\"p3d-image\",",
        "\"source_chunk_kind\":\"image\"},",
        "{\"id\":\"sprite-a\",\"role\":\"ui\",",
        "\"path\":\"extracted/ui/sprite/extra/components/sprite/main.json\",",
        "\"type\":\"ui\",\"kind\":\"p3d-sprite\",",
        "\"source_chunk_kind\":\"sprite\"},",
        "{\"id\":\"manifest-a\",\"role\":\"metadata\",",
        "\"path\":\"extracted/ui/sprite/extra/components.jsonl\",",
        "\"type\":\"metadata\",\"kind\":\"package-manifest\",",
        "\"source_chunk_kind\":\"none\"},",
        "{\"id\":\"metadata-a\",\"role\":\"metadata\",",
        "\"path\":\"extracted/ui/sprite/extra/components/meta/extra.json\",",
        "\"type\":\"metadata\",\"kind\":\"p3d-metadata\",",
        "\"source_chunk_kind\":\"metadata\"}],\"text_keys\":[]}",
    );
    PhaseThreePackageIndex::from_jsonl(&format!("{row}\n"))
        .map_err(|error| error.to_string())
}

fn ui_sprite_evidence() -> Vec<UnrealSourceEvidence> {
    let mut evidence = [
        (
            "sprite-a",
            "extracted/ui/sprite/components/sprite/main.json",
            "json",
            "ui",
            "sprite",
            "p3d-sprite",
        ),
        (
            "image-a",
            "extracted/ui/sprite/components/image/tile.dds",
            "dds",
            "texture",
            "image",
            "p3d-image",
        ),
    ]
    .into_iter()
    .map(|parts| {
        let (id, path, extension, unit_type, subtype, kind) = parts;
        UnrealSourceEvidence {
            id: id.to_owned(),
            path: path.to_owned(),
            file_extension: extension.to_owned(),
            unit_type: unit_type.to_owned(),
            subtype: subtype.to_owned(),
            kind: kind.to_owned(),
            function: "normalized UI sprite evidence".to_owned(),
            schema: subtype.to_owned(),
            origin: "p3d-package".to_owned(),
            source_path: "extracted/ui/sprite/source.p3d".to_owned(),
            source_chunk_kind: subtype.to_owned(),
            size_bytes: 16,
            sha256: if id == "sprite-a" { "1" } else { "2" }.repeat(64),
            unreal_import_relation: "import-after-conversion".to_owned(),
            future_normalization: "sprite-to-raster".to_owned(),
        }
    })
    .collect::<Vec<_>>();
    evidence.push(UnrealSourceEvidence {
        id: "manifest-a".to_owned(),
        path: "extracted/ui/sprite/components.jsonl".to_owned(),
        file_extension: "jsonl".to_owned(),
        unit_type: "metadata".to_owned(),
        subtype: "jsonl".to_owned(),
        kind: "package-manifest".to_owned(),
        function: "package component manifest".to_owned(),
        schema: "p3d.components-jsonl".to_owned(),
        origin: "p3d-package".to_owned(),
        source_path: "extracted/ui/sprite/components.jsonl".to_owned(),
        source_chunk_kind: "none".to_owned(),
        size_bytes: 16,
        sha256: "4".repeat(64),
        unreal_import_relation: "editor-only-metadata".to_owned(),
        future_normalization: "keep".to_owned(),
    });
    evidence
}

fn ui_sprite_with_extra_evidence() -> Vec<UnrealSourceEvidence> {
    let mut evidence = ui_sprite_evidence();
    for source in &mut evidence {
        source.path = source
            .path
            .replace("extracted/ui/sprite/", "extracted/ui/sprite/extra/");
        source.source_path = "extracted/ui/sprite/extra/source.p3d".to_owned();
    }
    evidence.push(UnrealSourceEvidence {
        id: "metadata-a".to_owned(),
        path: "extracted/ui/sprite/extra/components/meta/extra.json".to_owned(),
        file_extension: "json".to_owned(),
        unit_type: "metadata".to_owned(),
        subtype: "metadata".to_owned(),
        kind: "p3d-metadata".to_owned(),
        function: "additional semantic evidence".to_owned(),
        schema: "metadata".to_owned(),
        origin: "p3d-package".to_owned(),
        source_path: "extracted/ui/sprite/extra/source.p3d".to_owned(),
        source_chunk_kind: "metadata".to_owned(),
        size_bytes: 16,
        sha256: "3".repeat(64),
        unreal_import_relation: "semantic-companion".to_owned(),
        future_normalization: "metadata-native".to_owned(),
    });
    evidence
}

fn verified_ui_raster() -> UnrealUiRasterArtifactEvidence {
    UnrealUiRasterArtifactEvidence {
        package_id: "extracted-ui-sprite".to_owned(),
        path: "ui-raster-assets/rasters/extracted-ui-sprite.png".to_owned(),
        size_bytes: 64,
        sha256: "e".repeat(64),
        source_revision: "f".repeat(64),
        width: 4,
        height: 4,
        tile_count: 1,
    }
}

#[test]
fn complete_ui_raster_catalog_promotes_sprite_and_clears_blocker()
-> Result<(), String> {
    let manifest =
        UnrealImportManifest::build(&ui_sprite_index()?, ui_sprite_evidence())?;
    let manifest_json = manifest.to_jsonl();
    if !manifest_json.contains("\"target_kind\":\"SemanticSource\"")
        || !manifest_json
            .contains("\"disposition\":\"requires-semantic-conversion\"")
    {
        return Err(
            "base manifest stopped preserving semantic provenance".to_owned()
        );
    }
    let revision = digest_hex(manifest_json.as_bytes());
    let pending = manifest.plan_bundle(&revision)?;
    if pending.semantic_blocker_count() != 1 {
        return Err("uncompiled UI sprite did not remain a blocker".to_owned());
    }
    let ready = manifest.plan_bundle_with_complete_generated_catalogs(
        &revision,
        None,
        &[verified_ui_raster()],
    )?;
    let cleared = ready.semantic_blocker_count() == 0
        && ready.semantic_blockers().is_empty();
    if !cleared {
        return Err(
            "verified UI raster did not clear its semantic blocker".to_owned()
        );
    }
    let import = ready
        .artifacts()
        .iter()
        .find(|artifact| artifact.family == PlanFamily::AssetImport)
        .ok_or_else(|| "promoted UI import plan is missing".to_owned())?;
    for expected in [
        "\"source_format\":\"image\"",
        "\"source_path\":\"ui-raster-assets/rasters/extracted-ui-sprite.png\"",
        "\"target_class\":\"Texture2D\"",
        "\"importer\":\"texture-factory\"",
        "\"import_profile\":\"shar-texture-v1\"",
        "\"readiness\":\"ready\"",
        &format!("\"source_revision\":\"{}\"", "e".repeat(64)),
    ] {
        if !import.json.contains(expected) {
            return Err(format!("promoted UI plan lost {expected}"));
        }
    }
    Ok(())
}

#[test]
fn ui_raster_does_not_claim_package_with_additional_semantic_member()
-> Result<(), String> {
    let manifest = UnrealImportManifest::build(
        &ui_sprite_with_extra_index()?,
        ui_sprite_with_extra_evidence(),
    )?;
    let revision = digest_hex(manifest.to_jsonl().as_bytes());
    let pending = manifest.plan_bundle(&revision)?;
    if pending.semantic_blocker_count() != 1 {
        return Err(
            "mixed UI package stopped being a semantic blocker".to_owned()
        );
    }
    let mut raster = verified_ui_raster();
    raster.package_id = "extracted-ui-sprite-extra".to_owned();
    raster.path =
        "ui-raster-assets/rasters/extracted-ui-sprite-extra.png".to_owned();
    let claimed = manifest.plan_bundle_with_complete_generated_catalogs(
        &revision,
        None,
        &[raster],
    );
    if claimed.is_ok() {
        return Err(
            "UI raster incorrectly claimed a package with extra semantics"
                .to_owned(),
        );
    }
    Ok(())
}

#[test]
fn complete_ui_raster_catalog_rejects_partial_extra_and_stale_evidence()
-> Result<(), String> {
    let manifest =
        UnrealImportManifest::build(&ui_sprite_index()?, ui_sprite_evidence())?;
    let revision = digest_hex(manifest.to_jsonl().as_bytes());
    if manifest
        .plan_bundle_with_complete_generated_catalogs(&revision, None, &[])
        .is_ok()
    {
        return Err("partial UI raster catalog was accepted".to_owned());
    }
    let mut extra = verified_ui_raster();
    extra.package_id = "unclaimed-ui-sprite".to_owned();
    extra.path = "ui-raster-assets/rasters/unclaimed-ui-sprite.png".to_owned();
    if manifest
        .plan_bundle_with_complete_generated_catalogs(
            &revision,
            None,
            &[verified_ui_raster(), extra],
        )
        .is_ok()
    {
        return Err("unclaimed UI raster package was accepted".to_owned());
    }
    for mutation in ["path", "digest", "source", "width", "height", "tiles"] {
        let mut stale = verified_ui_raster();
        match mutation {
            "path" => {
                stale.path = "ui-raster-assets/rasters/other.png".to_owned();
            }
            "digest" => stale.sha256 = "E".repeat(64),
            "source" => stale.source_revision = "F".repeat(64),
            "width" => stale.width = 0,
            "height" => stale.height = 0,
            "tiles" => stale.tile_count = 0,
            _ => return Err("unknown UI raster mutation".to_owned()),
        }
        let accepted = manifest.plan_bundle_with_complete_generated_catalogs(
            &revision,
            None,
            &[stale],
        );
        if accepted.is_ok() {
            return Err(format!("stale UI raster {mutation} was accepted"));
        }
    }
    Ok(())
}
