// File:
//   - path_validation.rs
// Path:
//   - src/filesystem/src/application/path_validation.rs
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
//   - Application path validation and contextual path failures.
// - Must-Not:
//   - Perform filesystem IO, select providers, or encode caller domains.
// - Allows:
//   - Validate explicit paths and adapter-returned tree descendants.
// - Split-When:
//   - Split when one path-validation family becomes independently versioned.
// - Merge-When:
//   - Another application module owns the same path validation contracts.
// - Summary:
//   - Filesystem application path validation.
// - Description:
//   - Adds stable operation and path context to application-owned failures.
// - Usage:
//   - Called by filesystem application use cases before or after port access.
// - Defaults:
//   - Portable paths and normalized descendants are required.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Application-owned path validation and diagnostics.
//!
//! Provider-independent failures retain operation and path context.
use std::path::{Component, Path};
use std::string::FromUtf8Error;
use std::{fmt, io};

use super::diagnostic_path::DiagnosticPath;
use crate::domain::{resolve_under, validate_portable_path, validate_root};

/// Context retained around one typed application failure.
#[derive(Debug)]
struct ContextualPathError<E> {
    /// Rendered operation, path, and application failure details.
    message: String,
    /// Original typed application failure retained for downcasting.
    source: E,
}

impl<E> fmt::Display for ContextualPathError<E> {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
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
pub(super) fn path_error(
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
    let message = format!(
        "{operation} `{}` failed: {source}",
        DiagnosticPath::new(path)
    );
    io::Error::new(
        kind,
        ContextualPathError {
            message,
            source,
        },
    )
}

/// Adds file and operation context to one UTF-8 decoding failure.
pub(super) fn utf8_error(
    path: &Path,
    source: FromUtf8Error,
) -> io::Error {
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
pub(super) fn require_explicit_path(
    path: &Path,
    operation: &'static str,
) -> io::Result<()> {
    if path
        .as_os_str()
        .is_empty()
    {
        return Err(
            path_error(
                io::ErrorKind::InvalidInput,
                operation,
                path,
                "filesystem path must not be empty",
            ),
        );
    }
    validate_portable_path(path).map_err(
        |error| {
            path_source_error(
                io::ErrorKind::InvalidInput,
                operation,
                path,
                error,
            )
        },
    )
}

/// Validates one tree root before the driven port can observe it.
///
/// # Errors
///
/// Returns invalid input when the root is empty, traversing, or nonportable.
pub(super) fn require_tree_root(root: &Path) -> io::Result<()> {
    validate_root(root).map_err(
        |error| {
            path_source_error(
                io::ErrorKind::InvalidInput,
                "collect regular files",
                root,
                error,
            )
        },
    )
}

/// Reports whether one path names an ordinary filesystem component.
pub(super) fn has_meaningful_component(path: &Path) -> bool {
    for component in path.components() {
        match component {
            Component::Normal(_) => return true,
            Component::Prefix(_)
            | Component::RootDir
            | Component::CurDir
            | Component::ParentDir => {}
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
    let mut bytes = path
        .as_os_str()
        .as_encoded_bytes()
        .iter()
        .rev();
    while let Some(byte) = bytes.next() {
        if is_path_separator(*byte) {
            continue;
        }
        if *byte != b'.' {
            return false;
        }
        let previous = bytes
            .next()
            .copied();
        return previous.is_none_or(is_path_separator);
    }
    false
}

/// Reports whether one operation ends at a named component.
pub(super) fn has_named_destination(path: &Path) -> bool {
    if ends_with_current_marker(path) {
        return false;
    }
    matches!(
        path.components()
            .next_back(),
        Some(Component::Normal(_))
    )
}

/// Reports whether one write request ends in explicit file syntax.
pub(super) fn has_file_destination(path: &Path) -> bool {
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
        .map_or(
            0,
            |index| index.saturating_add(1),
        );
    let segment = text
        .get(start..)
        .unwrap_or_default();
    !matches!(
        segment,
        "" | "." | ".."
    )
}

/// Rejects adapter paths that are not normalized descendants of the request.
///
/// # Errors
///
/// Returns invalid data when one adapter path escapes or aliases its root.
pub(super) fn require_tree_descendant(
    root: &Path,
    path: &Path,
) -> io::Result<String> {
    let relative = path
        .strip_prefix(root)
        .map_err(
            |error| {
                path_source_error(
                    io::ErrorKind::InvalidData,
                    "validate tree entry",
                    path,
                    error,
                )
            },
        )?;
    let resolved = resolve_under(
        root, relative,
    )
    .map_err(
        |error| {
            path_source_error(
                io::ErrorKind::InvalidData,
                "validate tree entry",
                path,
                error,
            )
        },
    )?;
    if resolved != path {
        return Err(
            path_error(
                io::ErrorKind::InvalidData,
                "validate tree entry",
                path,
                "tree reader returned a non-normalized path",
            ),
        );
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
