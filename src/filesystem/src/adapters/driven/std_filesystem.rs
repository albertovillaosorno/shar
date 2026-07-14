// File:
//   - std_filesystem.rs
// Path:
//   - src/filesystem/src/adapters/driven/std_filesystem.rs
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
//   - Standard-library implementations of shared filesystem ports.
// - Must-Not:
//   - Infer paths, classify content, or encode workflow recovery policy.
// - Allows:
//   - Perform explicit local reads, writes, inspection, and sorted traversal.
// - Split-When:
//   - Split when another provider needs an independent adapter.
// - Merge-When:
//   - Another adapter owns the same standard-library mechanism contract.
// - Summary:
//   - Standard local filesystem adapter.
// - Description:
//   - Implements narrow ports with `std::fs` and deterministic ordering.
// - Usage:
//   - Selected by the local driving composition adapter.
// - Defaults:
//   - Recursive traversal returns only regular files and does not follow links.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Driven adapter backed by the Rust standard library.
//!
//! The adapter owns mechanisms only; path meaning stays with callers.
use std::path::{Path, PathBuf};
use std::{fs, io};

use super::io_context::{invalid_input, with_path};
use crate::domain::PathKind;
use crate::ports::{FileReader, FileWriter, PathInspector, TreeReader};

/// Rejects an existing link before one filesystem access.
fn reject_existing_link(path: &Path) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(metadata) => {
            let file_type = metadata.file_type();
            if file_type.is_symlink() {
                Err(
                    invalid_input(
                        "validate filesystem access",
                        path,
                        "path must not include a link",
                    ),
                )
            } else {
                Ok(())
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(
            with_path(
                "inspect path link metadata",
                path,
                source,
            ),
        ),
    }
}

/// Rejects links in every existing lexical prefix of one access path.
fn reject_links_in_path(path: &Path) -> io::Result<()> {
    for prefix in path.ancestors() {
        reject_existing_link(prefix)?;
    }
    Ok(())
}

/// Rejects links in existing parent prefixes while preserving the final kind.
fn reject_links_in_parents(path: &Path) -> io::Result<()> {
    for prefix in path
        .ancestors()
        .skip(1)
    {
        reject_existing_link(prefix)?;
    }
    Ok(())
}

/// Maps link-aware metadata to the public path-kind model.
fn path_kind_from_metadata(metadata: &fs::Metadata) -> PathKind {
    if metadata
        .file_type()
        .is_symlink()
    {
        PathKind::Other
    } else if metadata.is_file() {
        PathKind::File
    } else if metadata.is_dir() {
        PathKind::Directory
    } else {
        PathKind::Other
    }
}

/// Standard-library local filesystem provider.
#[derive(Debug, Default, Clone, Copy)]
pub struct StdFilesystem;

impl FileReader for StdFilesystem {
    fn read_bytes(
        &self,
        path: &Path,
    ) -> io::Result<Vec<u8>> {
        reject_links_in_path(path)?;
        match fs::read(path) {
            Ok(bytes) => Ok(bytes),
            Err(source) => Err(
                with_path(
                    "read file",
                    path,
                    source,
                ),
            ),
        }
    }
}

impl FileWriter for StdFilesystem {
    fn create_dir_all(
        &self,
        path: &Path,
    ) -> io::Result<()> {
        reject_links_in_path(path)?;
        match fs::create_dir_all(path) {
            Ok(()) => Ok(()),
            Err(source) => Err(
                with_path(
                    "create directory tree",
                    path,
                    source,
                ),
            ),
        }
    }

    fn write_bytes(
        &self,
        path: &Path,
        bytes: &[u8],
    ) -> io::Result<()> {
        reject_links_in_path(path)?;
        match fs::write(
            path, bytes,
        ) {
            Ok(()) => Ok(()),
            Err(source) => Err(
                with_path(
                    "write file",
                    path,
                    source,
                ),
            ),
        }
    }
}

impl PathInspector for StdFilesystem {
    fn path_kind(
        &self,
        path: &Path,
    ) -> io::Result<PathKind> {
        reject_links_in_parents(path)?;
        match fs::symlink_metadata(path) {
            Ok(metadata) => Ok(path_kind_from_metadata(&metadata)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                Ok(PathKind::Missing)
            }
            Err(source) => Err(
                with_path(
                    "inspect path metadata",
                    path,
                    source,
                ),
            ),
        }
    }

    fn file_len(
        &self,
        path: &Path,
    ) -> io::Result<u64> {
        reject_links_in_path(path)?;
        let metadata = match fs::metadata(path) {
            Ok(metadata) => metadata,
            Err(source) => {
                return Err(
                    with_path(
                        "inspect file metadata",
                        path,
                        source,
                    ),
                );
            }
        };
        if !metadata.is_file() {
            return Err(
                invalid_input(
                    "inspect file metadata",
                    path,
                    "path must identify a regular file",
                ),
            );
        }
        Ok(metadata.len())
    }

    fn canonicalize(
        &self,
        path: &Path,
    ) -> io::Result<PathBuf> {
        match fs::canonicalize(path) {
            Ok(identity) => Ok(identity),
            Err(source) => Err(
                with_path(
                    "canonicalize path",
                    path,
                    source,
                ),
            ),
        }
    }
}

impl TreeReader for StdFilesystem {
    fn regular_files(
        &self,
        root: &Path,
    ) -> io::Result<Vec<PathBuf>> {
        reject_links_in_path(root)?;
        let root_metadata = match fs::symlink_metadata(root) {
            Ok(metadata) => metadata,
            Err(source) => {
                return Err(
                    with_path(
                        "inspect traversal root",
                        root,
                        source,
                    ),
                );
            }
        };
        let root_type = root_metadata.file_type();
        if root_type.is_symlink() || !root_type.is_dir() {
            return Err(
                invalid_input(
                    "inspect traversal root",
                    root,
                    "traversal root must be a real directory",
                ),
            );
        }
        let mut pending = vec![root.to_path_buf()];
        let mut files = Vec::new();
        while let Some(directory) = pending.pop() {
            let entries = match fs::read_dir(&directory) {
                Ok(entries) => entries,
                Err(source) => {
                    return Err(
                        with_path(
                            "read directory",
                            &directory,
                            source,
                        ),
                    );
                }
            };
            for entry_result in entries {
                let entry = match entry_result {
                    Ok(entry) => entry,
                    Err(source) => {
                        return Err(
                            with_path(
                                "read directory entry",
                                &directory,
                                source,
                            ),
                        );
                    }
                };
                let path = entry.path();
                let file_type = match entry.file_type() {
                    Ok(file_type) => file_type,
                    Err(source) => {
                        return Err(
                            with_path(
                                "inspect directory entry",
                                &path,
                                source,
                            ),
                        );
                    }
                };
                if file_type.is_dir() {
                    pending.push(path);
                } else if file_type.is_file() {
                    files.push(path);
                }
            }
        }
        files.sort();
        Ok(files)
    }
}
