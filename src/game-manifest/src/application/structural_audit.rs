// File:
//   - structural_audit.rs
// Path:
//   - src/game-manifest/src/application/structural_audit.rs
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
//   - Ephemeral extension-audit counting and deterministic row ordering.
// - Must-Not:
//   - Traverse filesystems, print results, or select concrete adapters.
// - Allows:
//   - Count noncanonical extensions from supplied file evidence.
// - Split-When:
//   - Split when filtering and report ordering become independent commands.
// - Merge-When:
//   - Another use case owns the same structural-audit contract.
// - Summary:
//   - Application command for structural extension audits.
// - Description:
//   - Produces deterministic extension counts from explicit tree evidence.
// - Usage:
//   - Invoked by a driving adapter with a game tree port.
// - Defaults:
//   - Known source and manifest extensions are ignored.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Application command for ephemeral structural extension audits.
//!
//! The command counts supplied path evidence without performing external IO.
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
    pub rows: Vec<(
        String,
        usize,
    )>,
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
        let mut counts = BTreeMap::new();
        let mut seen = BTreeSet::new();
        for path in &files {
            let Ok(relative) = path.strip_prefix(game_dir) else {
                return Err(
                    ManifestError::Invalid(
                        "tree snapshot contains an unsafe game path".to_owned(),
                    ),
                );
            };
            if resolve_under(
                game_dir, relative,
            )
            .is_err()
            {
                return Err(
                    ManifestError::Invalid(
                        "tree snapshot contains an unsafe game path".to_owned(),
                    ),
                );
            }
            if !seen.insert(path) {
                continue;
            }
            let extension = extension_of(path);
            if !IGNORED_EXTENSIONS.contains(&extension.as_str()) {
                let count = counts
                    .entry(extension)
                    .or_insert(0_usize);
                *count = count.saturating_add(1);
            }
        }
        let total_dirty_extensions = counts
            .values()
            .sum();
        let mut rows = counts
            .into_iter()
            .collect::<Vec<_>>();
        rows.sort_by(
            |left, right| {
                right
                    .1
                    .cmp(&left.1)
                    .then_with(
                        || {
                            left.0
                                .cmp(&right.0)
                        },
                    )
            },
        );
        Ok(
            StructuralAuditReport {
                total_dirty_extensions,
                rows,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use std::path::{Path, PathBuf};

    use super::StructuralAudit;
    use crate::domain::{NO_EXTENSION, extension_of};
    use crate::ports::{GameTree, PathKind};

    struct DuplicateTree;
    struct OutsideTree;
    struct ParentTraversalTree;

    impl GameTree for DuplicateTree {
        fn kind(
            &self,
            _path: &Path,
        ) -> io::Result<PathKind> {
            Ok(PathKind::Directory)
        }

        fn files(
            &self,
            root: &Path,
        ) -> io::Result<Vec<PathBuf>> {
            let path = root.join("asset.mfk");
            Ok(
                vec![
                    path.clone(),
                    path,
                ],
            )
        }
    }

    impl GameTree for OutsideTree {
        fn kind(
            &self,
            _path: &Path,
        ) -> io::Result<PathKind> {
            Ok(PathKind::Directory)
        }

        fn files(
            &self,
            _root: &Path,
        ) -> io::Result<Vec<PathBuf>> {
            Ok(vec![PathBuf::from("other/asset.mfk")])
        }
    }

    impl GameTree for ParentTraversalTree {
        fn kind(
            &self,
            _path: &Path,
        ) -> io::Result<PathKind> {
            Ok(PathKind::Directory)
        }

        fn files(
            &self,
            root: &Path,
        ) -> io::Result<Vec<PathBuf>> {
            Ok(vec![root.join("area/../asset.mfk")])
        }
    }

    #[test]
    fn duplicate_file_evidence_counts_once() {
        let report = StructuralAudit::execute(
            &DuplicateTree,
            Path::new("game"),
        );

        assert_eq!(
            report
                .ok()
                .map(|value| value.total_dirty_extensions,),
            Some(1),
        );
    }

    #[test]
    fn outside_root_evidence_is_rejected() {
        let result = StructuralAudit::execute(
            &OutsideTree,
            Path::new("game"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn parent_traversal_evidence_is_rejected() {
        let result = StructuralAudit::execute(
            &ParentTraversalTree,
            Path::new("game"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn trailing_dot_extension_is_missing() {
        assert_eq!(
            extension_of(Path::new("asset.")),
            NO_EXTENSION
        );
    }
}
