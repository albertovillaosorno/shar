// File:
//   - cli.rs
// Path:
//   - src/rcf/src/adapters/driving/cli.rs
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
//   - The RCF command-line driving adapter.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Decode CLI requests and present application results.
// - Split-When:
//   - Split when another inbound protocol requires an independent adapter.
// - Merge-When:
//   - Another rcf module owns the same driving adapter boundary with no
//   - distinct invariant.
// - Summary:
//   - Command-line adapter for the RCF extractor.
// - Description:
//   - Defines command-line request handling data and behavior for rcf root.
// - Usage:
//   - Invoked through the crate command-line binary.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Command dispatch, archive preflight, extraction aggregation, and
//   - focused CLI regressions remain one cohesive driving-adapter boundary.
//

//! Command-line adapter for the RCF extractor.
//!
//! Command syntax and presentation remain local while shared process mechanics
//! stay centralized in `schoenwald-cli`.
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
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
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
        [
            command,
            archive,
        ] if command == "list" => list_archive(archive),
        [
            command,
            archive,
            output_root,
        ] if command == "extract" => extract_archives(
            output_root,
            &[archive.as_str()],
        ),
        [
            command,
            output_root,
            archives @ ..,
        ] if command == "extract-many" && !archives.is_empty() => {
            let archive_paths = archives
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>();
            extract_archives(
                output_root,
                &archive_paths,
            )
        }
        _ => failure_with_lines(
            &[],
            USAGE,
        ),
    }
}

/// Renders one archive index as ordered tabular stdout rows.
fn list_archive(archive: &str) -> CommandOutcome {
    let source = FileArchiveSource::new(archive);
    let entries = match ListArchive::execute(&source) {
        Ok(entries) => entries,
        Err(error) => {
            let message = error.to_string();
            return failure_with_lines(
                &[],
                &message,
            );
        }
    };
    let mut outcome = CommandOutcome::success();
    for entry in entries {
        outcome = outcome.stdout_line(
            format!(
                "{hash:08x}\t{offset}\t{length}\t{name}",
                hash = entry.hash,
                offset = entry.offset,
                length = entry.length,
                name = entry.name
            ),
        );
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
    archives: &[&str]
) -> Result<Vec<FileArchiveSource>, String> {
    let mut sources = Vec::with_capacity(archives.len());
    let mut output_directories = BTreeSet::new();
    let mut sink_validator = FileEntrySink::new(PathBuf::new());
    for archive in archives {
        let source = FileArchiveSource::new(PathBuf::from(archive));
        let archive_stem = source
            .archive_stem()
            .map_err(|error| error.to_string())?;
        sink_validator
            .prepare_archive(
                &archive_stem,
                &[],
            )
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
fn extract_archives(
    output_root: &str,
    archives: &[&str],
) -> CommandOutcome {
    let sources = match prepare_sources(archives) {
        Ok(sources) => sources,
        Err(message) => {
            return failure_with_lines(
                &[],
                &message,
            );
        }
    };
    let mut lines = Vec::new();
    let mut total_archive_bytes = 0_u64;
    let mut total_extracted_bytes = 0_u64;
    let mut total_entries = 0_usize;
    for source in &sources {
        let mut sink = FileEntrySink::new(PathBuf::from(output_root));
        let mut observer = NoopObserver;
        let report = match Extractor::extract(
            source,
            &mut sink,
            &mut observer,
        ) {
            Ok(report) => report,
            Err(error) => {
                let message = error.to_string();
                return failure_with_lines(
                    &lines, &message,
                );
            }
        };
        lines.push(
            format!(
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
            ),
        );
        total_archive_bytes =
            match total_archive_bytes.checked_add(report.archive_bytes) {
                Some(value) => value,
                None => {
                    return failure_with_lines(
                        &lines,
                        "total archive byte count overflowed",
                    );
                }
            };
        total_extracted_bytes =
            match total_extracted_bytes.checked_add(report.extracted_bytes) {
                Some(value) => value,
                None => {
                    return failure_with_lines(
                        &lines,
                        "total extracted byte count overflowed",
                    );
                }
            };
        total_entries = match total_entries.checked_add(report.entry_count) {
            Some(value) => value,
            None => {
                return failure_with_lines(
                    &lines,
                    "total entry count overflowed",
                );
            }
        };
    }
    lines.push(
        format!(
            "TOTAL: archives={archives} entries={entries} \
             extracted_bytes={extracted} archive_bytes={archive_bytes} \
             overhead_bytes={overhead} ratio={ratio:.4}",
            archives = archives.len(),
            entries = total_entries,
            extracted = total_extracted_bytes,
            archive_bytes = total_archive_bytes,
            overhead =
                total_archive_bytes.saturating_sub(total_extracted_bytes),
            ratio = extracted_ratio(
                total_extracted_bytes,
                total_archive_bytes
            )
        ),
    );
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
fn failure_with_lines(
    lines: &[String],
    error: &str,
) -> CommandOutcome {
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
fn extracted_ratio(
    extracted_bytes: u64,
    archive_bytes: u64,
) -> f64 {
    if archive_bytes == 0 {
        0.0
    } else {
        extracted_bytes as f64 / archive_bytes as f64
    }
}

#[cfg(test)]
mod tests {
    use schoenwald_cli::CliProgram;

    use super::{RcfProgram, USAGE, prepare_sources};

    #[test]
    fn unsafe_archive_output_directories_fail_before_io() {
        let archives = [
            "first/good.rcf",
            "second/CON.rcf",
        ];
        let result = prepare_sources(&archives);

        assert!(
            result.is_err(),
            "unsafe archive stems must fail during batch preflight"
        );
    }

    #[test]
    fn duplicate_archive_output_directories_fail_before_io()
    -> Result<(), String> {
        let arguments = vec![
            "extract-many".to_owned(),
            "output".to_owned(),
            "first/Music.rcf".to_owned(),
            "second/music.rcf".to_owned(),
        ];
        let outcome = RcfProgram.execute(&arguments);
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err("duplicate archive stems were accepted".to_owned());
        }
        let [chunk] = outcome.output() else {
            return Err("duplicate stems must emit one diagnostic".to_owned());
        };
        let diagnostic = chunk.text();
        if !diagnostic.contains("duplicate archive output directory") {
            let message =
                format!("unexpected duplicate-stem diagnostic: {diagnostic:?}");
            return Err(message);
        }
        Ok(())
    }

    #[test]
    fn invalid_requests_return_one_prefixed_usage_diagnostic()
    -> Result<(), String> {
        for arguments in [
            Vec::new(),
            vec!["list".to_owned()],
            vec![
                "extract-many".to_owned(),
                "output".to_owned(),
            ],
        ] {
            let outcome = RcfProgram.execute(&arguments);
            if outcome.status() != schoenwald_cli::ExitStatus::Failure {
                return Err(format!("invalid request passed: {arguments:?}"));
            }
            let [chunk] = outcome.output() else {
                return Err(
                    "invalid request must emit one diagnostic".to_owned(),
                );
            };
            if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
                return Err(
                    "usage diagnostic must be written to stderr".to_owned(),
                );
            }
            let expected = format!("error: {USAGE}\n");
            if chunk.text() != expected {
                return Err(
                    format!(
                        "unexpected usage diagnostic: {:?}",
                        chunk.text()
                    ),
                );
            }
        }
        Ok(())
    }
}
