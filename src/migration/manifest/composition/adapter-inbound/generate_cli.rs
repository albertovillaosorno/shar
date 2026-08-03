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
//   - Generate cli inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Generate cli inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Generate cli inbound adapter.

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
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        if let Some(outcome) = reject_extra_arguments(arguments, 1, USAGE) {
            return outcome;
        }
        let game_dir = arguments
            .first()
            .map_or_else(|| PathBuf::from("game"), PathBuf::from);
        match GenerateManifest::execute(
            &FilesystemGameTree,
            &FilesystemTextStore,
            &game_dir,
        ) {
            Ok(report) => CommandOutcome::success().stdout_line(format!(
                "wrote {} folder/type records ({} files) to {}",
                report.record_count,
                report.total_files,
                report.manifest_path.display()
            )),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            },
        }
    }
}

/// Executes the generator CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&GenerateManifestCli)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/manifest/unit/adapter-inbound/generate_cli/tests.rs"]
mod tests;
