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
//   - Validate cli inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Validate cli inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Validate cli inbound adapter.

use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use super::support::reject_extra_arguments;
use crate::adapters::driven::{FilesystemGameTree, FilesystemTextStore};
use crate::application::ValidateManifest;
use crate::domain::MANIFEST_FILE_NAME;

/// Exact usage contract for manifest validation.
const USAGE: &str = "usage: validate-game [game-directory] [manifest-path]";

/// Process-neutral minimum-manifest validation CLI program.
#[derive(Debug, Default, Clone, Copy)]
pub struct ValidateManifestCli;

impl CliProgram for ValidateManifestCli {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        if let Some(outcome) = reject_extra_arguments(arguments, 2, USAGE) {
            return outcome;
        }
        let game_dir = arguments
            .first()
            .map_or_else(|| PathBuf::from("game"), PathBuf::from);
        let manifest_path = arguments
            .get(1)
            .map_or_else(|| game_dir.join(MANIFEST_FILE_NAME), PathBuf::from);
        match ValidateManifest::execute_with_manifest(
            &FilesystemGameTree,
            &FilesystemTextStore,
            &game_dir,
            &manifest_path,
        ) {
            Ok(report) if report.shortfalls.is_empty() => {
                CommandOutcome::success().stdout_line(format!(
                    "game manifest ok: all {} folder/type minimums met in \
                         {}",
                    report.required_records,
                    game_dir.display()
                ))
            },
            Ok(report) => {
                let mut outcome =
                    CommandOutcome::failure().stderr_line(format!(
                        "game manifest FAILED: {} of {} folder/type records \
                         below minimum in {}",
                        report.shortfalls.len(),
                        report.required_records,
                        game_dir.display()
                    ));
                for shortfall in report.shortfalls {
                    outcome = outcome.stderr_line(shortfall);
                }
                outcome
            },
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
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/manifest/unit/adapter-inbound/validate_cli/tests.rs"]
mod tests;
