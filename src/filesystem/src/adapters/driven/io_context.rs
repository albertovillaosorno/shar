// File:
//   - io_context.rs
// Path:
//   - src/filesystem/src/adapters/driven/io_context.rs
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
//   - Stable operation and path context around native IO errors.
// - Must-Not:
//   - Perform filesystem access or select recovery policy.
// - Allows:
//   - Preserve error kind and source while adding adapter diagnostics.
// - Split-When:
//   - Split when another provider has an independent diagnostic format.
// - Merge-When:
//   - Another driven module owns the same IO context mechanism.
// - Summary:
//   - Standard adapter IO error context.
// - Description:
//   - Wraps native errors without discarding their category or source.
// - Usage:
//   - Called by standard filesystem adapter operations.
// - Defaults:
//   - Paths use the shared reversible diagnostic renderer.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Stable operation and path context for native IO failures.
//!
//! Native error categories and source chains remain available.
use std::path::Path;
use std::{fmt, io};

use crate::domain::{DiagnosticPath, DiagnosticText};

/// Context retained around one native filesystem failure.
#[derive(Debug)]
struct ContextualIoError {
    /// Rendered operation, path, and native failure details.
    message: String,
    /// Original native IO failure retained for source inspection.
    source: io::Error,
}

impl fmt::Display for ContextualIoError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ContextualIoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Adds stable adapter context while retaining the native error category.
pub(super) fn with_path(
    operation: &'static str,
    path: &Path,
    source: io::Error,
) -> io::Error {
    let kind = source.kind();
    let source_text = source.to_string();
    let message = format!(
        "{operation} `{}` failed: {}",
        DiagnosticPath::new(path),
        DiagnosticText::new(&source_text)
    );
    io::Error::new(
        kind,
        ContextualIoError {
            message,
            source,
        },
    )
}

/// Creates one contextual invalid-input failure without a native source.
pub(super) fn invalid_input(
    operation: &'static str,
    path: &Path,
    message: &'static str,
) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
            "{operation} `{}` failed: {message}",
            DiagnosticPath::new(path)
        ),
    )
}

#[cfg(test)]
mod tests {
    use std::error::Error as _;
    use std::io;
    use std::path::Path;

    use super::with_path;

    #[test]
    fn native_source_message_escapes_control_characters() {
        let error = with_path(
            "read file",
            Path::new("file.bin"),
            io::Error::other("source\nfailure"),
        );

        let rendered = error.to_string();

        assert!(
            !rendered
                .chars()
                .any(char::is_control),
            "diagnostic contains a control character: {rendered:?}"
        );
        assert!(rendered.contains(r"source\nfailure"));
        assert!(
            error
                .source()
                .is_some()
        );
    }
}
