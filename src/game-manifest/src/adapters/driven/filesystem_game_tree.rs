// File:
//   - filesystem_game_tree.rs
// Path:
//   - src/game-manifest/src/adapters/driven/filesystem_game_tree.rs
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
//   - Filesystem implementation of recursive game tree evidence.
// - Must-Not:
//   - Classify files, render records, or select roots.
// - Allows:
//   - Inspect metadata and collect sorted regular-file paths.
// - Split-When:
//   - Split when symlink policy or remote traversal needs another adapter.
// - Merge-When:
//   - Another adapter owns the same filesystem tree contract.
// - Summary:
//   - Driven filesystem tree adapter.
// - Description:
//   - Implements deterministic recursive evidence behind the tree port.
// - Usage:
//   - Selected by manifest CLI composition roots.
// - Defaults:
//   - Only regular files are returned.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Filesystem adapter for deterministic game tree evidence.
//!
//! Shared local traversal and path inspection remain centralized.
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind as SharedPathKind;
use schoenwald_filesystem::adapters::driving::local;

use crate::ports::{GameTree, PathKind};

/// Supplies local filesystem tree evidence.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemGameTree;

impl GameTree for FilesystemGameTree {
    fn kind(
        &self,
        path: &Path,
    ) -> io::Result<PathKind> {
        local::path_kind(path).map(map_path_kind)
    }

    fn files(
        &self,
        root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        local::regular_files(root)
    }
}

/// Maps shared filesystem evidence into the game-manifest port type.
const fn map_path_kind(kind: SharedPathKind) -> PathKind {
    match kind {
        SharedPathKind::Missing => PathKind::Missing,
        SharedPathKind::File => PathKind::File,
        SharedPathKind::Directory => PathKind::Directory,
        SharedPathKind::Other => PathKind::Other,
    }
}
