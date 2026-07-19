// File:
//   - current.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry/current.rs
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
//   - Current-process registry installation, progress publication, and checks.
// - Must-Not:
//   - Acquire filesystem leases, parse CLI syntax, or force termination.
// - Allows:
//   - Expose cooperative checkpoints to local pipeline stages.
// - Summary:
//   - Current pipeline run control bridge.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Current-process bridge to the cooperative run supervisor.

use std::cell::RefCell;
use std::sync::Arc;

use super::supervisor::RunControl;
use crate::domain::PipelineError;

thread_local! {
    /// Current-thread control used by progress and cancellation checkpoints.
    static CURRENT_CONTROL: RefCell<Option<Arc<RunControl>>> =
        const { RefCell::new(None) };
}

/// Publish the current stage and optional item counts to the active registry.
pub(in crate::adapters) fn update_current_progress(
    stage: &str,
    done_count: Option<usize>,
    total_count: Option<usize>,
    item: Option<&str>,
) -> Result<(), String> {
    let Some(control) = current_control()? else {
        return Ok(());
    };
    let done_u64 = done_count
        .map(u64::try_from)
        .transpose()
        .map_err(
            |error| format!("pipeline progress count overflowed: {error}"),
        )?;
    let total_u64 = total_count
        .map(u64::try_from)
        .transpose()
        .map_err(
            |error| format!("pipeline progress total overflowed: {error}"),
        )?;
    control.update_progress(
        stage, done_u64, total_u64, item,
    )
}

/// Fail one cooperative checkpoint after cancellation was requested.
pub(in crate::adapters) fn check_cancellation() -> Result<(), PipelineError> {
    let maybe_control = current_control().map_err(PipelineError::new)?;
    let Some(control) = maybe_control else {
        return Ok(());
    };
    let cancelled = control
        .is_cancelled()
        .map_err(PipelineError::new)?;
    if cancelled {
        Err(
            PipelineError::new(
                format!(
                    "pipeline run {} cancelled by request",
                    control
                        .run_id()
                        .map_err(PipelineError::new)?
                ),
            ),
        )
    } else {
        Ok(())
    }
}

/// Return the installed current-thread control without retaining a borrow.
fn current_control() -> Result<Option<Arc<RunControl>>, String> {
    CURRENT_CONTROL
        .try_with(
            |state| {
                state
                    .try_borrow()
                    .map(|control| control.clone())
                    .map_err(
                        |_borrowed| {
                            String::from(
                                "pipeline current-run state is borrowed",
                            )
                        },
                    )
            },
        )
        .map_err(
            |_destroyed| {
                String::from("pipeline current-run thread state is unavailable")
            },
        )?
}

/// Install one current-thread control exactly once per active invocation.
pub(super) fn install_current_control(
    control: Arc<RunControl>
) -> Result<(), String> {
    CURRENT_CONTROL
        .try_with(
            |state| {
                let mut slot = state
                    .try_borrow_mut()
                    .map_err(
                        |_borrowed| {
                            String::from(
                                "pipeline current-run state is borrowed",
                            )
                        },
                    )?;
                if slot.is_some() {
                    return Err(
                        String::from(
                            "pipeline run control is already installed",
                        ),
                    );
                }
                *slot = Some(control);
                Ok(())
            },
        )
        .map_err(
            |_destroyed| {
                String::from("pipeline current-run thread state is unavailable")
            },
        )?
}

/// Clear current-thread control only when the expected run still owns it.
pub(super) fn clear_current_control(run_id: &str) {
    let _access = CURRENT_CONTROL.try_with(
        |state| {
            let Ok(mut slot) = state.try_borrow_mut() else {
                return;
            };
            let owned = slot
                .as_ref()
                .and_then(
                    |control| {
                        control
                            .run_id()
                            .ok()
                    },
                )
                .is_some_and(|current| current == run_id);
            if owned {
                *slot = None;
            }
        },
    );
}
