// File:
//   - batch_cli.rs
// Path:
//   - src/p3d/src/adapters/driving/batch_cli.rs
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
//   - p3d module behavior for bin batch extract.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute batch extract.
// - Split-When:
//   - Split when batch extract contains two independently testable contracts.
// - Merge-When:
//   - Another p3d module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Batch `Pure3D` package extraction CLI.
// - Description:
//   - Defines batch extract data and behavior for p3d bin.
// - Usage:
//   - Used by p3d bin code that needs batch extract.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Batch `Pure3D` package extraction CLI.
//!
//! The adapter owns argument meaning and presentation while batch filesystem
//! mechanisms remain behind an outbound port.
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
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
        let Some((output_root, input_roots)) = arguments.split_first() else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        if input_roots.is_empty() {
            return CommandOutcome::failure().stderr_line(USAGE);
        }
        let output_path = PathBuf::from(output_root);
        let input_paths = input_roots
            .iter()
            .map(PathBuf::from)
            .collect::<Vec<_>>();
        match ExportPackageBatch::execute(
            &FilesystemBatchExporter,
            &output_path,
            &input_paths,
        ) {
            Ok(report) => CommandOutcome::success().stdout_line(
                format!(
                    "p3d batch ok: {} scanned, {} skipped, {} extracted, {} \
                     failed",
                    report.scanned,
                    report.skipped,
                    report.extracted,
                    report.failed
                ),
            ),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            }
        }
    }
}

/// Executes the batch CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&BatchExtractProgram)
}

#[cfg(test)]
mod tests {
    use schoenwald_cli::CliProgram;

    use super::{BatchExtractProgram, USAGE};

    #[test]
    fn missing_output_or_input_roots_return_batch_usage() -> Result<(), String>
    {
        for arguments in [
            Vec::new(),
            vec!["output".to_owned()],
        ] {
            let outcome = BatchExtractProgram.execute(&arguments);
            if !outcome.is_failure_with_stderr_line(USAGE) {
                return Err(
                    format!(
                        "invalid batch usage outcome for arguments: \
                         {arguments:?}"
                    ),
                );
            }
        }
        Ok(())
    }
}
