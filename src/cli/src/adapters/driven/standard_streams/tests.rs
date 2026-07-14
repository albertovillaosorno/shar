// File:
//   - tests.rs
// Path:
//   - src/cli/src/adapters/driven/standard_streams/tests.rs
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
//   - Regressions for complete writes and operation error context.
// - Must-Not:
//   - Access operating-system standard streams.
// - Allows:
//   - Use a deterministic in-memory writer.
// - Split-When:
//   - Fixture families no longer exercise the same write helper.
// - Merge-When:
//   - The standard-stream adapter no longer owns flushing.
// - Summary:
//   - Standard stream write and error-context tests.
// - Description:
//   - Proves exact delivery and distinct write or flush failures.
// - Usage:
//   - Compiled with the schoenwald-cli unit test target.
// - Defaults:
//   - The recording writer accepts every byte.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Regression coverage for complete standard-stream presentation.
//!
//! Complete writes flush successfully and failures retain operation context.

use std::io::{self, Write};

/// Increments one bounded fixture counter.
const fn increment(count: usize) -> usize {
    count.saturating_add(1)
}

/// Writes the shared non-empty fixture text through the adapter helper.
fn write_alpha(writer: &mut impl Write) -> io::Result<()> {
    super::write_complete(
        writer, "alpha",
    )
}

/// Writes the shared empty fixture text through the adapter helper.
fn write_empty(writer: &mut impl Write) -> io::Result<()> {
    super::write_complete(
        writer, "",
    )
}

#[derive(Default)]
struct RecordingWriter {
    bytes: Vec<u8>,
    was_flushed: bool,
}

impl Write for RecordingWriter {
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<usize> {
        self.bytes = buffer.to_vec();
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.was_flushed = true;
        Ok(())
    }
}

#[derive(Default)]
struct InterruptedFlushWriter {
    /// Exact bytes accepted before the flush attempt.
    bytes: Vec<u8>,
    /// Number of flush attempts observed by the writer.
    flush_calls: usize,
}

impl Write for InterruptedFlushWriter {
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<usize> {
        self.bytes
            .extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let call = self.flush_calls;
        self.flush_calls = increment(self.flush_calls);
        if call == 0 {
            return Err(
                io::Error::new(
                    io::ErrorKind::Interrupted,
                    "flush interrupted",
                ),
            );
        }
        Ok(())
    }
}

#[test]
fn complete_write_flushes_exact_bytes_before_returning() {
    let mut writer = RecordingWriter::default();

    let result = write_alpha(&mut writer);

    assert!(result.is_ok());
    assert_eq!(
        writer.bytes,
        b"alpha"
    );
    assert!(writer.was_flushed);
}

#[test]
fn interrupted_flush_is_retried_before_write_completion() {
    let mut writer = InterruptedFlushWriter::default();

    let result = write_alpha(&mut writer);

    assert!(result.is_ok());
    assert_eq!(
        writer.bytes,
        b"alpha"
    );
    assert_eq!(
        writer.flush_calls,
        2
    );
}

#[derive(Default)]
struct RejectingWriter {
    /// Number of write attempts observed by the fixture.
    write_calls: usize,
    /// Number of flush attempts observed by the fixture.
    flush_calls: usize,
}

impl Write for RejectingWriter {
    fn write(
        &mut self,
        _buffer: &[u8],
    ) -> io::Result<usize> {
        self.write_calls = increment(self.write_calls);
        Err(
            io::Error::new(
                io::ErrorKind::BrokenPipe,
                "write rejected",
            ),
        )
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush_calls = increment(self.flush_calls);
        Err(
            io::Error::new(
                io::ErrorKind::BrokenPipe,
                "flush rejected",
            ),
        )
    }
}

#[test]
fn empty_output_does_not_touch_the_writer() {
    let mut writer = RejectingWriter::default();

    let result = write_empty(&mut writer);

    assert!(result.is_ok());
    assert_eq!(
        writer.write_calls,
        0
    );
    assert_eq!(
        writer.flush_calls,
        0
    );
}

#[derive(Default)]
struct PrefixThenDenied {
    bytes: Vec<u8>,
    calls: usize,
}

impl Write for PrefixThenDenied {
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<usize> {
        let call = self.calls;
        self.calls = increment(self.calls);
        if call == 0 {
            let accepted = 2;
            self.bytes
                .extend(
                    buffer
                        .iter()
                        .take(accepted)
                        .copied(),
                );
            return Ok(accepted);
        }
        Err(
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                "blocked",
            ),
        )
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[derive(Default)]
struct FlushDenied {
    /// Bytes accepted before flushing fails.
    bytes: Vec<u8>,
}

impl Write for FlushDenied {
    fn write(
        &mut self,
        buffer: &[u8],
    ) -> io::Result<usize> {
        self.bytes
            .extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Err(
            io::Error::new(
                io::ErrorKind::PermissionDenied,
                "blocked",
            ),
        )
    }
}

#[test]
fn write_and_flush_failures_retain_progress_and_operation_context() {
    let mut write_denied = PrefixThenDenied::default();
    let mut flush_denied = FlushDenied::default();

    let write_result = write_alpha(&mut write_denied);
    let flush_result = write_alpha(&mut flush_denied);

    assert!(write_result.is_err());
    assert!(flush_result.is_err());
    let Some(write_error) = write_result.err() else {
        return;
    };
    let Some(flush_error) = flush_result.err() else {
        return;
    };
    assert_eq!(
        write_error.to_string(),
        "failed to write standard stream after 2 of 5 bytes: blocked"
    );
    assert_eq!(
        flush_error.to_string(),
        "failed to flush standard stream after accepting 5 bytes: blocked"
    );
    assert_eq!(
        write_denied.bytes,
        b"al"
    );
    assert_eq!(
        flush_denied.bytes,
        b"alpha"
    );
}
