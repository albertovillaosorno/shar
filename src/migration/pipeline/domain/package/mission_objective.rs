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
//   - Closed legacy AddObjective alias and arity preflight.
// - Must-Not:
//   - Compile stage graphs or infer unresolved objective semantics.
// - Allows:
//   - Map reviewed source aliases to canonical objective-kind identities.
// - Split-When:
//   - Objective parameter schemas gain independently versioned families.
// - Merge-When:
//   - Mission semantic compilation owns this exact closed registry.
// - Summary:
//   - Mission objective alias registry.
// - Description:
//   - Converts reviewed source objective names into typed compiler evidence.
// - Usage:
//   - Runs only after mission-script structural semantic preflight succeeds.
// - Defaults:
//   - Unknown aliases, arity drift, and malformed calls fail closed.
//

//! Reviewed legacy mission-objective alias registry.

use super::mission_script::MissionScriptEvidence;

mod directive;
mod modifier;
mod parameter;

pub use directive::{
    MissionObjectiveDirective, MissionObjectiveNpcReference,
    MissionObjectiveSemanticBinding, MissionObjectiveSemanticReport,
    preflight_mission_objective_semantics,
};
pub use modifier::{
    MissionObjectiveCommandBinding, MissionObjectiveCommandReport,
    preflight_mission_objective_commands,
};
pub use parameter::{
    MissionObjectiveParameterBinding, MissionObjectiveParameterReport,
    MissionObjectiveParameters, MissionRoadArrowBinding, MissionRoadArrowMode,
    preflight_mission_objective_parameters,
};

const UNAVAILABLE_DUMMY_OBJECTIVE: &str =
    "legacy-dummy-objective-unavailable-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ObjectiveAliasSchema {
    source_alias: &'static str,
    canonical_kind: Option<&'static str>,
    minimum_arguments: usize,
    maximum_arguments: usize,
    unavailable_code: Option<&'static str>,
}

const OBJECTIVE_ALIASES: [ObjectiveAliasSchema; 20] = [
    mapped("buycar", "buy_vehicle", 2, 2),
    mapped("buyskin", "buy_costume", 2, 2),
    mapped("coins", "wager_entry", 1, 1),
    mapped("delivery", "deliver", 1, 2),
    mapped("destroy", "destroy", 1, 2),
    mapped("destroyboss", "boss_phase", 1, 1),
    mapped("dialogue", "dialogue", 1, 1),
    unavailable("dummy", 1, 1, UNAVAILABLE_DUMMY_OBJECTIVE),
    mapped("dump", "dumped_collectible", 1, 2),
    mapped("fmv", "cinematic", 1, 1),
    mapped("follow", "follow", 1, 2),
    mapped("getin", "enter_vehicle", 1, 2),
    mapped("gooutside", "exit_interior", 1, 1),
    mapped("goto", "travel", 1, 2),
    mapped("interior", "enter_interior", 1, 2),
    mapped("losetail", "avoid", 1, 2),
    mapped("pickupitem", "item_pickup", 1, 1),
    mapped("race", "race", 1, 3),
    mapped("talkto", "talk", 1, 2),
    mapped("timer", "timer", 1, 1),
];

const fn mapped(
    source_alias: &'static str,
    canonical_kind: &'static str,
    minimum_arguments: usize,
    maximum_arguments: usize,
) -> ObjectiveAliasSchema {
    ObjectiveAliasSchema {
        source_alias,
        canonical_kind: Some(canonical_kind),
        minimum_arguments,
        maximum_arguments,
        unavailable_code: None,
    }
}

const fn unavailable(
    source_alias: &'static str,
    minimum_arguments: usize,
    maximum_arguments: usize,
    unavailable_code: &'static str,
) -> ObjectiveAliasSchema {
    ObjectiveAliasSchema {
        source_alias,
        canonical_kind: None,
        minimum_arguments,
        maximum_arguments,
        unavailable_code: Some(unavailable_code),
    }
}

/// One reviewed source objective retained for semantic compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveBinding {
    ordinal: usize,
    source_alias: String,
    canonical_kind: Option<&'static str>,
    legacy_parameters: Vec<String>,
    unavailable_code: Option<&'static str>,
}

impl MissionObjectiveBinding {
    /// Return the source statement ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// Return the exact reviewed source alias.
    #[must_use]
    pub fn source_alias(&self) -> &str {
        &self.source_alias
    }

    /// Return the canonical runtime objective kind when mapping is complete.
    #[must_use]
    pub const fn canonical_kind(&self) -> Option<&'static str> {
        self.canonical_kind
    }

    /// Return source-only parameters retained for later typed conversion.
    #[must_use]
    pub fn legacy_parameters(&self) -> &[String] {
        &self.legacy_parameters
    }

    /// Return the versioned explicit-unavailable identity when not mapped.
    #[must_use]
    pub const fn unavailable_code(&self) -> Option<&'static str> {
        self.unavailable_code
    }

    /// Return whether this objective can proceed to parameter compilation.
    #[must_use]
    pub const fn is_mapped(&self) -> bool {
        self.canonical_kind.is_some()
    }
}

/// Complete objective-alias preflight for one structurally valid mission
/// script.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveReport {
    objectives: Vec<MissionObjectiveBinding>,
    mapped_count: usize,
    unavailable_count: usize,
}

impl MissionObjectiveReport {
    /// Return objective bindings in source order.
    #[must_use]
    pub fn objectives(&self) -> &[MissionObjectiveBinding] {
        &self.objectives
    }

    /// Return the number of reviewed aliases mapped to canonical kinds.
    #[must_use]
    pub const fn mapped_count(&self) -> usize {
        self.mapped_count
    }

    /// Return the number of explicitly reviewed but unavailable aliases.
    #[must_use]
    pub const fn unavailable_count(&self) -> usize {
        self.unavailable_count
    }

    /// Return whether every source objective has a canonical runtime kind.
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.unavailable_count == 0
    }
}

/// Resolve every `AddObjective` call through the closed reviewed registry.
///
/// # Errors
///
/// Returns an error for unknown aliases, missing aliases, or argument-count
/// drift. Explicitly reviewed unavailable aliases remain typed report entries.
pub fn preflight_mission_objectives(
    evidence: &MissionScriptEvidence,
) -> Result<MissionObjectiveReport, String> {
    let mut objectives = Vec::new();
    let mut mapped_count = 0usize;
    let mut unavailable_count = 0usize;
    for invocation in evidence
        .invocations()
        .iter()
        .filter(|invocation| invocation.name() == "addobjective")
    {
        let arguments = invocation.arguments();
        let Some(source_alias) = arguments.first() else {
            return Err(
                "mission objective call requires a source alias".to_owned()
            );
        };
        let schema = objective_alias_schema(source_alias).ok_or_else(|| {
            "mission objective alias is not registered".to_owned()
        })?;
        if arguments.len() < schema.minimum_arguments
            || arguments.len() > schema.maximum_arguments
        {
            return Err(
                "mission objective argument count is not registered".to_owned()
            );
        }
        if schema.canonical_kind.is_some() {
            mapped_count = mapped_count.saturating_add(1);
        } else {
            unavailable_count = unavailable_count.saturating_add(1);
        }
        objectives.push(MissionObjectiveBinding {
            ordinal: invocation.ordinal(),
            source_alias: source_alias.clone(),
            canonical_kind: schema.canonical_kind,
            legacy_parameters: arguments.iter().skip(1).cloned().collect(),
            unavailable_code: schema.unavailable_code,
        });
    }
    Ok(MissionObjectiveReport {
        objectives,
        mapped_count,
        unavailable_count,
    })
}

fn objective_alias_schema(alias: &str) -> Option<ObjectiveAliasSchema> {
    OBJECTIVE_ALIASES
        .iter()
        .copied()
        .find(|schema| schema.source_alias == alias)
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_objective/tests.rs"]
mod tests;
