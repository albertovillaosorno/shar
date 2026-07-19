// File:
//   - storage.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry/storage.rs
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
//   - Derived run-record files, exclusive lease creation, and stale cleanup.
// - Must-Not:
//   - Parse CLI arguments, estimate progress, or terminate processes.
// - Allows:
//   - Maintain ignored local runtime state below the working directory.
// - Summary:
//   - Filesystem storage for cooperative pipeline process leases.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - true
//   - Reason: state publication and stale cleanup share one filesystem
//     contract.
//   - Split: separate record decoding if another registry format is added.
//   - Validation: focused registry tests and canonical pipeline validation.
//   - Review: required when state publication or stale cleanup changes.
//

//! Filesystem persistence for the cooperative pipeline run registry.

mod lease;

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use super::model::RunSnapshot;

/// Derived registry root relative to the pipeline working directory.
const DEFAULT_ROOT: &str = "temp/pipeline/runtime";
/// Per-run state file.
const STATE_FILE: &str = "state.json";
/// Lease age after which a crashed process record may be pruned.
const STALE_AFTER_MS: u64 = 120_000;
/// Maximum reads spanning one Windows state-replacement gap.
const STATE_READ_ATTEMPTS: u8 = 20;
/// Delay between state reads while one replacement is in progress.
const STATE_READ_DELAY: Duration = Duration::from_millis(5);

/// Filesystem paths and operations for one working-directory registry.
#[derive(Clone, Debug)]
pub(super) struct RegistryStorage {
    /// Derived registry root.
    root: PathBuf,
}

impl RegistryStorage {
    /// Construct the default working-directory registry.
    pub(super) fn for_current_workspace() -> Self {
        Self::new(PathBuf::from(DEFAULT_ROOT))
    }

    /// Construct one explicit registry root for tests or composition.
    pub(super) const fn new(root: PathBuf) -> Self {
        Self {
            root,
        }
    }

    /// Return current Unix time in milliseconds.
    pub(super) fn now_unix_ms() -> Result<u64, String> {
        let elapsed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(
                |error| format!("system clock precedes Unix epoch: {error}"),
            )?;
        u64::try_from(elapsed.as_millis()).map_err(
            |error| format!("Unix millisecond value overflowed: {error}"),
        )
    }

    /// Create the complete derived registry directory hierarchy.
    pub(super) fn prepare(&self) -> Result<(), String> {
        fs::create_dir_all(self.runs_root()).map_err(
            |error| {
                format!(
                    "pipeline runtime directory creation failed for {}: \
                     {error}",
                    self.root
                        .display()
                )
            },
        )
    }

    /// Read active records and prune expired crash residue.
    pub(super) fn active_runs(
        &self,
        now_unix_ms: u64,
    ) -> Result<Vec<RunSnapshot>, String> {
        self.prepare()?;
        let mut active = Vec::new();
        let entries = fs::read_dir(self.runs_root()).map_err(
            |error| format!("pipeline runtime enumeration failed: {error}"),
        )?;
        for entry_result in entries {
            let entry = entry_result.map_err(
                |error| format!("pipeline runtime entry read failed: {error}"),
            )?;
            let file_type = entry
                .file_type()
                .map_err(
                    |error| {
                        format!(
                            "pipeline runtime entry metadata failed: {error}"
                        )
                    },
                )?;
            if !file_type.is_dir() {
                continue;
            }
            let run_dir = entry.path();
            let mut snapshot = Self::read_snapshot(&run_dir)?;
            if snapshot.is_stale(
                now_unix_ms,
                STALE_AFTER_MS,
            ) {
                self.remove_run(snapshot.run_id())?;
                self.release_exclusive(snapshot.run_id())?;
                continue;
            }
            if self.cancellation_requested(snapshot.run_id()) {
                snapshot.request_cancellation(now_unix_ms);
            }
            active.push(snapshot);
        }
        active.sort_by(
            |left, right| {
                left.run_id()
                    .cmp(right.run_id())
            },
        );
        Ok(active)
    }

    /// Create one new run directory and initial state record.
    pub(super) fn create_run(
        &self,
        snapshot: &RunSnapshot,
    ) -> Result<(), String> {
        self.prepare()?;
        let run_dir = self.run_dir(snapshot.run_id());
        let pending = self
            .root
            .join(
                format!(
                    "pending-{}-{}",
                    snapshot.run_id(),
                    std::process::id()
                ),
            );
        let bytes = snapshot.json_bytes()?;
        fs::create_dir(&pending).map_err(
            |error| {
                format!(
                    "pipeline pending run creation failed for {}: {error}",
                    snapshot.run_id()
                )
            },
        )?;
        if let Err(error) = fs::write(
            pending.join(STATE_FILE),
            bytes,
        ) {
            drop(fs::remove_dir_all(&pending));
            return Err(
                format!("pipeline initial run state write failed: {error}"),
            );
        }
        fs::rename(
            &pending, &run_dir,
        )
        .map_err(
            |error| {
                drop(fs::remove_dir_all(&pending));
                format!("pipeline initial run publication failed: {error}")
            },
        )
    }

    /// Replace one run state record through a same-directory temporary file.
    pub(super) fn write_snapshot(
        &self,
        snapshot: &RunSnapshot,
    ) -> Result<(), String> {
        let run_dir = self.run_dir(snapshot.run_id());
        let state_path = run_dir.join(STATE_FILE);
        let temporary = run_dir.join(
            format!(
                "state.{}.tmp",
                std::process::id()
            ),
        );
        let bytes = snapshot.json_bytes()?;
        fs::write(
            &temporary, bytes,
        )
        .map_err(|error| format!("pipeline run state write failed: {error}"))?;
        match fs::remove_file(&state_path) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => {
                drop(fs::remove_file(&temporary));
                return Err(
                    format!("pipeline prior run state removal failed: {error}"),
                );
            }
        }
        fs::rename(
            &temporary,
            &state_path,
        )
        .map_err(
            |error| {
                drop(fs::remove_file(&temporary));
                format!("pipeline run state publication failed: {error}")
            },
        )
    }

    /// Remove one active-run directory after completion or stale cleanup.
    pub(super) fn remove_run(
        &self,
        run_id: &str,
    ) -> Result<(), String> {
        validate_run_id(run_id)?;
        match fs::remove_dir_all(self.run_dir(run_id)) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(()),
            Err(error) => Err(
                format!("pipeline run cleanup failed for {run_id}: {error}"),
            ),
        }
    }

    /// Return the derived runtime root for diagnostics and tests.
    #[cfg(test)]
    pub(super) fn root(&self) -> &Path {
        &self.root
    }

    /// Return one run directory.
    fn run_dir(
        &self,
        run_id: &str,
    ) -> PathBuf {
        self.runs_root()
            .join(run_id)
    }

    /// Return the active-runs directory.
    fn runs_root(&self) -> PathBuf {
        self.root
            .join("runs")
    }

    /// Read one state record across a bounded atomic replacement window.
    fn read_snapshot(run_dir: &Path) -> Result<RunSnapshot, String> {
        let path = run_dir.join(STATE_FILE);
        for attempt in 0_u8..STATE_READ_ATTEMPTS {
            match fs::read(&path) {
                Ok(bytes) => return RunSnapshot::parse(&bytes),
                Err(error)
                    if matches!(
                        error.kind(),
                        ErrorKind::NotFound | ErrorKind::PermissionDenied
                    ) && attempt < STATE_READ_ATTEMPTS.saturating_sub(1) =>
                {
                    std::thread::sleep(STATE_READ_DELAY);
                }
                Err(error) => {
                    return Err(
                        format!("pipeline run state read failed: {error}"),
                    );
                }
            }
        }
        Err(String::from("pipeline run state read attempts were exhausted"))
    }
}

/// Reject run identities that could escape the derived registry directory.
fn validate_run_id(run_id: &str) -> Result<(), String> {
    let valid = !run_id.is_empty()
        && run_id.len() <= 96
        && run_id
            .bytes()
            .all(
                |byte| {
                    byte.is_ascii_alphanumeric()
                        || matches!(
                            byte,
                            b'-' | b'_'
                        )
                },
            );
    if valid {
        Ok(())
    } else {
        Err(String::from("pipeline run id is invalid"))
    }
}
