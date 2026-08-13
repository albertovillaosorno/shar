//! Thin CLI over folder conversion, inspection, and manual conversion.

use std::path::Path;

use schoenwald_cli::{CliProgram, CommandOutcome};

use crate::batch::run_default;
use crate::convert::{convert, inspect};

const USAGE: &str = concat!(
    "usage: shar-lmlm [batch] | inspect INPUT.lmlm | ",
    "convert INPUT.lmlm OUTPUT_DIR"
);

/// Process-neutral LMLM compatibility CLI.
#[derive(Debug, Default, Clone, Copy)]
pub struct LmlmProgram;

fn render(value: &impl serde::Serialize) -> CommandOutcome {
    match serde_json::to_string(value) {
        Ok(json) => CommandOutcome::success().stdout_line(json),
        Err(error) => failure(&error),
    }
}

fn failure(error: &impl std::fmt::Display) -> CommandOutcome {
    CommandOutcome::failure().stderr_line(format!("shar-lmlm: {error}"))
}

impl CliProgram for LmlmProgram {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        match arguments {
            [] => match run_default() {
                Ok(report) => render(&report),
                Err(error) => failure(&error),
            },
            [command] if command == "batch" => match run_default() {
                Ok(report) => render(&report),
                Err(error) => failure(&error),
            },
            [command, input] if command == "inspect" => match inspect(Path::new(input)) {
                Ok(report) => render(&report),
                Err(error) => failure(&error),
            },
            [command, input, output] if command == "convert" => {
                match convert(Path::new(input), Path::new(output)) {
                    Ok(report) => render(&report),
                    Err(error) => failure(&error),
                }
            }
            _ => CommandOutcome::failure().stderr_line(USAGE),
        }
    }
}
