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
//   - Spt outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Spt outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Spt outbound adapter.

use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::domain::escape_json as json_escape;

/// To json.
pub(super) fn to_json(
    input: &Path,
    source_identity: &str,
) -> io::Result<String> {
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
            objects.push(sound_object_json(
                &current_kind,
                &current_name,
                &commands,
            ));
            current_kind.clear();
            current_name.clear();
            commands.clear();
            continue;
        }
        if !current_name.is_empty() {
            commands.push(sound_command_json(line));
        }
    }
    Ok(format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.sound-script.v1\",",
            "\"source\":\"{}\",",
            "\"object_count\":{},",
            "\"objects\":[{}]}}\n"
        ),
        json_escape(source_identity),
        objects.len(),
        objects.join(",")
    ))
}

/// Parse create line.
fn parse_create_line(line: &str) -> Option<(String, String)> {
    let rest = line.strip_prefix("create ")?;
    let (kind, name) = rest.split_once(" named ")?;
    Some((kind.trim().to_owned(), name.trim().to_owned()))
}

/// Sound object json.
fn sound_object_json(kind: &str, name: &str, commands: &[String]) -> String {
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
            (body.trim(), Some(option.trim()))
        } else {
            (line, None)
        };
    let name_end = body
        .find('(')
        .or_else(|| body.find(char::is_whitespace))
        .unwrap_or(body.len());
    let name = body.get(..name_end).unwrap_or_default().trim();
    let args = body
        .find('(')
        .and_then(|start| {
            body.rfind(')').map(|end| {
                body.get(start.saturating_add(1)..end).unwrap_or_default()
            })
        })
        .unwrap_or("")
        .trim();
    format!(
        "{{\"name\":\"{}\",\"arguments_raw\":\"{}\",\"option\":{},\"raw\":\"\
         {}\"}}",
        json_escape(name),
        json_escape(args),
        option.map_or_else(
            || "null".to_owned(),
            |value| format!("\"{}\"", json_escape(value))
        ),
        json_escape(line)
    )
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/one/spt/tests.rs"]
mod tests;
