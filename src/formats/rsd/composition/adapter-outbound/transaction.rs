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
//   - Transaction outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Transaction outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for rsd.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Transaction outbound adapter.

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
        },
        Ok(metadata) if metadata.permissions().readonly() => {
            return Err(RsdError::InvalidPath(path.to_path_buf()));
        },
        Ok(_) => {},
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => {},
        Err(source) => {
            return Err(RsdError::io(path.to_path_buf(), source.to_string()));
        },
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
            },
            Err(source) => {
                return Err(RsdError::io(
                    ancestor.to_path_buf(),
                    source.to_string(),
                ));
            },
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
        },
        Err(source) => {
            Err(RsdError::io(path.to_path_buf(), source.to_string()))
        },
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
    let temporary = transaction_path(&output.destination, index, "tmp")?;
    let backup = transaction_path(&output.destination, index, "bak")?;
    Ok(TransactionOutput {
        destination: output.destination,
        bytes: output.bytes,
        temporary,
        backup,
    })
}

/// Writes one complete staging file without replacing an existing entry.
fn stage_output(output: &TransactionOutput) -> Result<(), RsdError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output.temporary)
        .map_err(|source| {
            RsdError::io(output.temporary.clone(), source.to_string())
        })?;
    if let Err(source) = file.write_all(&output.bytes) {
        remove_transaction_entry(&output.temporary);
        return Err(RsdError::io(output.temporary.clone(), source.to_string()));
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
fn rollback_backups(backed_up: &[(PathBuf, PathBuf)]) {
    for (destination, backup) in backed_up.iter().rev() {
        remove_transaction_entry(destination);
        let _restore_result = fs::rename(backup, destination);
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
    outputs: &[TransactionOutput],
) -> Result<Vec<(PathBuf, PathBuf)>, RsdError> {
    let mut backed_up = Vec::new();
    for output in outputs {
        if !path_entry_exists(&output.destination)? {
            continue;
        }
        if let Err(source) = fs::rename(&output.destination, &output.backup) {
            rollback_backups(&backed_up);
            cleanup_staging(outputs);
            return Err(RsdError::io(
                output.destination.clone(),
                source.to_string(),
            ));
        }
        backed_up.push((output.destination.clone(), output.backup.clone()));
    }
    Ok(backed_up)
}

/// Commits all staged files or restores the complete previous destination set.
fn commit_staged_outputs(
    outputs: &[TransactionOutput],
    backed_up: &[(PathBuf, PathBuf)],
) -> Result<(), RsdError> {
    let mut committed = Vec::<PathBuf>::new();
    for output in outputs {
        if let Err(source) = fs::rename(&output.temporary, &output.destination)
        {
            for destination in committed {
                remove_transaction_entry(&destination);
            }
            rollback_backups(backed_up);
            cleanup_staging(outputs);
            return Err(RsdError::io(
                output.destination.clone(),
                source.to_string(),
            ));
        }
        committed.push(output.destination.clone());
    }
    for (_, backup) in backed_up {
        fs::remove_file(backup).map_err(|source| {
            RsdError::io(backup.clone(), source.to_string())
        })?;
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
    outputs: &[TransactionOutput],
) -> Result<(), RsdError> {
    let mut destinations = BTreeMap::new();
    for output in outputs {
        for path in [&output.destination, &output.temporary, &output.backup] {
            let identity = path_identity(path);
            let replaced = destinations.insert(identity, path.clone());
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
        check_destination(&output.destination, resolved_output_root)?;
        for path in [&output.temporary, &output.backup] {
            check_destination(path, resolved_output_root)?;
            if path_entry_exists(path)? {
                return Err(RsdError::InvalidPath(path.clone()));
            }
        }
    }
    Ok(())
}

/// Creates every destination parent before staging starts.
fn create_destination_parents(
    outputs: &[TransactionOutput],
) -> Result<(), RsdError> {
    for output in outputs {
        if let Some(parent) = output.destination.parent() {
            fs::create_dir_all(parent).map_err(|source| {
                RsdError::io(parent.to_path_buf(), source.to_string())
            })?;
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
        .map(|(index, output)| transaction_output(output, index))
        .collect::<Result<Vec<_>, _>>()?;
    validate_destination_namespace(&transaction_outputs)?;
    validate_transaction_paths(&transaction_outputs, &resolved_output_root)?;
    create_destination_parents(&transaction_outputs)?;
    stage_outputs(&transaction_outputs)?;
    let backed_up = backup_destinations(&transaction_outputs)?;
    commit_staged_outputs(&transaction_outputs, &backed_up)
}

/// Resolves a target path without requiring its final components to exist.
pub(super) fn resolve_target(path: &Path) -> Result<PathBuf, RsdError> {
    if path.as_os_str().is_empty() {
        return Err(RsdError::InvalidPath(path.to_path_buf()));
    }
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|source| {
                RsdError::io(path.to_path_buf(), source.to_string())
            })?
            .join(path)
    };
    let mut existing = absolute.as_path();
    let mut suffix = Vec::<OsString>::new();
    loop {
        match fs::canonicalize(existing) {
            Ok(mut resolved) => {
                for component in suffix.iter().rev() {
                    resolved.push(component);
                }
                return Ok(resolved);
            },
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                let Some(name) = existing.file_name() else {
                    return Err(RsdError::InvalidPath(path.to_path_buf()));
                };
                suffix.push(name.to_os_string());
                let Some(parent) = existing.parent() else {
                    return Err(RsdError::InvalidPath(path.to_path_buf()));
                };
                existing = parent;
            },
            Err(source) => {
                return Err(RsdError::io(
                    existing.to_path_buf(),
                    source.to_string(),
                ));
            },
        }
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/rsd/unit/adapter-outbound/transaction/tests.rs"]
mod tests;
