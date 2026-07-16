// File:
//   - run_invocation_write_failure.rs
// Path:
//   - src/cli/tests/run_invocation_write_failure.rs
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
//   - Regression coverage for ordered presentation after one sink failure.
// - Must-Not:
//   - Exercise operating-system streams or caller command policy.
// - Allows:
//   - Use deterministic in-memory ports.
// - Split-When:
//   - Another output-failure behavior needs independent coverage.
// - Merge-When:
//   - The application runner no longer owns ordered presentation.
// - Summary:
//   - Run-invocation write-failure regression.
// - Description:
//   - Proves a failed chunk does not suppress later ordered diagnostics.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The first write fails and the second write remains observable.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for ordered presentation after an output failure.
//!
//! A failed chunk must not suppress later chunks that can still be written.

#[path = "support/output_error.rs"]
mod support;

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
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::success()
            .stdout("primary")
            .stderr("diagnostic")
    }
}

#[derive(Default)]
struct FailingFirstSink {
    calls: Cell<usize>,
    successful_chunks: RefCell<
        Vec<(
            OutputStream,
            String,
        )>,
    >,
}

impl OutputSink for FailingFirstSink {
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
        if call == 0 {
            return Err(
                io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "first write failed",
                ),
            );
        }
        self.successful_chunks
            .borrow_mut()
            .push(
                (
                    stream,
                    text.to_owned(),
                ),
            );
        Ok(())
    }
}

#[test]
fn a_failed_chunk_does_not_suppress_later_ordered_output() {
    let mut arguments = EmptyArguments;
    let mut sink = FailingFirstSink::default();

    let result = RunInvocation::execute(
        &TwoChunkProgram,
        &mut arguments,
        &mut sink,
    );

    assert!(
        matches!(
            result,
            Err(error) if error.kind() == io::ErrorKind::BrokenPipe
        )
    );
    assert_eq!(
        sink.calls
            .get(),
        2
    );
    assert_eq!(
        sink.successful_chunks
            .borrow()
            .as_slice(),
        &[
            (
                OutputStream::Stderr,
                "diagnostic".to_owned()
            )
        ]
    );
}

struct ThreeChunkProgram;

impl CliProgram for ThreeChunkProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
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

    let error = output_error(
        RunInvocation::execute(
            &ThreeChunkProgram,
            &mut arguments,
            &mut sink,
        ),
    );
    assert_eq!(
        error.kind(),
        io::ErrorKind::BrokenPipe
    );
    assert_eq!(
        error.output_chunk_count(),
        3
    );
    assert_eq!(
        error.presented_chunk_count(),
        1
    );
    assert_eq!(
        error.suppressed_chunk_count(),
        1
    );
    assert_eq!(
        sink.calls
            .get(),
        2
    );
    assert_eq!(
        sink.successful_chunks
            .borrow()
            .as_slice(),
        &[
            (
                OutputStream::Stderr,
                "diagnostic".to_owned()
            )
        ]
    );
}
