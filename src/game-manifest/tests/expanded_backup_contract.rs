// File:
//   - expanded_backup_contract.rs
// Path:
//   - src/game-manifest/tests/expanded_backup_contract.rs
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
//   - Expanded-manifest backup exclusion regressions.
// - Must-Not:
//   - Read licensed inputs or repository-local generated trees.
// - Allows:
//   - Synthetic Schoenwald-original backups and compiled generator execution.
// - Split-When:
//   - Split when backup identity requires source-specific fixtures.
// - Merge-When:
//   - Another test owns backup exclusion across expanded sources.
// - Summary:
//   - Protects expanded generation from Schoenwald-original contamination.
// - Description:
//   - Verifies backups never become expanded manifest records or failures.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Temporary fixtures are removed after each test.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Expanded-manifest backup exclusion regression coverage.
//!
//! Synthetic backups prove parity with the minimum scanner without reading
//! repository-local source or output trees.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn generate_with_backup() -> io::Result<(
    std::process::Output,
    String,
)> {
    let sequence = NEXT_FIXTURE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let root = std::env::temp_dir().join(
        format!(
            "game-manifest-expanded-backup-{}-{sequence}",
            std::process::id()
        ),
    );
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    let game = root.join("input");
    let extracted = root.join("decoded");
    let output_path = root.join("result.jsonl");
    fs::create_dir_all(&game)?;
    fs::create_dir_all(&extracted)?;
    fs::write(
        game.join("asset.p3d"),
        b"fixture",
    )?;
    fs::write(
        game.join("asset.p3d.schoenwald-original"),
        b"backup",
    )?;
    let result = (|| {
        let output =
            Command::new(env!("CARGO_BIN_EXE_generate-expanded-manifest"))
                .arg(&game)
                .arg(&extracted)
                .arg(&output_path)
                .output()?;
        let manifest = fs::read_to_string(output_path)?;
        Ok(
            (
                output, manifest,
            ),
        )
    })();
    drop(fs::remove_dir_all(&root));
    result
}

fn run_backup_only() -> io::Result<std::process::Output> {
    let sequence = NEXT_FIXTURE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let root = std::env::temp_dir().join(
        format!(
            "game-manifest-expanded-backup-only-{}-{sequence}",
            std::process::id()
        ),
    );
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    let game = root.join("input");
    let extracted = root.join("decoded");
    let output_path = root.join("result.jsonl");
    fs::create_dir_all(&game)?;
    fs::create_dir_all(&extracted)?;
    fs::write(
        game.join("asset.p3d.schoenwald-original"),
        b"backup",
    )?;
    let result = Command::new(env!("CARGO_BIN_EXE_generate-expanded-manifest"))
        .arg(&game)
        .arg(&extracted)
        .arg(&output_path)
        .output();
    drop(fs::remove_dir_all(&root));
    result
}

#[test]
fn expanded_generator_ignores_backup_files() {
    let result = generate_with_backup();
    assert!(result.is_ok());
    let Some((output, manifest)) = result.ok() else {
        return;
    };
    assert!(
        output
            .status
            .success()
    );
    assert!(!manifest.contains("asset.p3d.schoenwald-original"));
}

#[test]
fn expanded_generator_rejects_backup_only_game() {
    let result = run_backup_only();
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

fn run_extracted_backup_only() -> io::Result<(
    std::process::Output,
    bool,
)> {
    let sequence = NEXT_FIXTURE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let root = std::env::temp_dir().join(
        format!(
            "game-manifest-extracted-backup-only-{}-{sequence}",
            std::process::id()
        ),
    );
    match fs::remove_dir_all(&root) {
        Ok(()) => {}
        Err(error) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error),
    }
    let game = root.join("input");
    let extracted = root.join("decoded");
    let output_path = root.join("result.jsonl");
    fs::create_dir_all(&game)?;
    fs::create_dir_all(&extracted)?;
    fs::write(
        game.join("asset.p3d"),
        b"fixture",
    )?;
    fs::write(
        game.join("source.rcf"),
        b"fixture",
    )?;
    fs::write(
        extracted.join("asset.p3d.schoenwald-original"),
        b"backup",
    )?;
    let result = Command::new(env!("CARGO_BIN_EXE_generate-expanded-manifest"))
        .arg(&game)
        .arg(&extracted)
        .arg(&output_path)
        .output()
        .map(
            |output| {
                (
                    output,
                    output_path.exists(),
                )
            },
        );
    drop(fs::remove_dir_all(&root));
    result
}

#[test]
fn expanded_generator_rejects_extracted_backup_only_evidence() {
    let result = run_extracted_backup_only();
    assert!(result.is_ok());
    let Some((output, output_exists)) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
    assert!(!output_exists);
}
