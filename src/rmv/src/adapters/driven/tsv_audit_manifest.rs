// File:
//   - tsv_audit_manifest.rs
// Path:
//   - src/rmv/src/adapters/driven/tsv_audit_manifest.rs
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
//   - TSV serialization and filesystem publication of completed audit reports.
// - Must-Not:
//   - Discover movies, change audit semantics, or parse CLI requests.
// - Allows:
//   - Escape report fields and publish one deterministic manifest artifact.
// - Split-When:
//   - Split when serialization and storage require independent providers.
// - Merge-When:
//   - Another driven adapter owns the same TSV manifest contract.
// - Summary:
//   - Driven TSV manifest adapter for RMV audits.
// - Description:
//   - Implements audit report publication behind the manifest sink port.
// - Usage:
//   - Selected by driving adapters that require the canonical TSV artifact.
// - Defaults:
//   - The manifest is named `manifest.tsv` under the supplied output root.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: The adapter keeps one serializer and its focused escaping tests
//   - together because they protect a single publication contract.
//

//! Driven adapter for deterministic RMV audit TSV publication.
use std::fmt::Write as _;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::domain::{AuditReport, MovieKind, RmvError};
use crate::ports::AuditManifestSink;

/// Filesystem-backed canonical TSV audit manifest sink.
#[derive(Debug, Default, Clone, Copy)]
pub struct TsvAuditManifestSink;

impl AuditManifestSink for TsvAuditManifestSink {
    fn write_manifest(
        &self,
        output_root: &Path,
        report: &AuditReport,
    ) -> Result<(), RmvError> {
        write_manifest(
            output_root,
            report,
        )
        .map_err(
            |source| RmvError::Io {
                path: output_root.join("manifest.tsv"),
                source,
            },
        )
    }
}

/// Write audit manifest.
fn write_manifest(
    output_root: &Path,
    report: &AuditReport,
) -> Result<(), std::io::Error> {
    let header_columns = [
        "sha256",
        "bytes",
        "kind",
        "current_source",
        "pre_bink_master_evidence",
        "expected_bk2",
    ];
    let mut manifest = header_columns.join("\t");
    manifest.push('\n');
    let mut rows = Vec::with_capacity(
        report
            .records
            .len(),
    );
    for record in &report.records {
        let kind = match record.kind {
            MovieKind::BinkV1 => "bink-v1",
            MovieKind::BinkV2 => "bink-v2",
            MovieKind::OggNamedRmv => "ogg-named-rmv",
            MovieKind::XboxXmvLike => "xbox-xmv-like",
            MovieKind::RadicalMovieHeader => "radical-movie-header",
            MovieKind::Unknown => "unknown",
        };
        let source_path = escape_manifest_path(&record.source_path);
        let provenance = escape_manifest_field(
            &record
                .provenance
                .summary(),
        );
        let output_path = escape_manifest_path(&record.output_path);
        let mut row = String::new();
        let _write_result = writeln!(
            row,
            "{}	{}	{}	{}	{}	{}",
            record
                .hash
                .hex(),
            record.bytes,
            kind,
            source_path,
            provenance,
            output_path,
        );
        rows.push(
            (
                source_path,
                output_path,
                row,
            ),
        );
    }
    rows.sort();
    for row in rows {
        manifest.push_str(&row.2);
    }
    local::write_text(
        &output_root.join("manifest.tsv"),
        &manifest,
        true,
    )
}

/// Escapes a filesystem path without losing Windows UTF-16 code units.
#[cfg(windows)]
fn escape_manifest_path(path: &Path) -> String {
    use std::os::windows::ffi::OsStrExt as _;

    let mut escaped = String::new();
    for decoded in char::decode_utf16(
        path.as_os_str()
            .encode_wide(),
    ) {
        match decoded {
            Ok(character) => append_manifest_character(
                &mut escaped,
                character,
            ),
            Err(error) => {
                escaped.push_str(r"\u");
                let _write_result = write!(
                    escaped,
                    "{:04X}",
                    error.unpaired_surrogate(),
                );
            }
        }
    }
    escaped
}

/// Escapes a path through Unicode text on platforms with byte-safe callers.
#[cfg(not(windows))]
fn escape_manifest_path(path: &Path) -> String {
    escape_manifest_field(&path.to_string_lossy())
}

/// Escapes control characters that would otherwise corrupt TSV structure.
fn escape_manifest_field(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        append_manifest_character(
            &mut escaped,
            character,
        );
    }
    escaped
}

/// Appends one manifest character with TSV-safe escaping.
fn append_manifest_character(
    escaped: &mut String,
    character: char,
) {
    match character {
        '\\' => escaped.push_str("\\\\"),
        '\t' => escaped.push_str("\\t"),
        '\n' => escaped.push_str("\\n"),
        '\r' => escaped.push_str("\\r"),
        control if control.is_control() => {
            escaped.push('\\');
            let _write_result = write!(
                escaped,
                "u{:04X}",
                u32::from(control),
            );
        }
        other => escaped.push(other),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::write_manifest;
    use crate::domain::{AuditReport, MovieKind};

    static TEMP_ID: AtomicU64 = AtomicU64::new(0);

    fn temp_root() -> PathBuf {
        let id = TEMP_ID.fetch_add(
            1,
            Ordering::Relaxed,
        );
        std::env::temp_dir().join(
            format!(
                "rmv-manifest-header-{}-{id}",
                std::process::id()
            ),
        )
    }

    #[cfg(windows)]
    #[test]
    fn preserves_non_unicode_windows_manifest_paths() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt as _;

        let root = temp_root();
        let source_path = PathBuf::from(OsString::from_wide(&[0xd800]));
        let report = AuditReport {
            records: vec![
                crate::domain::MovieRecord {
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
                },
            ],
            missing_bk2_outputs: 1,
            duplicate_inputs: 0,
        };
        let result = write_manifest(
            &root, &report,
        );
        assert!(
            result.is_ok(),
            "manifest should be writable"
        );
        let manifest_result = fs::read_to_string(root.join("manifest.tsv"));
        let _cleanup_result = fs::remove_dir_all(&root);
        assert!(
            manifest_result.is_ok(),
            "manifest should be readable"
        );
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
            records: vec![
                crate::domain::MovieRecord {
                    source_root: PathBuf::from("input"),
                    source_path: PathBuf::from("input/movie\tname\nsource.rmv"),
                    relative_path: PathBuf::from("movie.rmv"),
                    output_path: PathBuf::from("output/movie\rname.bk2"),
                    bytes: 4,
                    kind: MovieKind::BinkV1,
                    hash: crate::domain::Sha256::digest(b"BIKi"),
                    provenance: crate::domain::ProvenanceEvidence {
                        embedded_source_names: vec![
                            "source\tname\u{0000}\u{000b}\u{001f}.mov"
                                .to_owned(),
                        ],
                    },
                },
            ],
            missing_bk2_outputs: 1,
            duplicate_inputs: 0,
        };
        let result = write_manifest(
            &root, &report,
        );
        assert!(
            result.is_ok(),
            "manifest should be writable"
        );
        let manifest_result = fs::read_to_string(root.join("manifest.tsv"));
        let _cleanup_result = fs::remove_dir_all(&root);
        assert!(
            manifest_result.is_ok(),
            "manifest should be readable"
        );
        let Ok(manifest) = manifest_result else {
            return;
        };
        let lines = manifest
            .lines()
            .collect::<Vec<_>>();
        assert_eq!(
            lines.len(),
            2
        );
        let Some(row) = lines.get(1) else {
            return;
        };
        assert_eq!(
            row.split('\t')
                .count(),
            6
        );
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
        let result = write_manifest(
            &left, &report,
        );
        assert!(
            result.is_ok(),
            "first manifest should be writable"
        );
        report
            .records
            .reverse();
        let second_result = write_manifest(
            &right, &report,
        );
        assert!(
            second_result.is_ok(),
            "second manifest should be writable"
        );
        let left_result = fs::read(left.join("manifest.tsv"));
        let right_result = fs::read(right.join("manifest.tsv"));
        let _cleanup_left = fs::remove_dir_all(&left);
        let _cleanup_right = fs::remove_dir_all(&right);
        assert!(
            left_result.is_ok(),
            "first manifest should be readable"
        );
        assert!(
            right_result.is_ok(),
            "second manifest should be readable"
        );
        let Ok(left_manifest) = left_result else {
            return;
        };
        let Ok(right_manifest) = right_result else {
            return;
        };
        assert_eq!(
            left_manifest,
            right_manifest
        );
    }

    #[test]
    fn writes_tab_separated_manifest_header() {
        let root = temp_root();
        let result = write_manifest(
            &root,
            &AuditReport::default(),
        );
        assert!(
            result.is_ok(),
            "empty audit manifest should be writable"
        );
        let manifest_result = fs::read_to_string(root.join("manifest.tsv"));
        let _cleanup_result = fs::remove_dir_all(&root);
        assert!(
            manifest_result.is_ok(),
            "written manifest should be readable"
        );
        let Ok(manifest) = manifest_result else {
            return;
        };
        let first_line = manifest
            .lines()
            .next()
            .unwrap_or_default();
        assert_eq!(
            first_line
                .split('\t')
                .collect::<Vec<_>>(),
            vec![
                "sha256",
                "bytes",
                "kind",
                "current_source",
                "pre_bink_master_evidence",
                "expected_bk2",
            ]
        );
    }
}
