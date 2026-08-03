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
//   - Run process trait object test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Run process trait object test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Run process trait object test module.

use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

struct SuccessfulProgram;

impl CliProgram for SuccessfulProgram {
    fn execute(&self, _arguments: &[String]) -> CommandOutcome {
        CommandOutcome::success()
    }
}

#[test]
fn process_runner_accepts_a_program_trait_object() {
    let command: &dyn CliProgram = &SuccessfulProgram;

    let status = run_process(command);

    assert_eq!(status, ExitCode::SUCCESS);
}
