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
use crate::domain::package::MissionPickupStatePropScope;
use crate::domain::{
    MissionConditionParameters, MissionConditionSemanticReport,
    MissionInitializationDirective, MissionInitializationReport,
    MissionObjectiveParameters, MissionObjectiveSemanticReport,
    MissionRoadArrowBinding, MissionRoadArrowMode, MissionStageDirective,
    MissionStageSemanticReport, MissionStageTransitionMarkerKind,
    MissionStageVisualTransition,
    preflight_mission_authored_stage_topology,
};

fn empty_initialization(mission_id: &str) -> MissionInitializationReport {
    MissionInitializationReport::from_directives_for_tests(
        mission_id,
        Vec::new(),
    )
}

fn reports() -> Result<
    (
        MissionStageSemanticReport,
        MissionObjectiveSemanticReport,
        MissionConditionSemanticReport,
        MissionAuthoredStageTopologyReport,
    ),
    String,
> {
    let stages = MissionStageSemanticReport::from_topology_entries_for_tests(
        vec![
            (
                2,
                0,
                false,
                vec![
                    MissionStageDirective::ResetCheckpoint {
                        source_ordinal: 3,
                    },
                    MissionStageDirective::IrisWipeLegacyArgument {
                        source_ordinal: 4,
                        source_value: "0.1".to_owned(),
                    },
                    MissionStageDirective::StartCountdown {
                        source_ordinal: 7,
                        sequence_id: "countdown".to_owned(),
                        character_id: Some("homer".to_owned()),
                    },
                    MissionStageDirective::CountdownSequenceEntry {
                        source_ordinal: 8,
                        token: "3".to_owned(),
                        duration_milliseconds: 1000,
                    },
                    MissionStageDirective::StageStartMusicEvent {
                        source_ordinal: 9,
                        event_id: "L7_drama".to_owned(),
                    },
                ],
            ),
            (
                10,
                1,
                true,
                vec![
                    MissionStageDirective::StayInBlack {
                        source_ordinal: 11,
                    },
                    MissionStageDirective::ShowStageComplete {
                        source_ordinal: 12,
                    },
                ],
            ),
        ],
    );
    let objectives = MissionObjectiveSemanticReport::
        from_route_entries_with_parameters_for_tests(vec![
            (
                2,
                0,
                5,
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
                13,
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
                Some(5),
                6,
                "timeout".to_owned(),
                MissionConditionScope::Objective,
                "legacy-mission-condition.timeout.v1",
                MissionConditionParameters::None,
            ),
            (
                10,
                1,
                None,
                14,
                "damage".to_owned(),
                MissionConditionScope::Stage,
                "legacy-mission-condition.damage.v1",
                MissionConditionParameters::DamageLegacyToken {
                    source_token: "neither".to_owned(),
                    code: "legacy-damage-condition-neither-parameter-v1",
                },
            ),
        ]);
    let topology = preflight_mission_authored_stage_topology(&stages)?;
    Ok((stages, objectives, conditions, topology))
}

#[test]
fn joins_source_backed_stage_definition_core() -> Result<(), String> {
    let (stages, objectives, conditions, topology) = reports()?;
    let report = build_definition_core(
        "m1",
        &empty_initialization("m1"),
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
    assert_eq!(first.visual_transition(), MissionStageVisualTransition::Iris);
    assert!(!first.stay_in_black());
    assert!(!first.show_stage_complete());
    assert!(first.objective_npc_waypoints().is_empty());
    assert!(first.pickup_state_props().is_empty());
    let first_markers = first
        .transition_markers()
        .iter()
        .map(|marker| (marker.source_ordinal(), marker.kind()))
        .collect::<Vec<_>>();
    assert_eq!(
        first_markers,
        [(4, MissionStageTransitionMarkerKind::IrisWipe)]
    );
    let countdown = first
        .countdown()
        .ok_or_else(|| "definition-core countdown is missing".to_owned())?;
    assert_eq!(countdown.stage_source_ordinal(), 2);
    assert_eq!(countdown.stage_sequence_ordinal(), 0);
    assert_eq!(countdown.start_source_ordinal(), 7);
    assert_eq!(countdown.sequence_id(), "countdown");
    assert_eq!(countdown.character_id(), Some("homer"));
    let [entry] = countdown.entries() else {
        return Err("definition-core countdown entry count changed".to_owned());
    };
    assert_eq!(entry.source_ordinal(), 8);
    assert_eq!(entry.token(), "3");
    assert_eq!(entry.duration_milliseconds(), 1000);
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
    assert_eq!(condition.source_ordinal(), 6);
    assert_eq!(condition.source_alias(), "timeout");
    assert_eq!(
        condition.schema_id(),
        "legacy-mission-condition.timeout.v1"
    );
    assert_eq!(condition.scope(), MissionConditionScope::Objective);
    assert_eq!(condition.owner_objective_source_ordinal(), Some(5));
    assert_eq!(condition.parameters(), &MissionConditionParameters::None);

    assert_eq!(second.sequence_ordinal(), 1);
    assert_eq!(second.next_authored_sequence_ordinal(), None);
    assert!(second.explicit_final());
    assert_eq!(second.visual_transition(), MissionStageVisualTransition::None);
    assert!(second.stay_in_black());
    assert!(second.show_stage_complete());
    assert!(second.countdown().is_none());
    assert!(second.objective_npc_waypoints().is_empty());
    assert!(second.pickup_state_props().is_empty());
    let second_markers = second
        .transition_markers()
        .iter()
        .map(|marker| (marker.source_ordinal(), marker.kind()))
        .collect::<Vec<_>>();
    assert_eq!(
        second_markers,
        [
            (11, MissionStageTransitionMarkerKind::StayInBlack),
            (12, MissionStageTransitionMarkerKind::ShowStageComplete),
        ]
    );
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
fn rejects_objective_with_wrong_stage_owner() -> Result<(), String> {
    let (stages, _objectives, conditions, topology) = reports()?;
    let objectives =
        MissionObjectiveSemanticReport::from_route_entries_for_tests(vec![
            (2, 1, 4, "goto".to_owned(), Vec::new()),
            (10, 1, 11, "dummy".to_owned(), Vec::new()),
        ]);
    let result = build_definition_core(
        "m1",
        &empty_initialization("m1"),
        &stages,
        &objectives,
        &conditions,
        &topology,
    );
    let Err(error) = result else {
        return Err("wrong objective owner must fail".to_owned());
    };
    assert!(error.to_string().contains("unique root objective"));
    Ok(())
}

#[test]
fn rejects_condition_with_unknown_stage_owner() -> Result<(), String> {
    let (stages, objectives, _conditions, topology) = reports()?;
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
    let result = build_definition_core(
        "m1",
        &empty_initialization("m1"),
        &stages,
        &objectives,
        &conditions,
        &topology,
    );
    let Err(error) = result else {
        return Err("unknown condition owner must fail".to_owned());
    };
    assert!(error.to_string().contains("unknown stage owner"));
    Ok(())
}

#[test]
fn rejects_objective_condition_with_wrong_root_owner() -> Result<(), String> {
    let (stages, objectives, _conditions, topology) = reports()?;
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
    let result = build_definition_core(
        "m1",
        &empty_initialization("m1"),
        &stages,
        &objectives,
        &conditions,
        &topology,
    );
    let Err(error) = result else {
        return Err("wrong condition objective owner must fail".to_owned());
    };
    assert!(error
        .to_string()
        .contains("condition owner disagrees"));
    Ok(())
}

#[test]
fn joins_collectible_waypoint_source_evidence() -> Result<(), String> {
    let stages = MissionStageSemanticReport::from_topology_entries_for_tests(
        vec![(
            2,
            0,
            false,
            vec![MissionStageDirective::Waypoint {
                source_ordinal: 4,
                locator_id: "route_a".to_owned(),
            }],
        )],
    );
    let objectives = MissionObjectiveSemanticReport::
        from_route_entries_for_tests(vec![(
            2,
            0,
            3,
            "dump".to_owned(),
            vec![
                crate::domain::package::MissionObjectiveDirective::Collectible {
                    source_ordinal: 5,
                    locator_id: "cargo_a".to_owned(),
                    drawable_id: None,
                    legacy_arguments: Vec::new(),
                },
                crate::domain::package::MissionObjectiveDirective::
                    BindCollectibleToWaypoint {
                        source_ordinal: 6,
                        collectible_index: 0,
                        waypoint_index: 0,
                    },
            ],
        )]);
    let conditions = MissionConditionSemanticReport::
        from_owned_entries_for_tests(Vec::new());
    let topology = preflight_mission_authored_stage_topology(&stages)?;
    let report = build_definition_core(
        "m1",
        &empty_initialization("m1"),
        &stages,
        &objectives,
        &conditions,
        &topology,
    )
    .map_err(|error| error.to_string())?;
    let [stage] = report.stages() else {
        return Err("definition-core stage count changed".to_owned());
    };
    let [binding] = stage.collectible_waypoints() else {
        return Err(
            "definition-core collectible route count changed".to_owned()
        );
    };
    assert_eq!(binding.stage_source_ordinal(), 2);
    assert_eq!(binding.stage_sequence_ordinal(), 0);
    assert_eq!(binding.objective_source_ordinal(), 3);
    assert_eq!(binding.source_ordinal(), 6);
    assert_eq!(binding.collectible_index(), 0);
    assert_eq!(binding.collectible_source_ordinal(), 5);
    assert_eq!(binding.collectible_locator_id(), "cargo_a");
    assert_eq!(binding.waypoint_index(), 0);
    assert_eq!(binding.waypoint_source_ordinal(), 4);
    assert_eq!(binding.waypoint_locator_id(), "route_a");
    Ok(())
}

#[test]
fn joins_pickup_state_prop_source_evidence() -> Result<(), String> {
    let initialization = empty_initialization("m1");
    let stages = MissionStageSemanticReport::from_topology_entries_for_tests(
        vec![(
            2,
            0,
            false,
            vec![MissionStageDirective::CollectibleStateProp {
                source_ordinal: 4,
                prop_id: "bombbarrel".to_owned(),
                locator_id: "barrel_start".to_owned(),
                source_state: 2,
            }],
        )],
    );
    let objectives = MissionObjectiveSemanticReport::
        from_route_entries_for_tests(vec![(
            2,
            0,
            3,
            "pickupitem".to_owned(),
            vec![crate::domain::package::MissionObjectiveDirective::
                PickupTarget {
                    source_ordinal: 5,
                    target_id: "bombbarrel".to_owned(),
                }],
        )]);
    let conditions = MissionConditionSemanticReport::
        from_owned_entries_for_tests(Vec::new());
    let topology = preflight_mission_authored_stage_topology(&stages)?;
    let report = build_definition_core(
        "m1",
        &initialization,
        &stages,
        &objectives,
        &conditions,
        &topology,
    )
    .map_err(|error| error.to_string())?;
    let [stage] = report.stages() else {
        return Err("definition-core stage count changed".to_owned());
    };
    let [binding] = stage.pickup_state_props() else {
        return Err("definition-core pickup binding count changed".to_owned());
    };
    assert_eq!(binding.owner_stage_source_ordinal(), 2);
    assert_eq!(binding.owner_stage_sequence_ordinal(), 0);
    assert_eq!(binding.owner_objective_source_ordinal(), 3);
    assert_eq!(binding.target_source_ordinal(), 5);
    assert_eq!(binding.target_id(), "bombbarrel");
    assert_eq!(binding.declaration_source_ordinal(), 4);
    assert_eq!(
        binding.declaration_scope(),
        MissionPickupStatePropScope::Stage {
            source_ordinal: 2,
            sequence_ordinal: 0,
        }
    );
    assert_eq!(binding.locator_id(), "barrel_start");
    assert_eq!(binding.source_state(), 2);
    Ok(())
}

#[test]
fn joins_mission_scope_pickup_state_prop_source_evidence(
) -> Result<(), String> {
    let initialization = MissionInitializationReport::from_directives_for_tests(
        "m1",
        vec![MissionInitializationDirective::CollectibleStateProp {
            source_ordinal: 1,
            prop_id: "bombbarrel".to_owned(),
            locator_id: "mission_barrel".to_owned(),
            source_state: 3,
        }],
    );
    let stages = MissionStageSemanticReport::from_topology_entries_for_tests(
        vec![(2, 0, false, Vec::new())],
    );
    let objectives = MissionObjectiveSemanticReport::
        from_route_entries_for_tests(vec![(
            2,
            0,
            3,
            "pickupitem".to_owned(),
            vec![crate::domain::package::MissionObjectiveDirective::
                PickupTarget {
                    source_ordinal: 4,
                    target_id: "bombbarrel".to_owned(),
                }],
        )]);
    let conditions = MissionConditionSemanticReport::
        from_owned_entries_for_tests(Vec::new());
    let topology = preflight_mission_authored_stage_topology(&stages)?;
    let report = build_definition_core(
        "m1",
        &initialization,
        &stages,
        &objectives,
        &conditions,
        &topology,
    )
    .map_err(|error| error.to_string())?;
    let [stage] = report.stages() else {
        return Err("definition-core stage count changed".to_owned());
    };
    let [binding] = stage.pickup_state_props() else {
        return Err("definition-core pickup binding count changed".to_owned());
    };
    assert_eq!(
        binding.declaration_scope(),
        MissionPickupStatePropScope::Mission
    );
    assert_eq!(binding.locator_id(), "mission_barrel");
    assert_eq!(binding.source_state(), 3);
    Ok(())
}

#[test]
fn renders_definition_core_as_stable_versioned_json() -> Result<(), String> {
    let (stages, objectives, conditions, topology) = reports()?;
    let report = build_definition_core(
        "m1",
        &empty_initialization("m1"),
        &stages,
        &objectives,
        &conditions,
        &topology,
    )
    .map_err(|error| error.to_string())?;
    let first = render_definition_core("script-test-source", &report)
        .map_err(|error| error.to_string())?;
    let second = render_definition_core("script-test-source", &report)
        .map_err(|error| error.to_string())?;
    assert_eq!(first, second);
    assert!(first.ends_with('\n'));
    let value = serde_json::from_str::<serde_json::Value>(&first)
        .map_err(|error| error.to_string())?;
    assert_eq!(
        value.get("schema").and_then(serde_json::Value::as_str),
        Some("shar-schoenwald.mission-definition-core.v3")
    );
    assert_eq!(
        value.get("source_id").and_then(serde_json::Value::as_str),
        Some("script-test-source")
    );
    assert_eq!(
        value.get("mission_id").and_then(serde_json::Value::as_str),
        Some("m1")
    );
    let stages = value
        .get("stages")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "rendered stages disappeared".to_owned())?;
    assert_eq!(stages.len(), 2);
    let first_stage = stages
        .first()
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "rendered first stage disappeared".to_owned())?;
    assert_eq!(
        first_stage
            .get("next_authored_sequence_ordinal")
            .and_then(serde_json::Value::as_u64),
        Some(1)
    );
    assert_eq!(
        first_stage
            .get("visual_transition")
            .and_then(serde_json::Value::as_str),
        Some("iris")
    );
    let music_events = first_stage
        .get("stage_start_music_events")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "rendered stage music events disappeared".to_owned())?;
    assert_eq!(music_events.len(), 1);
    let music_event = music_events
        .first()
        .ok_or_else(|| "rendered stage music event disappeared".to_owned())?;
    assert_eq!(
        music_event
            .get("channel")
            .and_then(serde_json::Value::as_str),
        Some("mission-drama")
    );
    assert_eq!(
        music_event
            .get("key_transform")
            .and_then(serde_json::Value::as_str),
        Some("legacy-case-insensitive-key32")
    );
    assert_eq!(
        music_event
            .get("event_id")
            .and_then(serde_json::Value::as_str),
        Some("L7_drama")
    );
    let objective = first_stage
        .get("objective")
        .and_then(serde_json::Value::as_object)
        .ok_or_else(|| "rendered objective disappeared".to_owned())?;
    assert_eq!(
        objective
            .get("canonical_kind")
            .and_then(serde_json::Value::as_str),
        Some("travel")
    );
    let conditions = first_stage
        .get("conditions")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "rendered conditions disappeared".to_owned())?;
    assert_eq!(conditions.len(), 1);
    let condition = conditions
        .first()
        .ok_or_else(|| "rendered condition disappeared".to_owned())?;
    assert_eq!(
        condition
            .get("violation_effect")
            .and_then(serde_json::Value::as_str),
        Some("stage-failure")
    );
    Ok(())
}

#[test]
fn rejects_noncanonical_definition_source_identity() -> Result<(), String> {
    let (stages, objectives, conditions, topology) = reports()?;
    let report = build_definition_core(
        "m1",
        &empty_initialization("m1"),
        &stages,
        &objectives,
        &conditions,
        &topology,
    )
    .map_err(|error| error.to_string())?;
    for source_id in [
        "../script",
        "Script-one",
        "script_one",
        "script.one",
        "script--one",
    ] {
        let result = render_definition_core(source_id, &report);
        let Err(error) = result else {
            return Err(format!(
                "noncanonical source id must fail: {source_id}"
            ));
        };
        assert!(error.to_string().contains("not canonical"));
    }
    Ok(())
}
