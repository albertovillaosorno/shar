// File:
//   - expanded_cli.rs
// Path:
//   - src/game-manifest/src/adapters/driving/expanded_cli.rs
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
//   - Expanded-ledger CLI request composition and result presentation.
// - Must-Not:
//   - Classify files or publish artifacts outside the application command.
// - Allows:
//   - Decode optional roots and output path, then select filesystem adapters.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter.
// - Merge-When:
//   - Another driving adapter owns the same expanded-ledger request contract.
// - Summary:
//   - Driving CLI for expanded manifest generation.
// - Description:
//   - Composes filesystem ports around the expanded generation use case.
// - Usage:
//   - Called by the thin `generate-expanded-manifest` executable.
// - Defaults:
//   - Game and extracted RCF roots retain their legacy defaults.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! CLI composition for expanded manifest generation.
//!
//! Argument defaults and result presentation remain at this inbound boundary.
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

/// Process-neutral expanded-manifest CLI program.
#[derive(Debug, Default, Clone, Copy)]
pub struct GenerateExpandedManifestCli;

impl CliProgram for GenerateExpandedManifestCli {
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
        if let Some(outcome) = reject_extra_arguments(
            arguments, 3, USAGE,
        ) {
            return outcome;
        }
        let game_dir = arguments
            .first()
            .map_or_else(
                || PathBuf::from("game"),
                PathBuf::from,
            );
        let extracted_rcf_dir = arguments
            .get(1)
            .map_or_else(
                || PathBuf::from("extracted/rcf"),
                PathBuf::from,
            );
        let output_path = arguments
            .get(2)
            .map_or_else(
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
            Ok(report) => CommandOutcome::success().stdout_line(
                format!(
                    "wrote {} expanded records to {}",
                    report.record_count,
                    report
                        .output_path
                        .display()
                ),
            ),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            }
        }
    }
}

/// Executes the expanded generator CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&GenerateExpandedManifestCli)
}

#[cfg(test)]
mod tests {
    use schoenwald_cli::CliProgram;

    use super::{GenerateExpandedManifestCli, USAGE};

    #[test]
    fn excess_arguments_return_expanded_usage_without_storage_access()
    -> Result<(), String> {
        let arguments = [
            "game".to_owned(),
            "extracted".to_owned(),
            "output".to_owned(),
            "extra".to_owned(),
        ];
        let outcome = GenerateExpandedManifestCli.execute(&arguments);
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err("excess expanded arguments must fail".to_owned());
        }
        let [chunk] = outcome.output() else {
            return Err("expanded usage must emit one diagnostic".to_owned());
        };
        if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
            return Err("expanded usage must be written to stderr".to_owned());
        }
        if chunk.text() != format!("{USAGE}\n") {
            return Err(
                format!(
                    "unexpected expanded usage: {:?}",
                    chunk.text()
                ),
            );
        }
        Ok(())
    }
}
