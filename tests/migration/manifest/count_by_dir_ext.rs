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
//   - Count by dir ext test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Count by dir ext test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Count by dir ext test module.

use std::fs;
use std::io::{self, ErrorKind};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use game_manifest::{
    DirExtCounts, EXPANDED_MANIFEST_FILE_NAME, MANIFEST_FILE_NAME,
    count_by_dir_ext, count_by_dir_ext_paths,
};
use schoenwald_cli as _;
use schoenwald_filesystem as _;

static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

struct FixtureRoot(PathBuf);

impl FixtureRoot {
    fn new(label: &str) -> io::Result<Self> {
        let sequence = NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "game-manifest-{label}-{}-{sequence}",
            std::process::id()
        ));
        match fs::remove_dir_all(&path) {
            Ok(()) => {},
            Err(error) if error.kind() == ErrorKind::NotFound => {},
            Err(error) => return Err(error),
        }
        fs::create_dir_all(&path)?;
        Ok(Self(path))
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.0));
    }
}

fn nested_manifest_counts() -> io::Result<DirExtCounts> {
    let fixture = FixtureRoot::new("nested-manifests")?;
    fs::create_dir_all(fixture.path().join("manifest"))?;
    fs::write(fixture.path().join(MANIFEST_FILE_NAME), b"root")?;
    fs::write(fixture.path().join(EXPANDED_MANIFEST_FILE_NAME), b"root")?;
    let nested = fixture.path().join("area");
    fs::create_dir_all(nested.join("manifest"))?;
    fs::write(nested.join(MANIFEST_FILE_NAME), b"nested")?;
    fs::write(nested.join(EXPANDED_MANIFEST_FILE_NAME), b"nested")?;
    count_by_dir_ext(fixture.path())
}

#[test]
fn output_names_are_excluded_only_at_root() {
    let result = nested_manifest_counts();
    assert!(result.is_ok());
    let Some(counts) = result.ok() else {
        return;
    };

    assert_eq!(
        counts.get(&("aa/mt".to_owned(), "jsonl".to_owned())),
        Some(&2)
    );
    assert!(!counts.contains_key(&(String::new(), "jsonl".to_owned())));
}

fn nested_legacy_lmlm_counts() -> io::Result<DirExtCounts> {
    let fixture = FixtureRoot::new("nested-optional")?;
    fs::write(fixture.path().join("language.lmlm"), b"root")?;
    let nested = fixture.path().join("area");
    fs::create_dir_all(&nested)?;
    fs::write(nested.join("content.lmlm"), b"nested")?;
    count_by_dir_ext(fixture.path())
}

#[test]
fn legacy_lmlm_has_no_special_counting_rule() {
    let result = nested_legacy_lmlm_counts();
    assert!(result.is_ok());
    let Some(counts) = result.ok() else {
        return;
    };

    assert_eq!(counts.get(&("aa".to_owned(), "lmlm".to_owned())), Some(&1));
    assert_eq!(counts.get(&(String::new(), "lmlm".to_owned())), Some(&1));
}

fn nested_png_counts() -> io::Result<DirExtCounts> {
    let fixture = FixtureRoot::new("nested-png")?;
    fs::write(fixture.path().join("generated.png"), b"root")?;
    let nested = fixture.path().join("area");
    fs::create_dir_all(&nested)?;
    fs::write(nested.join("texture.png"), b"nested")?;
    count_by_dir_ext(fixture.path())
}

#[test]
fn generated_png_is_excluded_only_at_root() {
    let result = nested_png_counts();
    assert!(result.is_ok());
    let Some(counts) = result.ok() else {
        return;
    };
    assert_eq!(counts.get(&("aa".to_owned(), "png".to_owned())), Some(&1));
    assert!(!counts.contains_key(&(String::new(), "png".to_owned())));
}

#[test]
fn root_machine_runtime_files_are_not_source_evidence() {
    let root = Path::new("game");
    let counts = count_by_dir_ext_paths(
        root,
        &[
            root.join("binkw32.dll"),
            root.join("simpsons.ini"),
            root.join("nested/config.ini"),
        ],
    );

    assert!(!counts.contains_key(&(String::new(), "dll".to_owned())));
    assert!(!counts.contains_key(&(String::new(), "ini".to_owned())));
    assert_eq!(
        counts.get(&("nd".to_owned(), "ini".to_owned())),
        Some(&1)
    );
}

#[test]
fn root_runtime_saves_are_not_source_evidence() {
    let root = Path::new("game");
    let counts = count_by_dir_ext_paths(
        root,
        &[
            root.join("Save1"),
            root.join("Save12"),
            root.join("nested/Save1"),
        ],
    );

    assert!(!counts.contains_key(&(String::new(), "(none)".to_owned())));
    assert_eq!(
        counts.get(&("nd".to_owned(), "(none)".to_owned())),
        Some(&1)
    );
    assert_eq!(
        counts.get(&(String::new(), "(none)".to_owned())),
        None
    );
}

#[test]
fn duplicate_file_evidence_counts_once() {
    let root = Path::new("game");
    let file = root.join("art").join("model.p3d");
    let counts = count_by_dir_ext_paths(root, &[file.clone(), file]);

    assert_eq!(counts.get(&("at".to_owned(), "p3d".to_owned(),),), Some(&1),);
}

#[test]
fn outside_root_file_evidence_is_ignored() {
    let counts = count_by_dir_ext_paths(Path::new("game"), &[PathBuf::from(
        "other/model.p3d",
    )]);

    assert!(counts.is_empty());
}

#[test]
fn parent_traversal_file_evidence_is_ignored() {
    let counts = count_by_dir_ext_paths(Path::new("game"), &[PathBuf::from(
        "game/area/../model.p3d",
    )]);

    assert!(counts.is_empty());
}

#[test]
fn root_manifest_directory_case_aliases_are_excluded() {
    let root = Path::new("game");
    for path in ["MANIFEST/GAME.JSONL", "Manifest/fbx.jsonl"] {
        let counts = count_by_dir_ext_paths(root, &[root.join(path)]);

        assert!(counts.is_empty());
    }
}

#[test]
fn colliding_obfuscated_directories_receive_stable_ordinals() {
    let root = Path::new("game");
    let counts = count_by_dir_ext_paths(root, &[
        root.join("alpha/first.p3d"),
        root.join("agenda/second.p3d"),
    ]);

    assert_eq!(
        counts.get(&("aa~01".to_owned(), "p3d".to_owned(),),),
        Some(&1),
    );
    assert_eq!(
        counts.get(&("aa~02".to_owned(), "p3d".to_owned(),),),
        Some(&1),
    );
    assert_eq!(counts.len(), 2,);
}
