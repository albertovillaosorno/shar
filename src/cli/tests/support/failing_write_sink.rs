// File:
//   - failing_write_sink.rs
// Path:
//   - src/cli/tests/support/failing_write_sink.rs
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
//   - A deterministic sink fixture that rejects one selected write.
// - Must-Not:
//   - Encode production retry, stream, or presentation policy.
// - Allows:
//   - Count writes and return one configured I/O error.
// - Split-When:
//   - Another sink behavior needs independent state or assertions.
// - Merge-When:
//   - The fixture is used by only one integration test.
// - Summary:
//   - Configurable failed-write integration fixture.
// - Description:
//   - Removes repeated counted-write failure implementations from CLI tests.
// - Usage:
//   - Loaded only by tests that require a selected failed write.
// - Defaults:
//   - No defaults; callers provide the index, kind, and message.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Deterministic selected-write failure fixture.

use std::io;

use schoenwald_cli::{OutputSink, OutputStream};

/// Output sink that fails one selected write.
#[derive(Debug, Clone, Copy)]
pub struct FailingWriteSink {
    calls: usize,
    failure_index: usize,
    kind: io::ErrorKind,
    message: &'static str,
}

impl FailingWriteSink {
    /// Construct a sink that rejects one zero-based write index.
    #[must_use]
    pub const fn new(
        failure_index: usize,
        kind: io::ErrorKind,
        message: &'static str,
    ) -> Self {
        Self {
            calls: 0,
            failure_index,
            kind,
            message,
        }
    }
}

impl OutputSink for FailingWriteSink {
    fn write(
        &mut self,
        _stream: OutputStream,
        _text: &str,
    ) -> io::Result<()> {
        let call = self.calls;
        self.calls = self
            .calls
            .saturating_add(1);
        if call == self.failure_index {
            return Err(
                io::Error::new(
                    self.kind,
                    self.message,
                ),
            );
        }
        Ok(())
    }
}
