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
//   - Single cli inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Single cli inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Single cli inbound adapter.

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
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        let [input, output_dir] = arguments else {
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
            },
        }
    }
}

/// Executes the single-package CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&SingleExtractProgram)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-inbound/single_cli/tests.rs"]
mod tests;
