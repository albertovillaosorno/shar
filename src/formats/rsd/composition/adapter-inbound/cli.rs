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
//   - Implements the declared inbound adapter responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Cli inbound adapter.

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
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        let Some((output_arg, root_args)) = arguments.split_first() else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        if root_args.is_empty() {
            return CommandOutcome::failure().stderr_line(USAGE);
        }
        let roots = root_args.iter().map(PathBuf::from).collect::<Vec<_>>();
        match ExportRoots::execute(
            &FilesystemExporter,
            &roots,
            Path::new(output_arg),
        ) {
            Ok(report) => report_outcome(&report),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            },
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
        outcome = outcome.stderr_line(format!(
            "{}: {} files, {} source bytes, {} wav bytes",
            EscapedPath::new(&root.root),
            root.files,
            root.source_bytes,
            root.wav_bytes
        ));
    }
    outcome = outcome
        .stderr_line(format!(
            "total: {} files, {} source bytes, {} wav bytes",
            report.total_files, report.source_bytes, report.wav_bytes
        ))
        .stderr_line("formats:");
    for (header, count) in &report.format_counts {
        let encoding = match header.encoding {
            RsdEncoding::PcmLittleEndian => "PCM",
            RsdEncoding::PcmBigEndian => "PCMB",
            RsdEncoding::RadicalAdpcm => "RADP",
        };
        outcome = outcome.stderr_line(format!(
            "  {count} x {encoding}, {} channel(s), {} Hz, {} bits",
            header.channels, header.sample_rate, header.bits_per_sample
        ));
    }
    outcome
}

/// Executes the RSD export CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&RsdExportProgram)
}

#[cfg(test)]
#[path = "../../../../../tests/formats/rsd/unit/adapter-inbound/cli/tests.rs"]
mod tests;
