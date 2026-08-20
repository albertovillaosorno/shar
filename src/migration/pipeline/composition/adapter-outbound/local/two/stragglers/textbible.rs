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
//   - Textbible outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Textbible outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Textbible outbound adapter.

use super::json::{JsonObject, json_string};

/// Append summary.
pub(super) fn append_summary(json: &mut JsonObject, text: &str, ext: &str) {
    let entries = entries(text);
    let numeric_keys =
        entries.iter().filter(|entry| entry.key_is_numeric).count();
    let max_line_len = entries
        .iter()
        .map(|entry| entry.raw.len())
        .max()
        .unwrap_or(0);
    let source_entries = entries
        .iter()
        .map(|entry| entry.raw.clone())
        .collect::<Vec<_>>();

    json.field("language_channel", language_channel(ext));
    json.number(
        "entry_count",
        u64::try_from(entries.len()).unwrap_or(u64::MAX),
    );
    json.number(
        "numeric_key_count",
        u64::try_from(numeric_keys).unwrap_or(u64::MAX),
    );
    json.number(
        "max_line_length",
        u64::try_from(max_line_len).unwrap_or(u64::MAX),
    );
    json.string_array("source_entries", &source_entries);
    json.raw_json("entries", &entries_json(&entries));
}

/// Textentry.
struct TextEntry {
    /// Ordinal.
    ordinal: usize,
    /// Key.
    key: String,
    /// Value.
    value: String,
    /// Raw.
    raw: String,
    /// Key is numeric.
    key_is_numeric: bool,
}

/// Entries.
fn entries(text: &str) -> Vec<TextEntry> {
    let mut entries = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        let (key, value) = split_key_value(trimmed);
        entries.push(TextEntry {
            ordinal: entries.len().saturating_add(1),
            key_is_numeric: !key.is_empty()
                && key.chars().all(|character| character.is_ascii_hexdigit()),
            key,
            value,
            raw: trimmed.to_owned(),
        });
    }
    entries
}

/// Split key value.
fn split_key_value(value: &str) -> (String, String) {
    if let Some((key, rest)) = value.split_once('=') {
        (key.trim().to_owned(), rest.trim().to_owned())
    } else if let Some((key, rest)) = value.split_once(char::is_whitespace) {
        (key.trim().to_owned(), rest.trim().to_owned())
    } else {
        (value.to_owned(), String::new())
    }
}

/// Entries json.
fn entries_json(entries: &[TextEntry]) -> String {
    let mut out = String::from("[");
    for (index, entry) in entries.iter().enumerate() {
        if index > 0 {
            out.push(',');
        }
        out.push('{');
        out.push_str("\"ordinal\":");
        out.push_str(&entry.ordinal.to_string());
        out.push_str(",\"key\":");
        out.push_str(&json_string(&entry.key));
        out.push_str(",\"value\":");
        out.push_str(&json_string(&entry.value));
        out.push_str(",\"raw\":");
        out.push_str(&json_string(&entry.raw));
        out.push_str(",\"key_is_numeric\":");
        out.push_str(if entry.key_is_numeric {
            "true"
        } else {
            "false"
        });
        out.push('}');
    }
    out.push(']');
    out
}

/// Language channel.
fn language_channel(ext: &str) -> &'static str {
    match ext {
        "e" => "english",
        "f" => "french",
        "g" => "german",
        "i" => "italian-stub",
        "s" => "spanish",
        "x" => "unknown-variant",
        "txt" => "source-text",
        _ => "none",
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/stragglers/textbible/tests.rs"]
mod tests;
