// File:
//   - presentation.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry/model/presentation.rs
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
//   - Compact active-run diagnostics and duration rendering.
// - Must-Not:
//   - Read registry files, mutate run state, or parse CLI arguments.
// - Allows:
//   - Render public-safe local process metadata.
// - Summary:
//   - Human presentation for active pipeline runs.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Human-readable presentation for active pipeline run records.

use super::RunSnapshot;

impl RunSnapshot {
    /// Render one compact human-readable active-run line.
    pub(in super::super) fn render(
        &self,
        now_unix_ms: u64,
    ) -> String {
        let elapsed_seconds = now_unix_ms
            .saturating_sub(self.started_unix_ms)
            .checked_div(1_000)
            .unwrap_or_default();
        let progress = match (
            self.done, self.total,
        ) {
            (Some(done), Some(total)) => format!("{done}/{total}"),
            (Some(done), None) => done.to_string(),
            _ => String::from("unknown"),
        };
        let eta = self
            .eta_seconds
            .map_or_else(
                || String::from("unknown"),
                format_duration,
            );
        let label = self
            .label
            .as_deref()
            .map_or_else(
                || String::from("-"),
                safe_text,
            );
        let item = self
            .item
            .as_deref()
            .map_or_else(
                || String::from("-"),
                safe_text,
            );
        format!(
            concat!(
                "run={} pid={} command={} label={} mode={} state={} ",
                "stage={} progress={} elapsed={} eta={} item={}"
            ),
            self.run_id,
            self.pid,
            safe_text(&self.command),
            label,
            self.mode
                .as_str(),
            self.state
                .as_str(),
            safe_text(&self.stage),
            progress,
            format_duration(elapsed_seconds),
            eta,
            item,
        )
    }
}

/// Render one compact duration without locale-sensitive behavior.
fn format_duration(seconds: u64) -> String {
    let hours = seconds
        .checked_div(3_600)
        .unwrap_or_default();
    let minutes = seconds
        .checked_rem(3_600)
        .unwrap_or_default()
        .checked_div(60)
        .unwrap_or_default();
    let remainder = seconds
        .checked_rem(60)
        .unwrap_or_default();
    if hours > 0 {
        format!("{hours}h{minutes:02}m{remainder:02}s")
    } else if minutes > 0 {
        format!("{minutes}m{remainder:02}s")
    } else {
        format!("{remainder}s")
    }
}

/// Escape control characters before rendering local process metadata.
fn safe_text(value: &str) -> String {
    value
        .chars()
        .flat_map(char::escape_default)
        .collect()
}
