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
//   - Mission condition violation-effect unit tests.
// - Must-Not:
//   - Infer retry, rollback, or recovery behavior.
// - Allows:
//   - Prove exact condition ownership and stage-failure projection.
// - Split-When:
//   - Another violation consumer gains independent fixtures.
// - Merge-When:
//   - Condition runtime-binding tests own the same contract.
// - Summary:
//   - Mission condition violation-effect tests.
// - Description:
//   - Locks the reviewed condition-violation consumer to stage failure only.
// - Usage:
//   - Included by the condition violation domain module under cfg(test).
// - Defaults:
//   - Every reviewed condition maps one-to-one to stage failure.
//

//! Mission condition violation-effect tests.

use super::{
    MissionConditionViolationEffect, preflight_mission_condition_violations,
};
use crate::domain::{MissionConditionScope, MissionConditionSemanticReport};

#[test]
fn binds_each_condition_to_stage_failure_without_recovery_policy() {
    let report = MissionConditionSemanticReport::from_owned_entries_for_tests(
        vec![
            (
                2,
                0,
                None,
                4,
                "timeout".to_owned(),
                MissionConditionScope::Stage,
                "legacy-mission-condition.timeout.v1",
            ),
            (
                8,
                1,
                Some(10),
                11,
                "damage".to_owned(),
                MissionConditionScope::Objective,
                "legacy-mission-condition.damage.v1",
            ),
        ],
    );
    let bindings = preflight_mission_condition_violations(&report)
        .expect("reviewed condition consumers must bind");
    assert_eq!(bindings.bindings().len(), 2);
    assert_eq!(
        bindings.bindings()[0].effect(),
        MissionConditionViolationEffect::StageFailure,
    );
    assert_eq!(bindings.bindings()[0].owner_stage_source_ordinal(), 2);
    assert_eq!(bindings.bindings()[0].owner_stage_sequence_ordinal(), 0);
    assert_eq!(bindings.bindings()[0].owner_objective_source_ordinal(), None);
    assert_eq!(bindings.bindings()[0].source_ordinal(), 4);
    assert_eq!(
        bindings.bindings()[1].owner_objective_source_ordinal(),
        Some(10),
    );
}

#[test]
fn empty_condition_report_has_no_violation_bindings() {
    let report = MissionConditionSemanticReport::from_owned_entries_for_tests(
        Vec::new(),
    );
    let bindings = preflight_mission_condition_violations(&report)
        .expect("empty condition set must remain empty");
    assert!(bindings.bindings().is_empty());
}
