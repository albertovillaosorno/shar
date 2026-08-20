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
//   - Commands observed inside reviewed AddCondition scopes.
// - Must-Not:
//   - Interpret modifier parameters or accept command/arity drift.
// - Allows:
//   - Apply already-validated structural adaptations before scope checks.
// - Split-When:
//   - One condition schema gains independently typed modifier parameters.
// - Merge-When:
//   - Condition compilation owns these exact scope contracts.
// - Summary:
//   - Mission condition-scope command registry.
// - Description:
//   - Freezes exact condition command membership and observed arities.
// - Usage:
//   - Runs after structural and condition-alias preflight succeeds.
// - Defaults:
//   - Unknown commands, wrong scopes, and unobserved arities fail closed.
//

//! Reviewed condition-scope command registry.

use super::{MissionScriptEvidence, preflight_mission_conditions};

const L7_KEEPBARREL_ADAPTATION: &str =
    "legacy-l7-m7i-close-keepbarrel-before-stage-complete-v1";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ConditionModifierSchema {
    source_alias: &'static str,
    command: &'static str,
    pub(super) argument_counts: &'static [usize],
}

pub(super) const CONDITION_MODIFIERS: [ConditionModifierSchema; 9] = [
    modifier("damage", "setcondminhealth", &[1]),
    modifier("damage", "setcondtargetvehicle", &[1]),
    modifier("damage", "setobjtargetboss", &[1]),
    modifier("followdistance", "setcondtargetvehicle", &[1]),
    modifier("followdistance", "setfollowdistances", &[2]),
    modifier("outofvehicle", "setcondtime", &[1]),
    modifier("position", "setconditionposition", &[1]),
    modifier("race", "setcondtargetvehicle", &[1]),
    modifier("timeout", "sethitnrun", &[0]),
];

const fn modifier(
    source_alias: &'static str,
    command: &'static str,
    argument_counts: &'static [usize],
) -> ConditionModifierSchema {
    ConditionModifierSchema {
        source_alias,
        command,
        argument_counts,
    }
}

/// One condition-scoped source command retained for typed compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionConditionCommandBinding {
    condition_alias: String,
    ordinal: usize,
    command: String,
    arguments: Vec<String>,
}

impl MissionConditionCommandBinding {
    /// Return the source condition alias owning this command.
    #[must_use]
    pub fn condition_alias(&self) -> &str {
        &self.condition_alias
    }

    /// Return the source statement ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// Return the normalized source command.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Return exact normalized source arguments.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
}

/// Condition-scope command coverage for one structurally valid mission script.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionConditionCommandReport {
    commands: Vec<MissionConditionCommandBinding>,
}

impl MissionConditionCommandReport {
    /// Return reviewed commands in source order.
    #[must_use]
    pub fn commands(&self) -> &[MissionConditionCommandBinding] {
        &self.commands
    }
}

/// Validate every command inside a condition against its exact source scope.
///
/// # Errors
///
/// Returns an error when a condition alias is invalid, a command has not been
/// reviewed in that condition scope, or its argument count was not observed.
pub fn preflight_mission_condition_commands(
    evidence: &MissionScriptEvidence,
) -> Result<MissionConditionCommandReport, String> {
    drop(preflight_mission_conditions(evidence)?);
    let mut current_alias: Option<&str> = None;
    let mut commands = Vec::new();
    for invocation in evidence.invocations() {
        if closes_condition_before_invocation(evidence, invocation.ordinal()) {
            current_alias = None;
        }
        if invocation.name() == "addcondition" {
            current_alias = invocation.arguments().first().map(String::as_str);
            continue;
        }
        if invocation.name() == "closecondition" {
            current_alias = None;
            continue;
        }
        let Some(source_alias) = current_alias else {
            continue;
        };
        let schema = condition_modifier_schema(source_alias, invocation.name())
            .ok_or_else(|| {
                "mission condition-scoped command is not registered".to_owned()
            })?;
        if !schema
            .argument_counts
            .contains(&invocation.arguments().len())
        {
            return Err(
                "mission condition-scoped command arity is not registered"
                    .to_owned(),
            );
        }
        commands.push(MissionConditionCommandBinding {
            condition_alias: source_alias.to_owned(),
            ordinal: invocation.ordinal(),
            command: invocation.name().to_owned(),
            arguments: invocation.arguments().to_vec(),
        });
    }
    Ok(MissionConditionCommandReport { commands })
}

fn closes_condition_before_invocation(
    evidence: &MissionScriptEvidence,
    ordinal: usize,
) -> bool {
    evidence.adaptations().iter().any(|adaptation| {
        adaptation.ordinal() == ordinal
            && adaptation.command() == "showstagecomplete"
            && adaptation.code() == L7_KEEPBARREL_ADAPTATION
    })
}

pub(super) fn condition_modifier_schema(
    source_alias: &str,
    command: &str,
) -> Option<ConditionModifierSchema> {
    CONDITION_MODIFIERS.iter().copied().find(|schema| {
        schema.source_alias == source_alias && schema.command == command
    })
}
