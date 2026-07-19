// File:
//   - lease.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/run_registry/storage/lease.rs
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
//   - Cooperative cancellation markers and exclusive create-new leases.
// - Must-Not:
//   - Enumerate active records, estimate progress, or terminate processes.
// - Allows:
//   - Create and release bounded local control markers.
// - Summary:
//   - Cooperative lease controls for pipeline runs.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Cooperative cancellation and exclusive lease storage.

use std::fs::{self, OpenOptions};
use std::io::{ErrorKind, Write as _};
use std::time::SystemTime;

use super::{RegistryStorage, STALE_AFTER_MS, validate_run_id};

/// Cooperative cancellation request marker.
const CANCEL_FILE: &str = "cancel.request";
/// Create-new lease used to serialize default starts.
const EXCLUSIVE_LOCK: &str = "exclusive.lock";

impl RegistryStorage {
    /// Create one cooperative cancellation marker.
    pub(in super::super) fn request_cancel(
        &self,
        run_id: &str,
    ) -> Result<(), String> {
        validate_run_id(run_id)?;
        let run_dir = self.run_dir(run_id);
        if !run_dir.is_dir() {
            return Err(format!("active pipeline run not found: {run_id}"));
        }
        let path = run_dir.join(CANCEL_FILE);
        match OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
        {
            Ok(mut file) => file
                .write_all(b"cancel\n")
                .map_err(
                    |error| {
                        format!("cancellation request write failed: {error}")
                    },
                ),
            Err(error) if error.kind() == ErrorKind::AlreadyExists => Ok(()),
            Err(error) => {
                Err(format!("cancellation request creation failed: {error}"))
            }
        }
    }

    /// Return whether one run received a cooperative cancellation request.
    pub(in super::super) fn cancellation_requested(
        &self,
        run_id: &str,
    ) -> bool {
        self.run_dir(run_id)
            .join(CANCEL_FILE)
            .is_file()
    }

    /// Acquire the create-new lease that serializes default starts.
    pub(in super::super) fn acquire_exclusive(
        &self,
        run_id: &str,
    ) -> Result<(), String> {
        validate_run_id(run_id)?;
        self.prepare()?;
        let path = self
            .root
            .join(EXCLUSIVE_LOCK);
        for _attempt in 0_u8..2_u8 {
            match OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
            {
                Ok(mut file) => {
                    return file
                        .write_all(run_id.as_bytes())
                        .and_then(|()| file.write_all(b"\n"))
                        .map_err(
                            |error| {
                                format!(
                                    "pipeline exclusive lease write failed: \
                                     {error}"
                                )
                            },
                        );
                }
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {
                    if self.remove_abandoned_exclusive(&path)? {
                        continue;
                    }
                    let owner = fs::read_to_string(&path)
                        .unwrap_or_else(|_read_error| String::from("unknown"));
                    return Err(
                        format!(
                            "pipeline exclusive lease is owned by {}",
                            owner.trim()
                        ),
                    );
                }
                Err(error) => {
                    return Err(
                        format!(
                            "pipeline exclusive lease creation failed: {error}"
                        ),
                    );
                }
            }
        }
        Err(String::from("pipeline exclusive lease could not be recovered"))
    }

    /// Remove one old lock that has no corresponding active-run directory.
    fn remove_abandoned_exclusive(
        &self,
        path: &std::path::Path,
    ) -> Result<bool, String> {
        let owner = fs::read_to_string(path).map_err(
            |error| format!("pipeline exclusive lease read failed: {error}"),
        )?;
        if self
            .run_dir(owner.trim())
            .is_dir()
        {
            return Ok(false);
        }
        let modified = fs::metadata(path)
            .and_then(|metadata| metadata.modified())
            .map_err(
                |error| {
                    format!("pipeline exclusive lease metadata failed: {error}")
                },
            )?;
        let age = SystemTime::now()
            .duration_since(modified)
            .map_err(
                |error| {
                    format!(
                        "pipeline exclusive lease timestamp is in the future: \
                         {error}"
                    )
                },
            )?;
        let age_ms = u64::try_from(age.as_millis()).map_err(
            |error| format!("pipeline exclusive lease age overflowed: {error}"),
        )?;
        if age_ms <= STALE_AFTER_MS {
            return Ok(false);
        }
        match fs::remove_file(path) {
            Ok(()) => Ok(true),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(true),
            Err(error) => Err(
                format!(
                    "pipeline abandoned exclusive lease removal failed: \
                     {error}"
                ),
            ),
        }
    }

    /// Release the exclusive lease only when the expected run still owns it.
    pub(in super::super) fn release_exclusive(
        &self,
        run_id: &str,
    ) -> Result<(), String> {
        validate_run_id(run_id)?;
        let path = self
            .root
            .join(EXCLUSIVE_LOCK);
        let owner = match fs::read_to_string(&path) {
            Ok(owner) => owner,
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(()),
            Err(error) => {
                return Err(
                    format!("pipeline exclusive lease read failed: {error}"),
                );
            }
        };
        if owner.trim() != run_id {
            return Ok(());
        }
        fs::remove_file(&path).map_err(
            |error| format!("pipeline exclusive lease release failed: {error}"),
        )
    }
}
