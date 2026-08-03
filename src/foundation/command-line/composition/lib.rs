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
//   - Command line lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Command line lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Command line lib.rs.

#[path = "adapters.rs"]
mod adapters;
#[path = "application/mod.rs"]
mod application;
#[path = "../domain/mod.rs"]
mod domain;
#[path = "ports.rs"]
mod ports;

pub use adapters::{EnvironmentArguments, StandardStreams, run_process};
pub use application::{OutputError, RunInvocation};
pub use domain::{
    ArgumentError, CommandOutcome, ExitStatus, OutputChunk, OutputStream,
};
pub use ports::{ArgumentSource, CliProgram, OutputSink};
