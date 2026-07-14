// File:
//   - ports.rs
// Path:
//   - src/filesystem/src/ports.rs
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
//   - Narrow outbound contracts for shared filesystem mechanisms.
// - Must-Not:
//   - Implement storage behavior or encode caller-specific policy.
// - Allows:
//   - Declare complete reads, writes, inspection, and tree snapshots.
// - Split-When:
//   - Split when one capability becomes an independent provider contract.
// - Merge-When:
//   - Another facade owns the same mechanism contracts.
// - Summary:
//   - Shared filesystem ports.
// - Description:
//   - Isolates application use cases from concrete local storage.
// - Usage:
//   - Implemented by driven adapters and supplied by driving composition.
// - Defaults:
//   - Ports infer no paths, domain meaning, or recovery policy.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Hexagonal ports for shared filesystem mechanisms.
//!
//! Each contract owns one stable storage capability.
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
    fn read_bytes(
        &self,
        path: &Path,
    ) -> io::Result<Vec<u8>>;
}

/// Creates directories and writes complete file contents.
pub trait FileWriter {
    /// Creates one directory and every missing parent.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when creation fails.
    fn create_dir_all(
        &self,
        path: &Path,
    ) -> io::Result<()>;

    /// Replaces one file with the supplied complete bytes.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when writing fails.
    fn write_bytes(
        &self,
        path: &Path,
        bytes: &[u8],
    ) -> io::Result<()>;
}

/// Inspects explicit external paths.
pub trait PathInspector {
    /// Returns the observable kind of one path.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when existing metadata cannot be inspected.
    fn path_kind(
        &self,
        path: &Path,
    ) -> io::Result<PathKind>;

    /// Returns the byte length of one regular file.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when metadata is unavailable or the path is not a
    /// regular file.
    fn file_len(
        &self,
        path: &Path,
    ) -> io::Result<u64>;

    /// Returns the provider's canonical identity for one existing path.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when canonicalization fails.
    fn canonicalize(
        &self,
        path: &Path,
    ) -> io::Result<PathBuf>;
}

/// Supplies sorted regular-file paths beneath explicit roots.
pub trait TreeReader {
    /// Collects every regular file beneath one real directory.
    ///
    /// # Errors
    ///
    /// Returns an I/O error when the root is not a real directory or when
    /// traversal or entry inspection fails.
    fn regular_files(
        &self,
        root: &Path,
    ) -> io::Result<Vec<PathBuf>>;
}
