// File:
//   - choreography.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/stragglers/choreography.rs
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
//   - The choreography contract for pipeline phase two stragglers.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute choreography.
// - Split-When:
//   - Split when choreography contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Choreography for pipeline phase two stragglers.
// - Description:
//   - Defines choreography data and behavior for pipeline phase two
//   - stragglers.
// - Usage:
//   - Used by pipeline phase two stragglers code that needs choreography.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Choreography for pipeline phase two stragglers.
//!
//! This boundary keeps choreography for pipeline phase two stragglers explicit
//! and returns deterministic results to pipeline callers.
use std::collections::BTreeMap;

use super::json::{JsonObject, json_string};

/// Append summary.
pub(super) fn append_summary(
    json: &mut JsonObject,
    text: &str,
) {
    let lines = choreography_lines(text);
    let mut keywords = BTreeMap::<String, usize>::new();
    let mut joint_mentions = 0usize;
    let mut rig_mentions = 0usize;
    let mut source_lines = Vec::<String>::new();
    for line in &lines {
        source_lines.push(
            line.raw
                .clone(),
        );
        if line.category == "joint" {
            joint_mentions = joint_mentions.saturating_add(1);
        }
        if line.category == "rig"
            || line.category == "skeleton"
            || line.category == "root"
        {
            rig_mentions = rig_mentions.saturating_add(1);
        }
        let count = keywords
            .entry(
                line.keyword
                    .clone(),
            )
            .or_insert(0);
        *count = count.saturating_add(1);
    }
    json.number(
        "line_count",
        u64::try_from(lines.len()).unwrap_or(u64::MAX),
    );
    json.number(
        "joint_mention_count",
        u64::try_from(joint_mentions).unwrap_or(u64::MAX),
    );
    json.number(
        "rig_mention_count",
        u64::try_from(rig_mentions).unwrap_or(u64::MAX),
    );
    json.map(
        "keyword_counts",
        &keywords,
    );
    json.string_array(
        "source_lines",
        &source_lines,
    );
    json.raw_json(
        "choreography_lines",
        &choreography_lines_json(&lines),
    );
}

/// Choreographyline.
struct ChoreographyLine {
    /// Ordinal.
    ordinal: usize,
    /// Keyword.
    keyword: String,
    /// Category.
    category: String,
    /// Raw.
    raw: String,
}

/// Choreography lines.
fn choreography_lines(text: &str) -> Vec<ChoreographyLine> {
    let mut lines = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('-') {
            continue;
        }
        let keyword = trimmed
            .split_whitespace()
            .next()
            .unwrap_or("line")
            .to_ascii_lowercase();
        let category = category_for(trimmed).to_owned();
        lines.push(
            ChoreographyLine {
                ordinal: lines
                    .len()
                    .saturating_add(1),
                keyword,
                category,
                raw: trimmed.to_owned(),
            },
        );
    }
    lines
}

/// Category for.
fn category_for(value: &str) -> &'static str {
    let lower = value.to_ascii_lowercase();
    if lower.contains("joint") {
        "joint"
    } else if lower.contains("skeleton") {
        "skeleton"
    } else if lower.contains("rig") {
        "rig"
    } else if lower.contains("root") {
        "root"
    } else if lower.contains("motion") || lower.contains("anim") {
        "animation"
    } else {
        "metadata"
    }
}

/// Choreography lines json.
fn choreography_lines_json(lines: &[ChoreographyLine]) -> String {
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
        out.push_str(",\"keyword\":");
        out.push_str(&json_string(&line.keyword));
        out.push_str(",\"category\":");
        out.push_str(&json_string(&line.category));
        out.push_str(",\"raw\":");
        out.push_str(&json_string(&line.raw));
        out.push('}');
    }
    out.push(']');
    out
}
