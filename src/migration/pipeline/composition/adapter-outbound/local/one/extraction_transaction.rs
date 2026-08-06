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
//   - Recoverable whole-root extraction staging and publication.
// - Must-Not:
//   - Interpret source assets, stage reports, or optional-package policy.
// - Allows:
//   - Real-directory inspection, sibling transaction artifacts, and rename.
// - Split-When:
//   - Split when another generated root needs the same recovery protocol.
// - Merge-When:
//   - Merge when extraction publication no longer has an independent lifecycle.
// - Summary:
//   - Recoverable extraction-root transaction.
// - Description:
//   - Builds below a sibling candidate and preserves the last accepted root
//     until complete publication succeeds.
// - Usage:
//   - Created once around every complete extraction command.
// - Defaults:
//   - Unknown, malformed, or symlinked transaction artifacts fail closed.
//

//! Recoverable whole-root extraction staging and publication.

use std::ffi::OsString;
use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use super::cleanup::remove_generated_tree;
use crate::domain::{PipelineError, PipelineOutcome};
use schoenwald_filesystem::adapters::driving::local as local_filesystem;

const STATE_BYTES: &[u8] =
    b"{\"schema\":\"shar-schoenwald.extraction-transaction.v1\"}\n";

/// One recoverable whole-root extraction transaction.
#[derive(Debug)]
pub(super) struct ExtractionTransaction {
    paths: TransactionPaths,
    _lease: StateLease,
}

#[derive(Debug)]
pub(super) struct ExtractionOutputLease {
    paths: TransactionPaths,
    lease: StateLease,
}

#[derive(Debug)]
struct StateLease {
    _file: File,
}

#[derive(Debug)]
struct TransactionPaths {
    destination: PathBuf,
    staging: PathBuf,
    backup: PathBuf,
    state: PathBuf,
    lock: PathBuf,
}

impl ExtractionOutputLease {
    /// Serialize one output root and recover an interrupted full publication.
    pub(super) fn acquire(destination: &Path) -> PipelineOutcome<Self> {
        let paths = TransactionPaths::new(destination)?;
        let lease = acquire_transaction_lease(&paths.lock)?;
        recover_interrupted(&paths)?;
        ensure_real_directory_or_missing(
            &paths.destination,
            "extraction output",
        )?;
        ensure_missing(&paths.staging, "extraction staging")?;
        ensure_missing(&paths.backup, "extraction backup")?;
        ensure_missing(&paths.state, "extraction transaction state")?;
        Ok(Self { paths, lease })
    }

    fn into_parts(self) -> (TransactionPaths, StateLease) {
        (self.paths, self.lease)
    }
}

impl ExtractionTransaction {
    /// Recover any interrupted publication and create one empty candidate root.
    pub(super) fn begin(destination: &Path) -> PipelineOutcome<Self> {
        let output_lease = ExtractionOutputLease::acquire(destination)?;
        let (paths, lease) = output_lease.into_parts();
        write_state(&paths.state)?;
        if let Err(error) = local_filesystem::create_dir_all(&paths.staging) {
            let failure = io_failure("create extraction staging", &error);
            return Err(cleanup_state_failure(&paths.state, failure));
        }
        Ok(Self {
            paths,
            _lease: lease,
        })
    }

    /// Return the isolated candidate root used by every extraction stage.
    pub(super) fn staging_root(&self) -> &Path {
        &self.paths.staging
    }

    /// Replace the accepted root with the complete candidate.
    pub(super) fn publish(self) -> PipelineOutcome<()> {
        let had_destination = path_present(&self.paths.destination)?;
        if had_destination
            && let Err(error) =
                fs::rename(&self.paths.destination, &self.paths.backup)
        {
            let failure = io_failure("back up extraction output", &error);
            return Err(recover_failure(&self.paths, failure));
        }
        if let Err(error) =
            fs::rename(&self.paths.staging, &self.paths.destination)
        {
            let failure = io_failure("publish extraction output", &error);
            return Err(recover_failure(&self.paths, failure));
        }
        recover_artifacts(&self.paths)
    }

    /// Remove a failed candidate while restoring any accepted backup.
    pub(super) fn abort(self) -> PipelineOutcome<()> {
        recover_artifacts(&self.paths)
    }
}

impl TransactionPaths {
    fn new(destination: &Path) -> PipelineOutcome<Self> {
        let name = destination.file_name().ok_or_else(|| {
            PipelineError::new("extraction output has no final path segment")
        })?;
        let parent = destination
            .parent()
            .filter(|candidate| !candidate.as_os_str().is_empty())
            .unwrap_or_else(|| Path::new("."));
        if parent != Path::new(".") {
            local_filesystem::create_dir_all(parent).map_err(|error| {
                io_failure("create extraction parent", &error)
            })?;
        }
        ensure_real_directory(parent, "extraction parent")?;
        Ok(Self {
            destination: destination.to_path_buf(),
            staging: sibling_path(parent, name, ".pipeline-staging"),
            backup: sibling_path(parent, name, ".pipeline-backup"),
            state: sibling_path(parent, name, ".pipeline-transaction.json"),
            lock: sibling_path(parent, name, ".pipeline-lock"),
        })
    }
}

fn sibling_path(
    parent: &Path,
    name: &std::ffi::OsStr,
    suffix: &str,
) -> PathBuf {
    let sibling = sibling_name(name, suffix);
    if parent == Path::new(".") {
        PathBuf::from(sibling)
    } else {
        parent.join(sibling)
    }
}

fn sibling_name(name: &std::ffi::OsStr, suffix: &str) -> OsString {
    let mut result = OsString::from(".");
    result.push(name);
    result.push(suffix);
    result
}

fn recover_interrupted(paths: &TransactionPaths) -> PipelineOutcome<()> {
    if !path_present(&paths.state)? {
        if path_present(&paths.staging)? || path_present(&paths.backup)? {
            return Err(PipelineError::new(concat!(
                "unowned extraction transaction artifacts exist; ",
                "inspect them before retrying"
            )));
        }
        return Ok(());
    }
    validate_state(&paths.state)?;
    recover_artifacts(paths)
}

fn recover_artifacts(paths: &TransactionPaths) -> PipelineOutcome<()> {
    ensure_real_directory_or_missing(&paths.destination, "extraction output")?;
    ensure_real_directory_or_missing(&paths.staging, "extraction staging")?;
    ensure_real_directory_or_missing(&paths.backup, "extraction backup")?;
    ensure_real_file(&paths.state, "extraction transaction state")?;

    if path_present(&paths.backup)? && !path_present(&paths.destination)? {
        fs::rename(&paths.backup, &paths.destination)
            .map_err(|error| io_failure("restore extraction backup", &error))?;
    } else if path_present(&paths.backup)? {
        remove_real_directory(&paths.backup, "extraction backup")?;
    }
    if path_present(&paths.staging)? {
        remove_real_directory(&paths.staging, "extraction staging")?;
    }
    remove_state(&paths.state)
}

fn acquire_transaction_lease(path: &Path) -> PipelineOutcome<StateLease> {
    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(path)
    {
        Ok(file) => file,
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            ensure_real_file(path, "extraction transaction lock")?;
            OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
                .map_err(|open_error| {
                    io_failure("open extraction transaction lock", &open_error)
                })?
        }
        Err(error) => {
            return Err(io_failure(
                "create extraction transaction lock",
                &error,
            ));
        }
    };
    match file.try_lock() {
        Ok(()) => {
            let metadata = file.metadata().map_err(|error| {
                io_failure("inspect extraction transaction lock", &error)
            })?;
            if metadata.len() != 0 {
                return Err(PipelineError::new(
                    "extraction transaction lock must be empty",
                ));
            }
            Ok(StateLease { _file: file })
        }
        Err(TryLockError::WouldBlock) => Err(PipelineError::new(
            "an active extraction transaction already owns this output",
        )),
        Err(TryLockError::Error(error)) => {
            Err(io_failure("lock extraction transaction", &error))
        }
    }
}

fn write_state(path: &Path) -> PipelineOutcome<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| {
        io_failure("create extraction transaction state", &error)
    })?;
    let result = (|| {
        file.write_all(STATE_BYTES).map_err(|error| {
            io_failure("write extraction transaction state", &error)
        })?;
        file.sync_all().map_err(|error| {
            io_failure("sync extraction transaction state", &error)
        })
    })();
    drop(file);
    match result {
        Ok(()) => Ok(()),
        Err(failure) => Err(cleanup_state_failure(path, failure)),
    }
}

fn validate_state(path: &Path) -> PipelineOutcome<()> {
    ensure_real_file(path, "extraction transaction state")?;
    let metadata = fs::metadata(path).map_err(|error| {
        io_failure("inspect extraction transaction state", &error)
    })?;
    if metadata.len() != u64::try_from(STATE_BYTES.len()).unwrap_or(u64::MAX) {
        return Err(PipelineError::new(
            "extraction transaction state is malformed",
        ));
    }
    let bytes = fs::read(path).map_err(|error| {
        io_failure("read extraction transaction state", &error)
    })?;
    if bytes == STATE_BYTES {
        Ok(())
    } else {
        Err(PipelineError::new(
            "extraction transaction state is malformed",
        ))
    }
}

fn remove_state(path: &Path) -> PipelineOutcome<()> {
    ensure_real_file(path, "extraction transaction state")?;
    fs::remove_file(path).map_err(|error| {
        io_failure("remove extraction transaction state", &error)
    })
}

fn ensure_missing(path: &Path, label: &str) -> PipelineOutcome<()> {
    if path_present(path)? {
        Err(PipelineError::new(format!("{label} already exists")))
    } else {
        Ok(())
    }
}

fn ensure_real_directory_or_missing(
    path: &Path,
    label: &str,
) -> PipelineOutcome<()> {
    if path_present(path)? {
        ensure_real_directory(path, label)
    } else {
        Ok(())
    }
}

fn ensure_real_directory(path: &Path, label: &str) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_failure("inspect extraction directory", &error))?;
    if metadata.is_dir() && !metadata.file_type().is_symlink() {
        Ok(())
    } else {
        Err(PipelineError::new(format!(
            "{label} must be a real directory"
        )))
    }
}

fn ensure_real_file(path: &Path, label: &str) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_failure("inspect extraction state", &error))?;
    if metadata.is_file() && !metadata.file_type().is_symlink() {
        Ok(())
    } else {
        Err(PipelineError::new(format!("{label} must be a real file")))
    }
}

fn remove_real_directory(path: &Path, label: &str) -> PipelineOutcome<()> {
    ensure_real_directory(path, label)?;
    remove_generated_tree(path)
        .map_err(|error| io_failure("remove extraction directory", &error))
}

fn path_present(path: &Path) -> PipelineOutcome<bool> {
    match fs::symlink_metadata(path) {
        Ok(_metadata) => Ok(true),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(io_failure("inspect extraction path", &error)),
    }
}

fn cleanup_state_failure(path: &Path, failure: PipelineError) -> PipelineError {
    match fs::remove_file(path) {
        Ok(()) => failure,
        Err(cleanup) if cleanup.kind() == io::ErrorKind::NotFound => failure,
        Err(cleanup) => PipelineError::new(format!(
            "{failure}; extraction state cleanup failed ({:?})",
            cleanup.kind()
        )),
    }
}

fn recover_failure(
    paths: &TransactionPaths,
    failure: PipelineError,
) -> PipelineError {
    match recover_artifacts(paths) {
        Ok(()) => failure,
        Err(recovery) => PipelineError::new(format!(
            "{failure}; extraction transaction recovery failed: {recovery}"
        )),
    }
}

fn io_failure(operation: &str, error: &io::Error) -> PipelineError {
    PipelineError::new(format!("{operation} failed ({:?})", error.kind()))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/one/extraction_transaction/tests.rs"]
mod tests;
