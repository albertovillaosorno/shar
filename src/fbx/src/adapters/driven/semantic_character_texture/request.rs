// File:
//   - request.rs
// Path:
//   - src/fbx/src/adapters/driven/semantic_character_texture/request.rs
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
//   - Explicit local component paths and reviewed semantic recipe input for one
//   - character texture preparation transaction.
// - Must-Not:
//   - Discover files, read assets, execute planning, or publish output.
// - Allows:
//   - JSON decoding and checked conversion into pure domain recipe values.
// - Split-When:
//   - Outfit or patterned-material recipes require an independent schema.
// - Merge-When:
//   - Another adapter request owns the same semantic preparation inputs.
// - Summary:
//   - Semantic character texture artifact request.
// - Description:
//   - Makes every source path, group address, color override, and output policy
//   - explicit rather than depending on workstation discovery.
// - Usage:
//   - Parsed by the focused semantic-character-texture CLI.
// - Defaults:
//   - No path, group, color, or atlas default is inferred by the adapter.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: path evidence and semantic recipe fields form one versioned input
//   - schema with checked domain conversion.
//

//! Explicit semantic character texture artifact request.
use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Deserialize;

use crate::domain::texture::semantic::{
    AtlasConfig, BodyRegion, BodySemanticRecipe, GroupAddress, Rgba8,
    SemanticTextureError, TextureAddressMode,
};

/// One versioned local character texture preparation request.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticTextureRequest {
    /// Stable character identity used only in the generated manifest.
    pub character_name: String,
    /// Explicit decoded skeleton component path.
    pub skeleton_path: PathBuf,
    /// Explicit decoded skin component paths.
    pub skin_paths: Vec<PathBuf>,
    /// Explicit decoded composite component paths.
    pub composite_paths: Vec<PathBuf>,
    /// Shared or default skeletal animation component paths.
    #[serde(default)]
    pub general_animation_paths: Vec<PathBuf>,
    /// Character-specific skeletal animation component paths.
    #[serde(default)]
    pub character_animation_paths: Vec<PathBuf>,
    /// Explicit source body palette or texture PNG path.
    pub body_texture_path: PathBuf,
    /// Body texture policy: `preserve-source` or `semantic-atlas`.
    pub body_texture_mode: String,
    /// Source texture addressing: `tile` or `clamp`.
    pub body_texture_address_mode: String,
    /// Exactly four source eye texture-frame PNG paths in animation order.
    pub eye_frame_paths: [PathBuf; 4],
    /// Primitive groups included in the integrated body and clothing atlas.
    pub body_groups: Vec<GroupAddressRequest>,
    /// Primitive group containing both eye components.
    pub eye_group: GroupAddressRequest,
    /// Reviewed exact source-color overrides.
    pub color_overrides: Vec<ColorOverrideRequest>,
    /// Maximum exposed-color luminance ratio classified as hair.
    pub hair_luminance_ratio: f32,
    /// Semantic-atlas width used only by `semantic-atlas` mode.
    pub body_atlas_width: u32,
    /// Semantic-atlas height used only by `semantic-atlas` mode.
    pub body_atlas_height: u32,
    /// Chart edge-dilation width.
    pub body_atlas_padding: u32,
    /// Opaque unused body-atlas color.
    pub body_atlas_background: [u8; 4],
    /// Square eye texture dimension; source resolution is preferred.
    pub eye_output_size: u32,
    /// Explicit non-body, non-eye material texture bindings.
    #[serde(default)]
    pub extra_materials: Vec<ExtraMaterialRequest>,
}

/// Body texture preparation policy selected by one explicit request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BodyTextureMode {
    /// Preserve source UVs and publish a normalized copy of the source PNG.
    PreserveSource,
    /// Generate and bind the experimental semantic body atlas.
    SemanticAtlas,
}

impl SemanticTextureRequest {
    /// Parse the explicit body texture preparation policy.
    ///
    /// # Errors
    ///
    /// Returns an error when the identity is not one of the two supported
    /// values.
    pub fn body_texture_mode(&self) -> Result<BodyTextureMode, RequestError> {
        match self
            .body_texture_mode
            .as_str()
        {
            "preserve-source" => Ok(BodyTextureMode::PreserveSource),
            "semantic-atlas" => Ok(BodyTextureMode::SemanticAtlas),
            value => {
                Err(RequestError::UnknownBodyTextureMode(value.to_owned()))
            }
        }
    }

    /// Convert request values into the pure body semantic recipe.
    ///
    /// # Errors
    ///
    /// Returns an error for unknown region names, duplicate color overrides, or
    /// invalid domain recipe values.
    pub fn body_recipe(&self) -> Result<BodySemanticRecipe, RequestError> {
        let atlas = AtlasConfig::new(
            self.body_atlas_width,
            self.body_atlas_height,
            self.body_atlas_padding,
            rgba(self.body_atlas_background),
        )?;
        let mut overrides = BTreeMap::new();
        for request in &self.color_overrides {
            let color = rgba(request.rgba);
            let region = parse_region(&request.region)?;
            if overrides
                .insert(
                    color, region,
                )
                .is_some()
            {
                return Err(RequestError::DuplicateColorOverride(color));
            }
        }
        BodySemanticRecipe::new(
            self.body_groups
                .iter()
                .copied()
                .map(Into::into)
                .collect(),
            overrides,
            parse_texture_address_mode(&self.body_texture_address_mode)?,
            self.hair_luminance_ratio,
            atlas,
        )
        .map_err(Into::into)
    }
}

/// JSON representation of one primitive-group address.
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GroupAddressRequest {
    /// Character part ordinal.
    pub part_index: usize,
    /// Primitive-group ordinal inside the part.
    pub group_index: usize,
}

impl From<GroupAddressRequest> for GroupAddress {
    fn from(value: GroupAddressRequest) -> Self {
        Self {
            part_index: value.part_index,
            group_index: value.group_index,
        }
    }
}

/// JSON representation of one reviewed exact-color classification.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColorOverrideRequest {
    /// Exact source RGBA color.
    pub rgba: [u8; 4],
    /// Stable semantic region name.
    pub region: String,
}

/// One explicit external texture bound to an otherwise unclassified material.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExtraMaterialRequest {
    /// Exact decoded shader identity used by the primitive group.
    pub material_name: String,
    /// Explicit source PNG path.
    pub texture_path: PathBuf,
    /// Portable output file name below `textures/`.
    pub output_file_name: String,
}

/// Request validation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RequestError {
    /// Domain recipe validation failed.
    Domain(SemanticTextureError),
    /// Region identity was not one of the five supported body regions.
    UnknownRegion(String),
    /// Body texture mode was not one of the two supported identities.
    UnknownBodyTextureMode(String),
    /// Texture-address identity was not `tile` or `clamp`.
    UnknownTextureAddressMode(String),
    /// Two override rows targeted the same exact source color.
    DuplicateColorOverride(Rgba8),
}

impl From<SemanticTextureError> for RequestError {
    fn from(error: SemanticTextureError) -> Self {
        Self::Domain(error)
    }
}

/// Convert one channel tuple into the domain color value.
const fn rgba(channels: [u8; 4]) -> Rgba8 {
    Rgba8::new(
        channels[0],
        channels[1],
        channels[2],
        channels[3],
    )
}

/// Parse one stable source texture-address identity.
fn parse_texture_address_mode(
    value: &str
) -> Result<TextureAddressMode, RequestError> {
    match value {
        "tile" => Ok(TextureAddressMode::Tile),
        "clamp" => Ok(TextureAddressMode::Clamp),
        _ => Err(RequestError::UnknownTextureAddressMode(value.to_owned())),
    }
}

/// Parse one stable body-region identity.
fn parse_region(value: &str) -> Result<BodyRegion, RequestError> {
    match value {
        "skin" => Ok(BodyRegion::Skin),
        "hair" => Ok(BodyRegion::Hair),
        "torso" => Ok(BodyRegion::Torso),
        "legs" => Ok(BodyRegion::Legs),
        "shoes" => Ok(BodyRegion::Shoes),
        _ => Err(RequestError::UnknownRegion(value.to_owned())),
    }
}
