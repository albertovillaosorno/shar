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

use super::error::ScalePolicyError;

/// Unit scale policy applied to model coordinates before serialization.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ScalePolicy {
    /// Multiplicative factor from decoded units into FBX scene units.
    pub unit_scale: f32,
    /// Whether coordinates are currently preserved without handedness changes.
    pub preserves_source_axes: bool,
}

impl ScalePolicy {
    /// Create a finite, positive scale policy.
    ///
    /// # Errors
    ///
    /// Returns an error when the scale is non-finite or not positive.
    pub fn new(
        unit_scale: f32,
        preserves_source_axes: bool,
    ) -> Result<Self, ScalePolicyError> {
        if !unit_scale.is_finite() || unit_scale <= 0.0 {
            return Err(ScalePolicyError::InvalidUnitScale);
        }
        Ok(
            Self {
                unit_scale,
                preserves_source_axes,
            },
        )
    }
}

#[cfg(test)]
#[path = "../../../../../tests/formats/fbx/unit/domain/scale/policy/tests.rs"]
mod tests;
