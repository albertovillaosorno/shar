// File:
//   - validate_cli.rs
// Path:
//   - src/game-manifest/src/adapters/driving/validate_cli.rs
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
//   - Minimum-manifest validation CLI composition and presentation.
// - Must-Not:
//   - Parse manifest rows or traverse trees outside the application command.
// - Allows:
//   - Decode one optional game root and present validation shortfalls.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter.
// - Merge-When:
//   - Another driving adapter owns the same validator request contract.
// - Summary:
//   - Driving CLI for minimum manifest validation.
// - Description:
//   - Composes filesystem ports around the validation use case.
// - Usage:
//   - Called by the thin `validate-game` executable.
// - Defaults:
//   - The game root defaults to `game`.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! CLI composition for minimum manifest validation.
//!
//! The adapter presents application shortfalls without owning validation rules.
use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use super::support::reject_extra_arguments;
use crate::adapters::driven::{FilesystemGameTree, FilesystemTextStore};
use crate::application::ValidateManifest;

/// Exact usage contract for manifest validation.
const USAGE: &str = "usage: validate-game [game-directory]";

/// Process-neutral minimum-manifest validation CLI program.
#[derive(Debug, Default, Clone, Copy)]
pub struct ValidateManifestCli;

impl CliProgram for ValidateManifestCli {
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
        match ValidateManifest::execute(
            &FilesystemGameTree,
            &FilesystemTextStore,
            &game_dir,
        ) {
            Ok(report)
                if report
                    .shortfalls
                    .is_empty() =>
            {
                CommandOutcome::success().stdout_line(
                    format!(
                        "game manifest ok: all {} folder/type minimums met in \
                         {}",
                        report.required_records,
                        game_dir.display()
                    ),
                )
            }
            Ok(report) => {
                let mut outcome = CommandOutcome::failure().stderr_line(
                    format!(
                        "game manifest FAILED: {} of {} folder/type records \
                         below minimum in {}",
                        report
                            .shortfalls
                            .len(),
                        report.required_records,
                        game_dir.display()
                    ),
                );
                for shortfall in report.shortfalls {
                    outcome = outcome.stderr_line(shortfall);
                }
                outcome
            }
            Err(error) => CommandOutcome::failure()
                .stderr_line(format!("game manifest FAILED: {error}")),
        }
    }
}

/// Executes the validator CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&ValidateManifestCli)
}

#[cfg(test)]
mod tests {
    use schoenwald_cli::CliProgram;

    use super::{USAGE, ValidateManifestCli};

    #[test]
    fn excess_arguments_return_validator_usage_without_storage_access()
    -> Result<(), String> {
        let outcome = ValidateManifestCli.execute(
            &[
                "first".to_owned(),
                "second".to_owned(),
            ],
        );
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err("excess validator arguments must fail".to_owned());
        }
        let [chunk] = outcome.output() else {
            return Err("validator usage must emit one diagnostic".to_owned());
        };
        if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
            return Err("validator usage must be written to stderr".to_owned());
        }
        if chunk.text() != format!("{USAGE}\n") {
            return Err(
                format!(
                    "unexpected validator usage: {:?}",
                    chunk.text()
                ),
            );
        }
        Ok(())
    }
}
