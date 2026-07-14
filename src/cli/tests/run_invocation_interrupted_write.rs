// File:
//   - run_invocation_interrupted_write.rs
// Path:
//   - src/cli/tests/run_invocation_interrupted_write.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Regression coverage for interrupted output replay.
// - Must-Not:
//   - Access operating-system streams or caller command policy.
// - Allows:
//   - Use a deterministic sink that reports interruption after a side effect.
// - Split-When:
//   - Another opaque-sink retry behavior needs independent coverage.
// - Merge-When:
//   - The application runner no longer owns output presentation.
// - Summary:
//   - Interrupted output replay regression.
// - Description:
//   - Proves the application does not duplicate an opaque sink operation.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The first call records the chunk and reports an interruption.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for interrupted opaque output sinks.
//!
//! Only concrete adapters know whether an interrupted operation is retry-safe.

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
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::success().stdout("complete")
    }
}

#[derive(Default)]
struct SideEffectingInterruptedSink {
    /// Number of attempted output writes.
    calls: Cell<usize>,
    /// Chunks observed before each reported failure.
    chunks: RefCell<
        Vec<(
            OutputStream,
            String,
        )>,
    >,
}

impl OutputSink for SideEffectingInterruptedSink {
    fn write(
        &mut self,
        stream: OutputStream,
        text: &str,
    ) -> io::Result<()> {
        let call = self
            .calls
            .get();
        self.calls
            .set(call.saturating_add(1));
        self.chunks
            .borrow_mut()
            .push(
                (
                    stream,
                    text.to_owned(),
                ),
            );
        let kind = if call == 0 {
            io::ErrorKind::Interrupted
        } else {
            io::ErrorKind::BrokenPipe
        };
        Err(
            io::Error::new(
                kind,
                "presentation failed after a side effect",
            ),
        )
    }
}

#[test]
fn interrupted_opaque_sink_operation_is_not_replayed() {
    let mut arguments = EmptyArguments;
    let mut sink = SideEffectingInterruptedSink::default();

    let result = RunInvocation::execute(
        &OneChunkProgram,
        &mut arguments,
        &mut sink,
    );

    assert!(
        matches!(
            result,
            Err(error) if error.kind() == io::ErrorKind::Interrupted
        )
    );
    assert_eq!(
        sink.calls
            .get(),
        1
    );
    assert_eq!(
        sink.chunks
            .borrow()
            .as_slice(),
        &[
            (
                OutputStream::Stdout,
                "complete".to_owned()
            )
        ]
    );
}
