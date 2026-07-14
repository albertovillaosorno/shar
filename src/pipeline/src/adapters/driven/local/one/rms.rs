// File:
//   - rms.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/one/rms.rs
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
//   - The rms contract for pipeline phase one.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute rms.
// - Split-When:
//   - Split when rms contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Rms for pipeline phase one.
// - Description:
//   - Defines rms data and behavior for pipeline phase one.
// - Usage:
//   - Used by pipeline phase one code that needs rms.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Rms for pipeline phase one keeps tightly coupled validation,
//   - ordering, and deterministic transformation invariants together; split
//   - when a stable independently testable sub-boundary is identified.
//

//! Rms for pipeline phase one.
//!
//! This boundary keeps rms for pipeline phase one explicit and returns
//! deterministic results to pipeline callers.
use std::collections::BTreeSet;
use std::io;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::domain::escape_json as json_escape;

#[derive(Debug, Clone)]
/// Symbol.
struct Symbol {
    /// Offset.
    offset: usize,
    /// Value.
    value: String,
}

/// To json.
pub(super) fn to_json(input: &Path) -> io::Result<String> {
    let bytes = local::read_bytes(input)?;
    Ok(
        bytes_to_json(
            &bytes,
            &input
                .display()
                .to_string(),
        ),
    )
}

/// Bytes to json.
fn bytes_to_json(
    bytes: &[u8],
    source: &str,
) -> String {
    let words = le_u32_words(bytes);
    let symbols = ascii_strings(bytes);
    let byte_hex = hex_bytes(bytes);
    let offset_refs = offset_references_json(
        &words, bytes, &symbols,
    );
    let symbol_json = symbols_json(&symbols);
    let schema_terms = filtered_symbols_json(
        &symbols,
        SymbolClass::SchemaTerm,
    );
    let named_assets = filtered_symbols_json(
        &symbols,
        SymbolClass::NamedAsset,
    );
    let regions = filtered_symbols_json(
        &symbols,
        SymbolClass::Region,
    );
    let placeholders = filtered_symbols_json(
        &symbols,
        SymbolClass::Placeholder,
    );
    let rsd_refs = filtered_symbols_json(
        &symbols,
        SymbolClass::RsdReference,
    );
    format!(
        concat!(
            r##"{{"schema":"shar-schoenwald.radmusic-compiled.v3","##,
            r##""source":"{}","##,
            r##""format_name":"{}","##,
            r##""runtime_loader":{{"##,
            r##""library":"radmusic","##,
            r##""container":"ods_block","##,
            r##""composition_chunk_id_hex":"0x43634211","##,
            r##""entry_instance":"my_comp","##,
            r##""loader_function":"##,
            r##""block_construct_from_stream_synch"}},"##,
            r##""schema_classes":[{}],"##,
            r##""byte_len":{},"##,
            r##""entry_count_hint":{},"##,
            r##""u32_word_count":{},"##,
            r##""symbol_count":{},"##,
            r##""radmusic_schema_terms":[{}],"##,
            r##""named_assets":[{}],"##,
            r##""regions":[{}],"##,
            r##""placeholders":[{}],"##,
            r##""audio_references":[{}],"##,
            r##""symbols":[{}],"##,
            r##""offset_references":[{}],"##,
            r##""bytes_hex":"{}"}}
"##,
        ),
        json_escape(source),
        json_escape(
            symbols
                .first()
                .map_or(
                    "",
                    |symbol| symbol
                        .value
                        .as_str()
                )
        ),
        schema_classes_json(),
        bytes.len(),
        words
            .first()
            .copied()
            .unwrap_or_default(),
        words.len(),
        symbols.len(),
        schema_terms,
        named_assets,
        regions,
        placeholders,
        rsd_refs,
        symbol_json,
        offset_refs,
        byte_hex
    )
}

#[derive(Debug, Clone, Copy)]
/// Symbolclass.
enum SymbolClass {
    /// Item.
    SchemaTerm,
    /// Item.
    NamedAsset,
    /// Item.
    Region,
    /// Item.
    Placeholder,
    /// Item.
    RsdReference,
}

/// Schema classes json.
const fn schema_classes_json() -> &'static str {
    concat!(
        r#""comp","group","fade_transition","#,
        r#""stitch_transition","event","event_matrix","#,
        r#""state","rsd_file","stream","clip","#,
        r#""region","sequence","sequence_event","#,
        r#""action","layer""#,
    )
}

/// Filtered symbols json.
fn filtered_symbols_json(
    symbols: &[Symbol],
    class: SymbolClass,
) -> String {
    symbols
        .iter()
        .filter(
            |symbol| {
                symbol_matches(
                    symbol, class,
                )
            },
        )
        .map(symbol_json)
        .collect::<Vec<_>>()
        .join(",")
}

/// Symbol matches.
fn symbol_matches(
    symbol: &Symbol,
    class: SymbolClass,
) -> bool {
    let value = symbol
        .value
        .as_str();
    match class {
        SymbolClass::SchemaTerm => is_radmusic_schema_term(value),
        SymbolClass::NamedAsset => {
            !is_radmusic_schema_term(value)
                && !is_placeholder(value)
                && !value
                    .to_ascii_lowercase()
                    .ends_with(".rsd")
        }
        SymbolClass::Region => value
            .to_ascii_lowercase()
            .ends_with("_region"),
        SymbolClass::Placeholder => is_placeholder(value),
        SymbolClass::RsdReference => value
            .to_ascii_lowercase()
            .ends_with(".rsd"),
    }
}

/// Is placeholder.
fn is_placeholder(value: &str) -> bool {
    value.starts_with("Placeholder")
        || {
            // cspell:disable-next-line -- Placeholde
            value.starts_with("Placeholde")
        }
        || value.starts_with("DoNotUse")
}

/// Is radmusic schema term.
fn is_radmusic_schema_term(value: &str) -> bool {
    if value == "radmusic_comp" || value == "comp" {
        return true;
    }
    value
        .chars()
        .all(
            |character| {
                character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || character == '_'
            },
        )
}

/// Symbols json.
fn symbols_json(symbols: &[Symbol]) -> String {
    symbols
        .iter()
        .map(symbol_json)
        .collect::<Vec<_>>()
        .join(",")
}

/// Symbol json.
fn symbol_json(symbol: &Symbol) -> String {
    format!(
        r#"{{"offset":{},"value":"{}"}}"#,
        symbol.offset,
        json_escape(&symbol.value)
    )
}

/// Offset references json.
fn offset_references_json(
    words: &[u32],
    bytes: &[u8],
    symbols: &[Symbol],
) -> String {
    let symbol_offsets = symbols
        .iter()
        .map(|symbol| symbol.offset)
        .collect::<BTreeSet<_>>();
    words
        .iter()
        .enumerate()
        .filter_map(
            |(index, value)| {
                let target = usize::try_from(*value).ok()?;
                if target >= bytes.len() {
                    return None;
                }
                let target_kind = if symbol_offsets.contains(&target) {
                    "symbol"
                } else if target % 4 == 0 {
                    "aligned_data"
                } else {
                    "data"
                };
                let quote = char::from(34);
                let fields = [
                    format!("{quote}word_index{quote}:{index}"),
                    format!(
                        "{quote}source_offset{quote}:{}",
                        index.saturating_mul(4)
                    ),
                    format!("{quote}target_offset{quote}:{target}"),
                    format!(
                        "{quote}target_kind{quote}:{quote}{target_kind}{quote}"
                    ),
                ];
                Some(
                    format!(
                        "{{{}}}",
                        fields.join(",")
                    ),
                )
            },
        )
        .collect::<Vec<_>>()
        .join(",")
}

/// Little-endian u32 words.
fn le_u32_words(bytes: &[u8]) -> Vec<u32> {
    let mut words = Vec::new();
    let mut cursor = 0usize;
    while cursor.saturating_add(4) <= bytes.len() {
        let Some(chunk) = bytes.get(cursor..cursor.saturating_add(4)) else {
            break;
        };
        let word_bytes = chunk
            .try_into()
            .unwrap_or([0_u8; 4]);
        words.push(u32::from_le_bytes(word_bytes));
        cursor = cursor.saturating_add(4);
    }
    words
}

/// Ascii strings.
fn ascii_strings(bytes: &[u8]) -> Vec<Symbol> {
    let mut strings = Vec::new();
    let mut cursor = 0usize;
    while cursor < bytes.len() {
        if bytes
            .get(cursor)
            .is_some_and(|byte| byte.is_ascii_graphic() || *byte == b' ')
        {
            let start = cursor;
            while cursor < bytes.len()
                && bytes
                    .get(cursor)
                    .is_some_and(
                        |byte| byte.is_ascii_graphic() || *byte == b' ',
                    )
            {
                cursor = cursor.saturating_add(1);
            }
            if cursor.saturating_sub(start) >= 4 {
                let value = String::from_utf8_lossy(
                    bytes
                        .get(start..cursor)
                        .unwrap_or_default(),
                )
                .into_owned();
                strings.push(
                    Symbol {
                        offset: start,
                        value,
                    },
                );
            }
        }
        cursor = cursor.saturating_add(1);
    }
    strings
}

/// Hex bytes.
fn hex_bytes(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(
        bytes
            .len()
            .saturating_mul(2),
    );
    for byte in bytes {
        output.push(
            char::from(
                *HEX.get(usize::from(byte >> 4_u8))
                    .unwrap_or(&b'0'),
            ),
        );
        output.push(
            char::from(
                *HEX.get(usize::from(byte & 15))
                    .unwrap_or(&b'0'),
            ),
        );
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{bytes_to_json, offset_references_json};

    #[test]
    fn offset_reference_rows_are_valid_json_objects() {
        let words = [4_u32];
        let bytes = [0_u8; 8];
        let actual = offset_references_json(
            &words,
            &bytes,
            &[],
        );
        let quote = char::from(34);
        assert!(actual.starts_with(&format!("{{{quote}word_index{quote}:0,")));
        assert!(actual.contains(&format!("{quote}source_offset{quote}:0")));
        assert!(actual.contains(&format!("{quote}target_offset{quote}:4")));
        let mut suffix = String::new();
        suffix.push(quote);
        suffix.push_str("target_kind");
        suffix.push(quote);
        suffix.push(':');
        suffix.push(quote);
        suffix.push_str("aligned_data");
        suffix.push(quote);
        suffix.push('}');
        assert!(actual.ends_with(&suffix));
        assert!(!actual.contains("r##"));
    }

    #[test]
    fn exposes_radmusic_symbols_and_offsets() {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&16_u32.to_le_bytes());
        bytes.extend_from_slice(&0_u32.to_le_bytes());
        bytes.extend_from_slice(&16_u32.to_le_bytes());
        bytes.extend_from_slice(&24_u32.to_le_bytes());
        bytes.extend_from_slice(
            b"radmusic_comp\0comp\0theme_region\0Placeholder1\0",
        );
        let json = bytes_to_json(
            &bytes,
            "sample.rms",
        );
        assert!(json.contains("\"format_name\":\"radmusic_comp\""));
        assert!(json.contains("theme_region"));
        assert!(json.contains("Placeholder1"));
        assert!(json.contains("\"target_kind\":\"symbol\""));
    }
}
