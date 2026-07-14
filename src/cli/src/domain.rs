// File:
//   - domain.rs
// Path:
//   - src/cli/src/domain.rs
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
//   - Process-neutral command outcomes, output chunks, and argument failures.
// - Must-Not:
//   - Read process state, write streams, or encode domain command policy.
// - Allows:
//   - Represent ordered output and success or failure status.
// - Split-When:
//   - Split when one process-neutral value family becomes independent.
// - Merge-When:
//   - Another domain module owns the same command result invariants.
// - Summary:
//   - Shared CLI domain model.
// - Description:
//   - Defines command results without depending on operating-system streams.
// - Usage:
//   - Returned by commands and consumed by the shared runner.
// - Defaults:
//   - Output order is preserved and no newline is inserted implicitly.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Pure process-neutral values for shared CLI mechanisms.
//!
//! Domain commands decide content while adapters own process interaction.

/// Stable command completion status.
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
    pub fn new(
        stream: OutputStream,
        text: impl Into<String>,
    ) -> Self {
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
    pub fn stdout(
        mut self,
        text: impl Into<String>,
    ) -> Self {
        self.output
            .push(
                OutputChunk::new(
                    OutputStream::Stdout,
                    text,
                ),
            );
        self
    }

    /// Appends one standard-output line.
    #[must_use]
    pub fn stdout_line(
        self,
        text: impl Into<String>,
    ) -> Self {
        self.stdout(line(text))
    }

    /// Appends exact standard-error text.
    #[must_use]
    pub fn stderr(
        mut self,
        text: impl Into<String>,
    ) -> Self {
        self.output
            .push(
                OutputChunk::new(
                    OutputStream::Stderr,
                    text,
                ),
            );
        self
    }

    /// Appends one standard-error line.
    #[must_use]
    pub fn stderr_line(
        self,
        text: impl Into<String>,
    ) -> Self {
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
    pub fn is_failure_with_stderr_line(
        &self,
        expected: &str,
    ) -> bool {
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
        Self {
            index,
        }
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
mod tests {
    use super::{CommandOutcome, ExitStatus, OutputStream};

    #[test]
    fn line_helpers_preserve_output_order_and_add_one_newline() {
        let outcome = CommandOutcome::failure()
            .stdout("raw")
            .stderr_line("problem")
            .stdout_line("done");
        let expected = vec![
            super::OutputChunk::new(
                OutputStream::Stdout,
                "raw",
            ),
            super::OutputChunk::new(
                OutputStream::Stderr,
                "problem
",
            ),
            super::OutputChunk::new(
                OutputStream::Stdout,
                "done
",
            ),
        ];

        assert_eq!(
            outcome.status(),
            ExitStatus::Failure
        );
        assert_eq!(
            outcome.output(),
            expected.as_slice()
        );
    }
}
