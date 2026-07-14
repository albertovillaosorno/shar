// File:
//   - application.rs
// Path:
//   - src/cli/src/application.rs
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
//   - Process-neutral command execution and ordered output presentation.
// - Must-Not:
//   - Read process state directly or encode caller command policy.
// - Allows:
//   - Coordinate argument, command, and output ports into one invocation.
// - Split-When:
//   - Split when interactive sessions need a separate execution contract.
// - Merge-When:
//   - Another use case owns the same complete command-run sequence.
// - Summary:
//   - Shared CLI run-command use case.
// - Description:
//   - Executes caller policy and presents every output chunk in order.
// - Usage:
//   - Called by process driving composition and test adapters.
// - Defaults:
//   - Invalid Unicode arguments become one failed diagnostic outcome.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Shared application use case for one complete CLI invocation.
//!
//! Process sources and sinks remain replaceable behind ports.
use crate::domain::{CommandOutcome, ExitStatus, OutputStream};
use crate::ports::{ArgumentSource, CliProgram, OutputSink};

mod output_error;

pub use output_error::OutputError;

/// Stateless command runner.
#[derive(Debug, Clone, Copy)]
pub struct RunInvocation;

impl RunInvocation {
    /// Executes one command and presents all output chunks in order.
    ///
    /// # Errors
    ///
    /// Returns [`OutputError`] when complete presentation fails.
    pub fn execute(
        command: &(impl CliProgram + ?Sized),
        arguments: &mut (impl ArgumentSource + ?Sized),
        output: &mut (impl OutputSink + ?Sized),
    ) -> Result<ExitStatus, OutputError> {
        let (outcome, argument_error) = match arguments.arguments() {
            Ok(argument_values) => (
                command.execute(&argument_values),
                None,
            ),
            Err(error) => (
                CommandOutcome::failure().stderr_line(error.to_string()),
                Some(error),
            ),
        };
        let output_chunk_count = outcome
            .output()
            .len();
        let mut first_output_error: Option<OutputError> = None;
        let mut presented_chunk_count = 0usize;
        let mut suppressed_chunk_count = 0usize;
        let mut stdout_failed = false;
        let mut stderr_failed = false;
        for (chunk_index, chunk) in outcome
            .output()
            .iter()
            .enumerate()
        {
            let stream = chunk.stream();
            let stream_failed = match stream {
                OutputStream::Stdout => stdout_failed,
                OutputStream::Stderr => stderr_failed,
            };
            if stream_failed {
                increment_count(&mut suppressed_chunk_count);
                continue;
            }
            if chunk
                .text()
                .is_empty()
            {
                increment_count(&mut presented_chunk_count);
                continue;
            }
            if let Err(error) = output.write(
                stream,
                chunk.text(),
            ) {
                match stream {
                    OutputStream::Stdout => stdout_failed = true,
                    OutputStream::Stderr => stderr_failed = true,
                }
                if let Some(first_error) = first_output_error.as_mut() {
                    first_error.record_secondary(
                        chunk_index,
                        stream,
                        error,
                    );
                } else {
                    first_output_error = Some(
                        OutputError::new(
                            outcome.status(),
                            argument_error,
                            output_chunk_count,
                            chunk_index,
                            stream,
                            error,
                        ),
                    );
                }
            } else {
                increment_count(&mut presented_chunk_count);
            }
        }
        if let Some(error) = first_output_error.as_mut() {
            error.record_delivery_summary(
                presented_chunk_count,
                suppressed_chunk_count,
            );
        }
        first_output_error.map_or_else(
            || Ok(outcome.status()),
            Err,
        )
    }
}

/// Increments one bounded invocation counter without overflow.
const fn increment_count(count: &mut usize) {
    *count = count.saturating_add(1);
}

#[cfg(test)]
mod tests;
