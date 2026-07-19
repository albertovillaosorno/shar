// File:
//   - run_registry.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry.rs
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
//   - Cooperative run acquisition, active listing, cancellation, and cleanup.
// - Must-Not:
//   - Parse CLI syntax, execute pipeline stages, or force-kill processes.
// - Allows:
//   - Compose local lease storage with one heartbeat supervisor.
// - Summary:
//   - Active-run registry facade for the pipeline process boundary.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - true
//   - Reason: acquisition and guard cleanup form one transactional lifecycle.
//   - Split: separate conflict presentation if another driving adapter uses it.
//   - Validation: focused registry tests and canonical pipeline validation.
//   - Review: required when acquisition or cleanup ordering changes.
//

//! Cooperative active-run registry for pipeline command processes.

use std::marker::PhantomData;
#[cfg(test)]
use std::path::PathBuf;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use self::current::{clear_current_control, install_current_control};
use self::model::RunSnapshot;
use self::storage::RegistryStorage;
use self::supervisor::{RunControl, RunSupervisor};

mod current;
mod error;
mod model;
mod storage;
mod supervisor;

#[cfg(test)]
#[path = "run_registry_tests.rs"]
mod tests;

/// Per-process nonce preventing same-millisecond run-id collisions.
static RUN_NONCE: AtomicU64 = AtomicU64::new(0);

pub(in crate::adapters) use current::{
    check_cancellation, update_current_progress,
};
pub(in crate::adapters) use error::RunStartError;
pub(in crate::adapters) use model::RunMode;

/// Registry access for one pipeline working directory.
#[derive(Clone, Debug)]
pub(in crate::adapters) struct RunRegistry {
    /// Local derived-state storage.
    storage: RegistryStorage,
}

impl RunRegistry {
    /// Construct the normal working-directory registry.
    pub(in crate::adapters) fn current_workspace() -> Self {
        Self {
            storage: RegistryStorage::for_current_workspace(),
        }
    }

    /// Construct one explicit registry root for tests.
    #[cfg(test)]
    const fn from_root(root: PathBuf) -> Self {
        Self {
            storage: RegistryStorage::new(root),
        }
    }

    /// Return compact human-readable lines for every active run.
    pub(in crate::adapters) fn active_lines(
        &self
    ) -> Result<Vec<String>, String> {
        let now = RegistryStorage::now_unix_ms()?;
        self.storage
            .active_runs(now)
            .map(
                |runs| {
                    runs.iter()
                        .map(|run| run.render(now))
                        .collect()
                },
            )
    }

    /// Request cancellation for one run id or every active run.
    pub(in crate::adapters) fn request_cancel(
        &self,
        target: &str,
    ) -> Result<Vec<String>, String> {
        if target != "all" {
            self.storage
                .request_cancel(target)?;
            return Ok(vec![target.to_owned()]);
        }
        let now = RegistryStorage::now_unix_ms()?;
        let runs = self
            .storage
            .active_runs(now)?;
        let mut requested = Vec::with_capacity(runs.len());
        for run in runs {
            self.storage
                .request_cancel(run.run_id())?;
            requested.push(
                run.run_id()
                    .to_owned(),
            );
        }
        Ok(requested)
    }

    /// Acquire one active-run lease and start its heartbeat supervisor.
    pub(in crate::adapters) fn start(
        &self,
        command: &str,
        label: Option<String>,
        mode: RunMode,
    ) -> Result<RunGuard, RunStartError> {
        let now =
            RegistryStorage::now_unix_ms().map_err(RunStartError::failure)?;
        let run_id = new_run_id(now);
        let exclusive = self.acquire_start_lease(
            now, &run_id, mode,
        )?;
        let snapshot = RunSnapshot::new(
            run_id.clone(),
            std::process::id(),
            command.to_owned(),
            label,
            mode,
            now,
        );
        if let Err(error) = self
            .storage
            .create_run(&snapshot)
        {
            self.rollback_start(
                &run_id, exclusive, false,
            );
            return Err(RunStartError::failure(error));
        }
        let control = RunControl::new(
            self.storage
                .clone(),
            snapshot,
        );
        if let Err(error) = install_current_control(Arc::clone(&control)) {
            self.rollback_start(
                &run_id, exclusive, false,
            );
            return Err(RunStartError::failure(error));
        }
        let supervisor = RunSupervisor::spawn(Arc::clone(&control)).map_err(
            |error| {
                self.rollback_start(
                    &run_id, exclusive, true,
                );
                RunStartError::failure(error)
            },
        )?;
        Ok(
            RunGuard {
                storage: self
                    .storage
                    .clone(),
                control,
                supervisor,
                run_id,
                exclusive,
                finished: false,
                thread_bound: PhantomData,
            },
        )
    }

    /// Validate active state and acquire the default exclusive lease when used.
    fn acquire_start_lease(
        &self,
        now_unix_ms: u64,
        run_id: &str,
        mode: RunMode,
    ) -> Result<bool, RunStartError> {
        let active = self
            .storage
            .active_runs(now_unix_ms)
            .map_err(RunStartError::failure)?;
        if mode == RunMode::Exclusive && !active.is_empty() {
            return Err(
                RunStartError::conflict(
                    &active,
                    now_unix_ms,
                ),
            );
        }
        if mode == RunMode::Concurrent {
            return Ok(false);
        }
        self.storage
            .acquire_exclusive(run_id)
            .map_err(RunStartError::failure)?;
        let raced = match self
            .storage
            .active_runs(now_unix_ms)
        {
            Ok(raced) => raced,
            Err(error) => {
                drop(
                    self.storage
                        .release_exclusive(run_id),
                );
                return Err(RunStartError::failure(error));
            }
        };
        if raced.is_empty() {
            Ok(true)
        } else {
            drop(
                self.storage
                    .release_exclusive(run_id),
            );
            Err(
                RunStartError::conflict(
                    &raced,
                    now_unix_ms,
                ),
            )
        }
    }

    /// Remove partially installed run state after a failed start transaction.
    fn rollback_start(
        &self,
        run_id: &str,
        exclusive: bool,
        clear_control: bool,
    ) {
        if clear_control {
            clear_current_control(run_id);
        }
        drop(
            self.storage
                .remove_run(run_id),
        );
        if exclusive {
            drop(
                self.storage
                    .release_exclusive(run_id),
            );
        }
    }
}

/// One acquired process lease released on every normal return path.
#[derive(Debug)]
pub(in crate::adapters) struct RunGuard {
    /// Registry storage used by this run.
    storage: RegistryStorage,
    /// Shared supervisor control.
    control: Arc<RunControl>,
    /// Joinable heartbeat worker.
    supervisor: RunSupervisor,
    /// Stable run identity.
    run_id: String,
    /// Whether this run owns the exclusive create-new lease.
    exclusive: bool,
    /// Whether explicit cleanup already completed.
    finished: bool,
    /// Prevents moving a thread-local run guard to another thread.
    thread_bound: PhantomData<Rc<()>>,
}

impl RunGuard {
    /// Return the stable run identity.
    pub(in crate::adapters) fn run_id(&self) -> &str {
        &self.run_id
    }

    /// Finish heartbeat and remove active derived state.
    pub(in crate::adapters) fn finish(mut self) -> Result<(), String> {
        let result = self.cleanup();
        self.finished = true;
        result
    }

    /// Stop the supervisor and release every owned derived-state resource.
    fn cleanup(&mut self) -> Result<(), String> {
        let mut failures = Vec::new();
        if let Err(error) = self
            .supervisor
            .shutdown(&self.control)
        {
            failures.push(error);
        }
        if let Some(error) = self
            .control
            .take_background_error()
        {
            failures.push(error);
        }
        clear_current_control(&self.run_id);
        if let Err(error) = self
            .storage
            .remove_run(&self.run_id)
        {
            failures.push(error);
        }
        if self.exclusive
            && let Err(error) = self
                .storage
                .release_exclusive(&self.run_id)
        {
            failures.push(error);
        }
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures.join("; "))
        }
    }
}

impl Drop for RunGuard {
    fn drop(&mut self) {
        if !self.finished {
            drop(self.cleanup());
        }
    }
}

/// Construct one portable process-local run identity.
fn new_run_id(now_unix_ms: u64) -> String {
    let nonce = RUN_NONCE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    format!(
        "run-{now_unix_ms}-{}-{nonce}",
        std::process::id()
    )
}
