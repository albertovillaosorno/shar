// File:
//   - audit_cli_contract.rs
// Path:
//   - src/game-manifest/tests/audit_cli_contract.rs
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
//   - End-to-end structural-audit command regressions.
// - Must-Not:
//   - Read licensed inputs or repository-local generated trees.
// - Allows:
//   - Synthetic temporary files and compiled audit execution.
// - Split-When:
//   - Split when output parsing requires independent fixtures.
// - Merge-When:
//   - Another test owns the same audit command contract.
// - Summary:
//   - Protects deterministic structural-audit CLI behavior.
// - Description:
//   - Executes ephemeral-structural-audit against isolated synthetic trees.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Temporary fixtures are removed after each test.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! End-to-end structural-audit command regression coverage.
//!
//! Synthetic trees prove operator behavior without reading repository-local
//! source or output trees.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn run_audit(
    extension: &str,
    extra_argument: Option<&str>,
) -> io::Result<Output> {
    let sequence = NEXT_FIXTURE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let root = std::env::temp_dir().join(
        format!(
            "game-manifest-audit-{}-{sequence}",
            std::process::id()
        ),
    );
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    fs::create_dir_all(&root)?;
    fs::write(
        root.join(format!("asset.{extension}")),
        b"fixture",
    )?;
    let mut command =
        Command::new(env!("CARGO_BIN_EXE_ephemeral_structural_audit"));
    let _root = command.arg(&root);
    if let Some(extra) = extra_argument {
        let _extra = command.arg(extra);
    }
    let result = command.output();
    drop(fs::remove_dir_all(&root));
    result
}

#[test]
fn structural_audit_rejects_extra_arguments() {
    let result = run_audit(
        "png",
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
fn structural_audit_ignores_local_backups() {
    let result = run_audit(
        "schoenwald-original",
        None,
    );
    assert!(result.is_ok());
    let Some(output) = result.ok() else {
        return;
    };
    assert!(
        output
            .status
            .success()
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "total_dirty_extensions	0
"
    );
}
