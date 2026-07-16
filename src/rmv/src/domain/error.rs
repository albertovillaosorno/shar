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
use std::path::{Path, PathBuf};

use schoenwald_filesystem::DiagnosticPath;

/// Returns untrusted diagnostic text without raw control characters.
fn escaped_text(value: &str) -> String {
    value
        .chars()
        .flat_map(char::escape_default)
        .collect()
}

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
            } => {
                let source_text = source.to_string();
                write!(
                    formatter,
                    "{}: {}",
                    DiagnosticPath::new(path),
                    escaped_text(&source_text)
                )
            }
            Self::InputRootInsideOutput(path) => {
                write!(
                    formatter,
                    "input root is inside the output tree: {}",
                    DiagnosticPath::new(path)
                )
            }
            Self::InvalidRootName(path) => {
                write!(
                    formatter,
                    "input root has no safe folder name: {}",
                    DiagnosticPath::new(path)
                )
            }
            Self::InvalidPath(path) => {
                write!(
                    formatter,
                    "path is not safe for export: {}",
                    DiagnosticPath::new(path)
                )
            }
            Self::InvalidMovieStem(stem) => {
                write!(
                    formatter,
                    "movie stem is not a single safe path component: {}",
                    DiagnosticPath::new(Path::new(stem))
                )
            }
            Self::OutputPathCollision(path) => {
                write!(
                    formatter,
                    "multiple RMV inputs map to the same output path: {}",
                    DiagnosticPath::new(path)
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

#[cfg(test)]
mod tests {
    use std::error::Error as _;
    #[cfg(windows)]
    use std::ffi::OsString;
    use std::io;
    #[cfg(windows)]
    use std::os::windows::ffi::OsStringExt as _;
    #[cfg(windows)]
    use std::path::PathBuf;

    use super::RmvError;

    #[test]
    fn io_error_escapes_source_control_characters() {
        let error = RmvError::Io {
            path: PathBuf::from("movie.rmv"),
            source: io::Error::other("read\nfailure"),
        };

        let rendered = error.to_string();

        assert!(
            !rendered
                .chars()
                .any(char::is_control),
            "diagnostic contains a control character: {rendered:?}"
        );
        assert!(rendered.contains(r"read\nfailure"));
        assert!(
            error
                .source()
                .is_some()
        );
    }

    #[test]
    fn invalid_movie_stem_error_escapes_control_characters() {
        let error = RmvError::InvalidMovieStem("bad\nstem".to_owned());

        let rendered = error.to_string();

        assert!(
            !rendered
                .chars()
                .any(char::is_control),
            "diagnostic contains a control character: {rendered:?}"
        );
        assert!(rendered.contains(r"bad\nstem"));
    }

    #[cfg(windows)]
    #[test]
    fn path_errors_preserve_unpaired_utf16_unit() {
        let path = PathBuf::from(
            OsString::from_wide(
                &[
                    u16::from(b'a'),
                    0xd800,
                    u16::from(b'b'),
                ],
            ),
        );
        let errors = [
            RmvError::Io {
                path: path.clone(),
                source: io::Error::other("read failure"),
            },
            RmvError::InputRootInsideOutput(path.clone()),
            RmvError::InvalidRootName(path.clone()),
            RmvError::InvalidPath(path.clone()),
            RmvError::OutputPathCollision(path),
        ];

        for error in errors {
            let rendered = error.to_string();
            assert!(
                rendered.contains(r"a\u{D800}b"),
                "diagnostic lost the native path unit: {rendered:?}"
            );
            assert!(!rendered.contains('\u{fffd}'));
        }
    }
}
