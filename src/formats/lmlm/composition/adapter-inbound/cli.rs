// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
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
//   - Implements the declared inbound adapter responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Cli inbound adapter.

use std::path::Path;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use crate::adapters::driven::{FileArchiveSource, FilesystemEntrySink};
use crate::application::{ExtractArchive, ExtractArchiveError};
use crate::domain::diagnostic::escaped_string;

/// Exact usage contract for LMLM extraction.
const USAGE: &str = "usage: lmlm-extract <INPUT.lmlm> <OUTPUT_DIR>";

/// Renders one successful extraction result.
fn success_outcome(count: usize, output_root: &str) -> CommandOutcome {
    let noun = if count == 1 {
        "file"
    } else {
        "files"
    };
    let escaped_output_root = escaped_string(output_root);
    let message = format!("extracted {count} {noun} to {escaped_output_root}");
    CommandOutcome::success().stderr_line(message)
}

/// Process-neutral LMLM extraction CLI program.
#[derive(Debug, Default, Clone, Copy)]
pub struct LmlmExtractProgram;

impl CliProgram for LmlmExtractProgram {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        let [input, output_root] = arguments else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        match run(Path::new(input), Path::new(output_root)) {
            Ok(count) => success_outcome(count, output_root),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            },
        }
    }
}

/// Executes one explicit extraction request.
///
/// # Errors
///
/// Returns a contextual application failure.
pub fn run(
    input: &Path,
    output_root: &Path,
) -> Result<usize, ExtractArchiveError> {
    ExtractArchive::execute(
        &FileArchiveSource,
        &FilesystemEntrySink,
        input,
        output_root,
    )
}

/// Executes the CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&LmlmExtractProgram)
}

#[cfg(test)]
#[path = "../../../../../tests/formats/lmlm/unit/adapter-inbound/cli/tests.rs"]
mod tests;
