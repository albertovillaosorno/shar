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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

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
            let call = calls.get().saturating_add(1);
            calls.set(call);
            if call < 3 {
                Err(io::Error::new(
                    io::ErrorKind::DirectoryNotEmpty,
                    "transient directory race",
                ))
            } else {
                Ok(())
            }
        },
        |_retry| {
            pauses.set(pauses.get().saturating_add(1));
        },
    );
    assert!(result.is_ok());
    assert_eq!(calls.get(), 3);
    assert_eq!(pauses.get(), 2);
}

#[test]
fn missing_tree_is_already_clean() {
    let result = remove_with_retries(
        Path::new("extracted"),
        2,
        |_path| Err(io::Error::new(io::ErrorKind::NotFound, "already removed")),
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
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "invalid generated root",
            ))
        },
        |_retry| {
            pauses.set(pauses.get().saturating_add(1));
        },
    );
    assert!(result.is_err());
    assert_eq!(pauses.get(), 0);
}
