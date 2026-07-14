// File:
//   - output_error.rs
// Path:
//   - src/cli/src/application/output_error.rs
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
//   - Contextual failures from one or both command output streams.
// - Must-Not:
//   - Execute commands, read arguments, or write provider streams directly.
// - Allows:
//   - Preserve primary and bounded secondary provider I/O failures.
// - Split-When:
//   - Another independent invocation error family is introduced.
// - Merge-When:
//   - Output failure provenance no longer belongs to application execution.
// - Summary:
//   - Contextual command output errors.
// - Description:
//   - Identifies failed streams while retaining original provider errors.
// - Usage:
//   - Returned by the shared invocation runner on presentation failure.
// - Defaults:
//   - The first failure is primary and the other stream is secondary.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Contextual failures from command output presentation.
//!
//! At most one failure per stream is retained after failed-stream suppression.

use std::io;

use crate::domain::{ArgumentError, ExitStatus, OutputStream};

mod display;

/// Failure while presenting one command output stream.
#[derive(Debug)]
pub struct OutputError {
    /// Command status produced before presentation failed.
    status: ExitStatus,
    /// Optional argument acquisition failure that replaced command execution.
    argument_error: Option<ArgumentError>,
    /// Total chunks in the command outcome.
    output_chunk_count: usize,
    /// Chunks presented successfully before the runner returned.
    presented_chunk_count: usize,
    /// Chunks suppressed after their stream failed.
    suppressed_chunk_count: usize,
    /// Zero-based output chunk that failed first.
    chunk_index: usize,
    /// Stream whose complete presentation failed first.
    stream: OutputStream,
    /// Original provider I/O failure for the first stream.
    source: io::Error,
    /// Optional zero-based chunk that failed after the primary failure.
    secondary_chunk_index: Option<usize>,
    /// Optional stream that failed after the primary failure.
    secondary_stream: Option<OutputStream>,
    /// Optional provider error from the secondary stream.
    secondary_source: Option<io::Error>,
}

impl OutputError {
    /// Creates one contextual output failure.
    pub(super) const fn new(
        status: ExitStatus,
        argument_error: Option<ArgumentError>,
        output_chunk_count: usize,
        chunk_index: usize,
        stream: OutputStream,
        source: io::Error,
    ) -> Self {
        Self {
            status,
            argument_error,
            output_chunk_count,
            presented_chunk_count: 0,
            suppressed_chunk_count: 0,
            chunk_index,
            stream,
            source,
            secondary_chunk_index: None,
            secondary_stream: None,
            secondary_source: None,
        }
    }

    /// Returns the command status produced before presentation failed.
    #[must_use]
    pub const fn status(&self) -> ExitStatus {
        self.status
    }

    /// Returns the argument acquisition failure that replaced command
    /// execution.
    #[must_use]
    pub const fn argument_error(&self) -> Option<&ArgumentError> {
        self.argument_error
            .as_ref()
    }

    /// Returns the total number of chunks in the command outcome.
    #[must_use]
    pub const fn output_chunk_count(&self) -> usize {
        self.output_chunk_count
    }

    /// Returns the number of chunks presented successfully.
    #[must_use]
    pub const fn presented_chunk_count(&self) -> usize {
        self.presented_chunk_count
    }

    /// Returns the number of chunks suppressed after a stream failure.
    #[must_use]
    pub const fn suppressed_chunk_count(&self) -> usize {
        self.suppressed_chunk_count
    }

    /// Returns the zero-based output chunk that failed first.
    #[must_use]
    pub const fn chunk_index(&self) -> usize {
        self.chunk_index
    }

    /// Returns the stream whose presentation failed.
    #[must_use]
    pub const fn stream(&self) -> OutputStream {
        self.stream
    }

    /// Returns the underlying I/O error kind.
    #[must_use]
    pub fn kind(&self) -> io::ErrorKind {
        self.source
            .kind()
    }

    /// Returns the underlying provider I/O error.
    #[must_use]
    pub const fn io_error(&self) -> &io::Error {
        &self.source
    }

    /// Returns the second failed chunk when both channels failed.
    #[must_use]
    pub const fn secondary_chunk_index(&self) -> Option<usize> {
        self.secondary_chunk_index
    }

    /// Returns the second failed stream when both channels failed.
    #[must_use]
    pub const fn secondary_stream(&self) -> Option<OutputStream> {
        self.secondary_stream
    }

    /// Returns the second provider I/O error when both channels failed.
    #[must_use]
    pub const fn secondary_io_error(&self) -> Option<&io::Error> {
        self.secondary_source
            .as_ref()
    }

    /// Records the final best-effort delivery summary.
    pub(super) const fn record_delivery_summary(
        &mut self,
        presented_chunk_count: usize,
        suppressed_chunk_count: usize,
    ) {
        self.presented_chunk_count = presented_chunk_count;
        self.suppressed_chunk_count = suppressed_chunk_count;
    }

    /// Records the other stream failure without replacing primary provenance.
    pub(super) fn record_secondary(
        &mut self,
        chunk_index: usize,
        stream: OutputStream,
        source: io::Error,
    ) {
        if self
            .secondary_source
            .is_none()
        {
            self.secondary_chunk_index = Some(chunk_index);
            self.secondary_stream = Some(stream);
            self.secondary_source = Some(source);
        }
    }
}

impl std::error::Error for OutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
