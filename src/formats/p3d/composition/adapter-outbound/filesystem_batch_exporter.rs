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
//   - Filesystem batch exporter outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem batch exporter outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem batch exporter outbound adapter.

use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local;
use schoenwald_filesystem::{DiagnosticPath, PathKind};

use super::LosslessPackageExporter;
use super::filesystem_batch_cache::is_cache_current;
use super::json::escape_json;
use super::root_identity::root_identity_path;
use crate::application::ExportPackage;
use crate::domain::PackageExportReport;
use crate::ports::PackageBatchExporter;

/// Local filesystem provider for batch package export.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemBatchExporter;

impl PackageBatchExporter for FilesystemBatchExporter {
    type Error = Box<dyn std::error::Error>;

    fn export_batch(
        &self,
        output_root: &Path,
        input_roots: &[PathBuf],
    ) -> Result<PackageExportReport, Self::Error> {
        export_batch(output_root, input_roots)
    }
}

/// Executes one complete local batch export pass.
fn export_batch(
    output_root: &Path,
    input_roots: &[PathBuf],
) -> Result<PackageExportReport, Box<dyn std::error::Error>> {
    local::create_dir_all(output_root)?;
    let report_path = output_root.join("p3d-batch-report.jsonl");
    let cache_root = PathBuf::from("cache/p3d");
    local::create_dir_all(&cache_root)?;
    let cache_path = cache_root.join("batch-cache.jsonl");
    let mut report = String::new();
    let mut cache = String::new();
    let mut totals = PackageExportReport::default();
    for input_root in input_roots {
        if local::path_kind(input_root)? == PathKind::Missing {
            continue;
        }
        let files = local::regular_files(input_root)?
            .into_iter()
            .filter(|path| has_p3d_extension(path))
            .collect::<Vec<_>>();
        for file in files {
            totals.scanned = totals.scanned.saturating_add(1);
            let relative = file.strip_prefix(input_root).unwrap_or(&file);
            let output_dir = output_root
                .join(root_identity_path(input_root))
                .join(path_without_extension(relative));
            if is_cache_current(&output_dir, &file) {
                totals.skipped = totals.skipped.saturating_add(1);
                let row = report_line(
                    "skipped_complete",
                    input_root,
                    &file,
                    &output_dir,
                    "",
                );
                report.push_str(&row);
                cache.push_str(&row);
                continue;
            }
            match ExportPackage::execute(
                &LosslessPackageExporter,
                &file,
                &output_dir,
            ) {
                Ok(()) => {
                    totals.extracted = totals.extracted.saturating_add(1);
                    let status = if is_cache_current(&output_dir, &file) {
                        "ok_complete"
                    } else {
                        "ok_pending"
                    };
                    let row =
                        report_line(status, input_root, &file, &output_dir, "");
                    report.push_str(&row);
                    cache.push_str(&row);
                },
                Err(error) => {
                    totals.failed = totals.failed.saturating_add(1);
                    let row = report_line(
                        "failed",
                        input_root,
                        &file,
                        &output_dir,
                        &error.to_string(),
                    );
                    report.push_str(&row);
                    cache.push_str(&row);
                },
            }
        }
    }
    local::write_text(&report_path, &report, true)?;
    local::write_text(&cache_path, &cache, true)?;
    Ok(totals)
}

/// Returns whether one path has a case-insensitive `.p3d` extension.
fn has_p3d_extension(path: &Path) -> bool {
    path.extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("p3d"))
}

/// Returns a path with `.p3d` leaf extensions removed.
fn path_without_extension(path: &Path) -> PathBuf {
    if has_p3d_extension(path) {
        path.with_extension("")
    } else {
        path.to_path_buf()
    }
}

/// Renders one JSONL batch-report row.
fn report_line(
    status: &str,
    root: &Path,
    input: &Path,
    output: &Path,
    error: &str,
) -> String {
    format!(
        concat!(
            "{{\"status\":\"{}\",",
            "\"root\":\"{}\",",
            "\"input\":\"{}\",",
            "\"output\":\"{}\",",
            "\"error\":\"{}\"}}\n",
        ),
        escape_json(status),
        escape_json(&DiagnosticPath::new(root).to_string()),
        escape_json(&DiagnosticPath::new(input).to_string()),
        escape_json(&DiagnosticPath::new(output).to_string()),
        escape_json(error)
    )
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-outbound/filesystem_batch_exporter_tests.rs"]
mod tests;
