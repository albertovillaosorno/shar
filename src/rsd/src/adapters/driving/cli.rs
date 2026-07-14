// File:
//   - cli.rs
// Path:
//   - src/rsd/src/adapters/driving/cli.rs
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
//   - The RSD command-line driving adapter.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute command-line composition.
// - Split-When:
//   - Split when command-line composition contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another rsd module owns the same driving adapter boundary with no
//   - distinct invariant.
// - Summary:
//   - Runs RSD audio export commands through the application boundary.
// - Description:
//   - Defines command-line composition data and behavior for rsd root.
// - Usage:
//   - Invoked through the crate command-line binary.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Process-neutral RSD export command composition.
//!
//! Argument meaning and export summaries remain local to this adapter.
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use crate::adapters::driven::FilesystemExporter;
use crate::application::ExportRoots;
use crate::domain::{EscapedPath, ExportReport, RsdEncoding};

/// Exact usage contract for RSD export.
const USAGE: &str = "usage: rsd-export <OUTPUT_ROOT> <INPUT_ROOT>...";

/// Process-neutral RSD export program.
#[derive(Debug, Default, Clone, Copy)]
pub struct RsdExportProgram;

impl CliProgram for RsdExportProgram {
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
        let Some((output_arg, root_args)) = arguments.split_first() else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        if root_args.is_empty() {
            return CommandOutcome::failure().stderr_line(USAGE);
        }
        let roots = root_args
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<_>>();
        match ExportRoots::execute(
            &FilesystemExporter,
            &roots,
            Path::new(output_arg),
        ) {
            Ok(report) => report_outcome(&report),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            }
        }
    }
}

/// Renders export evidence in its historical stderr order.
fn report_outcome(report: &ExportReport) -> CommandOutcome {
    if let Err(error) = report.validate() {
        return CommandOutcome::failure().stderr_line(error.to_string());
    }
    let mut outcome = CommandOutcome::success();
    for root in &report.source_roots {
        outcome = outcome.stderr_line(
            format!(
                "{}: {} files, {} source bytes, {} wav bytes",
                EscapedPath::new(&root.root),
                root.files,
                root.source_bytes,
                root.wav_bytes
            ),
        );
    }
    outcome = outcome
        .stderr_line(
            format!(
                "total: {} files, {} source bytes, {} wav bytes",
                report.total_files, report.source_bytes, report.wav_bytes
            ),
        )
        .stderr_line("formats:");
    for (header, count) in &report.format_counts {
        let encoding = match header.encoding {
            RsdEncoding::PcmLittleEndian => "PCM",
            RsdEncoding::PcmBigEndian => "PCMB",
            RsdEncoding::RadicalAdpcm => "RADP",
        };
        outcome = outcome.stderr_line(
            format!(
                "  {count} x {encoding}, {} channel(s), {} Hz, {} bits",
                header.channels, header.sample_rate, header.bits_per_sample
            ),
        );
    }
    outcome
}

/// Executes the RSD export CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&RsdExportProgram)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use schoenwald_cli::CliProgram;

    use super::{RsdExportProgram, USAGE, report_outcome};
    use crate::domain::{
        ExportReport, RsdEncoding, RsdHeader, SourceRootReport,
    };

    #[test]
    fn inconsistent_reports_fail_instead_of_printing_success() {
        let report = ExportReport {
            source_roots: Vec::new(),
            total_files: 1,
            source_bytes: 2,
            wav_bytes: 46,
            format_counts: BTreeMap::new(),
        };

        let outcome = report_outcome(&report);

        assert_eq!(
            outcome.status(),
            schoenwald_cli::ExitStatus::Failure,
            "inconsistent export evidence must not produce a success status"
        );
    }

    #[test]
    fn successful_reports_escape_control_characters_in_paths()
    -> Result<(), String> {
        let header = RsdHeader {
            encoding: RsdEncoding::PcmLittleEndian,
            channels: 1,
            bits_per_sample: 16,
            sample_rate: 24_000,
        };
        let mut format_counts = BTreeMap::new();
        let _previous_count = format_counts.insert(
            header, 1_usize,
        );
        let report = ExportReport {
            source_roots: vec![
                SourceRootReport {
                    // cspell:disable-next-line -- Jbad
                    root: PathBuf::from("\u{1b}[2Jbad\nroot"),
                    files: 1,
                    source_bytes: 2,
                    wav_bytes: 46,
                },
            ],
            total_files: 1,
            source_bytes: 2,
            wav_bytes: 46,
            format_counts,
        };

        let outcome = report_outcome(&report);
        let Some(first) = outcome
            .output()
            .first()
        else {
            return Err(
                "successful RSD report emitted no root line".to_owned(),
            );
        };
        let Some(line) = first
            .text()
            .strip_suffix('\n')
        else {
            return Err(
                "successful RSD report omitted its line terminator".to_owned(),
            );
        };
        if line.contains('\u{1b}') || line.contains('\n') {
            return Err(
                "successful RSD report emitted raw control characters"
                    .to_owned(),
            );
        }
        // cspell:disable-next-line -- Jbad
        if !line.contains(r"\u{1b}[2Jbad\nroot") {
            return Err(
                format!(
                    "successful RSD report lost escaped path evidence: {:?}",
                    first.text()
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn missing_roots_return_one_usage_diagnostic() -> Result<(), String> {
        for arguments in [
            Vec::new(),
            vec!["output".to_owned()],
        ] {
            let outcome = RsdExportProgram.execute(&arguments);
            if !outcome.is_failure_with_stderr_line(USAGE) {
                return Err(
                    format!(
                        "invalid RSD usage outcome for arguments: \
                         {arguments:?}"
                    ),
                );
            }
        }
        Ok(())
    }
}
