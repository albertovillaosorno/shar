// File:
//   - json.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/stragglers/json.rs
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
//   - The json contract for pipeline phase two stragglers.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute json.
// - Split-When:
//   - Split when json contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Json for pipeline phase two stragglers.
// - Description:
//   - Defines json data and behavior for pipeline phase two stragglers.
// - Usage:
//   - Used by pipeline phase two stragglers code that needs json.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Json for pipeline phase two stragglers.
//!
//! This boundary keeps json for pipeline phase two stragglers explicit and
//! returns deterministic results to pipeline callers.
use std::collections::BTreeMap;

use crate::domain::escape_json as escape;

/// Jsonobject.
pub(super) struct JsonObject {
    /// Text.
    text: String,
    /// Has fields.
    has_fields: bool,
}

impl JsonObject {
    /// New.
    pub(super) fn new() -> Self {
        Self {
            text: String::from("{"),
            has_fields: false,
        }
    }

    /// Field.
    pub(super) fn field(
        &mut self,
        name: &str,
        value: &str,
    ) {
        self.separator();
        self.text
            .push('"');
        self.text
            .push_str(name);
        self.text
            .push_str("\":\"");
        self.text
            .push_str(&escape(value));
        self.text
            .push('"');
    }

    /// Number.
    pub(super) fn number(
        &mut self,
        name: &str,
        value: u64,
    ) {
        self.separator();
        self.text
            .push('"');
        self.text
            .push_str(name);
        self.text
            .push_str("\":");
        self.text
            .push_str(&value.to_string());
    }

    /// Bool.
    pub(super) fn bool(
        &mut self,
        name: &str,
        value: bool,
    ) {
        self.separator();
        self.text
            .push('"');
        self.text
            .push_str(name);
        self.text
            .push_str("\":");
        self.text
            .push_str(
                if value {
                    "true"
                } else {
                    "false"
                },
            );
    }

    /// Map.
    pub(super) fn map(
        &mut self,
        name: &str,
        values: &BTreeMap<String, usize>,
    ) {
        self.separator();
        self.text
            .push('"');
        self.text
            .push_str(name);
        self.text
            .push_str("\":{");
        for (index, (key, value)) in values
            .iter()
            .enumerate()
        {
            if index > 0 {
                self.text
                    .push(',');
            }
            self.text
                .push('"');
            self.text
                .push_str(&escape(key));
            self.text
                .push_str("\":");
            self.text
                .push_str(&value.to_string());
        }
        self.text
            .push('}');
    }

    /// String array.
    pub(super) fn string_array(
        &mut self,
        name: &str,
        values: &[String],
    ) {
        self.separator();
        self.text
            .push('"');
        self.text
            .push_str(name);
        self.text
            .push_str("\":[");
        for (index, value) in values
            .iter()
            .enumerate()
        {
            if index > 0 {
                self.text
                    .push(',');
            }
            self.text
                .push('"');
            self.text
                .push_str(&escape(value));
            self.text
                .push('"');
        }
        self.text
            .push(']');
    }

    /// Raw json.
    pub(super) fn raw_json(
        &mut self,
        name: &str,
        value: &str,
    ) {
        self.separator();
        self.text
            .push('"');
        self.text
            .push_str(name);
        self.text
            .push_str("\":");
        self.text
            .push_str(value);
    }

    /// Finish.
    pub(super) fn finish(mut self) -> String {
        self.text
            .push('}');
        self.text
    }

    /// Separator.
    fn separator(&mut self) {
        if self.has_fields {
            self.text
                .push(',');
        }
        self.has_fields = true;
    }
}

/// Append one normalized printable run only when it remains meaningful.
fn append_printable_token(
    tokens: &mut Vec<String>,
    current: &[u8],
) {
    let token = String::from_utf8_lossy(current)
        .trim()
        .to_owned();
    if token.len() >= 4 {
        tokens.push(token);
    }
}

/// Printable tokens.
pub(super) fn printable_tokens(bytes: &[u8]) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = Vec::new();
    for byte in bytes {
        if byte.is_ascii_graphic() || *byte == b' ' {
            current.push(*byte);
        } else {
            append_printable_token(
                &mut tokens,
                &current,
            );
            current.clear();
        }
    }
    append_printable_token(
        &mut tokens,
        &current,
    );
    tokens
}

/// Json string.
pub(super) fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    out.push_str(&escape(value));
    out.push('"');
    out
}

#[cfg(test)]
mod tests {
    use super::printable_tokens;

    #[test]
    fn rejects_tokens_shortened_below_minimum_after_trim() -> Result<(), String>
    {
        let tokens = printable_tokens(b"    \0 ab \0valid");
        if tokens == ["valid".to_owned()] {
            Ok(())
        } else {
            Err(format!("unexpected printable tokens: {tokens:?}"))
        }
    }
}
