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
//   - Error outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Error outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Error outbound adapter.

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
    pub(super) const fn io(path: PathBuf, source: std::io::Error) -> Self {
        Self::Io { path, source }
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
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io { path, source } => {
                let source_text = source.to_string();
                let rendered_source = escaped_diagnostic_text(&source_text);
                write!(
                    formatter,
                    "{}: {rendered_source}",
                    DiagnosticPath::new(path)
                )
            },
            Self::InvalidSource(message) => {
                let rendered_message = escaped_diagnostic_text(message);
                formatter.write_str(&rendered_message)
            },
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::InvalidSource(_) => None,
        }
    }
}

/// Result shared by localization source operations.
pub(super) type Outcome<T> = Result<T, Error>;

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/localization/error/tests.rs"]
mod tests;
