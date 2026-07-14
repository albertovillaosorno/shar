// File:
//   - error.rs
// Path:
//   - src/rmv/src/domain/error.rs
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
//   - Pure rmv domain rules for domain error.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when error contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Error variants for RMV/Bink audit and conversion gates.
// - Description:
//   - Defines error data and behavior for rmv domain.
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

//! Error variants for RMV/Bink audit and conversion gates.
use std::path::PathBuf;

#[derive(Debug)]
/// Rmverror.
pub enum RmvError {
    /// Item.
    Io {
        /// Path.
        path: PathBuf,
        /// Source.
        source: std::io::Error,
    },
    /// The input root resolves to the output tree or one of its descendants.
    InputRootInsideOutput(PathBuf),
    /// Item.
    InvalidRootName(PathBuf),
    /// Item.
    InvalidPath(PathBuf),
    /// Item.
    InvalidMovieStem(String),
    /// Item.
    OutputPathCollision(PathBuf),
    /// Item.
    NoInputRoots,
    /// Item.
    NoMovieInputs,
}

impl core::fmt::Display for RmvError {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        match self {
            Self::Io {
                path,
                source,
            } => write!(
                formatter,
                "{}: {source}",
                path.display()
            ),
            Self::InputRootInsideOutput(path) => {
                write!(
                    formatter,
                    "input root is inside the output tree: {}",
                    path.display()
                )
            }
            Self::InvalidRootName(path) => {
                write!(
                    formatter,
                    "input root has no safe folder name: {}",
                    path.display()
                )
            }
            Self::InvalidPath(path) => {
                write!(
                    formatter,
                    "path is not safe for export: {}",
                    path.display()
                )
            }
            Self::InvalidMovieStem(stem) => {
                write!(
                    formatter,
                    "movie stem is not a single safe path component: {stem}"
                )
            }
            Self::OutputPathCollision(path) => {
                write!(
                    formatter,
                    "multiple RMV inputs map to the same output path: {}",
                    path.display()
                )
            }
            Self::NoInputRoots => write!(
                formatter,
                "at least one input root is required"
            ),
            Self::NoMovieInputs => write!(
                formatter,
                "no .rmv movie inputs were found"
            ),
        }
    }
}

impl std::error::Error for RmvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io {
                source,
                ..
            } => Some(source),
            _ => None,
        }
    }
}
