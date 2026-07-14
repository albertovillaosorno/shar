// File:
//   - error_log.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/stragglers/error_log.rs
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
//   - The error log contract for pipeline phase two stragglers.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute error log.
// - Split-When:
//   - Split when error log contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Error log for pipeline phase two stragglers.
// - Description:
//   - Defines error log data and behavior for pipeline phase two stragglers.
// - Usage:
//   - Used by pipeline phase two stragglers code that needs error log.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Error log for pipeline phase two stragglers.
//!
//! This boundary keeps error log for pipeline phase two stragglers explicit
//! and returns deterministic results to pipeline callers.
use super::json::{JsonObject, json_string};

/// Append summary.
pub(super) fn append_summary(
    json: &mut JsonObject,
    text: &str,
) {
    let lines = error_lines(text);
    let source_lines = lines
        .iter()
        .map(
            |line| {
                line.raw
                    .clone()
            },
        )
        .collect::<Vec<_>>();
    json.number(
        "line_count",
        u64::try_from(lines.len()).unwrap_or(u64::MAX),
    );
    json.bool(
        "runtime_import",
        false,
    );
    json.field(
        "disposition",
        "junk-build-artifact",
    );
    json.string_array(
        "source_lines",
        &source_lines,
    );
    json.raw_json(
        "error_lines",
        &error_lines_json(&lines),
    );
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
        lines.push(
            ErrorLine {
                ordinal: lines
                    .len()
                    .saturating_add(1),
                raw: line.to_owned(),
                category: category_for(trimmed).to_owned(),
            },
        );
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
    for (index, line) in lines
        .iter()
        .enumerate()
    {
        if index > 0 {
            out.push(',');
        }
        out.push('{');
        out.push_str("\"ordinal\":");
        out.push_str(
            &line
                .ordinal
                .to_string(),
        );
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
mod tests {
    use super::super::json::JsonObject;
    use super::append_summary;

    #[test]
    fn does_not_classify_embedded_missing_phrases() {
        assert_eq!(
            super::category_for("feature is not foundational"),
            "build-log"
        );
    }

    #[test]
    fn does_not_classify_embedded_error_substrings() {
        assert_eq!(
            super::category_for("terror level increased"),
            "build-log"
        );
    }

    #[test]
    fn preserves_nonempty_error_line_whitespace() -> Result<(), String> {
        let mut json = JsonObject::new();
        append_summary(
            &mut json,
            "  ERROR failed  ",
        );
        let output = json.finish();
        if output.contains("\"source_lines\":[\"  ERROR failed  \"]")
            && output.contains("\"raw\":\"  ERROR failed  \"")
        {
            Ok(())
        } else {
            Err(format!("error-line whitespace was lost: {output}"))
        }
    }
}
