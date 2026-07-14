// File:
//   - output_error_argument_origin.rs
// Path:
//   - src/cli/tests/output_error_argument_origin.rs
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
//   - Regression coverage for argument-failure output provenance.
// - Must-Not:
//   - Access operating-system arguments or streams.
// - Allows:
//   - Compare an acquisition failure with matching command output.
// - Split-When:
//   - Another invocation-origin field needs independent coverage.
// - Merge-When:
//   - Output errors no longer preserve invocation origin.
// - Summary:
//   - Argument-origin output-error regression.
// - Description:
//   - Proves failed argument diagnostics remain distinguishable from commands.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - Both invocations attempt the same standard-error text.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for argument acquisition provenance.
//!
//! Matching command text must not erase whether the command was executed.

use std::io;

use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, OutputSink,
    OutputStream, RunInvocation,
};

const ARGUMENT_DIAGNOSTIC: &str = "command argument 3 is not valid Unicode";

struct InvalidArguments;

impl ArgumentSource for InvalidArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Err(ArgumentError::non_unicode(2))
    }
}

struct EmptyArguments;

impl ArgumentSource for EmptyArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Ok(Vec::new())
    }
}

struct MatchingDiagnosticProgram;

impl CliProgram for MatchingDiagnosticProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure().stderr_line(ARGUMENT_DIAGNOSTIC)
    }
}

struct UnusedProgram;

impl CliProgram for UnusedProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::success()
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

fn render_argument_failure() -> String {
    let mut arguments = InvalidArguments;
    let mut output = DeniedOutput;
    let result = RunInvocation::execute(
        &UnusedProgram,
        &mut arguments,
        &mut output,
    );
    let Some(error) = result.err() else {
        return String::new();
    };
    error.to_string()
}

fn render_matching_command_failure() -> String {
    let mut arguments = EmptyArguments;
    let mut output = DeniedOutput;
    let result = RunInvocation::execute(
        &MatchingDiagnosticProgram,
        &mut arguments,
        &mut output,
    );
    let Some(error) = result.err() else {
        return String::new();
    };
    error.to_string()
}

#[test]
fn output_error_distinguishes_argument_acquisition_failure() {
    let argument_failure = render_argument_failure();
    let command_failure = render_matching_command_failure();

    assert_eq!(
        argument_failure,
        concat!(
            "failed to write standard error chunk 1 of 1: denied ",
            "[I/O error kind: permission denied] ",
            "(command status: failure; argument acquisition error: ",
            "command argument 3 is not valid Unicode)"
        )
    );
    assert_eq!(
        command_failure,
        concat!(
            "failed to write standard error chunk 1 of 1: denied ",
            "[I/O error kind: permission denied] ",
            "(command status: failure)"
        )
    );
}
