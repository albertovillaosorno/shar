// File:
//   - error.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/localization/error.rs
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
//   - Typed failures for localization source normalization.
// - Must-Not:
//   - Flatten decoder or filesystem failures into successful empty records.
// - Allows:
//   - Path-aware IO errors and source-contract diagnostics.
// - Split-When:
//   - A new error family requires structured fields beyond source
//   - normalization.
// - Merge-When:
//   - Another localization error type carries the same caller-visible
//   - context.
// - Summary:
//   - Typed localization normalization failures.
// - Description:
//   - Preserves enough context for pipeline callers to reject corrupt
//   - sources.
// - Usage:
//   - Returned by every localization parser and overlay operation.
// - Defaults:
//   - Malformed source data fails closed without implicit output.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Typed localization failures preserve source context for fail-closed pipeline
//! callers.

use std::path::PathBuf;

/// Errors returned by localization parsing and merge boundaries.
#[derive(Debug)]
pub(super) enum Error {
    /// Filesystem access failed for a specific source.
    Io {
        /// Source involved in the failed operation.
        path: PathBuf,
        /// Original filesystem failure.
        source: std::io::Error,
    },
    /// Input bytes violated the declared localization format.
    InvalidSource(String),
}

impl Error {
    /// Preserve a source path beside its filesystem failure.
    #[must_use]
    pub(super) const fn io(
        path: PathBuf,
        source: std::io::Error,
    ) -> Self {
        Self::Io {
            path,
            source,
        }
    }

    /// Create a fail-closed source-contract error.
    #[must_use]
    pub(super) fn invalid(message: impl Into<String>) -> Self {
        Self::InvalidSource(message.into())
    }
}

impl std::fmt::Display for Error {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Io {
                path,
                source,
            } => {
                write!(
                    formatter,
                    "{}: {source}",
                    path.display()
                )
            }
            Self::InvalidSource(message) => formatter.write_str(message),
        }
    }
}

impl std::error::Error for Error {}

/// Result shared by localization source operations.
pub(super) type Outcome<T> = Result<T, Error>;
