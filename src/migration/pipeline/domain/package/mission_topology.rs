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
//   - Authored mission-stage order and explicit terminal/final invariants.
// - Must-Not:
//   - Treat authored adjacency as a runtime success or failure transition.
//   - Infer retry, rollback, recovery, or mission-completion behavior.
// - Allows:
//   - Expose the next authored stage and explicit final/terminal markers.
//   - Reject contradictory authored ordering and terminal placement.
// - Split-When:
//   - Runtime successor or recovery graphs gain independent source authority.
// - Merge-When:
//   - Final mission graph compilation owns this exact authored ordering.
// - Summary:
//   - Authored mission-stage topology preflight.
// - Description:
//   - Preserves source stage adjacency without promoting it to runtime flow.
// - Usage:
//   - Runs after typed stage semantics and transition classification.
// - Defaults:
//   - Missing final markers are accepted; contradictory markers fail closed.
//

//! Source-backed authored mission-stage topology.

use super::{
    MissionStageKind, MissionStageSemanticReport, MissionStageTerminalOutcome,
    preflight_mission_stage_transitions,
};

/// One stage's position in the authored mission-stage sequence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissionAuthoredStageTopologyBinding {
    source_ordinal: usize,
    sequence_ordinal: usize,
    next_authored_sequence_ordinal: Option<usize>,
    explicit_final: bool,
    terminal: MissionStageTerminalOutcome,
}

impl MissionAuthoredStageTopologyBinding {
    /// Return the source `AddStage` ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the dense authored stage ordinal.
    #[must_use]
    pub const fn sequence_ordinal(&self) -> usize {
        self.sequence_ordinal
    }

    /// Return the next authored stage ordinal, without runtime semantics.
    #[must_use]
    pub const fn next_authored_sequence_ordinal(&self) -> Option<usize> {
        self.next_authored_sequence_ordinal
    }

    /// Return whether the source explicitly marked this stage `final`.
    #[must_use]
    pub const fn explicit_final(&self) -> bool {
        self.explicit_final
    }

    /// Return the explicit level/game terminal marker classification.
    #[must_use]
    pub const fn terminal(&self) -> MissionStageTerminalOutcome {
        self.terminal
    }
}

/// Complete authored topology for one normalized selected mission source.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionAuthoredStageTopologyReport {
    stages: Vec<MissionAuthoredStageTopologyBinding>,
}

impl MissionAuthoredStageTopologyReport {
    /// Return stage topology bindings in authored order.
    #[must_use]
    pub fn stages(&self) -> &[MissionAuthoredStageTopologyBinding] {
        &self.stages
    }
}

/// Compile source stage adjacency and explicit final/terminal invariants.
///
/// Authored adjacency is retained as evidence only. This function does not
/// claim that completing one stage causes the runtime to enter its neighbor.
///
/// # Errors
///
/// Fails when stage ordinals are not dense and increasing, more than one
/// explicit `final` marker exists, an explicit `final` marker is not last, or
/// an explicit level/game terminal marker appears before the last stage.
pub fn preflight_mission_authored_stage_topology(
    semantics: &MissionStageSemanticReport,
) -> Result<MissionAuthoredStageTopologyReport, String> {
    let transitions = preflight_mission_stage_transitions(semantics);
    if transitions.stages().len() != semantics.stages().len() {
        return Err("mission stage transition count drifted".to_owned());
    }

    let last_index = semantics.stages().len().checked_sub(1);
    let mut previous_source_ordinal = None;
    let mut explicit_final_index = None;
    let mut stages = Vec::with_capacity(semantics.stages().len());

    for (index, (stage, transition)) in semantics
        .stages()
        .iter()
        .zip(transitions.stages())
        .enumerate()
    {
        if stage.sequence_ordinal() != index
            || transition.sequence_ordinal() != index
            || transition.source_ordinal() != stage.source_ordinal()
        {
            return Err("mission stage authored order is not dense".to_owned());
        }
        if previous_source_ordinal.is_some_and(|ordinal| {
            stage.source_ordinal() <= ordinal
        }) {
            return Err(
                "mission stage source ordinals are not increasing".to_owned(),
            );
        }
        previous_source_ordinal = Some(stage.source_ordinal());

        let explicit_final = matches!(
            stage.kind(),
            MissionStageKind::Standard {
                final_stage: true,
                ..
            }
        );
        if explicit_final {
            if explicit_final_index.replace(index).is_some() {
                return Err(
                    "mission has more than one explicit final stage".to_owned(),
                );
            }
            if Some(index) != last_index {
                return Err(
                    "mission explicit final stage is not authored last"
                        .to_owned(),
                );
            }
        }
        if transition.terminal() != MissionStageTerminalOutcome::None
            && Some(index) != last_index
        {
            return Err(
                "mission explicit terminal stage is not authored last"
                    .to_owned(),
            );
        }

        stages.push(MissionAuthoredStageTopologyBinding {
            source_ordinal: stage.source_ordinal(),
            sequence_ordinal: index,
            next_authored_sequence_ordinal:
                (index + 1 < semantics.stages().len()).then_some(index + 1),
            explicit_final,
            terminal: transition.terminal(),
        });
    }

    Ok(MissionAuthoredStageTopologyReport { stages })
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_topology/tests.rs"]
mod tests;
