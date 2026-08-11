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
//   - Unit evidence for source-backed mission definition-core joins.
// - Must-Not:
//   - Add runtime transition or recovery semantics.
// - Allows:
//   - Verify exact stage/objective/condition ownership and retained identity.
// - Split-When:
//   - Final mission asset emission gains independently testable behavior.
// - Merge-When:
//   - Definition compiler tests own this exact join contract.
// - Summary:
//   - Mission definition-core context unit tests.
// - Description:
//   - Locks ownership joins and source-only authored topology evidence.
// - Usage:
//   - Compiled with local outbound adapter unit tests.
// - Defaults:
//   - Missing or mismatched ownership fails closed.
//

//! Unit evidence for source-backed mission definition-core joins.

use super::*;
use crate::domain::{
    MissionConditionParameters, MissionConditionSemanticReport,
    MissionObjectiveParameters, MissionObjectiveSemanticReport,
    MissionRoadArrowBinding, MissionRoadArrowMode, MissionStageDirective,
    MissionStageSemanticReport, preflight_mission_authored_stage_topology,
};

fn reports() -> (
    MissionStageSemanticReport,
    MissionObjectiveSemanticReport,
    MissionConditionSemanticReport,
    MissionAuthoredStageTopologyReport,
) {
    let stages = MissionStageSemanticReport::from_topology_entries_for_tests(
        vec![
            (
                2,
                0,
                false,
                vec![MissionStageDirective::ResetCheckpoint {
                    source_ordinal: 3,
                }],
            ),
            (10, 1, true, Vec::new()),
        ],
    );
    let objectives = MissionObjectiveSemanticReport::
        from_route_entries_with_parameters_for_tests(vec![
            (
                2,
                0,
                4,
                "goto".to_owned(),
                MissionObjectiveParameters::RoadArrows(
                    MissionRoadArrowBinding::Effective(
                        MissionRoadArrowMode::Both,
                    ),
                ),
                Vec::new(),
            ),
            (
                10,
                1,
                11,
                "dummy".to_owned(),
                MissionObjectiveParameters::None,
                Vec::new(),
            ),
        ]);
    let conditions = MissionConditionSemanticReport::
        from_owned_entries_with_parameters_for_tests(vec![
            (
                2,
                0,
                Some(4),
                5,
                "timeout".to_owned(),
                MissionConditionScope::Objective,
                "legacy-mission-condition.timeout.v1",
                MissionConditionParameters::None,
            ),
            (
                10,
                1,
                None,
                12,
                "damage".to_owned(),
                MissionConditionScope::Stage,
                "legacy-mission-condition.damage.v1",
                MissionConditionParameters::DamageLegacyToken {
                    source_token: "neither".to_owned(),
                    code: "legacy-damage-condition-neither-parameter-v1",
                },
            ),
        ]);
    let topology = preflight_mission_authored_stage_topology(&stages)
        .expect("topology fixture must stay valid");
    (stages, objectives, conditions, topology)
}

#[test]
fn joins_source_backed_stage_definition_core() -> Result<(), String> {
    let (stages, objectives, conditions, topology) = reports();
    let report = build_definition_core(
        "m1",
        &stages,
        &objectives,
        &conditions,
        &topology,
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(report.mission_id(), "m1");
    assert!(!report.has_only_mapped_objectives());
    let [first, second] = report.stages() else {
        return Err("definition-core stage count changed".to_owned());
    };
    assert_eq!(first.stage_source_ordinal(), 2);
    assert_eq!(first.sequence_ordinal(), 0);
    assert_eq!(first.next_authored_sequence_ordinal(), Some(1));
    assert_eq!(first.checkpoint_source_ordinal(), Some(3));
    assert!(!first.explicit_final());
    assert_eq!(first.terminal(), MissionStageTerminalOutcome::None);
    assert_eq!(first.objective_source_alias(), "goto");
    assert_eq!(first.objective_canonical_kind(), Some("travel"));
    assert_eq!(
        first.objective_parameters(),
        &MissionObjectiveParameters::RoadArrows(
            MissionRoadArrowBinding::Effective(MissionRoadArrowMode::Both)
        )
    );
    let [condition] = first.conditions() else {
        return Err(
            "definition-core first-stage condition count changed".to_owned()
        );
    };
    assert_eq!(condition.source_ordinal(), 5);
    assert_eq!(condition.source_alias(), "timeout");
    assert_eq!(
        condition.schema_id(),
        "legacy-mission-condition.timeout.v1"
    );
    assert_eq!(condition.scope(), MissionConditionScope::Objective);
    assert_eq!(condition.owner_objective_source_ordinal(), Some(4));
    assert_eq!(condition.parameters(), &MissionConditionParameters::None);

    assert_eq!(second.sequence_ordinal(), 1);
    assert_eq!(second.next_authored_sequence_ordinal(), None);
    assert!(second.explicit_final());
    assert_eq!(second.objective_source_alias(), "dummy");
    assert_eq!(second.objective_canonical_kind(), None);
    assert_eq!(
        second.objective_parameters(),
        &MissionObjectiveParameters::None
    );
    let [condition] = second.conditions() else {
        return Err(
            "definition-core second-stage condition count changed".to_owned()
        );
    };
    assert_eq!(
        condition.parameters(),
        &MissionConditionParameters::DamageLegacyToken {
            source_token: "neither".to_owned(),
            code: "legacy-damage-condition-neither-parameter-v1",
        }
    );
    Ok(())
}

#[test]
fn rejects_objective_with_wrong_stage_owner() {
    let (stages, _objectives, conditions, topology) = reports();
    let objectives =
        MissionObjectiveSemanticReport::from_route_entries_for_tests(vec![
            (2, 1, 4, "goto".to_owned(), Vec::new()),
            (10, 1, 11, "dummy".to_owned(), Vec::new()),
        ]);
    let error = build_definition_core(
        "m1",
        &stages,
        &objectives,
        &conditions,
        &topology,
    )
    .expect_err("wrong objective owner must fail");
    assert!(error.to_string().contains("unique root objective"));
}

#[test]
fn rejects_condition_with_unknown_stage_owner() {
    let (stages, objectives, _conditions, topology) = reports();
    let conditions =
        MissionConditionSemanticReport::from_owned_entries_for_tests(vec![(
            99,
            0,
            None,
            100,
            "timeout".to_owned(),
            MissionConditionScope::Stage,
            "legacy-mission-condition.timeout.v1",
        )]);
    let error = build_definition_core(
        "m1",
        &stages,
        &objectives,
        &conditions,
        &topology,
    )
    .expect_err("unknown condition owner must fail");
    assert!(error.to_string().contains("unknown stage owner"));
}

#[test]
fn rejects_objective_condition_with_wrong_root_owner() {
    let (stages, objectives, _conditions, topology) = reports();
    let conditions =
        MissionConditionSemanticReport::from_owned_entries_for_tests(vec![(
            2,
            0,
            Some(9),
            5,
            "timeout".to_owned(),
            MissionConditionScope::Objective,
            "legacy-mission-condition.timeout.v1",
        )]);
    let error = build_definition_core(
        "m1",
        &stages,
        &objectives,
        &conditions,
        &topology,
    )
    .expect_err("wrong condition objective owner must fail");
    assert!(error
        .to_string()
        .contains("condition owner disagrees"));
}
