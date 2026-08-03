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
//   - Inspection outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Inspection outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Inspection outbound adapter.

use std::path::Path;
use std::{fmt, io};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;

/// Returns provider text without raw controls or a second escape layer.
fn escaped_provider_text(value: &str) -> String {
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

/// Escapes one provider failure while retaining its native error chain.
#[derive(Debug)]
struct EscapedProviderError {
    /// Escaped single-line public diagnostic text.
    message: String,
    /// Original provider failure retained for error-chain inspection.
    source: io::Error,
}

impl fmt::Display for EscapedProviderError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    local::path_kind(path).map_err(|source| {
        let kind = source.kind();
        let source_text = source.to_string();
        let message = escaped_provider_text(&source_text);
        io::Error::new(kind, EscapedProviderError { message, source })
    })
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../../tests/formats/lmlm/unit/adapter-outbound/filesystem_entry_sink/inspection/tests.rs"]
mod tests;
