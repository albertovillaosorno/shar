// File:
//   - lib.rs
// Path:
//   - src/cli/src/lib.rs
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
//   - The shared CLI crate public hexagonal facade.
// - Must-Not:
//   - Encode domain command names, argument meaning, or default paths.
// - Allows:
//   - Expose process-neutral outcomes, ports, runner, and process composition.
// - Split-When:
//   - Split when one stable process mechanism becomes an independent crate.
// - Merge-When:
//   - Another facade owns the same crate-level contracts.
// - Summary:
//   - Shared CLI public facade.
// - Description:
//   - Centralizes stable process mechanics without domain-policy coupling.
// - Usage:
//   - Used by driving CLI adapters and thin binaries across the workspace.
// - Defaults:
//   - Commands receive arguments after the executable name.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Hexagonal shared CLI mechanisms for Schoenwald crates.
//!
//! Process interaction is centralized while command policy remains with
//! callers.
mod adapters;
mod application;
mod domain;
mod ports;

pub use adapters::{EnvironmentArguments, StandardStreams, run_process};
pub use application::{OutputError, RunInvocation};
pub use domain::{
    ArgumentError, CommandOutcome, ExitStatus, OutputChunk, OutputStream,
};
pub use ports::{ArgumentSource, CliProgram, OutputSink};
