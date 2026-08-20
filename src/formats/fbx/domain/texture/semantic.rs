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
//   - Semantic domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Semantic domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Semantic domain module.

#![expect(
    clippy::module_name_repetitions,
    reason = "Public semantic texture names remain explicit at the domain \
              boundary."
)]

pub mod body;
pub mod color;
pub mod eye;
pub mod image;
pub mod region;
mod sampling;

pub use body::{
    AtlasChart, AtlasConfig, BodySemanticRecipe, BodyTexturePlan, GroupAddress,
    PixelRect, ProjectionAxis, SemanticTextureError, SourceColorAssignment,
    plan_body_texture,
};
pub use color::{LinearRgb, Rgba8};
pub use eye::{
    EyeComponent, EyeFrameEvidence, EyeRegion, EyeSemanticPlan, EyeSide,
    EyeTextureError, analyze_eye_frames,
};
pub use image::{RgbaImage, RgbaImageError};
pub use region::{BodyRegion, BoneFamily};
pub use sampling::TextureAddressMode;
