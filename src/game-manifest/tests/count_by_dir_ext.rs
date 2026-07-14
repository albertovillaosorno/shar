// File:
//   - count_by_dir_ext.rs
// Path:
//   - src/game-manifest/tests/count_by_dir_ext.rs
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
//   - Filesystem traversal regressions for game-manifest directory counts.
// - Must-Not:
//   - Read licensed inputs or mutable repository output trees.
// - Allows:
//   - Process-local temporary directories and deterministic fixture files.
// - Split-When:
//   - Split when platform-specific filesystem semantics require isolation.
// - Merge-When:
//   - Another game-manifest test owns the same traversal boundary.
// - Summary:
//   - Protects deterministic directory and extension counting.
// - Description:
//   - Builds synthetic trees to verify exclusions and bucket coordinates.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Every fixture is removed when its test guard is dropped.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Filesystem traversal regression coverage.
//!
//! These tests create isolated synthetic trees and never inspect repository
//! content, source containers, or generated outputs.

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
        let sequence = NEXT_FIXTURE.fetch_add(
            1,
            Ordering::Relaxed,
        );
        let path = std::env::temp_dir().join(
            format!(
                "game-manifest-{label}-{}-{sequence}",
                std::process::id()
            ),
        );
        match fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => {}
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
    fs::write(
        fixture
            .path()
            .join(MANIFEST_FILE_NAME),
        b"root",
    )?;
    fs::write(
        fixture
            .path()
            .join(EXPANDED_MANIFEST_FILE_NAME),
        b"root",
    )?;
    let nested = fixture
        .path()
        .join("area");
    fs::create_dir_all(&nested)?;
    fs::write(
        nested.join(MANIFEST_FILE_NAME),
        b"nested",
    )?;
    fs::write(
        nested.join(EXPANDED_MANIFEST_FILE_NAME),
        b"nested",
    )?;
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
        counts.get(
            &(
                "aa".to_owned(),
                "jsonl".to_owned()
            )
        ),
        Some(&2)
    );
    assert!(
        !counts.contains_key(
            &(
                String::new(),
                "jsonl".to_owned()
            )
        )
    );
}

fn nested_optional_mod_counts() -> io::Result<DirExtCounts> {
    let fixture = FixtureRoot::new("nested-optional")?;
    fs::write(
        fixture
            .path()
            .join("language.lmlm"),
        b"root",
    )?;
    let nested = fixture
        .path()
        .join("area");
    fs::create_dir_all(&nested)?;
    fs::write(
        nested.join("content.lmlm"),
        b"nested",
    )?;
    count_by_dir_ext(fixture.path())
}

#[test]
fn optional_mods_are_excluded_only_at_root() {
    let result = nested_optional_mod_counts();
    assert!(result.is_ok());
    let Some(counts) = result.ok() else {
        return;
    };

    assert_eq!(
        counts.get(
            &(
                "aa".to_owned(),
                "lmlm".to_owned()
            )
        ),
        Some(&1)
    );
    assert!(
        !counts.contains_key(
            &(
                String::new(),
                "lmlm".to_owned()
            )
        )
    );
}

fn nested_png_counts() -> io::Result<DirExtCounts> {
    let fixture = FixtureRoot::new("nested-png")?;
    fs::write(
        fixture
            .path()
            .join("generated.png"),
        b"root",
    )?;
    let nested = fixture
        .path()
        .join("area");
    fs::create_dir_all(&nested)?;
    fs::write(
        nested.join("texture.png"),
        b"nested",
    )?;
    count_by_dir_ext(fixture.path())
}

#[test]
fn generated_png_is_excluded_only_at_root() {
    let result = nested_png_counts();
    assert!(result.is_ok());
    let Some(counts) = result.ok() else {
        return;
    };
    assert_eq!(
        counts.get(
            &(
                "aa".to_owned(),
                "png".to_owned()
            )
        ),
        Some(&1)
    );
    assert!(
        !counts.contains_key(
            &(
                String::new(),
                "png".to_owned()
            )
        )
    );
}

#[test]
fn duplicate_file_evidence_counts_once() {
    let root = Path::new("game");
    let file = root
        .join("art")
        .join("model.p3d");
    let counts = count_by_dir_ext_paths(
        root,
        &[
            file.clone(),
            file,
        ],
    );

    assert_eq!(
        counts.get(
            &(
                "at".to_owned(),
                "p3d".to_owned(),
            ),
        ),
        Some(&1),
    );
}

#[test]
fn outside_root_file_evidence_is_ignored() {
    let counts = count_by_dir_ext_paths(
        Path::new("game"),
        &[PathBuf::from("other/model.p3d")],
    );

    assert!(counts.is_empty());
}

#[test]
fn parent_traversal_file_evidence_is_ignored() {
    let counts = count_by_dir_ext_paths(
        Path::new("game"),
        &[PathBuf::from("game/area/../model.p3d")],
    );

    assert!(counts.is_empty());
}

#[test]
fn root_manifest_case_aliases_are_excluded() {
    let root = Path::new("game");
    for file_name in [
        "MANIFEST.JSONL",
        "MANIFEST-EXPANDED.JSONL",
    ] {
        let counts = count_by_dir_ext_paths(
            root,
            &[root.join(file_name)],
        );

        assert!(counts.is_empty());
    }
}

#[test]
fn colliding_obfuscated_directories_receive_stable_ordinals() {
    let root = Path::new("game");
    let counts = count_by_dir_ext_paths(
        root,
        &[
            root.join("alpha/first.p3d"),
            root.join("agenda/second.p3d"),
        ],
    );

    assert_eq!(
        counts.get(
            &(
                "aa~01".to_owned(),
                "p3d".to_owned(),
            ),
        ),
        Some(&1),
    );
    assert_eq!(
        counts.get(
            &(
                "aa~02".to_owned(),
                "p3d".to_owned(),
            ),
        ),
        Some(&1),
    );
    assert_eq!(
        counts.len(),
        2,
    );
}
