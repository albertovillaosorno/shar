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
    let case_id = CASE_ID.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "schoenwald-rsd-duplicate-output-{}-{case_id}",
        std::process::id()
    ));
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

    let result = write_pending_outputs(pending, &output_root);
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
            "duplicate output preflight created destination state".to_owned()
        );
    }
    Ok(())
}

#[cfg(windows)]
#[test]
fn case_folded_transaction_collision_fails_before_output_tree()
-> Result<(), String> {
    let case_id = CASE_ID.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "schoenwald-rsd-case-output-{}-{case_id}",
        std::process::id()
    ));
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

    let result = write_pending_outputs(pending, &output_root);
    let output_exists = output_root.exists();
    let _cleanup = fs::remove_dir_all(&root);

    if !matches!(result, Err(RsdError::CollidingOutputPath(_))) {
        return Err("case-folded transaction paths did not return a collision"
            .to_owned());
    }
    if output_exists {
        return Err(
            "case-folded transaction preflight created output state".to_owned()
        );
    }
    Ok(())
}

#[test]
fn transaction_namespace_collision_fails_before_output_tree()
-> Result<(), String> {
    let case_id = CASE_ID.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "schoenwald-rsd-transaction-output-{}-{case_id}",
        std::process::id()
    ));
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

    let result = write_pending_outputs(pending, &output_root);
    let output_exists = output_root.exists();
    let _cleanup = fs::remove_dir_all(&root);

    if !matches!(
        &result,
        Err(RsdError::CollidingOutputPath(path))
            if path.starts_with(&reserved) && path != &reserved
    ) {
        return Err("transaction namespace did not return its typed collision"
            .to_owned());
    }
    if output_exists {
        return Err(
            "transaction namespace preflight created destination state"
                .to_owned(),
        );
    }
    Ok(())
}
