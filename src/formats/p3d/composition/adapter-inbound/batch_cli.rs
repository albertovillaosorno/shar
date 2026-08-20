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
//   - Batch cli inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Batch cli inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Batch cli inbound adapter.

use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use crate::adapters::driven::FilesystemBatchExporter;
use crate::application::ExportPackageBatch;

/// Exact usage contract for batch package export.
const USAGE: &str = "usage: p3d-batch-extract <output-root> <input-root>...";

/// Process-neutral batch package-export program.
#[derive(Debug, Default, Clone, Copy)]
pub struct BatchExtractProgram;

impl CliProgram for BatchExtractProgram {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        let Some((output_root, input_roots)) = arguments.split_first() else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        if input_roots.is_empty() {
            return CommandOutcome::failure().stderr_line(USAGE);
        }
        let output_path = PathBuf::from(output_root);
        let input_paths =
            input_roots.iter().map(PathBuf::from).collect::<Vec<_>>();
        match ExportPackageBatch::execute(
            &FilesystemBatchExporter,
            &output_path,
            &input_paths,
        ) {
            Ok(report) => CommandOutcome::success().stdout_line(format!(
                "p3d batch ok: {} scanned, {} skipped, {} extracted, {} \
                     failed",
                report.scanned, report.skipped, report.extracted, report.failed
            )),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            },
        }
    }
}

/// Executes the batch CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&BatchExtractProgram)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-inbound/batch_cli/tests.rs"]
mod tests;
