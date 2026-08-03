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
//   - Custom text outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Custom text outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Custom text outbound adapter.

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
    let bytes = local::read_bytes(path)
        .map_err(|source| Error::io(path.to_path_buf(), source))?;
    parse_custom_text_bytes(&bytes, &path.display().to_string())
}

/// Parse loaded bytes so package builders can avoid duplicate filesystem IO.
pub(super) fn parse_custom_text_bytes(
    bytes: &[u8],
    source_label: &str,
) -> Outcome<Vec<CustomTextEntry>> {
    let text = decode_text_source(bytes, source_label)?;
    let mut entries = Vec::new();
    let mut key_lines = BTreeMap::new();
    for (index, line) in text.lines().enumerate() {
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
        let line_number = index.checked_add(1).ok_or_else(|| {
            Error::invalid("custom-text source line overflowed")
        })?;
        let (raw_key, value) = trimmed.split_once('=').ok_or_else(|| {
            Error::invalid(format!(
                "custom-text line {line_number} is missing '='"
            ))
        })?;
        let normalized_key = raw_key.trim();
        if normalized_key.is_empty() {
            return Err(Error::invalid(format!(
                "custom-text line {line_number} has an empty key"
            )));
        }
        let key = normalized_key.to_owned();
        if let Some(first_line) = key_lines.insert(key.clone(), line_number) {
            return Err(Error::invalid(format!(
                "custom-text key '{key}' is duplicated on lines \
                         {first_line} and {line_number}"
            )));
        }
        entries.push(CustomTextEntry {
            key,
            value: value.trim().to_owned(),
            line: line_number,
        });
    }
    Ok(entries)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/localization/custom_text/tests.rs"]
mod tests;
