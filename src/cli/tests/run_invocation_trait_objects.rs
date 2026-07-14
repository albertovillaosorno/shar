// File:
//   - run_invocation_trait_objects.rs
// Path:
//   - src/cli/tests/run_invocation_trait_objects.rs
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
//   - Regression coverage for trait-object invocation ports.
// - Must-Not:
//   - Access process arguments or standard streams.
// - Allows:
//   - Compose deterministic in-memory trait objects.
// - Split-When:
//   - Another invocation composition boundary needs distinct fixtures.
// - Merge-When:
//   - The runner no longer accepts replaceable ports.
// - Summary:
//   - Trait-object invocation regression.
// - Description:
//   - Proves all invocation ports accept dynamic dispatch.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The program returns success without output.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for dynamically dispatched invocation ports.
//!
//! Replaceable adapters must remain usable behind trait-object boundaries.

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

struct SuccessfulProgram;

impl CliProgram for SuccessfulProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::success()
    }
}

struct AcceptingOutput;

impl OutputSink for AcceptingOutput {
    fn write(
        &mut self,
        _stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn invocation_accepts_all_ports_as_trait_objects() {
    let command: &dyn CliProgram = &SuccessfulProgram;
    let mut empty_arguments = EmptyArguments;
    let arguments: &mut dyn ArgumentSource = &mut empty_arguments;
    let mut accepting_output = AcceptingOutput;
    let output: &mut dyn OutputSink = &mut accepting_output;

    let result = RunInvocation::execute(
        command, arguments, output,
    );

    assert!(
        matches!(
            result,
            Ok(ExitStatus::Success)
        )
    );
}
