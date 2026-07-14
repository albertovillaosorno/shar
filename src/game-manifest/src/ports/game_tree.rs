// File:
//   - game_tree.rs
// Path:
//   - src/game-manifest/src/ports/game_tree.rs
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
//   - Outbound recursive path-evidence contract for manifest workflows.
// - Must-Not:
//   - Classify files, render manifests, or select roots.
// - Allows:
//   - Report path kind and sorted regular-file snapshots for explicit roots.
// - Split-When:
//   - Split when remote and local tree evidence require different DTOs.
// - Merge-When:
//   - Another port owns the same path-evidence boundary.
// - Summary:
//   - Port for reading game tree evidence.
// - Description:
//   - Isolates application commands from concrete directory traversal.
// - Usage:
//   - Implemented by driven adapters and supplied by driving adapters.
// - Defaults:
//   - Missing roots remain explicit.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Outbound port for deterministic tree evidence.
//!
//! Application commands receive path snapshots without knowing the provider.
use std::io;
use std::path::{Path, PathBuf};

/// Observable kind of one external path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathKind {
    /// The path does not exist.
    Missing,
    /// The path is a regular file.
    File,
    /// The path is a directory.
    Directory,
    /// The path exists but is neither a regular file nor directory.
    Other,
}

/// Supplies recursive regular-file evidence for caller-selected roots.
pub trait GameTree {
    /// Returns the observable kind of one path.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when metadata cannot be inspected.
    fn kind(
        &self,
        path: &Path,
    ) -> io::Result<PathKind>;

    /// Returns all regular files beneath one directory in deterministic order.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when traversal or metadata access fails.
    fn files(
        &self,
        root: &Path,
    ) -> io::Result<Vec<PathBuf>>;
}
