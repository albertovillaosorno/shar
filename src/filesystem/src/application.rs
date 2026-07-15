// File:
//   - application.rs
// Path:
//   - src/filesystem/src/application.rs
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
//   - Shared complete-file, inspection, and tree-snapshot use cases.
// - Must-Not:
//   - Implement storage providers or caller-specific workflow policy.
// - Allows:
//   - Coordinate narrow filesystem ports with explicit caller choices.
// - Split-When:
//   - Split when one use-case family becomes independently versioned.
// - Merge-When:
//   - Another facade owns the same filesystem commands.
// - Summary:
//   - Shared filesystem application use cases.
// - Description:
//   - Provides reusable mechanism orchestration without concrete storage.
// - Usage:
//   - Called by driving composition and advanced library clients.
// - Defaults:
//   - Every path and parent-creation choice is explicit.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Shared filesystem application use cases.
//!
//! Commands depend only on pure domain values and outbound ports.
mod diagnostic_path;
mod path_validation;

use std::collections::BTreeSet;
use std::io;
use std::path::{Path, PathBuf};

use self::path_validation::{
    has_file_destination, has_meaningful_component, has_named_destination,
    path_error, require_explicit_path, require_tree_descendant,
    require_tree_root, utf8_error,
};
use crate::domain::PathKind;
use crate::ports::{FileReader, FileWriter, PathInspector, TreeReader};

/// Stateless complete-file read use cases.
#[derive(Debug, Clone, Copy)]
pub struct ReadFile;

impl ReadFile {
    /// Reads complete bytes from one explicit path.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when reading fails.
    pub fn bytes(
        reader: &(impl FileReader + ?Sized),
        path: &Path,
    ) -> io::Result<Vec<u8>> {
        require_explicit_path(
            path,
            "read file",
        )?;
        if !has_file_destination(path) {
            return Err(
                path_error(
                    io::ErrorKind::InvalidInput,
                    "read file",
                    path,
                    "path must identify an explicit file destination",
                ),
            );
        }
        reader.read_bytes(path)
    }

    /// Reads and validates complete UTF-8 text.
    ///
    /// # Errors
    ///
    /// Returns an I/O error for read failures or invalid UTF-8.
    pub fn utf8(
        reader: &(impl FileReader + ?Sized),
        path: &Path,
    ) -> io::Result<String> {
        let bytes = Self::bytes(
            reader, path,
        )?;
        String::from_utf8(bytes).map_err(
            |error| {
                utf8_error(
                    path, error,
                )
            },
        )
    }

    /// Reads optional UTF-8 text, mapping only not-found to `None`.
    ///
    /// # Errors
    ///
    /// Returns an I/O error for other failures or invalid UTF-8.
    pub fn optional_utf8(
        reader: &(impl FileReader + ?Sized),
        path: &Path,
    ) -> io::Result<Option<String>> {
        match Self::utf8(
            reader, path,
        ) {
            Ok(text) => Ok(Some(text)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }
}

/// Stateless complete-file write use cases.
#[derive(Debug, Clone, Copy)]
pub struct WriteFile;

impl WriteFile {
    /// Writes complete bytes and optionally creates missing parents.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when creation or writing fails.
    pub fn bytes(
        writer: &(impl FileWriter + ?Sized),
        path: &Path,
        bytes: &[u8],
        create_parents: bool,
    ) -> io::Result<()> {
        require_explicit_path(
            path,
            "write file",
        )?;
        if !has_file_destination(path) {
            return Err(
                path_error(
                    io::ErrorKind::InvalidInput,
                    "write file",
                    path,
                    "path must identify an explicit file destination",
                ),
            );
        }
        if create_parents
            && let Some(parent) = path.parent()
            && !parent
                .as_os_str()
                .is_empty()
            && has_meaningful_component(parent)
        {
            writer.create_dir_all(parent)?;
        }
        writer.write_bytes(
            path, bytes,
        )
    }

    /// Writes complete UTF-8 text and optionally creates missing parents.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when creation or writing fails.
    pub fn text(
        writer: &(impl FileWriter + ?Sized),
        path: &Path,
        text: &str,
        create_parents: bool,
    ) -> io::Result<()> {
        Self::bytes(
            writer,
            path,
            text.as_bytes(),
            create_parents,
        )
    }

    /// Creates one directory and every missing parent.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when creation fails.
    pub fn directory(
        writer: &(impl FileWriter + ?Sized),
        path: &Path,
    ) -> io::Result<()> {
        require_explicit_path(
            path,
            "create directory tree",
        )?;
        if !has_named_destination(path) {
            return Err(
                path_error(
                    io::ErrorKind::InvalidInput,
                    "create directory tree",
                    path,
                    "path must identify explicit directory state",
                ),
            );
        }
        writer.create_dir_all(path)
    }
}

/// Stateless path-inspection use cases.
#[derive(Debug, Clone, Copy)]
pub struct InspectPath;

impl InspectPath {
    /// Returns the stable kind of one path.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when inspection fails.
    pub fn kind(
        inspector: &(impl PathInspector + ?Sized),
        path: &Path,
    ) -> io::Result<PathKind> {
        require_explicit_path(
            path,
            "inspect path metadata",
        )?;
        inspector.path_kind(path)
    }

    /// Returns the metadata byte length of one existing path.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when metadata is unavailable.
    pub fn len(
        inspector: &(impl PathInspector + ?Sized),
        path: &Path,
    ) -> io::Result<u64> {
        require_explicit_path(
            path,
            "inspect file metadata",
        )?;
        if !has_file_destination(path) {
            return Err(
                path_error(
                    io::ErrorKind::InvalidInput,
                    "inspect file metadata",
                    path,
                    "path must identify an explicit file destination",
                ),
            );
        }
        inspector.file_len(path)
    }

    /// Returns the canonical identity of one existing path.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when canonicalization fails.
    pub fn canonicalize(
        inspector: &(impl PathInspector + ?Sized),
        path: &Path,
    ) -> io::Result<PathBuf> {
        require_explicit_path(
            path,
            "canonicalize path",
        )?;
        inspector.canonicalize(path)
    }
}

/// Stateless regular-file collection use case.
#[derive(Debug, Clone, Copy)]
pub struct CollectRegularFiles;

impl CollectRegularFiles {
    /// Collects sorted regular-file paths beneath one root.
    ///
    /// # Errors
    ///
    /// Returns the provider I/O error when traversal fails.
    pub fn execute(
        reader: &(impl TreeReader + ?Sized),
        root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        require_tree_root(root)?;
        let mut files = reader.regular_files(root)?;
        files.sort();
        files.dedup();
        let mut identities = BTreeSet::new();
        for path in &files {
            let identity = require_tree_descendant(
                root, path,
            )?;
            if !identities.insert(identity) {
                return Err(
                    path_error(
                        io::ErrorKind::InvalidData,
                        "validate tree identity",
                        path,
                        "path collides with an earlier portable identity",
                    ),
                );
            }
        }
        Ok(files)
    }
}
