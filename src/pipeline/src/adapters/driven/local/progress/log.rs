// File:
//   - log.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/progress/log.rs
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
//   - Bounded JSONL persistence for the current pipeline run.
// - Must-Not:
//   - Fail extraction because diagnostics cannot be appended after startup.
// - Allows:
//   - Create parent directories, truncate stale logs, append, and flush events.
// - Split-When:
//   - Split when log rotation or multiple durable sinks become required.
// - Merge-When:
//   - The progress facade can own persistence without violating SRP.
// - Summary:
//   - Persists one bounded latest-run pipeline diagnostic log.
// - Description:
//   - Creates and writes a size-limited JSONL file for progress diagnostics.
// - Usage:
//   - Owned behind a mutex by the local progress adapter.
// - Defaults:
//   - The configured file is truncated at the start of every process run.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Persists one bounded latest-run diagnostic log for pipeline progress.
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
            && !parent
                .as_os_str()
                .is_empty()
        {
            std::fs::create_dir_all(parent).map_err(
                |error| {
                    format!(
                        "failed to create log directory {}: {error}",
                        parent.display()
                    )
                },
            )?;
        }
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .map_err(
                |error| {
                    format!(
                        "failed to open run log {}: {error}",
                        path.display()
                    )
                },
            )?;
        Ok(
            Self {
                file,
                bytes_written: 0,
                limit_reported: false,
                sequence: 0,
            },
        )
    }

    /// Append one complete diagnostic object with a run-local sequence.
    pub(super) fn append(
        &mut self,
        body: &str,
    ) {
        self.sequence = self
            .sequence
            .saturating_add(1);
        let encoded = format!(
            "{{\"sequence\":{},{}\n",
            self.sequence,
            body.strip_prefix('{')
                .unwrap_or(body),
        );
        let encoded_bytes = u64::try_from(encoded.len()).unwrap_or(u64::MAX);
        if self
            .bytes_written
            .saturating_add(encoded_bytes)
            > MAX_LOG_BYTES
        {
            self.write_limit_marker();
            return;
        }
        if self
            .file
            .write_all(encoded.as_bytes())
            .is_ok()
        {
            self.bytes_written = self
                .bytes_written
                .saturating_add(encoded_bytes);
            drop(
                self.file
                    .flush(),
            );
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
        if self
            .bytes_written
            .saturating_add(marker_bytes)
            > MAX_LOG_BYTES
        {
            return;
        }
        if self
            .file
            .write_all(marker.as_bytes())
            .is_ok()
        {
            self.bytes_written = self
                .bytes_written
                .saturating_add(marker_bytes);
            drop(
                self.file
                    .flush(),
            );
        }
    }
}

/// Render an optional total as one JSON number or `null`.
pub(super) fn optional_total_json(total: Option<usize>) -> String {
    total.map_or_else(
        || String::from("null"),
        |value| value.to_string(),
    )
}
