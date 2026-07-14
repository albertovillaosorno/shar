// File:
//   - error_display.rs
// Path:
//   - src/game-manifest/tests/error_display.rs
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
//   - Game-manifest application error rendering regressions.
// - Must-Not:
//   - Access local files or emit diagnostics directly to process streams.
// - Allows:
//   - Synthetic paths and provider errors rendered in memory.
// - Split-When:
//   - Split when another error family needs independent fixtures.
// - Merge-When:
//   - Another test owns the same error-display boundary.
// - Summary:
//   - Protects diagnostics from terminal control injection.
// - Description:
//   - Verifies path context remains visible without raw control characters.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Synthetic errors remain deterministic and side-effect free.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Game-manifest error-display regression coverage.
//!
//! Path diagnostics must preserve context without emitting terminal controls.

use std::io;
use std::path::{Path, PathBuf};

use game_manifest::{
    GameTree, GenerateManifest, ManifestError, PathKind, TextArtifactStore,
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
        let error = io::Error::other("unexpected scan");
        Err(error)
    }
}

struct UnusedStore;

impl TextArtifactStore for UnusedStore {
    fn read_optional(
        &self,
        _path: &Path,
    ) -> io::Result<Option<String>> {
        let error = io::Error::other("unexpected read");
        Err(error)
    }

    fn write(
        &self,
        _path: &Path,
        _text: &str,
    ) -> io::Result<()> {
        let error = io::Error::other("unexpected write");
        Err(error)
    }
}

#[test]
fn invalid_path_errors_escape_control_characters() {
    // cspell:disable-next-line -- Jgame
    let path = PathBuf::from("\u{1b}[2Jgame");
    let result = GenerateManifest::execute(
        &MissingTree,
        &UnusedStore,
        &path,
    );
    assert!(result.is_err());
    let Some(error) = result.err() else {
        return;
    };
    let rendered = error.to_string();

    assert!(!rendered.contains('\u{1b}'));
    assert!(rendered.contains("\\x1b"));
    assert!(rendered.contains("game"));
}

#[test]
fn path_errors_escape_control_characters() {
    // cspell:disable-next-line -- Jgame
    let path = PathBuf::from("\u{1b}[2Jgame");
    let error = ManifestError::io(
        "inspect",
        path,
        io::Error::other("blocked"),
    );
    let rendered = error.to_string();

    assert!(!rendered.contains('\u{1b}'));
    assert!(rendered.contains("\\x1b"));
    assert!(rendered.contains("game"));
    assert!(rendered.contains("blocked"));
}
