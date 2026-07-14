// File:
//   - ports.rs
// Path:
//   - src/cli/src/ports.rs
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
//   - Shared command, argument-source, and output-sink contracts.
// - Must-Not:
//   - Implement process behavior or encode domain argument meaning.
// - Allows:
//   - Isolate command execution from operating-system process mechanisms.
// - Split-When:
//   - Split when one capability becomes an independent provider contract.
// - Merge-When:
//   - Another facade owns the same CLI mechanism contracts.
// - Summary:
//   - Shared CLI ports.
// - Description:
//   - Defines replaceable command input and output boundaries.
// - Usage:
//   - Implemented by command adapters and process driven adapters.
// - Defaults:
//   - Argument lists exclude the executable name.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Hexagonal ports for shared CLI mechanisms.
//!
//! Command policy and process mechanisms remain independently replaceable.
use std::io;

use crate::domain::{ArgumentError, CommandOutcome, OutputStream};

/// Supplies decoded command arguments excluding the executable name.
pub trait ArgumentSource {
    /// Reads the complete argument vector.
    ///
    /// # Errors
    ///
    /// Returns [`ArgumentError`] when an argument cannot be decoded.
    fn arguments(&mut self) -> Result<Vec<String>, ArgumentError>;
}

/// Executes caller-owned CLI policy over decoded arguments.
pub trait CliProgram {
    /// Executes one command invocation and returns its complete outcome.
    ///
    /// Callers must present or inspect the returned status and output.
    ///
    /// ```compile_fail
    /// #![deny(unused_must_use)]
    /// use schoenwald_cli::{CliProgram, CommandOutcome};
    ///
    /// struct Program;
    ///
    /// impl CliProgram for Program {
    ///     fn execute(&self, _arguments: &[String]) -> CommandOutcome {
    ///         CommandOutcome::success()
    ///     }
    /// }
    ///
    /// Program.execute(&[]);
    /// ```
    #[must_use = "command outcomes must be presented or inspected"]
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome;
}

/// Presents exact output text to process-neutral streams.
pub trait OutputSink {
    /// Writes one exact output chunk.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when complete presentation fails.
    ///
    /// Implementations own retries because only the concrete provider knows
    /// whether an interrupted operation can be repeated without side effects.
    fn write(
        &mut self,
        stream: OutputStream,
        text: &str,
    ) -> io::Result<()>;
}
