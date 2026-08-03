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
//   - Expanded duplicate evidence test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Expanded duplicate evidence test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Expanded duplicate evidence test module.

use std::cell::RefCell;
use std::io;
use std::path::{Path, PathBuf};

use game_manifest::{
    GameTree, GenerateExpandedManifest, PathKind, TextArtifactStore,
};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

struct DuplicateTree;
struct TraversingTree;

impl GameTree for DuplicateTree {
    fn kind(&self, path: &Path) -> io::Result<PathKind> {
        Ok(if path == Path::new("game") {
            PathKind::Directory
        } else {
            PathKind::Missing
        })
    }

    fn files(&self, root: &Path) -> io::Result<Vec<PathBuf>> {
        let path = root.join("art/model.p3d");
        Ok(vec![path.clone(), path])
    }
}

impl GameTree for TraversingTree {
    fn kind(&self, path: &Path) -> io::Result<PathKind> {
        Ok(if path == Path::new("game") {
            PathKind::Directory
        } else {
            PathKind::Missing
        })
    }

    fn files(&self, root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![root.join("area/../model.p3d")])
    }
}

#[derive(Default)]
struct MemoryStore {
    written: RefCell<Option<String>>,
}

impl TextArtifactStore for MemoryStore {
    fn read_optional(&self, _path: &Path) -> io::Result<Option<String>> {
        Ok(None)
    }

    fn write(&self, _path: &Path, text: &str) -> io::Result<()> {
        let _previous = self.written.replace(Some(text.to_owned()));
        Ok(())
    }
}

#[test]
fn expanded_duplicate_file_evidence_emits_one_record() {
    let store = MemoryStore::default();
    let report = GenerateExpandedManifest::execute(
        &DuplicateTree,
        &store,
        Path::new("game"),
        Path::new("extracted/rcf"),
        Path::new("output/expanded.jsonl"),
    );

    assert_eq!(report.ok().map(|value| value.record_count), Some(1));
}

#[test]
fn expanded_parent_traversal_evidence_is_rejected() {
    let store = MemoryStore::default();
    let report = GenerateExpandedManifest::execute(
        &TraversingTree,
        &store,
        Path::new("game"),
        Path::new("extracted/rcf"),
        Path::new("output/expanded.jsonl"),
    );

    assert!(report.is_err());
}
