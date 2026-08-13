//! Thin CLI over inspection and conversion.

use std::path::Path;

use schoenwald_cli::{CliProgram, CommandOutcome};

use crate::convert::{convert, inspect};

const USAGE: &str =
    "usage: shar-lmlm <inspect INPUT.lmlm | convert INPUT.lmlm OUTPUT_DIR>";

/// Process-neutral LMLM compatibility CLI.
#[derive(Debug, Default, Clone, Copy)]
pub struct LmlmProgram;

impl CliProgram for LmlmProgram {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        let result = match arguments {
            [command, input] if command == "inspect" => inspect(Path::new(input)),
            [command, input, output] if command == "convert" => {
                convert(Path::new(input), Path::new(output))
            },
            _ => return CommandOutcome::failure().stderr_line(USAGE),
        };
        match result {
            Ok(report) => match serde_json::to_string(&report) {
                Ok(json) => CommandOutcome::success().stdout_line(json),
                Err(error) => CommandOutcome::failure()
                    .stderr_line(format!("shar-lmlm: {error}")),
            },
            Err(error) => CommandOutcome::failure()
                .stderr_line(format!("shar-lmlm: {error}")),
        }
    }
}
