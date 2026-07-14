// File:
//   - cleanup.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/one/cleanup.rs
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
//   - Reliable removal of generated extraction trees on local filesystems.
// - Must-Not:
//   - Remove caller paths outside the explicitly supplied generated root.
// - Allows:
//   - Retry transient Windows deletion races with bounded backoff.
// - Split-When:
//   - Split when another cleanup policy needs independent retry semantics.
// - Merge-When:
//   - Another phase-one adapter owns identical generated-tree removal policy.
// - Summary:
//   - Removes generated extraction trees despite transient Windows races.
// - Description:
//   - Retries directory-not-empty, permission, and would-block failures before
//   - returning the final filesystem error to the pipeline.
// - Usage:
//   - Called only before a clean `extract-game` run recreates `extracted/`.
// - Defaults:
//   - Missing paths succeed and retries are bounded to avoid indefinite waits.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Removes generated extraction trees with bounded Windows-friendly retries.
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
            std::thread::sleep(
                Duration::from_millis(
                    RETRY_DELAY_MILLISECONDS.saturating_mul(multiplier),
                ),
            );
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
            }
            Err(error)
                if is_transient_removal_error(&error)
                    && attempt.saturating_add(1) < bounded_attempts =>
            {
                pause(attempt.saturating_add(1));
            }
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
mod tests {
    use std::cell::Cell;
    use std::io;
    use std::path::Path;

    use super::remove_with_retries;

    #[test]
    fn retries_transient_directory_not_empty_failures() {
        let calls = Cell::new(0usize);
        let pauses = Cell::new(0usize);
        let result = remove_with_retries(
            Path::new("extracted"),
            4,
            |_path| {
                let call = calls
                    .get()
                    .saturating_add(1);
                calls.set(call);
                if call < 3 {
                    Err(
                        io::Error::new(
                            io::ErrorKind::DirectoryNotEmpty,
                            "transient directory race",
                        ),
                    )
                } else {
                    Ok(())
                }
            },
            |_retry| {
                pauses.set(
                    pauses
                        .get()
                        .saturating_add(1),
                );
            },
        );
        assert!(result.is_ok());
        assert_eq!(
            calls.get(),
            3
        );
        assert_eq!(
            pauses.get(),
            2
        );
    }

    #[test]
    fn missing_tree_is_already_clean() {
        let result = remove_with_retries(
            Path::new("extracted"),
            2,
            |_path| {
                Err(
                    io::Error::new(
                        io::ErrorKind::NotFound,
                        "already removed",
                    ),
                )
            },
            |_retry| {},
        );
        assert!(result.is_ok());
    }

    #[test]
    fn permanent_errors_fail_without_retry() {
        let pauses = Cell::new(0usize);
        let result = remove_with_retries(
            Path::new("extracted"),
            4,
            |_path| {
                Err(
                    io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "invalid generated root",
                    ),
                )
            },
            |_retry| {
                pauses.set(
                    pauses
                        .get()
                        .saturating_add(1),
                );
            },
        );
        assert!(result.is_err());
        assert_eq!(
            pauses.get(),
            0
        );
    }
}
