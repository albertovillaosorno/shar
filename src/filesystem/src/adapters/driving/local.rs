// File:
//   - local.rs
// Path:
//   - src/filesystem/src/adapters/driving/local.rs
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
//   - Local composition of shared filesystem use cases and provider.
// - Must-Not:
//   - Add domain policy, infer caller paths, or bypass application commands.
// - Allows:
//   - Expose explicit local read, write, inspection, and traversal operations.
// - Split-When:
//   - Split when another inbound composition protocol is required.
// - Merge-When:
//   - Another adapter owns the same local composition surface.
// - Summary:
//   - Local filesystem composition adapter.
// - Description:
//   - Binds application use cases to the standard driven adapter.
// - Usage:
//   - Called inside filesystem-facing adapters of other crates.
// - Defaults:
//   - All paths and parent-creation choices remain explicit.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driving composition for local filesystem mechanisms.
//!
//! This is the convenience surface used by other crates' adapters.
use std::io;
use std::path::{Path, PathBuf};

use crate::adapters::driven::StdFilesystem;
use crate::application::{
    CollectRegularFiles, InspectPath, ReadFile, WriteFile,
};
use crate::domain::PathKind;

/// Reads complete bytes from one local path.
///
/// # Errors
///
/// Returns the local provider I/O error.
pub fn read_bytes(path: &Path) -> io::Result<Vec<u8>> {
    ReadFile::bytes(
        &StdFilesystem,
        path,
    )
}

/// Reads and validates complete UTF-8 text from one local path.
///
/// # Errors
///
/// Returns an I/O error for storage failure or invalid UTF-8.
pub fn read_utf8(path: &Path) -> io::Result<String> {
    ReadFile::utf8(
        &StdFilesystem,
        path,
    )
}

/// Reads optional UTF-8 text, mapping only not-found to `None`.
///
/// # Errors
///
/// Returns any other storage or UTF-8 validation error.
pub fn read_optional_utf8(path: &Path) -> io::Result<Option<String>> {
    ReadFile::optional_utf8(
        &StdFilesystem,
        path,
    )
}

/// Writes complete bytes to one explicit local path.
///
/// # Errors
///
/// Returns the local provider I/O error.
pub fn write_bytes(
    path: &Path,
    bytes: &[u8],
    create_parents: bool,
) -> io::Result<()> {
    WriteFile::bytes(
        &StdFilesystem,
        path,
        bytes,
        create_parents,
    )
}

/// Writes complete UTF-8 text to one explicit local path.
///
/// # Errors
///
/// Returns the local provider I/O error.
pub fn write_text(
    path: &Path,
    text: &str,
    create_parents: bool,
) -> io::Result<()> {
    WriteFile::text(
        &StdFilesystem,
        path,
        text,
        create_parents,
    )
}

/// Creates one local directory and every missing parent.
///
/// # Errors
///
/// Returns the local provider I/O error.
pub fn create_dir_all(path: &Path) -> io::Result<()> {
    WriteFile::directory(
        &StdFilesystem,
        path,
    )
}

/// Returns the stable kind of one local path.
///
/// # Errors
///
/// Returns the local provider I/O error when inspection fails.
pub fn path_kind(path: &Path) -> io::Result<PathKind> {
    InspectPath::kind(
        &StdFilesystem,
        path,
    )
}

/// Returns the metadata byte length of one local path.
///
/// # Errors
///
/// Returns the local provider I/O error when metadata is unavailable.
pub fn file_len(path: &Path) -> io::Result<u64> {
    InspectPath::len(
        &StdFilesystem,
        path,
    )
}

/// Returns the canonical identity of one existing local path.
///
/// # Errors
///
/// Returns the local provider I/O error when canonicalization fails.
pub fn canonicalize(path: &Path) -> io::Result<PathBuf> {
    InspectPath::canonicalize(
        &StdFilesystem,
        path,
    )
}

/// Collects sorted regular files beneath one local root.
///
/// # Errors
///
/// Returns the local provider I/O error when traversal fails.
pub fn regular_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    CollectRegularFiles::execute(
        &StdFilesystem,
        root,
    )
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{read_utf8, regular_files, write_text};

    static CASE_ID: AtomicUsize = AtomicUsize::new(0);

    fn case_root(label: &str) -> std::path::PathBuf {
        let id = CASE_ID.fetch_add(
            1,
            Ordering::Relaxed,
        );
        std::env::temp_dir().join(
            format!(
                "schoenwald-filesystem-{label}-{}-{id}",
                std::process::id()
            ),
        )
    }

    #[test]
    fn complete_text_write_creates_parents_and_round_trips()
    -> Result<(), String> {
        let root = case_root("text-round-trip");
        let destination = root.join("nested/report.txt");

        write_text(
            &destination,
            "complete report\n",
            true,
        )
        .map_err(|error| error.to_string())?;
        let actual =
            read_utf8(&destination).map_err(|error| error.to_string())?;

        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
        if actual
            != "complete report
"
        {
            return Err(format!("unexpected round-trip text: {actual:?}"));
        }
        Ok(())
    }

    #[test]
    fn recursive_snapshot_is_sorted_and_contains_only_files()
    -> Result<(), String> {
        let root = case_root("sorted-tree");
        fs::create_dir_all(root.join("z/nested"))
            .map_err(|error| error.to_string())?;
        fs::create_dir_all(root.join("a"))
            .map_err(|error| error.to_string())?;
        fs::write(
            root.join("z/nested/two.bin"),
            b"2",
        )
        .map_err(|error| error.to_string())?;
        fs::write(
            root.join("a/one.bin"),
            b"1",
        )
        .map_err(|error| error.to_string())?;

        let actual = regular_files(&root).map_err(|error| error.to_string())?;
        let expected = vec![
            root.join("a/one.bin"),
            root.join("z/nested/two.bin"),
        ];

        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
        if actual != expected {
            return Err(format!("unexpected recursive snapshot: {actual:?}"));
        }
        Ok(())
    }

    #[test]
    fn invalid_utf8_is_reported_as_invalid_data() -> Result<(), String> {
        let root = case_root("invalid-utf8");
        fs::create_dir_all(&root).map_err(|error| error.to_string())?;
        let source = root.join("invalid.txt");
        fs::write(
            &source,
            [0xff],
        )
        .map_err(|error| error.to_string())?;

        let read_error = match read_utf8(&source) {
            Ok(text) => {
                fs::remove_dir_all(&root)
                    .map_err(|cleanup_error| cleanup_error.to_string())?;
                return Err(
                    format!("invalid UTF-8 unexpectedly decoded as {text:?}"),
                );
            }
            Err(read_error) => read_error,
        };

        fs::remove_dir_all(&root)
            .map_err(|cleanup_error| cleanup_error.to_string())?;
        if read_error.kind() != std::io::ErrorKind::InvalidData {
            return Err(
                format!(
                    "unexpected invalid UTF-8 error kind: {:?}",
                    read_error.kind()
                ),
            );
        }
        Ok(())
    }
}
