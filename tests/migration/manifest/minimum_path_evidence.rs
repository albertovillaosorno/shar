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
//   - Minimum path evidence test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Minimum path evidence test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Minimum path evidence test module.

use std::cell::RefCell;
use std::io;
use std::path::{Path, PathBuf};

use game_manifest::{GameTree, GenerateManifest, PathKind, TextArtifactStore};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

struct OutsideTree;

struct AmbiguousTree;

impl GameTree for AmbiguousTree {
    fn kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::Directory)
    }

    fn files(&self, root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![
            root.join("Simpsons.exe"),
            root.join("Simpsons.ico"),
            root.join("README.rtf"),
            root.join("dialog.rcf"),
            root.join("art/frontend/scrooby2/resource/txtbible/srr2.E"),
            root.join("art/frontend/scrooby2/resource/txtbible/srr2.txt"),
            root.join("alpha/first.p3d"),
            root.join("agenda/second.p3d"),
        ])
    }
}

impl GameTree for OutsideTree {
    fn kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::Directory)
    }

    fn files(&self, root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(vec![
            root.join("asset.p3d"),
            PathBuf::from("outside/hidden.p3d"),
        ])
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
fn generation_rejects_outside_root_evidence() {
    let store = MemoryStore::default();
    let result =
        GenerateManifest::execute(&OutsideTree, &store, Path::new("game"));

    assert!(result.is_err());
    assert!(store.written.borrow().is_none());
}

#[test]
fn generation_disambiguates_colliding_obfuscated_coordinates() {
    let store = MemoryStore::default();
    let result =
        GenerateManifest::execute(&AmbiguousTree, &store, Path::new("game"));

    assert!(result.is_ok());
    let written = store.written.borrow();
    let Some(manifest) = written.as_ref() else {
        return;
    };
    assert!(manifest.contains("\"dir\":\"aa~01\""));
    assert!(manifest.contains("\"dir\":\"aa~02\""));
    assert!(!manifest.contains("\"dir\":\"aa\""));
}
