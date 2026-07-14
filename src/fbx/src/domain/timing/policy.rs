// File:
//   - policy.rs
// Path:
//   - src/fbx/src/domain/timing/policy.rs
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
//   - Pure fbx domain rules for domain timing policy.
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
//   - Timing policy for animation-capable exports.
// - Description:
//   - Defines policy data and behavior for fbx domain timing.
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

//! Timing policy for animation-capable exports.
//!
//! This boundary keeps timing policy for animation-capable exports explicit
//! and returns deterministic results to fbx callers.
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
        if !frames_per_second.is_finite() || frames_per_second <= 0.0 {
            return Err(TimingPolicyError::InvalidFrameRate);
        }
        Ok(
            Self {
                frames_per_second,
                preserves_cycles,
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::TimingPolicy;

    #[test]
    fn accepts_positive_finite_frame_rate() -> Result<(), String> {
        let policy = TimingPolicy::new(
            30.0, true,
        )
        .map_err(|error| format!("valid timing policy failed: {error:?}"))?;
        if policy
            .frames_per_second
            .to_bits()
            == 30.0_f32.to_bits()
            && policy.preserves_cycles
        {
            Ok(())
        } else {
            Err(format!("unexpected timing policy: {policy:?}"))
        }
    }

    #[test]
    fn rejects_nonpositive_or_nonfinite_frame_rate() -> Result<(), String> {
        for value in [
            0.0,
            -1.0,
            f32::INFINITY,
            f32::NAN,
        ] {
            if TimingPolicy::new(
                value, false,
            )
            .is_ok()
            {
                return Err(
                    format!("invalid frame rate was accepted: {value}"),
                );
            }
        }
        Ok(())
    }
}
