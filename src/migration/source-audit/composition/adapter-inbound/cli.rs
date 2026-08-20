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
//   - Process-neutral CLI adaptation for deep source validation.
// - Must-Not:
//   - Own source-audit domain policy or expose selected private paths.
// - Allows:
//   - Map command arguments and audit outcomes to process exit evidence.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Deep source validation CLI adapter.
// - Description:
//   - Process-neutral CLI adaptation for deep source validation.
// - Usage:
//   - Used through the owning source-audit function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! CLI for read-only deep source validation.

use std::path::PathBuf;
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use crate::application::DeepSourceAudit;

const USAGE: &str = "usage: validate-source-deep [game-directory]";

/// Process-neutral deep source validation CLI.
#[derive(Debug, Default, Clone, Copy)]
pub struct DeepSourceCli;

impl CliProgram for DeepSourceCli {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        if arguments.len() > 1 {
            return CommandOutcome::failure().stderr_line(USAGE);
        }
        let source_root = arguments
            .first()
            .map_or_else(|| PathBuf::from("game"), PathBuf::from);
        match DeepSourceAudit::execute(&source_root) {
            Ok(report) => CommandOutcome::success().stdout_line(format!(
                "deep-source\tfiles={}\tp3d={}\trcf={}\trsd={}\trmv={}",
                report.files, report.p3d, report.rcf, report.rsd, report.rmv
            )),
            Err(error) => {
                CommandOutcome::failure().stderr_line(error.to_string())
            },
        }
    }
}

/// Runs deep source validation in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&DeepSourceCli)
}
