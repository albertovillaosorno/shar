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
//   - Provenance domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Provenance domain module.
// - Description:
//   - Implements the declared domain module responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Provenance domain module.

use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Provenanceevidence.
pub struct ProvenanceEvidence {
    /// Embedded source names.
    pub embedded_source_names: Vec<String>,
}

/// Returns a one-to-one uppercase mapping for source-name identity.
fn source_case_character(character: char) -> char {
    let mut uppercase = character.to_uppercase();
    let first_uppercase = uppercase.next().unwrap_or(character);
    if uppercase.next().is_some() {
        character
    } else {
        first_uppercase
    }
}

/// Produces a case-insensitive identity without expanding one source-name
/// character into multiple identity characters.
fn source_name_identity(value: &str) -> Vec<u32> {
    value
        .chars()
        .map(source_case_character)
        .map(u32::from)
        .collect()
}

impl ProvenanceEvidence {
    #[must_use]
    /// From bytes.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut seen = BTreeSet::new();
        let mut strings = Vec::new();
        for value in utf8_strings(bytes)
            .into_iter()
            .chain(utf16le_strings(bytes))
        {
            if !looks_like_source_name(&value) {
                continue;
            }
            let identity = source_name_identity(&value);
            if seen.insert(identity) {
                strings.push(value);
            }
        }
        Self {
            embedded_source_names: strings,
        }
    }

    #[must_use]
    /// Summary.
    pub fn summary(&self) -> String {
        if self.embedded_source_names.is_empty() {
            "pre-bink-master-not-embedded".to_owned()
        } else {
            self.embedded_source_names.join(",")
        }
    }
}

/// Extracts bounded printable UTF-8 strings without crossing invalid bytes.
fn utf8_strings(bytes: &[u8]) -> Vec<String> {
    let mut out = Vec::new();
    let mut current = String::new();
    let bounded = bytes;
    let mut cursor = 0_usize;
    while cursor < bounded.len() {
        let remaining = bounded.get(cursor..).unwrap_or_default();
        match std::str::from_utf8(remaining) {
            Ok(valid) => {
                append_printable(valid, &mut out, &mut current);
                cursor = bounded.len();
            },
            Err(error) => {
                let valid_len = error.valid_up_to();
                if valid_len > 0 {
                    let valid_bytes =
                        remaining.get(..valid_len).unwrap_or_default();
                    let valid =
                        std::str::from_utf8(valid_bytes).unwrap_or_default();
                    append_printable(valid, &mut out, &mut current);
                    cursor = cursor.saturating_add(valid_len);
                } else {
                    push_string(&mut out, &mut current);
                    cursor =
                        cursor.saturating_add(error.error_len().unwrap_or(1));
                }
            },
        }
    }
    push_string(&mut out, &mut current);
    out
}

/// Extracts bounded printable UTF-16LE strings used by Windows metadata.
fn utf16le_strings(bytes: &[u8]) -> Vec<String> {
    let bounded = bytes;
    let mut out = Vec::new();
    for offset in [0_usize, 1_usize] {
        append_utf16le_strings(bounded, offset, &mut out);
    }
    out
}

/// Scans one byte alignment for printable UTF-16LE strings.
fn append_utf16le_strings(bytes: &[u8], offset: usize, out: &mut Vec<String>) {
    let mut current = Vec::new();
    let mut cursor = offset;
    while let Some(pair) = bytes.get(cursor..cursor.saturating_add(2)) {
        let Ok(pair_bytes) = <[u8; 2]>::try_from(pair) else {
            break;
        };
        let unit = u16::from_le_bytes(pair_bytes);
        let is_boundary = unit == 0
            || char::from_u32(u32::from(unit)).is_some_and(char::is_control);
        if is_boundary {
            push_utf16_string(out, &mut current);
        } else {
            current.push(unit);
        }
        cursor = cursor.saturating_add(2);
    }
    push_utf16_string(out, &mut current);
}

/// Emits a valid UTF-16 string candidate without replacement decoding.
fn push_utf16_string(out: &mut Vec<String>, current: &mut Vec<u16>) {
    if current.len() >= 4 {
        let mut decoded = String::new();
        for decoded_result in char::decode_utf16(current.iter().copied()) {
            match decoded_result {
                Ok(character) => decoded.push(character),
                Err(_error) => push_string(out, &mut decoded),
            }
        }
        push_string(out, &mut decoded);
    }
    current.clear();
}

/// Appends printable Unicode text while treating controls as boundaries.
fn append_printable(value: &str, out: &mut Vec<String>, current: &mut String) {
    for character in value.chars() {
        if character.is_control() {
            push_string(out, current);
        } else {
            current.push(character);
        }
    }
}

/// Emits a source-like string candidate when it has useful content.
fn push_string(out: &mut Vec<String>, current: &mut String) {
    if current.len() >= 4 {
        out.push(std::mem::take(current));
    } else {
        current.clear();
    }
}

/// Looks like source name.
fn looks_like_source_name(value: &str) -> bool {
    let lowered = value.to_ascii_lowercase();
    [
        ".mov", ".mp4", ".m4v", ".avi", ".wmv", ".mpg", ".mpeg", ".wav",
        ".aif", ".aiff", ".bik", ".bk2", ".rmv",
    ]
    .iter()
    .any(|extension| contains_source_extension(&lowered, extension))
}

/// Reports whether an extension has a non-empty filename stem before it.
fn has_filename_stem(value: &str, extension_start: usize) -> bool {
    value
        .get(..extension_start)
        .and_then(|prefix| prefix.chars().next_back())
        .is_some_and(|previous| {
            !previous.is_whitespace()
                && previous != '.'
                && previous != '/'
                && previous != char::from(92)
                && previous != char::from(34)
                && previous != char::from(39)
        })
}

/// Reports whether an extension is followed by a filename boundary rather than
/// being only the prefix of a longer word or suffix.
fn contains_source_extension(value: &str, extension: &str) -> bool {
    value.match_indices(extension).any(|(start, _matched)| {
        has_filename_stem(value, start)
            && value
                .get(start.saturating_add(extension.len())..)
                .and_then(|remainder| remainder.chars().next())
                .is_none_or(|next| {
                    !next.is_ascii_alphanumeric()
                        && next != '_'
                        && next != '.'
                        && next != '-'
                        && next != '/'
                        && next != char::from(92)
                })
    })
}

#[cfg(test)]
#[path = "../../../../tests/formats/rmv/unit/domain/provenance/tests.rs"]
mod tests;
