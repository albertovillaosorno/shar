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
//   - Structural audit cli inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Structural audit cli inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Structural audit cli inbound adapter.

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
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        if let Some(outcome) = reject_extra_arguments(arguments, 1, USAGE) {
            return outcome;
        }
        let game_dir = arguments
            .first()
            .map_or_else(|| PathBuf::from("game"), PathBuf::from);
        match StructuralAudit::execute(&FilesystemGameTree, &game_dir) {
            Ok(report) => {
                let mut outcome =
                    CommandOutcome::success().stdout_line(format!(
                        "total_dirty_extensions\t{}",
                        report.total_dirty_extensions
                    ));
                for (extension, count) in report.rows {
                    outcome =
                        outcome.stdout_line(format!("{extension}\t{count}"));
                }
                outcome
            },
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            },
        }
    }
}

/// Executes the structural audit CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&StructuralAuditCli)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/manifest/unit/adapter-inbound/structural_audit_cli/tests.rs"]
mod tests;
