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
//   - Error domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Error domain module.
// - Description:
//   - Implements the declared domain module responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Error domain module.

use std::path::{Path, PathBuf};

use super::escaped_path::EscapedPath;

/// Returns untrusted diagnostic text without raw control characters.
fn escaped_text(value: &str) -> String {
    value.chars().flat_map(char::escape_default).collect()
}

/// Stable domain evidence copied from one external I/O failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoFailure {
    /// Exact external failure text retained for diagnostics and inspection.
    message: String,
}

impl IoFailure {
    /// Captures one external failure without retaining its runtime type.
    #[must_use]
    pub fn new(source: impl core::fmt::Display) -> Self {
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

impl core::fmt::Display for IoFailure {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        formatter.write_str(&escaped_text(&self.message))
    }
}

impl core::error::Error for IoFailure {}

/// Failure while planning or materializing one RMV audit.
#[derive(Debug)]
pub enum RmvError {
    /// One external I/O operation failed at an explicit path.
    Io {
        /// Path associated with the failed operation.
        path: PathBuf,
        /// Stable copied evidence from the external failure.
        source: IoFailure,
    },
    /// The input root resolves to the output tree or one of its descendants.
    InputRootInsideOutput(PathBuf),
    /// The input root has no safe portable folder name.
    InvalidRootName(PathBuf),
    /// One path cannot be represented safely in the export tree.
    InvalidPath(PathBuf),
    /// One movie stem is not a single safe path component.
    InvalidMovieStem(String),
    /// Multiple RMV inputs map to the same output path.
    OutputPathCollision(PathBuf),
    /// No input roots were supplied.
    NoInputRoots,
    /// No RMV movie inputs were discovered.
    NoMovieInputs,
}

impl RmvError {
    /// Captures one external I/O failure as stable domain evidence.
    #[must_use]
    pub fn io(path: PathBuf, source: impl core::fmt::Display) -> Self {
        Self::Io {
            path,
            source: IoFailure::new(source),
        }
    }

    /// Returns copied I/O failure evidence when this is an I/O error.
    #[must_use]
    pub const fn io_failure(&self) -> Option<&IoFailure> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::InputRootInsideOutput(_)
            | Self::InvalidRootName(_)
            | Self::InvalidPath(_)
            | Self::InvalidMovieStem(_)
            | Self::OutputPathCollision(_)
            | Self::NoInputRoots
            | Self::NoMovieInputs => None,
        }
    }
}

impl core::fmt::Display for RmvError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Io { path, source } => write!(
                formatter,
                "{}: {}",
                EscapedPath::new(path),
                escaped_text(source.message())
            ),
            Self::InputRootInsideOutput(path) => write!(
                formatter,
                "input root is inside the output tree: {}",
                EscapedPath::new(path)
            ),
            Self::InvalidRootName(path) => write!(
                formatter,
                "input root has no safe folder name: {}",
                EscapedPath::new(path)
            ),
            Self::InvalidPath(path) => write!(
                formatter,
                "path is not safe for export: {}",
                EscapedPath::new(path)
            ),
            Self::InvalidMovieStem(stem) => write!(
                formatter,
                "movie stem is not a single safe path component: {}",
                EscapedPath::new(Path::new(stem))
            ),
            Self::OutputPathCollision(path) => write!(
                formatter,
                "multiple RMV inputs map to the same output path: {}",
                EscapedPath::new(path)
            ),
            Self::NoInputRoots => {
                formatter.write_str("at least one input root is required")
            },
            Self::NoMovieInputs => {
                formatter.write_str("no .rmv movie inputs were found")
            },
        }
    }
}

impl core::error::Error for RmvError {}

#[cfg(test)]
#[path = "../../../../tests/formats/rmv/unit/domain/error/tests.rs"]
mod tests;
