// File:
//   - model.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry/model.rs
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
//   - Portable active-run state and progress arithmetic.
// - Must-Not:
//   - Read directories, create locks, spawn threads, or terminate processes.
// - Allows:
//   - Carry local derived runtime records behind typed values.
// - Summary:
//   - Process-registry state model for cooperative pipeline execution.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Portable state values for the cooperative pipeline run registry.

mod codec;
mod presentation;

/// Stable active-run record schema.
const SCHEMA: &str = "shar.pipeline-active-run.v1";

/// Cooperative execution mode selected by one invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::adapters) enum RunMode {
    /// Refuse to start while any other active run exists.
    Exclusive,
    /// Permit this explicitly acknowledged run beside other active runs.
    Concurrent,
}

impl RunMode {
    /// Return the stable serialized mode identity.
    pub(super) const fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Concurrent => "concurrent",
        }
    }

    /// Parse one stable serialized mode identity.
    fn parse(value: &str) -> Option<Self> {
        match value {
            "exclusive" => Some(Self::Exclusive),
            "concurrent" => Some(Self::Concurrent),
            _ => None,
        }
    }
}

/// Current cooperative lifecycle state of one active process.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunState {
    /// Process is running normally.
    Running,
    /// Another invocation requested cancellation.
    CancellationRequested,
}

impl RunState {
    /// Return the stable serialized state identity.
    const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::CancellationRequested => "cancellation-requested",
        }
    }

    /// Parse one stable serialized state identity.
    fn parse(value: &str) -> Option<Self> {
        match value {
            "running" => Some(Self::Running),
            "cancellation-requested" => Some(Self::CancellationRequested),
            _ => None,
        }
    }
}

/// Complete local state for one active pipeline process.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RunSnapshot {
    /// Stable run identity.
    run_id: String,
    /// Operating-system process identifier.
    pid: u32,
    /// Exact pipeline command identity.
    command: String,
    /// Optional caller-provided display label.
    label: Option<String>,
    /// Cooperative concurrency mode.
    mode: RunMode,
    /// Current lifecycle state.
    state: RunState,
    /// Process start time in Unix milliseconds.
    started_unix_ms: u64,
    /// Last lease heartbeat in Unix milliseconds.
    heartbeat_unix_ms: u64,
    /// Current stage start time in Unix milliseconds.
    stage_started_unix_ms: u64,
    /// Current stage identity.
    stage: String,
    /// Completed stage items when known.
    done: Option<u64>,
    /// Total stage items when known.
    total: Option<u64>,
    /// Current item identity when available.
    item: Option<String>,
    /// Estimated remaining seconds when calculable.
    eta_seconds: Option<u64>,
}

impl RunSnapshot {
    /// Construct one new active-run state.
    pub(super) fn new(
        run_id: String,
        pid: u32,
        command: String,
        label: Option<String>,
        mode: RunMode,
        now_unix_ms: u64,
    ) -> Self {
        Self {
            run_id,
            pid,
            stage: command.clone(),
            command,
            label,
            mode,
            state: RunState::Running,
            started_unix_ms: now_unix_ms,
            heartbeat_unix_ms: now_unix_ms,
            stage_started_unix_ms: now_unix_ms,
            done: None,
            total: None,
            item: None,
            eta_seconds: None,
        }
    }

    /// Refresh the lease heartbeat.
    pub(super) const fn heartbeat(
        &mut self,
        now_unix_ms: u64,
    ) {
        self.heartbeat_unix_ms = now_unix_ms;
    }

    /// Mark a cooperative cancellation request.
    pub(super) const fn request_cancellation(
        &mut self,
        now_unix_ms: u64,
    ) {
        self.state = RunState::CancellationRequested;
        self.heartbeat(now_unix_ms);
    }

    /// Update the current stage and its progress evidence.
    pub(super) fn update_progress(
        &mut self,
        stage: &str,
        done: Option<u64>,
        total: Option<u64>,
        item: Option<&str>,

        now_unix_ms: u64,
    ) {
        if self.stage != stage {
            stage.clone_into(&mut self.stage);
            self.stage_started_unix_ms = now_unix_ms;
        }
        self.done = done;
        self.total = total;
        self.item = item.map(str::to_owned);
        self.eta_seconds = estimate_eta_seconds(
            self.stage_started_unix_ms,
            now_unix_ms,
            done,
            total,
        );
        self.heartbeat(now_unix_ms);
    }

    /// Return whether the lease is older than the supplied threshold.
    pub(super) const fn is_stale(
        &self,
        now_unix_ms: u64,
        stale_after_ms: u64,
    ) -> bool {
        now_unix_ms.saturating_sub(self.heartbeat_unix_ms) > stale_after_ms
    }

    /// Return the stable run identity.
    pub(super) fn run_id(&self) -> &str {
        &self.run_id
    }
}

/// Estimate remaining seconds from current stage throughput.
fn estimate_eta_seconds(
    stage_started_unix_ms: u64,
    now_unix_ms: u64,
    done: Option<u64>,
    total: Option<u64>,
) -> Option<u64> {
    let completed = done?;
    let expected = total?;
    if completed == 0 || completed >= expected {
        return (completed >= expected).then_some(0);
    }
    let elapsed_ms = now_unix_ms.saturating_sub(stage_started_unix_ms);
    let remaining = expected.saturating_sub(completed);
    let numerator =
        u128::from(elapsed_ms).checked_mul(u128::from(remaining))?;
    let remaining_ms = numerator.checked_div(u128::from(completed))?;
    u64::try_from(remaining_ms)
        .ok()?
        .checked_div(1_000)
}
