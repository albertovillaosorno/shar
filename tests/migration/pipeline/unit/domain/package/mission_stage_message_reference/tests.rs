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
//   - Unit evidence for canonical stage-message localization binding.
// - Must-Not:
//   - Assert localized payload text or runtime presentation policy.
// - Allows:
//   - Verify objective/locked key formatting and mirror resolution.
// - Split-When:
//   - Localization payload compilation needs independent fixtures.
// - Merge-When:
//   - Final mission asset tests own these exact key references.
// - Summary:
//   - Stage-message localization reference unit tests.
// - Description:
//   - Proves authored indices bind to one exact package-index text key.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Missing and ambiguous text-key mirrors fail closed.
//

//! Unit evidence for stage-message localization-key references.

use super::*;
use crate::domain::{MissionStageKind, PhaseThreePackageIndex};

fn physical_source_row() -> &'static str {
    concat!(
        "{\"package_id\":\"zeta-source\",",
        "\"package_root\":\"zeta-source\",",
        "\"package_category\":\"cars\",",
        "\"package_subcategory\":\"cars/character-rigs/homer-v\",",
        "\"unit_count\":1,\"text_key_count\":0,",
        "\"unit_ids\":[\"source-a\"],",
        "\"world_ids\":[],\"texture_ids\":[],",
        "\"material_ids\":[],\"model_ids\":[\"source-a\"],",
        "\"physics_ids\":[],\"animation_ids\":[],",
        "\"scene_ids\":[],\"locator_ids\":[],",
        "\"camera_ids\":[],\"light_ids\":[],",
        "\"particle_ids\":[],\"controller_ids\":[],",
        "\"audio_ids\":[],\"movie_ids\":[],",
        "\"script_ids\":[],\"text_ids\":[],",
        "\"ui_ids\":[],\"metadata_ids\":[],\"error_ids\":[],",
        "\"source_unit_ids\":[],\"text_key_ids\":[],",
        "\"members\":[{\"id\":\"source-a\",\"role\":\"model\",",
        "\"path\":\"extracted/model.p3d\",\"type\":\"model\",",
        "\"kind\":\"mesh\",\"source_chunk_kind\":\"mesh\"}],",
        "\"text_keys\":[]}"
    )
}

fn derived_row(
    package_id: &str,
    subcategory: &str,
    key_id: &str,
    key: &str,
) -> String {
    format!(
        concat!(
            "{{\"package_id\":\"{0}\",\"package_root\":\"{0}\",",
            "\"package_category\":\"language\",",
            "\"package_subcategory\":\"{1}\",",
            "\"unit_count\":0,\"text_key_count\":1,\"unit_ids\":[],",
            "\"world_ids\":[],\"texture_ids\":[],\"material_ids\":[],",
            "\"model_ids\":[],\"physics_ids\":[],\"animation_ids\":[],",
            "\"scene_ids\":[],\"locator_ids\":[],\"camera_ids\":[],",
            "\"light_ids\":[],\"particle_ids\":[],\"controller_ids\":[],",
            "\"audio_ids\":[],\"movie_ids\":[],\"script_ids\":[],",
            "\"text_ids\":[],\"ui_ids\":[],\"metadata_ids\":[],",
            "\"error_ids\":[],\"source_unit_ids\":[\"source-a\"],",
            "\"text_key_ids\":[\"{2}\"],\"members\":[],",
            "\"text_keys\":[{{\"id\":\"{2}\",\"key\":\"{3}\",",
            "\"source_unit_id\":\"source-a\",",
            "\"subcategory\":\"{1}\"}}]}}"
        ),
        package_id, subcategory, key_id, key,
    )
}

fn index() -> Result<PhaseThreePackageIndex, String> {
    let contents = [
        derived_row(
            "aaa-objective",
            "language/text/missions/objective-lines",
            "text-objective",
            "MISSION_OBJECTIVE_42",
        ),
        derived_row(
            "middle-locked",
            "language/text/ui/runtime",
            "text-locked",
            "INGAME_MESSAGE_03",
        ),
        physical_source_row().to_owned(),
    ]
    .join("\n");
    PhaseThreePackageIndex::from_jsonl(&contents)
        .map_err(|error| error.to_string())
}

fn stages() -> MissionStageSemanticReport {
    MissionStageSemanticReport::from_message_entries_for_tests(vec![
        (
            2,
            0,
            MissionStageKind::Standard {
                legacy_flags: None,
                final_stage: false,
            },
            vec![MissionStageDirective::MessageIndex {
                source_ordinal: 3,
                kind: MissionStageMessageKind::Objective,
                index: 42,
                unused_argument: None,
            }],
        ),
        (
            8,
            1,
            MissionStageKind::LockedVehicle {
                vehicle_id: "car".to_owned(),
            },
            vec![MissionStageDirective::MessageIndex {
                source_ordinal: 9,
                kind: MissionStageMessageKind::Locked,
                index: 3,
                unused_argument: None,
            }],
        ),
    ])
}

#[test]
fn binds_objective_and_locked_message_keys() -> Result<(), String> {
    let report =
        preflight_mission_stage_message_references(&index()?, &stages())?;
    let [objective, locked] = report.bindings() else {
        return Err("stage-message binding count drifted".to_owned());
    };
    assert_eq!(objective.stage_source_ordinal(), 2);
    assert_eq!(objective.stage_sequence_ordinal(), 0);
    assert_eq!(objective.source_ordinal(), 3);
    assert_eq!(objective.kind(), MissionStageMessageKind::Objective);
    assert_eq!(objective.index(), 42);
    assert_eq!(objective.key(), "MISSION_OBJECTIVE_42");
    assert_eq!(objective.text_key_id(), "text-objective");
    assert_eq!(objective.source_unit_id(), "source-a");
    assert_eq!(objective.package_id(), "aaa-objective");
    assert_eq!(
        objective.package_subcategory(),
        "language/text/missions/objective-lines"
    );
    assert_eq!(locked.key(), "INGAME_MESSAGE_03");
    assert_eq!(locked.text_key_id(), "text-locked");
    Ok(())
}

#[test]
fn rejects_missing_stage_message_key() -> Result<(), String> {
    let report =
        MissionStageSemanticReport::from_message_entries_for_tests(vec![(
            2,
            0,
            MissionStageKind::Standard {
                legacy_flags: None,
                final_stage: false,
            },
            vec![MissionStageDirective::MessageIndex {
                source_ordinal: 3,
                kind: MissionStageMessageKind::Objective,
                index: 41,
                unused_argument: None,
            }],
        )]);
    let result = preflight_mission_stage_message_references(&index()?, &report);
    let Err(error) = result else {
        return Err("missing stage-message key unexpectedly passed".to_owned());
    };
    assert!(error.contains("MISSION_OBJECTIVE_41"));
    Ok(())
}

#[test]
fn rejects_ambiguous_stage_message_key() -> Result<(), String> {
    let contents = [
        derived_row(
            "aaa-objective",
            "language/text/missions/objective-lines",
            "text-a",
            "MISSION_OBJECTIVE_42",
        ),
        derived_row(
            "beta-objective",
            "language/text/missions/objective-lines",
            "text-b",
            "MISSION_OBJECTIVE_42",
        ),
        physical_source_row().to_owned(),
    ]
    .join("\n");
    let index = PhaseThreePackageIndex::from_jsonl(&contents)
        .map_err(|error| error.to_string())?;
    let report =
        MissionStageSemanticReport::from_message_entries_for_tests(vec![(
            2,
            0,
            MissionStageKind::Standard {
                legacy_flags: None,
                final_stage: false,
            },
            vec![MissionStageDirective::MessageIndex {
                source_ordinal: 3,
                kind: MissionStageMessageKind::Objective,
                index: 42,
                unused_argument: None,
            }],
        )]);
    let result = preflight_mission_stage_message_references(&index, &report);
    let Err(error) = result else {
        return Err(
            "ambiguous stage-message key unexpectedly passed".to_owned(),
        );
    };
    assert!(error.contains("ambiguous"));
    Ok(())
}
