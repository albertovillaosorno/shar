// File:
//   - error.rs
// Path:
//   - src/game-manifest/src/application/error.rs
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
//   - Typed application failures for manifest workflows.
// - Must-Not:
//   - Traverse trees, render CLI diagnostics, or select adapters.
// - Allows:
//   - Preserve operation, path, and source context across ports.
// - Split-When:
//   - Split when one error family gains an independent public contract.
// - Merge-When:
//   - Another application error module owns the same failure semantics.
// - Summary:
//   - Manifest application error model.
// - Description:
//   - Distinguishes invalid requests from external I/O failures.
// - Usage:
//   - Returned by manifest application commands.
// - Defaults:
//   - Invalid contracts fail closed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Typed failures for game-manifest application commands.
//!
//! Errors preserve stage and path context without depending on concrete
//! adapters.
use std::io;
use std::path::PathBuf;

/// Failure from an application command.
#[derive(Debug)]
pub enum ManifestError {
    /// External I/O failed for an explicit path.
    Io {
        /// Stable operation label.
        operation: &'static str,
        /// Path owned by the failed operation.
        path: PathBuf,
        /// Underlying adapter error.
        source: io::Error,
    },
    /// Request or manifest data violated a contract.
    Invalid(String),
}

impl ManifestError {
    /// Builds one path-owning I/O failure.
    #[must_use]
    pub const fn io(
        operation: &'static str,
        path: PathBuf,
        source: io::Error,
    ) -> Self {
        Self::Io {
            operation,
            path,
            source,
        }
    }
}

impl core::fmt::Display for ManifestError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Io {
                operation,
                path,
                source,
            } => {
                let rendered_path = super::diagnostic_path::escaped_path(path);
                write!(
                    formatter,
                    "{operation} {rendered_path}: {source}"
                )
            }
            Self::Invalid(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for ManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io {
                source,
                ..
            } => Some(source),
            Self::Invalid(_) => None,
        }
    }
}
