// File:
//   - recipe.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/recipe.rs
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
//   - Validated semantic body-group selection, color overrides, and atlas
//   - configuration.
// - Must-Not:
//   - Inspect characters, sample images, build charts, or mutate UVs.
// - Allows:
//   - Checked dimensions, explicit reviewed overrides, and deterministic group
//   - ordering.
// - Split-When:
//   - Outfit-specific recipes require a separate compatibility contract.
// - Merge-When:
//   - Another body-planning module owns the same input validation.
// - Summary:
//   - Semantic body preparation recipe.
// - Description:
//   - Separates human-reviewed evidence from pure automatic classification.
// - Usage:
//   - Constructed by adapters before invoking the body planner.
// - Defaults:
//   - Selected groups are sorted and duplicate selection is rejected.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: group identity, atlas validation, and reviewed overrides form one
//   - cohesive request contract.
//

//! Validated semantic body preparation recipe.
#![expect(
    clippy::indexing_slicing,
    reason = "Recipe group addresses are validated before exact part and \
              group access."
)]

use std::collections::BTreeMap;

use super::super::color::Rgba8;
use super::super::region::BodyRegion;
use super::super::sampling::TextureAddressMode;
use super::error::SemanticTextureError;

/// Stable location of one primitive group inside a character aggregate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct GroupAddress {
    /// Character part ordinal.
    pub part_index: usize,
    /// Primitive-group ordinal inside the part.
    pub group_index: usize,
}

/// Deterministic atlas dimensions and padding policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AtlasConfig {
    /// Atlas width in pixels.
    pub width: u32,
    /// Atlas height in pixels.
    pub height: u32,
    /// Chart edge-dilation width in pixels.
    pub padding: u32,
    /// Opaque neutral fill for unused atlas texels.
    pub background: Rgba8,
}

impl AtlasConfig {
    /// Validate one atlas configuration.
    ///
    /// # Errors
    ///
    /// Returns an error when dimensions cannot hold five semantic columns and
    /// chart padding.
    pub fn new(
        width: u32,
        height: u32,
        padding: u32,
        background: Rgba8,
    ) -> Result<Self, SemanticTextureError> {
        let minimum_dimension = padding
            .checked_mul(2)
            .and_then(|value| value.checked_add(8))
            .ok_or(SemanticTextureError::AtlasDimensionsOverflow)?;
        let region_count = u32::try_from(BodyRegion::ALL.len())
            .map_err(|_error| SemanticTextureError::AtlasDimensionsOverflow)?;
        let minimum_width = minimum_dimension
            .checked_mul(region_count)
            .ok_or(SemanticTextureError::AtlasDimensionsOverflow)?;
        if width < minimum_width || height < minimum_dimension {
            return Err(
                SemanticTextureError::AtlasTooSmall {
                    width,
                    height,
                    padding,
                },
            );
        }
        if background.alpha != u8::MAX {
            return Err(SemanticTextureError::TransparentBodyAtlasBackground);
        }
        Ok(
            Self {
                width,
                height,
                padding,
                background,
            },
        )
    }
}

/// Evidence and overrides for one flat-color body preparation pass.
#[derive(Clone, Debug, PartialEq)]
pub struct BodySemanticRecipe {
    /// Primitive groups that belong to the integrated body and clothing atlas.
    pub groups: Vec<GroupAddress>,
    /// Reviewed exact source-color classifications for ambiguous evidence.
    pub color_overrides: BTreeMap<Rgba8, BodyRegion>,
    /// Source texture addressing applied before palette sampling.
    pub texture_address_mode: TextureAddressMode,
    /// Maximum exposed-color luminance ratio classified as hair.
    pub hair_luminance_ratio: f32,
    /// Destination atlas contract.
    pub atlas: AtlasConfig,
}

impl BodySemanticRecipe {
    /// Build and normalize one semantic body recipe.
    ///
    /// # Errors
    ///
    /// Returns an error for empty or duplicate groups or an invalid hair ratio.
    pub fn new(
        mut groups: Vec<GroupAddress>,
        color_overrides: BTreeMap<Rgba8, BodyRegion>,
        texture_address_mode: TextureAddressMode,
        hair_luminance_ratio: f32,
        atlas: AtlasConfig,
    ) -> Result<Self, SemanticTextureError> {
        if groups.is_empty() {
            return Err(SemanticTextureError::MissingBodyGroups);
        }
        if !hair_luminance_ratio.is_finite()
            || !(0.0..1.0).contains(&hair_luminance_ratio)
        {
            return Err(SemanticTextureError::InvalidHairLuminanceRatio);
        }
        groups.sort_unstable();
        if groups
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err(SemanticTextureError::DuplicateBodyGroup);
        }
        Ok(
            Self {
                groups,
                color_overrides,
                texture_address_mode,
                hair_luminance_ratio,
                atlas,
            },
        )
    }
}
