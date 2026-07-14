// File:
//   - single_cli.rs
// Path:
//   - src/p3d/src/adapters/driving/single_cli.rs
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
//   - The single-package Pure3D driving adapter.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Decode one package request and present the application result.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter.
// - Merge-When:
//   - Another p3d module owns the same driving adapter boundary with no
//   - distinct invariant.
// - Summary:
//   - Single `Pure3D` package extraction CLI.
// - Description:
//   - Defines single-package request composition for Pure3D export.
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

//! Single `Pure3D` package extraction CLI.
//!
//! Command policy remains local while shared process mechanics stay
//! centralized.
use std::path::Path;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use crate::adapters::driven::LosslessPackageExporter;
use crate::application::ExportPackage;

/// Exact usage contract for single-package export.
const USAGE: &str = "usage: extract <input.p3d> <output-dir>";

/// Process-neutral single-package export program.
#[derive(Debug, Default, Clone, Copy)]
pub struct SingleExtractProgram;

impl CliProgram for SingleExtractProgram {
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
        let [
            input,
            output_dir,
        ] = arguments
        else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        match ExportPackage::execute(
            &LosslessPackageExporter,
            Path::new(input),
            Path::new(output_dir),
        ) {
            Ok(()) => CommandOutcome::success(),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            }
        }
    }
}

/// Executes the single-package CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&SingleExtractProgram)
}

#[cfg(test)]
mod tests {
    use schoenwald_cli::CliProgram;

    use super::{SingleExtractProgram, USAGE};

    #[test]
    fn invalid_argument_counts_return_single_extract_usage()
    -> Result<(), String> {
        for arguments in [
            Vec::new(),
            vec!["input".to_owned()],
            vec![
                "input".to_owned(),
                "output".to_owned(),
                "extra".to_owned(),
            ],
        ] {
            let outcome = SingleExtractProgram.execute(&arguments);
            if outcome.status() != schoenwald_cli::ExitStatus::Failure {
                return Err(format!("invalid arguments passed: {arguments:?}"));
            }
            let [chunk] = outcome.output() else {
                return Err("single usage must emit one diagnostic".to_owned());
            };
            if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
                return Err(
                    "single usage must be written to stderr".to_owned(),
                );
            }
            if chunk.text() != format!("{USAGE}\n") {
                return Err(
                    format!(
                        "unexpected single usage: {:?}",
                        chunk.text()
                    ),
                );
            }
        }
        Ok(())
    }
}
