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
//   - Error outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Error outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Error outbound adapter.

use super::model::RunSnapshot;

/// One run-start failure with optional active-run diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::adapters) struct RunStartError {
    /// Primary failure message.
    message: String,
    /// Active-run lines supporting one conflict diagnosis.
    active_lines: Vec<String>,
}

impl RunStartError {
    /// Construct one storage or supervisor failure.
    pub(super) const fn failure(message: String) -> Self {
        Self {
            message,
            active_lines: Vec::new(),
        }
    }

    /// Construct one active-run conflict with complete bounded diagnostics.
    pub(super) fn conflict(active: &[RunSnapshot], now_unix_ms: u64) -> Self {
        Self {
            message: String::from("another pipeline run is already active"),
            active_lines: active
                .iter()
                .map(|run| run.render(now_unix_ms))
                .collect(),
        }
    }

    /// Return the primary failure message.
    pub(in crate::adapters) fn message(&self) -> &str {
        &self.message
    }

    /// Return active-run evidence associated with this failure.
    pub(in crate::adapters) fn active_lines(&self) -> &[String] {
        &self.active_lines
    }
}
