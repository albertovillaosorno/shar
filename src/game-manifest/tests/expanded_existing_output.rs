// File:
//   - expanded_existing_output.rs
// Path:
//   - src/game-manifest/tests/expanded_existing_output.rs
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
//   - Existing expanded-output replacement regressions.
// - Must-Not:
//   - Read licensed inputs or repository-local generated trees.
// - Allows:
//   - Synthetic temporary trees and compiled generator execution.
// - Split-When:
//   - Split when output identity needs independent parser fixtures.
// - Merge-When:
//   - Another test owns the same existing-output protection boundary.
// - Summary:
//   - Protects unrelated JSONL files from expanded-output replacement.
// - Description:
//   - Executes generation with a preexisting non-manifest JSONL destination.
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

//! Existing expanded-output replacement regression coverage.
//!
//! Synthetic destinations prove the generator overwrites only its own durable
//! expanded ledger format.

use std::fs;
use std::io::{self, ErrorKind};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest::{EXPANDED_SCHEMA_LINE, kind_taxonomy_jsonl};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

fn run_with_existing_output(
    existing: &str
) -> io::Result<(
    std::process::Output,
    String,
)> {
    let sequence = NEXT_FIXTURE.fetch_add(
        1,
        Ordering::Relaxed,
    );
    let root = std::env::temp_dir().join(
        format!(
            "game-manifest-existing-output-{}-{sequence}",
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
    let output_path = game.join("custom.jsonl");
    fs::create_dir_all(&game)?;
    fs::create_dir_all(&extracted)?;
    fs::write(
        game.join("asset.p3d"),
        b"fixture",
    )?;
    fs::write(
        &output_path,
        existing,
    )?;
    let result = (|| {
        let output =
            Command::new(env!("CARGO_BIN_EXE_generate-expanded-manifest"))
                .arg(&game)
                .arg(&extracted)
                .arg(&output_path)
                .output()?;
        let remaining = fs::read_to_string(&output_path)?;
        Ok(
            (
                output, remaining,
            ),
        )
    })();
    drop(fs::remove_dir_all(&root));
    result
}

#[test]
fn expanded_output_preserves_unrelated_jsonl() {
    let result = run_with_existing_output(
        "unrelated-ledger
",
    );
    assert!(result.is_ok());
    let Some((output, remaining)) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
    assert_eq!(
        remaining,
        "unrelated-ledger\n"
    );
}

#[test]
fn expanded_output_rejects_spoofed_second_line() {
    let existing = format!(
        "unrelated-ledger
{EXPANDED_SCHEMA_LINE}
",
    );
    let result = run_with_existing_output(&existing);
    assert!(result.is_ok());
    let Some((output, remaining)) = result.ok() else {
        return;
    };
    assert!(
        !output
            .status
            .success()
    );
    assert_eq!(
        remaining,
        existing
    );
}

#[test]
fn expanded_output_rejects_noncanonical_line_endings() {
    let taxonomy = kind_taxonomy_jsonl();
    for existing in [
        format!("{taxonomy}\r\n{EXPANDED_SCHEMA_LINE}\r\n"),
        format!("{taxonomy}\n{EXPANDED_SCHEMA_LINE}"),
    ] {
        let result = run_with_existing_output(&existing);
        assert!(result.is_ok());
        let Some((output, remaining)) = result.ok() else {
            continue;
        };
        assert!(
            !output
                .status
                .success()
        );
        assert_eq!(
            remaining,
            existing
        );
    }
}
