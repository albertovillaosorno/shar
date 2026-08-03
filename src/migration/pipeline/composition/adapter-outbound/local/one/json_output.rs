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
//   - Json output outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Json output outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Json output outbound adapter.

use std::fs;
use std::path::Path;

use crate::domain::{PipelineError, PipelineOutcome};

/// Validate one generated JSON or JSONL file based on its extension.
///
/// # Errors
///
/// Returns an error when the file cannot be read or contains malformed JSON.
pub(super) fn validate_generated_text_file(path: &Path) -> PipelineOutcome<()> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase);
    match extension.as_deref() {
        Some("json") => {
            let label =
                schoenwald_filesystem::DiagnosticPath::new(path).to_string();
            let bytes = fs::read(path).map_err(|error| {
                PipelineError::new(format!(
                    "failed to read generated JSON {label}: {error}"
                ))
            })?;
            validate_document(&bytes, &label)
        },
        Some("jsonl") => {
            let label =
                schoenwald_filesystem::DiagnosticPath::new(path).to_string();
            let text = fs::read_to_string(path).map_err(|error| {
                PipelineError::new(format!(
                    "failed to read generated JSONL {label}: {error}"
                ))
            })?;
            validate_lines(&text, &label)
        },
        _ => Ok(()),
    }
}

/// Validate one complete JSON document.
fn validate_document(bytes: &[u8], label: &str) -> PipelineOutcome<()> {
    serde_json::from_slice::<serde_json::Value>(bytes)
        .map(|_value| ())
        .map_err(|error| {
            PipelineError::new(format!(
                "generated JSON is invalid for {label}: {error}"
            ))
        })
}

/// Validate every nonempty JSONL row independently.
fn validate_lines(text: &str, label: &str) -> PipelineOutcome<()> {
    for (index, line) in text
        .lines()
        .enumerate()
        .filter(|(_index, line)| !line.trim().is_empty())
    {
        let _value = serde_json::from_str::<serde_json::Value>(line).map_err(
            |error| {
                PipelineError::new(format!(
                    "generated JSONL row {} is invalid for {label}: \
                         {error}",
                    index.saturating_add(1),
                ))
            },
        )?;
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/one/json_output/tests.rs"]
mod tests;
