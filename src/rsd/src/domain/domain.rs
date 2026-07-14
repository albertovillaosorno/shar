// File:
//   - domain.rs
// Path:
//   - src/rsd/src/domain/domain.rs
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
//   - rsd module behavior for domain.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute domain.
// - Split-When:
//   - Split when domain contains two independently testable contracts.
// - Merge-When:
//   - Another rsd module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Domain model for Radical Sound `.rsd` audio containers.
// - Description:
//   - Defines domain data and behavior for rsd root.
// - Usage:
//   - Used by rsd root code that needs domain.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Domain model for Radical Sound `.rsd` audio containers.
mod error;
mod export_report;
mod rsd;
mod wav;

/// Wraps one untrusted path for exact, control-safe diagnostic rendering.
pub(crate) struct EscapedPath<'a>(&'a std::path::Path);

impl<'a> EscapedPath<'a> {
    /// Creates one borrowed diagnostic wrapper without normalizing the path.
    #[must_use]
    pub(crate) const fn new(path: &'a std::path::Path) -> Self {
        Self(path)
    }
}

impl core::fmt::Display for EscapedPath<'_> {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        escaped_path::write_path(
            formatter, self.0,
        )
    }
}

mod escaped_path;

pub use error::RsdError;
pub use export_report::{ExportReport, SourceRootReport};
pub use rsd::{RsdAudio, RsdEncoding, RsdHeader};
pub use wav::WavAudio;

/// Allocates one exact byte buffer for parser and serializer output.
///
/// # Errors
///
/// Returns [`RsdError::AllocationFailed`] when the requested capacity cannot be
/// represented or reserved.
pub(crate) fn byte_buffer(capacity: usize) -> Result<Vec<u8>, RsdError> {
    let mut bytes = Vec::new();
    bytes
        .try_reserve_exact(capacity)
        .map_err(|_reserve_error| RsdError::AllocationFailed(capacity))?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::{RsdError, byte_buffer};

    #[test]
    fn impossible_buffer_capacity_returns_without_panicking() {
        let allocation = std::panic::catch_unwind(|| byte_buffer(usize::MAX));

        assert!(
            allocation.is_ok(),
            "untrusted buffer sizes must return a typed error instead of \
             panicking"
        );
        let Ok(result) = allocation else {
            return;
        };
        assert!(
            matches!(
                result,
                Err(RsdError::AllocationFailed(usize::MAX))
            ),
            "impossible capacities must retain the requested byte count"
        );
    }
}
