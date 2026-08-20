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
//   - Expanded validation order test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Expanded validation order test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Expanded validation order test module.

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
    fn kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::Missing)
    }

    fn files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        let error = io::Error::other("unexpected tree scan");
        Err(error)
    }
}

#[derive(Default)]
struct ReadObservingStore {
    read: Cell<bool>,
}

impl TextArtifactStore for ReadObservingStore {
    fn read_optional(&self, _path: &Path) -> io::Result<Option<String>> {
        self.read.set(true);
        let error = io::Error::other("unexpected output read");
        Err(error)
    }

    fn write(&self, _path: &Path, _text: &str) -> io::Result<()> {
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
    let was_read = store.read.get();
    assert!(!was_read);
}
