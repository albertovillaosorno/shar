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
//   - Output error argument origin test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Output error argument origin test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Output error argument origin test module.

use std::io;

use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, OutputSink,
    OutputStream, RunInvocation,
};

const ARGUMENT_DIAGNOSTIC: &str = "command argument 3 is not valid Unicode";

struct InvalidArguments;

impl ArgumentSource for InvalidArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Err(ArgumentError::non_unicode(2))
    }
}

struct EmptyArguments;

impl ArgumentSource for EmptyArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Ok(Vec::new())
    }
}

struct MatchingDiagnosticProgram;

impl CliProgram for MatchingDiagnosticProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::failure().stderr_line(ARGUMENT_DIAGNOSTIC)
    }
}

struct UnusedProgram;

impl CliProgram for UnusedProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::success()
    }
}

struct DeniedOutput;

impl OutputSink for DeniedOutput {
    fn write(&mut self, _stream: OutputStream, _text: &str) -> io::Result<()> {
        Err(io::Error::new(io::ErrorKind::PermissionDenied, "denied"))
    }
}

fn render_argument_failure() -> String {
    let mut arguments = InvalidArguments;
    let mut output = DeniedOutput;
    let result =
        RunInvocation::execute(&UnusedProgram, &mut arguments, &mut output);
    let Some(error) = result.err() else {
        return String::new();
    };
    error.to_string()
}

fn render_matching_command_failure() -> String {
    let mut arguments = EmptyArguments;
    let mut output = DeniedOutput;
    let result = RunInvocation::execute(
        &MatchingDiagnosticProgram,
        &mut arguments,
        &mut output,
    );
    let Some(error) = result.err() else {
        return String::new();
    };
    error.to_string()
}

#[test]
fn output_error_distinguishes_argument_acquisition_failure() {
    let argument_failure = render_argument_failure();
    let command_failure = render_matching_command_failure();

    assert_eq!(
        argument_failure,
        concat!(
            "failed to write standard error chunk 1 of 1: denied ",
            "[I/O error kind: permission denied] ",
            "(command status: failure; argument acquisition error: ",
            "command argument 3 is not valid Unicode)"
        )
    );
    assert_eq!(
        command_failure,
        concat!(
            "failed to write standard error chunk 1 of 1: denied ",
            "[I/O error kind: permission denied] ",
            "(command status: failure)"
        )
    );
}
