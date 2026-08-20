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
//   - Mutable output sink test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Mutable output sink test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Mutable output sink test module.

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

struct OneChunkProgram;

impl CliProgram for OneChunkProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::success().stdout("direct")
    }
}

#[derive(Default)]
struct VecOutput {
    /// Exact chunks accepted by the sink.
    chunks: Vec<(OutputStream, String)>,
}

impl OutputSink for VecOutput {
    fn write(&mut self, stream: OutputStream, text: &str) -> io::Result<()> {
        self.chunks.push((stream, text.to_owned()));
        Ok(())
    }
}

#[test]
fn invocation_accepts_a_directly_mutable_output_sink() {
    let mut arguments = EmptyArguments;
    let mut output = VecOutput::default();

    let result =
        RunInvocation::execute(&OneChunkProgram, &mut arguments, &mut output);

    assert!(matches!(result, Ok(ExitStatus::Success)));
    assert_eq!(output.chunks, [(OutputStream::Stdout, "direct".to_owned())]);
}
