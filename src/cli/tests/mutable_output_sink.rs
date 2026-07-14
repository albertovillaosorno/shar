// File:
//   - mutable_output_sink.rs
// Path:
//   - src/cli/tests/mutable_output_sink.rs
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
//   - Regression coverage for ordinary mutable output sinks.
// - Must-Not:
//   - Require interior mutability or operating-system streams.
// - Allows:
//   - Record exact output in a directly owned vector.
// - Split-When:
//   - Another port receiver contract needs independent coverage.
// - Merge-When:
//   - Output sinks no longer own mutable presentation state.
// - Summary:
//   - Mutable output-sink regression.
// - Description:
//   - Proves sequential sinks can mutate through exclusive port access.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The program emits one standard-output chunk.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for directly mutable output sinks.
//!
//! Sequential presentation must not force runtime interior-mutability wrappers.

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
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::success().stdout("direct")
    }
}

#[derive(Default)]
struct VecOutput {
    /// Exact chunks accepted by the sink.
    chunks: Vec<(
        OutputStream,
        String,
    )>,
}

impl OutputSink for VecOutput {
    fn write(
        &mut self,
        stream: OutputStream,
        text: &str,
    ) -> io::Result<()> {
        self.chunks
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
fn invocation_accepts_a_directly_mutable_output_sink() {
    let mut arguments = EmptyArguments;
    let mut output = VecOutput::default();

    let result = RunInvocation::execute(
        &OneChunkProgram,
        &mut arguments,
        &mut output,
    );

    assert!(
        matches!(
            result,
            Ok(ExitStatus::Success)
        )
    );
    assert_eq!(
        output.chunks,
        [
            (
                OutputStream::Stdout,
                "direct".to_owned()
            )
        ]
    );
}
