// File:
//   - custom_text.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/localization/custom_text.rs
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
//   - Custom-text file decoding and record grammar.
// - Must-Not:
//   - Silently discard malformed rows, duplicate keys, or encoding failures.
// - Allows:
//   - Section and comment skipping with strict key-value record parsing.
// - Split-When:
//   - Custom text gains another record family with independent grammar.
// - Merge-When:
//   - Another source adapter owns the same custom-text parsing invariant.
// - Summary:
//   - Fail-closed custom-text normalization.
// - Description:
//   - Produces deterministic records before text-key package classification.
// - Usage:
//   - Called by phase-two package derivation and localization planning.
// - Defaults:
//   - Malformed source data fails closed without implicit output.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Custom-text records fail closed before stable ids are derived.

use std::collections::BTreeMap;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use super::encoding::decode_text_source;
use super::{CustomTextEntry, Error, Outcome};

/// Parse a custom text overlay from UTF-8 or supported UTF-16 input.
///
/// # Errors
///
/// Returns an error for IO, invalid encoding, malformed records, empty keys,
/// or duplicate exact keys.
pub(super) fn parse_custom_text(path: &Path) -> Outcome<Vec<CustomTextEntry>> {
    let bytes = local::read_bytes(path).map_err(
        |source| {
            Error::io(
                path.to_path_buf(),
                source,
            )
        },
    )?;
    parse_custom_text_bytes(
        &bytes,
        &path
            .display()
            .to_string(),
    )
}

/// Parse loaded bytes so package builders can avoid duplicate filesystem IO.
pub(super) fn parse_custom_text_bytes(
    bytes: &[u8],
    source_label: &str,
) -> Outcome<Vec<CustomTextEntry>> {
    let text = decode_text_source(
        bytes,
        source_label,
    )?;
    let mut entries = Vec::new();
    let mut key_lines = BTreeMap::new();
    for (index, line) in text
        .lines()
        .enumerate()
    {
        let raw_trimmed = line.trim();
        let trimmed = if index == 0 {
            raw_trimmed.trim_start_matches('\u{feff}')
        } else {
            raw_trimmed
        };
        if trimmed.is_empty()
            || trimmed.starts_with(';')
            || (trimmed.starts_with('[') && trimmed.ends_with(']'))
        {
            continue;
        }
        let line_number = index
            .checked_add(1)
            .ok_or_else(
                || Error::invalid("custom-text source line overflowed"),
            )?;
        let (raw_key, value) = trimmed
            .split_once('=')
            .ok_or_else(
                || {
                    Error::invalid(
                        format!(
                            "custom-text line {line_number} is missing '='"
                        ),
                    )
                },
            )?;
        let normalized_key = raw_key.trim();
        if normalized_key.is_empty() {
            return Err(
                Error::invalid(
                    format!("custom-text line {line_number} has an empty key"),
                ),
            );
        }
        let key = normalized_key.to_owned();
        if let Some(first_line) = key_lines.insert(
            key.clone(),
            line_number,
        ) {
            return Err(
                Error::invalid(
                    format!(
                        "custom-text key '{key}' is duplicated on lines \
                         {first_line} and {line_number}"
                    ),
                ),
            );
        }
        entries.push(
            CustomTextEntry {
                key,
                value: value
                    .trim()
                    .to_owned(),
                line: line_number,
            },
        );
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::parse_custom_text_bytes;

    #[test]
    fn strips_utf8_bom_from_first_line() -> Result<(), String> {
        let entries = parse_custom_text_bytes(
            "\u{feff}FIRST=one".as_bytes(),
            "test",
        )
        .map_err(|error| error.to_string())?;
        if entries
            .first()
            .is_some_and(|entry| entry.key == "FIRST")
        {
            Ok(())
        } else {
            Err(format!("unexpected BOM-prefixed entries: {entries:?}"))
        }
    }

    #[test]
    fn preserves_bom_after_first_line() -> Result<(), String> {
        let entries = parse_custom_text_bytes(
            "FIRST=one\n\u{feff}SECOND=two".as_bytes(),
            "test",
        )
        .map_err(|error| error.to_string())?;
        let second = entries
            .get(1)
            .ok_or_else(|| "missing second custom-text entry".to_owned())?;
        if second.key == "\u{feff}SECOND" {
            Ok(())
        } else {
            Err(
                format!(
                    "second-line BOM was rewritten as {:?}",
                    second.key,
                ),
            )
        }
    }

    #[test]
    fn rejects_malformed_record() -> Result<(), String> {
        if parse_custom_text_bytes(
            b"BROKEN_LINE",
            "test",
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("malformed custom-text record was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_empty_key() -> Result<(), String> {
        if parse_custom_text_bytes(
            b" =value", "test",
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("empty custom-text key was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_duplicate_key() -> Result<(), String> {
        if parse_custom_text_bytes(
            b"KEY=first\nKEY=second",
            "test",
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("duplicate custom-text key was accepted".to_owned())
        }
    }

    #[test]
    fn parses_utf16_custom_text() -> Result<(), String> {
        let bytes = [
            0xff, 0xfe, b'A', 0, b'=', 0, b'B', 0,
        ];
        let entries = parse_custom_text_bytes(
            &bytes, "test",
        )
        .map_err(|error| error.to_string())?;
        if entries.len() == 1
            && entries
                .first()
                .is_some_and(|entry| entry.key == "A")
        {
            Ok(())
        } else {
            Err(format!("unexpected custom-text entries: {entries:?}"))
        }
    }
}
