// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Failing write sink test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Failing write sink test module.
// - Description:
//   - Implements the declared test module responsibility for command line.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Failing write sink test module.

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
    fn write(&mut self, _stream: OutputStream, _text: &str) -> io::Result<()> {
        let call = self.calls;
        self.calls = self.calls.saturating_add(1);
        if call == self.failure_index {
            return Err(io::Error::new(self.kind, self.message));
        }
        Ok(())
    }
}
