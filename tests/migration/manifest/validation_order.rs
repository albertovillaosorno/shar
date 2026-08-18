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
//   - Validation order test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Validation order test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Validation order test module.

use std::cell::Cell;
#[cfg(windows)]
use std::ffi::OsString;
use std::io;
#[cfg(windows)]
use std::os::windows::ffi::OsStringExt as _;
use std::path::{Path, PathBuf};

use game_manifest::{GameTree, PathKind, TextArtifactStore, ValidateManifest};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[derive(Default)]
struct ScanObservingTree {
    scanned: Cell<bool>,
}

impl GameTree for ScanObservingTree {
    fn kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::Directory)
    }

    fn files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        self.scanned.set(true);
        let error = io::Error::other("unexpected tree scan");
        Err(error)
    }
}

struct MalformedStore;

#[cfg(windows)]
struct MissingTree;

#[cfg(windows)]
impl GameTree for MissingTree {
    fn kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::Missing)
    }

    fn files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        Ok(Vec::new())
    }
}

impl TextArtifactStore for MalformedStore {
    fn read_optional(&self, _path: &Path) -> io::Result<Option<String>> {
        let value = Some("not-a-manifest\n".to_owned());
        Ok(value)
    }

    fn write(&self, _path: &Path, _text: &str) -> io::Result<()> {
        Ok(())
    }
}

#[test]
fn malformed_manifest_fails_before_tree_scan() {
    let tree = ScanObservingTree::default();
    let result =
        ValidateManifest::execute(&tree, &MalformedStore, Path::new("game"));

    assert!(result.is_err());
    let was_scanned = tree.scanned.get();
    assert!(!was_scanned);
}

#[cfg(windows)]
#[test]
fn missing_game_error_preserves_unpaired_utf16_path_unit() {
    let game_dir = PathBuf::from(OsString::from_wide(&[
        u16::from(b'a'),
        0xd800,
        u16::from(b'b'),
    ]));

    let result =
        ValidateManifest::execute(&MissingTree, &MalformedStore, &game_dir);
    assert!(
        result.is_err(),
        "missing game directory unexpectedly validated"
    );
    let Err(error) = result else {
        return;
    };
    let rendered = error.to_string();

    assert!(
        rendered.contains(r"a\u{D800}b"),
        "diagnostic lost the native path unit: {rendered:?}"
    );
    assert!(!rendered.contains(r"\u{fffd}"));
}
