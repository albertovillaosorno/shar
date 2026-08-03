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
//   - Validation path evidence test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Validation path evidence test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Validation path evidence test module.

use std::io;
use std::path::{Path, PathBuf};

use game_manifest::{
    DirCount, GameTree, PathKind, TextArtifactStore, ValidateManifest,
    kind_taxonomy_jsonl,
};
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

struct StaticStore;

impl TextArtifactStore for StaticStore {
    fn read_optional(&self, _path: &Path) -> io::Result<Option<String>> {
        let text = manifest();
        let value = Some(text);
        Ok(value)
    }

    fn write(&self, _path: &Path, _text: &str) -> io::Result<()> {
        Ok(())
    }
}

fn manifest() -> String {
    let rows = [
        DirCount {
            dir: String::new(),
            extension: "lmlm".to_owned(),
            min_count: 0,
            kind: "language_mod".to_owned(),
        },
        DirCount {
            dir: String::new(),
            extension: "png".to_owned(),
            min_count: 0,
            kind: "generated_artifact".to_owned(),
        },
        DirCount {
            dir: "aa~01".to_owned(),
            extension: "p3d".to_owned(),
            min_count: 1,
            kind: "p3d_container".to_owned(),
        },
        DirCount {
            dir: "aa~02".to_owned(),
            extension: "p3d".to_owned(),
            min_count: 1,
            kind: "p3d_container".to_owned(),
        },
    ];
    let body = rows
        .iter()
        .map(DirCount::to_jsonl)
        .collect::<Vec<_>>()
        .join("\n");
    format!("{}\n{body}\n", kind_taxonomy_jsonl())
}

#[test]
fn validation_rejects_outside_root_evidence() {
    let store = StaticStore;
    let result =
        ValidateManifest::execute(&OutsideTree, &store, Path::new("game"));

    assert!(result.is_err());
}

#[test]
fn validation_accepts_disambiguated_obfuscated_coordinates() {
    let store = StaticStore;
    let result =
        ValidateManifest::execute(&AmbiguousTree, &store, Path::new("game"));

    assert!(
        result
            .as_ref()
            .is_ok_and(|report| { report.shortfalls.is_empty() },)
    );
}
