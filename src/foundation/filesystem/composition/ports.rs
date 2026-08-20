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
//   - Port outbound outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Port outbound outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Port outbound outbound port.

use std::io;
use std::path::{Path, PathBuf};

use crate::domain::PathKind;

/// Reads complete file contents.
pub trait FileReader {
    /// Reads all bytes from one explicit path.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when the complete read fails.
    fn read_bytes(&self, path: &Path) -> io::Result<Vec<u8>>;
}

/// Creates directories and writes complete file contents.
pub trait FileWriter {
    /// Creates one directory and every missing parent.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when creation fails.
    fn create_dir_all(&self, path: &Path) -> io::Result<()>;

    /// Replaces one file with the supplied complete bytes.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when writing fails.
    fn write_bytes(&self, path: &Path, bytes: &[u8]) -> io::Result<()>;

    /// Creates one new file with the supplied complete bytes.
    ///
    /// # Errors
    ///
    /// Returns `AlreadyExists` when the destination exists, or another provider
    /// I/O error when creation or writing fails.
    fn write_new_bytes(&self, path: &Path, bytes: &[u8]) -> io::Result<()>;
}

/// Inspects explicit external paths.
pub trait PathInspector {
    /// Returns the observable kind of one path.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when existing metadata cannot be inspected.
    fn path_kind(&self, path: &Path) -> io::Result<PathKind>;

    /// Returns the byte length of one regular file.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when metadata is unavailable or the path is not a
    /// regular file.
    fn file_len(&self, path: &Path) -> io::Result<u64>;

    /// Returns the provider's canonical identity for one existing path.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when canonicalization fails.
    fn canonicalize(&self, path: &Path) -> io::Result<PathBuf>;
}

/// Supplies sorted regular-file paths beneath explicit roots.
pub trait TreeReader {
    /// Collects every regular file beneath one real directory.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the root is not a real directory or when
    /// traversal or entry inspection fails.
    fn regular_files(&self, root: &Path) -> io::Result<Vec<PathBuf>>;

    /// Collects regular files while rejecting redirects and special entries.
    ///
    /// Providers that cannot distinguish strict tree membership may keep the
    /// default behavior; security-sensitive concrete adapters should override
    /// this method and fail closed on every non-file, non-directory entry.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when traversal or strict entry validation fails.
    fn strict_regular_files(&self, root: &Path) -> io::Result<Vec<PathBuf>> {
        self.regular_files(root)
    }
}
