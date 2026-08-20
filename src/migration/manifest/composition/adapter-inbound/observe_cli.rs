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
//   - Observed manifest CLI inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Observed manifest CLI inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Observed manifest CLI inbound adapter.

use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use super::support::reject_extra_arguments;
use crate::adapters::driven::FilesystemGameTree;
use crate::application::ObserveManifest;

/// Exact usage contract for read-only source count observation.
const USAGE: &str = "usage: observe-manifest-counts [game-directory]";

/// Process-neutral read-only source count CLI.
#[derive(Debug, Default, Clone, Copy)]
pub struct ObserveManifestCli;

impl CliProgram for ObserveManifestCli {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        if let Some(outcome) = reject_extra_arguments(arguments, 1, USAGE) {
            return outcome;
        }
        let game_dir = arguments
            .first()
            .map_or_else(|| PathBuf::from("game"), PathBuf::from);
        match ObserveManifest::execute(&FilesystemGameTree, &game_dir) {
            Ok(report) => report
                .rows
                .into_iter()
                .fold(CommandOutcome::success(), |outcome, row| {
                    outcome.stdout_line(row.to_observation_jsonl())
                }),
            Err(_error) => CommandOutcome::failure()
                .stderr_line("source count observation failed"),
        }
    }
}

/// Executes the read-only source count CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&ObserveManifestCli)
}
