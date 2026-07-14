// File:
//   - cli.rs
// Path:
//   - src/rmv/src/adapters/driving/cli.rs
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
//   - The RMV command-line driving adapter.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute command-line composition.
// - Split-When:
//   - Split when command-line composition contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another rmv module owns the same driving adapter boundary with no
//   - distinct invariant.
// - Summary:
//   - Runs RMV audit and planning commands through the application boundary.
// - Description:
//   - Defines command-line composition data and behavior for rmv root.
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

//! Process-neutral RMV audit command composition.
//!
//! Argument meaning and audit diagnostics remain local to this adapter.
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
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
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
        let roots = input_roots
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<_>>();
        match RunMovieAudit::execute(
            &FilesystemMovieAuditor,
            &TsvAuditManifestSink,
            &roots,
            &output_root,
        ) {
            Ok(report) => report_outcome(&report),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            }
        }
    }
}

/// Renders deterministic audit summary rows in their historical order.
fn report_outcome(report: &crate::domain::AuditReport) -> CommandOutcome {
    let mut outcome = CommandOutcome::success()
        .stdout_line(
            format!(
                "movie inputs: {}",
                report
                    .records
                    .len()
            ),
        )
        .stdout_line(
            format!(
                "unique hashes: {}",
                report.unique_hashes()
            ),
        )
        .stdout_line(
            format!(
                "duplicate inputs: {}",
                report.duplicate_inputs
            ),
        )
        .stdout_line(
            format!(
                "missing bk2 outputs: {}",
                report.missing_bk2_outputs
            ),
        );
    for (kind, count) in report.kind_counts() {
        outcome = outcome.stdout_line(
            format!(
                "  {count} x {}",
                kind.label()
            ),
        );
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
mod tests {
    use schoenwald_cli::{CliProgram, OutputStream};

    use super::{RmvAuditProgram, USAGE, report_outcome};
    use crate::domain::AuditReport;

    #[test]
    fn successful_audit_summaries_use_standard_output() -> Result<(), String> {
        let report = AuditReport::default();
        let outcome = report_outcome(&report);
        if outcome.status() != schoenwald_cli::ExitStatus::Success {
            return Err("successful RMV report returned failure".to_owned());
        }
        if outcome
            .output()
            .is_empty()
        {
            return Err("successful RMV report emitted no summary".to_owned());
        }
        for chunk in outcome.output() {
            if chunk.stream() != OutputStream::Stdout {
                return Err(
                    format!(
                        "successful RMV summary used diagnostic stream: {:?}",
                        chunk.text()
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn invalid_arguments_return_one_usage_diagnostic() -> Result<(), String> {
        for arguments in [
            Vec::new(),
            vec!["output".to_owned()],
            vec![
                String::new(),
                "input".to_owned(),
            ],
            vec![
                "output".to_owned(),
                String::new(),
            ],
        ] {
            let outcome = RmvAuditProgram.execute(&arguments);
            if outcome.status() != schoenwald_cli::ExitStatus::Failure {
                return Err(
                    format!("invalid RMV arguments passed: {arguments:?}"),
                );
            }
            let [chunk] = outcome.output() else {
                return Err("RMV usage must emit one diagnostic".to_owned());
            };
            if chunk.stream() != OutputStream::Stderr {
                return Err("RMV usage must be written to stderr".to_owned());
            }
            if chunk.text() != format!("{USAGE}\n") {
                return Err(
                    format!(
                        "unexpected RMV usage: {:?}",
                        chunk.text()
                    ),
                );
            }
        }
        Ok(())
    }
}
