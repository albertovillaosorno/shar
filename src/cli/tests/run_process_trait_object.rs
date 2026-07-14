// File:
//   - run_process_trait_object.rs
// Path:
//   - src/cli/tests/run_process_trait_object.rs
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
//   - Regression coverage for trait-object process programs.
// - Must-Not:
//   - Emit output or interpret test-runner arguments.
// - Allows:
//   - Invoke one dynamically dispatched no-output program.
// - Split-When:
//   - Another process-composition behavior needs distinct fixtures.
// - Merge-When:
//   - Current-process composition no longer accepts caller programs.
// - Summary:
//   - Trait-object process regression.
// - Description:
//   - Proves process composition accepts dynamic dispatch.
// - Usage:
//   - Executed by the schoenwald-cli integration test target.
// - Defaults:
//   - The program ignores arguments and succeeds.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for dynamically dispatched process programs.
//!
//! Thin binaries may keep caller programs behind stable trait objects.

use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

struct SuccessfulProgram;

impl CliProgram for SuccessfulProgram {
    fn execute(
        &self,
        _arguments: &[String],
    ) -> CommandOutcome {
        CommandOutcome::success()
    }
}

#[test]
fn process_runner_accepts_a_program_trait_object() {
    let command: &dyn CliProgram = &SuccessfulProgram;

    let status = run_process(command);

    assert_eq!(
        status,
        ExitCode::SUCCESS
    );
}
