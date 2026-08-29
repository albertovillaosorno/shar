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
//   - Fail-closed normalized vehicle-tuning evidence intake.
// - Must-Not:
//   - Interpret gameplay units, map tuning fields, or emit Unreal assets.
// - Allows:
//   - Validate exact schema, source replay, summaries, and command evidence.
// - Split-When:
//   - Vehicle tuning semantic compilation gains an independent lifecycle.
// - Merge-When:
//   - Another package domain owns identical config-script evidence.
// - Summary:
//   - Vehicle tuning semantic preflight.
// - Description:
//   - Accepts reproducible config-script evidence for later compilation.
// - Usage:
//   - Run before any vehicle tuning field mapping or Unreal construction.
// - Defaults:
//   - Stale, malformed, inconsistent, or non-reproducible input fails.
//

//! Normalized vehicle-tuning semantic preflight.

use std::collections::BTreeMap;

/// Exact normalized config-script schema accepted by tuning compilation.
pub const VEHICLE_TUNING_SCHEMA: &str =
    "shar-schoenwald.straggler.config-script.v2";

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
const VEHICLE_PHYSICS_NEEDLES: [&str; 7] = [
    "mass",
    "speed",
    "steer",
    "grip",
    "suspension",
    "gas",
    "brake",
];

/// One normalized tuning command retained without gameplay interpretation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VehicleTuningCommandInvocation {
    ordinal: usize,
    name: String,
    args_raw: String,
    semantic_role: String,
    arguments: Vec<String>,
}

impl VehicleTuningCommandInvocation {
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

/// Structurally clean normalized vehicle-tuning evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VehicleTuningEvidence {
    route_class: String,
    source_bytes: u64,
    source_statements: Vec<String>,
    invocations: Vec<VehicleTuningCommandInvocation>,
}

impl VehicleTuningEvidence {
    /// Return extraction routing provenance retained by the config script.
    #[must_use]
    pub fn route_class(&self) -> &str {
        &self.route_class
    }

    /// Return the exact original config-script byte count.
    #[must_use]
    pub const fn source_bytes(&self) -> u64 {
        self.source_bytes
    }

    /// Return normalized non-comment source statements in authored order.
    #[must_use]
    pub fn source_statements(&self) -> &[String] {
        &self.source_statements
    }

    /// Return canonical parsed command invocations in source order.
    #[must_use]
    pub fn invocations(&self) -> &[VehicleTuningCommandInvocation] {
        &self.invocations
    }
}

#[derive(Debug)]
pub(crate) struct VehicleTuningDocument {
    pub(crate) schema: String,
    pub(crate) source_extension: String,
    pub(crate) route_class: String,
    pub(crate) source_bytes: u64,
    pub(crate) context_command_count: usize,
    pub(crate) context_adaptation_count: usize,
    pub(crate) context_adaptations: Vec<()>,
    pub(crate) context_finding_count: usize,
    pub(crate) context_findings: Vec<()>,
    pub(crate) statement_count: usize,
    pub(crate) unique_command_count: usize,
    pub(crate) load_p3d_reference_count: usize,
    pub(crate) mission_flow_command_count: usize,
    pub(crate) vehicle_physics_command_count: usize,
    pub(crate) semantic_family: String,
    pub(crate) command_counts: BTreeMap<String, usize>,
    pub(crate) source_statements: Vec<String>,
    pub(crate) p3d_references: Vec<String>,
    pub(crate) command_invocations: Vec<VehicleTuningCommandDocument>,
}

#[derive(Debug)]
pub(crate) struct VehicleTuningCommandDocument {
    pub(crate) ordinal: usize,
    pub(crate) name: String,
    pub(crate) args_raw: String,
    pub(crate) semantic_role: String,
    pub(crate) arguments: Vec<String>,
}

/// Validate one normalized config-script document before semantic compilation.
///
/// # Errors
///
/// Returns an error when schema, structure, summaries, or command replay drift.
pub(crate) fn preflight_vehicle_tuning_document(
    document: VehicleTuningDocument,
) -> Result<VehicleTuningEvidence, String> {
    validate_identity(&document)?;
    validate_context_evidence(&document)?;
    validate_source_evidence(&document)?;
    validate_command_evidence(&document)?;
    Ok(VehicleTuningEvidence {
        route_class: document.route_class,
        source_bytes: document.source_bytes,
        source_statements: document.source_statements,
        invocations: document
            .command_invocations
            .into_iter()
            .map(|invocation| VehicleTuningCommandInvocation {
                ordinal: invocation.ordinal,
                name: invocation.name,
                args_raw: invocation.args_raw,
                semantic_role: invocation.semantic_role,
                arguments: invocation.arguments,
            })
            .collect(),
    })
}

fn validate_identity(document: &VehicleTuningDocument) -> Result<(), String> {
    if document.schema != VEHICLE_TUNING_SCHEMA {
        return Err(
            "normalized vehicle tuning schema is not supported".to_owned()
        );
    }
    if document.source_extension != "con"
        || !matches!(
            document.route_class.as_str(),
            "vehicle-config" | "mission"
        )
        || document.semantic_family != "vehicle-config-script"
    {
        return Err(
            "normalized vehicle tuning routing identity is invalid".to_owned()
        );
    }
    Ok(())
}

fn validate_context_evidence(
    document: &VehicleTuningDocument,
) -> Result<(), String> {
    if document.context_adaptation_count != document.context_adaptations.len()
        || document.context_finding_count != document.context_findings.len()
    {
        return Err("vehicle tuning context counts are inconsistent".to_owned());
    }
    if !document.context_adaptations.is_empty()
        || !document.context_findings.is_empty()
    {
        return Err(
            "vehicle tuning unexpectedly contains mission context evidence"
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
        return Err(
            "vehicle tuning context command count is inconsistent".to_owned()
        );
    }
    Ok(())
}

fn validate_source_evidence(
    document: &VehicleTuningDocument,
) -> Result<(), String> {
    if document.source_statements.len() != document.statement_count {
        return Err(
            "vehicle tuning source statement count is inconsistent".to_owned()
        );
    }
    if document.source_statements.iter().any(|statement| {
        statement.is_empty() || has_unsafe_text_control(statement)
    }) {
        return Err(
            "vehicle tuning source statement evidence is malformed".to_owned()
        );
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
            VEHICLE_PHYSICS_NEEDLES
                .iter()
                .any(|needle| invocation.name.contains(needle))
        })
        .count();
    if document.load_p3d_reference_count != load_p3d_reference_count
        || document.mission_flow_command_count != mission_flow_command_count
        || document.vehicle_physics_command_count
            != vehicle_physics_command_count
    {
        return Err(
            "vehicle tuning command summary is not reproducible".to_owned()
        );
    }
    let p3d_references = document
        .command_invocations
        .iter()
        .filter(|invocation| invocation.name.contains("loadp3d"))
        .filter_map(|invocation| invocation.arguments.first().cloned())
        .collect::<Vec<_>>();
    if document.p3d_references != p3d_references {
        return Err(
            "vehicle tuning P3D reference evidence is not reproducible"
                .to_owned(),
        );
    }
    if document.p3d_references.iter().any(|reference| {
        reference.is_empty() || has_unsafe_text_control(reference)
    }) {
        return Err(
            "vehicle tuning P3D reference evidence is malformed".to_owned()
        );
    }
    Ok(())
}

fn validate_command_evidence(
    document: &VehicleTuningDocument,
) -> Result<(), String> {
    if document.command_counts.len() != document.unique_command_count {
        return Err(
            "vehicle tuning unique command count is inconsistent".to_owned()
        );
    }
    let total = document
        .command_counts
        .values()
        .try_fold(0usize, |sum, count| sum.checked_add(*count))
        .ok_or_else(|| "vehicle tuning command count overflow".to_owned())?;
    if total != document.command_invocations.len() {
        return Err(
            "vehicle tuning command histogram is inconsistent".to_owned()
        );
    }
    let mut observed = BTreeMap::<String, usize>::new();
    let mut previous_ordinal = 0usize;
    for invocation in &document.command_invocations {
        validate_invocation(
            invocation,
            previous_ordinal,
            document.statement_count,
        )?;
        let statement_index = invocation.ordinal.checked_sub(1).ok_or_else(|| {
            "vehicle tuning command ordinal underflow".to_owned()
        })?;
        let statement = document
            .source_statements
            .get(statement_index)
            .ok_or_else(|| {
                "vehicle tuning command source statement is missing".to_owned()
            })?;
        if invocation.semantic_role != semantic_role_for_statement(statement) {
            return Err(
                "vehicle tuning semantic role is not reproducible".to_owned()
            );
        }
        previous_ordinal = invocation.ordinal;
        let count = observed.entry(invocation.name.clone()).or_default();
        *count = count.checked_add(1).ok_or_else(|| {
            "vehicle tuning invocation count overflow".to_owned()
        })?;
    }
    if observed != document.command_counts {
        return Err(
            "vehicle tuning command histogram does not match invocations"
                .to_owned(),
        );
    }
    validate_invocation_replay(document)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReplayedVehicleTuningCommand {
    ordinal: usize,
    name: String,
    args_raw: String,
    arguments: Vec<String>,
}

fn validate_invocation_replay(
    document: &VehicleTuningDocument,
) -> Result<(), String> {
    let mut replayed = Vec::<ReplayedVehicleTuningCommand>::new();
    for (index, statement) in document.source_statements.iter().enumerate() {
        let Some((name, args_raw)) = parse_source_call(statement) else {
            continue;
        };
        let ordinal = index.checked_add(1).ok_or_else(|| {
            "vehicle tuning source statement ordinal overflow".to_owned()
        })?;
        replayed.push(ReplayedVehicleTuningCommand {
            ordinal,
            name: name.to_ascii_lowercase(),
            arguments: split_source_arguments(&args_raw),
            args_raw,
        });
    }
    if replayed.len() != document.command_invocations.len() {
        return Err(
            "vehicle tuning invocation evidence is not reproducible".to_owned()
        );
    }
    for (expected, actual) in
        replayed.iter().zip(&document.command_invocations)
    {
        if expected.ordinal != actual.ordinal
            || expected.name != actual.name
            || expected.args_raw != actual.args_raw
            || expected.arguments != actual.arguments
        {
            return Err(
                "vehicle tuning invocation evidence is not reproducible"
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
    invocation: &VehicleTuningCommandDocument,
    previous_ordinal: usize,
    statement_count: usize,
) -> Result<(), String> {
    if invocation.ordinal == 0
        || invocation.ordinal <= previous_ordinal
        || invocation.ordinal > statement_count
    {
        return Err(
            "vehicle tuning command ordinals are not canonical".to_owned()
        );
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
        return Err("vehicle tuning command invocation is malformed".to_owned());
    }
    Ok(())
}

fn semantic_role_for_statement(statement: &str) -> &'static str {
    let lower = statement.to_ascii_lowercase();
    if lower.contains("mass")
        || lower.contains("speed")
        || lower.contains("grip")
    {
        "vehicle-physics"
    } else if lower.contains("sound") {
        "vehicle-sound"
    } else {
        "vehicle-config"
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
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/vehicle_tuning/tests.rs"]
mod tests;
