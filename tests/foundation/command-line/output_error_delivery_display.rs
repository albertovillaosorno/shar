// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Output error delivery display test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Output error delivery display test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Output error delivery display test module.

#[path = "support/failing_write_sink.rs"]
pub mod failing_write_sink;
#[path = "support/output_error.rs"]
pub mod support;

use std::io;

use failing_write_sink::FailingWriteSink;
use schoenwald_cli::{
    ArgumentError, ArgumentSource, CliProgram, CommandOutcome, RunInvocation,
};
use support::output_error;

struct EmptyArguments;

impl ArgumentSource for EmptyArguments {
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError> {
        Ok(Vec::new())
    }
}

struct LaterDiagnosticProgram;

impl CliProgram for LaterDiagnosticProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::failure()
            .stdout("primary")
            .stderr("diagnostic")
    }
}

struct LaterPrimaryProgram;

impl CliProgram for LaterPrimaryProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::failure()
            .stdout("primary")
            .stdout("suppressed")
    }
}

fn render_delivery(command: &dyn CliProgram) -> String {
    let mut arguments = EmptyArguments;
    let mut output =
        FailingWriteSink::new(0, io::ErrorKind::BrokenPipe, "blocked");
    output_error(RunInvocation::execute(command, &mut arguments, &mut output))
        .to_string()
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
