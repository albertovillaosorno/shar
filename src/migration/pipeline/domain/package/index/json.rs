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
//   - Package-index JSON lexical parsing.
// - Must-Not:
//   - Own filesystem reads or package classification policy.
// - Allows:
//   - Fail-closed JSON field discovery and scalar decoding.
// - Split-When:
//   - Split when one JSON value family gains independent grammar rules.
// - Merge-When:
//   - Merge when another module owns the identical lexical grammar.
// - Summary:
//   - Package-index JSON lexical parser.
// - Description:
//   - Parses canonical package-index fields without accepting aliases.
// - Usage:
//   - Used by the owning package-index domain module.
// - Defaults:
//   - Unknown, duplicate, or malformed JSON fields fail explicitly.
//

//! Package-index JSON lexical parser.

use super::{PackageIntakeError, PackageRole};

/// Reads one required nonnegative integer with canonical delimiters.
pub(super) fn extract_usize_field(
    row: &str,
    field: &str,
) -> Result<usize, PackageIntakeError> {
    let start = value_cursor(row, field)?;
    let bytes = row.as_bytes();
    let mut end = start;
    while bytes.get(end).is_some_and(u8::is_ascii_digit) {
        end = end.saturating_add(1);
    }
    if end == start {
        return Err(PackageIntakeError::new(format!(
            "field {field} is not a nonnegative integer"
        )));
    }
    if end.saturating_sub(start) > 1 && bytes.get(start) == Some(&b'0') {
        return Err(PackageIntakeError::new(format!(
            "field {field} has a leading zero"
        )));
    }
    let delimiter = skip_json_ws(row, end);
    if !matches!(bytes.get(delimiter), Some(b',' | b'}')) {
        return Err(PackageIntakeError::new(format!(
            "field {field} has malformed integer syntax"
        )));
    }
    row.get(start..end)
        .ok_or_else(|| PackageIntakeError::new("invalid integer field range"))?
        .parse::<usize>()
        .map_err(|error| {
            PackageIntakeError::new(format!(
                "field {field} integer overflow: {error}"
            ))
        })
}

/// Require one decoded field value to end at an object delimiter.
fn validate_field_value_end(
    row: &str,
    end: usize,
    field: &str,
) -> Result<(), PackageIntakeError> {
    let delimiter = skip_json_ws(row, end);
    if matches!(row.as_bytes().get(delimiter), Some(b',' | b'}')) {
        return Ok(());
    }
    Err(PackageIntakeError::new(format!(
        "field {field} has trailing JSON content"
    )))
}

/// Reads one required JSON string without accepting alternate field shapes.
pub(super) fn extract_string_field(
    row: &str,
    field: &str,
) -> Result<String, PackageIntakeError> {
    let cursor = value_cursor(row, field)?;
    let bytes = row.as_bytes();
    if bytes.get(cursor) != Some(&b'"') {
        return Err(PackageIntakeError::new(format!(
            "field {field} is not a string"
        )));
    }
    let (value, end) = parse_json_string_at(row, cursor)?;
    validate_field_value_end(row, end, field)?;
    Ok(value)
}

/// Reads one required JSON string array with fail-closed delimiter checks.
pub(super) fn extract_string_array(
    row: &str,
    field: &str,
) -> Result<Vec<String>, PackageIntakeError> {
    let mut cursor = value_cursor(row, field)?;
    let bytes = row.as_bytes();
    if bytes.get(cursor) != Some(&b'[') {
        return Err(PackageIntakeError::new(format!(
            "field {field} is not a string array"
        )));
    }
    cursor = cursor.saturating_add(1);
    let mut values = Vec::new();
    loop {
        cursor = skip_json_ws(row, cursor);
        match bytes.get(cursor) {
            Some(b']') => {
                validate_field_value_end(row, cursor.saturating_add(1), field)?;
                return Ok(values);
            },
            Some(b'"') => {
                let (value, next_cursor) = parse_json_string_at(row, cursor)?;
                values.push(value);
                cursor = skip_json_ws(row, next_cursor);
                match bytes.get(cursor) {
                    Some(b',') => {
                        cursor = skip_json_ws(row, cursor.saturating_add(1));
                        if bytes.get(cursor) == Some(&b']') {
                            return Err(PackageIntakeError::new(format!(
                                "field {field} has a trailing array \
                                         comma"
                            )));
                        }
                    },
                    Some(b']') => {
                        validate_field_value_end(
                            row,
                            cursor.saturating_add(1),
                            field,
                        )?;
                        return Ok(values);
                    },
                    _ => {
                        return Err(PackageIntakeError::new(format!(
                            "field {field} has malformed string array"
                        )));
                    },
                }
            },
            _ => {
                return Err(PackageIntakeError::new(format!(
                    "field {field} has malformed string array"
                )));
            },
        }
    }
}

/// Number of fields in one canonical package-index row.
const CANONICAL_PACKAGE_FIELD_COUNT: usize = 30;

/// Return one field's canonical package-index position.
fn canonical_package_field_position(field: &str) -> Option<usize> {
    match field {
        "package_id" => Some(0),
        "package_root" => Some(1),
        "package_category" => Some(2),
        "package_subcategory" => Some(3),
        "unit_count" => Some(4),
        "text_key_count" => Some(5),
        "unit_ids" => Some(6),
        "source_unit_ids" => Some(26),
        "text_key_ids" => Some(27),
        "members" => Some(28),
        "text_keys" => Some(29),
        _ => PackageRole::all()
            .into_iter()
            .position(|role| role.id_field() == field)
            .map(|position| position.saturating_add(7)),
    }
}

/// Return whether one field belongs to the canonical package-index schema.
fn is_known_package_field(field: &str) -> bool {
    canonical_package_field_position(field).is_some()
}

/// Locates one unique top-level field while validating the complete object.
// One scanner owns field order, uniqueness, framing, and delimiters.
pub(super) fn value_cursor(
    row: &str,
    field: &str,
) -> Result<usize, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut cursor = skip_json_ws(row, 0);
    if bytes.get(cursor) != Some(&b'{') {
        return Err(PackageIntakeError::new(
            "package row is not a JSON object",
        ));
    }
    cursor = cursor.saturating_add(1);
    let mut seen = std::collections::BTreeSet::new();
    let mut expected_position = 0usize;
    let mut found = None;
    loop {
        cursor = skip_json_ws(row, cursor);
        if bytes.get(cursor) == Some(&b'}') {
            let end = skip_json_ws(row, cursor.saturating_add(1));
            if end != row.len() {
                return Err(PackageIntakeError::new(
                    "package row has trailing JSON content",
                ));
            }
            if expected_position != CANONICAL_PACKAGE_FIELD_COUNT {
                return Err(PackageIntakeError::new(
                    "package row has an incomplete canonical field set",
                ));
            }
            return found.ok_or_else(|| {
                PackageIntakeError::new(format!("missing field: {field}"))
            });
        }
        if bytes.get(cursor) != Some(&b'"') {
            return Err(PackageIntakeError::new(
                "package row has a malformed top-level key",
            ));
        }
        let (key, next) = parse_json_string_at(row, cursor)?;
        if !seen.insert(key.clone()) {
            return Err(PackageIntakeError::new(format!(
                "package row duplicates top-level field: {key}"
            )));
        }
        if !is_known_package_field(&key) {
            return Err(PackageIntakeError::new(format!(
                "package row contains unknown top-level field: {key}"
            )));
        }
        let position =
            canonical_package_field_position(&key).ok_or_else(|| {
                PackageIntakeError::new("package field position is unavailable")
            })?;
        if position != expected_position {
            return Err(PackageIntakeError::new(format!(
                "package row field {key} is out of canonical order"
            )));
        }
        expected_position = expected_position.saturating_add(1);
        cursor = skip_json_ws(row, next);
        if bytes.get(cursor) != Some(&b':') {
            return Err(PackageIntakeError::new(
                "package row key is missing a colon",
            ));
        }
        let value_start = skip_json_ws(row, cursor.saturating_add(1));
        if key == field {
            found = Some(value_start);
        }
        cursor = skip_top_level_value(row, value_start)?;
        cursor = skip_json_ws(row, cursor);
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(row, cursor.saturating_add(1));
                if bytes.get(cursor) == Some(&b'}') {
                    return Err(PackageIntakeError::new(
                        "package row has a trailing object comma",
                    ));
                }
            },
            Some(b'}') => {},
            _ => {
                return Err(PackageIntakeError::new(
                    "package row has a malformed top-level delimiter",
                ));
            },
        }
    }
}

/// Maximum accepted JSON container depth for generated package rows.
pub(super) const MAX_JSON_NESTING: usize = 128;

/// Skip one complete top-level JSON value with strict nested grammar.
fn skip_top_level_value(
    row: &str,
    start: usize,
) -> Result<usize, PackageIntakeError> {
    skip_json_value(row, start, 0)
}

/// Skip one complete JSON value and return the first byte after it.
fn skip_json_value(
    row: &str,
    start: usize,
    depth: usize,
) -> Result<usize, PackageIntakeError> {
    if depth > MAX_JSON_NESTING {
        return Err(PackageIntakeError::new(
            "package row exceeds the JSON nesting limit",
        ));
    }
    let cursor = skip_json_ws(row, start);
    match row.as_bytes().get(cursor) {
        Some(b'"') => parse_json_string_at(row, cursor).map(|(_, end)| end),
        Some(b'{') => skip_json_object(row, cursor, depth.saturating_add(1)),
        Some(b'[') => skip_json_array(row, cursor, depth.saturating_add(1)),
        Some(b't') => skip_json_literal(row, cursor, b"true"),
        Some(b'f') => skip_json_literal(row, cursor, b"false"),
        Some(b'n') => skip_json_literal(row, cursor, b"null"),
        Some(b'-' | b'0'..=b'9') => skip_json_number(row, cursor),
        _ => Err(PackageIntakeError::new(
            "package row contains an invalid JSON value",
        )),
    }
}

/// Skip one strict JSON object without accepting trailing commas.
fn skip_json_object(
    row: &str,
    start: usize,
    depth: usize,
) -> Result<usize, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut cursor = skip_json_ws(row, start.saturating_add(1));
    if bytes.get(cursor) == Some(&b'}') {
        return Ok(cursor.saturating_add(1));
    }
    loop {
        if bytes.get(cursor) != Some(&b'"') {
            return Err(PackageIntakeError::new(
                "package row has a malformed nested object key",
            ));
        }
        let (_, next) = parse_json_string_at(row, cursor)?;
        cursor = skip_json_ws(row, next);
        if bytes.get(cursor) != Some(&b':') {
            return Err(PackageIntakeError::new(
                "package row nested object key is missing a colon",
            ));
        }
        cursor = skip_json_value(
            row,
            skip_json_ws(row, cursor.saturating_add(1)),
            depth,
        )?;
        cursor = skip_json_ws(row, cursor);
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(row, cursor.saturating_add(1));
                if bytes.get(cursor) == Some(&b'}') {
                    return Err(PackageIntakeError::new(
                        "package row has a trailing nested object comma",
                    ));
                }
            },
            Some(b'}') => return Ok(cursor.saturating_add(1)),
            _ => {
                return Err(PackageIntakeError::new(
                    "package row has a malformed nested object delimiter",
                ));
            },
        }
    }
}

/// Skip one strict JSON array without accepting trailing commas.
fn skip_json_array(
    row: &str,
    start: usize,
    depth: usize,
) -> Result<usize, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut cursor = skip_json_ws(row, start.saturating_add(1));
    if bytes.get(cursor) == Some(&b']') {
        return Ok(cursor.saturating_add(1));
    }
    loop {
        cursor = skip_json_value(row, cursor, depth)?;
        cursor = skip_json_ws(row, cursor);
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_ws(row, cursor.saturating_add(1));
                if bytes.get(cursor) == Some(&b']') {
                    return Err(PackageIntakeError::new(
                        "package row has a trailing nested array comma",
                    ));
                }
            },
            Some(b']') => return Ok(cursor.saturating_add(1)),
            _ => {
                return Err(PackageIntakeError::new(
                    "package row has a malformed nested array delimiter",
                ));
            },
        }
    }
}

/// Skip one exact JSON literal.
fn skip_json_literal(
    row: &str,
    start: usize,
    literal: &[u8],
) -> Result<usize, PackageIntakeError> {
    let end = start.saturating_add(literal.len());
    if row.as_bytes().get(start..end) != Some(literal) {
        return Err(PackageIntakeError::new(
            "package row contains a malformed JSON literal",
        ));
    }
    Ok(end)
}

/// Skip one JSON number with canonical integer, fraction, and exponent grammar.
fn skip_json_number(
    row: &str,
    start: usize,
) -> Result<usize, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut cursor = start;
    if bytes.get(cursor) == Some(&b'-') {
        cursor = cursor.saturating_add(1);
    }
    match bytes.get(cursor) {
        Some(b'0') => {
            cursor = cursor.saturating_add(1);
            if bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
                return Err(PackageIntakeError::new(
                    "package row JSON number has a leading zero",
                ));
            }
        },
        Some(b'1'..=b'9') => {
            cursor = cursor.saturating_add(1);
            while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
                cursor = cursor.saturating_add(1);
            }
        },
        _ => {
            return Err(PackageIntakeError::new(
                "package row contains a malformed JSON number",
            ));
        },
    }
    if bytes.get(cursor) == Some(&b'.') {
        cursor = cursor.saturating_add(1);
        let fraction_start = cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor = cursor.saturating_add(1);
        }
        if cursor == fraction_start {
            return Err(PackageIntakeError::new(
                "package row JSON fraction has no digits",
            ));
        }
    }
    if matches!(bytes.get(cursor), Some(b'e' | b'E')) {
        cursor = cursor.saturating_add(1);
        if matches!(bytes.get(cursor), Some(b'+' | b'-')) {
            cursor = cursor.saturating_add(1);
        }
        let exponent_start = cursor;
        while bytes.get(cursor).is_some_and(u8::is_ascii_digit) {
            cursor = cursor.saturating_add(1);
        }
        if cursor == exponent_start {
            return Err(PackageIntakeError::new(
                "package row JSON exponent has no digits",
            ));
        }
    }
    Ok(cursor)
}

/// Advances across JSON whitespace so structural parsers share one rule.
pub(super) fn skip_json_ws(row: &str, mut cursor: usize) -> usize {
    while matches!(
        row.as_bytes().get(cursor),
        Some(b' ' | b'\n' | b'\r' | b'\t')
    ) {
        cursor = cursor.saturating_add(1);
    }
    cursor
}

/// Decode one JSON Unicode escape, including a required surrogate pair.
fn parse_json_unicode_escape(
    row: &str,
    escape_cursor: usize,
) -> Result<(char, usize), PackageIntakeError> {
    let first_start = escape_cursor.saturating_add(1);
    let first = parse_json_hex_quad(row, first_start)?;
    let first_end = first_start.saturating_add(4);
    if (0xd800..=0xdbff).contains(&first) {
        let bytes = row.as_bytes();
        if bytes.get(first_end) != Some(&b'\\')
            || bytes.get(first_end.saturating_add(1)) != Some(&b'u')
        {
            return Err(PackageIntakeError::new(
                "high JSON surrogate is missing a low surrogate",
            ));
        }
        let second_start = first_end.saturating_add(2);
        let second = parse_json_hex_quad(row, second_start)?;
        if !(0xdc00..=0xdfff).contains(&second) {
            return Err(PackageIntakeError::new(
                "high JSON surrogate is followed by an invalid low \
                     surrogate",
            ));
        }
        let high = u32::from(first)
            .checked_sub(0xd800_u32)
            .ok_or_else(|| PackageIntakeError::new("invalid high surrogate"))?;
        let low = u32::from(second)
            .checked_sub(0xdc00_u32)
            .ok_or_else(|| PackageIntakeError::new("invalid low surrogate"))?;
        let shifted_high = high.checked_shl(10).ok_or_else(|| {
            PackageIntakeError::new("surrogate shift overflow")
        })?;
        let code_point = 0x1_0000_u32
            .checked_add(shifted_high)
            .and_then(|value| value.checked_add(low))
            .ok_or_else(|| PackageIntakeError::new("surrogate sum overflow"))?;
        let character = char::from_u32(code_point).ok_or_else(|| {
            PackageIntakeError::new("invalid JSON surrogate code point")
        })?;
        return Ok((character, second_start.saturating_add(4)));
    }
    if (0xdc00..=0xdfff).contains(&first) {
        return Err(PackageIntakeError::new(
            "low JSON surrogate has no leading high surrogate",
        ));
    }
    let character = char::from_u32(u32::from(first)).ok_or_else(|| {
        PackageIntakeError::new("invalid JSON Unicode code point")
    })?;
    Ok((character, first_end))
}

/// Parse exactly four hexadecimal digits from one JSON Unicode escape.
fn parse_json_hex_quad(
    row: &str,
    start: usize,
) -> Result<u16, PackageIntakeError> {
    let bytes = row.as_bytes();
    let mut value = 0u16;
    for offset in 0..4usize {
        let byte = bytes
            .get(start.saturating_add(offset))
            .copied()
            .ok_or_else(|| {
                PackageIntakeError::new("incomplete JSON Unicode escape")
            })?;
        let nibble = match byte {
            b'0'..=b'9' => byte.checked_sub(b'0').map(u16::from),
            b'a'..=b'f' => byte
                .checked_sub(b'a')
                .map(u16::from)
                .and_then(|nibble_base| nibble_base.checked_add(10_u16)),
            b'A'..=b'F' => byte
                .checked_sub(b'A')
                .map(u16::from)
                .and_then(|nibble_base| nibble_base.checked_add(10_u16)),
            _ => None,
        }
        .ok_or_else(|| {
            PackageIntakeError::new("invalid hex digit in JSON Unicode escape")
        })?;
        value = value
            .checked_shl(4)
            .map(|shifted| shifted | nibble)
            .ok_or_else(|| {
                PackageIntakeError::new("Unicode escape overflow")
            })?;
    }
    Ok(value)
}

/// Decodes one JSON string while rejecting malformed or incomplete escapes.
pub(super) fn parse_json_string_at(
    row: &str,
    start: usize,
) -> Result<(String, usize), PackageIntakeError> {
    let bytes = row.as_bytes();
    if bytes.get(start) != Some(&b'"') {
        return Err(PackageIntakeError::new("expected JSON string"));
    }
    let mut cursor = start.saturating_add(1);
    let mut output = String::new();
    while let Some(byte) = bytes.get(cursor).copied() {
        match byte {
            b'"' => {
                return Ok((output, cursor.saturating_add(1)));
            },
            b'\\' => {
                cursor = cursor.saturating_add(1);
                let Some(escaped) = bytes.get(cursor).copied() else {
                    return Err(PackageIntakeError::new(
                        "unterminated JSON escape",
                    ));
                };
                match escaped {
                    b'"' => output.push('"'),
                    b'\\' => output.push('\\'),
                    b'/' => output.push('/'),
                    b'b' => output.push('\u{0008}'),
                    b'f' => output.push('\u{000C}'),
                    b'n' => output.push('\n'),
                    b'r' => output.push('\r'),
                    b't' => output.push('\t'),
                    b'u' => {
                        let (character, next_cursor) =
                            parse_json_unicode_escape(row, cursor)?;
                        output.push(character);
                        cursor = next_cursor;
                        continue;
                    },
                    _ => {
                        return Err(PackageIntakeError::new(
                            "unsupported JSON escape in package index",
                        ));
                    },
                }
            },
            control if control <= 0x1f => {
                return Err(PackageIntakeError::new(
                    "unescaped control character in JSON string",
                ));
            },
            _ if byte.is_ascii() => output.push(char::from(byte)),
            _ => {
                let tail = row.get(cursor..).ok_or_else(|| {
                    PackageIntakeError::new("invalid UTF-8 string cursor")
                })?;
                let character = tail.chars().next().ok_or_else(|| {
                    PackageIntakeError::new("invalid UTF-8 package string")
                })?;
                output.push(character);
                cursor = cursor.saturating_add(character.len_utf8());
                continue;
            },
        }
        cursor = cursor.saturating_add(1);
    }
    Err(PackageIntakeError::new("unterminated JSON string"))
}
