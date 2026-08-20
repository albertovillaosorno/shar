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
//   - Translator domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Translator domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Translator domain module.

use super::capability::AnimationCapability;
use super::requirement::{AnimationRequirement, AnimationRequirementError};

/// Preserve animation ids as an explicit requirement until clip binding exists.
///
/// # Errors
///
/// Returns an error when one animation member identity is blank or duplicated.
pub fn preserve_animation_ids(
    member_ids: Vec<String>,
) -> Result<AnimationRequirement, AnimationRequirementError> {
    AnimationRequirement::new(member_ids, AnimationCapability::PreservedOnly)
}
