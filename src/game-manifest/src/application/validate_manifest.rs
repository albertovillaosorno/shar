// File:
//   - validate_manifest.rs
// Path:
//   - src/game-manifest/src/application/validate_manifest.rs
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
//   - Minimum manifest validation against current tree evidence.
// - Must-Not:
//   - Traverse filesystems, print diagnostics, or select concrete adapters.
// - Allows:
//   - Parse canonical rows, enforce ordering, and report count shortfalls.
// - Split-When:
//   - Split when schema validation and evidence comparison become independent.
// - Merge-When:
//   - Another use case owns the same minimum-manifest validation contract.
// - Summary:
//   - Application command for game manifest validation.
// - Description:
//   - Compares canonical requirements with current tree evidence.
// - Usage:
//   - Invoked by driving adapters with tree and text-store ports.
// - Defaults:
//   - Malformed or stale manifest data fails closed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application command for validating a minimum game manifest.
//!
//! Canonical rows are checked before current tree evidence is compared.
use std::collections::BTreeSet;
use std::path::Path;

use super::ManifestError;
use super::path_evidence::require_rooted_paths;
use crate::domain::{
    DirCount, DirExtCounts, MANIFEST_FILE_NAME, classify_manifest_bucket,
    count_by_dir_ext_paths, kind_taxonomy_jsonl,
};
use crate::ports::{GameTree, PathKind, TextArtifactStore};

/// Canonical obfuscated directory and normalized extension coordinate.
type Coordinate = (
    String,
    String,
);

/// Parsed requirement row paired with its canonical coordinate.
type Requirement = (
    DirCount,
    Coordinate,
);

/// Result of comparing manifest requirements with current evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateManifestReport {
    /// Number of canonical requirement rows.
    pub required_records: usize,
    /// Human-readable shortfall rows in manifest order.
    pub shortfalls: Vec<String>,
}

/// Stateless minimum-manifest validation use case.
#[derive(Debug, Clone, Copy)]
pub struct ValidateManifest;

impl ValidateManifest {
    /// Validates one game directory against its canonical manifest artifact.
    ///
    /// # Errors
    ///
    /// Returns a typed path, read, scan, or manifest-shape failure.
    pub fn execute(
        tree: &impl GameTree,
        store: &impl TextArtifactStore,
        game_dir: &Path,
    ) -> Result<ValidateManifestReport, ManifestError> {
        let kind = tree
            .kind(game_dir)
            .map_err(
                |error| {
                    ManifestError::io(
                        "inspect",
                        game_dir.to_path_buf(),
                        error,
                    )
                },
            )?;
        if kind != PathKind::Directory {
            return Err(
                ManifestError::Invalid(
                    format!(
                        "game directory not found: {}",
                        super::diagnostic_path::escaped_path(game_dir)
                    ),
                ),
            );
        }
        let manifest_path = game_dir.join(MANIFEST_FILE_NAME);
        let manifest = store
            .read_optional(&manifest_path)
            .map_err(
                |error| {
                    ManifestError::io(
                        "read",
                        manifest_path.clone(),
                        error,
                    )
                },
            )?
            .ok_or_else(
                || {
                    ManifestError::Invalid(
                        format!(
                            "manifest not found: {}",
                            super::diagnostic_path::escaped_path(
                                &manifest_path
                            )
                        ),
                    )
                },
            )?;
        let parsed = parse_manifest(&manifest);
        let requirements = parsed.map_err(ManifestError::Invalid)?;
        let files = tree
            .files(game_dir)
            .map_err(
                |error| {
                    ManifestError::io(
                        "scan",
                        game_dir.to_path_buf(),
                        error,
                    )
                },
            )?;
        require_rooted_paths(
            game_dir, &files,
        )
        .map_err(ManifestError::Invalid)?;
        let actual = count_by_dir_ext_paths(
            game_dir, &files,
        );
        Ok(
            compare_requirements(
                &requirements,
                &actual,
            ),
        )
    }
}

/// Validates one row and returns its canonical coordinate.
fn validate_record(
    record: &DirCount,
    line_number: usize,
    seen: &mut BTreeSet<Coordinate>,
    previous: &mut Option<Coordinate>,
) -> Result<Coordinate, String> {
    let expected_kind = classify_manifest_bucket(
        &record.dir,
        &record.extension,
    );
    if expected_kind == "error" {
        return Err(format!("unclassified coordinate at line {line_number}"));
    }
    if record.kind != expected_kind {
        return Err(
            format!(
                "kind mismatch at line {line_number}: expected {expected_kind}"
            ),
        );
    }
    let optional_zero = record
        .dir
        .is_empty()
        && matches!(
            record
                .extension
                .as_str(),
            "lmlm" | "png"
        );
    if record.min_count == 0 && !optional_zero {
        return Err(
            format!(
                "zero minimum for required coordinate at line {line_number}"
            ),
        );
    }
    let key = (
        record
            .dir
            .clone(),
        record
            .extension
            .clone(),
    );
    if !seen.insert(key.clone()) {
        return Err(format!("duplicate coordinate at line {line_number}"));
    }
    if previous
        .as_ref()
        .is_some_and(|value| value > &key)
    {
        return Err(format!("out-of-order coordinate at line {line_number}"));
    }
    *previous = Some(key.clone());
    Ok(key)
}

/// Parses and validates all canonical manifest requirement rows.
fn parse_manifest(manifest: &str) -> Result<Vec<Requirement>, String> {
    if manifest.contains('\r') {
        return Err("manifest must use LF line endings".to_owned());
    }
    if !manifest.ends_with('\n') {
        return Err("manifest must end with a newline".to_owned());
    }
    let expected_taxonomy = kind_taxonomy_jsonl();
    let mut lines = manifest.lines();
    if lines.next() != Some(expected_taxonomy.as_str()) {
        return Err("missing or stale taxonomy header".to_owned());
    }
    let mut requirements = Vec::new();
    let mut seen = BTreeSet::new();
    let mut previous = None;
    for (line_offset, line) in lines.enumerate() {
        let line_number = line_offset.saturating_add(2);
        let record = DirCount::parse(line)
            .ok_or_else(|| format!("invalid row at line {line_number}"))?;
        let key = validate_record(
            &record,
            line_number,
            &mut seen,
            &mut previous,
        )?;
        requirements.push(
            (
                record, key,
            ),
        );
    }
    if requirements.is_empty() {
        return Err("no directory-count records found".to_owned());
    }
    let language_mod = (
        String::new(),
        crate::domain::OPTIONAL_EXTENSION.to_owned(),
    );
    if !seen.contains(&language_mod) {
        return Err("missing synthetic root coordinate: .lmlm".to_owned());
    }
    let generated_image = (
        String::new(),
        crate::domain::GENERATED_IMAGE_EXTENSION.to_owned(),
    );
    if !seen.contains(&generated_image) {
        return Err("missing synthetic root coordinate: .png".to_owned());
    }
    Ok(requirements)
}

/// Compares parsed requirements with current deterministic counts.
fn compare_requirements(
    requirements: &[Requirement],
    actual: &DirExtCounts,
) -> ValidateManifestReport {
    let mut shortfalls = Vec::new();
    for (record, key) in requirements {
        let have = actual
            .get(key)
            .copied()
            .unwrap_or(0);
        if have < record.min_count {
            let location = if record
                .dir
                .is_empty()
            {
                "<root>".to_owned()
            } else {
                record
                    .dir
                    .clone()
            };
            shortfalls.push(
                format!(
                    "  {location} .{}: have {have}, need at least {}",
                    record.extension, record.min_count
                ),
            );
        }
    }
    ValidateManifestReport {
        required_records: requirements.len(),
        shortfalls,
    }
}
