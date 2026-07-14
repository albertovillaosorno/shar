// File:
//   - policy.rs
// Path:
//   - src/fbx/src/domain/scale/policy.rs
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
//   - Pure fbx domain rules for domain scale policy.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when policy contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Multiplicative factor from decoded units into FBX scene units.
// - Description:
//   - Defines policy data and behavior for fbx domain scale.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Unit scale policy applied to model coordinates before serialization.
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
mod tests {
    use super::ScalePolicy;

    #[test]
    fn accepts_positive_finite_unit_scale() -> Result<(), String> {
        let policy = ScalePolicy::new(
            1.0, true,
        )
        .map_err(|error| format!("valid scale policy failed: {error:?}"))?;
        if policy
            .unit_scale
            .to_bits()
            == 1.0_f32.to_bits()
            && policy.preserves_source_axes
        {
            Ok(())
        } else {
            Err(format!("unexpected scale policy: {policy:?}"))
        }
    }

    #[test]
    fn rejects_nonpositive_or_nonfinite_unit_scale() -> Result<(), String> {
        for value in [
            0.0,
            -1.0,
            f32::INFINITY,
            f32::NAN,
        ] {
            if ScalePolicy::new(
                value, false,
            )
            .is_ok()
            {
                return Err(
                    format!("invalid unit scale was accepted: {value}"),
                );
            }
        }
        Ok(())
    }
}
