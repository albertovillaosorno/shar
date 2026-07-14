// File:
//   - sound_type.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/stragglers/sound_type.rs
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
//   - The sound type contract for pipeline phase two stragglers.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute sound type.
// - Split-When:
//   - Split when sound type contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Sound type for pipeline phase two stragglers.
// - Description:
//   - Defines sound type data and behavior for pipeline phase two
//   - stragglers.
// - Usage:
//   - Used by pipeline phase two stragglers code that needs sound type.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Sound type for pipeline phase two stragglers.
//!
//! This boundary keeps sound type for pipeline phase two stragglers explicit
//! and returns deterministic results to pipeline callers.
use std::collections::BTreeSet;

use super::json::{JsonObject, json_string, printable_tokens};

/// Appends deterministic sound-symbol evidence to one JSON summary.
pub(super) fn append_summary(
    json: &mut JsonObject,
    bytes: &[u8],
) {
    let tokens = printable_tokens(bytes);
    let records = symbol_records(&tokens);
    let interfaces = filtered(
        &tokens,
        |token| {
            token.starts_with('I')
                && (token.contains("Sound") || token.contains("CarSound"))
        },
    );
    let sound_methods = filtered(
        &tokens,
        |token| is_sound_method(token),
    );
    let parameter_names = filtered(
        &tokens,
        |token| is_parameter_name(token),
    );
    let primitive_types = filtered(
        &tokens,
        |token| {
            matches!(
                token.as_str(),
                "void" | "bool" | "char" | "float" | "int"
            )
        },
    );

    json.number(
        "printable_token_count",
        u64::try_from(tokens.len()).unwrap_or(u64::MAX),
    );
    json.number(
        "interface_count",
        u64::try_from(interfaces.len()).unwrap_or(u64::MAX),
    );
    json.number(
        "method_count",
        u64::try_from(sound_methods.len()).unwrap_or(u64::MAX),
    );
    json.number(
        "parameter_name_count",
        u64::try_from(parameter_names.len()).unwrap_or(u64::MAX),
    );
    json.bool(
        "contains_sound_resource_interfaces",
        !interfaces.is_empty(),
    );
    json.bool(
        "contains_car_sound_parameters",
        tokens
            .iter()
            .any(|token| token.contains("CarSound")),
    );
    json.bool(
        "contains_looping_control",
        sound_methods
            .iter()
            .any(|token| token.contains("Looping")),
    );
    json.bool(
        "contains_pitch_range_control",
        sound_methods
            .iter()
            .any(|token| token.contains("PitchRange")),
    );
    json.string_array(
        "interfaces",
        &interfaces,
    );
    json.string_array(
        "methods",
        &sound_methods,
    );
    json.string_array(
        "parameter_names",
        &parameter_names,
    );
    json.string_array(
        "primitive_types",
        &primitive_types,
    );
    json.string_array(
        "printable_tokens",
        &tokens,
    );
    json.raw_json(
        "symbol_records",
        &symbol_records_json(&records),
    );
}

/// Symbolrecord.
struct SymbolRecord {
    /// Ordinal.
    ordinal: usize,
    /// Token.
    token: String,
    /// Role.
    role: String,
}

/// Symbol records.
fn symbol_records(tokens: &[String]) -> Vec<SymbolRecord> {
    tokens
        .iter()
        .enumerate()
        .map(
            |(index, token)| SymbolRecord {
                ordinal: index.saturating_add(1),
                token: token.clone(),
                role: symbol_role(token).to_owned(),
            },
        )
        .collect()
}

/// Return whether one symbol is a callable sound method.
fn is_sound_method(token: &str) -> bool {
    matches!(
        token,
        "AddFilename"
            | "SetPitchRange"
            | "SetTrimRange"
            | "SetTrim"
            | "SetStreaming"
            | "SetLooping"
            | "Play"
            | "SetClip"
            | "SetVolume"
            | "SetPan"
            | "SetBalance"
    ) || token.starts_with("Set")
}

/// Return whether one symbol is a sound parameter rather than a method.
fn is_parameter_name(token: &str) -> bool {
    !is_sound_method(token)
        && (token.ends_with("Pitch")
            || token.ends_with("Trim")
            || token.ends_with("Volume")
            || token.ends_with("Clip")
            || matches!(
                token,
                "newFileName" | "streaming" | "looping"
            ))
}

/// Symbol role.
fn symbol_role(token: &str) -> &'static str {
    if token.starts_with('I') && token.contains("Sound") {
        "interface"
    } else if is_sound_method(token) {
        "method"
    } else if matches!(
        token,
        "void" | "bool" | "char" | "float" | "int"
    ) {
        "primitive-type"
    } else if is_parameter_name(token) {
        "parameter"
    } else {
        "symbol"
    }
}

/// Symbol records json.
fn symbol_records_json(records: &[SymbolRecord]) -> String {
    let mut out = String::from("[");
    for (index, record) in records
        .iter()
        .enumerate()
    {
        if index > 0 {
            out.push(',');
        }
        out.push('{');
        out.push_str("\"ordinal\":");
        out.push_str(
            &record
                .ordinal
                .to_string(),
        );
        out.push_str(",\"token\":");
        out.push_str(&json_string(&record.token));
        out.push_str(",\"role\":");
        out.push_str(&json_string(&record.role));
        out.push('}');
    }
    out.push(']');
    out
}

/// Filtered.
fn filtered(
    tokens: &[String],
    predicate: impl Fn(&String) -> bool,
) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut values = Vec::new();
    for token in tokens {
        if predicate(token) && seen.insert(token.clone()) {
            values.push(token.clone());
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::super::json::JsonObject;
    use super::{append_summary, is_parameter_name, is_sound_method};

    #[test]
    fn method_tokens_are_not_parameter_names() -> Result<(), String> {
        if !is_sound_method("SetVolume") || is_parameter_name("SetVolume") {
            return Err("SetVolume classification is inconsistent".to_owned());
        }
        let mut json = JsonObject::new();
        append_summary(
            &mut json,
            b"SetVolume\0carVolume",
        );
        let output = json.finish();
        if output.contains("\"parameter_names\":[\"carVolume\"]")
            && !output.contains("\"parameter_names\":[\"SetVolume")
        {
            Ok(())
        } else {
            Err(format!("method/parameter output overlapped: {output}"))
        }
    }
}
