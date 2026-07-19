// File:
//   - error.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry/error.rs
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
//   - Active-run acquisition failures and conflict evidence.
// - Must-Not:
//   - Read registry files, acquire leases, or render command usage.
// - Allows:
//   - Preserve bounded active-run diagnostics for the driving adapter.
// - Summary:
//   - Typed pipeline run-start failure.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Typed active-run start failures.

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
    pub(super) fn conflict(
        active: &[RunSnapshot],
        now_unix_ms: u64,
    ) -> Self {
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
