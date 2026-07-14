// File:
//   - structural_audit_cli.rs
// Path:
//   - src/game-manifest/src/adapters/driving/structural_audit_cli.rs
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
//   - Structural-audit CLI composition and tabular result presentation.
// - Must-Not:
//   - Traverse trees or count extensions outside the application command.
// - Allows:
//   - Decode one optional game root and print deterministic audit rows.
// - Split-When:
//   - Split when another inbound protocol needs an independent adapter.
// - Merge-When:
//   - Another driving adapter owns the same structural-audit request contract.
// - Summary:
//   - Driving CLI for structural extension audits.
// - Description:
//   - Composes the filesystem tree adapter around the audit use case.
// - Usage:
//   - Called by the thin `ephemeral_structural_audit` executable.
// - Defaults:
//   - The game root defaults to `game`.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! CLI composition for structural extension audits.
//!
//! The adapter presents deterministic tabular rows from the audit use case.
use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use super::support::reject_extra_arguments;
use crate::adapters::driven::FilesystemGameTree;
use crate::application::StructuralAudit;

/// Exact usage contract for the structural audit.
const USAGE: &str = "usage: ephemeral-structural-audit [game-directory]";

/// Process-neutral structural-audit CLI program.
#[derive(Debug, Default, Clone, Copy)]
pub struct StructuralAuditCli;

impl CliProgram for StructuralAuditCli {
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
        match StructuralAudit::execute(
            &FilesystemGameTree,
            &game_dir,
        ) {
            Ok(report) => {
                let mut outcome = CommandOutcome::success().stdout_line(
                    format!(
                        "total_dirty_extensions\t{}",
                        report.total_dirty_extensions
                    ),
                );
                for (extension, count) in report.rows {
                    outcome =
                        outcome.stdout_line(format!("{extension}\t{count}"));
                }
                outcome
            }
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            }
        }
    }
}

/// Executes the structural audit CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&StructuralAuditCli)
}

#[cfg(test)]
mod tests {
    use schoenwald_cli::CliProgram;

    use super::{StructuralAuditCli, USAGE};

    #[test]
    fn excess_arguments_return_audit_usage_without_tree_access()
    -> Result<(), String> {
        let outcome = StructuralAuditCli.execute(
            &[
                "first".to_owned(),
                "second".to_owned(),
            ],
        );
        if outcome.status() != schoenwald_cli::ExitStatus::Failure {
            return Err("excess audit arguments must fail".to_owned());
        }
        let [chunk] = outcome.output() else {
            return Err("audit usage must emit one diagnostic".to_owned());
        };
        if chunk.stream() != schoenwald_cli::OutputStream::Stderr {
            return Err("audit usage must be written to stderr".to_owned());
        }
        if chunk.text() != format!("{USAGE}\n") {
            return Err(
                format!(
                    "unexpected audit usage: {:?}",
                    chunk.text()
                ),
            );
        }
        Ok(())
    }
}
