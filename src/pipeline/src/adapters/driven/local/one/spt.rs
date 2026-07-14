// File:
//   - spt.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/one/spt.rs
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
//   - The spt contract for pipeline phase one.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute spt.
// - Split-When:
//   - Split when spt contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Spt for pipeline phase one.
// - Description:
//   - Defines spt data and behavior for pipeline phase one.
// - Usage:
//   - Used by pipeline phase one code that needs spt.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Spt for pipeline phase one.
//!
//! This boundary keeps spt for pipeline phase one explicit and returns
//! deterministic results to pipeline callers.
use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::domain::escape_json as json_escape;

/// To json.
pub(super) fn to_json(input: &Path) -> io::Result<String> {
    let text = local::read_utf8(input)?;
    let mut objects = Vec::new();
    let mut current_kind = String::new();
    let mut current_name = String::new();
    let mut commands = Vec::new();
    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some((kind, name)) = parse_create_line(line) {
            current_kind = kind;
            current_name = name;
            commands.clear();
            continue;
        }
        if line == "{" {
            continue;
        }
        if line == "}" {
            objects.push(
                sound_object_json(
                    &current_kind,
                    &current_name,
                    &commands,
                ),
            );
            current_kind.clear();
            current_name.clear();
            commands.clear();
            continue;
        }
        if !current_name.is_empty() {
            commands.push(sound_command_json(line));
        }
    }
    Ok(
        format!(
            concat!(
                "{{\"schema\":\"shar-schoenwald.sound-script.v1\",",
                "\"source\":\"{}\",",
                "\"object_count\":{},",
                "\"objects\":[{}]}}\n"
            ),
            json_escape(
                &input
                    .display()
                    .to_string()
            ),
            objects.len(),
            objects.join(",")
        ),
    )
}

/// Parse create line.
fn parse_create_line(
    line: &str
) -> Option<(
    String,
    String,
)> {
    let rest = line.strip_prefix("create ")?;
    let (kind, name) = rest.split_once(" named ")?;
    Some(
        (
            kind.trim()
                .to_owned(),
            name.trim()
                .to_owned(),
        ),
    )
}

/// Sound object json.
fn sound_object_json(
    kind: &str,
    name: &str,
    commands: &[String],
) -> String {
    format!(
        "{{\"kind\":\"{}\",\"name\":\"{}\",\"command_count\":{},\"commands\":\
         [{}]}}",
        json_escape(kind),
        json_escape(name),
        commands.len(),
        commands.join(",")
    )
}

/// Sound command json.
fn sound_command_json(line: &str) -> String {
    let (body, option) =
        if let Some((body, option)) = line.split_once(" option ") {
            (
                body.trim(),
                Some(option.trim()),
            )
        } else {
            (
                line, None,
            )
        };
    let name_end = body
        .find('(')
        .or_else(|| body.find(char::is_whitespace))
        .unwrap_or(body.len());
    let name = body
        .get(..name_end)
        .unwrap_or_default()
        .trim();
    let args = body
        .find('(')
        .and_then(
            |start| {
                body.rfind(')')
                    .map(
                        |end| {
                            body.get(start.saturating_add(1)..end)
                                .unwrap_or_default()
                        },
                    )
            },
        )
        .unwrap_or("")
        .trim();
    format!(
        "{{\"name\":\"{}\",\"arguments_raw\":\"{}\",\"option\":{},\"raw\":\"\
         {}\"}}",
        json_escape(name),
        json_escape(args),
        option.map_or_else(
            || "null".to_owned(),
            |value| format!(
                "\"{}\"",
                json_escape(value)
            )
        ),
        json_escape(line)
    )
}
