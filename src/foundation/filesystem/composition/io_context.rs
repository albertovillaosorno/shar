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
//   - Io context outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Io context outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Io context outbound adapter.

#![expect(
    clippy::redundant_pub_crate,
    reason = "crate-root private module shares helpers with sibling modules"
)]

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
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ContextualIoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Adds stable adapter context while retaining the native error category.
pub(crate) fn with_path(
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
    io::Error::new(kind, ContextualIoError { message, source })
}

/// Creates one contextual invalid-input failure without a native source.
pub(crate) fn invalid_input(
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
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../tests/foundation/filesystem/unit/adapter-outbound/io_context/tests.rs"]
mod tests;
