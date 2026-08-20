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
//   - Cli inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Cli inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for rcf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Cli inbound adapter.

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use crate::adapters::driven::{FileArchiveSource, FileEntrySink};
use crate::application::{Extractor, ListArchive};
use crate::ports::{ArchiveSource, EntrySink, NoopObserver};

/// Exact multi-command usage contract.
const USAGE: &str = concat!(
    "usage:\n  rcf list <archive.rcf>\n",
    "  rcf extract <archive.rcf> <output-root>\n",
    "  rcf extract-many <output-root> <archive.rcf>...",
);

/// Process-neutral RCF command program.
#[derive(Debug, Default, Clone, Copy)]
pub struct RcfProgram;

impl CliProgram for RcfProgram {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        run(arguments)
    }
}

/// Executes the RCF command using process arguments.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&RcfProgram)
}

/// Executes one decoded command request without touching process streams.
#[must_use]
pub fn run(arguments: &[String]) -> CommandOutcome {
    match arguments {
        [command, archive] if command == "list" => list_archive(archive),
        [command, archive, output_root] if command == "extract" => {
            extract_archives(output_root, &[archive.as_str()])
        },
        [command, output_root, archives @ ..]
            if command == "extract-many" && !archives.is_empty() =>
        {
            let archive_paths =
                archives.iter().map(String::as_str).collect::<Vec<_>>();
            extract_archives(output_root, &archive_paths)
        },
        _ => failure_with_lines(&[], USAGE),
    }
}

/// Renders one archive index as ordered tabular stdout rows.
fn list_archive(archive: &str) -> CommandOutcome {
    let source = FileArchiveSource::new(archive);
    let entries = match ListArchive::execute(&source) {
        Ok(entries) => entries,
        Err(error) => {
            let message = error.to_string();
            return failure_with_lines(&[], &message);
        },
    };
    let mut outcome = CommandOutcome::success();
    for entry in entries {
        outcome = outcome.stdout_line(format!(
            "{hash:08x}\t{offset}\t{length}\t{name}",
            hash = entry.hash,
            offset = entry.offset,
            length = entry.length,
            name = entry.name
        ));
    }
    outcome
}

/// Resolves archive sources and proves their output directories are unique.
///
/// # Errors
///
/// Returns a diagnostic when a source has no usable stem or two archive stems
/// identify the same case-insensitive output directory.
fn prepare_sources(
    archives: &[&str],
) -> Result<Vec<FileArchiveSource>, String> {
    let mut sources = Vec::with_capacity(archives.len());
    let mut output_directories = BTreeSet::new();
    let mut sink_validator = FileEntrySink::new(PathBuf::new());
    for archive in archives {
        let source = FileArchiveSource::new(PathBuf::from(archive));
        let archive_stem =
            source.archive_stem().map_err(|error| error.to_string())?;
        sink_validator
            .prepare_archive(&archive_stem, &[])
            .map_err(|error| error.to_string())?;
        let output_identity = archive_stem.to_lowercase();
        if !output_directories.insert(output_identity) {
            let message =
                format!("duplicate archive output directory: {archive_stem}");
            return Err(message);
        }
        sources.push(source);
    }
    Ok(sources)
}

/// Extracts one or more archives while preserving completed summaries on error.
fn extract_archives(output_root: &str, archives: &[&str]) -> CommandOutcome {
    let sources = match prepare_sources(archives) {
        Ok(sources) => sources,
        Err(message) => {
            return failure_with_lines(&[], &message);
        },
    };
    let mut lines = Vec::new();
    let mut total_archive_bytes = 0_u64;
    let mut total_extracted_bytes = 0_u64;
    let mut total_entries = 0_usize;
    for source in &sources {
        let mut sink = FileEntrySink::new(PathBuf::from(output_root));
        let mut observer = NoopObserver;
        let report = match Extractor::extract(source, &mut sink, &mut observer)
        {
            Ok(report) => report,
            Err(error) => {
                let message = error.to_string();
                return failure_with_lines(&lines, &message);
            },
        };
        lines.push(format!(
            "{stem}: entries={entries} extracted_bytes={extracted} \
                 archive_bytes={archive_bytes} overhead_bytes={overhead} \
                 ratio={ratio:.4} zero_length_entries={zero}",
            stem = report.archive_stem,
            entries = report.entry_count,
            extracted = report.extracted_bytes,
            archive_bytes = report.archive_bytes,
            overhead = report.overhead_bytes(),
            ratio = report.extracted_ratio(),
            zero = report.zero_length_entries
        ));
        total_archive_bytes =
            match total_archive_bytes.checked_add(report.archive_bytes) {
                Some(value) => value,
                None => {
                    return failure_with_lines(
                        &lines,
                        "total archive byte count overflowed",
                    );
                },
            };
        total_extracted_bytes =
            match total_extracted_bytes.checked_add(report.extracted_bytes) {
                Some(value) => value,
                None => {
                    return failure_with_lines(
                        &lines,
                        "total extracted byte count overflowed",
                    );
                },
            };
        total_entries = match total_entries.checked_add(report.entry_count) {
            Some(value) => value,
            None => {
                return failure_with_lines(
                    &lines,
                    "total entry count overflowed",
                );
            },
        };
    }
    lines.push(format!(
        "TOTAL: archives={archives} entries={entries} \
             extracted_bytes={extracted} archive_bytes={archive_bytes} \
             overhead_bytes={overhead} ratio={ratio:.4}",
        archives = archives.len(),
        entries = total_entries,
        extracted = total_extracted_bytes,
        archive_bytes = total_archive_bytes,
        overhead = total_archive_bytes.saturating_sub(total_extracted_bytes),
        ratio = extracted_ratio(total_extracted_bytes, total_archive_bytes)
    ));
    success_with_lines(&lines)
}

/// Builds one successful outcome from ordered stdout lines.
fn success_with_lines(lines: &[String]) -> CommandOutcome {
    let mut outcome = CommandOutcome::success();
    for line in lines {
        outcome = outcome.stdout_line(line);
    }
    outcome
}

/// Preserves completed stdout rows before one final stderr diagnostic.
fn failure_with_lines(lines: &[String], error: &str) -> CommandOutcome {
    let mut outcome = CommandOutcome::failure();
    for line in lines {
        outcome = outcome.stdout_line(line);
    }
    outcome.stderr_line(format!("error: {error}"))
}

// Floating-point is restricted to human diagnostics beside exact byte counts.
#[expect(
    clippy::as_conversions,
    clippy::cast_precision_loss,
    reason = "Human-facing ratios accompany exact archive and payload byte \
              counts, so diagnostic precision loss cannot affect extraction."
)]
/// Computes the human-facing extraction ratio printed beside exact counts.
fn extracted_ratio(extracted_bytes: u64, archive_bytes: u64) -> f64 {
    if archive_bytes == 0 {
        0.
    } else {
        extracted_bytes as f64 / archive_bytes as f64
    }
}

#[cfg(test)]
#[path = "../../../../../tests/formats/rcf/unit/adapter-inbound/cli/tests.rs"]
mod tests;
