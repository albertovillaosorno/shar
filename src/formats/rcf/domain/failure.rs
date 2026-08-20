// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Closed RCF parsing and extraction failure evidence.
// - Must-Not:
//   - Retain runtime-specific error types or perform filesystem access.
// - Allows:
//   - Stable copied failure text, native path identity, and format diagnostics.
// - Split-When:
//   - Split when one failure family gains an independent consumer contract.
// - Merge-When:
//   - Merge when another domain module owns the identical error taxonomy.
// - Summary:
//   - Pure RCF failure evidence.
// - Description:
//   - Represents malformed archives, unsafe paths, and copied I/O failures.
// - Usage:
//   - Returned by RCF application services, ports, and adapters.
// - Defaults:
//   - Untrusted diagnostic text and native path units are escaped reversibly.
//

//! Pure RCF failure evidence.

use std::fmt::{Display, Formatter, Write as _};
use std::path::PathBuf;

use super::escaped_path::EscapedPath;

/// Stable domain evidence copied from one external I/O failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoFailure {
    /// Exact external failure text retained for diagnostics and inspection.
    message: String,
}

impl IoFailure {
    /// Captures one external failure without retaining its runtime type.
    #[must_use]
    pub fn new(source: impl Display) -> Self {
        Self {
            message: source.to_string(),
        }
    }

    /// Returns the exact captured external failure text.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl Display for IoFailure {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write_escaped_text(formatter, &self.message)
    }
}

impl core::error::Error for IoFailure {}

/// Closed error taxonomy for archive parsing and extraction.
#[derive(Debug)]
pub enum ArchiveError {
    /// Archive bytes are malformed or internally inconsistent.
    InvalidArchive(String),
    /// An archive entry path is unsafe for filesystem output.
    UnsafeEntryPath(String),
    /// A filesystem operation failed.
    Io {
        /// Path involved in the failed operation.
        path: PathBuf,
        /// Stable copied evidence from the external failure.
        source: IoFailure,
    },
}

impl ArchiveError {
    /// Builds an invalid archive error.
    #[must_use]
    pub fn invalid_archive(message: impl Into<String>) -> Self {
        Self::InvalidArchive(message.into())
    }

    /// Builds an unsafe path error.
    #[must_use]
    pub fn unsafe_entry_path(path: impl Into<String>) -> Self {
        Self::UnsafeEntryPath(path.into())
    }

    /// Builds an I/O error with path context and copied failure evidence.
    #[must_use]
    pub fn io(path: impl Into<PathBuf>, source: impl Display) -> Self {
        Self::Io {
            path: path.into(),
            source: IoFailure::new(source),
        }
    }

    /// Returns copied I/O failure evidence when this is an I/O error.
    #[must_use]
    pub const fn io_failure(&self) -> Option<&IoFailure> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::InvalidArchive(_) | Self::UnsafeEntryPath(_) => None,
        }
    }
}

/// Writes untrusted archive text without emitting raw controls.
fn write_escaped_text(
    formatter: &mut Formatter<'_>,
    value: &str,
) -> std::fmt::Result {
    for character in value.chars() {
        for escaped in character.escape_default() {
            formatter.write_char(escaped)?;
        }
    }
    Ok(())
}

impl Display for ArchiveError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidArchive(message) => {
                write!(formatter, "invalid RCF archive: {message}")
            },
            Self::UnsafeEntryPath(path) => {
                formatter.write_str("unsafe RCF entry path: ")?;
                write_escaped_text(formatter, path)
            },
            Self::Io { path, source } => {
                write!(
                    formatter,
                    "IO error at {}: {source}",
                    EscapedPath::new(path)
                )
            },
        }
    }
}

impl core::error::Error for ArchiveError {}

#[cfg(test)]
#[path = "../../../../tests/formats/rcf/unit/domain/failure/tests.rs"]
mod tests;
