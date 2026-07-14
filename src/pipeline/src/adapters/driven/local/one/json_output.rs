// File:
//   - json_output.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/one/json_output.rs
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
//   - Final syntax validation for pipeline-generated JSON and JSONL files.
// - Must-Not:
//   - Change generated content, classify assets, or validate unrelated files.
// - Allows:
//   - Read one generated text file and reject malformed JSON before completion.
// - Split-When:
//   - Split when another generated text format gains independent validation.
// - Merge-When:
//   - Another phase-one module owns the same generated JSON syntax contract.
// - Summary:
//   - Fails extraction closed when generated JSON syntax is malformed.
// - Description:
//   - Validates complete JSON documents and each nonempty JSONL row before the
//   - pipeline reports normalized extraction success.
// - Usage:
//   - Called by package-resume checks and the final normalization audit.
// - Defaults:
//   - Non-JSON files are accepted without reading their contents.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Validates generated JSON syntax before extraction can report completion.
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
            let bytes = fs::read(path).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to read generated JSON {}: {error}",
                            path.display()
                        ),
                    )
                },
            )?;
            validate_document(
                &bytes,
                &path
                    .display()
                    .to_string(),
            )
        }
        Some("jsonl") => {
            let text = fs::read_to_string(path).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to read generated JSONL {}: {error}",
                            path.display()
                        ),
                    )
                },
            )?;
            validate_lines(
                &text,
                &path
                    .display()
                    .to_string(),
            )
        }
        _ => Ok(()),
    }
}

/// Validate one complete JSON document.
fn validate_document(
    bytes: &[u8],
    label: &str,
) -> PipelineOutcome<()> {
    serde_json::from_slice::<serde_json::Value>(bytes)
        .map(|_value| ())
        .map_err(
            |error| {
                PipelineError::new(
                    format!("generated JSON is invalid for {label}: {error}"),
                )
            },
        )
}

/// Validate every nonempty JSONL row independently.
fn validate_lines(
    text: &str,
    label: &str,
) -> PipelineOutcome<()> {
    for (index, line) in text
        .lines()
        .enumerate()
        .filter(
            |(_index, line)| {
                !line
                    .trim()
                    .is_empty()
            },
        )
    {
        let _value = serde_json::from_str::<serde_json::Value>(line).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "generated JSONL row {} is invalid for {label}: \
                         {error}",
                        index.saturating_add(1),
                    ),
                )
            },
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{validate_document, validate_lines};

    #[test]
    fn rejects_raw_control_characters() {
        let quote = char::from(34);
        let nul = char::from(0);
        let invalid =
            format!("{{{quote}name{quote}:{quote}bad{nul}value{quote}}}");
        assert!(
            validate_document(
                invalid.as_bytes(),
                "raw-control",
            )
            .is_err()
        );
    }

    #[test]
    fn accepts_escaped_controls_and_valid_jsonl_rows() {
        let quote = char::from(34);
        let slash = char::from(92);
        let escaped = format!(
            "{{{quote}name{quote}:{quote}good{slash}u0000value{quote}}}"
        );
        let rows =
            format!("{{{quote}row{quote}:1}}\n{{{quote}row{quote}:2}}\n");
        assert!(
            validate_document(
                escaped.as_bytes(),
                "escaped-control",
            )
            .is_ok()
        );
        assert!(
            validate_lines(
                &rows, "rows",
            )
            .is_ok()
        );
    }
}
