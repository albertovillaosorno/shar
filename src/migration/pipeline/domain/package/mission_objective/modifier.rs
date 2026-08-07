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
//   - Commands observed inside reviewed AddObjective scopes.
// - Must-Not:
//   - Interpret modifier parameters or accept command/arity drift.
// - Allows:
//   - Preserve exact objective-scoped commands for later typed compilation.
// - Split-When:
//   - One modifier family gains an independently versioned parameter schema.
// - Merge-When:
//   - Objective compilation owns these exact scope contracts.
// - Summary:
//   - Mission objective-scope command registry.
// - Description:
//   - Freezes command membership and exact observed arities by objective alias.
// - Usage:
//   - Runs after the closed AddObjective alias registry succeeds.
// - Defaults:
//   - Unknown commands, wrong scopes, and unobserved arities fail closed.
//

//! Reviewed objective-scope command registry.

use super::{MissionScriptEvidence, preflight_mission_objectives};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ObjectiveModifierSchema {
    source_alias: &'static str,
    command: &'static str,
    pub(super) argument_counts: &'static [usize],
}

pub(super) const OBJECTIVE_MODIFIERS: [ObjectiveModifierSchema; 93] = [
    modifier("coins", "setcoinfee", &[1]),
    modifier("delivery", "addcollectible", &[1, 2, 3, 4]),
    modifier("delivery", "addnpc", &[2]),
    modifier("delivery", "addstagevehicle", &[5]),
    modifier("delivery", "setcollectibleeffect", &[1]),
    modifier("destroy", "activatevehicle", &[3]),
    modifier("destroy", "setobjtargetvehicle", &[1]),
    modifier("destroy", "setvehicleaiparams", &[3]),
    modifier("destroyboss", "addcondition", &[1]),
    modifier("destroyboss", "closecondition", &[0]),
    modifier("destroyboss", "setobjtargetboss", &[1]),
    modifier("dialogue", "addambientnpcanimation", &[1]),
    modifier("dialogue", "addambientpcanimation", &[1]),
    modifier("dialogue", "addnpc", &[2]),
    modifier("dialogue", "addstagecharacter", &[4]),
    modifier("dialogue", "addstagevehicle", &[4]),
    modifier("dialogue", "ambientanimationrandomize", &[2]),
    modifier("dialogue", "removenpc", &[1]),
    modifier("dialogue", "setcambestside", &[1]),
    modifier("dialogue", "setconversationcam", &[2]),
    modifier("dialogue", "setdialogueinfo", &[4]),
    modifier("dialogue", "setdialoguepositions", &[3, 4]),
    modifier("dialogue", "setpresentationbitmap", &[1]),
    modifier("dialogue", "setstagemessageindex", &[2]),
    modifier("dump", "activatevehicle", &[3]),
    modifier("dump", "addcollectible", &[2, 4]),
    modifier("dump", "addnpc", &[2]),
    modifier("dump", "addstagecharacter", &[3]),
    modifier("dump", "addstagevehicle", &[5]),
    modifier("dump", "bindcollectibleto", &[2]),
    modifier("dump", "setcollectibleeffect", &[1]),
    modifier("dump", "setobjtargetvehicle", &[1]),
    modifier("dump", "setstageaitargetcatchupparams", &[3]),
    modifier("dump", "setvehicleaiparams", &[3]),
    modifier("fmv", "setfmvinfo", &[1, 2]),
    modifier("follow", "addnpc", &[2]),
    modifier("follow", "addstagevehicle", &[5]),
    modifier("follow", "setobjtargetvehicle", &[1]),
    modifier("getin", "addnpc", &[2]),
    modifier("getin", "addstagevehicle", &[4, 5]),
    modifier("getin", "setobjtargetvehicle", &[1]),
    modifier("gooutside", "addstagevehicle", &[4]),
    modifier("gooutside", "setdestination", &[1]),
    modifier("gooutside", "turngotodialogoff", &[0]),
    modifier("goto", "addnpc", &[2, 3]),
    modifier("goto", "addobjectivenpcwaypoint", &[2]),
    modifier("goto", "addsafezone", &[2]),
    modifier("goto", "addstagecharacter", &[4]),
    modifier("goto", "addstagevehicle", &[4, 5]),
    modifier("goto", "allowrockout", &[0]),
    modifier("goto", "mustactiontrigger", &[0]),
    modifier("goto", "removedriver", &[1]),
    modifier("goto", "removenpc", &[1]),
    modifier("goto", "setcollectibleeffect", &[1]),
    modifier("goto", "setdestination", &[1, 2]),
    modifier("goto", "setpresentationbitmap", &[1]),
    modifier("goto", "turngotodialogoff", &[0]),
    modifier("interior", "addnpc", &[2]),
    modifier("interior", "setdestination", &[1, 2]),
    modifier("losetail", "addnpc", &[2]),
    modifier("losetail", "setobjdistance", &[1]),
    modifier("losetail", "setobjtargetvehicle", &[1]),
    modifier("pickupitem", "addnpc", &[2]),
    modifier("pickupitem", "setpickuptarget", &[1]),
    modifier("race", "addcollectible", &[1, 2]),
    modifier("race", "addcondition", &[1]),
    modifier("race", "adddriver", &[2]),
    modifier("race", "addnpc", &[2]),
    modifier("race", "closecondition", &[0]),
    modifier("race", "disablehitandrun", &[0]),
    modifier("race", "removenpc", &[1]),
    modifier("race", "setcollectibleeffect", &[1]),
    modifier("race", "setcondminhealth", &[1]),
    modifier("race", "setcondtargetvehicle", &[1]),
    modifier("race", "setcondtime", &[1]),
    modifier("race", "setpartime", &[1]),
    modifier("race", "setracelaps", &[1]),
    modifier("talkto", "addnpc", &[2, 3]),
    modifier("talkto", "addobjectivenpcwaypoint", &[2]),
    modifier("talkto", "addsafezone", &[2]),
    modifier("talkto", "addstagecharacter", &[4]),
    modifier("talkto", "addstagevehicle", &[4, 5]),
    modifier("talkto", "setcambestside", &[1]),
    modifier("talkto", "setpresentationbitmap", &[1]),
    modifier("talkto", "settalktotarget", &[1, 3, 4]),
    modifier("timer", "addnpc", &[2]),
    modifier("timer", "addstagecharacter", &[4]),
    modifier("timer", "addstagevehicle", &[4]),
    modifier("timer", "removedriver", &[1]),
    modifier("timer", "setdurationtime", &[1]),
    modifier("timer", "setgameover", &[0]),
    modifier("timer", "setlevelover", &[0]),
    modifier("timer", "stayinblack", &[0]),
];

const fn modifier(
    source_alias: &'static str,
    command: &'static str,
    argument_counts: &'static [usize],
) -> ObjectiveModifierSchema {
    ObjectiveModifierSchema {
        source_alias,
        command,
        argument_counts,
    }
}

/// One objective-scoped source command retained for later typed compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveCommandBinding {
    objective_alias: String,
    ordinal: usize,
    command: String,
    arguments: Vec<String>,
}

impl MissionObjectiveCommandBinding {
    /// Return the source objective alias owning this command.
    #[must_use]
    pub fn objective_alias(&self) -> &str {
        &self.objective_alias
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

/// Objective-scope command coverage for one structurally valid mission script.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveCommandReport {
    commands: Vec<MissionObjectiveCommandBinding>,
}

impl MissionObjectiveCommandReport {
    /// Return reviewed commands in source order.
    #[must_use]
    pub fn commands(&self) -> &[MissionObjectiveCommandBinding] {
        &self.commands
    }
}

/// Validate every command inside an objective against its exact source scope.
///
/// # Errors
///
/// Returns an error if an objective alias is invalid, a command has never been
/// reviewed in that objective scope, or its argument count was not observed.
pub fn preflight_mission_objective_commands(
    evidence: &MissionScriptEvidence,
) -> Result<MissionObjectiveCommandReport, String> {
    drop(preflight_mission_objectives(evidence)?);
    let mut current_alias: Option<&str> = None;
    let mut commands = Vec::new();
    for invocation in evidence.invocations() {
        if invocation.name() == "addobjective" {
            current_alias = invocation.arguments().first().map(String::as_str);
            continue;
        }
        if invocation.name() == "closeobjective" {
            current_alias = None;
            continue;
        }
        let Some(source_alias) = current_alias else {
            continue;
        };
        let schema = objective_modifier_schema(source_alias, invocation.name())
            .ok_or_else(|| {
                "mission objective-scoped command is not registered".to_owned()
            })?;
        if !schema
            .argument_counts
            .contains(&invocation.arguments().len())
        {
            return Err(
                "mission objective-scoped command arity is not registered"
                    .to_owned(),
            );
        }
        commands.push(MissionObjectiveCommandBinding {
            objective_alias: source_alias.to_owned(),
            ordinal: invocation.ordinal(),
            command: invocation.name().to_owned(),
            arguments: invocation.arguments().to_vec(),
        });
    }
    Ok(MissionObjectiveCommandReport { commands })
}

pub(super) fn objective_modifier_schema(
    source_alias: &str,
    command: &str,
) -> Option<ObjectiveModifierSchema> {
    OBJECTIVE_MODIFIERS.iter().copied().find(|schema| {
        schema.source_alias == source_alias && schema.command == command
    })
}
