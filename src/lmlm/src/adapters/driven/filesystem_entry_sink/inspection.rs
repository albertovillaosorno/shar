// File:
//   - inspection.rs
// Path:
//   - src/lmlm/src/adapters/driven/filesystem_entry_sink/inspection.rs
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
//   - Shared-filesystem path inspection for LMLM destination preflight.
// - Must-Not:
//   - Apply destination policy or mutate filesystem state.
// - Allows:
//   - Preserve provider error chains while escaping public diagnostics.
// - Split-When:
//   - Another inspection provider needs independent composition.
// - Merge-When:
//   - Destination preflight no longer inspects existing paths.
// - Summary:
//   - Adapts shared path inspection to LMLM diagnostics.
// - Description:
//   - Preserves native error categories, sources, and escaped text.
// - Usage:
//   - Called by destination preflight before publication begins.
// - Defaults:
//   - Missing paths return `PathKind::Missing`.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Shared path inspection for LMLM destination preflight.
//!
//! Provider failures retain their source chain and safe public display.

use std::path::Path;
use std::{fmt, io};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;

use crate::diagnostic::escaped_string;

/// Escapes one provider failure while retaining its native error chain.
#[derive(Debug)]
struct EscapedProviderError {
    /// Escaped single-line public diagnostic text.
    message: String,
    /// Original provider failure retained for error-chain inspection.
    source: io::Error,
}

impl fmt::Display for EscapedProviderError {
    fn fmt(
        &self,
        formatter: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for EscapedProviderError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// Inspects one path through the shared filesystem adapter.
pub(super) fn inspect_path_kind(path: &Path) -> io::Result<PathKind> {
    local::path_kind(path).map_err(
        |source| {
            let kind = source.kind();
            let source_text = source.to_string();
            let message = escaped_string(&source_text);
            io::Error::new(
                kind,
                EscapedProviderError {
                    message,
                    source,
                },
            )
        },
    )
}
