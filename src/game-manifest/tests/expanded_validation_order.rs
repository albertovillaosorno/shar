// File:
//   - expanded_validation_order.rs
// Path:
//   - src/game-manifest/tests/expanded_validation_order.rs
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
//   - Expanded-manifest validation and storage-access ordering regressions.
// - Must-Not:
//   - Access local game data or repository output directories.
// - Allows:
//   - In-memory ports that expose tree and storage access ordering.
// - Split-When:
//   - Split when another validation stage requires an independent fixture.
// - Merge-When:
//   - Another test owns the same expanded validation-order boundary.
// - Summary:
//   - Protects invalid expanded requests from premature output access.
// - Description:
//   - Verifies source roots fail before existing output is inspected.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Synthetic paths remain portable and deterministic.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Expanded-manifest validation-order regression coverage.
//!
//! Invalid source roots must fail before destination storage is consulted.

use std::cell::Cell;
use std::io;
use std::path::{Path, PathBuf};

use game_manifest::{
    GameTree, GenerateExpandedManifest, PathKind, TextArtifactStore,
};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

struct MissingTree;

impl GameTree for MissingTree {
    fn kind(
        &self,
        _path: &Path,
    ) -> io::Result<PathKind> {
        Ok(PathKind::Missing)
    }

    fn files(
        &self,
        _root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        let error = io::Error::other("unexpected tree scan");
        Err(error)
    }
}

#[derive(Default)]
struct ReadObservingStore {
    read: Cell<bool>,
}

impl TextArtifactStore for ReadObservingStore {
    fn read_optional(
        &self,
        _path: &Path,
    ) -> io::Result<Option<String>> {
        self.read
            .set(true);
        let error = io::Error::other("unexpected output read");
        Err(error)
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
fn missing_game_root_fails_before_output_read() {
    let store = ReadObservingStore::default();
    let result = GenerateExpandedManifest::execute(
        &MissingTree,
        &store,
        Path::new("missing-game"),
        Path::new("missing-extracted"),
        Path::new("output/result.jsonl"),
    );

    assert!(result.is_err());
    let was_read = store
        .read
        .get();
    assert!(!was_read);
}
