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
//   - Audit minor units outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Audit minor units outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Audit minor units outbound adapter.

use std::path::Path;

use schoenwald_filesystem::adapters::driving::local::{
    read_utf8 as local_read_utf8, write_text as local_write_text,
};
use shar_sha256::digest_hex;

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
pub(in crate::adapters::driven::local) fn audit_minor_units(
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    validate_taxonomy_source()?;
    let manifest_path = taxonomy::manifest_path(extracted_root);
    let manifest =
        local_read_utf8(&manifest_path).map_err(io_error(&manifest_path))?;
    let mut rows = 0usize;
    let mut failures = Vec::new();
    let mut error_rows = 0usize;

    for (line_index, line) in manifest.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        rows = rows.saturating_add(1);
        for field in taxonomy::REQUIRED_FIELDS {
            if read_string_field(line, field).is_none() {
                failures.push(format!(
                    "line {} missing field {field}",
                    line_index.saturating_add(1)
                ));
            }
        }
        if let Some(path) = read_string_field(line, "path") {
            if !path.starts_with("extracted/") && !path.starts_with("game/") {
                failures.push(format!(
                    "line {} path is not under the generated asset \
                         staging root",
                    line_index.saturating_add(1)
                ));
            }
            if path.starts_with("extracted/minor-unit/") {
                failures.push(format!(
                    "line {} inventories generated minor-unit output",
                    line_index.saturating_add(1)
                ));
            }
        }
        if read_string_field(line, "id")
            .is_none_or(|value| value.is_empty() || value == taxonomy::UNKNOWN)
        {
            failures.push(format!(
                "line {} has no stamped opaque id",
                line_index.saturating_add(1)
            ));
        }
        if read_string_field(line, "obfuscated_route")
            .is_none_or(|value| value.is_empty())
        {
            failures.push(format!(
                "line {} has no obfuscated route",
                line_index.saturating_add(1)
            ));
        }
        if taxonomy::CLASSIFICATION_FIELDS.iter().any(|field| {
            read_string_field(line, field)
                .is_some_and(|value| value == taxonomy::UNKNOWN)
        }) {
            error_rows = error_rows.saturating_add(1);
            failures.push(format!(
                "line {} still has error classification metadata",
                line_index.saturating_add(1)
            ));
        }
        for field in taxonomy::REQUIRED_FIELDS {
            if let Some(values) = taxonomy::controlled_values(field)
                && let Some(value) = read_string_field(line, field)
                && !values.contains(&value.as_str())
            {
                failures.push(format!(
                    "line {} field {field} has value {value}, outside \
                         taxonomy",
                    line_index.saturating_add(1)
                ));
            }
        }
    }

    let report = audit_summary_json(
        rows,
        failures.len(),
        error_rows,
        &digest_hex(manifest.as_bytes()),
    );
    let audit_path = taxonomy::audit_path(extracted_root);
    local_write_text(&audit_path, &report, true)
        .map_err(io_error(&audit_path))?;

    if !failures.is_empty() {
        return Err(PipelineError::new(format!(
            "minor-unit audit failed with {} issue(s); first: {}",
            failures.len(),
            failures.first().map_or("<none>", String::as_str)
        )));
    }

    Ok(StageReport {
        name: "minor-unit-audit",
        files: rows,
        bytes: 0,
        note: "minor-unit manifest conforms to taxonomy and output \
                   boundaries"
            .to_owned(),
    })
}

/// Validate taxonomy source.
fn validate_taxonomy_source() -> PipelineOutcome<()> {
    let taxonomy = taxonomy::TAXONOMY_JSON;
    for field in taxonomy::REQUIRED_FIELDS {
        if !taxonomy.contains(field) {
            return Err(PipelineError::new(format!(
                "minor-unit taxonomy is missing field {field}"
            )));
        }
    }
    if !taxonomy.contains("kebab-case") || !taxonomy.contains(taxonomy::UNKNOWN)
    {
        return Err(PipelineError::new(
            "minor-unit taxonomy is missing value case policy or error \
                 sentinel",
        ));
    }
    Ok(())
}

/// Emits stable audit JSON because downstream validation compares the schema
/// string instead of human prose.
fn audit_summary_json(
    rows: usize,
    failures: usize,
    error_rows: usize,
    manifest_sha256: &str,
) -> String {
    format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.minor-unit-audit.v2\",",
            "\"rows\":{},\"failures\":{},\"error_rows\":{},",
            "\"manifest_sha256\":\"{}\"}}\n"
        ),
        rows, failures, error_rows, manifest_sha256,
    )
}

/// Io error.
fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> PipelineError + '_ {
    move |error| PipelineError::new(format!("{}: {error}", path.display()))
}
