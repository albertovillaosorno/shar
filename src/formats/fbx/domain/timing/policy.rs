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
//   - Policy domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Policy domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Policy domain module.

use super::error::TimingPolicyError;

/// Timing policy for animation-capable exports.
// The explicit domain name keeps policy vocabulary unambiguous at public call
// sites without suppressing any sibling item or test module.
#[expect(
    clippy::module_name_repetitions,
    reason = "Explicit naming distinguishes validated timing at public call \
              sites."
)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimingPolicy {
    /// Frames per second selected for clip export.
    pub frames_per_second: f32,
    /// Whether cyclic clips should remain cyclic.
    pub preserves_cycles: bool,
}

impl TimingPolicy {
    /// Create a finite, positive timing policy.
    ///
    /// # Errors
    ///
    /// Returns an error when frame rate is non-finite or not positive.
    pub fn new(
        frames_per_second: f32,
        preserves_cycles: bool,
    ) -> Result<Self, TimingPolicyError> {
        if !frames_per_second.is_finite() || frames_per_second <= 0. {
            return Err(TimingPolicyError::InvalidFrameRate);
        }
        Ok(Self {
            frames_per_second,
            preserves_cycles,
        })
    }
}

#[cfg(test)]
#[path = "../../../../../tests/formats/fbx/unit/domain/timing/policy/tests.rs"]
mod tests;
