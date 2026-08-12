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
//   - Expanded cli inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Expanded cli inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Expanded cli inbound adapter.

use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use super::support::reject_extra_arguments;
use crate::adapters::driven::{FilesystemGameTree, FilesystemTextStore};
use crate::application::GenerateExpandedManifest;
use crate::domain::EXPANDED_MANIFEST_FILE_NAME;

/// Exact usage contract for expanded manifest generation.
const USAGE: &str = concat!(
    "usage: generate-expanded-manifest [game-directory] ",
    "[extracted-rcf-directory] [output-path]",
);

/// Default physical extracted RCF workspace.
const EXTRACTED_RCF_WORKSPACE_ROOT: &str = ".cache/pipeline/extracted/rcf";

/// Process-neutral expanded-manifest CLI program.
#[derive(Debug, Default, Clone, Copy)]
pub struct GenerateExpandedManifestCli;

impl CliProgram for GenerateExpandedManifestCli {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        if let Some(outcome) = reject_extra_arguments(arguments, 3, USAGE) {
            return outcome;
        }
        let game_dir = arguments
            .first()
            .map_or_else(|| PathBuf::from("game"), PathBuf::from);
        let extracted_rcf_dir = arguments
            .get(1)
            .map_or_else(
                || PathBuf::from(EXTRACTED_RCF_WORKSPACE_ROOT),
                PathBuf::from,
            );
        let output_path = arguments.get(2).map_or_else(
            || game_dir.join(EXPANDED_MANIFEST_FILE_NAME),
            PathBuf::from,
        );
        match GenerateExpandedManifest::execute(
            &FilesystemGameTree,
            &FilesystemTextStore,
            &game_dir,
            &extracted_rcf_dir,
            &output_path,
        ) {
            Ok(report) => CommandOutcome::success().stdout_line(format!(
                "wrote {} expanded records to {}",
                report.record_count,
                report.output_path.display()
            )),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            },
        }
    }
}

/// Executes the expanded generator CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&GenerateExpandedManifestCli)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/manifest/unit/adapter-inbound/expanded_cli/tests.rs"]
mod tests;
