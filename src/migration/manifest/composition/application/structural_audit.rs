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
//   - Structural audit application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Structural audit application service.
// - Description:
//   - Implements the declared application service responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Structural audit application service.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use schoenwald_filesystem::resolve_under;

use super::ManifestError;
use crate::domain::{BACKUP_EXTENSION, extension_of};
use crate::ports::{GameTree, PathKind};

/// Extensions intentionally excluded from the ephemeral structural audit.
const IGNORED_EXTENSIONS: &[&str] = &[
    "rcf",
    "p3d",
    "rmv",
    "lmlm",
    "ico",
    "rtf",
    "jsonl",
    BACKUP_EXTENSION,
];

/// Deterministically ordered extension audit result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralAuditReport {
    /// Total files whose extension is outside the ignored set.
    pub total_dirty_extensions: usize,
    /// Extension/count rows ordered by descending count then extension.
    pub rows: Vec<(String, usize)>,
}

/// Stateless structural audit use case.
#[derive(Debug, Clone, Copy)]
pub struct StructuralAudit;

impl StructuralAudit {
    /// Audits one explicit game directory.
    ///
    /// # Errors
    ///
    /// Returns a typed path inspection or traversal failure.
    pub fn execute(
        tree: &impl GameTree,
        game_dir: &Path,
    ) -> Result<StructuralAuditReport, ManifestError> {
        let kind = tree.kind(game_dir).map_err(|error| {
            ManifestError::io("inspect", game_dir.to_path_buf(), error)
        })?;
        if kind != PathKind::Directory {
            return Err(ManifestError::Invalid(format!(
                "game directory not found: {}",
                super::diagnostic_path::escaped_path(game_dir)
            )));
        }
        let files = tree.files(game_dir).map_err(|error| {
            ManifestError::io("scan", game_dir.to_path_buf(), error)
        })?;
        let mut counts = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for path in &files {
            let Ok(relative) = path.strip_prefix(game_dir) else {
                return Err(ManifestError::Invalid(
                    "tree snapshot contains an unsafe game path".to_owned(),
                ));
            };
            if resolve_under(game_dir, relative).is_err() {
                return Err(ManifestError::Invalid(
                    "tree snapshot contains an unsafe game path".to_owned(),
                ));
            }
            if !seen.insert(path) {
                continue;
            }
            let extension = extension_of(path);
            if !IGNORED_EXTENSIONS.contains(&extension.as_str()) {
                let count = counts.entry(extension).or_insert(0_usize);
                *count = count.saturating_add(1);
            }
        }
        let total_dirty_extensions = counts.values().sum();
        let mut rows = counts.into_iter().collect::<Vec<_>>();
        rows.sort_by(|left, right| {
            right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0))
        });
        Ok(StructuralAuditReport {
            total_dirty_extensions,
            rows,
        })
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/manifest/unit/application/structural_audit/tests.rs"]
mod tests;
