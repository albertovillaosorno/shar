// File:
//   - expanded_output_location.rs
// Path:
//   - src/game-manifest/tests/expanded_output_location.rs
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
//   - Expanded-manifest destination isolation from evidence roots.
// - Must-Not:
//   - Read licensed inputs or repository-local generated trees.
// - Allows:
//   - Synthetic temporary trees and generator process execution.
// - Split-When:
//   - Split when another evidence root gains distinct destination rules.
// - Merge-When:
//   - Another test owns the same output-location contract.
// - Summary:
//   - Protects extracted RCF evidence from generated ledger contamination.
// - Description:
//   - Verifies output publication cannot write beneath an evidence root.
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

//! Expanded-manifest output-location regression coverage.
//!
//! Generated ledgers must remain outside extracted evidence roots.

use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest as _;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new() -> io::Result<Self> {
        let sequence = NEXT_FIXTURE.fetch_add(
            1,
            Ordering::Relaxed,
        );
        let path = std::env::temp_dir().join(
            format!(
                "game-manifest-output-location-{}-{sequence}",
                std::process::id()
            ),
        );
        match fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        fs::create_dir_all(&path)?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.0));
    }
}

#[test]
fn expanded_output_must_not_contaminate_rcf_evidence() {
    let result = (|| -> io::Result<(std::process::Output, bool)> {
        let fixture = FixtureRoot::new()?;
        let game = fixture
            .path()
            .join("input");
        let extracted = fixture
            .path()
            .join("decoded");
        fs::create_dir_all(&game)?;
        fs::create_dir_all(&extracted)?;
        fs::write(
            game.join("asset.p3d"),
            b"fixture",
        )?;
        let output_path = extracted.join("result.jsonl");

        let output = Command::new(
            env!("CARGO_BIN_EXE_generate-expanded-manifest"),
        )
        .arg(&game)
        .arg(&extracted)
        .arg(&output_path)
        .output()?;
        Ok(
            (
                output, output_path.exists(),
            ),
        )
    })();
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

#[test]
fn aliased_evidence_roots_are_rejected() {
    let result = (|| -> io::Result<(std::process::Output, bool)> {
        let fixture = FixtureRoot::new()?;
        let game = fixture
            .path()
            .join("game");
        let alias_parent = fixture
            .path()
            .join("alias");
        fs::create_dir_all(&game)?;
        fs::create_dir_all(&alias_parent)?;
        fs::write(
            game.join("asset.p3d"),
            b"fixture",
        )?;
        fs::write(
            game.join("source.rcf"),
            b"fixture",
        )?;
        let aliased_game = alias_parent
            .join("..")
            .join("game");
        let output_path = fixture
            .path()
            .join("result.jsonl");

        let output = Command::new(
            env!("CARGO_BIN_EXE_generate-expanded-manifest"),
        )
        .arg(&aliased_game)
        .arg(&game)
        .arg(&output_path)
        .output()?;
        Ok(
            (
                output, output_path.exists(),
            ),
        )
    })();
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
