// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Mutable argument source test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Mutable argument source test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Mutable argument source test module.

use std::io;

use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, ExitStatus,
    OutputSink, OutputStream, RunInvocation,
};

struct OneShotArguments {
    /// Arguments moved into the invocation on the first read.
    values: Vec<String>,
}

impl ArgumentSource for OneShotArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Ok(std::mem::take(&mut self.values))
    }
}

struct EchoProgram;

impl CliProgram for EchoProgram {
    fn execute(&self, arguments: &[String]) -> CommandOutcome {
        CommandOutcome::success().stdout(arguments.join("|"))
    }
}

#[derive(Default)]
struct VecOutput {
    /// Exact text accepted by the sink.
    text: String,
}

impl OutputSink for VecOutput {
    fn write(&mut self, _stream: OutputStream, text: &str) -> io::Result<()> {
        self.text.push_str(text);
        Ok(())
    }
}

#[test]
fn invocation_accepts_a_consuming_argument_source() {
    let mut arguments = OneShotArguments {
        values: vec!["alpha".to_owned()],
    };
    let mut output = VecOutput::default();

    let result =
        RunInvocation::execute(&EchoProgram, &mut arguments, &mut output);

    assert!(matches!(result, Ok(ExitStatus::Success)));
    assert!(arguments.values.is_empty());
    assert_eq!(output.text, "alpha");
}
