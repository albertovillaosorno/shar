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
//   - Log outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Log outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Log outbound adapter.

use std::fs::{File, OpenOptions};
use std::io::Write as _;
use std::path::Path;

/// Maximum bytes retained in one current-run log.
const MAX_LOG_BYTES: u64 = 16 * 1024 * 1024;

/// Bounded latest-run JSONL writer.
#[derive(Debug)]
pub(super) struct RunLog {
    /// Truncated-at-start file for the current process run.
    file: File,
    /// Bytes successfully written in this run.
    bytes_written: u64,
    /// Whether a size-limit marker has already been attempted.
    limit_reported: bool,
    /// Monotonic event sequence within the current process run.
    sequence: u64,
}

impl RunLog {
    /// Create one bounded current-run log.
    ///
    /// # Errors
    ///
    /// Returns an error when the parent directory or log file cannot be
    /// created.
    pub(super) fn open(path: &Path) -> Result<Self, String> {
        if let Some(parent) = path.parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(|error| {
                format!(
                    "failed to create log directory {}: {error}",
                    parent.display()
                )
            })?;
        }
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(|error| {
                format!("failed to open run log {}: {error}", path.display())
            })?;
        Ok(Self {
            file,
            bytes_written: 0,
            limit_reported: false,
            sequence: 0,
        })
    }

    /// Append one complete diagnostic object with a run-local sequence.
    pub(super) fn append(&mut self, body: &str) {
        self.sequence = self.sequence.saturating_add(1);
        let encoded = format!(
            "{{\"sequence\":{},{}\n",
            self.sequence,
            body.strip_prefix('{').unwrap_or(body),
        );
        let encoded_bytes = u64::try_from(encoded.len()).unwrap_or(u64::MAX);
        if self.bytes_written.saturating_add(encoded_bytes) > MAX_LOG_BYTES {
            self.write_limit_marker();
            return;
        }
        if self.file.write_all(encoded.as_bytes()).is_ok() {
            self.bytes_written =
                self.bytes_written.saturating_add(encoded_bytes);
            drop(self.file.flush());
        }
    }

    /// Record one size-limit marker without exceeding the configured cap.
    fn write_limit_marker(&mut self) {
        if self.limit_reported {
            return;
        }
        self.limit_reported = true;
        let marker = format!(
            concat!(
                "{{\"sequence\":{},",
                "\"event\":\"log-limit\",",
                "\"max_bytes\":{}}}\n"
            ),
            self.sequence, MAX_LOG_BYTES,
        );
        let marker_bytes = u64::try_from(marker.len()).unwrap_or(u64::MAX);
        if self.bytes_written.saturating_add(marker_bytes) > MAX_LOG_BYTES {
            return;
        }
        if self.file.write_all(marker.as_bytes()).is_ok() {
            self.bytes_written =
                self.bytes_written.saturating_add(marker_bytes);
            drop(self.file.flush());
        }
    }
}

/// Render an optional total as one JSON number or `null`.
pub(super) fn optional_total_json(total: Option<usize>) -> String {
    total.map_or_else(|| String::from("null"), |value| value.to_string())
}
