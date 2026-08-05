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

use std::collections::BTreeSet;

use super::super::index_render::render_index_jsonl;
use super::{
    MinorUnitId, MinorUnitPackage, MinorUnitRole, PackageCategory, PackageId,
    PackageMember, category_from_root, package_root, role_from_fields,
    subcategory_from_root, validate_package_coverage,
};

#[test]
fn error_packages_publish_only_the_error_role() -> Result<(), String> {
    let mut package =
        package_from_id("extracted-lmlm-latino-customfiles-apu", vec![
            member_with_fields(
                "latino-audio",
                MinorUnitRole::Audio,
                "audio",
                "audio-override",
                "none",
            )?,
        ]);
    package.category = PackageCategory::Error;
    package.subcategory = "error/unclassified".to_owned();

    let rendered = render_index_jsonl(&[package]);
    for required in [
        "\"audio_ids\":[]",
        "\"error_ids\":[\"latino-audio\"]",
        "\"role\":\"error\"",
    ] {
        if !rendered.contains(required) {
            return Err(format!(
                "missing fail-closed role evidence: {required}"
            ));
        }
    }
    Ok(())
}

#[test]
fn package_ids_collapse_separator_runs() {
    for (root, expected) in [
        ("/Extracted//Art/Homer V/", "extracted-art-homer-v"),
        ("derived///language__menu", "derived-language-menu"),
    ] {
        assert_eq!(PackageId::from_root(root).as_str(), expected,);
    }
}

#[test]
fn package_root_groups_package_components() {
    assert_eq!(
        package_root("extracted/art/L1_TERRA/components/texture/a.png"),
        "extracted/art/L1_TERRA"
    );
    assert_eq!(
        package_root("extracted/movies/fmv1A/movie.mov"),
        "extracted/movies/fmv1A"
    );
}

#[test]
fn character_subcategories_split_models_costumes_and_effects() {
    assert_eq!(
        subcategory_from_root("extracted/art/chars/homer_m"),
        "characters/homer/base-model"
    );
    assert_eq!(
        subcategory_from_root("extracted/art/chars/h_stcr_m"),
        "characters/homer/costume/stonecutter"
    );
    assert_eq!(
        subcategory_from_root("extracted/art/chars/homer_electrocuted"),
        "characters/homer/effect/electrocuted"
    );
    assert_eq!(
        subcategory_from_root("extracted/art/chars/b_man_m"),
        "characters/bart/costume/bartman"
    );
    assert_eq!(
        subcategory_from_root("extracted/art/chars/bart_a"),
        "characters/bart/animation-set"
    );
}

#[test]
fn conversation_package_root_splits_by_speaker_kind_and_mission() {
    assert_eq!(
        package_root(
            "extracted/dialog/conversations/c_accuse_1_convinit_hom_l1m7.\
             wav"
        ),
        "extracted/dialog/conversations/hom/convinit/l1m7/accuse"
    );
    assert_eq!(
        subcategory_from_root(
            "extracted/dialog/conversations/hom/convinit/l1m7/accuse"
        ),
        "dialog/homer/conversation/convinit/l1m7"
    );
}

#[test]
fn package_category_routes_export_domains() {
    assert_eq!(
        category_from_root("extracted/art/chars/homer_m"),
        PackageCategory::Characters
    );
    assert_eq!(
        category_from_root("extracted/art/cars/homer_v"),
        PackageCategory::Cars
    );
    assert_eq!(
        category_from_root("extracted/art/L1_TERRA"),
        PackageCategory::TerrainWorld
    );
    assert_eq!(
        subcategory_from_root("extracted/art/missions/level01/m0"),
        "missions/tutorial/m0"
    );
    assert_eq!(
        subcategory_from_root("extracted/art/missions/level02/m3"),
        "missions/level-02/m3"
    );
    assert_eq!(
        category_from_root("extracted/art/missions/l1m1"),
        PackageCategory::Missions
    );
    assert_eq!(
        category_from_root("extracted/art/cards"),
        PackageCategory::Cards
    );
    assert_eq!(
        category_from_root("extracted/music/sound/music"),
        PackageCategory::Music
    );
    assert_eq!(
        category_from_root("extracted/game/art/frontend/scrooby2/pages"),
        PackageCategory::UiScreens
    );
    assert_eq!(
        category_from_root("extracted/lmlm/CustomFiles/homer"),
        PackageCategory::Dialog
    );
    assert_eq!(
        category_from_root("extracted"),
        PackageCategory::ExtractionReports
    );
    assert_eq!(category_from_root("game"), PackageCategory::GameIcons);
    assert_eq!(
        category_from_root("extracted/art/frontend/dynaload/images/foo"),
        PackageCategory::UiImages
    );
    assert_eq!(
        category_from_root("extracted/art/frontend/scrooby2/resource/foo"),
        PackageCategory::UiResources
    );
    assert_eq!(
        category_from_root("extracted/art/frontend/dynaload/cars/homer_v"),
        PackageCategory::UiVehiclePreviews
    );
    assert_eq!(
        category_from_root("extracted/game/scripts/cars/Missions/level01"),
        PackageCategory::VehicleTuning
    );
    assert_eq!(
        category_from_root("extracted/game/scripts/missions/level01"),
        PackageCategory::MissionScripts
    );
    assert_eq!(
        category_from_root("extracted/scripts/sound/scripts"),
        PackageCategory::SoundScripts
    );
    assert_eq!(
        category_from_root("extracted/lmlm/CustomText"),
        PackageCategory::Language
    );
    assert_eq!(
        category_from_root("extracted/art/frontend/scrooby2/language"),
        PackageCategory::Language
    );
    assert_eq!(
        subcategory_from_root("extracted/art/cards"),
        "cards/pickup-effects"
    );
}

#[test]
fn manifest_evidence_routes_mission_script_packages() -> Result<(), String> {
    let package =
        package_from_id("extracted-game-scripts-missions-level03", vec![
            member_with_fields(
                "script-l3m2",
                MinorUnitRole::Script,
                "script",
                "mission-script",
                "mfk",
            )?,
        ]);
    expect_package_classification(
        &package,
        PackageCategory::MissionScripts,
        "missions/level-03/scripts",
    )
}

#[test]
fn manifest_evidence_routes_vehicle_tuning_packages() -> Result<(), String> {
    let package =
        package_from_id("extracted-game-scripts-cars-missions-level04", vec![
            member_with_fields(
                "vehicle-l4m1",
                MinorUnitRole::Script,
                "script",
                "vehicle-tuning",
                "con",
            )?,
        ]);
    expect_package_classification(
        &package,
        PackageCategory::VehicleTuning,
        "missions/level-04/vehicle-tuning/missions-level04",
    )
}

#[test]
fn manifest_evidence_routes_tutorial_mission_art() -> Result<(), String> {
    let package = package_from_id("extracted-art-missions-level01-m0", vec![
        member_with_fields(
            "mission-model-l1m0",
            MinorUnitRole::Model,
            "model",
            "p3d-model",
            "mesh",
        )?,
    ]);
    expect_package_classification(
        &package,
        PackageCategory::Missions,
        "missions/tutorial/models/m0",
    )
}

#[test]
fn manifest_evidence_routes_level08_to_head_to_head() -> Result<(), String> {
    let package =
        package_from_id("extracted-art-missions-level08-chkpts", vec![
            member_with_fields(
                "locator-head-to-head-checkpoint",
                MinorUnitRole::Locator,
                "locator",
                "p3d-locator",
                "srr_locator",
            )?,
        ]);
    expect_package_classification(
        &package,
        PackageCategory::Missions,
        "missions/head-to-head/assets/chkpts",
    )
}

#[test]
fn manifest_routes_dialog_by_speaker_and_context() -> Result<(), String> {
    let package = package_from_id(
        "extracted-dialog-conversations-hom-convinit-l1m7-accuse",
        vec![member_with_fields(
            "dialog-l1m7-hom",
            MinorUnitRole::Audio,
            "audio",
            "rsd-audio",
            "wav-pcm",
        )?],
    );
    expect_package_classification(
        &package,
        PackageCategory::Dialog,
        "dialog/homer/conversation/mission/level-01/convinit/l1m7-accuse/\
         default",
    )
}

#[test]
fn manifest_evidence_routes_unscoped_dialog_archives_to_free_roam()
-> Result<(), String> {
    let package = package_from_id("extracted-dialogf-dr-frink", vec![
        member_with_fields(
            "audio-speaker-archive",
            MinorUnitRole::Audio,
            "audio",
            "rsd-audio",
            "wav-pcm",
        )?,
    ]);
    expect_package_classification(
        &package,
        PackageCategory::Dialog,
        "dialog/dr-frink/ad-lib/free-roam/french",
    )
}

#[test]
fn mission_runtime_assets_use_concrete_scope() -> Result<(), String> {
    let package = package_from_id("extracted-art-missions-generic-door", vec![
        member_with_fields(
            "shared-mission-door",
            MinorUnitRole::Animation,
            "animation",
            "p3d-animation",
            "animation",
        )?,
    ]);
    expect_package_classification(
        &package,
        PackageCategory::Missions,
        "missions/runtime/animations/door",
    )
}

#[test]
fn manifest_evidence_routes_mod_conversation_overrides() -> Result<(), String> {
    let package =
        package_from_id("extracted-lmlm-customfiles-conversations", vec![
            member_with_fields(
                "mod-conversation-audio",
                MinorUnitRole::Audio,
                "audio",
                "audio-override",
                "none",
            )?,
        ]);
    expect_package_classification(
        &package,
        PackageCategory::Dialog,
        "dialog/mod-conversations/mod-override/free-roam",
    )
}

#[test]
fn manifest_evidence_routes_terrain_world_by_level_and_role()
-> Result<(), String> {
    let terrain =
        package_from_id("extracted-art-l04-fx", vec![member_with_fields(
            "terrain-level-four-effects",
            MinorUnitRole::World,
            "world",
            "p3d-mesh",
            "mesh",
        )?]);
    expect_package_classification(
        &terrain,
        PackageCategory::TerrainWorld,
        "terrain-world/level-04/effects",
    )?;
    let interior =
        package_from_id("extracted-art-l7i02", vec![member_with_fields(
            "terrain-level-seven-interior",
            MinorUnitRole::World,
            "world",
            "p3d-mesh",
            "mesh",
        )?]);
    expect_package_classification(
        &interior,
        PackageCategory::TerrainWorld,
        "terrain-world/level-07/interiors/l7i02",
    )?;
    let bonus =
        package_from_id("extracted-art-b02data", vec![member_with_fields(
            "bonus-area-data",
            MinorUnitRole::World,
            "world",
            "p3d-mesh",
            "mesh",
        )?]);
    expect_package_classification(
        &bonus,
        PackageCategory::TerrainWorld,
        "terrain-world/bonus-area/data-records/b02data",
    )
}

#[test]
fn manifest_evidence_routes_cars_by_vehicle_family() -> Result<(), String> {
    let character = package_from_id("extracted-art-cars-homer-v", vec![
        member_with_fields(
            "character-car-model",
            MinorUnitRole::Model,
            "model",
            "p3d-mesh",
            "mesh",
        )?,
    ]);
    expect_package_classification(
        &character,
        PackageCategory::Cars,
        "cars/character-rigs/homer-v",
    )?;
    let commercial =
        package_from_id("extracted-art-cars-ccola", vec![member_with_fields(
            "commercial-car-model",
            MinorUnitRole::Model,
            "model",
            "p3d-mesh",
            "mesh",
        )?]);
    expect_package_classification(
        &commercial,
        PackageCategory::Cars,
        "cars/commercial-vehicles/ccola",
    )?;
    let base =
        package_from_id("extracted-art-cars-common", vec![member_with_fields(
            "vehicle-runtime-base",
            MinorUnitRole::Metadata,
            "metadata",
            "package-manifest",
            "none",
        )?]);
    expect_package_classification(
        &base,
        PackageCategory::Cars,
        "cars/runtime-base/common",
    )
}

#[test]
fn manifest_evidence_routes_frontend_vehicle_models_by_family()
-> Result<(), String> {
    let character =
        package_from_id("extracted-art-frontend-dynaload-cars-homer-v", vec![
            member_with_fields(
                "frontend-character-vehicle-preview",
                MinorUnitRole::Model,
                "model",
                "p3d-mesh",
                "mesh",
            )?,
        ]);
    expect_package_classification(
        &character,
        PackageCategory::UiVehiclePreviews,
        "ui-vehicle-previews/character-rigs/homer-v",
    )?;
    let commercial =
        package_from_id("extracted-art-frontend-dynaload-cars-ccola", vec![
            member_with_fields(
                "frontend-commercial-vehicle-preview",
                MinorUnitRole::Model,
                "model",
                "p3d-mesh",
                "mesh",
            )?,
        ]);
    expect_package_classification(
        &commercial,
        PackageCategory::UiVehiclePreviews,
        "ui-vehicle-previews/commercial-vehicles/ccola",
    )?;

    let metadata =
        package_from_id("extracted-game-art-frontend-dynaload-cars", vec![
            member_with_fields(
                "frontend-vehicle-source-metadata",
                MinorUnitRole::Metadata,
                "metadata",
                "none",
                "none",
            )?,
        ]);
    expect_package_classification(
        &metadata,
        PackageCategory::UiVehiclePreviews,
        "ui-vehicle-previews/source-metadata",
    )
}

#[test]
fn manifest_evidence_routes_media_and_screens() -> Result<(), String> {
    let music = package_from_id("extracted-music02-sound-music-homer", vec![
        member_with_fields(
            "homer-music-bank-entry",
            MinorUnitRole::Audio,
            "audio",
            "runtime-asset",
            "none",
        )?,
    ]);
    expect_package_classification(
        &music,
        PackageCategory::Music,
        "music/bank-02/character-homer",
    )?;
    let sound = package_from_id(
        "extracted-soundfx-sound-soundfx-interactive-props-spanish",
        vec![member_with_fields(
            "localized-interactive-prop-sound",
            MinorUnitRole::Audio,
            "audio",
            "runtime-asset",
            "none",
        )?],
    );
    expect_package_classification(
        &sound,
        PackageCategory::SoundEffects,
        "sound-effects/effects/interactive-props/spanish",
    )?;
    let movie =
        package_from_id("extracted-movies-fmv4", vec![member_with_fields(
            "story-movie",
            MinorUnitRole::Movie,
            "movie",
            "runtime-asset",
            "none",
        )?]);
    expect_package_classification(
        &movie,
        PackageCategory::Movies,
        "movies/story/fmv4",
    )?;
    let screen =
        package_from_id("extracted-art-frontend-scrooby-ingamel4", vec![
            member_with_fields(
                "level-screen-layout",
                MinorUnitRole::Ui,
                "ui",
                "p3d-scrooby-project",
                "scrooby_project",
            )?,
        ]);
    expect_package_classification(
        &screen,
        PackageCategory::UiScreens,
        "ui-screens/sprite-layouts/in-game/level-04",
    )
}

#[test]
fn manifest_evidence_routes_cinematics_by_scope_and_role() -> Result<(), String>
{
    let level_gag = package_from_id("extracted-art-nis-gags-l04-azte", vec![
        member_with_fields(
            "level-four-gag-scene",
            MinorUnitRole::Scene,
            "scene",
            "p3d-scene",
            "scene",
        )?,
    ]);
    expect_package_classification(
        &level_gag,
        PackageCategory::Cinematics,
        "cinematics/gags/level-04/named/azte",
    )?;
    let numbered_gag = package_from_id("extracted-art-nis-gags-gag0207", vec![
        member_with_fields(
            "numbered-gag-scene",
            MinorUnitRole::Scene,
            "scene",
            "p3d-scene",
            "scene",
        )?,
    ]);
    expect_package_classification(
        &numbered_gag,
        PackageCategory::Cinematics,
        "cinematics/gags/series-02/numbered/gag0207",
    )?;
    let audio = package_from_id("extracted-nis-sound-nis-spanish", vec![
        member_with_fields(
            "localized-nis-audio",
            MinorUnitRole::Audio,
            "audio",
            "rsd-audio",
            "audio",
        )?,
    ]);
    expect_package_classification(
        &audio,
        PackageCategory::Cinematics,
        "cinematics/nis-audio/spanish",
    )
}

#[test]
fn manifest_evidence_routes_ui_images_by_scope_and_role() -> Result<(), String>
{
    let vehicle = package_from_id(
        "extracted-art-frontend-dynaload-images-cars2d-apu-vd",
        vec![member_with_fields(
            "damaged-vehicle-preview",
            MinorUnitRole::Texture,
            "texture",
            "png-image",
            "image",
        )?],
    );
    expect_package_classification(
        &vehicle,
        PackageCategory::UiImages,
        "ui-images/vehicle-icons/damaged/apu",
    )?;
    let mission_icon = package_from_id(
        "extracted-art-frontend-dynaload-images-msnicons-location-house",
        vec![member_with_fields(
            "mission-location-icon",
            MinorUnitRole::Texture,
            "texture",
            "png-image",
            "image",
        )?],
    );
    expect_package_classification(
        &mission_icon,
        PackageCategory::UiImages,
        "ui-images/mission-icons/locations/house",
    )?;

    let mission_metadata = package_from_id(
        "extracted-game-art-frontend-dynaload-images-msnicons",
        vec![member_with_fields(
            "mission-icon-source-metadata",
            MinorUnitRole::Metadata,
            "metadata",
            "none",
            "none",
        )?],
    );
    expect_package_classification(
        &mission_metadata,
        PackageCategory::UiImages,
        "ui-images/mission-icons/source-metadata",
    )?;
    let metadata =
        package_from_id("extracted-game-art-frontend-dynaload-images", vec![
            member_with_fields(
                "unscoped-image-metadata",
                MinorUnitRole::Metadata,
                "metadata",
                "none",
                "none",
            )?,
        ]);
    expect_package_classification(
        &metadata,
        PackageCategory::UiImages,
        "ui-images/source-metadata",
    )
}

#[test]
fn manifest_evidence_routes_ui_resources_by_scope_and_role()
-> Result<(), String> {
    let card = package_from_id(
        "extracted-art-frontend-scrooby2-resource-frontend-card12",
        vec![member_with_fields(
            "frontend-card-icon",
            MinorUnitRole::Ui,
            "ui",
            "p3d-texture",
            "texture",
        )?],
    );
    expect_package_classification(
        &card,
        PackageCategory::UiResources,
        "ui-resources/frontend/cards/card12",
    )?;
    let scene_resource = package_from_id(
        "extracted-art-frontend-scrooby-resource-pure3d-camset",
        vec![member_with_fields(
            "frontend-scene-camera-set",
            MinorUnitRole::Ui,
            "ui",
            "p3d-scene",
            "scene",
        )?],
    );
    expect_package_classification(
        &scene_resource,
        PackageCategory::UiResources,
        "ui-resources/frontend-scenes/camera-sets/sprite-layouts/camset",
    )?;
    let loading = package_from_id(
        "extracted-art-frontend-scrooby2-resource-backend-loading0",
        vec![member_with_fields(
            "backend-loading-icon",
            MinorUnitRole::Ui,
            "ui",
            "p3d-texture",
            "texture",
        )?],
    );
    expect_package_classification(
        &loading,
        PackageCategory::UiResources,
        "ui-resources/backend/loading/loading0",
    )
}

#[test]
fn role_mapping_preserves_world_and_texture_ids() {
    assert_eq!(
        role_from_fields("world", "p3d-road-network", "srr_road"),
        MinorUnitRole::World
    );
    assert_eq!(
        role_from_fields("image", "p3d-texture", "texture"),
        MinorUnitRole::Texture
    );
}

#[test]
fn coverage_rejects_uncataloged_manifest_id() -> Result<(), String> {
    let manifest_ids = id_set(&["world-a", "texture-b"])?;
    let packages = vec![package("extracted/art/L1_TERRA", &["world-a"])?];
    let error = match validate_package_coverage(&manifest_ids, &packages) {
        Ok(()) => {
            return Err("missing package member should fail".to_owned());
        },
        Err(error) => error,
    };
    if !error.to_string().contains("coverage mismatch") {
        return Err(format!("unexpected missing-id error: {error}"));
    }
    Ok(())
}

#[test]
fn coverage_rejects_duplicate_index_id() -> Result<(), String> {
    let manifest_ids = id_set(&["world-a"])?;
    let packages = vec![
        package("extracted/art/L1_TERRA", &["world-a"])?,
        package("extracted/art/L1_INTERIOR", &["world-a"])?,
    ];
    let error = match validate_package_coverage(&manifest_ids, &packages) {
        Ok(()) => {
            return Err("duplicate package member should fail".to_owned());
        },
        Err(error) => error,
    };
    if !error.to_string().contains("more than one package") {
        return Err(format!("unexpected duplicate-id error: {error}"));
    }
    Ok(())
}

#[test]
fn coverage_accepts_exact_manifest_index_match() -> Result<(), String> {
    let manifest_ids = id_set(&["world-a", "texture-b"])?;
    let packages = vec![package("extracted/art/L1_TERRA", &[
        "world-a",
        "texture-b",
    ])?];
    validate_package_coverage(&manifest_ids, &packages)
        .map_err(|error| error.to_string())
}

fn expect_package_classification(
    package: &MinorUnitPackage,
    category: PackageCategory,
    subcategory: &str,
) -> Result<(), String> {
    if package.category != category {
        return Err(format!(
            "expected category {:?}, got {:?}",
            category, package.category
        ));
    }
    if package.subcategory != subcategory {
        return Err(format!(
            "expected subcategory {subcategory}, got {}",
            package.subcategory
        ));
    }
    Ok(())
}

fn id_set(values: &[&str]) -> Result<BTreeSet<MinorUnitId>, String> {
    let mut output = BTreeSet::new();
    for value in values {
        let id = minor_unit_id(value)?;
        let _inserted = output.insert(id);
    }
    Ok(output)
}

fn package_from_id(
    package_id: &str,
    members: Vec<PackageMember>,
) -> MinorUnitPackage {
    let mut package = MinorUnitPackage {
        package_id: PackageId(package_id.to_owned()),
        package_root: "manifest-evidence-group".to_owned(),
        category: PackageCategory::Error,
        subcategory: "error/unclassified".to_owned(),
        members,
        source_unit_ids: Vec::new(),
        text_keys: Vec::new(),
    };
    package.refresh_classification_from_members();
    package
}

fn package(root: &str, ids: &[&str]) -> Result<MinorUnitPackage, String> {
    let mut package = MinorUnitPackage {
        package_id: PackageId::from_root(root),
        package_root: root.to_owned(),
        category: PackageCategory::Error,
        subcategory: "error/unclassified".to_owned(),
        members: ids
            .iter()
            .map(|id| member(id))
            .collect::<Result<Vec<_>, _>>()?,
        source_unit_ids: Vec::new(),
        text_keys: Vec::new(),
    };
    package.refresh_classification_from_members();
    Ok(package)
}

fn member(value: &str) -> Result<PackageMember, String> {
    member_with_fields(
        value,
        MinorUnitRole::World,
        "world",
        "p3d-world-dsg",
        "srr_fence_dsg",
    )
}

fn member_with_fields(
    value: &str,
    role: MinorUnitRole,
    type_: &str,
    kind: &str,
    source_chunk_kind: &str,
) -> Result<PackageMember, String> {
    Ok(PackageMember {
        id: minor_unit_id(value)?,
        role,
        path: format!("extracted/art/L1_TERRA/components/world/{value}.json"),
        type_: type_.to_owned(),
        kind: kind.to_owned(),
        source_chunk_kind: source_chunk_kind.to_owned(),
    })
}

fn minor_unit_id(value: &str) -> Result<MinorUnitId, String> {
    MinorUnitId::new(value.to_owned())
        .ok_or_else(|| "test id should be non-empty".to_owned())
}
