// File:
//   - output_error_status_display.rs
// Path:
//   - src/cli/tests/output_error_status_display.rs
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
//   - Regression coverage for command status in output-error diagnostics.
// - Must-Not:
//   - Access operating-system arguments or streams.
// - Allows:
//   - Compare deterministic failures from successful and failed commands.
// - Split-When:
//   - Another output-error display field needs independent coverage.
// - Merge-When:
//   - Output errors no longer retain command status.
// - Summary:
//   - Output-error status display regression.
// - Description:
//   - Proves ordinary diagnostics distinguish command outcome status.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - Both commands fail on the same standard-error write.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for command status in output-error text.
//!
//! Otherwise identical presentation failures must remain distinguishable.

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

struct SuccessfulProgram;

impl CliProgram for SuccessfulProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::success().stderr("diagnostic")
    }
}

struct FailedProgram;

impl CliProgram for FailedProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure().stderr("diagnostic")
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

fn render_failure(command: &dyn CliProgram) -> String {
    let mut arguments = EmptyArguments;
    let mut output = DeniedOutput;
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
fn display_distinguishes_successful_and_failed_commands() {
    let success = render_failure(&SuccessfulProgram);
    let failure = render_failure(&FailedProgram);

    assert_eq!(
        success,
        concat!(
            "failed to write standard error chunk 1 of 1: denied ",
            "[I/O error kind: permission denied] ",
            "(command status: success)"
        )
    );
    assert_eq!(
        failure,
        concat!(
            "failed to write standard error chunk 1 of 1: denied ",
            "[I/O error kind: permission denied] ",
            "(command status: failure)"
        )
    );
}
