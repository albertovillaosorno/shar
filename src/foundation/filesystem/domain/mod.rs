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
//   - Domain domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Domain domain module.
// - Description:
//   - Implements the declared domain module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Domain domain module.

use std::path::{Component, Path, PathBuf};

mod diagnostic_path;
mod path_safety;

pub use diagnostic_path::DiagnosticPath;
pub(crate) use diagnostic_path::DiagnosticText;
pub use path_safety::validate_portable_path;

/// Observable kind of one external path.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathKind {
    /// The path does not exist.
    Missing,
    /// The path is a regular file.
    File,
    /// The path is a directory.
    Directory,
    /// The path exists but is neither a regular file nor directory.
    Other,
}

/// Failure while resolving a caller-supplied path beneath a root.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootedPathError {
    /// The supplied containment root is empty.
    EmptyRoot,
    /// The supplied path does not identify a descendant beneath the root.
    Empty,
    /// The supplied path contains an absolute root or platform prefix.
    Absolute,
    /// The supplied path contains a parent-directory component.
    ParentTraversal,
    /// One path component targets a reserved host identity.
    ReservedHostAlias,
    /// One path component ends with a dot that Windows discards.
    TrailingDot,
    /// One path component ends with a space that Windows discards.
    TrailingSpace,
    /// One path component selects a Windows alternate data stream.
    AlternateDataStream,
    /// One path component contains punctuation reserved by Windows.
    ForbiddenCharacter,
    /// One path component contains a control character.
    ControlCharacter,
    /// One path component contains an invisible Unicode path modifier.
    UnicodePathModifier,
    /// One path component exceeds the portable UTF-16 unit limit.
    ComponentTooLong,
    /// One path component cannot be represented as Unicode text.
    NonUnicodeComponent,
}

impl core::fmt::Display for RootedPathError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::EmptyRoot => {
                formatter.write_str("path root must not be empty")
            },
            Self::Empty => {
                formatter.write_str("path must identify a descendant")
            },
            Self::Absolute => {
                formatter.write_str("path must be relative to its root")
            },
            Self::ParentTraversal => {
                formatter.write_str("path must not traverse above its root")
            },
            Self::ReservedHostAlias => {
                formatter.write_str("path must not target a host alias")
            },
            Self::TrailingDot => {
                formatter.write_str("path component must not end with a dot")
            },
            Self::TrailingSpace => {
                formatter.write_str("path component must not end with a space")
            },
            Self::AlternateDataStream => {
                formatter.write_str("path component must not select a stream")
            },
            Self::ForbiddenCharacter => {
                formatter.write_str("path component has reserved punctuation")
            },
            Self::ControlCharacter => {
                formatter.write_str("path component has a control character")
            },
            Self::UnicodePathModifier => {
                formatter.write_str("path component has a Unicode modifier")
            },
            Self::ComponentTooLong => {
                formatter.write_str("path component exceeds the host limit")
            },
            Self::NonUnicodeComponent => {
                formatter.write_str("path component must be valid Unicode")
            },
        }
    }
}

impl std::error::Error for RootedPathError {}

/// Validates one containment authority before descendant resolution.
///
/// # Errors
///
/// Returns [`RootedPathError`] when the root is empty, traversing, or not
/// portable.
pub(crate) fn validate_root(root: &Path) -> Result<(), RootedPathError> {
    if root.as_os_str().is_empty() {
        return Err(RootedPathError::EmptyRoot);
    }
    for component in root.components() {
        match component {
            Component::ParentDir => {
                return Err(RootedPathError::ParentTraversal);
            },
            Component::Prefix(_)
            | Component::RootDir
            | Component::CurDir
            | Component::Normal(_) => {},
        }
    }
    validate_portable_path(root)
}

/// Resolves one lexical descendant beneath an explicit root.
///
/// # Errors
///
/// Returns [`RootedPathError`] for empty, absolute, prefixed, rooted, or
/// parent paths.
pub fn resolve_under(
    root: &Path,
    relative: &Path,
) -> Result<PathBuf, RootedPathError> {
    validate_root(root)?;
    validate_portable_path(relative)?;
    let mut resolved = root.to_path_buf();
    let mut has_descendant = false;
    for component in relative.components() {
        match component {
            Component::Normal(value) => {
                has_descendant = true;
                resolved.push(value);
            },
            Component::CurDir => {},
            Component::ParentDir => {
                return Err(RootedPathError::ParentTraversal);
            },
            Component::Prefix(_) | Component::RootDir => {
                return Err(RootedPathError::Absolute);
            },
        }
    }
    if !has_descendant {
        return Err(RootedPathError::Empty);
    }
    Ok(resolved)
}

#[cfg(test)]
#[path = "../../../../tests/foundation/filesystem/unit/domain/tests.rs"]
mod tests;
