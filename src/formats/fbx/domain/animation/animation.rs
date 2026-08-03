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
//   - Animation domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Animation domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Animation domain module.

pub mod capability;
pub mod clip;
pub mod quaternion;
pub mod requirement;
pub mod translator;

// Explicit facade names keep downstream imports unambiguous across domain
// modules.
#[expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these explicit facade names preserve stable domain \
              imports."
)]
pub use capability::AnimationCapability;
// Explicit facade names keep downstream imports unambiguous across domain
// modules.
#[expect(
    clippy::module_name_repetitions,
    reason = "Tests verify explicit animation clip facade names remain stable."
)]
pub use clip::{
    AnimationClip, AnimationClipError, BoneAnimationTrack, LocalTransformSample,
};
// Explicit facade names keep downstream imports unambiguous across domain
// modules.
#[expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these explicit facade names preserve stable domain \
              imports."
)]
pub use requirement::{AnimationRequirement, AnimationRequirementError};
pub use translator::preserve_animation_ids;
