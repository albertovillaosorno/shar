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
//   - Body domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Body domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Body domain module.

#![expect(
    clippy::module_name_repetitions,
    reason = "Body semantic types retain explicit names across adapter \
              boundaries."
)]

use super::image::RgbaImage;
use crate::domain::character::CharacterAsset;

#[path = "body/charts.rs"]
mod charts;
#[path = "body/classification.rs"]
mod classification;
#[path = "body/error.rs"]
mod error;
#[path = "body/raster.rs"]
mod raster;
#[path = "body/recipe.rs"]
mod recipe;
#[path = "body/types.rs"]
mod types;

pub use error::SemanticTextureError;
pub use recipe::{AtlasConfig, BodySemanticRecipe, GroupAddress};
pub use types::{
    AtlasChart, BodyTexturePlan, PixelRect, ProjectionAxis,
    SourceColorAssignment,
};

/// Classify, chart, pack, rasterize, and UV-remap one character body.
///
/// # Errors
///
/// Returns an error whenever evidence is incomplete, ambiguous, mixed inside a
/// triangle, non-projectable, or too large for the declared atlas.
pub fn plan_body_texture(
    character: &CharacterAsset,
    source_texture: &RgbaImage,
    recipe: &BodySemanticRecipe,
) -> Result<BodyTexturePlan, SemanticTextureError> {
    let classification =
        classification::classify(character, source_texture, recipe)?;
    charts::build_plan(character, source_texture, recipe, classification)
}
