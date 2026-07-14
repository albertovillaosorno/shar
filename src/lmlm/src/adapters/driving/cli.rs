// File:
//   - cli.rs
// Path:
//   - src/lmlm/src/adapters/driving/cli.rs
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
//   - The LMLM command-line driving adapter.
// - Must-Not:
//   - Parse archive bytes or materialize payloads directly.
// - Allows:
//   - Decode arguments, select concrete ports, and present results.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter.
// - Merge-When:
//   - Another driving adapter owns the same CLI request contract.
// - Summary:
//   - Command-line adapter for validated LMLM extraction.
// - Description:
//   - Composes filesystem ports around the extraction application command.
// - Usage:
//   - Called by the thin `lmlm-extract` executable.
// - Defaults:
//   - Input and output paths must be supplied explicitly.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Command-line composition for validated LMLM extraction.
//!
//! The adapter owns request policy while shared process mechanics remain in
//! `schoenwald-cli`.
use std::path::Path;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use crate::adapters::driven::{FileArchiveSource, FilesystemEntrySink};
use crate::application::{ExtractArchive, ExtractArchiveError};
use crate::diagnostic::escaped_string;

/// Exact usage contract for LMLM extraction.
const USAGE: &str = "usage: lmlm-extract <INPUT.lmlm> <OUTPUT_DIR>";

/// Renders one successful extraction result.
fn success_outcome(
    count: usize,
    output_root: &str,
) -> CommandOutcome {
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
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
        let [
            input,
            output_root,
        ] = arguments
        else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        match run(
            Path::new(input),
            Path::new(output_root),
        ) {
            Ok(count) => success_outcome(
                count,
                output_root,
            ),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            }
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
mod tests {
    use schoenwald_cli::CliProgram;

    use super::{LmlmExtractProgram, USAGE, success_outcome};

    #[test]
    fn missing_or_excess_arguments_return_the_same_usage_contract()
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
            let outcome = LmlmExtractProgram.execute(&arguments);
            if outcome.status() != schoenwald_cli::ExitStatus::Failure {
                return Err(
                    format!(
                        "invalid arguments unexpectedly passed: {arguments:?}"
                    ),
                );
            }
            let [chunk] = outcome.output() else {
                return Err(
                    "usage failure must emit one diagnostic".to_owned(),
                );
            };
            if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
                return Err("LMLM usage must be written to stderr".to_owned());
            }
            if chunk.text()
                != format!(
                    "{USAGE}
"
                )
            {
                return Err(
                    format!(
                        "unexpected LMLM usage: {:?}",
                        chunk.text()
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn one_file_uses_the_singular_success_noun() {
        let outcome = success_outcome(
            1, "output",
        );
        let rendered = outcome
            .output()
            .first()
            .map(schoenwald_cli::OutputChunk::text);

        assert_eq!(
            rendered,
            Some("extracted 1 file to output\n")
        );
    }

    #[test]
    fn successful_extractions_escape_control_characters_in_output_paths()
    -> Result<(), String> {
        let outcome = success_outcome(
            1,
            "bad\nroot",
        );
        let Some(first) = outcome
            .output()
            .first()
        else {
            return Err(
                "successful extraction emitted no diagnostic".to_owned(),
            );
        };
        let Some(line) = first
            .text()
            .strip_suffix('\n')
        else {
            return Err(
                "successful extraction omitted its line terminator".to_owned(),
            );
        };
        if line.contains('\n') {
            return Err(
                "successful extraction emitted a raw path newline".to_owned(),
            );
        }
        if !line.contains(r"bad\nroot") {
            return Err(
                format!(
                    "successful extraction lost escaped path evidence: {:?}",
                    first.text()
                ),
            );
        }
        Ok(())
    }
}
