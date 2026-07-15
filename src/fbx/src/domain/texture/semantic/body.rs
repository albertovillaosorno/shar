// File:
//   - body.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body.rs
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
//   - The public pure-domain facade and orchestration for semantic body-atlas
//   - planning.
// - Must-Not:
//   - Read files, decode PNG containers, serialize manifests, or alter
//   - topology, skeletons, skin weights, normals, or animation data.
// - Allows:
//   - Focused recipe, evidence, chart, raster, and UV-remapping modules.
// - Split-When:
//   - A new texture family cannot reuse body classification and chart planning.
// - Merge-When:
//   - The parent semantic facade can expose the same API directly.
// - Summary:
//   - Deterministic semantic body-atlas planning facade.
// - Description:
//   - Coordinates classification and chart planning behind stable pure types.
// - Usage:
//   - Called by repository-owned artifact adapters and synthetic tests.
// - Defaults:
//   - Ambiguous evidence fails closed and every minimum body region is
//   - required.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Public semantic body-atlas planning facade.
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
    let classification = classification::classify(
        character,
        source_texture,
        recipe,
    )?;
    charts::build_plan(
        character,
        recipe,
        classification,
    )
}
