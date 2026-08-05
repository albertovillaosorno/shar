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
//   - Public-safe optional-mod preview result.
// - Must-Not:
//   - Expose local package paths or package payloads.
// - Allows:
//   - Canonical rendered preview evidence and aggregate counts.
// - Split-When:
//   - Preview evidence gains another independently versioned representation.
// - Merge-When:
//   - Another domain value owns the same preview contract.
// - Summary:
//   - Carries deterministic dry-run evidence for supported optional packages.
// - Description:
//   - Preserves one public-safe JSON preview and its checked aggregate counts.
// - Usage:
//   - Returned by the optional-mod preview application use case.
// - Defaults:
//   - The rendered preview contains no machine-local paths.
//

//! Public-safe optional-mod preview result.

/// Versioned schema for the canonical optional-mod dry-run document.
pub const OPTIONAL_MOD_PREVIEW_SCHEMA: &str =
    "shar-schoenwald.optional-mod-preview.v1";

/// Deterministic dry-run result for the supported local package set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalModPreview {
    /// Canonical single-document JSON representation.
    json: String,
    /// Number of supported packages inspected.
    packages: usize,
    /// Number of members that would write output.
    writes: usize,
    /// Number of members that would be skipped by policy.
    skips: usize,
    /// Total normalized bytes that would be written.
    normalized_bytes: u64,
}

impl OptionalModPreview {
    /// Creates one validated preview result.
    #[must_use]
    pub(crate) const fn new(
        json: String,
        packages: usize,
        writes: usize,
        skips: usize,
        normalized_bytes: u64,
    ) -> Self {
        Self {
            json,
            packages,
            writes,
            skips,
            normalized_bytes,
        }
    }

    /// Returns the canonical JSON document.
    #[must_use]
    pub fn json(&self) -> &str {
        &self.json
    }

    /// Returns the number of supported packages inspected.
    #[must_use]
    pub const fn package_count(&self) -> usize {
        self.packages
    }

    /// Returns the number of members that would write output.
    #[must_use]
    pub const fn write_count(&self) -> usize {
        self.writes
    }

    /// Returns the number of members that would be skipped.
    #[must_use]
    pub const fn skip_count(&self) -> usize {
        self.skips
    }

    /// Returns the total normalized output byte count.
    #[must_use]
    pub const fn normalized_bytes(&self) -> u64 {
        self.normalized_bytes
    }
}
