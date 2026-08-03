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
//   - Cleanup outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Cleanup outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Cleanup outbound adapter.

use std::path::Path;
use std::time::Duration;
use std::{fs, io};

/// Maximum attempts for one generated-tree removal.
const REMOVE_ATTEMPTS: usize = 12;
/// Base delay multiplied by the one-based retry number.
const RETRY_DELAY_MILLISECONDS: u64 = 50;

/// Remove one generated directory tree with bounded transient-error retries.
///
/// # Errors
///
/// Returns the final filesystem error after all retry attempts are exhausted.
pub(super) fn remove_generated_tree(path: &Path) -> io::Result<()> {
    remove_with_retries(
        path,
        REMOVE_ATTEMPTS,
        |candidate| fs::remove_dir_all(candidate),
        |retry| {
            let multiplier = u64::try_from(retry).unwrap_or(u64::MAX);
            std::thread::sleep(Duration::from_millis(
                RETRY_DELAY_MILLISECONDS.saturating_mul(multiplier),
            ));
        },
    )
}

/// Execute one bounded removal policy with injectable operations for tests.
fn remove_with_retries<Remove, Pause>(
    path: &Path,
    attempts: usize,
    mut remove: Remove,
    mut pause: Pause,
) -> io::Result<()>
where
    Remove: FnMut(&Path) -> io::Result<()>,
    Pause: FnMut(usize),
{
    let bounded_attempts = attempts.max(1);
    for attempt in 0..bounded_attempts {
        match remove(path) {
            Ok(()) => return Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Ok(());
            },
            Err(error)
                if is_transient_removal_error(&error)
                    && attempt.saturating_add(1) < bounded_attempts =>
            {
                pause(attempt.saturating_add(1));
            },
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

/// Return whether one local deletion failure is reasonable to retry.
fn is_transient_removal_error(error: &io::Error) -> bool {
    matches!(
        error.kind(),
        io::ErrorKind::DirectoryNotEmpty
            | io::ErrorKind::PermissionDenied
            | io::ErrorKind::WouldBlock
    )
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/one/cleanup/tests.rs"]
mod tests;
