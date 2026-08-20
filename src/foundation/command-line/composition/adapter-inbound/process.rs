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
//   - Process inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Process inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Process inbound adapter.

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
    match RunInvocation::execute(command, &mut arguments, &mut output) {
        Ok(ExitStatus::Success) => ExitCode::SUCCESS,
        Ok(ExitStatus::Failure) | Err(_) => ExitCode::FAILURE,
    }
}
