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
//   - Source-runtime condition-violation consumer binding.
// - Must-Not:
//   - Infer retry, rollback, checkpoint restore, or mission progression.
// - Allows:
//   - Preserve the reviewed runtime fact that a violated condition fails stage.
// - Split-When:
//   - Another condition result gains a distinct runtime consumer.
// - Merge-When:
//   - Condition semantic compilation owns this exact consumer boundary.
// - Summary:
//   - Mission condition violation-effect binding.
// - Description:
//   - Projects validated condition owners to the reviewed stage-failure effect.
// - Usage:
//   - Runs after condition semantic ownership is validated.
// - Defaults:
//   - Missing, duplicate, or non-monotonic condition owners fail closed.
//

//! Source-runtime mission-condition violation effects.

use std::collections::BTreeSet;

use super::MissionConditionSemanticReport;

/// Runtime effect consumed when one reviewed mission condition is violated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionConditionViolationEffect {
    /// The owning mission stage enters its failure state.
    StageFailure,
}

/// One semantic condition bound to its reviewed runtime violation effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissionConditionViolationBinding {
    owner_stage_source_ordinal: usize,
    owner_stage_sequence_ordinal: usize,
    owner_objective_source_ordinal: Option<usize>,
    source_ordinal: usize,
    effect: MissionConditionViolationEffect,
}

impl MissionConditionViolationBinding {
    /// Return the source `AddStage` ordinal owning the condition.
    #[must_use]
    pub const fn owner_stage_source_ordinal(&self) -> usize {
        self.owner_stage_source_ordinal
    }

    /// Return the dense authored stage ordinal owning the condition.
    #[must_use]
    pub const fn owner_stage_sequence_ordinal(&self) -> usize {
        self.owner_stage_sequence_ordinal
    }

    /// Return the root objective owner when the condition is objective-scoped.
    #[must_use]
    pub const fn owner_objective_source_ordinal(&self) -> Option<usize> {
        self.owner_objective_source_ordinal
    }

    /// Return the source `AddCondition` ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the reviewed runtime effect of a violated condition.
    #[must_use]
    pub const fn effect(&self) -> MissionConditionViolationEffect {
        self.effect
    }
}

/// Complete violation-effect bindings for one selected mission source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionConditionViolationReport {
    bindings: Vec<MissionConditionViolationBinding>,
}

impl MissionConditionViolationReport {
    /// Return violation-effect bindings in source order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionConditionViolationBinding] {
        &self.bindings
    }
}

/// Bind each semantic condition to the source-runtime stage-failure consumer.
///
/// # Errors
///
/// Returns an error if condition ownership is duplicated or source order
/// drifts.
pub fn preflight_mission_condition_violations(
    conditions: &MissionConditionSemanticReport,
) -> Result<MissionConditionViolationReport, String> {
    let mut seen = BTreeSet::new();
    let mut previous_source_ordinal = None;
    let mut bindings = Vec::with_capacity(conditions.conditions().len());
    for condition in conditions.conditions() {
        let key = (
            condition.owner_stage_source_ordinal(),
            condition.owner_stage_sequence_ordinal(),
            condition.source_ordinal(),
        );
        if !seen.insert(key) {
            return Err(
                "mission condition violation owner is duplicated".to_owned(),
            );
        }
        if previous_source_ordinal
            .is_some_and(|previous| condition.source_ordinal() <= previous)
        {
            return Err(
                "mission condition violation source order is not increasing"
                    .to_owned(),
            );
        }
        previous_source_ordinal = Some(condition.source_ordinal());
        bindings.push(MissionConditionViolationBinding {
            owner_stage_source_ordinal: condition.owner_stage_source_ordinal(),
            owner_stage_sequence_ordinal:
                condition.owner_stage_sequence_ordinal(),
            owner_objective_source_ordinal:
                condition.owner_objective_source_ordinal(),
            source_ordinal: condition.source_ordinal(),
            effect: MissionConditionViolationEffect::StageFailure,
        });
    }
    Ok(MissionConditionViolationReport { bindings })
}

#[cfg(test)]
// jig-ignore-next-line: cfg test path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/domain/package/mission_condition/violation_tests.rs"]
mod tests;
