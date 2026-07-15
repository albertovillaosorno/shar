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
            let label =
                schoenwald_filesystem::DiagnosticPath::new(path).to_string();
            let bytes = fs::read(path).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to read generated JSON {label}: {error}"
                        ),
                    )
                },
            )?;
            validate_document(
                &bytes, &label,
            )
        }
        Some("jsonl") => {
            let label =
                schoenwald_filesystem::DiagnosticPath::new(path).to_string();
            let text = fs::read_to_string(path).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to read generated JSONL {label}: {error}"
                        ),
                    )
                },
            )?;
            validate_lines(
                &text, &label,
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

    #[cfg(windows)]
    #[test]
    fn non_unicode_json_path_error_is_reversible() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt as _;

        let path = std::path::PathBuf::from(
            OsString::from_wide(
                &[
                    u16::from(b'a'),
                    0xd800_u16,
                    u16::from(b'b'),
                    u16::from(b'.'),
                    u16::from(b'j'),
                    u16::from(b's'),
                    u16::from(b'o'),
                    u16::from(b'n'),
                ],
            ),
        );
        let error = super::validate_generated_text_file(&path)
            .expect_err("missing non-Unicode JSON unexpectedly passed");
        let rendered = error.to_string();
        let prefix = r"failed to read generated JSON a\u{D800}b.json: ";
        let Some(_source_message) = rendered.strip_prefix(prefix) else {
            panic!("diagnostic lost native path: {rendered:?}");
        };
    }

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
