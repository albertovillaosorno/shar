// File:
//   - process.rs
// Path:
//   - src/cli/src/adapters/driving/process.rs
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
//   - Current-process composition for shared CLI commands.
// - Must-Not:
//   - Interpret domain arguments or alter command output text.
// - Allows:
//   - Bind the command runner to environment arguments and standard streams.
// - Split-When:
//   - Split when another inbound process environment needs a distinct adapter.
// - Merge-When:
//   - Another driving adapter owns the same current-process composition.
// - Summary:
//   - Current-process CLI composition adapter.
// - Description:
//   - Maps process-neutral status into standard library exit codes.
// - Usage:
//   - Called by thin binaries and compatibility `run_env` functions.
// - Defaults:
//   - Output failures map to process failure without inventing diagnostics.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driving composition for one current-process CLI invocation.
//!
//! Caller commands retain all domain argument and message policy.
use std::process::ExitCode;

use crate::adapters::driven::{EnvironmentArguments, StandardStreams};
use crate::application::RunInvocation;
use crate::domain::ExitStatus;
use crate::ports::CliProgram;

/// Runs one caller-owned command in the current process.
#[must_use]
pub fn run_process(command: &(impl CliProgram + ?Sized)) -> ExitCode {
    let mut arguments = EnvironmentArguments;
    let mut output = StandardStreams;
    match RunInvocation::execute(
        command,
        &mut arguments,
        &mut output,
    ) {
        Ok(ExitStatus::Success) => ExitCode::SUCCESS,
        Ok(ExitStatus::Failure) | Err(_) => ExitCode::FAILURE,
    }
}
