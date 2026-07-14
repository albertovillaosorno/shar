// File:
//   - display.rs
// Path:
//   - src/cli/src/application/output_error/display.rs
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
//   - Stable human-readable rendering for contextual output failures.
// - Must-Not:
//   - Mutate delivery state or expose provider implementation details.
// - Allows:
//   - Render retained status, stream, chunk, kind, and delivery provenance.
// - Split-When:
//   - Another independent output-error presentation format is introduced.
// - Merge-When:
//   - OutputError no longer owns human-readable presentation.
// - Summary:
//   - Output-error display implementation.
// - Description:
//   - Renders deterministic operator diagnostics from retained typed state.
// - Usage:
//   - Loaded privately by the output-error application module.
// - Defaults:
//   - Primary failure appears before bounded secondary failure context.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Stable human-readable output-error presentation.
//!
//! Rendering remains separate from mutation and provider error retention.

use std::io;

mod raw_os_error;

use super::OutputError;
use crate::domain::{ExitStatus, OutputStream};

/// Returns the stable human name for one process output stream.
const fn stream_name(stream: OutputStream) -> &'static str {
    match stream {
        OutputStream::Stdout => "standard output",
        OutputStream::Stderr => "standard error",
    }
}

/// Returns the stable human name for one command status.
const fn status_name(status: ExitStatus) -> &'static str {
    match status {
        ExitStatus::Success => "success",
        ExitStatus::Failure => "failure",
    }
}

/// Converts one I/O kind into its stable human-readable error value.
fn display_kind(kind: io::ErrorKind) -> io::Error {
    io::Error::from(kind)
}

/// Returns whether one error has nonzero delivery-summary evidence.
const fn has_delivery_summary(error: &OutputError) -> bool {
    error.presented_chunk_count != 0 || error.suppressed_chunk_count != 0
}

/// Renders one raw operating-system error code when the provider has one.
fn write_raw_os_error(
    formatter: &mut core::fmt::Formatter<'_>,
    source: &io::Error,
) -> core::fmt::Result {
    let provider_source = source
        .get_ref()
        .unwrap_or(source);
    let Some(raw_os_error) = raw_os_error::find(provider_source) else {
        return Ok(());
    };
    write!(
        formatter,
        " [OS error code: {raw_os_error}]"
    )
}

/// Converts one zero-based chunk index into a bounded display position.
const fn one_based(index: usize) -> usize {
    index.saturating_add(1)
}

impl core::fmt::Display for OutputError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        let primary_stream_name = stream_name(self.stream);
        let primary_chunk_position = one_based(self.chunk_index);
        let output_chunk_count = self.output_chunk_count;
        let primary_source = &self.source;
        let primary_kind = display_kind(primary_source.kind());
        write!(
            formatter,
            "failed to write {primary_stream_name} chunk \
             {primary_chunk_position} of {output_chunk_count}: \
             {primary_source}"
        )?;
        write!(
            formatter,
            " [I/O error kind: {primary_kind}]"
        )?;
        write_raw_os_error(
            formatter,
            primary_source,
        )?;
        if let Some(secondary_stream) = self.secondary_stream
            && let Some(secondary_source) = &self.secondary_source
            && let Some(secondary_chunk_index) = self.secondary_chunk_index
        {
            let secondary_stream_name = stream_name(secondary_stream);
            let secondary_chunk_position = one_based(secondary_chunk_index);
            let secondary_kind = display_kind(secondary_source.kind());
            write!(
                formatter,
                "; failed to write {secondary_stream_name} chunk \
                 {secondary_chunk_position} of {output_chunk_count}: \
                 {secondary_source}"
            )?;
            write!(
                formatter,
                " [I/O error kind: {secondary_kind}]"
            )?;
            write_raw_os_error(
                formatter,
                secondary_source,
            )?;
        }
        let command_status_name = status_name(self.status);
        write!(
            formatter,
            " (command status: {command_status_name}"
        )?;
        if let Some(argument_error) = &self.argument_error {
            write!(
                formatter,
                "; argument acquisition error: {argument_error}"
            )?;
        }
        if has_delivery_summary(self) {
            let presented_chunk_count = self.presented_chunk_count;
            let suppressed_chunk_count = self.suppressed_chunk_count;
            write!(
                formatter,
                "; presented chunks: {presented_chunk_count}; suppressed \
                 chunks: {suppressed_chunk_count}"
            )?;
        }
        write!(
            formatter,
            ")"
        )
    }
}
