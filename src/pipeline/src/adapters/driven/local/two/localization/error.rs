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

use schoenwald_filesystem::DiagnosticPath;

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

/// Returns untrusted diagnostic text without raw control characters.
fn escaped_diagnostic_text(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        if character.is_control() {
            output.extend(character.escape_default());
        } else {
            output.push(character);
        }
    }
    output
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
                let source_text = source.to_string();
                let rendered_source = escaped_diagnostic_text(&source_text);
                write!(
                    formatter,
                    "{}: {rendered_source}",
                    DiagnosticPath::new(path)
                )
            }
            Self::InvalidSource(message) => {
                let rendered_message = escaped_diagnostic_text(message);
                formatter.write_str(&rendered_message)
            }
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io {
                source,
                ..
            } => Some(source),
            Self::InvalidSource(_) => None,
        }
    }
}

/// Result shared by localization source operations.
pub(super) type Outcome<T> = Result<T, Error>;

#[cfg(test)]
mod tests {
    #[test]
    fn io_error_escapes_source_controls_and_preserves_chain() {
        let error = super::Error::io(
            std::path::PathBuf::from("language.bin"),
            std::io::Error::other("read\ninjected"),
        );

        assert_eq!(
            error.to_string(),
            r"language.bin: read\ninjected"
        );
        assert!(std::error::Error::source(&error).is_some());
    }

    #[test]
    fn invalid_source_escapes_control_characters() {
        let error = super::Error::invalid("invalid\nsource");

        assert_eq!(
            error.to_string(),
            r"invalid\nsource"
        );
    }

    #[cfg(windows)]
    #[test]
    fn io_error_preserves_unpaired_utf16_path_unit() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt as _;

        let path = std::path::PathBuf::from(
            OsString::from_wide(
                &[
                    u16::from(b'a'),
                    0xd800_u16,
                    u16::from(b'b'),
                ],
            ),
        );
        let error = super::Error::io(
            path,
            std::io::Error::other("read failure"),
        );

        assert_eq!(
            error.to_string(),
            r"a\u{D800}b: read failure"
        );
    }
}
