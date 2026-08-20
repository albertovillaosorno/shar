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
//   - Command-line translation for LMLM compatibility operations.
// - Must-Not:
//   - Parse archives or publish files outside composition services.
// - Allows:
//   - Translate arguments into batch, inspect, and convert requests.
// - Split-When:
//   - A new inbound protocol requires an independent adapter.
// - Merge-When:
//   - Another inbound adapter owns the identical CLI contract.
// - Summary:
//   - LMLM command-line inbound adapter.
// - Description:
//   - Maps stable CLI commands to the compatibility library facade.
// - Usage:
//   - Included by the process root and CLI integration tests.
// - Defaults:
//   - Unknown argument shapes return the stable usage contract.
//

//! Thin CLI over folder conversion, inspection, and manual conversion.

use std::path::Path;

use schoenwald_cli::{CliProgram, CommandOutcome};
use shar_lmlm::batch::run_default;
use shar_lmlm::convert::{convert, inspect};

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
            [command, input] if command == "inspect" => {
                match inspect(Path::new(input)) {
                    Ok(report) => render(&report),
                    Err(error) => failure(&error),
                }
            },
            [command, input, output] if command == "convert" => {
                match convert(Path::new(input), Path::new(output)) {
                    Ok(report) => render(&report),
                    Err(error) => failure(&error),
                }
            },
            _ => CommandOutcome::failure().stderr_line(USAGE),
        }
    }
}
