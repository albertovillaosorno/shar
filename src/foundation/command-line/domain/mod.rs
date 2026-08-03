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
//   - Domain domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Domain domain module.
// - Description:
//   - Implements the declared domain module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Domain domain module.

/// Stable process-neutral completion state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitStatus {
    /// The command completed successfully.
    Success,
    /// The command failed or could not present its complete output.
    Failure,
}

/// Destination stream for one ordered output chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputStream {
    /// Standard output is the command result channel.
    Stdout,
    /// Standard error is the diagnostic channel.
    Stderr,
}

/// One ordered command output chunk.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputChunk {
    /// Destination selected by the caller command.
    stream: OutputStream,
    /// Exact text presented without implicit formatting.
    text: String,
}

impl OutputChunk {
    /// Creates one exact output chunk without adding a newline.
    #[must_use]
    pub fn new(stream: OutputStream, text: impl Into<String>) -> Self {
        Self {
            stream,
            text: text.into(),
        }
    }

    /// Returns the destination stream.
    #[must_use]
    pub const fn stream(&self) -> OutputStream {
        self.stream
    }

    /// Returns the exact output text.
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }
}

/// Complete process-neutral outcome of one command invocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandOutcome {
    /// Stable completion status.
    status: ExitStatus,
    /// Ordered output chunks.
    output: Vec<OutputChunk>,
}

impl CommandOutcome {
    /// Creates a successful outcome with no output.
    #[must_use]
    pub const fn success() -> Self {
        Self {
            status: ExitStatus::Success,
            output: Vec::new(),
        }
    }

    /// Creates a failed outcome with no output.
    #[must_use]
    pub const fn failure() -> Self {
        Self {
            status: ExitStatus::Failure,
            output: Vec::new(),
        }
    }

    /// Appends exact standard-output text.
    #[must_use]
    pub fn stdout(mut self, text: impl Into<String>) -> Self {
        self.output
            .push(OutputChunk::new(OutputStream::Stdout, text));
        self
    }

    /// Appends one standard-output line.
    #[must_use]
    pub fn stdout_line(self, text: impl Into<String>) -> Self {
        self.stdout(line(text))
    }

    /// Appends exact standard-error text.
    #[must_use]
    pub fn stderr(mut self, text: impl Into<String>) -> Self {
        self.output
            .push(OutputChunk::new(OutputStream::Stderr, text));
        self
    }

    /// Appends one standard-error line.
    #[must_use]
    pub fn stderr_line(self, text: impl Into<String>) -> Self {
        self.stderr(line(text))
    }

    /// Returns the command status.
    #[must_use]
    pub const fn status(&self) -> ExitStatus {
        self.status
    }

    /// Returns output chunks in presentation order.
    #[must_use]
    pub fn output(&self) -> &[OutputChunk] {
        &self.output
    }

    /// Reports whether this is one failed command with one stderr line.
    #[must_use]
    pub fn is_failure_with_stderr_line(&self, expected: &str) -> bool {
        self.status == ExitStatus::Failure
            && matches!(
                self.output.as_slice(),
                [chunk]
                    if chunk.stream() == OutputStream::Stderr
                        && chunk.text() == line(expected)
            )
    }
}

/// Failure while decoding process arguments.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArgumentError {
    /// Zero-based argument index after the executable name.
    index: usize,
}

impl ArgumentError {
    /// Creates an invalid-Unicode argument failure.
    #[must_use]
    pub const fn non_unicode(index: usize) -> Self {
        Self { index }
    }

    /// Returns the zero-based command argument index.
    #[must_use]
    pub const fn index(&self) -> usize {
        self.index
    }
}

impl core::fmt::Display for ArgumentError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let position = u128::try_from(self.index)
            .unwrap_or(u128::MAX)
            .saturating_add(1);
        write!(
            formatter,
            "command argument {position} is not valid Unicode"
        )
    }
}

impl std::error::Error for ArgumentError {}

/// Ensures caller-supplied text ends with a newline.
fn line(text: impl Into<String>) -> String {
    let mut line_text = text.into();
    if !line_text.ends_with('\n') {
        line_text.push('\n');
    }
    line_text
}

#[cfg(test)]
#[path = "../../../../tests/foundation/command-line/unit/domain/tests.rs"]
mod tests;
