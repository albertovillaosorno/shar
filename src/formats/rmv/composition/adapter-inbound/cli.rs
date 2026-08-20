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
//   - Implements the declared inbound adapter responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Cli inbound adapter.

use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use crate::adapters::driven::{FilesystemMovieAuditor, TsvAuditManifestSink};
use crate::application::RunMovieAudit;

/// Exact usage contract for RMV auditing.
const USAGE: &str = "usage: rmv-audit <OUTPUT_ROOT> <INPUT_ROOT>...";

/// Process-neutral RMV audit program.
#[derive(Debug, Default, Clone, Copy)]
pub struct RmvAuditProgram;

impl CliProgram for RmvAuditProgram {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        let Some((output_arg, input_roots)) = arguments.split_first() else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        if output_arg.is_empty() {
            return CommandOutcome::failure().stderr_line(USAGE);
        }
        if input_roots.is_empty() {
            return CommandOutcome::failure().stderr_line(USAGE);
        }
        for input_root in input_roots {
            if input_root.is_empty() {
                return CommandOutcome::failure().stderr_line(USAGE);
            }
        }
        let output_root = PathBuf::from(output_arg);
        let roots = input_roots.iter().map(PathBuf::from).collect::<Vec<_>>();
        match RunMovieAudit::execute(
            &FilesystemMovieAuditor,
            &TsvAuditManifestSink,
            &roots,
            &output_root,
        ) {
            Ok(report) => report_outcome(&report),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            },
        }
    }
}

/// Renders deterministic audit summary rows in their historical order.
fn report_outcome(report: &crate::domain::AuditReport) -> CommandOutcome {
    let mut outcome = CommandOutcome::success()
        .stdout_line(format!("movie inputs: {}", report.records.len()))
        .stdout_line(format!("unique hashes: {}", report.unique_hashes()))
        .stdout_line(format!("duplicate inputs: {}", report.duplicate_inputs))
        .stdout_line(format!(
            "missing bk2 outputs: {}",
            report.missing_bk2_outputs
        ));
    for (kind, count) in report.kind_counts() {
        outcome = outcome.stdout_line(format!("  {count} x {}", kind.label()));
    }
    if report.missing_bk2_outputs > 0 {
        outcome = outcome.stderr_line(
            "bk2 encoder gate remains blocked; no fake files were emitted",
        );
    }
    outcome
}

/// Executes the RMV audit CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&RmvAuditProgram)
}

#[cfg(test)]
#[path = "../../../../../tests/formats/rmv/unit/adapter-inbound/cli/tests.rs"]
mod tests;
