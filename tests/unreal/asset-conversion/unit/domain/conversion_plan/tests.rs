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
//   - Unreal conversion-plan domain tests.
// - Must-Not:
//   - Read product content, contact Unreal, or publish generated artifacts.
// - Allows:
//   - Synthetic source evidence and deterministic assertions.
// - Split-When:
//   - Split when another plan family needs independent fixtures.
// - Merge-When:
//   - Merge when another test module owns identical evidence.
// - Summary:
//   - Unreal conversion-plan domain tests.
// - Description:
//   - Proves ordering, revisions, path safety, and collision rejection.
// - Usage:
//   - Included only by the conversion-plan domain module under cfg(test).
// - Defaults:
//   - All fixtures are synthetic and public-safe.
//

//! Unreal conversion-plan domain tests.

use super::{
    ConversionPlan, NativeAssetFamily, OperationReadiness, PlanBundle,
    PlanContext, PlanFamily, SourceFormat, UNREAL_PLAN_BUNDLE_SCHEMA,
    UNREAL_PLAN_SCHEMA,
};

fn context() -> PlanContext {
    PlanContext {
        source_manifest_revision: "a".repeat(64),
        engine_contract_revision: "shar-unreal-porting-contract-v1".to_owned(),
        target_engine_version: "5.8".to_owned(),
        target_platform: "editor".to_owned(),
    }
}

fn wav_operation() -> ConversionPlan {
    ConversionPlan {
        package_identity: "dialog-package".to_owned(),
        source_identity: "audio-source".to_owned(),
        source_format: SourceFormat::Wav,
        target_family: NativeAssetFamily::Audio,
        source_path: "extracted/dialog/audio.wav".to_owned(),
        source_revision: "b".repeat(64),
        destination: concat!(
            "/Game/Generated/SHAR/dialog/dialog_package/",
            "audio_source.audio_source"
        )
        .to_owned(),
        target_class: "SoundWave".to_owned(),
        importer: "sound-wave-factory".to_owned(),
        import_profile: "shar-audio-v1".to_owned(),
        dependencies: Vec::new(),
        readiness: OperationReadiness::Ready,
        world_owned: false,
        runtime_bound: true,
    }
}

fn json_operation() -> ConversionPlan {
    ConversionPlan {
        package_identity: "mission-package".to_owned(),
        source_identity: "mission-source".to_owned(),
        source_format: SourceFormat::Json,
        target_family: NativeAssetFamily::StructuredData,
        source_path: "extracted/missions/mission.json".to_owned(),
        source_revision: "c".repeat(64),
        destination: concat!(
            "/Game/Generated/SHAR/missions/mission_package/",
            "mission_package.mission_package"
        )
        .to_owned(),
        target_class: "StateTree".to_owned(),
        importer: "shar-state-tree-factory".to_owned(),
        import_profile: "shar-state-tree-v1".to_owned(),
        dependencies: Vec::new(),
        readiness: OperationReadiness::RequiresEditorFactory,
        world_owned: false,
        runtime_bound: true,
    }
}

#[test]
fn builds_all_plan_families_with_stable_revisions() -> Result<(), String> {
    let first =
        PlanBundle::build(&context(), vec![json_operation(), wav_operation()])?;
    let second =
        PlanBundle::build(&context(), vec![wav_operation(), json_operation()])?;
    if first != second {
        return Err("input order changed the plan bundle".to_owned());
    }
    if first.artifacts().len() != PlanFamily::all().len() {
        return Err("bundle does not contain all plan families".to_owned());
    }
    let import = first
        .artifacts()
        .iter()
        .find(|artifact| artifact.family == PlanFamily::AssetImport)
        .ok_or_else(|| "asset import plan is missing".to_owned())?;
    let construction = first
        .artifacts()
        .iter()
        .find(|artifact| artifact.family == PlanFamily::AssetConstruction)
        .ok_or_else(|| "asset construction plan is missing".to_owned())?;
    if import.operation_count != 1 || construction.operation_count != 1 {
        return Err("operations were assigned to the wrong family".to_owned());
    }
    if !import.json.contains(UNREAL_PLAN_SCHEMA)
        || !first.index_json().contains(UNREAL_PLAN_BUNDLE_SCHEMA)
        || first.index_revision().len() != 64
    {
        return Err("rendered plan identity is invalid".to_owned());
    }
    Ok(())
}

#[test]
fn pins_every_plan_dependency_to_the_exact_revision() -> Result<(), String> {
    let bundle =
        PlanBundle::build(&context(), vec![json_operation(), wav_operation()])?;
    for artifact in bundle.artifacts() {
        let dependency_ids = artifact
            .dependencies
            .iter()
            .map(|dependency| dependency.plan_id.as_str())
            .collect::<Vec<_>>();
        let lexical_ids = dependency_ids
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        if dependency_ids != lexical_ids {
            return Err(format!(
                "plan dependencies are not unique and lexical: {}",
                artifact.family.plan_id()
            ));
        }
        for dependency in &artifact.dependencies {
            let referenced = bundle
                .artifacts()
                .iter()
                .find(|candidate| {
                    candidate.family.plan_id() == dependency.plan_id
                })
                .ok_or_else(|| {
                    format!("missing dependency plan: {}", dependency.plan_id)
                })?;
            if dependency.revision != referenced.revision {
                return Err(format!(
                    "dependency revision drifted for {}",
                    dependency.plan_id
                ));
            }
            let rendered = format!(
                r#"{{"plan_id":"{}","revision":"{}"}}"#,
                dependency.plan_id, dependency.revision
            );
            if !artifact.json.contains(&rendered) {
                return Err(format!(
                    "rendered dependency is missing for {}",
                    dependency.plan_id
                ));
            }
        }
    }
    Ok(())
}

#[test]
fn upstream_revision_changes_cascade_through_the_bundle() -> Result<(), String>
{
    let baseline =
        PlanBundle::build(&context(), vec![json_operation(), wav_operation()])?;
    let mut changed_wav = wav_operation();
    changed_wav.source_revision = "d".repeat(64);
    let changed =
        PlanBundle::build(&context(), vec![json_operation(), changed_wav])?;
    if baseline.index_revision() == changed.index_revision() {
        return Err(
            "bundle revision ignored an upstream source change".to_owned()
        );
    }
    for family in PlanFamily::all() {
        let before = baseline
            .artifacts()
            .iter()
            .find(|artifact| artifact.family == family)
            .ok_or_else(|| {
                format!("missing baseline plan: {}", family.plan_id())
            })?;
        let after = changed
            .artifacts()
            .iter()
            .find(|artifact| artifact.family == family)
            .ok_or_else(|| {
                format!("missing changed plan: {}", family.plan_id())
            })?;
        if before.revision == after.revision {
            return Err(format!(
                "upstream revision did not cascade into {}",
                family.plan_id()
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_unsafe_paths_and_destination_collisions() -> Result<(), String> {
    let private_path = "C:/private/audio.wav";
    let mut unsafe_operation = wav_operation();
    unsafe_operation.source_path = private_path.to_owned();
    let Err(path_error) = PlanBundle::build(&context(), vec![unsafe_operation])
    else {
        return Err("absolute source path was accepted".to_owned());
    };
    if path_error.contains(private_path) {
        return Err(format!("source-path diagnostic leaked: {path_error}"));
    }
    let first = wav_operation();
    let mut second = json_operation();
    second.destination = first.destination.to_ascii_uppercase();
    if PlanBundle::build(&context(), vec![first, second]).is_ok() {
        return Err("destination collision was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_uppercase_revisions_and_unsorted_dependencies() -> Result<(), String>
{
    let mut invalid_revision = wav_operation();
    invalid_revision.source_revision = "B".repeat(64);
    if PlanBundle::build(&context(), vec![invalid_revision]).is_ok() {
        return Err("uppercase SHA-256 was accepted".to_owned());
    }
    let dependency = wav_operation();
    let mut dependent = json_operation();
    dependent.dependencies = vec![
        "operation-ffffffffffffffff".to_owned(),
        dependency.operation_id(),
    ];
    if PlanBundle::build(&context(), vec![dependency, dependent]).is_ok() {
        return Err("unsorted dependencies were accepted".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_source_target_and_readiness_mismatches() -> Result<(), String> {
    let mut wrong_family = wav_operation();
    wrong_family.target_family = NativeAssetFamily::Texture;
    let Err(family_error) = PlanBundle::build(&context(), vec![wrong_family])
    else {
        return Err("WAV evidence was accepted as a texture".to_owned());
    };
    if !family_error.contains("target family") {
        return Err(format!("unexpected family failure: {family_error}"));
    }

    let mut wrong_readiness = wav_operation();
    wrong_readiness.readiness = OperationReadiness::RequiresConversion;
    let Err(readiness_error) =
        PlanBundle::build(&context(), vec![wrong_readiness])
    else {
        return Err(
            "ready WAV evidence was accepted as pending conversion".to_owned()
        );
    };
    if !readiness_error.contains("operation readiness") {
        return Err(format!("unexpected readiness failure: {readiness_error}"));
    }
    Ok(())
}

#[test]
fn target_class_participates_in_operation_identity() -> Result<(), String> {
    let baseline = wav_operation();
    let mut changed = baseline.clone();
    changed.target_class = "SoundCue".to_owned();
    if baseline.operation_id() == changed.operation_id() {
        return Err("target class did not affect operation identity".to_owned());
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_dependencies_and_destinations() -> Result<(), String> {
    let mut dependency = wav_operation();
    dependency.dependencies = vec!["audio-source".to_owned()];
    let Err(dependency_error) = PlanBundle::build(&context(), vec![dependency])
    else {
        return Err("noncanonical operation dependency was accepted".to_owned());
    };
    if !dependency_error.contains("dependency identity") {
        return Err(format!(
            "unexpected dependency failure: {dependency_error}"
        ));
    }

    for destination in [
        "/Game/Generated/SHAR/audio/bad name/bad name.bad name",
        "/Game/Generated/SHAR/audio/bad.name/bad.name",
        "/Game/Generated/SHAR/audio/good.bad",
    ] {
        let mut operation = wav_operation();
        operation.destination = destination.to_owned();
        if PlanBundle::build(&context(), vec![operation]).is_ok() {
            return Err(format!(
                "noncanonical Unreal destination was accepted: {destination}"
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_cyclic_operation_dependencies() -> Result<(), String> {
    let mut first = wav_operation();
    first.source_identity = "audio-source-a".to_owned();
    first.destination =
        "/Game/Generated/SHAR/audio/source_a/source_a.source_a".to_owned();
    let mut second = wav_operation();
    second.source_identity = "audio-source-b".to_owned();
    second.destination =
        "/Game/Generated/SHAR/audio/source_b/source_b.source_b".to_owned();
    let first_id = first.operation_id();
    let second_id = second.operation_id();
    first.dependencies = vec![second_id];
    second.dependencies = vec![first_id];

    let Err(error) = PlanBundle::build(&context(), vec![first, second]) else {
        return Err("cyclic operation dependencies were accepted".to_owned());
    };
    if !error.contains("dependency graph contains a cycle") {
        return Err(format!("unexpected cycle failure: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_path_shaped_identities_without_echoing_them() -> Result<(), String> {
    let private_identity = "C:/private/operator/package";
    let mut operation = wav_operation();
    operation.package_identity = private_identity.to_owned();
    let Err(error) = PlanBundle::build(&context(), vec![operation]) else {
        return Err("path-shaped package identity was accepted".to_owned());
    };
    if error.contains(private_identity) {
        return Err(format!("identity diagnostic leaked: {error}"));
    }
    if !error.contains("invalid package identity") {
        return Err(format!("unexpected identity failure: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_dependencies_on_later_plan_families() -> Result<(), String> {
    let construction = json_operation();
    let mut import = wav_operation();
    import.dependencies = vec![construction.operation_id()];
    let Err(error) = PlanBundle::build(&context(), vec![import, construction])
    else {
        return Err("import operation depended on construction".to_owned());
    };
    if !error.contains("later plan family") {
        return Err(format!("unexpected family-order failure: {error}"));
    }
    Ok(())
}

#[test]
fn rendered_validation_names_the_enforced_destination_guard()
-> Result<(), String> {
    let bundle = PlanBundle::build(&context(), vec![wav_operation()])?;
    for artifact in bundle.artifacts() {
        if !artifact
            .json
            .contains("case-insensitive-destinations-unique")
        {
            return Err(format!(
                "{} omits the destination collision guard",
                artifact.family.plan_id()
            ));
        }
        if artifact.json.contains("case-insensitive-identities-unique") {
            return Err(format!(
                "{} claims an unenforced identity guard",
                artifact.family.plan_id()
            ));
        }
    }
    Ok(())
}

#[test]
fn rejects_control_characters_in_portable_source_paths() -> Result<(), String> {
    for source_path in
        ["extracted/audio/clip\n.wav", "extracted/audio/clip\0.wav"]
    {
        let mut operation = wav_operation();
        operation.source_path = source_path.to_owned();
        let Err(error) = PlanBundle::build(&context(), vec![operation]) else {
            return Err("control-bearing source path was accepted".to_owned());
        };
        if error.contains(source_path) {
            return Err(format!("source-path diagnostic leaked: {error}"));
        }
    }
    Ok(())
}
