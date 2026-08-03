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
//   - Game tree outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Game tree outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Game tree outbound port.

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
    fn kind(&self, path: &Path) -> io::Result<PathKind>;

    /// Returns all regular files beneath one directory in deterministic order.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when traversal or metadata access fails.
    fn files(&self, root: &Path) -> io::Result<Vec<PathBuf>>;
}
