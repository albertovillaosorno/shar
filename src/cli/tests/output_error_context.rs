// File:
//   - output_error_context.rs
// Path:
//   - src/cli/tests/output_error_context.rs
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
//   - Regression coverage for failed-output stream provenance.
// - Must-Not:
//   - Access operating-system streams or caller domain policy.
// - Allows:
//   - Fail one deterministic sink write and inspect public error context.
// - Split-When:
//   - Another invocation error family needs independent coverage.
// - Merge-When:
//   - The application runner no longer reports output failures.
// - Summary:
//   - Output error context regression.
// - Description:
//   - Proves output failures identify their destination stream and I/O kind.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The standard-error write fails with permission denied.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for output-failure provenance.
//!
//! Callers must distinguish result-channel and diagnostic-channel failures.

#[path = "support/failing_write_sink.rs"]
pub mod failing_write_sink;
#[path = "support/output_error.rs"]
pub mod support;

use std::io;

use failing_write_sink::FailingWriteSink;
use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, ExitStatus,
    OutputSink, OutputStream, RunInvocation,
};
use support::output_error;

struct EmptyArguments;

impl ArgumentSource for EmptyArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Ok(Vec::new())
    }
}

struct DiagnosticProgram;

impl CliProgram for DiagnosticProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure().stderr("problem")
    }
}

struct DeniedOutput;

impl OutputSink for DeniedOutput {
    fn write(
        &mut self,
        _stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        Err(
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                "denied",
            ),
        )
    }
}

#[test]
fn output_error_identifies_the_failed_stream() {
    let mut arguments = EmptyArguments;
    let mut output = DeniedOutput;

    let error = output_error(
        RunInvocation::execute(
            &DiagnosticProgram,
            &mut arguments,
            &mut output,
        ),
    );
    assert_eq!(
        error.status(),
        ExitStatus::Failure
    );
    assert_eq!(
        error.stream(),
        OutputStream::Stderr
    );
    assert_eq!(
        error.kind(),
        io::ErrorKind::PermissionDenied
    );
    assert_eq!(
        error
            .io_error()
            .to_string(),
        "denied"
    );
    assert_eq!(
        error.to_string(),
        concat!(
            "failed to write standard error chunk 1 of 1: denied ",
            "[I/O error kind: permission denied] ",
            "(command status: failure)"
        )
    );
}

struct BothStreamsProgram;

impl CliProgram for BothStreamsProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure()
            .stdout("result")
            .stderr("diagnostic")
    }
}

struct BothStreamsDenied;

impl OutputSink for BothStreamsDenied {
    fn write(
        &mut self,
        stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        let kind = match stream {
            OutputStream::Stdout => io::ErrorKind::BrokenPipe,
            OutputStream::Stderr => io::ErrorKind::PermissionDenied,
        };
        Err(
            io::Error::new(
                kind,
                "stream unavailable",
            ),
        )
    }
}

#[test]
fn output_error_preserves_the_second_failed_stream() {
    let mut arguments = EmptyArguments;
    let mut output = BothStreamsDenied;

    let error = output_error(
        RunInvocation::execute(
            &BothStreamsProgram,
            &mut arguments,
            &mut output,
        ),
    );
    assert_eq!(
        error.stream(),
        OutputStream::Stdout
    );
    assert_eq!(
        error.secondary_stream(),
        Some(OutputStream::Stderr)
    );
    assert_eq!(
        error.secondary_chunk_index(),
        Some(1)
    );
    assert_eq!(
        error
            .secondary_io_error()
            .map(io::Error::kind),
        Some(io::ErrorKind::PermissionDenied)
    );
    let expected = concat!(
        "failed to write standard output chunk 1 of 2: stream unavailable ",
        "[I/O error kind: broken pipe]; failed to write standard error ",
        "chunk 2 of 2: stream unavailable ",
        "[I/O error kind: permission denied] ",
        "(command status: failure)"
    );
    assert_eq!(
        error.to_string(),
        expected
    );
}

struct TwoChunkDiagnosticProgram;

impl CliProgram for TwoChunkDiagnosticProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure()
            .stdout("accepted")
            .stderr("denied")
    }
}

#[test]
fn output_error_identifies_the_failed_chunk_position() {
    let mut arguments = EmptyArguments;
    let mut output = FailingWriteSink::new(
        1,
        io::ErrorKind::PermissionDenied,
        "second chunk denied",
    );

    let error = output_error(
        RunInvocation::execute(
            &TwoChunkDiagnosticProgram,
            &mut arguments,
            &mut output,
        ),
    );
    assert_eq!(
        error.chunk_index(),
        1
    );
}
