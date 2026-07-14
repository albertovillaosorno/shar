// File:
//   - transaction.rs
// Path:
//   - src/rsd/src/adapters/driven/transaction.rs
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
//   - Transactional materialization and rollback of converted WAV outputs.
// - Must-Not:
//   - Discover source files or decode RSD audio payloads.
// - Allows:
//   - Destination preflight, staging, backup, commit, and rollback operations.
// - Split-When:
//   - Split when path resolution and output commit evolve independently.
// - Merge-When:
//   - Filesystem discovery owns the same transaction state and invariants.
// - Summary:
//   - Commits one validated RSD export batch without partial destinations.
// - Description:
//   - Stages WAV bytes, preserves existing entries, and restores on failure.
// - Usage:
//   - Called by the filesystem adapter after all source conversion succeeds.
// - Defaults:
//   - No destination changes survive a failed commit phase.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Transaction preflight, staging, commit, and rollback form one
//   - indivisible filesystem safety contract.
//

//! Transactional filesystem materialization for converted RSD outputs.
//!
//! This adapter stages every WAV, preserves existing destination entries, and
//! restores the complete prior set when any backup or commit operation fails.
//! Discovery and codec conversion remain owned by the filesystem adapter.
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};

use super::filesystem::{PendingOutput, name_identity};
use crate::domain::RsdError;

/// One output represented entirely inside the destination transaction.
struct TransactionOutput {
    /// Final WAV destination.
    destination: PathBuf,
    /// Validated RIFF bytes waiting for staging.
    bytes: Vec<u8>,
    /// Fully written WAV waiting for commit.
    temporary: PathBuf,
    /// Previous destination identity waiting for restoration or cleanup.
    backup: PathBuf,
}

/// Verifies one destination can be materialized without path-type conflicts.
fn check_destination(
    path: &Path,
    resolved_output_root: &Path,
) -> Result<(), RsdError> {
    match fs::metadata(path) {
        Ok(metadata) if metadata.is_dir() => {
            return Err(RsdError::InvalidPath(path.to_path_buf()));
        }
        Ok(metadata)
            if metadata
                .permissions()
                .readonly() =>
        {
            return Err(RsdError::InvalidPath(path.to_path_buf()));
        }
        Ok(_) => {}
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {}
        Err(source) => {
            return Err(
                RsdError::Io {
                    path: path.to_path_buf(),
                    source,
                },
            );
        }
    }
    let Some(mut ancestor) = path.parent() else {
        return Err(RsdError::InvalidPath(path.to_path_buf()));
    };
    let resolved_parent = resolve_target(ancestor)?;
    if !resolved_parent.starts_with(resolved_output_root) {
        return Err(RsdError::InvalidPath(ancestor.to_path_buf()));
    }
    loop {
        match fs::metadata(ancestor) {
            Ok(metadata) if metadata.is_dir() => return Ok(()),
            Ok(_) => return Err(RsdError::InvalidPath(ancestor.to_path_buf())),
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                let Some(parent) = ancestor.parent() else {
                    return Err(RsdError::InvalidPath(path.to_path_buf()));
                };
                ancestor = parent;
            }
            Err(source) => {
                return Err(
                    RsdError::Io {
                        path: ancestor.to_path_buf(),
                        source,
                    },
                );
            }
        }
    }
}

/// Derives one short sibling transaction path independent of destination
/// length.
fn transaction_path(
    destination: &Path,
    index: usize,
    suffix: &str,
) -> Result<PathBuf, RsdError> {
    let Some(parent) = destination.parent() else {
        return Err(RsdError::InvalidPath(destination.to_path_buf()));
    };
    Ok(parent.join(format!(".rsd-export-{index:016x}.{suffix}")))
}

/// Reports whether one filesystem entry exists without following links.
fn path_entry_exists(path: &Path) -> Result<bool, RsdError> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
            Ok(false)
        }
        Err(source) => Err(
            RsdError::Io {
                path: path.to_path_buf(),
                source,
            },
        ),
    }
}

/// Removes one transaction-owned entry during best-effort rollback.
fn remove_transaction_entry(path: &Path) {
    if path_entry_exists(path).unwrap_or(false) {
        let _cleanup_result = fs::remove_file(path);
    }
}

/// Converts one pending result into transaction-owned state.
fn transaction_output(
    output: PendingOutput,
    index: usize,
) -> Result<TransactionOutput, RsdError> {
    let temporary = transaction_path(
        &output.destination,
        index,
        "tmp",
    )?;
    let backup = transaction_path(
        &output.destination,
        index,
        "bak",
    )?;
    Ok(
        TransactionOutput {
            destination: output.destination,
            bytes: output.bytes,
            temporary,
            backup,
        },
    )
}

/// Writes one complete staging file without replacing an existing entry.
fn stage_output(output: &TransactionOutput) -> Result<(), RsdError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output.temporary)
        .map_err(
            |source| RsdError::Io {
                path: output
                    .temporary
                    .clone(),
                source,
            },
        )?;
    if let Err(source) = file.write_all(&output.bytes) {
        remove_transaction_entry(&output.temporary);
        return Err(
            RsdError::Io {
                path: output
                    .temporary
                    .clone(),
                source,
            },
        );
    }
    Ok(())
}

/// Cleans every staged output after a pre-commit failure.
fn cleanup_staging(outputs: &[TransactionOutput]) {
    for output in outputs {
        remove_transaction_entry(&output.temporary);
    }
}

/// Restores every destination moved aside for this transaction.
fn rollback_backups(
    backed_up: &[(
        PathBuf,
        PathBuf,
    )]
) {
    for (destination, backup) in backed_up
        .iter()
        .rev()
    {
        remove_transaction_entry(destination);
        let _restore_result = fs::rename(
            backup,
            destination,
        );
    }
}

/// Stages all bytes before any destination identity is changed.
fn stage_outputs(outputs: &[TransactionOutput]) -> Result<(), RsdError> {
    for output in outputs {
        if let Err(error) = stage_output(output) {
            cleanup_staging(outputs);
            return Err(error);
        }
    }
    Ok(())
}

/// Moves all existing destinations aside before the first final commit.
fn backup_destinations(
    outputs: &[TransactionOutput]
) -> Result<
    Vec<(
        PathBuf,
        PathBuf,
    )>,
    RsdError,
> {
    let mut backed_up = Vec::new();
    for output in outputs {
        if !path_entry_exists(&output.destination)? {
            continue;
        }
        if let Err(source) = fs::rename(
            &output.destination,
            &output.backup,
        ) {
            rollback_backups(&backed_up);
            cleanup_staging(outputs);
            return Err(
                RsdError::Io {
                    path: output
                        .destination
                        .clone(),
                    source,
                },
            );
        }
        backed_up.push(
            (
                output
                    .destination
                    .clone(),
                output
                    .backup
                    .clone(),
            ),
        );
    }
    Ok(backed_up)
}

/// Commits all staged files or restores the complete previous destination set.
fn commit_staged_outputs(
    outputs: &[TransactionOutput],
    backed_up: &[(
        PathBuf,
        PathBuf,
    )],
) -> Result<(), RsdError> {
    let mut committed = Vec::<PathBuf>::new();
    for output in outputs {
        if let Err(source) = fs::rename(
            &output.temporary,
            &output.destination,
        ) {
            for destination in committed {
                remove_transaction_entry(&destination);
            }
            rollback_backups(backed_up);
            cleanup_staging(outputs);
            return Err(
                RsdError::Io {
                    path: output
                        .destination
                        .clone(),
                    source,
                },
            );
        }
        committed.push(
            output
                .destination
                .clone(),
        );
    }
    for (_, backup) in backed_up {
        fs::remove_file(backup).map_err(
            |source| RsdError::Io {
                path: backup.clone(),
                source,
            },
        )?;
    }
    Ok(())
}

/// Produces one platform-aware component identity for an output path.
fn path_identity(path: &Path) -> Vec<Vec<u32>> {
    let mut identity = Vec::new();
    for component in path.components() {
        identity.push(name_identity(component.as_os_str()));
    }
    identity
}

/// Rejects collisions across final and transaction-owned output paths.
fn validate_destination_namespace(
    outputs: &[TransactionOutput]
) -> Result<(), RsdError> {
    let mut destinations = BTreeMap::new();
    for output in outputs {
        for path in [
            &output.destination,
            &output.temporary,
            &output.backup,
        ] {
            let identity = path_identity(path);
            let replaced = destinations.insert(
                identity,
                path.clone(),
            );
            if replaced.is_some() {
                return Err(RsdError::CollidingOutputPath(path.clone()));
            }
        }
    }
    let mut previous = None;
    for (identity, destination) in &destinations {
        if let Some(parent) = previous
            && identity.starts_with(parent)
        {
            return Err(RsdError::CollidingOutputPath(destination.clone()));
        }
        previous = Some(identity);
    }
    Ok(())
}

/// Verifies every final and transaction-owned path before staging.
fn validate_transaction_paths(
    outputs: &[TransactionOutput],
    resolved_output_root: &Path,
) -> Result<(), RsdError> {
    for output in outputs {
        check_destination(
            &output.destination,
            resolved_output_root,
        )?;
        for path in [
            &output.temporary,
            &output.backup,
        ] {
            check_destination(
                path,
                resolved_output_root,
            )?;
            if path_entry_exists(path)? {
                return Err(RsdError::InvalidPath(path.clone()));
            }
        }
    }
    Ok(())
}

/// Creates every destination parent before staging starts.
fn create_destination_parents(
    outputs: &[TransactionOutput]
) -> Result<(), RsdError> {
    for output in outputs {
        if let Some(parent) = output
            .destination
            .parent()
        {
            fs::create_dir_all(parent).map_err(
                |source| RsdError::Io {
                    path: parent.to_path_buf(),
                    source,
                },
            )?;
        }
    }
    Ok(())
}

/// Writes converted outputs as one staged destination transaction.
pub(super) fn write_pending_outputs(
    pending_outputs: Vec<PendingOutput>,
    output_root: &Path,
) -> Result<(), RsdError> {
    let resolved_output_root = resolve_target(output_root)?;
    let transaction_outputs = pending_outputs
        .into_iter()
        .enumerate()
        .map(
            |(index, output)| {
                transaction_output(
                    output, index,
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    validate_destination_namespace(&transaction_outputs)?;
    validate_transaction_paths(
        &transaction_outputs,
        &resolved_output_root,
    )?;
    create_destination_parents(&transaction_outputs)?;
    stage_outputs(&transaction_outputs)?;
    let backed_up = backup_destinations(&transaction_outputs)?;
    commit_staged_outputs(
        &transaction_outputs,
        &backed_up,
    )
}

/// Resolves a target path without requiring its final components to exist.
pub(super) fn resolve_target(path: &Path) -> Result<PathBuf, RsdError> {
    if path
        .as_os_str()
        .is_empty()
    {
        return Err(RsdError::InvalidPath(path.to_path_buf()));
    }
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(
                |source| RsdError::Io {
                    path: path.to_path_buf(),
                    source,
                },
            )?
            .join(path)
    };
    let mut existing = absolute.as_path();
    let mut suffix = Vec::<OsString>::new();
    loop {
        match fs::canonicalize(existing) {
            Ok(mut resolved) => {
                for component in suffix
                    .iter()
                    .rev()
                {
                    resolved.push(component);
                }
                return Ok(resolved);
            }
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                let Some(name) = existing.file_name() else {
                    return Err(RsdError::InvalidPath(path.to_path_buf()));
                };
                suffix.push(name.to_os_string());
                let Some(parent) = existing.parent() else {
                    return Err(RsdError::InvalidPath(path.to_path_buf()));
                };
                existing = parent;
            }
            Err(source) => {
                return Err(
                    RsdError::Io {
                        path: existing.to_path_buf(),
                        source,
                    },
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{resolve_target, write_pending_outputs};
    use crate::adapters::driven::filesystem::PendingOutput;
    use crate::domain::RsdError;

    static CASE_ID: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn empty_target_path_is_rejected() {
        let result = resolve_target(Path::new(""));

        assert!(
            matches!(
                result,
                Err(RsdError::InvalidPath(path))
                    if path.as_os_str().is_empty()
            ),
            "empty target paths must not resolve to the working directory"
        );
    }

    #[test]
    fn duplicate_destinations_fail_before_creating_output_tree()
    -> Result<(), String> {
        let case_id = CASE_ID.fetch_add(
            1,
            Ordering::Relaxed,
        );
        let root = std::env::temp_dir().join(
            format!(
                "schoenwald-rsd-duplicate-output-{}-{case_id}",
                std::process::id()
            ),
        );
        let output_root = root.join("output");
        let destination = output_root.join("tone.wav");
        let pending = vec![
            PendingOutput {
                destination: destination.clone(),
                bytes: vec![1_u8],
            },
            PendingOutput {
                destination: destination.clone(),
                bytes: vec![2_u8],
            },
        ];

        let result = write_pending_outputs(
            pending,
            &output_root,
        );
        let output_exists = output_root.exists();
        let _cleanup = fs::remove_dir_all(&root);

        if !matches!(
            result,
            Err(RsdError::CollidingOutputPath(path)) if path == destination
        ) {
            return Err(
                "duplicate output destinations did not return their typed \
                 collision"
                    .to_owned(),
            );
        }
        if output_exists {
            return Err(
                "duplicate output preflight created destination state"
                    .to_owned(),
            );
        }
        Ok(())
    }

    #[cfg(windows)]
    #[test]
    fn case_folded_transaction_collision_fails_before_output_tree()
    -> Result<(), String> {
        let case_id = CASE_ID.fetch_add(
            1,
            Ordering::Relaxed,
        );
        let root = std::env::temp_dir().join(
            format!(
                "schoenwald-rsd-case-output-{}-{case_id}",
                std::process::id()
            ),
        );
        let output_root = root.join("output");
        let reserved = output_root.join(".RSD-EXPORT-0000000000000001.TMP");
        let pending = vec![
            PendingOutput {
                destination: reserved.join("nested.wav"),
                bytes: vec![1_u8],
            },
            PendingOutput {
                destination: output_root.join("z.wav"),
                bytes: vec![2_u8],
            },
        ];

        let result = write_pending_outputs(
            pending,
            &output_root,
        );
        let output_exists = output_root.exists();
        let _cleanup = fs::remove_dir_all(&root);

        if !matches!(
            result,
            Err(RsdError::CollidingOutputPath(_))
        ) {
            return Err(
                "case-folded transaction paths did not return a collision"
                    .to_owned(),
            );
        }
        if output_exists {
            return Err(
                "case-folded transaction preflight created output state"
                    .to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn transaction_namespace_collision_fails_before_output_tree()
    -> Result<(), String> {
        let case_id = CASE_ID.fetch_add(
            1,
            Ordering::Relaxed,
        );
        let root = std::env::temp_dir().join(
            format!(
                "schoenwald-rsd-transaction-output-{}-{case_id}",
                std::process::id()
            ),
        );
        let output_root = root.join("output");
        let reserved = output_root.join(".rsd-export-0000000000000001.tmp");
        let nested = reserved.join("nested.wav");
        let pending = vec![
            PendingOutput {
                destination: nested,
                bytes: vec![1_u8],
            },
            PendingOutput {
                destination: output_root.join("z.wav"),
                bytes: vec![2_u8],
            },
        ];

        let result = write_pending_outputs(
            pending,
            &output_root,
        );
        let output_exists = output_root.exists();
        let _cleanup = fs::remove_dir_all(&root);

        if !matches!(
            result,
            Err(RsdError::CollidingOutputPath(ref path))
                if path.starts_with(&reserved) && path != &reserved
        ) {
            return Err(
                "transaction namespace did not return its typed collision"
                    .to_owned(),
            );
        }
        if output_exists {
            return Err(
                "transaction namespace preflight created destination state"
                    .to_owned(),
            );
        }
        Ok(())
    }
}
