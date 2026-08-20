// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Sha256 domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Sha256 domain module.
// - Description:
//   - Implements the declared domain module responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Sha256 domain module.

/// Exact SHA-256 digest used by RMV provenance records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sha256(pub [u8; 32]);

impl Sha256 {
    /// Render the lowercase hexadecimal identity.
    #[must_use]
    pub fn hex(self) -> String {
        let mut output = String::with_capacity(64);
        for byte in self.0 {
            use core::fmt::Write as _;
            if write!(output, "{byte:02x}").is_err() {
                return output;
            }
        }
        output
    }
}

#[cfg(test)]
#[path = "../../../../tests/formats/rmv/unit/domain/sha256/tests.rs"]
mod tests;
