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
//   - Run invocation trait objects test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Run invocation trait objects test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Run invocation trait objects test module.

use std::io;

use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, ExitStatus,
    OutputSink, OutputStream, RunInvocation,
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
        CommandOutcome::success()
    }
}

struct AcceptingOutput;

impl OutputSink for AcceptingOutput {
    fn write(&mut self, _stream: OutputStream, _text: &str) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn invocation_accepts_all_ports_as_trait_objects() {
    let command: &dyn CliProgram = &SuccessfulProgram;
    let mut empty_arguments = EmptyArguments;
    let arguments: &mut dyn ArgumentSource = &mut empty_arguments;
    let mut accepting_output = AcceptingOutput;
    let output: &mut dyn OutputSink = &mut accepting_output;

    let result = RunInvocation::execute(command, arguments, output);

    assert!(matches!(result, Ok(ExitStatus::Success)));
}
