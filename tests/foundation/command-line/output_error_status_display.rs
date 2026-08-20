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
//   - Output error status display test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Output error status display test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Output error status display test module.

use std::io;

use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, OutputSink,
    OutputStream, RunInvocation,
};

struct EmptyArguments;

impl ArgumentSource for EmptyArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Ok(Vec::new())
    }
}

struct SuccessfulProgram;

impl CliProgram for SuccessfulProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::success().stderr("diagnostic")
    }
}

struct FailedProgram;

impl CliProgram for FailedProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::failure().stderr("diagnostic")
    }
}

struct DeniedOutput;

impl OutputSink for DeniedOutput {
    fn write(&mut self, _stream: OutputStream, _text: &str) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "denied"))
    }
}

fn render_failure(command: &dyn CliProgram) -> String {
    let mut arguments = EmptyArguments;
    let mut output = DeniedOutput;
    let result = RunInvocation::execute(command, &mut arguments, &mut output);
    let Some(error) = result.err() else {
        return String::new();
    };
    error.to_string()
}

#[test]
fn display_distinguishes_successful_and_failed_commands() {
    let success = render_failure(&SuccessfulProgram);
    let failure = render_failure(&FailedProgram);

    assert_eq!(
        success,
        concat!(
            "failed to write standard error chunk 1 of 1: denied ",
            "[I/O error kind: permission denied] ",
            "(command status: success)"
        )
    );
    assert_eq!(
        failure,
        concat!(
            "failed to write standard error chunk 1 of 1: denied ",
            "[I/O error kind: permission denied] ",
            "(command status: failure)"
        )
    );
}
