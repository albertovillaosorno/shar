// File:
//   - audit_minor_units.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/audit_minor_units.rs
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
//   - The audit minor units contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute audit minor units.
// - Split-When:
//   - Split when audit minor units contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Returns an error when validation, filesystem access, or output
//   - writing.
// - Description:
//   - Defines audit minor units data and behavior for pipeline phase two
//   - minor units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs audit minor
//   - units.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Returns an error when validation, filesystem access, or output writing.
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local::{
    read_utf8 as local_read_utf8, write_text as local_write_text,
};

use super::metadata_fill::read_string_field;
use super::taxonomy;
use crate::domain::{PipelineError, StageReport};

/// Result.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Audit minor units.
///
/// # Errors
///
/// Returns an error when validation, filesystem access, or output writing
/// fails.
// One immutable manifest snapshot owns every audit count and report row.
#[expect(
    clippy::too_many_lines,
    reason = "The audit evaluates coverage, errors, counts, and report \
              emission from one immutable manifest snapshot."
)]
pub(in crate::adapters::driven::local) fn audit_minor_units(
    extracted_root: &Path
) -> PipelineOutcome<StageReport> {
    validate_taxonomy_source()?;
    let manifest_path = taxonomy::manifest_path(extracted_root);
    let manifest =
        local_read_utf8(&manifest_path).map_err(io_error(&manifest_path))?;
    let mut rows = 0usize;
    let mut failures = Vec::new();
    let mut error_rows = 0usize;

    for (line_index, line) in manifest
        .lines()
        .enumerate()
    {
        if line
            .trim()
            .is_empty()
        {
            continue;
        }
        rows = rows.saturating_add(1);
        for field in taxonomy::REQUIRED_FIELDS {
            if read_string_field(
                line, field,
            )
            .is_none()
            {
                failures.push(
                    format!(
                        "line {} missing field {field}",
                        line_index.saturating_add(1)
                    ),
                );
            }
        }
        if let Some(path) = read_string_field(
            line, "path",
        ) {
            if !path.starts_with("extracted/") && !path.starts_with("game/") {
                failures.push(
                    format!(
                        "line {} path is not under the generated asset \
                         staging root",
                        line_index.saturating_add(1)
                    ),
                );
            }
            if path.starts_with("extracted/minor-unit/") {
                failures.push(
                    format!(
                        "line {} inventories generated minor-unit output",
                        line_index.saturating_add(1)
                    ),
                );
            }
        }
        if read_string_field(
            line, "id",
        )
        .is_none_or(|value| value.is_empty() || value == taxonomy::UNKNOWN)
        {
            failures.push(
                format!(
                    "line {} has no stamped opaque id",
                    line_index.saturating_add(1)
                ),
            );
        }
        if read_string_field(
            line,
            "obfuscated_route",
        )
        .is_none_or(|value| value.is_empty())
        {
            failures.push(
                format!(
                    "line {} has no obfuscated route",
                    line_index.saturating_add(1)
                ),
            );
        }
        if taxonomy::CLASSIFICATION_FIELDS
            .iter()
            .any(
                |field| {
                    read_string_field(
                        line, field,
                    )
                    .is_some_and(|value| value == taxonomy::UNKNOWN)
                },
            )
        {
            error_rows = error_rows.saturating_add(1);
            failures.push(
                format!(
                    "line {} still has error classification metadata",
                    line_index.saturating_add(1)
                ),
            );
        }
        for field in taxonomy::REQUIRED_FIELDS {
            if let Some(values) = taxonomy::controlled_values(field)
                && let Some(value) = read_string_field(
                    line, field,
                )
                && !values.contains(&value.as_str())
            {
                failures.push(
                    format!(
                        "line {} field {field} has value {value}, outside \
                         taxonomy",
                        line_index.saturating_add(1)
                    ),
                );
            }
        }
    }

    let report = audit_summary_json(
        rows,
        failures.len(),
        error_rows,
    );
    let audit_path = taxonomy::audit_path(extracted_root);
    local_write_text(
        &audit_path,
        &report,
        true,
    )
    .map_err(io_error(&audit_path))?;

    if !failures.is_empty() {
        return Err(
            PipelineError::new(
                format!(
                    "minor-unit audit failed with {} issue(s); first: {}",
                    failures.len(),
                    failures
                        .first()
                        .map_or(
                            "<none>",
                            String::as_str
                        )
                ),
            ),
        );
    }

    Ok(
        StageReport {
            name: "minor-unit-audit",
            files: rows,
            bytes: 0,
            note: "minor-unit manifest conforms to taxonomy and output \
                   boundaries"
                .to_owned(),
        },
    )
}

/// Validate taxonomy source.
fn validate_taxonomy_source() -> PipelineOutcome<()> {
    let taxonomy = taxonomy::TAXONOMY_JSON;
    for field in taxonomy::REQUIRED_FIELDS {
        if !taxonomy.contains(field) {
            return Err(
                PipelineError::new(
                    format!("minor-unit taxonomy is missing field {field}"),
                ),
            );
        }
    }
    if !taxonomy.contains("kebab-case") || !taxonomy.contains(taxonomy::UNKNOWN)
    {
        return Err(
            PipelineError::new(
                "minor-unit taxonomy is missing value case policy or error \
                 sentinel",
            ),
        );
    }
    Ok(())
}

/// Emits stable audit JSON because downstream validation compares the schema
/// string instead of human prose.
fn audit_summary_json(
    rows: usize,
    failures: usize,
    error_rows: usize,
) -> String {
    format!(
        "{{\"schema\":\"shar-schoenwald.minor-unit-audit.v1\",\"rows\":{rows},\
         \"failures\":{failures},\"error_rows\":{error_rows}}}
"
    )
}

/// Io error.
fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> PipelineError + '_ {
    move |error| {
        PipelineError::new(
            format!(
                "{}: {error}",
                path.display()
            ),
        )
    }
}
