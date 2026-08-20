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
//   - Output error kind display test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Output error kind display test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Output error kind display test module.

#[path = "support/output_error.rs"]
pub mod support;

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

struct DiagnosticProgram;

impl CliProgram for DiagnosticProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::failure().stderr("diagnostic")
    }
}

struct MatchingMessageSink {
    /// Provider category returned with the shared message.
    kind: io::ErrorKind,
}

impl OutputSink for MatchingMessageSink {
    fn write(&mut self, _stream: OutputStream, _text: &str) -> io::Result<()> {
        Err(io::Error::new(self.kind, "blocked"))
    }
}

fn render_failure(kind: io::ErrorKind) -> String {
    let mut arguments = EmptyArguments;
    let mut output = MatchingMessageSink { kind };
    output_error(RunInvocation::execute(
        &DiagnosticProgram,
        &mut arguments,
        &mut output,
    ))
    .to_string()
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
    fn write(&mut self, _stream: OutputStream, _text: &str) -> io::Result<()> {
        Err(io::Error::from_raw_os_error(RAW_OS_ERROR_CODE))
    }
}

#[test]
fn display_retains_the_raw_operating_system_error_code() {
    let mut arguments = EmptyArguments;
    let mut output = RawOsErrorSink;

    let error = output_error(RunInvocation::execute(
        &DiagnosticProgram,
        &mut arguments,
        &mut output,
    ));
    assert!(error.to_string().contains("[OS error code: 5]"));
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
    fn write(&mut self, _stream: OutputStream, _text: &str) -> io::Result<()> {
        let source = io::Error::from_raw_os_error(RAW_OS_ERROR_CODE);
        Err(io::Error::new(source.kind(), NestedRawOsError { source }))
    }
}

#[test]
fn display_finds_raw_codes_in_provider_source_chains() {
    let mut arguments = EmptyArguments;
    let mut output = NestedRawOsErrorSink;

    let error = output_error(RunInvocation::execute(
        &DiagnosticProgram,
        &mut arguments,
        &mut output,
    ));
    assert!(error.to_string().contains("[OS error code: 5]"));
}
