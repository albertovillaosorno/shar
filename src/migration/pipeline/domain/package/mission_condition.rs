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
//   - Closed legacy AddCondition alias and arity preflight.
// - Must-Not:
//   - Claim final runtime condition semantics or compile stage transitions.
// - Allows:
//   - Bind reviewed source aliases to versioned legacy condition schemas.
// - Split-When:
//   - Modifier commands gain independently versioned parameter schemas.
// - Merge-When:
//   - Mission semantic compilation owns this exact closed registry.
// - Summary:
//   - Mission condition alias registry.
// - Description:
//   - Converts reviewed source condition names into typed compiler evidence.
// - Usage:
//   - Runs only after mission-script structural semantic preflight succeeds.
// - Defaults:
//   - Unknown aliases, arity drift, and malformed calls fail closed.
//

//! Reviewed legacy mission-condition alias registry.

use super::mission_script::MissionScriptEvidence;

mod directive;
mod modifier;
mod parameter;
mod violation;

pub use directive::{
    MissionConditionDirective, MissionConditionSemanticBinding,
    MissionConditionSemanticReport, preflight_mission_condition_semantics,
};
pub use modifier::{
    MissionConditionCommandBinding, MissionConditionCommandReport,
    preflight_mission_condition_commands,
};
pub use parameter::{
    MissionConditionParameterBinding, MissionConditionParameterReport,
    MissionConditionParameters, preflight_mission_condition_parameters,
};
pub use violation::{
    MissionConditionViolationBinding, MissionConditionViolationEffect,
    MissionConditionViolationReport, preflight_mission_condition_violations,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct ConditionAliasSchema {
    source_alias: &'static str,
    schema_id: &'static str,
    minimum_arguments: usize,
    maximum_arguments: usize,
}

const CONDITION_ALIASES: [ConditionAliasSchema; 7] = [
    condition("damage", "legacy-mission-condition.damage.v1", 1, 2),
    condition(
        "followdistance",
        "legacy-mission-condition.follow-distance.v1",
        1,
        1,
    ),
    condition(
        "keepbarrel",
        "legacy-mission-condition.keep-barrel.v1",
        2,
        2,
    ),
    condition(
        "outofvehicle",
        "legacy-mission-condition.out-of-vehicle.v1",
        1,
        1,
    ),
    condition("position", "legacy-mission-condition.position.v1", 1, 1),
    condition("race", "legacy-mission-condition.race.v1", 1, 1),
    condition("timeout", "legacy-mission-condition.timeout.v1", 1, 1),
];

const fn condition(
    source_alias: &'static str,
    schema_id: &'static str,
    minimum_arguments: usize,
    maximum_arguments: usize,
) -> ConditionAliasSchema {
    ConditionAliasSchema {
        source_alias,
        schema_id,
        minimum_arguments,
        maximum_arguments,
    }
}

/// One reviewed source condition retained for semantic compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionConditionBinding {
    ordinal: usize,
    source_alias: String,
    schema_id: &'static str,
    legacy_parameters: Vec<String>,
}

impl MissionConditionBinding {
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

    /// Return the versioned legacy condition schema identity.
    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    /// Return source-only parameters retained for later typed conversion.
    #[must_use]
    pub fn legacy_parameters(&self) -> &[String] {
        &self.legacy_parameters
    }
}

/// Complete condition-alias preflight for one structurally valid mission
/// script.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionConditionReport {
    conditions: Vec<MissionConditionBinding>,
}

impl MissionConditionReport {
    /// Return condition bindings in source order.
    #[must_use]
    pub fn conditions(&self) -> &[MissionConditionBinding] {
        &self.conditions
    }
}

/// Resolve every `AddCondition` call through the closed reviewed registry.
///
/// # Errors
///
/// Returns an error for unknown aliases, missing aliases, or argument-count
/// drift. Modifier commands remain separate evidence for later schema mapping.
pub fn preflight_mission_conditions(
    evidence: &MissionScriptEvidence,
) -> Result<MissionConditionReport, String> {
    let mut conditions = Vec::new();
    for invocation in evidence
        .invocations()
        .iter()
        .filter(|invocation| invocation.name() == "addcondition")
    {
        let arguments = invocation.arguments();
        let Some(source_alias) = arguments.first() else {
            return Err(
                "mission condition call requires a source alias".to_owned()
            );
        };
        let schema = condition_alias_schema(source_alias).ok_or_else(|| {
            "mission condition alias is not registered".to_owned()
        })?;
        if arguments.len() < schema.minimum_arguments
            || arguments.len() > schema.maximum_arguments
        {
            return Err(
                "mission condition argument count is not registered".to_owned()
            );
        }
        conditions.push(MissionConditionBinding {
            ordinal: invocation.ordinal(),
            source_alias: source_alias.clone(),
            schema_id: schema.schema_id,
            legacy_parameters: arguments.iter().skip(1).cloned().collect(),
        });
    }
    Ok(MissionConditionReport { conditions })
}

fn condition_alias_schema(alias: &str) -> Option<ConditionAliasSchema> {
    CONDITION_ALIASES
        .iter()
        .copied()
        .find(|schema| schema.source_alias == alias)
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_condition/tests.rs"]
mod tests;
