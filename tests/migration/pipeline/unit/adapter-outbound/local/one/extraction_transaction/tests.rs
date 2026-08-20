// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Whole-root extraction transaction unit tests.
// - Must-Not:
//   - Read the local game installation or mutate repository outputs.
// - Allows:
//   - Isolated temporary directories and deterministic interruption fixtures.
// - Split-When:
//   - Split when publication and recovery fixtures gain separate ownership.
// - Merge-When:
//   - Merge when the extraction transaction loses its independent lifecycle.
// - Summary:
//   - Extraction transaction tests.
// - Description:
//   - Proves atomic publication, rollback, recovery, and fail-closed ownership.
// - Usage:
//   - Included only by the extraction transaction under cfg(test).
// - Defaults:
//   - Every temporary artifact is uniquely named and explicitly removed.
//

//! Whole-root extraction transaction unit tests.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{ExtractionTransaction, STATE_BYTES, TransactionPaths};

static CASE_ID: AtomicUsize = AtomicUsize::new(0);

type TestResult = Result<(), String>;

#[test]
fn abort_preserves_the_accepted_root() -> TestResult {
    let root = case_root("abort");
    prepare_root(&root)?;
    let destination = root.join("extracted");
    write_member(&destination, "accepted.txt", b"accepted")?;

    let transaction = ExtractionTransaction::begin(&destination)
        .map_err(|error| error.to_string())?;
    write_member(transaction.staging_root(), "candidate.txt", b"candidate")?;
    transaction.abort().map_err(|error| error.to_string())?;

    require_bytes(&destination.join("accepted.txt"), b"accepted")?;
    require_missing(&destination.join("candidate.txt"))?;
    require_transaction_absent(&destination)?;
    cleanup(&root)
}

#[test]
fn publish_replaces_the_complete_root() -> TestResult {
    let root = case_root("publish");
    prepare_root(&root)?;
    let destination = root.join("extracted");
    write_member(&destination, "accepted.txt", b"accepted")?;

    let transaction = ExtractionTransaction::begin(&destination)
        .map_err(|error| error.to_string())?;
    write_member(transaction.staging_root(), "candidate.txt", b"candidate")?;
    transaction.publish().map_err(|error| error.to_string())?;

    require_missing(&destination.join("accepted.txt"))?;
    require_bytes(&destination.join("candidate.txt"), b"candidate")?;
    require_transaction_absent(&destination)?;
    cleanup(&root)
}

#[test]
fn recovery_restores_a_backup_before_publication() -> TestResult {
    let root = case_root("restore-backup");
    prepare_root(&root)?;
    let destination = root.join("extracted");
    write_member(&destination, "accepted.txt", b"accepted")?;
    let paths = TransactionPaths::new(&destination)
        .map_err(|error| error.to_string())?;
    fs::write(&paths.state, STATE_BYTES).map_err(|error| error.to_string())?;
    fs::rename(&paths.destination, &paths.backup)
        .map_err(|error| error.to_string())?;
    write_member(&paths.staging, "partial.txt", b"partial")?;

    let transaction = ExtractionTransaction::begin(&destination)
        .map_err(|error| error.to_string())?;
    require_bytes(&destination.join("accepted.txt"), b"accepted")?;
    require_missing(&destination.join("partial.txt"))?;
    require_empty(transaction.staging_root())?;
    transaction.abort().map_err(|error| error.to_string())?;
    require_transaction_absent(&destination)?;
    cleanup(&root)
}

#[test]
fn recovery_keeps_an_already_published_candidate() -> TestResult {
    let root = case_root("keep-candidate");
    prepare_root(&root)?;
    let destination = root.join("extracted");
    write_member(&destination, "accepted.txt", b"accepted")?;
    let paths = TransactionPaths::new(&destination)
        .map_err(|error| error.to_string())?;
    fs::write(&paths.state, STATE_BYTES).map_err(|error| error.to_string())?;
    write_member(&paths.staging, "candidate.txt", b"candidate")?;
    fs::rename(&paths.destination, &paths.backup)
        .map_err(|error| error.to_string())?;
    fs::rename(&paths.staging, &paths.destination)
        .map_err(|error| error.to_string())?;

    let transaction = ExtractionTransaction::begin(&destination)
        .map_err(|error| error.to_string())?;
    require_missing(&destination.join("accepted.txt"))?;
    require_bytes(&destination.join("candidate.txt"), b"candidate")?;
    require_empty(transaction.staging_root())?;
    transaction.abort().map_err(|error| error.to_string())?;
    require_transaction_absent(&destination)?;
    cleanup(&root)
}

#[test]
fn unowned_transaction_artifacts_fail_closed() -> TestResult {
    let root = case_root("unowned");
    prepare_root(&root)?;
    let destination = root.join("extracted");
    write_member(&destination, "accepted.txt", b"accepted")?;
    let paths = TransactionPaths::new(&destination)
        .map_err(|error| error.to_string())?;
    write_member(&paths.staging, "unknown.txt", b"unknown")?;

    let error = match ExtractionTransaction::begin(&destination) {
        Ok(_transaction) => {
            return Err("unowned staging unexpectedly succeeded".to_owned());
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("unowned extraction transaction artifacts") {
        return Err(format!("unexpected unowned-artifact error: {error}"));
    }
    require_bytes(&paths.staging.join("unknown.txt"), b"unknown")?;
    require_bytes(&destination.join("accepted.txt"), b"accepted")?;
    cleanup(&root)
}

#[test]
fn active_transaction_lease_rejects_concurrent_recovery() -> TestResult {
    let root = case_root("active-lease");
    prepare_root(&root)?;
    let destination = root.join("extracted");
    write_member(&destination, "accepted.txt", b"accepted")?;
    let transaction = ExtractionTransaction::begin(&destination)
        .map_err(|error| error.to_string())?;
    write_member(transaction.staging_root(), "candidate.txt", b"candidate")?;

    let error = match ExtractionTransaction::begin(&destination) {
        Ok(_second) => {
            return Err("concurrent transaction succeeded".to_owned());
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("active extraction transaction") {
        return Err(format!("unexpected active-lease error: {error}"));
    }
    require_bytes(
        &transaction.staging_root().join("candidate.txt"),
        b"candidate",
    )?;
    require_bytes(&destination.join("accepted.txt"), b"accepted")?;
    transaction.abort().map_err(|error| error.to_string())?;
    require_transaction_absent(&destination)?;
    cleanup(&root)
}

#[test]
fn nonempty_transaction_lock_fails_without_mutation() -> TestResult {
    let root = case_root("nonempty-lock");
    prepare_root(&root)?;
    let destination = root.join("extracted");
    write_member(&destination, "accepted.txt", b"accepted")?;
    let paths = TransactionPaths::new(&destination)
        .map_err(|error| error.to_string())?;
    fs::write(&paths.lock, b"unexpected").map_err(|error| error.to_string())?;

    let error = match ExtractionTransaction::begin(&destination) {
        Ok(_transaction) => {
            return Err("nonempty lock unexpectedly succeeded".to_owned());
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("lock must be empty") {
        return Err(format!("unexpected nonempty-lock error: {error}"));
    }
    require_bytes(&destination.join("accepted.txt"), b"accepted")?;
    require_bytes(&paths.lock, b"unexpected")?;
    require_missing(&paths.staging)?;
    require_missing(&paths.state)?;
    cleanup(&root)
}

#[test]
fn malformed_state_fails_without_mutation() -> TestResult {
    let root = case_root("malformed-state");
    prepare_root(&root)?;
    let destination = root.join("extracted");
    write_member(&destination, "accepted.txt", b"accepted")?;
    let paths = TransactionPaths::new(&destination)
        .map_err(|error| error.to_string())?;
    fs::write(&paths.state, b"malformed").map_err(|error| error.to_string())?;
    write_member(&paths.staging, "partial.txt", b"partial")?;

    let error = match ExtractionTransaction::begin(&destination) {
        Ok(_transaction) => {
            return Err("malformed state unexpectedly succeeded".to_owned());
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("transaction state is malformed") {
        return Err(format!("unexpected malformed-state error: {error}"));
    }
    require_bytes(&destination.join("accepted.txt"), b"accepted")?;
    require_bytes(&paths.staging.join("partial.txt"), b"partial")?;
    require_bytes(&paths.state, b"malformed")?;
    cleanup(&root)
}

#[test]
fn invalid_destination_does_not_create_its_parent() -> TestResult {
    let root = case_root("invalid-destination-parent");
    prepare_root(&root)?;
    let would_be_parent = root.join("must-not-exist");
    let destination = would_be_parent.join("..");

    let error = match TransactionPaths::new(&destination) {
        Ok(_paths) => {
            return Err(
                "parent-only destination unexpectedly passed".to_owned(),
            );
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("no final path segment") {
        return Err(format!("unexpected invalid-destination error: {error}"));
    }
    require_missing(&would_be_parent)?;
    cleanup(&root)
}

#[test]
fn symlinked_parent_prefix_fails_without_external_creation() -> TestResult {
    let root = case_root("symlinked-parent-prefix");
    prepare_root(&root)?;
    let target = root.join("target");
    let link = root.join("link");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    create_directory_link(&target, &link)?;
    let destination = link.join("created-through-link").join("extracted");

    let error = match ExtractionTransaction::begin(&destination) {
        Ok(_transaction) => {
            return Err(
                "symlinked parent prefix unexpectedly succeeded".to_owned(),
            );
        }
        Err(error) => error.to_string(),
    };
    if !error.contains("create extraction parent") {
        return Err(format!("unexpected symlink-prefix error: {error}"));
    }
    require_missing(&target.join("created-through-link"))?;
    remove_directory_link(&link)?;
    cleanup(&root)
}

#[test]
fn relative_output_uses_portable_sibling_names() -> TestResult {
    let paths = TransactionPaths::new(Path::new("extracted"))
        .map_err(|error| error.to_string())?;
    if paths.staging.as_path() != Path::new(".extracted.pipeline-staging")
        || paths.backup.as_path() != Path::new(".extracted.pipeline-backup")
        || paths.state.as_path()
            != Path::new(".extracted.pipeline-transaction.json")
        || paths.lock.as_path() != Path::new(".extracted.pipeline-lock")
    {
        return Err(format!(
            "unexpected relative transaction paths: {paths:?}"
        ));
    }
    Ok(())
}

#[cfg(unix)]
fn create_directory_link(target: &Path, link: &Path) -> TestResult {
    std::os::unix::fs::symlink(target, link).map_err(|error| error.to_string())
}

#[cfg(windows)]
fn create_directory_link(target: &Path, link: &Path) -> TestResult {
    std::os::windows::fs::symlink_dir(target, link)
        .map_err(|error| error.to_string())
}

#[cfg(unix)]
fn remove_directory_link(link: &Path) -> TestResult {
    fs::remove_file(link).map_err(|error| error.to_string())
}

#[cfg(windows)]
fn remove_directory_link(link: &Path) -> TestResult {
    fs::remove_dir(link).map_err(|error| error.to_string())
}

fn case_root(label: &str) -> PathBuf {
    let id = CASE_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "pipeline-extraction-transaction-{label}-{}-{id}",
        std::process::id()
    ))
}

fn prepare_root(root: &Path) -> TestResult {
    if root.exists() {
        fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(root).map_err(|error| error.to_string())
}

fn cleanup(root: &Path) -> TestResult {
    fs::remove_dir_all(root).map_err(|error| error.to_string())
}

fn write_member(root: &Path, name: &str, bytes: &[u8]) -> TestResult {
    fs::create_dir_all(root).map_err(|error| error.to_string())?;
    fs::write(root.join(name), bytes).map_err(|error| error.to_string())
}

fn require_bytes(path: &Path, expected: &[u8]) -> TestResult {
    let actual = fs::read(path).map_err(|error| error.to_string())?;
    if actual == expected {
        Ok(())
    } else {
        Err(format!("unexpected bytes for {}", path.display()))
    }
}

fn require_missing(path: &Path) -> TestResult {
    if path.exists() {
        Err(format!("unexpected path remained: {}", path.display()))
    } else {
        Ok(())
    }
}

fn require_empty(path: &Path) -> TestResult {
    let mut entries = fs::read_dir(path).map_err(|error| error.to_string())?;
    if entries.next().is_none() {
        Ok(())
    } else {
        Err("fresh extraction staging was not empty".to_owned())
    }
}

fn require_transaction_absent(destination: &Path) -> TestResult {
    let paths = TransactionPaths::new(destination)
        .map_err(|error| error.to_string())?;
    for path in [paths.staging, paths.backup, paths.state] {
        require_missing(&path)?;
    }
    Ok(())
}
