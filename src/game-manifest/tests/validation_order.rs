// File:
//   - validation_order.rs
// Path:
//   - src/game-manifest/tests/validation_order.rs
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
//   - Minimum-manifest parsing and tree-access ordering regressions.
// - Must-Not:
//   - Access local game data or repository output directories.
// - Allows:
//   - In-memory ports that expose parse-before-scan ordering.
// - Split-When:
//   - Split when another validation stage needs an independent fixture.
// - Merge-When:
//   - Another test owns the same validation-order boundary.
// - Summary:
//   - Protects malformed manifests from premature tree scans.
// - Description:
//   - Verifies manifest shape fails before current evidence is traversed.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Synthetic inputs remain portable and deterministic.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Minimum-manifest validation-order regression coverage.
//!
//! Manifest shape must be proven before current tree evidence is scanned.

use std::cell::Cell;
use std::io;
use std::path::{Path, PathBuf};

use game_manifest::{GameTree, PathKind, TextArtifactStore, ValidateManifest};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[derive(Default)]
struct ScanObservingTree {
    scanned: Cell<bool>,
}

impl GameTree for ScanObservingTree {
    fn kind(
        &self,
        _path: &Path,
    ) -> io::Result<PathKind> {
        Ok(PathKind::Directory)
    }

    fn files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        self.scanned
            .set(true);
        let error = io::Error::other("unexpected tree scan");
        Err(error)
    }
}

struct MalformedStore;

impl TextArtifactStore for MalformedStore {
    fn read_optional(
        &self,
        _path: &Path,
    ) -> io::Result<Option<String>> {
        let value = Some("not-a-manifest\n".to_owned());
        Ok(value)
    }

    fn write(
        &self,
        _path: &Path,
        _text: &str,
    ) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn malformed_manifest_fails_before_tree_scan() {
    let tree = ScanObservingTree::default();
    let result = ValidateManifest::execute(
        &tree,
        &MalformedStore,
        Path::new("game"),
    );

    assert!(result.is_err());
    let was_scanned = tree
        .scanned
        .get();
    assert!(!was_scanned);
}
