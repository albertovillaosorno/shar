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
//   - Canonical repository-relative generated workspace locations.
//   - Compatibility migration from retired root-level workspace defaults.
// - Must-Not:
//   - Define portable artifact identities stored inside manifests.
//   - Merge competing legacy and canonical generated trees.
// - Allows:
//   - Shared physical defaults for pipeline adapters and CLI composition.
//   - Fail-closed same-filesystem rename of clean legacy workspaces.
// - Split-When:
//   - One generated workspace gains an independent lifecycle contract.
// - Merge-When:
//   - Another composition module owns the identical physical defaults.
// - Summary:
//   - Canonical generated workspace paths and legacy migration.
// - Description:
//   - Keeps regenerable pipeline output below one ignored cache hierarchy while
//     logical manifest identities remain stable.
// - Usage:
//   - Used by command defaults and local generated-output adapters.
// - Defaults:
//   - Generated pipeline output lives below `.cache/pipeline`.
//   - Competing or interrupted legacy state fails closed before migration.
//

//! Canonical generated workspace paths and legacy compatibility migration.

use std::fs::{self, File, OpenOptions, TryLockError};
use std::path::Path;

use crate::domain::{PipelineError, PipelineOutcome};
use schoenwald_filesystem::adapters::driving::local as local_filesystem;

/// Default physical extraction workspace.
pub(crate) const EXTRACTED_WORKSPACE_ROOT: &str = ".cache/pipeline/extracted";
/// Default physical complete FBX catalog workspace.
pub(crate) const FBX_WORKSPACE_ROOT: &str = ".cache/pipeline/fbx-assets";
/// Default physical Unreal staging workspace.
pub(crate) const UNREAL_STAGING_WORKSPACE_ROOT: &str =
    ".cache/pipeline/unreal-staging";

const LEGACY_EXTRACTED_WORKSPACE_ROOT: &str = "extracted";
const EXTRACTED_LOCK_NAME: &str = ".extracted.pipeline-lock";
const EXTRACTED_TRANSACTION_BLOCKERS: &[&str] = &[
    ".extracted.pipeline-staging",
    ".extracted.pipeline-backup",
    ".extracted.pipeline-transaction.json",
];

/// Move one clean root-level extraction workspace to its cached default.
///
/// Returns whether a legacy workspace was migrated.
///
/// # Errors
///
/// Returns a deterministic failure when legacy state competes with canonical
/// state, a transaction is active or interrupted, a path is not a real
/// directory/file, or the same-filesystem rename fails.
pub(crate) fn migrate_legacy_extracted_workspace() -> PipelineOutcome<bool> {
    migrate_legacy_extracted_workspace_at(Path::new("."))
}

fn migrate_legacy_extracted_workspace_at(
    repository_root: &Path,
) -> PipelineOutcome<bool> {
    let legacy_root = repository_root.join(LEGACY_EXTRACTED_WORKSPACE_ROOT);
    if !path_present(&legacy_root)? {
        return Ok(false);
    }
    ensure_real_directory(&legacy_root, "legacy extraction workspace")?;

    let canonical_root = repository_root.join(EXTRACTED_WORKSPACE_ROOT);
    if path_present(&canonical_root)? {
        return Err(PipelineError::new(concat!(
            "legacy and canonical extraction workspaces both exist; ",
            "reconcile them before retrying"
        )));
    }

    let legacy_parent = legacy_root.parent().unwrap_or(repository_root);
    let canonical_parent = canonical_root.parent().ok_or_else(|| {
        PipelineError::new("canonical extraction workspace has no parent")
    })?;
    reject_transaction_blockers(
        legacy_parent,
        EXTRACTED_TRANSACTION_BLOCKERS,
        "legacy extraction transaction is incomplete; recover it explicitly \
         before migration",
    )?;
    reject_transaction_blockers(
        canonical_parent,
        EXTRACTED_TRANSACTION_BLOCKERS,
        "canonical extraction transaction is incomplete; recover it before \
         migration",
    )?;

    let legacy_lock = legacy_parent.join(EXTRACTED_LOCK_NAME);
    let canonical_lock = canonical_parent.join(EXTRACTED_LOCK_NAME);
    if path_present(&canonical_lock)? {
        return Err(PipelineError::new(concat!(
            "canonical extraction transaction lock already exists while a ",
            "legacy workspace needs migration"
        )));
    }
    ensure_cache_parent(canonical_parent)?;
    migrate_extraction_with_lock(
        &legacy_root,
        &canonical_root,
        &legacy_lock,
        &canonical_lock,
    )?;
    Ok(true)
}

fn migrate_extraction_with_lock(
    legacy_root: &Path,
    canonical_root: &Path,
    legacy_lock: &Path,
    canonical_lock: &Path,
) -> PipelineOutcome<()> {
    let (lease, created_lock) = acquire_migration_lock(legacy_lock)?;
    if let Err(error) = fs::rename(legacy_lock, canonical_lock) {
        drop(lease);
        if created_lock {
            let _cleanup = fs::remove_file(legacy_lock);
        }
        return Err(io_failure("move extraction transaction lock", &error));
    }

    match fs::rename(legacy_root, canonical_root) {
        Ok(()) => {
            drop(lease);
            Ok(())
        },
        Err(error) => {
            let restore = fs::rename(canonical_lock, legacy_lock);
            drop(lease);
            if restore.is_ok() && created_lock {
                let _cleanup = fs::remove_file(legacy_lock);
            }
            match restore {
                Ok(()) => Err(io_failure(
                    "move legacy extraction workspace",
                    &error,
                )),
                Err(rollback) => Err(PipelineError::new(format!(
                    concat!(
                        "move legacy extraction workspace failed ({:?}); ",
                        "restore extraction transaction lock failed ({:?})"
                    ),
                    error.kind(),
                    rollback.kind(),
                ))),
            }
        },
    }
}

fn ensure_cache_parent(parent: &Path) -> PipelineOutcome<()> {
    local_filesystem::create_dir_all(parent)
        .map_err(|error| io_failure("create cached workspace parent", &error))?;
    ensure_real_directory(parent, "cached workspace parent")
}

fn acquire_migration_lock(path: &Path) -> PipelineOutcome<(File, bool)> {
    let (file, created) = match OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(file) => (file, true),
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            ensure_real_empty_file(
                path,
                "legacy extraction transaction lock",
            )?;
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .map_err(|open_error| {
                    io_failure(
                        "open legacy extraction transaction lock",
                        &open_error,
                    )
                })?;
            (file, false)
        },
        Err(error) => {
            return Err(io_failure(
                "create legacy extraction transaction lock",
                &error,
            ));
        },
    };
    match file.try_lock() {
        Ok(()) => Ok((file, created)),
        Err(TryLockError::WouldBlock) => Err(PipelineError::new(concat!(
            "an active legacy extraction transaction owns the workspace; ",
            "retry after it finishes"
        ))),
        Err(TryLockError::Error(error)) => Err(io_failure(
            "lock legacy extraction workspace for migration",
            &error,
        )),
    }
}

fn reject_transaction_blockers(
    parent: &Path,
    names: &[&str],
    message: &str,
) -> PipelineOutcome<()> {
    for name in names {
        if path_present(&parent.join(name))? {
            return Err(PipelineError::new(message));
        }
    }
    Ok(())
}

fn ensure_real_directory(path: &Path, label: &str) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_failure("inspect workspace directory", &error))?;
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        Ok(())
    } else {
        Err(PipelineError::new(format!(
            "{label} must be a real directory"
        )))
    }
}

fn ensure_real_empty_file(path: &Path, label: &str) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_failure("inspect workspace file", &error))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(PipelineError::new(format!(
            "{label} must be a real file"
        )));
    }
    if metadata.len() != 0 {
        return Err(PipelineError::new(format!("{label} must be empty")));
    }
    Ok(())
}

fn path_present(path: &Path) -> PipelineOutcome<bool> {
    match fs::symlink_metadata(path) {
        Ok(_metadata) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(io_failure("inspect workspace path", &error)),
    }
}

fn io_failure(operation: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{operation} failed ({:?})", error.kind()))
}

#[cfg(test)]
#[path = "../../../../tests/migration/pipeline/unit/workspace/tests.rs"]
mod tests;
