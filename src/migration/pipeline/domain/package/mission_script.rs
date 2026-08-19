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
//   - Fail-closed normalized mission-script evidence intake.
// - Must-Not:
//   - Map legacy gameplay semantics or emit Unreal assets.
// - Allows:
//   - Validate exact schema, structural findings, and command evidence.
// - Split-When:
//   - Split when mission semantic compilation gains an independent lifecycle.
// - Merge-When:
//   - Merge when another package domain owns identical mission evidence.
// - Summary:
//   - Mission script semantic preflight.
// - Description:
//   - Accepts only structurally clean v2 MFK evidence for later compilation.
// - Usage:
//   - Run before any legacy objective mapping or Unreal asset construction.
// - Defaults:
//   - Stale, malformed, inconsistent, or structurally ambiguous input fails.
//

//! Normalized mission-script semantic preflight.

use std::collections::{BTreeMap, BTreeSet};

/// Exact normalized mission-script schema accepted by semantic compilation.
pub const MISSION_SCRIPT_SCHEMA: &str =
    "shar-schoenwald.straggler.mission-script.v3";

const CONTEXT_COMMANDS: [&str; 8] = [
    "selectmission",
    "closemission",
    "addstage",
    "closestage",
    "addobjective",
    "closeobjective",
    "addcondition",
    "closecondition",
];

/// One canonical normalized command retained for future semantic compilation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionCommandInvocation {
    ordinal: usize,
    name: String,
    args_raw: String,
    semantic_role: String,
    arguments: Vec<String>,
}

impl MissionCommandInvocation {
    /// Return the source statement ordinal.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// Return the normalized lowercase command identity.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Return the exact normalized argument payload inside the call.
    #[must_use]
    pub fn args_raw(&self) -> &str {
        &self.args_raw
    }

    /// Return the extraction-level semantic role classification.
    #[must_use]
    pub fn semantic_role(&self) -> &str {
        &self.semantic_role
    }

    /// Return normalized argument values in source order.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        &self.arguments
    }
}

/// One reviewed structural compatibility adaptation retained as evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionContextAdaptation {
    ordinal: usize,
    command: String,
    code: String,
}

impl MissionContextAdaptation {
    /// Return the source statement ordinal where adaptation takes effect.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// Return the context command where adaptation takes effect.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Return the versioned reviewed compatibility identity.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.code
    }
}

/// Structurally clean normalized mission-script evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionScriptEvidence {
    source_bytes: u64,
    statement_count: usize,
    adaptations: Vec<MissionContextAdaptation>,
    invocations: Vec<MissionCommandInvocation>,
}

impl MissionScriptEvidence {
    /// Return the exact normalized source byte count.
    #[must_use]
    pub const fn source_bytes(&self) -> u64 {
        self.source_bytes
    }

    /// Return the number of non-comment source statements.
    #[must_use]
    pub const fn statement_count(&self) -> usize {
        self.statement_count
    }

    /// Return reviewed structural adaptations in source order.
    #[must_use]
    pub fn adaptations(&self) -> &[MissionContextAdaptation] {
        &self.adaptations
    }

    /// Return canonical command invocations in source order.
    #[must_use]
    pub fn invocations(&self) -> &[MissionCommandInvocation] {
        &self.invocations
    }
}

#[derive(Debug)]
pub(crate) struct MissionScriptDocument {
    pub(crate) schema: String,
    pub(crate) source_extension: String,
    pub(crate) route_class: String,
    pub(crate) source_bytes: u64,
    pub(crate) context_command_count: usize,
    pub(crate) context_adaptation_count: usize,
    pub(crate) context_adaptations: Vec<MissionContextAdaptationDocument>,
    pub(crate) context_finding_count: usize,
    pub(crate) context_findings: Vec<MissionContextFinding>,
    pub(crate) statement_count: usize,
    pub(crate) unique_command_count: usize,
    pub(crate) load_p3d_reference_count: usize,
    pub(crate) mission_flow_command_count: usize,
    pub(crate) vehicle_physics_command_count: usize,
    pub(crate) semantic_family: String,
    pub(crate) command_counts: BTreeMap<String, usize>,
    pub(crate) source_statements: Vec<String>,
    pub(crate) p3d_references: Vec<String>,
    pub(crate) command_invocations: Vec<MissionCommandDocument>,
}

#[derive(Debug)]
pub(crate) struct MissionContextFinding {
    pub(crate) ordinal: usize,
    pub(crate) command: String,
    pub(crate) code: String,
}

#[derive(Debug)]
pub(crate) struct MissionContextAdaptationDocument {
    pub(crate) ordinal: usize,
    pub(crate) command: String,
    pub(crate) code: String,
}

#[derive(Debug)]
pub(crate) struct MissionCommandDocument {
    pub(crate) ordinal: usize,
    pub(crate) name: String,
    pub(crate) args_raw: String,
    pub(crate) semantic_role: String,
    pub(crate) arguments: Vec<String>,
}

/// Validate one normalized MFK document before semantic compilation.
///
/// # Errors
///
/// Returns an error when schema, structure, findings, or command evidence
/// drift.
pub(crate) fn preflight_mission_script_document(
    document: MissionScriptDocument,
) -> Result<MissionScriptEvidence, String> {
    validate_identity(&document)?;
    validate_context_evidence(&document)?;
    validate_context_structure(&document)?;
    validate_source_evidence(&document)?;
    validate_command_evidence(&document)?;
    Ok(MissionScriptEvidence {
        source_bytes: document.source_bytes,
        statement_count: document.statement_count,
        adaptations: document
            .context_adaptations
            .into_iter()
            .map(|adaptation| MissionContextAdaptation {
                ordinal: adaptation.ordinal,
                command: adaptation.command,
                code: adaptation.code,
            })
            .collect(),
        invocations: document
            .command_invocations
            .into_iter()
            .map(|invocation| MissionCommandInvocation {
                ordinal: invocation.ordinal,
                name: invocation.name,
                args_raw: invocation.args_raw,
                semantic_role: invocation.semantic_role,
                arguments: invocation.arguments,
            })
            .collect(),
    })
}

fn validate_identity(document: &MissionScriptDocument) -> Result<(), String> {
    if document.schema != MISSION_SCRIPT_SCHEMA {
        return Err(
            "normalized mission script schema is not supported".to_owned()
        );
    }
    if document.source_extension != "mfk"
        || document.route_class != "mission"
        || document.semantic_family != "mission-script"
    {
        return Err(
            "normalized mission script routing identity is invalid".to_owned()
        );
    }
    if (document.source_bytes == 0) != (document.statement_count == 0) {
        return Err(
            concat!(
                "normalized mission script byte and statement evidence ",
                "is inconsistent",
            )
            .to_owned(),
        );
    }
    Ok(())
}

fn validate_context_evidence(
    document: &MissionScriptDocument,
) -> Result<(), String> {
    if document.context_adaptation_count != document.context_adaptations.len() {
        return Err(
            "mission context adaptation count is inconsistent".to_owned()
        );
    }
    let mut previous_adaptation_ordinal = 0usize;
    let mut seen_adaptation_codes = BTreeSet::new();
    for adaptation in &document.context_adaptations {
        if adaptation.ordinal == 0
            || adaptation.ordinal <= previous_adaptation_ordinal
            || adaptation.command.is_empty()
            || adaptation.code.is_empty()
        {
            return Err("mission context adaptation is malformed".to_owned());
        }
        if !seen_adaptation_codes.insert(adaptation.code.as_str()) {
            return Err(
                "mission context adaptation identity is duplicated".to_owned()
            );
        }
        validate_context_adaptation(document, adaptation)?;
        previous_adaptation_ordinal = adaptation.ordinal;
    }
    if document.context_finding_count != document.context_findings.len() {
        return Err("mission context finding count is inconsistent".to_owned());
    }
    for finding in &document.context_findings {
        if finding.ordinal == 0
            || finding.command.is_empty()
            || finding.code.is_empty()
        {
            return Err("mission context finding is malformed".to_owned());
        }
    }
    if !document.context_findings.is_empty() {
        return Err(
            concat!(
                "mission context findings must be resolved before semantic ",
                "compilation",
            )
            .to_owned(),
        );
    }
    let context_count = document
        .command_invocations
        .iter()
        .filter(|invocation| {
            CONTEXT_COMMANDS.contains(&invocation.name.as_str())
        })
        .count();
    if context_count != document.context_command_count {
        return Err("mission context command count is inconsistent".to_owned());
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct MissionContextState {
    mission: bool,
    stage: bool,
    objective: bool,
    condition: bool,
}

fn validate_context_structure(
    document: &MissionScriptDocument,
) -> Result<(), String> {
    let mut state = MissionContextState::default();
    for invocation in &document.command_invocations {
        if closes_condition_before_invocation(document, invocation.ordinal) {
            state.condition = false;
        }
        match invocation.name.as_str() {
            "selectmission" => {
                require_context_arity(invocation, 1, 1)?;
                if state != MissionContextState::default() {
                    return Err(
                        "mission context structure disagrees with evidence"
                            .to_owned(),
                    );
                }
                state.mission = true;
            },
            "closemission" => {
                require_context_arity(invocation, 0, 0)?;
                if !state.mission
                    || state.stage
                    || state.objective
                    || state.condition
                {
                    return Err(
                        "mission context structure disagrees with evidence"
                            .to_owned(),
                    );
                }
                state = MissionContextState::default();
            },
            "addstage" => {
                require_context_arity(invocation, 0, 3)?;
                if !state.mission
                    || state.stage
                    || state.objective
                    || state.condition
                {
                    return Err(
                        "mission context structure disagrees with evidence"
                            .to_owned(),
                    );
                }
                state.stage = true;
            },
            "closestage" => {
                require_context_arity(invocation, 0, 0)?;
                if !state.stage || state.objective || state.condition {
                    return Err(
                        "mission context structure disagrees with evidence"
                            .to_owned(),
                    );
                }
                state.stage = false;
            },
            "addobjective" => {
                require_context_arity(invocation, 1, 3)?;
                if !state.stage || state.objective || state.condition {
                    return Err(
                        "mission context structure disagrees with evidence"
                            .to_owned(),
                    );
                }
                state.objective = true;
            },
            "closeobjective" => {
                require_context_arity(invocation, 0, 0)?;
                if !state.objective || state.condition {
                    return Err(
                        "mission context structure disagrees with evidence"
                            .to_owned(),
                    );
                }
                state.objective = false;
            },
            "addcondition" => {
                require_context_arity(invocation, 1, 2)?;
                if !state.stage || state.condition {
                    return Err(
                        "mission context structure disagrees with evidence"
                            .to_owned(),
                    );
                }
                state.condition = true;
            },
            "closecondition" => {
                require_context_arity(invocation, 0, 0)?;
                if !state.condition
                    && !ignores_orphan_condition_close(
                        document,
                        invocation.ordinal,
                    )
                {
                    return Err(
                        "mission context structure disagrees with evidence"
                            .to_owned(),
                    );
                }
                state.condition = false;
            },
            _ => {},
        }
    }
    if state != MissionContextState::default() {
        return Err("mission context structure is unclosed".to_owned());
    }
    Ok(())
}

fn require_context_arity(
    invocation: &MissionCommandDocument,
    minimum: usize,
    maximum: usize,
) -> Result<(), String> {
    if (minimum..=maximum).contains(&invocation.arguments.len()) {
        Ok(())
    } else {
        Err("mission context command arity disagrees with evidence".to_owned())
    }
}

fn has_context_adaptation(
    document: &MissionScriptDocument,
    ordinal: usize,
    code: &str,
) -> bool {
    document.context_adaptations.iter().any(|adaptation| {
        adaptation.ordinal == ordinal && adaptation.code == code
    })
}

fn ignores_orphan_condition_close(
    document: &MissionScriptDocument,
    ordinal: usize,
) -> bool {
    has_context_adaptation(
        document,
        ordinal,
        "legacy-l2-m6sdi-ignore-orphan-condition-close-v1",
    )
}

fn closes_condition_before_invocation(
    document: &MissionScriptDocument,
    ordinal: usize,
) -> bool {
    has_context_adaptation(
        document,
        ordinal,
        "legacy-l7-m7i-close-keepbarrel-before-stage-complete-v1",
    )
}

fn validate_context_adaptation(
    document: &MissionScriptDocument,
    adaptation: &MissionContextAdaptationDocument,
) -> Result<(), String> {
    let valid = match adaptation.code.as_str() {
        "legacy-l2-m6sdi-ignore-orphan-condition-close-v1" => {
            adaptation.ordinal == 70
                && adaptation.command == "closecondition"
                && matches_invocation(document, 1, "selectmission", &["m6sd"])
                && matches_invocation(document, 68, "addstagemusicchange", &[])
                && matches_invocation(document, 69, "setstagemusicalwayson", &[
                ])
                && matches_invocation(document, 70, "closecondition", &[])
                && matches_invocation(document, 71, "closestage", &[])
        },
        "legacy-l7-m7i-close-keepbarrel-before-stage-complete-v1" => {
            adaptation.ordinal == 114
                && adaptation.command == "showstagecomplete"
                && matches_invocation(document, 1, "selectmission", &["m7"])
                && matches_invocation(document, 112, "stagestartmusicevent", &[
                    "L7_drama",
                ])
                && matches_invocation(document, 113, "addcondition", &[
                    "keepbarrel",
                    "2",
                ])
                && matches_invocation(document, 114, "showstagecomplete", &[])
                && matches_invocation(document, 115, "closestage", &[])
        },
        _ => false,
    };
    if valid {
        Ok(())
    } else {
        Err(
            "mission context adaptation is not reviewed for this evidence"
                .to_owned(),
        )
    }
}

fn matches_invocation(
    document: &MissionScriptDocument,
    ordinal: usize,
    name: &str,
    arguments: &[&str],
) -> bool {
    document.command_invocations.iter().any(|invocation| {
        invocation.ordinal == ordinal
            && invocation.name == name
            && invocation.arguments.len() == arguments.len()
            && invocation
                .arguments
                .iter()
                .zip(arguments.iter())
                .all(|(actual, expected)| actual == expected)
    })
}

fn validate_source_evidence(
    document: &MissionScriptDocument,
) -> Result<(), String> {
    if document.source_statements.len() != document.statement_count {
        return Err("mission source statement count is inconsistent".to_owned());
    }
    if document.source_statements.iter().any(|statement| {
        statement.is_empty() || has_unsafe_text_control(statement)
    }) {
        return Err("mission source statement evidence is malformed".to_owned());
    }

    let load_p3d_reference_count = document
        .command_invocations
        .iter()
        .filter(|invocation| invocation.name.contains("loadp3d"))
        .count();
    let mission_flow_command_count = document
        .command_invocations
        .iter()
        .filter(|invocation| {
            invocation.name.contains("stage")
                || invocation.name.contains("mission")
                || invocation.name.contains("objective")
        })
        .count();
    let vehicle_physics_command_count = document
        .command_invocations
        .iter()
        .filter(|invocation| {
            [
                "mass",
                "speed",
                "steer",
                "grip",
                "suspension",
                "gas",
                "brake",
            ]
            .iter()
            .any(|needle| invocation.name.contains(needle))
        })
        .count();
    if document.load_p3d_reference_count != load_p3d_reference_count
        || document.mission_flow_command_count != mission_flow_command_count
        || document.vehicle_physics_command_count
            != vehicle_physics_command_count
    {
        return Err("mission command summary is not reproducible".to_owned());
    }

    let p3d_references = document
        .command_invocations
        .iter()
        .filter(|invocation| invocation.name.contains("loadp3d"))
        .filter_map(|invocation| invocation.arguments.first().cloned())
        .collect::<Vec<_>>();
    if document.p3d_references != p3d_references {
        return Err(
            "mission P3D reference evidence is not reproducible".to_owned()
        );
    }
    if document.p3d_references.iter().any(|reference| {
        reference.is_empty() || has_unsafe_text_control(reference)
    }) {
        return Err("mission P3D reference evidence is malformed".to_owned());
    }
    Ok(())
}

fn validate_command_evidence(
    document: &MissionScriptDocument,
) -> Result<(), String> {
    if document.command_counts.len() != document.unique_command_count {
        return Err("mission unique command count is inconsistent".to_owned());
    }
    let total = document
        .command_counts
        .values()
        .try_fold(0usize, |sum, count| sum.checked_add(*count))
        .ok_or_else(|| "mission command count overflow".to_owned())?;
    if total != document.command_invocations.len() {
        return Err("mission command histogram is inconsistent".to_owned());
    }

    let mut observed = BTreeMap::<String, usize>::new();
    let mut previous_ordinal = 0usize;
    for invocation in &document.command_invocations {
        validate_invocation(
            invocation,
            previous_ordinal,
            document.statement_count,
        )?;
        let statement_index = invocation
            .ordinal
            .checked_sub(1)
            .ok_or_else(|| "mission command ordinal underflow".to_owned())?;
        let statement =
            document.source_statements.get(statement_index).ok_or_else(
                || "mission command source statement is missing".to_owned(),
            )?;
        if invocation.semantic_role != semantic_role_for_statement(statement) {
            return Err(
                "mission command semantic role is not reproducible".to_owned()
            );
        }
        previous_ordinal = invocation.ordinal;
        let count = observed.entry(invocation.name.clone()).or_default();
        *count = count
            .checked_add(1)
            .ok_or_else(|| "mission invocation count overflow".to_owned())?;
    }
    if observed != document.command_counts {
        return Err(
            "mission command histogram does not match invocations".to_owned()
        );
    }
    validate_invocation_replay(document)?;
    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReplayedMissionCommand {
    ordinal: usize,
    name: String,
    args_raw: String,
    arguments: Vec<String>,
}

fn validate_invocation_replay(
    document: &MissionScriptDocument,
) -> Result<(), String> {
    let mut replayed = Vec::<ReplayedMissionCommand>::new();
    for (index, statement) in document.source_statements.iter().enumerate() {
        let Some((name, args_raw)) = parse_source_call(statement) else {
            continue;
        };
        let ordinal = index.checked_add(1).ok_or_else(|| {
            "mission source statement ordinal overflow".to_owned()
        })?;
        replayed.push(ReplayedMissionCommand {
            ordinal,
            name: name.to_ascii_lowercase(),
            arguments: split_source_arguments(&args_raw),
            args_raw,
        });
    }
    if replayed.len() != document.command_invocations.len() {
        return Err("mission command invocation evidence is not reproducible"
            .to_owned());
    }
    for (expected, actual) in replayed.iter().zip(&document.command_invocations)
    {
        if expected.ordinal != actual.ordinal
            || expected.name != actual.name
            || expected.args_raw != actual.args_raw
            || expected.arguments != actual.arguments
        {
            return Err(
                "mission command invocation evidence is not reproducible"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn parse_source_call(statement: &str) -> Option<(&str, String)> {
    let open = statement.find('(')?;
    let mut in_quotes = false;
    let mut escaped = false;
    let mut depth = 0usize;
    let tail = statement.get(open..)?;
    for (offset, character) in tail.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' && in_quotes {
            escaped = true;
            continue;
        }
        if character == '"' {
            in_quotes = !in_quotes;
            continue;
        }
        if in_quotes {
            continue;
        }
        if character == '(' {
            depth = depth.saturating_add(1);
            continue;
        }
        if character == ')' {
            depth = depth.checked_sub(1)?;
            if depth == 0 {
                let close = open.checked_add(offset)?;
                let args_start = open.checked_add(1)?;
                return Some((
                    statement.get(..open)?.trim(),
                    statement.get(args_start..close)?.trim().to_owned(),
                ));
            }
        }
    }
    None
}

fn split_source_arguments(value: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escaped = false;
    let mut depth = 0usize;
    for character in value.chars() {
        if escaped {
            current.push(character);
            escaped = false;
            continue;
        }
        if character == '\\' && in_quotes {
            current.push(character);
            escaped = true;
            continue;
        }
        if character == '"' {
            in_quotes = !in_quotes;
            continue;
        }
        if !in_quotes {
            if character == '(' {
                depth = depth.saturating_add(1);
            } else if character == ')' {
                depth = depth.saturating_sub(1);
            } else if character == ',' && depth == 0 {
                push_source_argument(&mut args, &mut current);
                continue;
            }
        }
        current.push(character);
    }
    push_source_argument(&mut args, &mut current);
    args
}

fn push_source_argument(args: &mut Vec<String>, current: &mut String) {
    let value = current.trim().trim_matches('"').to_owned();
    if !value.is_empty() {
        args.push(value);
    }
    current.clear();
}

fn validate_invocation(
    invocation: &MissionCommandDocument,
    previous_ordinal: usize,
    statement_count: usize,
) -> Result<(), String> {
    if invocation.ordinal == 0
        || invocation.ordinal <= previous_ordinal
        || invocation.ordinal > statement_count
    {
        return Err("mission command ordinals are not canonical".to_owned());
    }
    if !is_command_name(&invocation.name)
        || invocation.semantic_role.is_empty()
        || invocation.semantic_role.chars().any(char::is_control)
        || has_unsafe_text_control(&invocation.args_raw)
        || invocation
            .arguments
            .iter()
            .any(|argument| has_unsafe_text_control(argument))
    {
        return Err("mission command invocation is malformed".to_owned());
    }
    Ok(())
}

fn semantic_role_for_statement(statement: &str) -> &'static str {
    let lower = statement.to_ascii_lowercase();
    if lower.contains("loadp3d") {
        "asset-load"
    } else if lower.contains("stage") {
        "mission-stage"
    } else if lower.contains("objective") {
        "mission-objective"
    } else if lower.contains("reward") {
        "mission-reward"
    } else {
        "mission-script"
    }
}

fn has_unsafe_text_control(value: &str) -> bool {
    value
        .chars()
        .any(|character| character != '\t' && character.is_control())
}

fn is_command_name(value: &str) -> bool {
    let mut bytes = value.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    (first.is_ascii_lowercase() || first == b'_')
        && bytes.all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_'
        })
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_script/tests.rs"]
mod tests;
