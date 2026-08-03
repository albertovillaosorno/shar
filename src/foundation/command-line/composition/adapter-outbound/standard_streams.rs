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
//   - Standard streams outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Standard streams outbound adapter.
// - Description:
//   - Implements the declared responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Standard streams outbound adapter.

use std::io;

use crate::domain::OutputStream;
use crate::ports::OutputSink;

mod operation_error;

use operation_error::{
    StreamOperation, contextualize, invalid_write_count_error,
};

/// Writes exact text to the current process standard streams.
#[derive(Debug, Default, Clone, Copy)]
pub struct StandardStreams;

impl OutputSink for StandardStreams {
    fn write(&mut self, stream: OutputStream, text: &str) -> io::Result<()> {
        match stream {
            OutputStream::Stdout => {
                write_if_non_empty(text, write_standard_output)
            },
            OutputStream::Stderr => {
                write_if_non_empty(text, write_standard_error)
            },
        }
    }
}

/// Invokes one provider only when text contains bytes.
fn write_if_non_empty(
    text: &str,
    write: impl FnOnce(&str) -> io::Result<()>,
) -> io::Result<()> {
    if text.is_empty() {
        return Ok(());
    }
    write(text)
}

/// Writes one non-empty chunk to current-process standard output.
fn write_standard_output(text: &str) -> io::Result<()> {
    write_complete(&mut io::stdout().lock(), text)
}

/// Writes one non-empty chunk to current-process standard error.
fn write_standard_error(text: &str) -> io::Result<()> {
    write_complete(&mut io::stderr().lock(), text)
}

/// Adds write-operation context to one provider error.
fn write_error(
    source: io::Error,
    accepted_bytes: usize,
    total_bytes: usize,
) -> io::Error {
    contextualize(
        StreamOperation::Write {
            accepted_bytes,
            total_bytes,
        },
        source,
    )
}

/// Adds flush-operation context to one provider error.
fn flush_error(source: io::Error, accepted_bytes: usize) -> io::Error {
    contextualize(StreamOperation::Flush { accepted_bytes }, source)
}

/// Writes all text bytes and flushes the writer before returning.
///
/// # Errors
///
/// Returns the first write or flush error from the writer.
fn write_complete(writer: &mut impl io::Write, text: &str) -> io::Result<()> {
    if text.is_empty() {
        return Ok(());
    }
    let bytes = text.as_bytes();
    let total_bytes = bytes.len();
    let mut accepted_bytes = 0usize;
    while accepted_bytes < total_bytes {
        let Some(remaining) = bytes.get(accepted_bytes..) else {
            let source = io::Error::new(
                io::ErrorKind::InvalidData,
                "write progress exceeded the source buffer",
            );
            return Err(write_error(source, accepted_bytes, total_bytes));
        };
        match writer.write(remaining) {
            Ok(0) => {
                let source = io::Error::new(
                    io::ErrorKind::WriteZero,
                    "writer accepted zero bytes",
                );
                return Err(write_error(source, accepted_bytes, total_bytes));
            },
            Ok(written) if written <= remaining.len() => {
                accepted_bytes = accepted_bytes.saturating_add(written);
            },
            Ok(written) => {
                let available_bytes = remaining.len();
                let source =
                    invalid_write_count_error(written, available_bytes);
                return Err(write_error(source, accepted_bytes, total_bytes));
            },
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {},
            Err(error) => {
                return Err(write_error(error, accepted_bytes, total_bytes));
            },
        }
    }
    loop {
        match writer.flush() {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {},
            Err(error) => {
                return Err(flush_error(error, total_bytes));
            },
            Ok(()) => return Ok(()),
        }
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/foundation/command-line/unit/adapter-outbound/standard_streams/tests.rs"]
mod tests;

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/foundation/command-line/unit/adapter-outbound/standard_streams/invalid_count_tests.rs"]
mod invalid_count_tests;
