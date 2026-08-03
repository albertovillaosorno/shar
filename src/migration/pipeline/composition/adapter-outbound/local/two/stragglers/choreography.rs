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
//   - Choreography outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Choreography outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Choreography outbound adapter.

use std::collections::BTreeMap;

use super::json::{JsonObject, json_string};

/// Append summary.
pub(super) fn append_summary(json: &mut JsonObject, text: &str) {
    let lines = choreography_lines(text);
    let mut keywords = BTreeMap::<String, usize>::new();
    let mut joint_mentions = 0usize;
    let mut rig_mentions = 0usize;
    let mut source_lines = Vec::<String>::new();
    for line in &lines {
        source_lines.push(line.raw.clone());
        if line.category == "joint" {
            joint_mentions = joint_mentions.saturating_add(1);
        }
        if line.category == "rig"
            || line.category == "skeleton"
            || line.category == "root"
        {
            rig_mentions = rig_mentions.saturating_add(1);
        }
        let count = keywords.entry(line.keyword.clone()).or_insert(0);
        *count = count.saturating_add(1);
    }
    json.number("line_count", u64::try_from(lines.len()).unwrap_or(u64::MAX));
    json.number(
        "joint_mention_count",
        u64::try_from(joint_mentions).unwrap_or(u64::MAX),
    );
    json.number(
        "rig_mention_count",
        u64::try_from(rig_mentions).unwrap_or(u64::MAX),
    );
    json.map("keyword_counts", &keywords);
    json.string_array("source_lines", &source_lines);
    json.raw_json("choreography_lines", &choreography_lines_json(&lines));
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
        lines.push(ChoreographyLine {
            ordinal: lines.len().saturating_add(1),
            keyword,
            category,
            raw: trimmed.to_owned(),
        });
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
    for (index, line) in lines.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('{');
        out.push_str("\"ordinal\":");
        out.push_str(&line.ordinal.to_string());
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
