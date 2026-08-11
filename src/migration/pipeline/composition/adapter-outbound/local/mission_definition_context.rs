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
//   - Lossless join of source-backed mission stage definition-core evidence.
// - Must-Not:
//   - Infer runtime success, failure, retry, rollback, or recovery transitions.
//   - Emit final Unreal mission assets or invent objective policy identities.
// - Allows:
//   - Join authored topology, objective identity, and condition ownership.
// - Split-When:
//   - Runtime transition authority or final asset emission becomes available.
// - Merge-When:
//   - Final mission definition compilation owns this exact preflight join.
// - Summary:
//   - Mission definition-core preflight context.
// - Description:
//   - Proves stage/objective/condition ownership before final asset
//     compilation.
// - Usage:
//   - Runs after typed mission scope, stage, objective, and condition
//     preflights.
// - Defaults:
//   - Missing, duplicate, mismatched, or unowned semantic rows fail closed.
//

//! Lossless preflight join for source-backed mission definition-core evidence.

use std::collections::BTreeSet;

use crate::domain::package::{
    MissionAuthoredStageTopologyReport, MissionCollectibleWaypointBinding,
    MissionCountdownBinding, MissionObjectiveNpcWaypointBinding,
};
use crate::domain::{
    MissionConditionParameters, MissionConditionScope,
    MissionConditionSemanticReport, MissionObjectiveParameters,
    MissionObjectiveSemanticReport, MissionScopeReport, MissionStageKind,
    MissionStageSemanticReport, MissionStageTerminalOutcome,
    MissionStageTransitionMarker, MissionStageVisualTransition, PipelineError,
    PipelineOutcome, preflight_mission_collectible_waypoints,
    preflight_mission_countdowns, preflight_mission_objective_npc_waypoints,
    preflight_mission_stage_transitions,
};

/// One condition identity retained under its exact owning stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionDefinitionConditionCoreBinding {
    source_ordinal: usize,
    source_alias: String,
    schema_id: &'static str,
    scope: MissionConditionScope,
    owner_objective_source_ordinal: Option<usize>,
    parameters: MissionConditionParameters,
}

/// One stage's source-backed definition core, without runtime transitions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionDefinitionStageCoreBinding {
    stage_source_ordinal: usize,
    sequence_ordinal: usize,
    kind: MissionStageKind,
    next_authored_sequence_ordinal: Option<usize>,
    checkpoint_source_ordinal: Option<usize>,
    explicit_final: bool,
    terminal: MissionStageTerminalOutcome,
    visual_transition: MissionStageVisualTransition,
    stay_in_black: bool,
    show_stage_complete: bool,
    transition_markers: Vec<MissionStageTransitionMarker>,
    countdown: Option<MissionCountdownBinding>,
    collectible_waypoints: Vec<MissionCollectibleWaypointBinding>,
    objective_npc_waypoints: Vec<MissionObjectiveNpcWaypointBinding>,
    objective_source_ordinal: usize,
    objective_source_alias: String,
    objective_canonical_kind: Option<&'static str>,
    objective_unavailable_code: Option<&'static str>,
    objective_parameters: MissionObjectiveParameters,
    conditions: Vec<MissionDefinitionConditionCoreBinding>,
}

/// Joined definition-core evidence for one selected mission source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionDefinitionCoreReport {
    mission_id: String,
    stages: Vec<MissionDefinitionStageCoreBinding>,
}

impl MissionDefinitionCoreReport {
    fn validate(&self) -> PipelineOutcome<()> {
        if self.mission_id.is_empty() {
            return Err(PipelineError::new(
                "mission definition core has empty mission identity",
            ));
        }
        for (index, stage) in self.stages.iter().enumerate() {
            if stage.sequence_ordinal != index {
                return Err(PipelineError::new(
                    "mission definition core stage order is not dense",
                ));
            }
            let expected_next =
                (index + 1 < self.stages.len()).then_some(index + 1);
            if stage.next_authored_sequence_ordinal != expected_next {
                return Err(PipelineError::new(
                    "mission definition core authored neighbor drifted",
                ));
            }
            if stage
                .checkpoint_source_ordinal
                .is_some_and(|ordinal| ordinal <= stage.stage_source_ordinal)
            {
                return Err(PipelineError::new(
                    "mission definition core checkpoint precedes its stage",
                ));
            }
            let kind_final = matches!(
                stage.kind,
                MissionStageKind::Standard {
                    final_stage: true,
                    ..
                }
            );
            if kind_final != stage.explicit_final {
                return Err(PipelineError::new(
                    "mission definition core final marker disagrees with stage",
                ));
            }
            match stage.terminal {
                MissionStageTerminalOutcome::None
                | MissionStageTerminalOutcome::ChapterTransition
                | MissionStageTerminalOutcome::GameCompletion => {},
            }
            if let Some(countdown) = &stage.countdown {
                if countdown.stage_source_ordinal()
                    != stage.stage_source_ordinal
                    || countdown.stage_sequence_ordinal()
                        != stage.sequence_ordinal
                    || countdown.start_source_ordinal()
                        <= stage.stage_source_ordinal
                {
                    return Err(PipelineError::new(
                        "mission definition core countdown owner drifted",
                    ));
                }
            }
            for binding in &stage.collectible_waypoints {
                if binding.stage_source_ordinal() != stage.stage_source_ordinal
                    || binding.stage_sequence_ordinal()
                        != stage.sequence_ordinal
                    || binding.objective_source_ordinal()
                        != stage.objective_source_ordinal
                {
                    return Err(PipelineError::new(
                        concat!(
                            "mission definition core collectible waypoint ",
                            "owner drifted"
                        ),
                    ));
                }
            }
            for waypoint in &stage.objective_npc_waypoints {
                if waypoint.owner_stage_source_ordinal()
                    != stage.stage_source_ordinal
                    || waypoint.owner_stage_sequence_ordinal()
                        != stage.sequence_ordinal
                    || waypoint.objective_source_ordinal()
                        != stage.objective_source_ordinal
                {
                    return Err(PipelineError::new(
                        "mission definition core NPC waypoint owner drifted",
                    ));
                }
            }
            if stage.objective_source_ordinal <= stage.stage_source_ordinal
                || stage.objective_source_alias.is_empty()
            {
                return Err(PipelineError::new(
                    "mission definition core objective identity is malformed",
                ));
            }
            if stage.objective_canonical_kind.is_some()
                == stage.objective_unavailable_code.is_some()
            {
                return Err(PipelineError::new(
                    concat!(
                        "mission definition core objective mapping is not ",
                        "exclusive"
                    ),
                ));
            }
            let mut previous_condition = None;
            for condition in &stage.conditions {
                if condition.source_ordinal <= stage.stage_source_ordinal
                    || condition.source_alias.is_empty()
                    || condition.schema_id.is_empty()
                    || previous_condition.is_some_and(|ordinal| {
                        condition.source_ordinal <= ordinal
                    })
                {
                    return Err(PipelineError::new(
                        concat!(
                            "mission definition core condition identity is ",
                            "malformed"
                        ),
                    ));
                }
                match condition.scope {
                    MissionConditionScope::Stage => {
                        if condition.owner_objective_source_ordinal.is_some() {
                            return Err(PipelineError::new(
                                concat!(
                                    "stage condition unexpectedly owns an ",
                                    "objective"
                                ),
                            ));
                        }
                    },
                    MissionConditionScope::Objective => {
                        if condition.owner_objective_source_ordinal
                            != Some(stage.objective_source_ordinal)
                        {
                            return Err(PipelineError::new(
                                concat!(
                                    "objective condition owner disagrees with ",
                                    "stage root objective"
                                ),
                            ));
                        }
                    },
                }
                previous_condition = Some(condition.source_ordinal);
            }
        }
        Ok(())
    }

    #[cfg(test)]
    pub(super) fn mission_id(&self) -> &str {
        &self.mission_id
    }

    #[cfg(test)]
    pub(super) fn stages(&self) -> &[MissionDefinitionStageCoreBinding] {
        &self.stages
    }

    #[cfg(test)]
    pub(super) fn has_only_mapped_objectives(&self) -> bool {
        self.stages
            .iter()
            .all(|stage| stage.objective_canonical_kind.is_some())
    }
}

impl MissionDefinitionStageCoreBinding {
    #[cfg(test)]
    pub(super) const fn stage_source_ordinal(&self) -> usize {
        self.stage_source_ordinal
    }

    #[cfg(test)]
    pub(super) const fn sequence_ordinal(&self) -> usize {
        self.sequence_ordinal
    }

    #[cfg(test)]
    pub(super) const fn checkpoint_source_ordinal(&self) -> Option<usize> {
        self.checkpoint_source_ordinal
    }

    #[cfg(test)]
    pub(super) const fn next_authored_sequence_ordinal(&self) -> Option<usize> {
        self.next_authored_sequence_ordinal
    }

    #[cfg(test)]
    pub(super) const fn explicit_final(&self) -> bool {
        self.explicit_final
    }

    #[cfg(test)]
    pub(super) const fn terminal(&self) -> MissionStageTerminalOutcome {
        self.terminal
    }

    #[cfg(test)]
    pub(super) const fn visual_transition(
        &self,
    ) -> MissionStageVisualTransition {
        self.visual_transition
    }

    #[cfg(test)]
    pub(super) const fn stay_in_black(&self) -> bool {
        self.stay_in_black
    }

    #[cfg(test)]
    pub(super) const fn show_stage_complete(&self) -> bool {
        self.show_stage_complete
    }

    #[cfg(test)]
    pub(super) fn transition_markers(&self) -> &[MissionStageTransitionMarker] {
        &self.transition_markers
    }

    #[cfg(test)]
    pub(super) const fn countdown(&self) -> Option<&MissionCountdownBinding> {
        self.countdown.as_ref()
    }

    #[cfg(test)]
    pub(super) fn collectible_waypoints(
        &self,
    ) -> &[MissionCollectibleWaypointBinding] {
        &self.collectible_waypoints
    }

    #[cfg(test)]
    pub(super) fn objective_npc_waypoints(
        &self,
    ) -> &[MissionObjectiveNpcWaypointBinding] {
        &self.objective_npc_waypoints
    }

    #[cfg(test)]
    pub(super) fn objective_source_alias(&self) -> &str {
        &self.objective_source_alias
    }

    #[cfg(test)]
    pub(super) const fn objective_canonical_kind(
        &self,
    ) -> Option<&'static str> {
        self.objective_canonical_kind
    }

    #[cfg(test)]
    pub(super) const fn objective_parameters(
        &self,
    ) -> &MissionObjectiveParameters {
        &self.objective_parameters
    }

    #[cfg(test)]
    pub(super) fn conditions(
        &self,
    ) -> &[MissionDefinitionConditionCoreBinding] {
        &self.conditions
    }
}

impl MissionDefinitionConditionCoreBinding {
    #[cfg(test)]
    pub(super) const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    #[cfg(test)]
    pub(super) fn source_alias(&self) -> &str {
        &self.source_alias
    }

    #[cfg(test)]
    pub(super) const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    #[cfg(test)]
    pub(super) const fn scope(&self) -> MissionConditionScope {
        self.scope
    }

    #[cfg(test)]
    pub(super) const fn owner_objective_source_ordinal(&self) -> Option<usize> {
        self.owner_objective_source_ordinal
    }

    #[cfg(test)]
    pub(super) const fn parameters(&self) -> &MissionConditionParameters {
        &self.parameters
    }
}

/// Join one normalized mission source into source-backed definition-core rows.
///
/// # Errors
///
/// Returns an error when source scope cardinality or semantic ownership drifts.
pub(super) fn preflight_mission_definition_core(
    scopes: &MissionScopeReport,
    stages: &MissionStageSemanticReport,
    objectives: &MissionObjectiveSemanticReport,
    conditions: &MissionConditionSemanticReport,
    topology: &MissionAuthoredStageTopologyReport,
) -> PipelineOutcome<Option<MissionDefinitionCoreReport>> {
    match scopes.missions() {
        [] => {
            if !stages.stages().is_empty()
                || !objectives.objectives().is_empty()
                || !conditions.conditions().is_empty()
                || !topology.stages().is_empty()
            {
                return Err(PipelineError::new(
                    concat!(
                        "mission definition core has semantics without ",
                        "mission scope"
                    ),
                ));
            }
            Ok(None)
        },
        [mission] => build_definition_core(
            mission.source_mission_id(),
            stages,
            objectives,
            conditions,
            topology,
        )
        .map(Some),
        _ => Err(PipelineError::new(
            "mission definition core source has multiple selected missions",
        )),
    }
}

fn build_definition_core(
    mission_id: &str,
    stages: &MissionStageSemanticReport,
    objectives: &MissionObjectiveSemanticReport,
    conditions: &MissionConditionSemanticReport,
    topology: &MissionAuthoredStageTopologyReport,
) -> PipelineOutcome<MissionDefinitionCoreReport> {
    if stages.stages().len() != topology.stages().len()
        || stages.stages().len() != objectives.objectives().len()
    {
        return Err(PipelineError::new(
            "mission definition core stage/objective/topology count drifted",
        ));
    }

    let transitions = preflight_mission_stage_transitions(stages);
    if transitions.stages().len() != stages.stages().len() {
        return Err(PipelineError::new(
            "mission definition core transition count drifted",
        ));
    }

    let countdowns = preflight_mission_countdowns(stages).map_err(|error| {
        PipelineError::new(format!(
            "mission definition core countdown preflight failed: {error}"
        ))
    })?;

    let stage_keys = stages
        .stages()
        .iter()
        .map(|stage| (stage.source_ordinal(), stage.sequence_ordinal()))
        .collect::<BTreeSet<_>>();
    if stage_keys.len() != stages.stages().len() {
        return Err(PipelineError::new(
            "mission definition core stage identity is duplicated",
        ));
    }
    for stage in stages.stages() {
        let objective_count = objectives
            .objectives()
            .iter()
            .filter(|objective| {
                objective.owner_stage_source_ordinal() == stage.source_ordinal()
                    && objective.owner_stage_sequence_ordinal()
                        == stage.sequence_ordinal()
            })
            .count();
        if objective_count != 1 {
            return Err(PipelineError::new(
                "mission definition core stage has no unique root objective",
            ));
        }
    }

    for stage in stages.stages() {
        let objective_count = objectives
            .objectives()
            .iter()
            .filter(|objective| {
                objective.owner_stage_source_ordinal() == stage.source_ordinal()
                    && objective.owner_stage_sequence_ordinal()
                        == stage.sequence_ordinal()
            })
            .count();
        if objective_count != 1 {
            return Err(PipelineError::new(
                "mission definition core stage has no unique root objective",
            ));
        }
    }

    for condition in conditions.conditions() {
        let owner = (
            condition.owner_stage_source_ordinal(),
            condition.owner_stage_sequence_ordinal(),
        );
        if !stage_keys.contains(&owner) {
            return Err(PipelineError::new(
                "mission definition core condition has unknown stage owner",
            ));
        }
    }

    let collectible_waypoints =
        preflight_mission_collectible_waypoints(stages, objectives)
            .map_err(|error| {
                PipelineError::new(format!(
                    concat!(
                        "mission definition core collectible waypoint ",
                        "preflight failed: {}"
                    ),
                    error
                ))
            })?;

    let objective_npc_waypoints =
        preflight_mission_objective_npc_waypoints(objectives)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission definition core NPC waypoint failed: {error}"
                ))
            })?;

    let mut result = Vec::with_capacity(stages.stages().len());
    for stage in stages.stages() {
        let key = (stage.source_ordinal(), stage.sequence_ordinal());
        let matching_topology = topology
            .stages()
            .iter()
            .filter(|item| {
                item.source_ordinal() == key.0
                    && item.sequence_ordinal() == key.1
            })
            .collect::<Vec<_>>();
        let [topology] = matching_topology.as_slice() else {
            return Err(PipelineError::new(
                "mission definition core stage has no unique topology row",
            ));
        };
        let matching_transitions = transitions
            .stages()
            .iter()
            .filter(|item| {
                item.source_ordinal() == key.0
                    && item.sequence_ordinal() == key.1
            })
            .collect::<Vec<_>>();
        let [transition] = matching_transitions.as_slice() else {
            return Err(PipelineError::new(
                "mission definition core stage has no unique transition row",
            ));
        };
        if transition.terminal() != topology.terminal() {
            return Err(PipelineError::new(
                "mission definition core terminal classification drifted",
            ));
        }
        let matching_countdowns = countdowns
            .countdowns()
            .iter()
            .filter(|countdown| {
                countdown.stage_source_ordinal() == key.0
                    && countdown.stage_sequence_ordinal() == key.1
            })
            .collect::<Vec<_>>();
        let countdown = match matching_countdowns.as_slice() {
            [] => None,
            [countdown] => Some((*countdown).clone()),
            _ => {
                return Err(PipelineError::new(
                    "mission definition core stage has duplicate countdowns",
                ));
            },
        };
        let stage_collectible_waypoints = collectible_waypoints
            .bindings()
            .iter()
            .filter(|binding| {
                binding.stage_source_ordinal() == key.0
                    && binding.stage_sequence_ordinal() == key.1
            })
            .cloned()
            .collect::<Vec<_>>();
        let stage_objective_npc_waypoints = objective_npc_waypoints
            .waypoints()
            .iter()
            .filter(|waypoint| {
                waypoint.owner_stage_source_ordinal() == key.0
                    && waypoint.owner_stage_sequence_ordinal() == key.1
            })
            .cloned()
            .collect::<Vec<_>>();
        let matching_objectives = objectives
            .objectives()
            .iter()
            .filter(|objective| {
                objective.owner_stage_source_ordinal() == key.0
                    && objective.owner_stage_sequence_ordinal() == key.1
            })
            .collect::<Vec<_>>();
        let [objective] = matching_objectives.as_slice() else {
            return Err(PipelineError::new(
                "mission definition core stage has no unique root objective",
            ));
        };
        if objective.source_ordinal() <= stage.source_ordinal() {
            return Err(PipelineError::new(
                "mission definition core objective precedes its stage",
            ));
        }

        let mut stage_conditions = conditions
            .conditions()
            .iter()
            .filter(|condition| {
                condition.owner_stage_source_ordinal() == key.0
                    && condition.owner_stage_sequence_ordinal() == key.1
            })
            .map(|condition| {
                if condition.source_ordinal() <= stage.source_ordinal() {
                    return Err(PipelineError::new(
                        "mission definition core condition precedes its stage",
                    ));
                }
                Ok(MissionDefinitionConditionCoreBinding {
                    source_ordinal: condition.source_ordinal(),
                    source_alias: condition.source_alias().to_owned(),
                    schema_id: condition.schema_id(),
                    scope: condition.scope(),
                    owner_objective_source_ordinal:
                        condition.owner_objective_source_ordinal(),
                    parameters: condition.parameters().clone(),
                })
            })
            .collect::<PipelineOutcome<Vec<_>>>()?;
        stage_conditions.sort_by_key(|condition| condition.source_ordinal);

        result.push(MissionDefinitionStageCoreBinding {
            stage_source_ordinal: stage.source_ordinal(),
            sequence_ordinal: stage.sequence_ordinal(),
            kind: stage.kind().clone(),
            next_authored_sequence_ordinal:
                topology.next_authored_sequence_ordinal(),
            checkpoint_source_ordinal: topology.checkpoint_source_ordinal(),
            explicit_final: topology.explicit_final(),
            terminal: topology.terminal(),
            visual_transition: transition.visual(),
            stay_in_black: transition.stay_in_black(),
            show_stage_complete: transition.show_stage_complete(),
            transition_markers: transition.markers().to_vec(),
            countdown,
            collectible_waypoints: stage_collectible_waypoints,
            objective_npc_waypoints: stage_objective_npc_waypoints,
            objective_source_ordinal: objective.source_ordinal(),
            objective_source_alias: objective.source_alias().to_owned(),
            objective_canonical_kind: objective.canonical_kind(),
            objective_unavailable_code: objective.unavailable_code(),
            objective_parameters: objective.parameters().clone(),
            conditions: stage_conditions,
        });
    }

    let report = MissionDefinitionCoreReport {
        mission_id: mission_id.to_owned(),
        stages: result,
    };
    report.validate()?;
    Ok(report)
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/mission_definition_context/tests.rs"]
mod tests;
