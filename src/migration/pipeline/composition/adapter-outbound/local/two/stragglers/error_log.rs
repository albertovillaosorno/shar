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
//   - Error log outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Error log outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Error log outbound adapter.

use super::json::{JsonObject, json_string};

/// Append summary.
pub(super) fn append_summary(json: &mut JsonObject, text: &str) {
    let lines = error_lines(text);
    let source_lines = lines
        .iter()
        .map(|line| line.raw.clone())
        .collect::<Vec<_>>();
    json.number("line_count", u64::try_from(lines.len()).unwrap_or(u64::MAX));
    json.bool("runtime_import", false);
    json.field("disposition", "junk-build-artifact");
    json.string_array("source_lines", &source_lines);
    json.raw_json("error_lines", &error_lines_json(&lines));
}

/// Errorline.
struct ErrorLine {
    /// Ordinal.
    ordinal: usize,
    /// Raw.
    raw: String,
    /// Category.
    category: String,
}

/// Error lines.
fn error_lines(text: &str) -> Vec<ErrorLine> {
    let mut lines = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        lines.push(ErrorLine {
            ordinal: lines.len().saturating_add(1),
            raw: line.to_owned(),
            category: category_for(trimmed).to_owned(),
        });
    }
    lines
}

/// Category for.
fn category_for(value: &str) -> &'static str {
    let lower = value.to_ascii_lowercase();
    if contains_missing_command_phrase(&lower) {
        "missing-command"
    } else if contains_error_word(&lower) {
        "error"
    } else {
        "build-log"
    }
}

/// Return whether one normalized log line contains a missing-command phrase.
fn contains_missing_command_phrase(value: &str) -> bool {
    let mut previous = "";
    for word in
        value.split(|character: char| !character.is_ascii_alphanumeric())
    {
        if previous == "not" && (word == "found" || word == "recognized") {
            return true;
        }
        if !word.is_empty() {
            previous = word;
        }
    }
    false
}

/// Return whether one normalized log line contains an error word.
fn contains_error_word(value: &str) -> bool {
    for word in
        value.split(|character: char| !character.is_ascii_alphanumeric())
    {
        if word == "error" || word == "errors" {
            return true;
        }
    }
    false
}

/// Error lines json.
fn error_lines_json(lines: &[ErrorLine]) -> String {
    let mut out = String::from("[");
    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('{');
        out.push_str("\"ordinal\":");
        out.push_str(&line.ordinal.to_string());
        out.push_str(",\"category\":");
        out.push_str(&json_string(&line.category));
        out.push_str(",\"raw\":");
        out.push_str(&json_string(&line.raw));
        out.push('}');
    }
    out.push(']');
    out
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/stragglers/error_log/tests.rs"]
mod tests;
