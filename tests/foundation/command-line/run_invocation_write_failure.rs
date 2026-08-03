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
//   - Run invocation write failure test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Run invocation write failure test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Run invocation write failure test module.

#[path = "support/output_error.rs"]
pub mod support;

use std::cell::{Cell, RefCell};
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

struct TwoChunkProgram;

impl CliProgram for TwoChunkProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::success()
            .stdout("primary")
            .stderr("diagnostic")
    }
}

#[derive(Default)]
struct FailingFirstSink {
    calls: Cell<usize>,
    successful_chunks: RefCell<Vec<(OutputStream, String)>>,
}

impl OutputSink for FailingFirstSink {
    fn write(&mut self, stream: OutputStream, text: &str) -> io::Result<()> {
        let call = self.calls.get();
        self.calls.set(call.saturating_add(1));
        if call == 0 {
            return Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "first write failed",
            ));
        }
        self.successful_chunks
            .borrow_mut()
            .push((stream, text.to_owned()));
        Ok(())
    }
}

#[test]
fn a_failed_chunk_does_not_suppress_later_ordered_output() {
    let mut arguments = EmptyArguments;
    let mut sink = FailingFirstSink::default();

    let result =
        RunInvocation::execute(&TwoChunkProgram, &mut arguments, &mut sink);

    assert!(matches!(
        result,
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe
    ));
    assert_eq!(sink.calls.get(), 2);
    assert_eq!(sink.successful_chunks.borrow().as_slice(), &[(
        OutputStream::Stderr,
        "diagnostic".to_owned()
    )]);
}

struct ThreeChunkProgram;

impl CliProgram for ThreeChunkProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::success()
            .stdout("primary")
            .stderr("diagnostic")
            .stdout("orphaned")
    }
}

#[test]
fn a_failed_stream_suppresses_later_chunks_on_that_stream() {
    let mut arguments = EmptyArguments;
    let mut sink = FailingFirstSink::default();

    let error = output_error(RunInvocation::execute(
        &ThreeChunkProgram,
        &mut arguments,
        &mut sink,
    ));
    assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
    assert_eq!(error.output_chunk_count(), 3);
    assert_eq!(error.presented_chunk_count(), 1);
    assert_eq!(error.suppressed_chunk_count(), 1);
    assert_eq!(sink.calls.get(), 2);
    assert_eq!(sink.successful_chunks.borrow().as_slice(), &[(
        OutputStream::Stderr,
        "diagnostic".to_owned()
    )]);
}
