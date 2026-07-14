// File:
//   - destination.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/destination.rs
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
//   - Complete destination resolution and preflight for LMLM entries.
// - Must-Not:
//   - Slice payloads or mutate filesystem state.
// - Allows:
//   - Inspect existing paths and reject portable identity collisions.
// - Split-When:
//   - Existing-tree inventory or platform identity gains independent state.
// - Merge-When:
//   - Another module owns the complete destination preflight contract.
// - Summary:
//   - Resolves and validates every destination before publication.
// - Description:
//   - Enforces path safety, parent types, collisions, and create-new policy.
// - Usage:
//   - Called by the filesystem materialization facade.
// - Defaults:
//   - No destination path may escape or overwrite the output root.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Destination preflight for LMLM filesystem materialization.
//!
//! Every path and portable identity is validated before publication begins.

use std::collections::BTreeSet;
use std::io;
use std::path::{Component, Path, PathBuf};

use schoenwald_filesystem::PathKind;

use super::inspection::inspect_path_kind;
use crate::diagnostic::EscapedText;
use crate::domain::{FileEntry, portable_identity, portable_path_is_safe};

/// Builds one local destination from a validated archive path.
fn destination_path(
    output_root: &Path,
    entry_path: &str,
) -> io::Result<PathBuf> {
    if !portable_path_is_safe(entry_path) {
        return Err(
            io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "unsafe archive entry path: {}",
                    EscapedText::new(entry_path)
                ),
            ),
        );
    }
    let mut destination = PathBuf::from(output_root);
    for component in entry_path.split('/') {
        let mut parsed = Path::new(component).components();
        let is_normal = matches!(
            parsed.next(),
            Some(Component::Normal(_))
        ) && parsed
            .next()
            .is_none()
            && !component.contains('\\');
        if !is_normal {
            return Err(
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "unsafe archive entry path: {}",
                        EscapedText::new(entry_path)
                    ),
                ),
            );
        }
        destination.push(component);
    }
    Ok(destination)
}

/// Rejects an existing nondirectory in one destination parent chain.
fn preflight_parent_paths(
    output_root: &Path,
    destination: &Path,
) -> io::Result<()> {
    let relative = destination
        .strip_prefix(output_root)
        .map_err(
            |error| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!("destination is outside the output root: {error}"),
                )
            },
        )?;
    let mut current = PathBuf::from(output_root);
    let parent = relative
        .parent()
        .unwrap_or_else(|| Path::new(""));
    for component in parent.components() {
        current.push(component);
        match inspect_path_kind(&current)? {
            PathKind::Missing | PathKind::Directory => {}
            PathKind::File | PathKind::Other => {
                let current_text = current.to_string_lossy();
                return Err(
                    io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        format!(
                            "destination parent is not a directory: {}",
                            EscapedText::new(current_text.as_ref())
                        ),
                    ),
                );
            }
        }
    }
    Ok(())
}

/// Registers one portable file identity and all required parent directories.
fn register_portable_destination(
    entry_path: &str,
    destination: &Path,
    files: &mut BTreeSet<String>,
    directories: &mut BTreeSet<String>,
) -> io::Result<()> {
    let identity = portable_identity(entry_path);
    if files.contains(&identity) || directories.contains(&identity) {
        let destination_text = destination.to_string_lossy();
        return Err(
            io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "portable destination collision: {}",
                    EscapedText::new(destination_text.as_ref())
                ),
            ),
        );
    }
    let mut parent_identity = String::new();
    let mut components = identity
        .split('/')
        .peekable();
    while let Some(component) = components.next() {
        if components
            .peek()
            .is_none()
        {
            break;
        }
        if !parent_identity.is_empty() {
            parent_identity.push('/');
        }
        parent_identity.push_str(component);
        if files.contains(&parent_identity) {
            let destination_text = destination.to_string_lossy();
            return Err(
                io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "portable destination collision: {}",
                        EscapedText::new(destination_text.as_ref())
                    ),
                ),
            );
        }
        let _ = directories.insert(parent_identity.clone());
    }
    let _ = files.insert(identity);
    Ok(())
}

/// Resolves and validates every destination before any write begins.
pub(super) fn preflight_destinations(
    entries: &[FileEntry],
    output_root: &Path,
) -> io::Result<Vec<PathBuf>> {
    if output_root
        .as_os_str()
        .is_empty()
    {
        return Err(
            io::Error::new(
                io::ErrorKind::InvalidInput,
                "output root cannot be empty",
            ),
        );
    }
    match inspect_path_kind(output_root)? {
        PathKind::Missing | PathKind::Directory => {}
        PathKind::File | PathKind::Other => {
            let output_root_text = output_root.to_string_lossy();
            return Err(
                io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "output root is not a directory: {}",
                        EscapedText::new(output_root_text.as_ref())
                    ),
                ),
            );
        }
    }
    let mut destinations = Vec::with_capacity(entries.len());
    let mut file_destinations = BTreeSet::new();
    let mut directory_destinations = BTreeSet::new();
    for entry in entries {
        let destination = destination_path(
            output_root,
            &entry.path,
        )?;
        preflight_parent_paths(
            output_root,
            &destination,
        )?;
        register_portable_destination(
            &entry.path,
            &destination,
            &mut file_destinations,
            &mut directory_destinations,
        )?;
        match inspect_path_kind(&destination)? {
            PathKind::Missing => {}
            PathKind::File | PathKind::Directory | PathKind::Other => {
                let destination_text = destination.to_string_lossy();
                return Err(
                    io::Error::new(
                        io::ErrorKind::AlreadyExists,
                        format!(
                            "destination already exists: {}",
                            EscapedText::new(destination_text.as_ref())
                        ),
                    ),
                );
            }
        }
        destinations.push(destination);
    }
    Ok(destinations)
}
