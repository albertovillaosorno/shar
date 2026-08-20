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
//   - Mission command context validation for normalized MFK evidence.
// - Must-Not:
//   - Interpret gameplay semantics or silently repair malformed source.
// - Allows:
//   - Validate reviewed context aliases, arities, and nesting.
// - Split-When:
//   - Split when another command family gains an independent context stack.
// - Merge-When:
//   - Merge when context validation no longer has independent policy.
// - Summary:
//   - Mission command context validator.
// - Description:
//   - Records deterministic structural findings without blocking extraction.
// - Usage:
//   - Called while rendering mission-script v2 normalized evidence.
// - Defaults:
//   - Unknown commands are left unreviewed and malformed nesting is reported.
//

//! Mission command context validation.

use std::path::Path;

use super::super::json::json_string;
use super::CommandInvocation;

mod compatibility;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct MissionContextState {
    mission: bool,
    stage: bool,
    objective: bool,
    condition: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ContextFinding {
    ordinal: usize,
    command: &'static str,
    code: &'static str,
}

pub(super) struct ContextValidation {
    pub(super) findings: Vec<ContextFinding>,
    pub(super) adaptations: Vec<compatibility::ContextAdaptation>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ContextCommand {
    SelectMission,
    CloseMission,
    AddStage,
    CloseStage,
    AddObjective,
    CloseObjective,
    AddCondition,
    CloseCondition,
}

fn command_contract(name: &str) -> Option<(ContextCommand, usize, usize)> {
    match name {
        "selectmission" => Some((ContextCommand::SelectMission, 1, 1)),
        "closemission" => Some((ContextCommand::CloseMission, 0, 0)),
        "addstage" => Some((ContextCommand::AddStage, 0, 3)),
        "closestage" => Some((ContextCommand::CloseStage, 0, 0)),
        "addobjective" => Some((ContextCommand::AddObjective, 1, 3)),
        "closeobjective" => Some((ContextCommand::CloseObjective, 0, 0)),
        "addcondition" => Some((ContextCommand::AddCondition, 1, 2)),
        "closecondition" => Some((ContextCommand::CloseCondition, 0, 0)),
        _ => None,
    }
}

pub(super) fn command_count(invocations: &[CommandInvocation]) -> usize {
    invocations
        .iter()
        .filter(|invocation| command_contract(&invocation.name).is_some())
        .count()
}

const fn command_name(command: ContextCommand) -> &'static str {
    match command {
        ContextCommand::SelectMission => "selectmission",
        ContextCommand::CloseMission => "closemission",
        ContextCommand::AddStage => "addstage",
        ContextCommand::CloseStage => "closestage",
        ContextCommand::AddObjective => "addobjective",
        ContextCommand::CloseObjective => "closeobjective",
        ContextCommand::AddCondition => "addcondition",
        ContextCommand::CloseCondition => "closecondition",
    }
}

fn push_finding(
    findings: &mut Vec<ContextFinding>,
    invocation: &CommandInvocation,
    command: ContextCommand,
    code: &'static str,
) {
    findings.push(ContextFinding {
        ordinal: invocation.ordinal,
        command: command_name(command),
        code,
    });
}

fn validate_arity(
    invocation: &CommandInvocation,
    command: ContextCommand,
    minimum: usize,
    maximum: usize,
    findings: &mut Vec<ContextFinding>,
) {
    if !(minimum..=maximum).contains(&invocation.arguments.len()) {
        push_finding(
            findings,
            invocation,
            command,
            "invalid-context-command-arity",
        );
    }
}

fn apply_command(
    invocation: &CommandInvocation,
    command: ContextCommand,
    state: &mut MissionContextState,
    findings: &mut Vec<ContextFinding>,
    adaptations: &[compatibility::ContextAdaptation],
) {
    match command {
        ContextCommand::SelectMission => {
            if state.mission
                || state.stage
                || state.objective
                || state.condition
            {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "mission-open-with-active-context",
                );
            }
            *state = MissionContextState {
                mission: true,
                ..MissionContextState::default()
            };
        },
        ContextCommand::CloseMission => {
            if !state.mission {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "mission-close-without-open-mission",
                );
            } else if state.stage || state.objective || state.condition {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "mission-close-with-open-context",
                );
            }
            *state = MissionContextState::default();
        },
        ContextCommand::AddStage => {
            if !state.mission {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "stage-open-outside-mission",
                );
            } else if state.stage || state.objective || state.condition {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "stage-open-with-active-context",
                );
            }
            state.stage = true;
            state.objective = false;
            state.condition = false;
        },
        ContextCommand::CloseStage => {
            if !state.stage {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "stage-close-without-open-stage",
                );
            } else if state.objective || state.condition {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "stage-close-with-open-context",
                );
            }
            state.stage = false;
            state.objective = false;
            state.condition = false;
        },
        ContextCommand::AddObjective => {
            if !state.stage {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "objective-open-outside-stage",
                );
            } else if state.objective || state.condition {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "objective-open-with-active-context",
                );
            }
            state.objective = true;
            state.condition = false;
        },
        ContextCommand::CloseObjective => {
            if !state.objective {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "objective-close-without-open-objective",
                );
            } else if state.condition {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "objective-close-with-open-condition",
                );
            }
            state.objective = false;
            state.condition = false;
        },
        ContextCommand::AddCondition => {
            if !state.stage {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "condition-open-outside-stage",
                );
            } else if state.condition {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "condition-open-with-active-condition",
                );
            }
            state.condition = true;
        },
        ContextCommand::CloseCondition => {
            if !state.condition
                && !compatibility::ignores_orphan_condition_close(
                    adaptations,
                    invocation.ordinal,
                )
            {
                push_finding(
                    findings,
                    invocation,
                    command,
                    "condition-close-without-open-condition",
                );
            }
            state.condition = false;
        },
    }
}

pub(super) fn validate(
    relative: &Path,
    invocations: &[CommandInvocation],
) -> ContextValidation {
    let adaptations =
        compatibility::reviewed_adaptations(relative, invocations);
    let mut state = MissionContextState::default();
    let mut findings = Vec::new();
    for invocation in invocations {
        if state.condition
            && compatibility::closes_condition_before_stage_complete(
                &adaptations,
                invocation.ordinal,
            )
        {
            state.condition = false;
        }
        let Some((command, minimum, maximum)) =
            command_contract(&invocation.name)
        else {
            continue;
        };
        validate_arity(invocation, command, minimum, maximum, &mut findings);
        apply_command(
            invocation,
            command,
            &mut state,
            &mut findings,
            &adaptations,
        );
    }
    if state.mission || state.stage || state.objective || state.condition {
        findings.push(ContextFinding {
            ordinal: invocations
                .last()
                .map_or(0, |invocation| invocation.ordinal),
            command: "eof",
            code: "unclosed-mission-context",
        });
    }
    ContextValidation { findings, adaptations }
}

pub(super) fn findings_json(findings: &[ContextFinding]) -> String {
    let mut out = String::from("[");
    for (index, finding) in findings.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push_str("{\"ordinal\":");
        out.push_str(&finding.ordinal.to_string());
        out.push_str(",\"command\":");
        out.push_str(&json_string(finding.command));
        out.push_str(",\"code\":");
        out.push_str(&json_string(finding.code));
        out.push('}');
    }
    out.push(']');
    out
}

pub(super) fn adaptations_json(
    adaptations: &[compatibility::ContextAdaptation],
) -> String {
    compatibility::adaptations_json(adaptations)
}
