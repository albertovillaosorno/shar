// File:
//   - minimum_path_evidence.rs
// Path:
//   - src/game-manifest/tests/minimum_path_evidence.rs
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
//   - Minimum-manifest application path-evidence regressions.
// - Must-Not:
//   - Access local game data or repository output directories.
// - Allows:
//   - In-memory ports with synthetic safe and unsafe path snapshots.
// - Split-When:
//   - Split when another minimum-manifest evidence invariant needs isolation.
// - Merge-When:
//   - Another test owns the same minimum path-snapshot boundary.
// - Summary:
//   - Protects minimum generation from unsafe adapter evidence.
// - Description:
//   - Verifies invalid snapshots fail before manifest publication.
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

//! Minimum-manifest path-evidence regression coverage.
//!
//! Invalid adapter snapshots must fail closed before publication.

use std::cell::RefCell;
use std::io;
use std::path::{Path, PathBuf};

use game_manifest::{GameTree, GenerateManifest, PathKind, TextArtifactStore};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

struct OutsideTree;

struct AmbiguousTree;

impl GameTree for AmbiguousTree {
    fn kind(
        &self,
        _path: &Path,
    ) -> io::Result<PathKind> {
        Ok(PathKind::Directory)
    }

    fn files(
        &self,
        root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(
            vec![
                root.join("alpha/first.p3d"),
                root.join("agenda/second.p3d"),
            ],
        )
    }
}

impl GameTree for OutsideTree {
    fn kind(
        &self,
        _path: &Path,
    ) -> io::Result<PathKind> {
        Ok(PathKind::Directory)
    }

    fn files(
        &self,
        root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        Ok(
            vec![
                root.join("asset.p3d"),
                PathBuf::from("outside/hidden.p3d"),
            ],
        )
    }
}

#[derive(Default)]
struct MemoryStore {
    written: RefCell<Option<String>>,
}

impl TextArtifactStore for MemoryStore {
    fn read_optional(
        &self,
        _path: &Path,
    ) -> io::Result<Option<String>> {
        Ok(None)
    }

    fn write(
        &self,
        _path: &Path,
        text: &str,
    ) -> io::Result<()> {
        let _previous = self
            .written
            .replace(Some(text.to_owned()));
        Ok(())
    }
}

#[test]
fn generation_rejects_outside_root_evidence() {
    let store = MemoryStore::default();
    let result = GenerateManifest::execute(
        &OutsideTree,
        &store,
        Path::new("game"),
    );

    assert!(result.is_err());
    assert!(
        store
            .written
            .borrow()
            .is_none()
    );
}

#[test]
fn generation_disambiguates_colliding_obfuscated_coordinates() {
    let store = MemoryStore::default();
    let result = GenerateManifest::execute(
        &AmbiguousTree,
        &store,
        Path::new("game"),
    );

    assert!(result.is_ok());
    let written = store
        .written
        .borrow();
    let Some(manifest) = written.as_ref() else {
        return;
    };
    assert!(manifest.contains("\"dir\":\"aa~01\""));
    assert!(manifest.contains("\"dir\":\"aa~02\""));
    assert!(!manifest.contains("\"dir\":\"aa\""));
}
