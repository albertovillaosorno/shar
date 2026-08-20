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
//   - Error application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Error application service.
// - Description:
//   - Implements the declared application service responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Error application service.

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
        Self::Io { operation, path, source }
    }
}

impl core::fmt::Display for ManifestError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Io { operation, path, source } => {
                let rendered_path = super::diagnostic_path::escaped_path(path);
                let source_text = source.to_string();
                let rendered_source =
                    super::diagnostic_path::escaped_text(&source_text);
                write!(
                    formatter,
                    "{operation} {rendered_path}: {rendered_source}"
                )
            },
            Self::Invalid(message) => {
                let rendered_message =
                    super::diagnostic_path::escaped_text(message);
                formatter.write_str(&rendered_message)
            },
        }
    }
}

impl std::error::Error for ManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Invalid(_) => None,
        }
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/migration/manifest/unit/application/error/tests.rs"]
mod tests;
