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
//   - Unit evidence for pickup-target state-prop cross-scope binding.
// - Must-Not:
//   - Infer state-prop lifetime or pickup runtime behavior.
// - Allows:
//   - Verify mission/stage declarations and fail-closed reference matching.
// - Split-When:
//   - State-prop lifecycle requires independent runtime fixtures.
// - Merge-When:
//   - Complete objective graph tests own this exact cross-reference.
// - Summary:
//   - Pickup-target state-prop binding unit tests.
// - Description:
//   - Proves source-wide authored identity binding across mission and stage.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Missing, ambiguous, and forward references fail closed.
//

//! Unit evidence for pickup-target state-prop bindings.

use super::*;

fn objective(target_ordinal: usize) -> MissionObjectiveSemanticReport {
    MissionObjectiveSemanticReport::from_route_entries_for_tests(vec![(
        7,
        0,
        8,
        "pickupitem".to_owned(),
        vec![MissionObjectiveDirective::PickupTarget {
            source_ordinal: target_ordinal,
            target_id: "bombbarrel".to_owned(),
        }],
    )])
}

fn mission_prop(source_ordinal: usize) -> MissionInitializationReport {
    MissionInitializationReport::from_directives_for_tests(
        "m6",
        vec![MissionInitializationDirective::CollectibleStateProp {
            source_ordinal,
            prop_id: "bombbarrel".to_owned(),
            locator_id: "m6_barrel1".to_owned(),
            source_state: 2,
        }],
    )
}

fn stage_prop(source_ordinal: usize) -> MissionStageSemanticReport {
    MissionStageSemanticReport::from_topology_entries_for_tests(vec![(
        2,
        0,
        false,
        vec![MissionStageDirective::CollectibleStateProp {
            source_ordinal,
            prop_id: "bombbarrel".to_owned(),
            locator_id: "m3_barrel_1".to_owned(),
            source_state: 2,
        }],
    )])
}

fn empty_stages() -> MissionStageSemanticReport {
    MissionStageSemanticReport::from_topology_entries_for_tests(vec![(
        2,
        0,
        false,
        vec![],
    )])
}

#[test]
fn binds_mission_scope_state_prop() -> Result<(), String> {
    let report = preflight_mission_pickup_state_props(
        &mission_prop(7),
        &empty_stages(),
        &objective(61),
    )?;
    let [binding] = report.bindings() else {
        return Err("pickup binding count drifted".to_owned());
    };
    assert_eq!(binding.owner_stage_source_ordinal(), 7);
    assert_eq!(binding.owner_stage_sequence_ordinal(), 0);
    assert_eq!(binding.owner_objective_source_ordinal(), 8);
    assert_eq!(binding.target_source_ordinal(), 61);
    assert_eq!(binding.target_id(), "bombbarrel");
    assert_eq!(binding.declaration_source_ordinal(), 7);
    assert_eq!(
        binding.declaration_scope(),
        MissionPickupStatePropScope::Mission,
    );
    assert_eq!(binding.locator_id(), "m6_barrel1");
    assert_eq!(binding.source_state(), 2);
    Ok(())
}

#[test]
fn binds_stage_scope_state_prop() -> Result<(), String> {
    let initialization =
        MissionInitializationReport::from_directives_for_tests("m3", vec![]);
    let report = preflight_mission_pickup_state_props(
        &initialization,
        &stage_prop(10),
        &objective(15),
    )?;
    let [binding] = report.bindings() else {
        return Err("stage pickup binding count drifted".to_owned());
    };
    assert_eq!(
        binding.declaration_scope(),
        MissionPickupStatePropScope::Stage {
            source_ordinal: 2,
            sequence_ordinal: 0,
        }
    );
    assert_eq!(binding.locator_id(), "m3_barrel_1");
    Ok(())
}

#[test]
fn rejects_missing_or_forward_state_prop() {
    let initialization =
        MissionInitializationReport::from_directives_for_tests("m3", vec![]);
    let missing = preflight_mission_pickup_state_props(
        &initialization,
        &empty_stages(),
        &objective(15),
    )
    .expect_err("missing pickup state prop must fail");
    assert!(missing.contains("no unique prior state-prop declaration"));

    let forward = preflight_mission_pickup_state_props(
        &mission_prop(20),
        &empty_stages(),
        &objective(15),
    )
    .expect_err("forward pickup state prop must fail");
    assert!(forward.contains("no unique prior state-prop declaration"));
}

#[test]
fn rejects_ambiguous_prior_state_prop() {
    let initialization = MissionInitializationReport::from_directives_for_tests(
        "m3",
        vec![MissionInitializationDirective::CollectibleStateProp {
            source_ordinal: 7,
            prop_id: "bombbarrel".to_owned(),
            locator_id: "mission_barrel".to_owned(),
            source_state: 2,
        }],
    );
    let error = preflight_mission_pickup_state_props(
        &initialization,
        &stage_prop(10),
        &objective(15),
    )
    .expect_err("ambiguous pickup state prop must fail");
    assert!(error.contains("no unique prior state-prop declaration"));
}

#[test]
fn rejects_target_before_owning_objective() {
    let initialization =
        MissionInitializationReport::from_directives_for_tests("m3", vec![]);
    let error = preflight_mission_pickup_state_props(
        &initialization,
        &empty_stages(),
        &objective(7),
    )
    .expect_err("pickup target before objective must fail");
    assert!(error.contains("precedes its owning objective"));
}
