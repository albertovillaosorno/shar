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
//   - Failed stream empty chunk test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Failed stream empty chunk test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Failed stream empty chunk test module.

#[path = "support/output_error.rs"]
pub mod support;

use std::io;

use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, OutputSink,
    OutputStream, RunInvocation,
};
use support::output_error;

struct EmptyArguments;

impl ArgumentSource for EmptyArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Ok(Vec::new())
    }
}

struct EmptyAfterFailureProgram;

impl CliProgram for EmptyAfterFailureProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::failure()
            .stdout("primary")
            .stdout("")
            .stderr("diagnostic")
    }
}

struct DenyStandardOutput;

impl OutputSink for DenyStandardOutput {
    fn write(&mut self, stream: OutputStream, _text: &str) -> io::Result<()> {
        match stream {
            OutputStream::Stdout => {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "blocked"))
            },
            OutputStream::Stderr => Ok(()),
        }
    }
}

#[test]
fn empty_chunk_after_failed_stream_is_suppressed() {
    let mut arguments = EmptyArguments;
    let mut output = DenyStandardOutput;

    let error = output_error(RunInvocation::execute(
        &EmptyAfterFailureProgram,
        &mut arguments,
        &mut output,
    ));
    assert_eq!(error.output_chunk_count(), 3);
    assert_eq!(error.presented_chunk_count(), 1);
    assert_eq!(error.suppressed_chunk_count(), 1);
}
