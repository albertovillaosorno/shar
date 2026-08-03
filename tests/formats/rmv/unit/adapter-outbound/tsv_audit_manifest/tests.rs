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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};

use super::write_manifest;
use crate::domain::{AuditReport, MovieKind};

static TEMP_ID: AtomicU64 = AtomicU64::new(0);

fn temp_root() -> PathBuf {
    let id = TEMP_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir()
        .join(format!("rmv-manifest-header-{}-{id}", std::process::id()))
}

#[cfg(windows)]
#[test]
fn preserves_non_unicode_windows_manifest_paths() {
    use std::ffi::OsString;
    use std::os::windows::ffi::OsStringExt as _;

    let root = temp_root();
    let source_path = PathBuf::from(OsString::from_wide(&[0xd800]));
    let report = AuditReport {
        records: vec![crate::domain::MovieRecord {
            source_root: PathBuf::from("input"),
            source_path,
            relative_path: PathBuf::from("movie.rmv"),
            output_path: PathBuf::from("output/movie.bk2"),
            bytes: 4,
            kind: MovieKind::BinkV1,
            hash: crate::domain::Sha256::digest(b"BIKi"),
            provenance: crate::domain::ProvenanceEvidence {
                embedded_source_names: Vec::new(),
            },
        }],
        missing_bk2_outputs: 1,
        duplicate_inputs: 0,
    };
    let result = write_manifest(&root, &report);
    assert!(result.is_ok(), "manifest should be writable");
    let manifest_result = fs::read_to_string(root.join("manifest.tsv"));
    let _cleanup_result = fs::remove_dir_all(&root);
    assert!(manifest_result.is_ok(), "manifest should be readable");
    let Ok(manifest) = manifest_result else {
        return;
    };
    assert!(manifest.contains("\\uD800"));
    assert!(!manifest.contains('\u{fffd}'));
}

#[test]
fn escapes_control_characters_in_manifest_fields() {
    let root = temp_root();
    let report = AuditReport {
        records: vec![crate::domain::MovieRecord {
            source_root: PathBuf::from("input"),
            source_path: PathBuf::from("input/movie\tname\nsource.rmv"),
            relative_path: PathBuf::from("movie.rmv"),
            output_path: PathBuf::from("output/movie\rname.bk2"),
            bytes: 4,
            kind: MovieKind::BinkV1,
            hash: crate::domain::Sha256::digest(b"BIKi"),
            provenance: crate::domain::ProvenanceEvidence {
                embedded_source_names: vec![
                    "source\tname\u{0000}\u{000b}\u{001f}.mov".to_owned(),
                ],
            },
        }],
        missing_bk2_outputs: 1,
        duplicate_inputs: 0,
    };
    let result = write_manifest(&root, &report);
    assert!(result.is_ok(), "manifest should be writable");
    let manifest_result = fs::read_to_string(root.join("manifest.tsv"));
    let _cleanup_result = fs::remove_dir_all(&root);
    assert!(manifest_result.is_ok(), "manifest should be readable");
    let Ok(manifest) = manifest_result else {
        return;
    };
    let lines = manifest.lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 2);
    let Some(row) = lines.get(1) else {
        return;
    };
    assert_eq!(row.split('\t').count(), 6);
    assert!(row.contains(r"movie\tname\nsource.rmv"));
    assert!(row.contains(r"source\tname\u0000\u000B\u001F.mov"));
    assert!(
        !row.chars()
            .any(|character| character.is_control() && character != '\t')
    );
    assert!(row.contains(r"movie\rname.bk2"));
}

#[test]
fn writes_records_in_deterministic_identity_order() {
    let left = temp_root();
    let right = temp_root();
    let mut report = AuditReport {
        records: vec![
            crate::domain::MovieRecord {
                source_root: PathBuf::from("input"),
                source_path: PathBuf::from("input/b.rmv"),
                relative_path: PathBuf::from("b.rmv"),
                output_path: PathBuf::from("output/b.bk2"),
                bytes: 4,
                kind: MovieKind::BinkV1,
                hash: crate::domain::Sha256::digest(b"b"),
                provenance: crate::domain::ProvenanceEvidence {
                    embedded_source_names: Vec::new(),
                },
            },
            crate::domain::MovieRecord {
                source_root: PathBuf::from("input"),
                source_path: PathBuf::from("input/a.rmv"),
                relative_path: PathBuf::from("a.rmv"),
                output_path: PathBuf::from("output/a.bk2"),
                bytes: 4,
                kind: MovieKind::BinkV1,
                hash: crate::domain::Sha256::digest(b"a"),
                provenance: crate::domain::ProvenanceEvidence {
                    embedded_source_names: Vec::new(),
                },
            },
        ],
        missing_bk2_outputs: 2,
        duplicate_inputs: 0,
    };
    let result = write_manifest(&left, &report);
    assert!(result.is_ok(), "first manifest should be writable");
    report.records.reverse();
    let second_result = write_manifest(&right, &report);
    assert!(second_result.is_ok(), "second manifest should be writable");
    let left_result = fs::read(left.join("manifest.tsv"));
    let right_result = fs::read(right.join("manifest.tsv"));
    let _cleanup_left = fs::remove_dir_all(&left);
    let _cleanup_right = fs::remove_dir_all(&right);
    assert!(left_result.is_ok(), "first manifest should be readable");
    assert!(right_result.is_ok(), "second manifest should be readable");
    let Ok(left_manifest) = left_result else {
        return;
    };
    let Ok(right_manifest) = right_result else {
        return;
    };
    assert_eq!(left_manifest, right_manifest);
}

#[test]
fn writes_tab_separated_manifest_header() {
    let root = temp_root();
    let result = write_manifest(&root, &AuditReport::default());
    assert!(result.is_ok(), "empty audit manifest should be writable");
    let manifest_result = fs::read_to_string(root.join("manifest.tsv"));
    let _cleanup_result = fs::remove_dir_all(&root);
    assert!(
        manifest_result.is_ok(),
        "written manifest should be readable"
    );
    let Ok(manifest) = manifest_result else {
        return;
    };
    let first_line = manifest.lines().next().unwrap_or_default();
    assert_eq!(first_line.split('\t').collect::<Vec<_>>(), vec![
        "sha256",
        "bytes",
        "kind",
        "current_source",
        "pre_bink_master_evidence",
        "expected_bk2",
    ]);
}
