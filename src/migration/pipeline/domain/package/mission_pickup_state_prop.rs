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
//   - Authored pickup-target references to collectible state-prop declarations.
// - Must-Not:
//   - Infer state-prop lifetime, pickup mechanics, respawn, or destruction.
// - Allows:
//   - Resolve mission- or stage-scoped declarations by exact prior identity.
//   - Reject missing, ambiguous, or forward pickup-target references.
// - Split-When:
//   - State-prop runtime lifecycle gains an authoritative independent model.
// - Merge-When:
//   - Final objective graph compilation owns this exact cross-reference.
// - Summary:
//   - Pickup-target state-prop semantic preflight.
// - Description:
//   - Closes authored pickup identities across mission and stage scopes.
// - Usage:
//   - Runs after initialization, stage, and objective semantic compilation.
// - Defaults:
//   - Missing, ambiguous, and forward references fail closed.
//

//! Source-backed pickup-target state-prop bindings.

use super::{
    MissionInitializationDirective, MissionInitializationReport,
    MissionObjectiveDirective, MissionObjectiveSemanticReport,
    MissionStageDirective, MissionStageSemanticReport,
};

/// Authored scope containing one collectible state-prop declaration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionPickupStatePropScope {
    /// Declaration appears in mission initialization scope.
    Mission,
    /// Declaration appears in one dense authored stage.
    Stage {
        /// Source `AddStage` ordinal owning the declaration.
        source_ordinal: usize,
        /// Dense authored stage ordinal.
        sequence_ordinal: usize,
    },
}

/// One pickup target resolved to its unique prior state-prop declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionPickupStatePropBinding {
    owner_stage_source_ordinal: usize,
    owner_stage_sequence_ordinal: usize,
    owner_objective_source_ordinal: usize,
    target_source_ordinal: usize,
    target_id: String,
    declaration_source_ordinal: usize,
    declaration_scope: MissionPickupStatePropScope,
    locator_id: String,
    source_state: u32,
}

impl MissionPickupStatePropBinding {
    /// Return source `AddStage` ordinal owning the pickup objective.
    #[must_use]
    pub const fn owner_stage_source_ordinal(&self) -> usize {
        self.owner_stage_source_ordinal
    }

    /// Return dense authored stage ordinal owning the pickup objective.
    #[must_use]
    pub const fn owner_stage_sequence_ordinal(&self) -> usize {
        self.owner_stage_sequence_ordinal
    }

    /// Return source `AddObjective` ordinal owning the pickup target.
    #[must_use]
    pub const fn owner_objective_source_ordinal(&self) -> usize {
        self.owner_objective_source_ordinal
    }

    /// Return the `SetPickupTarget` source ordinal.
    #[must_use]
    pub const fn target_source_ordinal(&self) -> usize {
        self.target_source_ordinal
    }

    /// Return the exact authored pickup target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        &self.target_id
    }

    /// Return the matched `AddCollectibleStateProp` source ordinal.
    #[must_use]
    pub const fn declaration_source_ordinal(&self) -> usize {
        self.declaration_source_ordinal
    }

    /// Return the authored scope containing the matched declaration.
    #[must_use]
    pub const fn declaration_scope(&self) -> MissionPickupStatePropScope {
        self.declaration_scope
    }

    /// Return the exact matched state-prop locator identity.
    #[must_use]
    pub fn locator_id(&self) -> &str {
        &self.locator_id
    }

    /// Return the exact authored state value.
    #[must_use]
    pub const fn source_state(&self) -> u32 {
        self.source_state
    }
}

/// All pickup-target state-prop bindings for one selected source.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionPickupStatePropReport {
    bindings: Vec<MissionPickupStatePropBinding>,
}

impl MissionPickupStatePropReport {
    /// Return bindings in objective/source order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionPickupStatePropBinding] {
        &self.bindings
    }
}

#[derive(Clone, Copy)]
struct StatePropDeclaration<'a> {
    source_ordinal: usize,
    scope: MissionPickupStatePropScope,
    prop_id: &'a str,
    locator_id: &'a str,
    source_state: u32,
}

/// Resolve pickup targets against unique prior state-prop declarations.
///
/// # Errors
///
/// Fails when a selected source has multiple initialization bindings or when a
/// pickup target has no unique prior state-prop declaration with the same id.
pub fn preflight_mission_pickup_state_props(
    initialization: &MissionInitializationReport,
    stages: &MissionStageSemanticReport,
    objectives: &MissionObjectiveSemanticReport,
) -> Result<MissionPickupStatePropReport, String> {
    if stages.stages().len() != objectives.objectives().len() {
        return Err(
            "pickup state-prop stage/objective count drifted".to_owned(),
        );
    }
    let mission = match initialization.missions() {
        [] if stages.stages().is_empty() => {
            return Ok(MissionPickupStatePropReport::default());
        },
        [] => {
            return Err(
                "pickup state-prop semantics have no selected mission"
                    .to_owned(),
            );
        },
        [mission] => mission,
        _ => {
            return Err(
                "pickup state-prop preflight has multiple selected missions"
                    .to_owned(),
            );
        },
    };

    let mut declarations = Vec::new();
    for directive in mission.directives() {
        if let MissionInitializationDirective::CollectibleStateProp {
            source_ordinal,
            prop_id,
            locator_id,
            source_state,
        } = directive
        {
            declarations.push(StatePropDeclaration {
                source_ordinal: *source_ordinal,
                scope: MissionPickupStatePropScope::Mission,
                prop_id,
                locator_id,
                source_state: *source_state,
            });
        }
    }
    for stage in stages.stages() {
        for directive in stage.directives() {
            if let MissionStageDirective::CollectibleStateProp {
                source_ordinal,
                prop_id,
                locator_id,
                source_state,
            } = directive
            {
                declarations.push(StatePropDeclaration {
                    source_ordinal: *source_ordinal,
                    scope: MissionPickupStatePropScope::Stage {
                        source_ordinal: stage.source_ordinal(),
                        sequence_ordinal: stage.sequence_ordinal(),
                    },
                    prop_id,
                    locator_id,
                    source_state: *source_state,
                });
            }
        }
    }

    let mut bindings = Vec::new();
    for objective in objectives.objectives() {
        for directive in objective.directives() {
            let MissionObjectiveDirective::PickupTarget {
                source_ordinal,
                target_id,
            } = directive
            else {
                continue;
            };
            if *source_ordinal <= objective.source_ordinal() {
                return Err(
                    "pickup target precedes its owning objective".to_owned(),
                );
            }
            let matching = declarations
                .iter()
                .filter(|declaration| {
                    declaration.prop_id == target_id
                        && declaration.source_ordinal < *source_ordinal
                })
                .copied()
                .collect::<Vec<_>>();
            let [declaration] = matching.as_slice() else {
                return Err(
                    "pickup target has no unique prior state-prop declaration"
                        .to_owned(),
                );
            };
            bindings.push(MissionPickupStatePropBinding {
                owner_stage_source_ordinal:
                    objective.owner_stage_source_ordinal(),
                owner_stage_sequence_ordinal:
                    objective.owner_stage_sequence_ordinal(),
                owner_objective_source_ordinal: objective.source_ordinal(),
                target_source_ordinal: *source_ordinal,
                target_id: target_id.clone(),
                declaration_source_ordinal: declaration.source_ordinal,
                declaration_scope: declaration.scope,
                locator_id: declaration.locator_id.to_owned(),
                source_state: declaration.source_state,
            });
        }
    }

    Ok(MissionPickupStatePropReport { bindings })
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_pickup_state_prop/tests.rs"]
mod tests;
