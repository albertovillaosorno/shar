// File:
//   - sha256.rs
// Path:
//   - src/rmv/src/domain/sha256.rs
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
//   - Pure rmv domain rules for domain sha256.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when sha256 contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Small SHA-256 implementation used to deduplicate movie inputs without a.
// - Description:
//   - Defines sha256 data and behavior for rmv domain.
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

//! RMV domain wrapper around the repository SHA-256 primitive.

/// Stable RMV movie-content identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sha256(pub [u8; 32]);

impl Sha256 {
    /// Hash exact movie bytes.
    #[must_use]
    pub fn digest(data: &[u8]) -> Self {
        Self(shar_sha256::digest(data))
    }

    /// Render the lowercase hexadecimal identity.
    #[must_use]
    pub fn hex(self) -> String {
        shar_sha256::hex(self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::Sha256;

    #[test]
    fn preserves_known_repository_digest() {
        assert_eq!(
            Sha256::digest(b"abc").hex(),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }
}
