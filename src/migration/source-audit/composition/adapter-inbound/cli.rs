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
            Err(error) => CommandOutcome::failure().stderr_line(error.to_string()),
        }
    }
}

/// Runs deep source validation in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&DeepSourceCli)
}
