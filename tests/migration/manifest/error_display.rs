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
//   - Error display test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Error display test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Error display test module.

use std::error::Error as _;
use std::io;
use std::path::{Path, PathBuf};

use game_manifest::{
    GameTree, GenerateManifest, ManifestError, PathKind, TextArtifactStore,
};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

struct MissingTree;

impl GameTree for MissingTree {
    fn kind(&self, _path: &Path) -> io::Result<PathKind> {
        Ok(PathKind::Missing)
    }

    fn files(&self, _root: &Path) -> io::Result<Vec<PathBuf>> {
        let error = io::Error::other("unexpected scan");
        Err(error)
    }
}

struct UnusedStore;

impl TextArtifactStore for UnusedStore {
    fn read_optional(&self, _path: &Path) -> io::Result<Option<String>> {
        let error = io::Error::other("unexpected read");
        Err(error)
    }

    fn write(&self, _path: &Path, _text: &str) -> io::Result<()> {
        let error = io::Error::other("unexpected write");
        Err(error)
    }
}

#[test]
fn invalid_path_errors_escape_control_characters() {
    let path = PathBuf::from("\u{1b}[2Jgame");
    let result = GenerateManifest::execute(&MissingTree, &UnusedStore, &path);
    assert!(result.is_err());
    let Some(error) = result.err() else {
        return;
    };
    let rendered = error.to_string();

    assert!(!rendered.contains('\u{1b}'));
    assert!(rendered.contains("\\u{1b}"));
    assert!(rendered.contains("game"));
}

#[test]
fn io_errors_escape_source_control_characters() {
    let error = ManifestError::io(
        "inspect",
        PathBuf::from("game"),
        io::Error::other("blocked\ninjected"),
    );

    let rendered = error.to_string();

    assert!(
        !rendered.chars().any(char::is_control),
        "diagnostic contains a control character: {rendered:?}"
    );
    assert!(rendered.contains(r"blocked\ninjected"));
    assert!(error.source().is_some());
}

#[test]
fn path_errors_escape_control_characters() {
    let path = PathBuf::from("\u{1b}[2Jgame");
    let error = ManifestError::io("inspect", path, io::Error::other("blocked"));
    let rendered = error.to_string();

    assert!(!rendered.contains('\u{1b}'));
    assert!(rendered.contains("\\u{1b}"));
    assert!(rendered.contains("game"));
    assert!(rendered.contains("blocked"));
}
