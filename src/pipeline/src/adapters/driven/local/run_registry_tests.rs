// File:
//   - run_registry_tests.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry_tests.rs
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
//   - Deterministic cooperative run-registry regression coverage.
// - Must-Not:
//   - Depend on repository runtime state, sleep, or launch external processes.
// - Allows:
//   - Use isolated temporary registries and serialized in-process controls.
// - Summary:
//   - Active-run lease, cancellation, ETA, and cleanup tests.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - true
//   - Reason: one serialized fixture protects the complete lease lifecycle.
//   - Split: separate state-codec tests if the JSON schema gains versions.
//   - Validation: canonical pipeline test and strict Clippy gates.
//   - Review: required when concurrency or stale cleanup semantics change.
//

//! Regression tests for the cooperative pipeline run registry.

use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::sync::atomic::{AtomicU64, Ordering};

use super::model::{RunMode, RunSnapshot};
use super::{RunRegistry, check_cancellation};

/// Serialize tests because the production registry has one process-wide
/// control.
static TEST_LOCK: Mutex<()> = Mutex::new(());
/// Unique fixture nonce.
static TEST_NONCE: AtomicU64 = AtomicU64::new(0);

/// Isolated registry fixture removed after each test.
#[derive(Debug)]
struct TestRegistry {
    /// Temporary runtime root.
    root: PathBuf,
    /// Registry facade under test.
    registry: RunRegistry,
}

impl TestRegistry {
    /// Construct one unique empty registry.
    fn new(label: &str) -> Result<Self, String> {
        let nonce = TEST_NONCE.fetch_add(
            1,
            Ordering::Relaxed,
        );
        let root = std::env::temp_dir().join(
            format!(
                "pipeline-run-registry-{label}-{}-{nonce}",
                std::process::id()
            ),
        );
        match fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(format!("test registry cleanup failed: {error}"));
            }
        }
        Ok(
            Self {
                registry: RunRegistry::from_root(root.clone()),
                root,
            },
        )
    }
}

impl Drop for TestRegistry {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.root));
    }
}

/// Acquire the serialized registry-test lane.
fn test_lane() -> Result<std::sync::MutexGuard<'static, ()>, String> {
    TEST_LOCK
        .lock()
        .map_err(|_poisoned| String::from("registry test lock is poisoned"))
}

#[test]
fn exclusive_run_blocks_another_default_start_and_lists_metadata()
-> Result<(), String> {
    let _lane = test_lane()?;
    let fixture = TestRegistry::new("exclusive")?;
    let first = fixture
        .registry
        .start(
            "fbx-export-world",
            Some(String::from("world-a")),
            RunMode::Exclusive,
        )
        .map_err(
            |error| {
                error
                    .message()
                    .to_owned()
            },
        )?;
    let first_id = first
        .run_id()
        .to_owned();
    let lines = fixture
        .registry
        .active_lines()?;
    let Some(line) = lines.first() else {
        return Err(String::from("active exclusive run was not listed"));
    };
    for expected in [
        first_id.as_str(),
        "command=fbx-export-world",
        "label=world-a",
        "mode=exclusive",
        "stage=fbx-export-world",
        "elapsed=",
        "eta=unknown",
    ] {
        if !line.contains(expected) {
            return Err(format!("active line lacks {expected:?}: {line}"));
        }
    }
    let blocked = fixture
        .registry
        .start(
            "fbx-export-vehicles",
            None,
            RunMode::Exclusive,
        );
    let Err(error) = blocked else {
        return Err(String::from("second exclusive run was not blocked"));
    };
    if error
        .active_lines()
        .len()
        != 1
    {
        return Err(
            String::from("conflict did not preserve active-run evidence"),
        );
    }
    first.finish()?;
    if !fixture
        .registry
        .active_lines()?
        .is_empty()
    {
        return Err(String::from("finished run remained active"));
    }
    Ok(())
}

#[test]
fn explicit_concurrent_start_can_join_an_existing_derived_lease()
-> Result<(), String> {
    let _lane = test_lane()?;
    let fixture = TestRegistry::new("concurrent")?;
    let now = super::storage::RegistryStorage::now_unix_ms()?;
    let existing = RunSnapshot::new(
        String::from("run-existing"),
        42,
        String::from("fbx-export-world"),
        Some(String::from("world-a")),
        RunMode::Concurrent,
        now,
    );
    fixture
        .registry
        .storage
        .create_run(&existing)?;
    let second = fixture
        .registry
        .start(
            "fbx-export-vehicles",
            Some(String::from("vehicles-b")),
            RunMode::Concurrent,
        )
        .map_err(
            |error| {
                error
                    .message()
                    .to_owned()
            },
        )?;
    if fixture
        .registry
        .active_lines()?
        .len()
        != 2
    {
        return Err(String::from("explicit concurrent run was not registered"));
    }
    second.finish()?;
    fixture
        .registry
        .storage
        .remove_run(existing.run_id())?;
    Ok(())
}

#[test]
fn cancellation_request_is_observed_at_a_cooperative_checkpoint()
-> Result<(), String> {
    let _lane = test_lane()?;
    let fixture = TestRegistry::new("cancel")?;
    let guard = fixture
        .registry
        .start(
            "extract-game-resume",
            None,
            RunMode::Exclusive,
        )
        .map_err(
            |error| {
                error
                    .message()
                    .to_owned()
            },
        )?;
    let run_id = guard
        .run_id()
        .to_owned();
    let requested = fixture
        .registry
        .request_cancel(&run_id)?;
    if requested != [run_id.clone()] {
        return Err(String::from("cancellation target was not preserved"));
    }
    let Err(error) = check_cancellation() else {
        return Err(
            String::from("requested cancellation did not fail the checkpoint"),
        );
    };
    if !error
        .to_string()
        .contains(&run_id)
    {
        return Err(String::from("cancellation error lacks the run id"));
    }
    let lines = fixture
        .registry
        .active_lines()?;
    let Some(line) = lines.first() else {
        return Err(String::from("cancelled run disappeared before cleanup"));
    };
    if !line.contains("state=cancellation-requested") {
        return Err(format!("active line lacks cancellation state: {line}"));
    }
    guard.finish()?;
    Ok(())
}

#[test]
fn progress_snapshot_renders_known_eta_without_wall_clock_sleep()
-> Result<(), String> {
    let _lane = test_lane()?;
    let mut snapshot = RunSnapshot::new(
        String::from("run-progress"),
        7,
        String::from("index-minor-units"),
        None,
        RunMode::Exclusive,
        10_000,
    );
    snapshot.update_progress(
        "index packages",
        Some(0),
        Some(100),
        None,
        10_000,
    );
    snapshot.update_progress(
        "index packages",
        Some(25),
        Some(100),
        Some("package-25"),
        20_000,
    );
    let line = snapshot.render(20_000);
    for expected in [
        "stage=index packages",
        "progress=25/100",
        "elapsed=10s",
        "eta=30s",
        "item=package-25",
    ] {
        if !line.contains(expected) {
            return Err(format!("progress line lacks {expected:?}: {line}"));
        }
    }
    Ok(())
}

#[test]
fn stale_records_and_invalid_run_ids_are_rejected_or_pruned()
-> Result<(), String> {
    let _lane = test_lane()?;
    let fixture = TestRegistry::new("stale")?;
    let stale = RunSnapshot::new(
        String::from("run-stale"),
        9,
        String::from("audit-minor-units"),
        None,
        RunMode::Concurrent,
        1,
    );
    fixture
        .registry
        .storage
        .create_run(&stale)?;
    let active = fixture
        .registry
        .storage
        .active_runs(500_000)?;
    if !active.is_empty() {
        return Err(String::from("stale run record was not pruned"));
    }
    if fixture
        .registry
        .storage
        .root()
        .join("runs/run-stale")
        .exists()
    {
        return Err(String::from("stale run directory remained on disk"));
    }
    if fixture
        .registry
        .request_cancel("../escape")
        .is_ok()
    {
        return Err(String::from("invalid cancellation run id was accepted"));
    }
    Ok(())
}
