// File:
//   - json.rs
// Path:
//   - src/game-manifest/src/domain/json.rs
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
//   - Canonical JSON string escaping for game-manifest records.
// - Must-Not:
//   - Parse manifest schemas or own record classification decisions.
// - Allows:
//   - Escaping caller-provided text into valid JSON string contents.
// - Split-When:
//   - Split when escaping and parsing become independently reusable contracts.
// - Merge-When:
//   - Another game-manifest module owns the same JSON string boundary.
// - Summary:
//   - Provides dependency-free canonical JSON string escaping.
// - Description:
//   - Converts reserved and control characters without changing Unicode text.
// - Usage:
//   - Called by manifest serializers before embedding string field values.
// - Defaults:
//   - Non-control Unicode scalar values are preserved exactly.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Dependency-free JSON string escaping for manifest records.
//!
//! This module keeps the serializer valid for adversarial path-derived values
//! while avoiding a broad serialization dependency at the narrow ledger edge.

use super::{DirCount, KIND_TAXONOMY};

impl DirCount {
    /// Serializes the record to a single canonical JSONL line, without a
    /// trailing newline.
    #[must_use]
    pub fn to_jsonl(&self) -> String {
        let mut line = format!(
            "{{\"dir\":\"{}\",\"ext\":\"{}\",\"min\":{}",
            escape(&self.dir),
            escape(&self.extension),
            self.min_count
        );
        line.push_str(",\"kind\":\"");
        line.push_str(&escape(&self.kind));
        line.push('"');
        line.push('}');
        line
    }
    /// Parses one canonical JSONL line produced by [`DirCount::to_jsonl`].
    /// Returns `None` for blank lines, comment lines (starting with `#`), or
    /// lines that do not match the canonical shape.
    #[must_use]
    pub fn parse(line: &str) -> Option<Self> {
        if line.trim() != line {
            return None;
        }
        if line.is_empty() || line.starts_with('#') {
            return None;
        }
        let inner = line
            .strip_prefix("{\"dir\":\"")?
            .strip_suffix('}')?;
        let (dir, rest) = inner.split_once("\",\"ext\":\"")?;
        let (extension, minimum_and_kind) = rest.split_once("\",\"min\":")?;
        let (minimum_token, kind_token) =
            minimum_and_kind.split_once(",\"kind\":\"")?;
        if minimum_token.is_empty()
            || (minimum_token.len() > 1 && minimum_token.starts_with('0'))
            || !minimum_token
                .chars()
                .all(|character| character.is_ascii_digit())
        {
            return None;
        }
        let min_count = minimum_token
            .parse::<usize>()
            .ok()?;
        let kind = unescape(kind_token.strip_suffix('"')?)?;
        if !KIND_TAXONOMY.contains(&kind.as_str()) {
            return None;
        }
        Some(
            Self {
                dir: unescape(dir)?,
                extension: unescape(extension)?,
                min_count,
                kind,
            },
        )
    }
}

/// Escapes one string for canonical JSON output.
fn escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\u{08}' => escaped.push_str("\\b"),
            '\u{0c}' => escaped.push_str("\\f"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            control if control <= '\u{1f}' => {
                let code = u32::from(control);
                escaped.push_str("\\u00");
                escaped.push(
                    char::from_digit(
                        code >> 4,
                        16_u32,
                    )
                    .unwrap_or('0'),
                );
                escaped.push(
                    char::from_digit(
                        code & 0x0f,
                        16_u32,
                    )
                    .unwrap_or('0'),
                );
            }
            other => escaped.push(other),
        }
    }
    escaped
}

/// Decodes canonical JSON string contents.
fn unescape(value: &str) -> Option<String> {
    let mut output = String::with_capacity(value.len());
    let mut characters = value.chars();
    while let Some(character) = characters.next() {
        if character <= '\u{1f}' {
            return None;
        }
        if character != '\\' {
            output.push(character);
            continue;
        }
        match characters.next()? {
            '"' => output.push('"'),
            '\\' => output.push('\\'),
            '/' => output.push('/'),
            'b' => output.push('\u{08}'),
            'f' => output.push('\u{0c}'),
            'n' => output.push('\n'),
            'r' => output.push('\r'),
            't' => output.push('\t'),
            'u' => {
                let first = read_hex_quad(&mut characters)?;
                if (0xd800_u16..=0xdbff_u16).contains(&first) {
                    if characters.next()? != '\\' || characters.next()? != 'u' {
                        return None;
                    }
                    let second = read_hex_quad(&mut characters)?;
                    if !(0xdc00_u16..=0xdfff_u16).contains(&second) {
                        return None;
                    }
                    let high = u32::from(first.checked_sub(0xd800_u16)?);
                    let low = u32::from(second.checked_sub(0xdc00_u16)?);
                    let scalar = high
                        .checked_shl(10_u32)?
                        .checked_add(low)?
                        .checked_add(0x1_0000_u32)?;
                    output.push(char::from_u32(scalar)?);
                } else if (0xdc00_u16..=0xdfff_u16).contains(&first) {
                    return None;
                } else {
                    output.push(char::from_u32(u32::from(first))?);
                }
            }
            _ => return None,
        }
    }
    Some(output)
}

/// Reads one four-digit JSON Unicode escape.
fn read_hex_quad(characters: &mut std::str::Chars<'_>) -> Option<u16> {
    let mut value = 0_u16;
    for _ in 0_u8..4_u8 {
        let digit = characters
            .next()?
            .to_digit(16_u32)?;
        value = value
            .checked_mul(16_u16)?
            .checked_add(u16::try_from(digit).ok()?)?;
    }
    Some(value)
}
