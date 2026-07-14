// File:
//   - standard_streams.rs
// Path:
//   - src/cli/src/adapters/driven/standard_streams.rs
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
//   - Exact writes to current-process standard output and standard error.
// - Must-Not:
//   - Add formatting, choose messages, or interpret command status.
// - Allows:
//   - Write and flush caller-supplied text to one selected stream.
// - Split-When:
//   - Split when buffered sessions need a distinct adapter.
// - Merge-When:
//   - Another adapter owns the same standard-stream mechanism.
// - Summary:
//   - Standard output-sink adapter.
// - Description:
//   - Implements exact flushed process output without print macros.
// - Usage:
//   - Selected by the standard process driving composition.
// - Defaults:
//   - No newline or prefix is inserted.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driven adapter for exact, flushed standard-stream writes.
//!
//! Formatting remains part of caller command policy.
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
    fn write(
        &mut self,
        stream: OutputStream,
        text: &str,
    ) -> io::Result<()> {
        match stream {
            OutputStream::Stdout => write_if_non_empty(
                text,
                write_standard_output,
            ),
            OutputStream::Stderr => write_if_non_empty(
                text,
                write_standard_error,
            ),
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
    write_complete(
        &mut io::stdout().lock(),
        text,
    )
}

/// Writes one non-empty chunk to current-process standard error.
fn write_standard_error(text: &str) -> io::Result<()> {
    write_complete(
        &mut io::stderr().lock(),
        text,
    )
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
fn flush_error(
    source: io::Error,
    accepted_bytes: usize,
) -> io::Error {
    contextualize(
        StreamOperation::Flush {
            accepted_bytes,
        },
        source,
    )
}

/// Writes all text bytes and flushes the writer before returning.
///
/// # Errors
///
/// Returns the first write or flush error from the writer.
fn write_complete(
    writer: &mut impl io::Write,
    text: &str,
) -> io::Result<()> {
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
            return Err(
                write_error(
                    source,
                    accepted_bytes,
                    total_bytes,
                ),
            );
        };
        match writer.write(remaining) {
            Ok(0) => {
                let source = io::Error::new(
                    io::ErrorKind::WriteZero,
                    "writer accepted zero bytes",
                );
                return Err(
                    write_error(
                        source,
                        accepted_bytes,
                        total_bytes,
                    ),
                );
            }
            Ok(written) if written <= remaining.len() => {
                accepted_bytes = accepted_bytes.saturating_add(written);
            }
            Ok(written) => {
                let available_bytes = remaining.len();
                let source = invalid_write_count_error(
                    written,
                    available_bytes,
                );
                return Err(
                    write_error(
                        source,
                        accepted_bytes,
                        total_bytes,
                    ),
                );
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => {
                return Err(
                    write_error(
                        error,
                        accepted_bytes,
                        total_bytes,
                    ),
                );
            }
        }
    }
    loop {
        match writer.flush() {
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
            Err(error) => {
                return Err(
                    flush_error(
                        error,
                        total_bytes,
                    ),
                );
            }
            Ok(()) => return Ok(()),
        }
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod invalid_count_tests {
    use std::io::{self, Write};

    /// Increments one bounded fixture byte count.
    const fn increment(count: usize) -> usize {
        count.saturating_add(1)
    }

    /// Writes the shared five-byte fixture through the adapter helper.
    fn write_alpha(writer: &mut impl Write) -> io::Result<()> {
        super::write_complete(
            writer, "alpha",
        )
    }

    /// Fails when an empty-write provider is invoked unexpectedly.
    fn reject_text(_text: &str) -> io::Result<()> {
        Err(
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                "provider must not be acquired",
            ),
        )
    }

    #[test]
    fn empty_text_skips_provider_acquisition() {
        let result = super::write_if_non_empty(
            "",
            reject_text,
        );

        assert!(result.is_ok());
    }

    struct OverreportingWriter;

    impl Write for OverreportingWriter {
        fn write(
            &mut self,
            buffer: &[u8],
        ) -> io::Result<usize> {
            Ok(increment(buffer.len()))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn invalid_writer_count_reports_actual_and_allowed_bytes() {
        let mut writer = OverreportingWriter;

        let result = write_alpha(&mut writer);

        assert!(result.is_err());
        let Some(error) = result.err() else {
            return;
        };
        assert_eq!(
            error.kind(),
            io::ErrorKind::InvalidData
        );
        assert_eq!(
            error.to_string(),
            concat!(
                "failed to write standard stream after 0 of 5 bytes: ",
                "writer reported 6 bytes for a 5-byte buffer"
            )
        );
    }
}
