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
//   - Implements the declared inbound adapter responsibility for rtf.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Cli inbound adapter.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};
use schoenwald_filesystem::DiagnosticPath;

use crate::adapters::driven::{FileMarkdownSink, FileRtfSource};
use crate::application::ConvertReadme;
use crate::ports::MarkdownSink as _;

/// Exact usage contract for RTF conversion.
const USAGE: &str = "usage: rtf-to-markdown [INPUT.rtf] [OUTPUT.md]";

/// Process-neutral RTF conversion program.
#[derive(Debug, Default, Clone, Copy)]
pub struct RtfConversionProgram;

impl CliProgram for RtfConversionProgram {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        if arguments.len() > 2 {
            return CommandOutcome::failure().stderr_line(USAGE);
        }
        let input = arguments
            .first()
            .map_or_else(|| PathBuf::from("game/README.rtf"), PathBuf::from);
        let output = arguments.get(1).map(PathBuf::from);
        match run(&input, output.as_deref()) {
            Ok(document) => output.as_ref().map_or_else(
                || CommandOutcome::success().stdout(document),
                |destination| {
                    CommandOutcome::success().stderr_line(format!(
                        "converted {} -> {}",
                        DiagnosticPath::new(&input),
                        DiagnosticPath::new(destination)
                    ))
                },
            ),
            Err(error) => CommandOutcome::failure().stderr_line(error),
        }
    }
}

/// Converts one explicit input and optionally publishes it to a file.
///
/// # Errors
///
/// Returns a contextual read or write failure.
pub fn run(input: &Path, output: Option<&Path>) -> Result<String, String> {
    let document = ConvertReadme::execute(&FileRtfSource, input)
        .map_err(|error| error.to_string())?;
    if let Some(destination) = output {
        FileMarkdownSink
            .write(destination, &document)
            .map_err(|error| {
                format!(
                    "failed to write {}: {error}",
                    DiagnosticPath::new(destination)
                )
            })?;
    }
    Ok(document)
}

/// Executes the RTF conversion CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&RtfConversionProgram)
}

#[cfg(test)]
#[path = "../../../../../tests/formats/rtf/unit/adapter-inbound/cli/tests.rs"]
mod tests;
