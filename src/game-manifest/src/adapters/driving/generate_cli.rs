// File:
//   - generate_cli.rs
// Path:
//   - src/game-manifest/src/adapters/driving/generate_cli.rs
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
//   - Minimum-manifest CLI request composition and result presentation.
// - Must-Not:
//   - Traverse trees or write artifacts outside the application command.
// - Allows:
//   - Decode one optional game root and select filesystem adapters.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter.
// - Merge-When:
//   - Another driving adapter owns the same generator request contract.
// - Summary:
//   - Driving CLI for minimum manifest generation.
// - Description:
//   - Composes filesystem ports around the generation use case.
// - Usage:
//   - Called by the thin `generate-manifest` executable.
// - Defaults:
//   - The game root defaults to `game`.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! CLI composition for minimum manifest generation.
//!
//! The adapter owns command policy while shared process mechanics remain in
//! `schoenwald-cli`.
use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use super::support::reject_extra_arguments;
use crate::adapters::driven::{FilesystemGameTree, FilesystemTextStore};
use crate::application::GenerateManifest;

/// Exact usage contract for minimum manifest generation.
const USAGE: &str = "usage: generate-manifest [game-directory]";

/// Process-neutral minimum-manifest CLI program.
#[derive(Debug, Default, Clone, Copy)]
pub struct GenerateManifestCli;

impl CliProgram for GenerateManifestCli {
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
        if let Some(outcome) = reject_extra_arguments(
            arguments, 1, USAGE,
        ) {
            return outcome;
        }
        let game_dir = arguments
            .first()
            .map_or_else(
                || PathBuf::from("game"),
                PathBuf::from,
            );
        match GenerateManifest::execute(
            &FilesystemGameTree,
            &FilesystemTextStore,
            &game_dir,
        ) {
            Ok(report) => CommandOutcome::success().stdout_line(
                format!(
                    "wrote {} folder/type records ({} files) to {}",
                    report.record_count,
                    report.total_files,
                    report
                        .manifest_path
                        .display()
                ),
            ),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            }
        }
    }
}

/// Executes the generator CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&GenerateManifestCli)
}

#[cfg(test)]
mod tests {
    use schoenwald_cli::CliProgram;

    use super::{GenerateManifestCli, USAGE};

    #[test]
    fn excess_arguments_return_usage_without_touching_storage()
    -> Result<(), String> {
        let outcome = GenerateManifestCli.execute(
            &[
                "first".to_owned(),
                "second".to_owned(),
            ],
        );
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err("excess arguments must fail".to_owned());
        }
        let [chunk] = outcome.output() else {
            return Err("usage failure must emit one diagnostic".to_owned());
        };
        if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
            return Err("usage must be written to stderr".to_owned());
        }
        if chunk.text() != format!("{USAGE}\n") {
            return Err(
                format!(
                    "unexpected usage text: {:?}",
                    chunk.text()
                ),
            );
        }
        Ok(())
    }
}
