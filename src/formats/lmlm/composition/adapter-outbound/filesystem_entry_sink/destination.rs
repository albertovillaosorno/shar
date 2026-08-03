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
//   - Destination outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Destination outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Destination outbound adapter.

use std::collections::BTreeSet;
use std::io;
use std::path::{Component, Path, PathBuf};

use schoenwald_filesystem::{DiagnosticPath, PathKind};

use super::inspection::inspect_path_kind;
use crate::domain::diagnostic::EscapedText;
use crate::domain::{FileEntry, portable_identity, portable_path_is_safe};

/// Builds one local destination from a validated archive path.
fn destination_path(
    output_root: &Path,
    entry_path: &str,
) -> io::Result<PathBuf> {
    if !portable_path_is_safe(entry_path) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "unsafe archive entry path: {}",
                EscapedText::new(entry_path)
            ),
        ));
    }
    let mut destination = PathBuf::from(output_root);
    for component in entry_path.split('/') {
        let mut parsed = Path::new(component).components();
        let is_normal = matches!(parsed.next(), Some(Component::Normal(_)))
            && parsed.next().is_none()
            && !component.contains('\\');
        if !is_normal {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!(
                    "unsafe archive entry path: {}",
                    EscapedText::new(entry_path)
                ),
            ));
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
    let relative = destination.strip_prefix(output_root).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("destination is outside the output root: {error}"),
        )
    })?;
    let mut current = PathBuf::from(output_root);
    let parent = relative.parent().unwrap_or_else(|| Path::new(""));
    for component in parent.components() {
        current.push(component);
        match inspect_path_kind(&current)? {
            PathKind::Missing | PathKind::Directory => {},
            PathKind::File | PathKind::Other => {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "destination parent is not a directory: {}",
                        DiagnosticPath::new(&current)
                    ),
                ));
            },
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
        return Err(io::Error::new(
            io::ErrorKind::AlreadyExists,
            format!(
                "portable destination collision: {}",
                DiagnosticPath::new(destination)
            ),
        ));
    }
    let mut parent_identity = String::new();
    let mut components = identity.split('/').peekable();
    while let Some(component) = components.next() {
        if components.peek().is_none() {
            break;
        }
        if !parent_identity.is_empty() {
            parent_identity.push('/');
        }
        parent_identity.push_str(component);
        if files.contains(&parent_identity) {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "portable destination collision: {}",
                    DiagnosticPath::new(destination)
                ),
            ));
        }
        let _inserted: bool = directories.insert(parent_identity.clone());
    }
    let _inserted: bool = files.insert(identity);
    Ok(())
}

/// Resolves and validates every destination before any write begins.
pub(super) fn preflight_destinations(
    entries: &[FileEntry],
    output_root: &Path,
) -> io::Result<Vec<PathBuf>> {
    if output_root.as_os_str().is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "output root cannot be empty",
        ));
    }
    match inspect_path_kind(output_root)? {
        PathKind::Missing | PathKind::Directory => {},
        PathKind::File | PathKind::Other => {
            return Err(io::Error::new(
                io::ErrorKind::AlreadyExists,
                format!(
                    "output root is not a directory: {}",
                    DiagnosticPath::new(output_root)
                ),
            ));
        },
    }
    let mut destinations = Vec::with_capacity(entries.len());
    let mut file_destinations = BTreeSet::new();
    let mut directory_destinations = BTreeSet::new();
    for entry in entries {
        let destination = destination_path(output_root, &entry.path)?;
        preflight_parent_paths(output_root, &destination)?;
        register_portable_destination(
            &entry.path,
            &destination,
            &mut file_destinations,
            &mut directory_destinations,
        )?;
        match inspect_path_kind(&destination)? {
            PathKind::Missing => {},
            PathKind::File | PathKind::Directory | PathKind::Other => {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    format!(
                        "destination already exists: {}",
                        DiagnosticPath::new(&destination)
                    ),
                ));
            },
        }
        destinations.push(destination);
    }
    Ok(destinations)
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../../tests/formats/lmlm/unit/adapter-outbound/filesystem_entry_sink/destination/tests.rs"]
mod tests;
