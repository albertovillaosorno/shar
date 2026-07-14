// File:
//   - output_error_delivery_display.rs
// Path:
//   - src/cli/tests/output_error_delivery_display.rs
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
//   - Regression coverage for delivery counts in output-error diagnostics.
// - Must-Not:
//   - Access operating-system arguments or streams.
// - Allows:
//   - Compare deterministic partial-delivery outcomes.
// - Split-When:
//   - Another output-error display field needs independent coverage.
// - Merge-When:
//   - Output errors no longer retain delivery counts.
// - Summary:
//   - Output-error delivery display regression.
// - Description:
//   - Proves ordinary diagnostics distinguish final delivery outcomes.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The first standard-output write fails permanently.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for delivery summaries in output-error text.
//!
//! Otherwise identical primary failures must preserve final delivery results.

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

struct LaterDiagnosticProgram;

impl CliProgram for LaterDiagnosticProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure()
            .stdout("primary")
            .stderr("diagnostic")
    }
}

struct LaterPrimaryProgram;

impl CliProgram for LaterPrimaryProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure()
            .stdout("primary")
            .stdout("suppressed")
    }
}

#[derive(Default)]
struct FirstWriteDenied {
    /// Number of sink calls attempted by the runner.
    calls: usize,
}

impl OutputSink for FirstWriteDenied {
    fn write(
        &mut self,
        _stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        let call = self.calls;
        self.calls = self
            .calls
            .saturating_add(1);
        if call == 0 {
            return Err(
                io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "blocked",
                ),
            );
        }
        Ok(())
    }
}

fn render_delivery(command: &dyn CliProgram) -> String {
    let mut arguments = EmptyArguments;
    let mut output = FirstWriteDenied::default();
    let result = RunInvocation::execute(
        command,
        &mut arguments,
        &mut output,
    );
    let Some(error) = result.err() else {
        return String::new();
    };
    error.to_string()
}

#[test]
fn display_distinguishes_presented_and_suppressed_chunks() {
    let presented = render_delivery(&LaterDiagnosticProgram);
    let suppressed = render_delivery(&LaterPrimaryProgram);

    assert_eq!(
        presented,
        concat!(
            "failed to write standard output chunk 1 of 2: blocked ",
            "[I/O error kind: broken pipe] ",
            "(command status: failure; presented chunks: 1; ",
            "suppressed chunks: 0)"
        )
    );
    assert_eq!(
        suppressed,
        concat!(
            "failed to write standard output chunk 1 of 2: blocked ",
            "[I/O error kind: broken pipe] ",
            "(command status: failure; presented chunks: 0; ",
            "suppressed chunks: 1)"
        )
    );
}
