// File:
//   - semantic.rs
// Path:
//   - src/fbx/src/domain/texture/semantic.rs
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
//   - Public pure-domain exports for semantic character texture preparation.
// - Must-Not:
//   - Read files, decode PNG containers, or serialize artifacts.
// - Allows:
//   - Focused color, image, body-atlas, and eye-frame domain modules.
// - Split-When:
//   - A semantic texture family needs an independent facade.
// - Merge-When:
//   - The parent texture facade can expose the same stable API directly.
// - Summary:
//   - Semantic character texture domain facade.
// - Description:
//   - Keeps repository-owned texture planning independent from adapters.
// - Usage:
//   - Imported by PNG and artifact adapters and behavioral tests.
// - Defaults:
//   - No filesystem or external authoring application is implied.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Pure semantic character texture preparation.
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
