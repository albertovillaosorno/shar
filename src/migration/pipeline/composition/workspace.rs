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

#![expect(
    clippy::redundant_pub_crate,
    reason = "crate-root private module shares workspace helpers with sibling \
               adapters"
)]

use std::fs::{self, File, OpenOptions, TryLockError};
use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local as local_filesystem;

use crate::domain::{PipelineError, PipelineOutcome};

/// Shared physical root for canonical generated pipeline workspaces.
pub(crate) const PIPELINE_WORKSPACE_ROOT: &str = ".cache/pipeline";
/// Default physical extraction workspace.
pub(crate) const EXTRACTED_WORKSPACE_ROOT: &str = ".cache/pipeline/extracted";
/// Default physical complete FBX catalog workspace.
pub(crate) const FBX_WORKSPACE_ROOT: &str = ".cache/pipeline/fbx-assets";
/// Default physical complete UI-sprite raster catalog workspace.
pub(crate) const UI_RASTER_WORKSPACE_ROOT: &str =
    ".cache/pipeline/ui-raster-assets";
/// Default physical Scrooby semantic-binding catalog workspace.
pub(crate) const UI_SCROOBY_BINDING_WORKSPACE_ROOT: &str =
    ".cache/pipeline/ui-scrooby-bindings";
/// Default physical Scrooby runtime-layout catalog workspace.
pub(crate) const UI_SCROOBY_LAYOUT_WORKSPACE_ROOT: &str =
    ".cache/pipeline/ui-scrooby-layout";
/// Default physical Scrooby page-resource lifecycle catalog workspace.
pub(crate) const UI_SCROOBY_RESOURCE_WORKSPACE_ROOT: &str =
    ".cache/pipeline/ui-scrooby-resources";
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
                Ok(()) => {
                    Err(io_failure("move legacy extraction workspace", &error))
                },
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
    local_filesystem::create_dir_all(parent).map_err(|error| {
        io_failure("create cached workspace parent", &error)
    })?;
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
            ensure_real_empty_file(path, "legacy extraction transaction lock")?;
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
        return Err(PipelineError::new(format!("{label} must be a real file")));
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

const LEGACY_FBX_WORKSPACE_ROOT: &str = "fbx-assets";
const LEGACY_UNREAL_WORKSPACE_ROOT: &str = "unreal-staging";
const LEGACY_FBX_MANIFEST_NAME: &str = "catalog.jsonl";
const LEGACY_UNREAL_MANIFEST_NAME: &str = "manifest.jsonl";
const FBX_STAGING_NAME: &str = ".fbx-assets.complete-staging";

/// Move one complete legacy FBX workspace and ledger into canonical storage.
///
/// # Errors
///
/// Returns a deterministic failure when legacy and canonical state compete,
/// transaction staging exists, a path changes kind, or rename fails.
pub(crate) fn migrate_legacy_fbx_workspace(
    manifest_destination: &Path,
) -> PipelineOutcome<bool> {
    migrate_legacy_payload_workspace_at(
        Path::new("."),
        LEGACY_FBX_WORKSPACE_ROOT,
        FBX_WORKSPACE_ROOT,
        Some(LEGACY_FBX_MANIFEST_NAME),
        manifest_destination,
        &[FBX_STAGING_NAME],
        "FBX",
    )
}

/// Move one legacy Unreal staging root and ledger into canonical storage.
///
/// # Errors
///
/// Returns a deterministic failure when legacy and canonical state compete, a
/// path changes kind, or rename fails.
pub(crate) fn migrate_legacy_unreal_workspace(
    manifest_destination: &Path,
) -> PipelineOutcome<bool> {
    migrate_legacy_payload_workspace_at(
        Path::new("."),
        LEGACY_UNREAL_WORKSPACE_ROOT,
        UNREAL_STAGING_WORKSPACE_ROOT,
        Some(LEGACY_UNREAL_MANIFEST_NAME),
        manifest_destination,
        &[],
        "Unreal",
    )
}

#[derive(Debug)]
struct PayloadMigration<'a> {
    legacy_root: PathBuf,
    canonical_root: PathBuf,
    legacy_manifest: Option<PathBuf>,
    manifest_destination: &'a Path,
    label: &'a str,
}

fn migrate_legacy_payload_workspace_at(
    repository_root: &Path,
    legacy_relative: &str,
    canonical_relative: &str,
    legacy_manifest_name: Option<&str>,
    manifest_destination: &Path,
    transaction_blockers: &[&str],
    label: &str,
) -> PipelineOutcome<bool> {
    let legacy_root = repository_root.join(legacy_relative);
    if !path_present(&legacy_root)? {
        return Ok(false);
    }
    ensure_real_directory(&legacy_root, &format!("legacy {label} workspace"))?;
    let canonical_root = repository_root.join(canonical_relative);
    reject_competing_payload_root(&canonical_root, label)?;

    let legacy_parent = legacy_root.parent().unwrap_or(repository_root);
    let canonical_parent = canonical_root.parent().ok_or_else(|| {
        PipelineError::new(format!("canonical {label} workspace has no parent"))
    })?;
    reject_payload_transaction_blockers(
        legacy_parent,
        canonical_parent,
        transaction_blockers,
        label,
    )?;
    let legacy_manifest = if let Some(name) = legacy_manifest_name {
        let path = legacy_root.join(name);
        path_present(&path)?.then_some(path)
    } else {
        None
    };
    if path_present(manifest_destination)? {
        return Err(PipelineError::new(format!(
            concat!(
                "canonical {} manifest already exists while a legacy ",
                "workspace needs migration"
            ),
            label,
        )));
    }
    reject_manifest_staging(manifest_destination, label)?;
    if let Some(path) = legacy_manifest.as_deref() {
        ensure_real_file(path, &format!("legacy {label} manifest"))?;
    }
    ensure_cache_parent(canonical_parent)?;
    if legacy_manifest.is_some() {
        ensure_real_parent(manifest_destination, label)?;
    }
    migrate_payload_and_manifest(&PayloadMigration {
        legacy_root,
        canonical_root,
        legacy_manifest,
        manifest_destination,
        label,
    })?;
    Ok(true)
}

fn migrate_payload_and_manifest(
    migration: &PayloadMigration<'_>,
) -> PipelineOutcome<()> {
    if let Some(source) = migration.legacy_manifest.as_ref() {
        fs::rename(source, migration.manifest_destination).map_err(
            |error| io_failure("move legacy generated manifest", &error),
        )?;
    }

    match fs::rename(&migration.legacy_root, &migration.canonical_root) {
        Ok(()) => Ok(()),
        Err(error) => {
            let manifest_restore =
                migration.legacy_manifest.as_ref().map_or(Ok(()), |source| {
                    fs::rename(migration.manifest_destination, source)
                });
            match manifest_restore {
                Ok(()) => {
                    Err(io_failure("move legacy generated workspace", &error))
                },
                Err(rollback) => Err(PipelineError::new(format!(
                    concat!(
                        "move legacy {} workspace failed ({:?}); ",
                        "restore legacy manifest failed ({:?})"
                    ),
                    migration.label,
                    error.kind(),
                    rollback.kind(),
                ))),
            }
        },
    }
}

fn reject_competing_payload_root(
    canonical_root: &Path,
    label: &str,
) -> PipelineOutcome<()> {
    if path_present(canonical_root)? {
        Err(PipelineError::new(format!(
            concat!(
                "legacy and canonical {} workspaces both exist; ",
                "reconcile them before retrying"
            ),
            label,
        )))
    } else {
        Ok(())
    }
}

fn reject_payload_transaction_blockers(
    legacy_parent: &Path,
    canonical_parent: &Path,
    names: &[&str],
    label: &str,
) -> PipelineOutcome<()> {
    for parent in [legacy_parent, canonical_parent] {
        for name in names {
            if path_present(&parent.join(name))? {
                return Err(PipelineError::new(format!(
                    concat!(
                        "{} publication staging exists; inspect the ",
                        "interrupted transaction before migration"
                    ),
                    label,
                )));
            }
        }
    }
    Ok(())
}

fn reject_manifest_staging(
    manifest_destination: &Path,
    label: &str,
) -> PipelineOutcome<()> {
    let Some(parent) = manifest_destination.parent() else {
        return Err(PipelineError::new(format!(
            "canonical {label} manifest has no parent"
        )));
    };
    let Some(name) = manifest_destination.file_name() else {
        return Err(PipelineError::new(format!(
            "canonical {label} manifest has no file name"
        )));
    };
    let mut staging_name = std::ffi::OsString::from(".");
    staging_name.push(name);
    staging_name.push(".complete-staging");
    if path_present(&parent.join(staging_name))? {
        return Err(PipelineError::new(format!(
            concat!(
                "canonical {} manifest staging exists; inspect the ",
                "interrupted transaction before migration"
            ),
            label,
        )));
    }
    Ok(())
}

fn ensure_real_parent(path: &Path, label: &str) -> PipelineOutcome<()> {
    let parent = path.parent().ok_or_else(|| {
        PipelineError::new(format!("canonical {label} manifest has no parent"))
    })?;
    local_filesystem::create_dir_all(parent).map_err(|error| {
        io_failure("create canonical manifest parent", &error)
    })?;
    ensure_real_directory(parent, &format!("canonical {label} manifest parent"))
}

fn ensure_real_file(path: &Path, label: &str) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_failure("inspect workspace file", &error))?;
    if metadata.is_file() && !metadata.file_type().is_symlink() {
        Ok(())
    } else {
        Err(PipelineError::new(format!("{label} must be a real file")))
    }
}

#[cfg(test)]
#[path = "../../../../tests/migration/pipeline/unit/workspace/tests.rs"]
mod tests;
