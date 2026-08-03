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
//   - Path validation application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Path validation application service.
// - Description:
//   - Implements the declared responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Path validation application service.

#![expect(
    clippy::redundant_pub_crate,
    reason = "crate-root private module shares helpers with sibling modules"
)]

use std::path::{Component, Path};
use std::string::FromUtf8Error;
use std::{fmt, io};

use crate::domain::{
    DiagnosticPath, DiagnosticText, resolve_under, validate_portable_path,
    validate_root,
};

/// Context retained around one typed application failure.
#[derive(Debug)]
struct ContextualPathError<E> {
    /// Rendered operation, path, and application failure details.
    message: String,
    /// Original typed application failure retained for downcasting.
    source: E,
}

impl<E> fmt::Display for ContextualPathError<E> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl<E> std::error::Error for ContextualPathError<E>
where
    E: std::error::Error + 'static,
{
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Creates one application error with stable operation and path context.
pub(crate) fn path_error(
    kind: io::ErrorKind,
    operation: &'static str,
    path: &Path,
    message: impl fmt::Display,
) -> io::Error {
    io::Error::new(
        kind,
        format!(
            "{operation} `{}` failed: {message}",
            DiagnosticPath::new(path)
        ),
    )
}

/// Creates one contextual application failure retaining its typed source.
fn path_source_error<E>(
    kind: io::ErrorKind,
    operation: &'static str,
    path: &Path,
    source: E,
) -> io::Error
where
    E: std::error::Error + Send + Sync + 'static,
{
    let source_text = source.to_string();
    let message = format!(
        "{operation} `{}` failed: {}",
        DiagnosticPath::new(path),
        DiagnosticText::new(&source_text)
    );
    io::Error::new(kind, ContextualPathError { message, source })
}

/// Adds file and operation context to one UTF-8 decoding failure.
pub(crate) fn utf8_error(path: &Path, source: FromUtf8Error) -> io::Error {
    path_source_error(
        io::ErrorKind::InvalidData,
        "decode UTF-8 file",
        path,
        source,
    )
}

/// Rejects omitted caller paths before provider access or state mapping.
///
/// # Errors
///
/// Returns invalid input when the supplied path is empty or nonportable.
pub(crate) fn require_explicit_path(
    path: &Path,
    operation: &'static str,
) -> io::Result<()> {
    if path.as_os_str().is_empty() {
        return Err(path_error(
            io::ErrorKind::InvalidInput,
            operation,
            path,
            "filesystem path must not be empty",
        ));
    }
    validate_portable_path(path).map_err(|error| {
        path_source_error(io::ErrorKind::InvalidInput, operation, path, error)
    })
}

/// Validates one tree root before the driven port can observe it.
///
/// # Errors
///
/// Returns invalid input when the root is empty, traversing, or nonportable.
pub(crate) fn require_tree_root(root: &Path) -> io::Result<()> {
    validate_root(root).map_err(|error| {
        path_source_error(
            io::ErrorKind::InvalidInput,
            "collect regular files",
            root,
            error,
        )
    })
}

/// Reports whether one path names an ordinary filesystem component.
pub(crate) fn has_meaningful_component(path: &Path) -> bool {
    for component in path.components() {
        match component {
            Component::Normal(_) => return true,
            Component::Prefix(_)
            | Component::RootDir
            | Component::CurDir
            | Component::ParentDir => {},
        }
    }
    false
}

/// Reports whether one byte is a supported lexical path separator.
const fn is_path_separator(byte: u8) -> bool {
    byte == b'/' || byte == 92
}

/// Reports whether one lexical path ends in a current-directory marker.
fn ends_with_current_marker(path: &Path) -> bool {
    let mut bytes = path.as_os_str().as_encoded_bytes().iter().rev();
    while let Some(byte) = bytes.next() {
        if is_path_separator(*byte) {
            continue;
        }
        if *byte != b'.' {
            return false;
        }
        let previous = bytes.next().copied();
        return previous.is_none_or(is_path_separator);
    }
    false
}

/// Reports whether one operation ends at a named component.
pub(crate) fn has_named_destination(path: &Path) -> bool {
    if ends_with_current_marker(path) {
        return false;
    }
    matches!(path.components().next_back(), Some(Component::Normal(_)))
}

/// Reports whether one write request ends in explicit file syntax.
pub(crate) fn has_file_destination(path: &Path) -> bool {
    let path_text = path.to_str();
    let Some(text) = path_text else {
        return false;
    };
    let trailing_slash = text.ends_with('/');
    let trailing_backslash = text.ends_with(char::from(92));
    if trailing_slash || trailing_backslash {
        return false;
    }
    let slash = text.rfind('/');
    let backslash = text.rfind(char::from(92));
    let start = slash
        .max(backslash)
        .map_or(0, |index| index.saturating_add(1));
    let segment = text.get(start..).unwrap_or_default();
    !matches!(segment, "" | "." | "..")
}

/// Rejects adapter paths that are not normalized descendants of the request.
///
/// # Errors
///
/// Returns invalid data when one adapter path escapes or aliases its root.
pub(crate) fn require_tree_descendant(
    root: &Path,
    path: &Path,
) -> io::Result<String> {
    let relative = path.strip_prefix(root).map_err(|error| {
        path_source_error(
            io::ErrorKind::InvalidData,
            "validate tree entry",
            path,
            error,
        )
    })?;
    let resolved = resolve_under(root, relative).map_err(|error| {
        path_source_error(
            io::ErrorKind::InvalidData,
            "validate tree entry",
            path,
            error,
        )
    })?;
    if resolved != path {
        return Err(path_error(
            io::ErrorKind::InvalidData,
            "validate tree entry",
            path,
            "tree reader returned a non-normalized path",
        ));
    }
    let mut identity = String::new();
    for component in relative.components() {
        let Component::Normal(value) = component else {
            continue;
        };
        if !identity.is_empty() {
            identity.push('/');
        }
        let name = value.to_string_lossy();
        for character in name.chars() {
            for uppercase in character.to_uppercase() {
                identity.push(uppercase);
            }
        }
    }
    Ok(identity)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../tests/foundation/filesystem/unit/application/path_validation/tests.rs"]
mod tests;
