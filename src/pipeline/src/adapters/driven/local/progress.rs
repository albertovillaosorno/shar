// File:
//   - progress.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/progress.rs
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
//   - Process-wide extraction progress configuration and stage lifecycle.
// - Must-Not:
//   - Change extraction results or write into generated asset outputs.
// - Allows:
//   - Throttled stderr updates and diagnostic events through the log sink.
// - Split-When:
//   - Split when another progress lifecycle becomes independently reusable.
// - Merge-When:
//   - Another pipeline adapter owns the same stage-progress lifecycle.
// - Summary:
//   - Reports long-running pipeline stage progress without affecting outputs.
// - Description:
//   - Coordinates verbosity, stage counters, elapsed time, ETA rendering, and
//   - local diagnostic logging for extraction commands.
// - Usage:
//   - Installed by the pipeline CLI and advanced by long-running local stages.
// - Defaults:
//   - Detailed terminal progress is selected by the driving CLI.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Pipeline extraction progress lifecycle and run-log coordination.
//!
//! This boundary keeps terminal progress and diagnostic logging outside
//! deterministic extraction outputs. The driving CLI installs one process-wide
//! configuration, and long-running local stages report counters, elapsed time,
//! ETA, and the current item through `StageProgress`.
use std::path::Path;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use self::log::{RunLog, optional_total_json};
use self::render::{format_duration, progress_line, shorten_item};
use super::run_registry::update_current_progress;
use crate::domain::escape_json;

mod log;
mod render;
mod terminal;

/// Minimum interval between detailed terminal and log updates.
const RENDER_INTERVAL: Duration = Duration::from_millis(100);

/// Operator-selected terminal detail.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(in crate::adapters) enum Verbosity {
    /// Continuously show percent, counts, elapsed time, ETA, and current item.
    #[default]
    Detailed,
    /// Show only stage start and completion summaries.
    Minimal,
}

/// Process-wide progress configuration selected by the driving CLI.
#[derive(Debug)]
enum ProgressState {
    /// Installed terminal and diagnostic settings.
    Installed {
        /// Live-detail level for stderr rendering.
        verbosity: Verbosity,
        /// Bounded current-run diagnostic log.
        log: Option<Mutex<RunLog>>,
    },
}

impl ProgressState {
    /// Return the installed terminal detail level.
    const fn verbosity(&self) -> Verbosity {
        match self {
            Self::Installed {
                verbosity,
                ..
            } => *verbosity,
        }
    }

    /// Return the installed log sink when logging is enabled.
    const fn log(&self) -> Option<&Mutex<RunLog>> {
        match self {
            Self::Installed {
                log,
                ..
            } => log.as_ref(),
        }
    }
}

/// One configuration for the current pipeline process.
static STATE: OnceLock<ProgressState> = OnceLock::new();

/// Install progress reporting exactly once for the current process.
///
/// # Errors
///
/// Returns an error when the log cannot be created or when reporting was
/// already configured.
pub(in crate::adapters) fn install(
    verbosity: Verbosity,
    log_file: Option<&Path>,
) -> Result<(), String> {
    let log = log_file
        .map(RunLog::open)
        .transpose()?
        .map(Mutex::new);
    STATE
        .set(
            ProgressState::Installed {
                verbosity,
                log,
            },
        )
        .map_err(|_state| String::from("progress already configured"))
}

/// Live reporter for one named stage loop.
#[derive(Debug)]
pub(super) struct StageProgress {
    /// Stable stage label shown on every rendered line.
    stage: String,
    /// Total number of items, or `None` while discovery is incremental.
    total: Option<usize>,
    /// Items reported so far.
    done: usize,
    /// Stage start instant used for elapsed and ETA arithmetic.
    started: Instant,
    /// Last live render instant used by the throttle.
    last_render: Instant,
}

impl StageProgress {
    /// Begin reporting one stage with a known item total.
    #[must_use]
    pub(super) fn begin(
        stage: impl Into<String>,
        total: usize,
    ) -> Self {
        Self::start(
            stage.into(),
            Some(total),
        )
    }

    /// Begin reporting one stage whose total is discovered while processing.
    #[must_use]
    pub(super) fn begin_unknown(stage: impl Into<String>) -> Self {
        Self::start(
            stage.into(),
            None,
        )
    }

    /// Construct and announce one stage reporter.
    fn start(
        stage: String,
        total: Option<usize>,
    ) -> Self {
        let now = Instant::now();
        if STATE
            .get()
            .is_some()
        {
            match total {
                Some(item_total) => {
                    terminal::line(
                        &format!("[{stage}] starting: {item_total} items"),
                    );
                }
                None => {
                    terminal::line(
                        &format!("[{stage}] starting: discovering items"),
                    );
                }
            }
            append_event(
                &format!(
                    concat!(
                        "{{\"event\":\"begin\",",
                        "\"stage\":\"{}\",",
                        "\"total\":{}}}"
                    ),
                    escape_json(&stage),
                    optional_total_json(total),
                ),
            );
        }
        drop(
            update_current_progress(
                &stage,
                Some(0),
                total,
                None,
            ),
        );
        Self {
            stage,
            total,
            done: 0,
            started: now,
            last_render: now
                .checked_sub(RENDER_INTERVAL)
                .unwrap_or(now),
        }
    }

    /// Report that one named item is being processed.
    ///
    /// The counter advances before work begins so a slow decoder or external
    /// tool remains identifiable from the terminal and diagnostic log.
    pub(super) fn advance(
        &mut self,
        item: &str,
    ) {
        self.done = self
            .done
            .saturating_add(1);
        drop(
            update_current_progress(
                &self.stage,
                Some(self.done),
                self.total,
                Some(item),
            ),
        );
        let Some(state) = STATE.get() else {
            return;
        };
        if state.verbosity() != Verbosity::Detailed {
            return;
        }
        let now = Instant::now();
        let render_due =
            now.duration_since(self.last_render) >= RENDER_INTERVAL;
        let complete = self
            .total
            .is_some_and(|item_total| self.done >= item_total);
        if !render_due && !complete && self.done != 1 {
            return;
        }
        self.last_render = now;
        let elapsed = self
            .started
            .elapsed();
        let display_item = shorten_item(item);
        let line = progress_line(
            &self.stage,
            self.done,
            self.total,
            elapsed,
            &display_item,
        );
        terminal::live(&line);
        append_event(
            &format!(
                concat!(
                    "{{\"event\":\"advance\",",
                    "\"stage\":\"{}\",",
                    "\"done\":{},",
                    "\"total\":{},",
                    "\"item\":\"{}\"}}"
                ),
                escape_json(&self.stage),
                self.done,
                optional_total_json(self.total),
                escape_json(item),
            ),
        );
    }

    /// Finish the stage and render its closing summary.
    pub(super) fn finish(mut self) {
        let Some(state) = STATE.get() else {
            return;
        };
        if let Some(item_total) = self.total {
            self.done = item_total;
        }
        let elapsed = self
            .started
            .elapsed()
            .as_secs();
        if state.verbosity() == Verbosity::Detailed {
            terminal::clear_live();
        }
        match self.total {
            Some(item_total) => {
                terminal::line(
                    &format!(
                        "[{}] done: {}/{} items in {}",
                        self.stage,
                        self.done,
                        item_total,
                        format_duration(elapsed),
                    ),
                );
            }
            None => {
                terminal::line(
                    &format!(
                        "[{}] done: {} items in {}",
                        self.stage,
                        self.done,
                        format_duration(elapsed),
                    ),
                );
            }
        }
        drop(
            update_current_progress(
                &self.stage,
                Some(self.done),
                self.total,
                None,
            ),
        );
        append_event(
            &format!(
                concat!(
                    "{{\"event\":\"end\",",
                    "\"stage\":\"{}\",",
                    "\"done\":{},",
                    "\"total\":{},",
                    "\"elapsed_seconds\":{}}}"
                ),
                escape_json(&self.stage),
                self.done,
                optional_total_json(self.total),
                elapsed,
            ),
        );
    }
}

/// Append one progress event when a local run log is configured.
fn append_event(body: &str) {
    let Some(state) = STATE.get() else {
        return;
    };
    let Some(log) = state.log() else {
        return;
    };
    if let Ok(mut log_guard) = log.lock() {
        log_guard.append(body);
    }
}
