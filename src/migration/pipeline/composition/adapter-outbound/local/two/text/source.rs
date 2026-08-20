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
//   - Source outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Source outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source outbound adapter.

use std::collections::BTreeSet;
use std::path::Path;

use serde::Deserialize;
use schoenwald_filesystem::adapters::driving::local;

use super::super::localization::read_custom_text_keys as read_validated_keys;
use super::PipelineOutcome;
use crate::domain::PipelineError;

/// Read validated custom-text keys without duplicating source grammar.
pub(super) fn read_custom_text_keys(
    path: &Path,
) -> PipelineOutcome<Vec<String>> {
    read_validated_keys(path)
}

const TEXT_BIBLE_SCHEMA: &str =
    "shar-schoenwald.straggler.text-bible.v1";

#[derive(Debug, Deserialize)]
struct SourceTextDocument {
    schema: String,
    source_extension: String,
    language_channel: String,
    entry_count: usize,
    source_entries: Vec<String>,
}

/// Read exact source-table keys from normalized `TextBible` evidence.
pub(super) fn read_source_text_keys(
    path: &Path,
) -> PipelineOutcome<Vec<String>> {
    let text = local::read_utf8(path).map_err(|_error| {
        PipelineError::new("normalized source-text table could not be read")
    })?;
    parse_source_text_keys(&text)
}

pub(super) fn parse_source_text_keys(
    text: &str,
) -> PipelineOutcome<Vec<String>> {
    let document: SourceTextDocument =
        serde_json::from_str(text).map_err(|error| {
            PipelineError::new(format!(
                "normalized source-text table is invalid JSON: {error}"
            ))
        })?;
    if document.schema != TEXT_BIBLE_SCHEMA {
        return Err(PipelineError::new(
            "normalized source-text table schema is stale",
        ));
    }
    if document.entry_count != document.source_entries.len() {
        return Err(PipelineError::new(
            "normalized source-text table entry count drifted",
        ));
    }
    if document.language_channel != "source-text" {
        return Ok(Vec::new());
    }
    if document.source_extension != "txt" {
        return Err(PipelineError::new(
            "normalized source-text table extension drifted",
        ));
    }
    if document
        .source_entries
        .first()
        .is_none_or(|row| !row.as_bytes().contains(&9))
    {
        return Ok(Vec::new());
    }
    let [languages, screen, term, data @ ..] =
        document.source_entries.as_slice()
    else {
        return Err(PipelineError::new(
            "normalized source-text table header is incomplete",
        ));
    };
    validate_source_text_header(languages, screen, term)?;
    let mut keys = Vec::with_capacity(data.len());
    let mut seen = BTreeSet::new();
    for row in data {
        let (namespace, key) = source_text_identity(row)?;
        if namespace != namespace.to_ascii_uppercase() {
            return Err(PipelineError::new(
                "normalized source-text namespace is not canonical",
            ));
        }
        if !seen.insert(key.to_owned()) {
            return Err(PipelineError::new(
                "normalized source-text key is duplicated",
            ));
        }
        keys.push(key.to_owned());
    }
    Ok(keys)
}

fn validate_source_text_header(
    languages: &str,
    screen: &str,
    term: &str,
) -> PipelineOutcome<()> {
    for (row, expected) in [
        (languages, "Languages"),
        (screen, "Screen"),
        (term, "TERM"),
    ] {
        let (label, value) = source_text_identity(row)?;
        if label != expected || value.is_empty() {
            return Err(PipelineError::new(
                "normalized source-text table header drifted",
            ));
        }
    }
    Ok(())
}

fn source_text_identity(row: &str) -> PipelineOutcome<(&str, &str)> {
    let mut fields = row.split('\t');
    let namespace = fields.next().unwrap_or_default();
    let key = fields.next().unwrap_or_default();
    if namespace.is_empty()
        || key.is_empty()
        || namespace != namespace.trim()
        || key != key.trim()
        || namespace.chars().any(char::is_control)
        || key.chars().any(char::is_control)
    {
        return Err(PipelineError::new(
            "normalized source-text key identity is malformed",
        ));
    }
    Ok((namespace, key))
}
