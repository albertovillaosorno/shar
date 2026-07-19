// File:
//   - supervisor.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry/supervisor.rs
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
//   - In-process heartbeat, cancellation observation, and progress publication.
// - Must-Not:
//   - Parse commands, create initial leases, or force process termination.
// - Allows:
//   - Coordinate one background supervisor with cooperative checkpoints.
// - Summary:
//   - Runtime control and heartbeat thread for one pipeline run.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - true
//   - Reason: control state and its sole heartbeat worker share one lifecycle.
//   - Split: separate error telemetry if another background worker is added.
//   - Validation: focused registry tests and canonical pipeline validation.
//   - Review: required when heartbeat cadence or cancellation semantics change.
//

//! In-process supervisor for one cooperative pipeline run.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use super::model::RunSnapshot;
use super::storage::RegistryStorage;

/// Lease refresh and cancellation polling interval.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(1);

/// Shared mutable control for one active process.
#[derive(Debug)]
pub(super) struct RunControl {
    /// Filesystem registry used by this process.
    storage: RegistryStorage,
    /// Current serializable active-run state.
    snapshot: Mutex<RunSnapshot>,
    /// Fast cancellation state observed by cooperative checkpoints.
    cancellation_requested: AtomicBool,
    /// Heartbeat shutdown request.
    stop_requested: AtomicBool,
    /// First background persistence failure.
    background_error: Mutex<Option<String>>,
}

impl RunControl {
    /// Construct one shared process control.
    pub(super) fn new(
        storage: RegistryStorage,
        snapshot: RunSnapshot,
    ) -> Arc<Self> {
        Arc::new(
            Self {
                storage,
                snapshot: Mutex::new(snapshot),
                cancellation_requested: AtomicBool::new(false),
                stop_requested: AtomicBool::new(false),
                background_error: Mutex::new(None),
            },
        )
    }

    /// Return the stable run identity.
    pub(super) fn run_id(&self) -> Result<String, String> {
        let snapshot = self
            .snapshot
            .lock()
            .map_err(
                |_poisoned| String::from("pipeline run state lock is poisoned"),
            )?;
        Ok(
            snapshot
                .run_id()
                .to_owned(),
        )
    }

    /// Publish one progress checkpoint and refresh cancellation state.
    pub(super) fn update_progress(
        &self,
        stage: &str,
        done: Option<u64>,
        total: Option<u64>,
        item: Option<&str>,
    ) -> Result<(), String> {
        self.refresh_cancellation()?;
        let now = RegistryStorage::now_unix_ms()?;
        let mut snapshot = self
            .snapshot
            .lock()
            .map_err(
                |_poisoned| String::from("pipeline run state lock is poisoned"),
            )?;
        snapshot.update_progress(
            stage, done, total, item, now,
        );
        drop(snapshot);
        Ok(())
    }

    /// Refresh heartbeat and observe an external cancellation marker.
    pub(super) fn heartbeat(&self) -> Result<(), String> {
        self.refresh_cancellation()?;
        let now = RegistryStorage::now_unix_ms()?;
        let published = {
            let mut snapshot = self
                .snapshot
                .lock()
                .map_err(
                    |_poisoned| {
                        String::from("pipeline run state lock is poisoned")
                    },
                )?;
            snapshot.heartbeat(now);
            snapshot.clone()
        };
        self.storage
            .write_snapshot(&published)
    }

    /// Return whether cooperative cancellation has been requested.
    pub(super) fn is_cancelled(&self) -> Result<bool, String> {
        self.refresh_cancellation()?;
        Ok(
            self.cancellation_requested
                .load(Ordering::Acquire),
        )
    }

    /// Ask the heartbeat worker to stop.
    pub(super) fn request_stop(&self) {
        self.stop_requested
            .store(
                true,
                Ordering::Release,
            );
    }

    /// Return whether the heartbeat worker should stop.
    fn should_stop(&self) -> bool {
        self.stop_requested
            .load(Ordering::Acquire)
    }

    /// Return and clear the first background error.
    pub(super) fn take_background_error(&self) -> Option<String> {
        self.background_error
            .lock()
            .ok()
            .and_then(|mut error| error.take())
    }

    /// Observe the cancellation marker and publish the lifecycle transition.
    fn refresh_cancellation(&self) -> Result<(), String> {
        if self
            .cancellation_requested
            .load(Ordering::Acquire)
        {
            return Ok(());
        }
        let run_id = self.run_id()?;
        if !self
            .storage
            .cancellation_requested(&run_id)
        {
            return Ok(());
        }
        self.cancellation_requested
            .store(
                true,
                Ordering::Release,
            );
        let now = RegistryStorage::now_unix_ms()?;
        let mut snapshot = self
            .snapshot
            .lock()
            .map_err(
                |_poisoned| String::from("pipeline run state lock is poisoned"),
            )?;
        snapshot.request_cancellation(now);
        drop(snapshot);
        Ok(())
    }

    /// Preserve the first supervisor failure for the owning guard.
    fn record_background_error(
        &self,
        error: String,
    ) {
        if let Ok(mut slot) = self
            .background_error
            .lock()
            && slot.is_none()
        {
            *slot = Some(error);
        }
    }
}

/// Joinable heartbeat worker owned by one run guard.
#[derive(Debug)]
pub(super) struct RunSupervisor {
    /// Background worker handle.
    worker: Option<JoinHandle<()>>,
}

impl RunSupervisor {
    /// Spawn one named heartbeat worker.
    pub(super) fn spawn(control: Arc<RunControl>) -> Result<Self, String> {
        let worker = thread::Builder::new()
            .name(String::from("pipeline-run-supervisor"))
            .spawn(move || supervise(&control))
            .map_err(
                |error| {
                    format!("pipeline run supervisor spawn failed: {error}")
                },
            )?;
        Ok(
            Self {
                worker: Some(worker),
            },
        )
    }

    /// Stop and join the heartbeat worker.
    pub(super) fn shutdown(
        &mut self,
        control: &RunControl,
    ) -> Result<(), String> {
        control.request_stop();
        let Some(worker) = self
            .worker
            .take()
        else {
            return Ok(());
        };
        worker
            .thread()
            .unpark();
        worker
            .join()
            .map_err(|_panic| String::from("pipeline run supervisor panicked"))
    }
}

/// Refresh one run lease until the owning guard requests shutdown.
fn supervise(control: &RunControl) {
    while !control.should_stop() {
        if let Err(error) = control.heartbeat() {
            control.record_background_error(error);
        }
        thread::park_timeout(HEARTBEAT_INTERVAL);
    }
}
