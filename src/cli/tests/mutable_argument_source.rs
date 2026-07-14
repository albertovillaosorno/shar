// File:
//   - mutable_argument_source.rs
// Path:
//   - src/cli/tests/mutable_argument_source.rs
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
//   - Regression coverage for ordinary mutable argument sources.
// - Must-Not:
//   - Require interior mutability or process environment access.
// - Allows:
//   - Move a one-shot argument vector through exclusive port access.
// - Split-When:
//   - Another input-port receiver contract needs independent coverage.
// - Merge-When:
//   - Argument sources no longer own consumable input state.
// - Summary:
//   - Mutable argument-source regression.
// - Description:
//   - Proves one-shot sources can consume state through the input port.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The source supplies one argument and is invoked once.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for directly mutable argument sources.
//!
//! Sequential acquisition must not force runtime interior-mutability wrappers.

use std::io;

use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, ExitStatus,
    OutputSink, OutputStream, RunInvocation,
};

struct OneShotArguments {
    /// Arguments moved into the invocation on the first read.
    values: Vec<String>,
}

impl ArgumentSource for OneShotArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Ok(std::mem::take(&mut self.values))
    }
}

struct EchoProgram;

impl CliProgram for EchoProgram {
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::success().stdout(arguments.join("|"))
    }
}

#[derive(Default)]
struct VecOutput {
    /// Exact text accepted by the sink.
    text: String,
}

impl OutputSink for VecOutput {
    fn write(
        &mut self,
        _stream: OutputStream,
        text: &str,
    ) -> io::Result<()> {
        self.text
            .push_str(text);
        Ok(())
    }
}

#[test]
fn invocation_accepts_a_consuming_argument_source() {
    let mut arguments = OneShotArguments {
        values: vec!["alpha".to_owned()],
    };
    let mut output = VecOutput::default();

    let result = RunInvocation::execute(
        &EchoProgram,
        &mut arguments,
        &mut output,
    );

    assert!(
        matches!(
            result,
            Ok(ExitStatus::Success)
        )
    );
    assert!(
        arguments
            .values
            .is_empty()
    );
    assert_eq!(
        output.text,
        "alpha"
    );
}
