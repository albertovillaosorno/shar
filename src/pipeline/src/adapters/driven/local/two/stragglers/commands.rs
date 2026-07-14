// File:
//   - commands.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/stragglers/commands.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - The commands contract for pipeline phase two stragglers.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute commands.
// - Split-When:
//   - Split when commands contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Commands for pipeline phase two stragglers.
// - Description:
//   - Defines commands data and behavior for pipeline phase two stragglers.
// - Usage:
//   - Used by pipeline phase two stragglers code that needs commands.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Commands for pipeline phase two stragglers.
//!
//! This boundary keeps commands for pipeline phase two stragglers explicit and
//! returns deterministic results to pipeline callers.
use std::collections::BTreeMap;

use super::json::{JsonObject, json_string};

/// Append summary.
// Ordered command evidence shares one accumulated summary map.
#[expect(
    clippy::too_many_lines,
    reason = "Command summaries classify one token stream through ordered \
              evidence rules that share accumulated counts."
)]
pub(super) fn append_summary(
    json: &mut JsonObject,
    text: &str,
    ext: &str,
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
        if let Some((raw_command, args)) = trimmed.split_once('(') {
            let command = raw_command
                .trim()
                .to_ascii_lowercase();
            if !command.is_empty() {
                let args_raw = args
                    .trim_end_matches(';')
                    .trim_end_matches(')')
                    .trim()
                    .to_owned();
                let arguments = split_arguments(&args_raw);
                if command.contains("loadp3d") {
                    load_p3d = load_p3d.saturating_add(1);
                    p3d_references.extend(
                        arguments
                            .iter()
                            .cloned(),
                    );
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
                let count = counts
                    .entry(command.clone())
                    .or_insert(0);
                *count = count.saturating_add(1);
                invocations.push(
                    CommandInvocation {
                        ordinal: statements,
                        name: command,
                        args_raw,
                        arguments,
                        semantic_role: semantic_role(
                            trimmed, ext,
                        )
                        .to_owned(),
                    },
                );
            }
        }
    }

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
    json.map(
        "command_counts",
        &counts,
    );
    json.string_array(
        "source_statements",
        &source_statements,
    );
    json.string_array(
        "p3d_references",
        &p3d_references,
    );
    json.raw_json(
        "command_invocations",
        &command_invocations_json(&invocations),
    );
}

/// Schema for.
pub(super) fn schema_for(ext: &str) -> &'static str {
    if ext == "con" {
        "shar-schoenwald.straggler.config-script.v1"
    } else {
        "shar-schoenwald.straggler.mission-script.v1"
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
    for (index, invocation) in invocations
        .iter()
        .enumerate()
    {
        if index > 0 {
            out.push(',');
        }
        out.push('{');
        out.push_str("\"ordinal\":");
        out.push_str(
            &invocation
                .ordinal
                .to_string(),
        );
        out.push_str(",\"name\":");
        out.push_str(&json_string(&invocation.name));
        out.push_str(",\"args_raw\":");
        out.push_str(&json_string(&invocation.args_raw));
        out.push_str(",\"semantic_role\":");
        out.push_str(&json_string(&invocation.semantic_role));
        out.push_str(",\"arguments\":[");
        for (arg_index, arg) in invocation
            .arguments
            .iter()
            .enumerate()
        {
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

/// Split arguments.
fn split_arguments(value: &str) -> Vec<String> {
    let mut args = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    for ch in value.chars() {
        if ch == '"' {
            in_quotes = !in_quotes;
            continue;
        }
        if ch == ',' && !in_quotes {
            push_arg(
                &mut args,
                &mut current,
            );
        } else {
            current.push(ch);
        }
    }
    push_arg(
        &mut args,
        &mut current,
    );
    args
}

/// Push arg.
fn push_arg(
    args: &mut Vec<String>,
    current: &mut String,
) {
    let value = current
        .trim()
        .trim_matches('"')
        .to_owned();
    if !value.is_empty() {
        args.push(value);
    }
    current.clear();
}

/// Semantic role.
fn semantic_role(
    statement: &str,
    ext: &str,
) -> &'static str {
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
