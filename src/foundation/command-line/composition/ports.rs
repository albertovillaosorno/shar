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
//   - Port outbound outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Port outbound outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Port outbound outbound port.

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
    fn execute(&self, arguments: &[String]) -> CommandOutcome;
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
    fn write(&mut self, stream: OutputStream, text: &str) -> io::Result<()>;
}
