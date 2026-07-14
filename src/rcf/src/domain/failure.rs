// File:
//   - failure.rs
// Path:
//   - src/rcf/src/domain/failure.rs
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
//   - Pure rcf domain rules for domain failure.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when failure contains two independently testable contracts.
// - Merge-When:
//   - Another rcf module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Error values for RCF archive parsing and extraction.
// - Description:
//   - Defines failure data and behavior for rcf domain.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Error values for RCF archive parsing and extraction.
use std::fmt::{Display, Formatter};
use std::io;
use std::path::PathBuf;

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
        /// Original IO error.
        source: io::Error,
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

    /// Builds an IO error with path context.
    #[must_use]
    pub fn io(
        path: impl Into<PathBuf>,
        source: io::Error,
    ) -> Self {
        Self::Io {
            path: path.into(),
            source,
        }
    }
}

impl Display for ArchiveError {
    fn fmt(
        &self,
        formatter: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::InvalidArchive(message) => write!(
                formatter,
                "invalid RCF archive: {message}"
            ),
            Self::UnsafeEntryPath(path) => write!(
                formatter,
                "unsafe RCF entry path: {path}"
            ),
            Self::Io {
                path,
                source,
            } => {
                write!(
                    formatter,
                    "IO error at {}: {source}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for ArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io {
                source,
                ..
            } => Some(source),
            Self::InvalidArchive(_) | Self::UnsafeEntryPath(_) => None,
        }
    }
}
