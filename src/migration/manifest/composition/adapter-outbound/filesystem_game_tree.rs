// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Filesystem game tree outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem game tree outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem game tree outbound adapter.

use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind as SharedPathKind;
use schoenwald_filesystem::adapters::driving::local;

use crate::ports::{GameTree, PathKind};

/// Supplies local filesystem tree evidence.
#[derive(Debug, Default, Clone, Copy)]
pub struct FilesystemGameTree;

impl GameTree for FilesystemGameTree {
    fn kind(&self, path: &Path) -> io::Result<PathKind> {
        local::path_kind(path).map(map_path_kind)
    }

    fn files(&self, root: &Path) -> io::Result<Vec<PathBuf>> {
        local::strict_regular_files(root)
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
