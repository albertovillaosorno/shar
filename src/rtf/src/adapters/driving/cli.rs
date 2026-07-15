// File:
//   - cli.rs
// Path:
//   - src/rtf/src/adapters/driving/cli.rs
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
//   - The RTF command-line driving adapter.
// - Must-Not:
//   - Parse RTF bytes or publish files without explicit ports.
// - Allows:
//   - Decode arguments, select adapters, and present conversion results.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter.
// - Merge-When:
//   - Another driving adapter owns the same CLI request contract.
// - Summary:
//   - Command-line adapter for RTF README conversion.
// - Description:
//   - Composes filesystem adapters around the conversion application command.
// - Usage:
//   - Called by the thin `rtf-to-markdown` executable.
// - Defaults:
//   - The legacy `game/README.rtf` input default remains CLI-local.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Process-neutral RTF README conversion command composition.
//!
//! Defaults and exact stdout-versus-file presentation remain local.
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
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
        if arguments.len() > 2 {
            return CommandOutcome::failure().stderr_line(USAGE);
        }
        let input = arguments
            .first()
            .map_or_else(
                || PathBuf::from("game/README.rtf"),
                PathBuf::from,
            );
        let output = arguments
            .get(1)
            .map(PathBuf::from);
        match run(
            &input,
            output.as_deref(),
        ) {
            Ok(document) => output
                .as_ref()
                .map_or_else(
                    || CommandOutcome::success().stdout(document),
                    |destination| {
                        CommandOutcome::success().stderr_line(
                            format!(
                                "converted {} -> {}",
                                DiagnosticPath::new(&input),
                                DiagnosticPath::new(destination)
                            ),
                        )
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
pub fn run(
    input: &Path,
    output: Option<&Path>,
) -> Result<String, String> {
    let document = ConvertReadme::execute(
        &FileRtfSource,
        input,
    )
    .map_err(|error| error.to_string())?;
    if let Some(destination) = output {
        FileMarkdownSink
            .write(
                destination,
                &document,
            )
            .map_err(
                |error| {
                    format!(
                        "failed to write {}: {error}",
                        DiagnosticPath::new(destination)
                    )
                },
            )?;
    }
    Ok(document)
}

/// Executes the RTF conversion CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&RtfConversionProgram)
}

#[cfg(test)]
mod tests {
    #[cfg(windows)]
    use std::ffi::OsString;
    #[cfg(windows)]
    use std::fs;
    #[cfg(windows)]
    use std::os::windows::ffi::OsStringExt as _;

    use schoenwald_cli::CliProgram;

    use super::{RtfConversionProgram, USAGE, run};

    #[test]
    fn excess_arguments_return_one_usage_diagnostic() -> Result<(), String> {
        let outcome = RtfConversionProgram.execute(
            &[
                "input".to_owned(),
                "output".to_owned(),
                "extra".to_owned(),
            ],
        );
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err("excess RTF arguments must fail".to_owned());
        }
        let [chunk] = outcome.output() else {
            return Err("RTF usage must emit one diagnostic".to_owned());
        };
        if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
            return Err("RTF usage must be written to stderr".to_owned());
        }
        if chunk.text() != format!("{USAGE}\n") {
            return Err(
                format!(
                    "unexpected RTF usage: {:?}",
                    chunk.text()
                ),
            );
        }
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn write_error_preserves_unpaired_utf16_destination_unit()
    -> Result<(), String> {
        let root = std::env::temp_dir().join(
            format!("schoenwald-rtf-diagnostic-{}", std::process::id()),
        );
        fs::create_dir_all(&root).map_err(|error| error.to_string())?;
        let input = root.join("input.rtf");
        fs::write(
            &input,
            br"{\rtf1\ansi hello}",
        )
        .map_err(|error| error.to_string())?;
        let output = root.join(OsString::from_wide(&[
            u16::from(b'a'),
            0xd800,
            u16::from(b'b'),
        ]));

        let result = run(
            &input,
            Some(&output),
        );

        fs::remove_file(&input).map_err(|error| error.to_string())?;
        fs::remove_dir(&root).map_err(|error| error.to_string())?;
        let Err(error) = result else {
            return Err("invalid destination unexpectedly succeeded".to_owned());
        };
        if !error.contains(r"a\u{D800}b") {
            return Err(format!("diagnostic lost native path unit: {error:?}"));
        }
        if error.contains('\u{fffd}') {
            return Err(format!("diagnostic used lossy replacement: {error:?}"));
        }
        Ok(())
    }
}
