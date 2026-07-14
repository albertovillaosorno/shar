// File:
//   - generate_cli_contract.rs
// Path:
//   - src/game-manifest/tests/generate_cli_contract.rs
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
//   - End-to-end minimum-manifest generator command regressions.
// - Must-Not:
//   - Read licensed inputs or repository-local generated trees.
// - Allows:
//   - Synthetic temporary files and compiled generator execution.
// - Split-When:
//   - Split when output persistence requires independent fixture support.
// - Merge-When:
//   - Another test owns the same generate-manifest command boundary.
// - Summary:
//   - Protects fail-closed generator command behavior.
// - Description:
//   - Executes generate-manifest against isolated synthetic directory trees.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Temporary fixtures are removed after each command invocation.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! End-to-end minimum-manifest generator contract coverage.
//!
//! Synthetic trees prove operator command behavior without exposing source
//! names or depending on repository-local outputs.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn run_generator(
    extension: Option<&str>,
    extra_argument: Option<&str>,
) -> io::Result<Output> {
    let sequence = NEXT_FIXTURE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let root = std::env::temp_dir().join(
        format!(
            "game-manifest-generate-{}-{sequence}",
            std::process::id()
        ),
    );
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    fs::create_dir_all(&root)?;
    let result = (|| {
        if let Some(file_extension) = extension {
            fs::write(
                root.join(format!("asset.{file_extension}")),
                b"fixture",
            )?;
        }
        let mut command = Command::new(env!("CARGO_BIN_EXE_generate-manifest"));
        let _root = command.arg(&root);
        if let Some(extra) = extra_argument {
            let _extra = command.arg(extra);
        }
        command.output()
    })();
    drop(fs::remove_dir_all(&root));
    result
}

#[test]
fn generator_rejects_extra_arguments() {
    let result = run_generator(
        Some("p3d"),
        Some("unexpected"),
    );
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
}

#[test]
fn generator_rejects_unclassified_buckets() {
    let result = run_generator(
        Some("mystery"),
        None,
    );
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("unclassified bucket")
    );
}

#[test]
fn generator_rejects_empty_game_directory() {
    let result = run_generator(
        None, None,
    );
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
}
