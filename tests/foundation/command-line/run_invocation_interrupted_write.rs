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
//   - Run invocation interrupted write test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Run invocation interrupted write test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Run invocation interrupted write test module.

use std::cell::{Cell, RefCell};
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

struct OneChunkProgram;

impl CliProgram for OneChunkProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::success().stdout("complete")
    }
}

#[derive(Default)]
struct SideEffectingInterruptedSink {
    /// Number of attempted output writes.
    calls: Cell<usize>,
    /// Chunks observed before each reported failure.
    chunks: RefCell<Vec<(OutputStream, String)>>,
}

impl OutputSink for SideEffectingInterruptedSink {
    fn write(&mut self, stream: OutputStream, text: &str) -> io::Result<()> {
        let call = self.calls.get();
        self.calls.set(call.saturating_add(1));
        self.chunks.borrow_mut().push((stream, text.to_owned()));
        let kind = if call == 0 {
            io::ErrorKind::Interrupted
        } else {
            io::ErrorKind::BrokenPipe
        };
        Err(io::Error::new(
            kind,
            "presentation failed after a side effect",
        ))
    }
}

#[test]
fn interrupted_opaque_sink_operation_is_not_replayed() {
    let mut arguments = EmptyArguments;
    let mut sink = SideEffectingInterruptedSink::default();

    let result =
        RunInvocation::execute(&OneChunkProgram, &mut arguments, &mut sink);

    assert!(matches!(
        result,
        Err(error) if error.kind() == io::ErrorKind::Interrupted
    ));
    assert_eq!(sink.calls.get(), 1);
    assert_eq!(sink.chunks.borrow().as_slice(), &[(
        OutputStream::Stdout,
        "complete".to_owned()
    )]);
}
