// File:
//   - failed_stream_empty_chunk.rs
// Path:
//   - src/cli/tests/failed_stream_empty_chunk.rs
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
//   - Regression coverage for empty chunks after a stream failure.
// - Must-Not:
//   - Access operating-system arguments or streams.
// - Allows:
//   - Use deterministic output failure and delivery-summary fixtures.
// - Split-When:
//   - Another failed-stream classification needs independent coverage.
// - Merge-When:
//   - Empty chunks no longer participate in invocation delivery summaries.
// - Summary:
//   - Failed-stream empty-chunk regression.
// - Description:
//   - Proves an empty chunk after failure is suppressed, not presented.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - Standard output fails before its later empty chunk is encountered.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for empty chunks after one stream fails.
//!
//! Delivery summaries classify chunks according to stream state first.

#[path = "support/output_error.rs"]
mod support;

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
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure()
            .stdout("primary")
            .stdout("")
            .stderr("diagnostic")
    }
}

struct DenyStandardOutput;

impl OutputSink for DenyStandardOutput {
    fn write(
        &mut self,
        stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        match stream {
            OutputStream::Stdout => Err(
                io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "blocked",
                ),
            ),
            OutputStream::Stderr => Ok(()),
        }
    }
}

#[test]
fn empty_chunk_after_failed_stream_is_suppressed() {
    let mut arguments = EmptyArguments;
    let mut output = DenyStandardOutput;

    let error = output_error(
        RunInvocation::execute(
            &EmptyAfterFailureProgram,
            &mut arguments,
            &mut output,
        ),
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
}
