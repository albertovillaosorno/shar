// File:
//   - output_error_kind_display.rs
// Path:
//   - src/cli/tests/output_error_kind_display.rs
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
//   - Regression coverage for I/O error kinds in output diagnostics.
// - Must-Not:
//   - Access operating-system arguments or streams.
// - Allows:
//   - Compare provider failures with identical messages and distinct kinds.
// - Split-When:
//   - Another provider-error display field needs independent coverage.
// - Merge-When:
//   - Output diagnostics no longer render provider failures.
// - Summary:
//   - Output-error kind display regression.
// - Description:
//   - Proves ordinary diagnostics distinguish provider failure categories.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - Every provider failure uses the same human-readable message.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for provider I/O kinds in output-error text.
//!
//! Equal provider messages must not erase distinct failure categories.

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

struct DiagnosticProgram;

impl CliProgram for DiagnosticProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::failure().stderr("diagnostic")
    }
}

struct MatchingMessageSink {
    /// Provider category returned with the shared message.
    kind: io::ErrorKind,
}

impl OutputSink for MatchingMessageSink {
    fn write(
        &mut self,
        _stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        Err(
            io::Error::new(
                self.kind, "blocked",
            ),
        )
    }
}

fn render_failure(kind: io::ErrorKind) -> String {
    let mut arguments = EmptyArguments;
    let mut output = MatchingMessageSink {
        kind,
    };
    let result = RunInvocation::execute(
        &DiagnosticProgram,
        &mut arguments,
        &mut output,
    );
    let Some(error) = result.err() else {
        return String::new();
    };
    error.to_string()
}

#[test]
fn display_distinguishes_matching_messages_with_different_kinds() {
    let denied = render_failure(io::ErrorKind::PermissionDenied);
    let broken_pipe = render_failure(io::ErrorKind::BrokenPipe);

    assert_eq!(
        denied,
        concat!(
            "failed to write standard error chunk 1 of 1: blocked ",
            "[I/O error kind: permission denied] ",
            "(command status: failure)"
        )
    );
    assert_eq!(
        broken_pipe,
        concat!(
            "failed to write standard error chunk 1 of 1: blocked ",
            "[I/O error kind: broken pipe] ",
            "(command status: failure)"
        )
    );
}

const RAW_OS_ERROR_CODE: i32 = 5;

struct RawOsErrorSink;

impl OutputSink for RawOsErrorSink {
    fn write(
        &mut self,
        _stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(RAW_OS_ERROR_CODE))
    }
}

#[test]
fn display_retains_the_raw_operating_system_error_code() {
    let mut arguments = EmptyArguments;
    let mut output = RawOsErrorSink;

    let result = RunInvocation::execute(
        &DiagnosticProgram,
        &mut arguments,
        &mut output,
    );

    assert!(result.is_err());
    let Some(error) = result.err() else {
        return;
    };
    assert!(
        error
            .to_string()
            .contains("[OS error code: 5]")
    );
}

#[derive(Debug)]
struct NestedRawOsError {
    /// Original operating-system provider error.
    source: io::Error,
}

impl core::fmt::Display for NestedRawOsError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        formatter.write_str("contextual provider failure")
    }
}

impl std::error::Error for NestedRawOsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

struct NestedRawOsErrorSink;

impl OutputSink for NestedRawOsErrorSink {
    fn write(
        &mut self,
        _stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        let source = io::Error::from_raw_os_error(RAW_OS_ERROR_CODE);
        Err(
            io::Error::new(
                source.kind(),
                NestedRawOsError {
                    source,
                },
            ),
        )
    }
}

#[test]
fn display_finds_raw_codes_in_provider_source_chains() {
    let mut arguments = EmptyArguments;
    let mut output = NestedRawOsErrorSink;

    let result = RunInvocation::execute(
        &DiagnosticProgram,
        &mut arguments,
        &mut output,
    );

    assert!(result.is_err());
    let Some(error) = result.err() else {
        return;
    };
    assert!(
        error
            .to_string()
            .contains("[OS error code: 5]")
    );
}
