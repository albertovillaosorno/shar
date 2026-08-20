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
//   - Commands outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Commands outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Commands outbound adapter.

use std::collections::BTreeMap;
use std::path::Path;

use super::json::{JsonObject, json_string};

mod context;

/// Append summary.
// Ordered command evidence shares one accumulated summary map.
pub(super) fn append_summary(
    json: &mut JsonObject,
    text: &str,
    ext: &str,
    relative: &Path,
) {
    let mut counts = BTreeMap::<String, usize>::new();
    let mut statements = 0usize;
    let mut load_p3d = 0usize;
    let mut mission_flow = 0usize;
    let mut vehicle_physics = 0usize;
    let mut source_statements = Vec::<String>::new();
    let mut p3d_references = Vec::<String>::new();
    let mut invocations = Vec::<CommandInvocation>::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with("//")
            || trimmed.starts_with('#')
        {
            continue;
        }
        statements = statements.saturating_add(1);
        source_statements.push(trimmed.to_owned());
        if let Some((raw_command, args_raw)) = parse_call(trimmed) {
            let command = raw_command.trim().to_ascii_lowercase();
            if !command.is_empty() {
                let arguments = split_arguments(&args_raw);
                if command.contains("loadp3d") {
                    load_p3d = load_p3d.saturating_add(1);
                    if let Some(reference) = arguments.first() {
                        p3d_references.push(reference.clone());
                    }
                }
                if command.contains("stage")
                    || command.contains("mission")
                    || command.contains("objective")
                {
                    mission_flow = mission_flow.saturating_add(1);
                }
                if command.contains("mass")
                    || command.contains("speed")
                    || command.contains("steer")
                    || command.contains("grip")
                    || command.contains("suspension")
                    || command.contains("gas")
                    || command.contains("brake")
                {
                    vehicle_physics = vehicle_physics.saturating_add(1);
                }
                let count = counts.entry(command.clone()).or_insert(0);
                *count = count.saturating_add(1);
                invocations.push(CommandInvocation {
                    ordinal: statements,
                    name: command,
                    args_raw,
                    arguments,
                    semantic_role: semantic_role(trimmed, ext).to_owned(),
                });
            }
        }
    }

    let context_command_count = context::command_count(&invocations);
    let context_validation = if ext == "mfk" {
        context::validate(relative, &invocations)
    } else {
        context::validate(Path::new(""), &[])
    };
    json.number(
        "context_command_count",
        u64::try_from(context_command_count).unwrap_or(u64::MAX),
    );
    json.number(
        "context_adaptation_count",
        u64::try_from(context_validation.adaptations.len()).unwrap_or(u64::MAX),
    );
    json.raw_json(
        "context_adaptations",
        &context::adaptations_json(&context_validation.adaptations),
    );
    json.number(
        "context_finding_count",
        u64::try_from(context_validation.findings.len()).unwrap_or(u64::MAX),
    );
    json.raw_json(
        "context_findings",
        &context::findings_json(&context_validation.findings),
    );

    json.number(
        "statement_count",
        u64::try_from(statements).unwrap_or(u64::MAX),
    );
    json.number(
        "unique_command_count",
        u64::try_from(counts.len()).unwrap_or(u64::MAX),
    );
    json.number(
        "load_p3d_reference_count",
        u64::try_from(load_p3d).unwrap_or(u64::MAX),
    );
    json.number(
        "mission_flow_command_count",
        u64::try_from(mission_flow).unwrap_or(u64::MAX),
    );
    json.number(
        "vehicle_physics_command_count",
        u64::try_from(vehicle_physics).unwrap_or(u64::MAX),
    );
    json.field(
        "semantic_family",
        if ext == "con" {
            "vehicle-config-script"
        } else {
            "mission-script"
        },
    );
    json.map("command_counts", &counts);
    json.string_array("source_statements", &source_statements);
    json.string_array("p3d_references", &p3d_references);
    json.raw_json(
        "command_invocations",
        &command_invocations_json(&invocations),
    );
}

/// Schema for.
pub(super) fn schema_for(ext: &str) -> &'static str {
    if ext == "con" {
        "shar-schoenwald.straggler.config-script.v2"
    } else {
        "shar-schoenwald.straggler.mission-script.v3"
    }
}

/// Commandinvocation.
struct CommandInvocation {
    /// Ordinal.
    ordinal: usize,
    /// Name.
    name: String,
    /// Args raw.
    args_raw: String,
    /// Arguments.
    arguments: Vec<String>,
    /// Semantic role.
    semantic_role: String,
}

/// Command invocations json.
fn command_invocations_json(invocations: &[CommandInvocation]) -> String {
    let mut out = String::from("[");
    for (index, invocation) in invocations.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('{');
        out.push_str("\"ordinal\":");
        out.push_str(&invocation.ordinal.to_string());
        out.push_str(",\"name\":");
        out.push_str(&json_string(&invocation.name));
        out.push_str(",\"args_raw\":");
        out.push_str(&json_string(&invocation.args_raw));
        out.push_str(",\"semantic_role\":");
        out.push_str(&json_string(&invocation.semantic_role));
        out.push_str(",\"arguments\":[");
        for (arg_index, arg) in invocation.arguments.iter().enumerate() {
            if arg_index > 0 {
                out.push(',');
            }
            out.push_str(&json_string(arg));
        }
        out.push_str("]}");
    }
    out.push(']');
    out
}

/// Parse one command call without trailing comments.
fn parse_call(statement: &str) -> Option<(&str, String)> {
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

/// Split arguments.
fn split_arguments(value: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut escaped = false;
    let mut depth = 0usize;
    for ch in value.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' && in_quotes {
            current.push(ch);
            escaped = true;
            continue;
        }
        if ch == '"' {
            in_quotes = !in_quotes;
            continue;
        }
        if !in_quotes {
            if ch == '(' {
                depth = depth.saturating_add(1);
            } else if ch == ')' {
                depth = depth.saturating_sub(1);
            } else if ch == ',' && depth == 0 {
                push_arg(&mut args, &mut current);
                continue;
            }
        }
        current.push(ch);
    }
    push_arg(&mut args, &mut current);
    args
}

/// Push arg.
fn push_arg(args: &mut Vec<String>, current: &mut String) {
    let value = current.trim().trim_matches('"').to_owned();
    if !value.is_empty() {
        args.push(value);
    }
    current.clear();
}

/// Semantic role.
fn semantic_role(statement: &str, ext: &str) -> &'static str {
    let lower = statement.to_ascii_lowercase();
    if ext == "con" {
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
    } else if lower.contains("loadp3d") {
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

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/stragglers/commands/tests.rs"]
mod tests;
