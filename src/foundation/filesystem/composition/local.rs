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
//   - Local inbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Local inbound adapter.
// - Description:
//   - Implements the declared inbound adapter responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Local inbound adapter.

use std::io;
use std::path::{Path, PathBuf};

use crate::application::{
    CollectRegularFiles, CollectStrictRegularFiles, InspectPath, ReadFile, WriteFile,
};
use crate::domain::PathKind;
use crate::std_filesystem::StdFilesystem;

/// Reads complete bytes from one local path.
///
/// # Errors
///
/// Returns the local provider I/O error.
pub fn read_bytes(path: &Path) -> io::Result<Vec<u8>> {
    ReadFile::bytes(&StdFilesystem, path)
}

/// Reads and validates complete UTF-8 text from one local path.
///
/// # Errors
///
/// Returns an I/O error for storage failure or invalid UTF-8.
pub fn read_utf8(path: &Path) -> io::Result<String> {
    ReadFile::utf8(&StdFilesystem, path)
}

/// Reads optional UTF-8 text, mapping only not-found to `None`.
///
/// # Errors
///
/// Returns any other storage or UTF-8 validation error.
pub fn read_optional_utf8(path: &Path) -> io::Result<Option<String>> {
    ReadFile::optional_utf8(&StdFilesystem, path)
}

/// Writes complete bytes to one explicit local path.
///
/// # Errors
///
/// Returns the local provider I/O error.
pub fn write_bytes(path: &Path, bytes: &[u8], create_parents: bool) -> io::Result<()> {
    WriteFile::bytes(&StdFilesystem, path, bytes, create_parents)
}

/// Writes complete UTF-8 text to one explicit local path.
///
/// # Errors
///
/// Returns the local provider I/O error.
pub fn write_text(path: &Path, text: &str, create_parents: bool) -> io::Result<()> {
    WriteFile::text(&StdFilesystem, path, text, create_parents)
}

/// Creates one new UTF-8 text file at an explicit local path.
///
/// # Errors
///
/// Returns `AlreadyExists` when the destination exists, or another local
/// provider I/O error.
pub fn write_new_text(
    path: &Path,
    text: &str,
    create_parents: bool,
) -> io::Result<()> {
    WriteFile::new_text(&StdFilesystem, path, text, create_parents)
}

/// Creates one local directory and every missing parent.
///
/// # Errors
///
/// Returns the local provider I/O error.
pub fn create_dir_all(path: &Path) -> io::Result<()> {
    WriteFile::directory(&StdFilesystem, path)
}

/// Returns the stable kind of one local path.
///
/// # Errors
///
/// Returns the local provider I/O error when inspection fails.
pub fn path_kind(path: &Path) -> io::Result<PathKind> {
    InspectPath::kind(&StdFilesystem, path)
}

/// Returns the metadata byte length of one local path.
///
/// # Errors
///
/// Returns the local provider I/O error when metadata is unavailable.
pub fn file_len(path: &Path) -> io::Result<u64> {
    InspectPath::len(&StdFilesystem, path)
}

/// Returns the canonical identity of one existing local path.
///
/// # Errors
///
/// Returns the local provider I/O error when canonicalization fails.
pub fn canonicalize(path: &Path) -> io::Result<PathBuf> {
    InspectPath::canonicalize(&StdFilesystem, path)
}

/// Collects sorted regular files beneath one local root.
///
/// # Errors
///
/// Returns the local provider I/O error when traversal fails.
pub fn regular_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    CollectRegularFiles::execute(&StdFilesystem, root)
}

/// Collects sorted regular files and rejects redirects or special entries.
///
/// # Errors
///
/// Returns the local provider I/O error when traversal or strict validation
/// fails.
pub fn strict_regular_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    CollectStrictRegularFiles::execute(&StdFilesystem, root)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../tests/foundation/filesystem/unit/adapter-inbound/local/tests.rs"]
mod tests;
