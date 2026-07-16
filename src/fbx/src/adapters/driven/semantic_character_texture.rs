// File:
//   - semantic_character_texture.rs
// Path:
//   - src/fbx/src/adapters/driven/semantic_character_texture.rs
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
//   - The in-memory driven transaction that loads one explicit decoded
//   - character, prepares semantic body and eye textures, and returns
//   - deterministic artifact bytes.
// - Must-Not:
//   - Discover paths, publish files, bulk-export characters, change topology,
//   - or invoke Blender, Maya, Unreal, Python, or external image processes.
// - Allows:
//   - Repository-owned decoded component loading, PNG conversion, pure semantic
//   - planning, and manifest rendering.
// - Split-When:
//   - Body and eye artifact transactions can succeed independently.
// - Merge-When:
//   - Another driven adapter owns the same character texture artifact bundle.
// - Summary:
//   - Semantic character texture artifact builder.
// - Description:
//   - Executes every preparation stage before filesystem publication so
//   - failures cannot be confused with a complete result.
// - Usage:
//   - Called by the focused local CLI and behavioral artifact tests.
// - Defaults:
//   - Preserves one body texture, derives three eye images, and renders one
//   - manifest in memory; semantic atlas remapping is explicit opt-in.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: explicit component loading, image decode, pure planning, encode,
//   - manifest, and summary assembly form one atomic in-memory transaction.
//

//! In-memory semantic character texture artifact transaction.
use std::collections::BTreeSet;
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local::read_bytes;

use crate::adapters::driven::decoded_animation_source::load_animation_clips;
use crate::adapters::driven::decoded_skin_source::load_character;
use crate::adapters::driven::semantic_texture_png::decode_png_bytes;
use crate::domain::animation::AnimationClip;
use crate::domain::mesh::PrimitiveGroup;
use crate::domain::texture::semantic::{
    BodyTexturePlan, GroupAddress, RgbaImage, analyze_eye_frames,
    plan_body_texture,
};

#[path = "semantic_character_texture/artifacts.rs"]
mod artifacts;
#[path = "semantic_character_texture/manifest.rs"]
mod manifest;
#[path = "semantic_character_texture/package.rs"]
mod package;
#[path = "semantic_character_texture/publication.rs"]
mod publication;
#[path = "semantic_character_texture/request.rs"]
pub mod request;

pub use publication::publish_prepared_semantic_character;
pub use request::{BodyTextureMode, SemanticTextureRequest};

/// Complete deterministic artifact byte bundle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticTextureArtifacts {
    /// Preserved source body texture or generated semantic atlas PNG bytes.
    pub body_texture_png: Vec<u8>,
    /// Derived open eye, pupil, and paired-lid PNG bytes when eyes exist.
    pub eye_layer_pngs: Option<[Vec<u8>; 3]>,
    /// SHA-256 identity of the eye-layer SSOT when eyes exist.
    pub eye_profile_sha256: Option<String>,
    /// Explicit extra material textures in stable file-name order.
    pub extra_textures: Vec<ExternalTextureArtifact>,
    /// Deterministic JSON manifest bytes with one trailing newline.
    pub manifest_json: Vec<u8>,
    /// Compact observable generation summary.
    pub summary: SemanticTextureSummary,
}

/// One explicit external texture copied into the prepared package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalTextureArtifact {
    /// Portable file name below `textures/`.
    pub file_name: String,
    /// Deterministic RGBA PNG bytes.
    pub png: Vec<u8>,
}

/// Complete prepared character package before filesystem publication.
#[derive(Clone, Debug, PartialEq)]
pub struct PreparedSemanticCharacter {
    /// Character with the selected body texture policy applied.
    pub character: crate::domain::character::CharacterAsset,
    /// Complete material bindings for every primitive-group shader.
    pub materials: Vec<crate::domain::texture::MaterialBinding>,
    /// General and character-specific skeletal clips in request order.
    pub animations: Vec<AnimationClip>,
    /// Generated and copied texture artifacts.
    pub artifacts: SemanticTextureArtifacts,
}

/// Compact artifact generation summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticTextureSummary {
    /// Stable character identity from the request.
    pub character_id: String,
    /// Selected body source vertex count.
    pub body_vertex_count: usize,
    /// Selected body source triangle count.
    pub body_triangle_count: usize,
    /// Generated body chart count.
    pub body_chart_count: usize,
    /// Semantic eye region count across two components.
    pub eye_region_count: usize,
    /// Skeletal animation clip count written into the prepared FBX.
    pub animation_count: usize,
    /// Published body texture dimensions.
    pub body_texture_size: [u32; 2],
    /// Modern square eye frame dimension when the model has an eye component.
    pub eye_frame_size: Option<u32>,
}

/// Semantic character texture artifact failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticTextureArtifactError {
    /// Character identity was blank after trimming.
    BlankCharacterName,
    /// Request did not supply any decoded skin component.
    MissingSkinPaths,
    /// Request JSON or recipe conversion failed.
    Request(String),
    /// Decoded character loading failed.
    Character(String),
    /// Skeletal animation loading or binding failed.
    Animation(String),
    /// Two requested skeletal clips resolved to the same stable name.
    DuplicateAnimationClip(String),
    /// Input image read failed.
    Read {
        /// Input role or frame identity.
        role: String,
        /// Underlying I/O diagnostic.
        message: String,
    },
    /// PNG decoding or encoding failed.
    Png(String),
    /// Pure semantic body planning failed.
    Body(String),
    /// Requested eye primitive group did not exist.
    MissingEyeGroup(GroupAddress),
    /// Pure semantic eye analysis failed.
    Eye(String),
    /// Material or extra-texture package assembly failed.
    Package(String),
    /// Exactly three canonical eye layers could not be assembled.
    EyeLayerCount,
    /// Exactly four compatibility eye images could not be assembled.
    EyeFrameCount,
    /// Manifest rendering failed.
    Manifest(String),
}

/// Prepare one complete remapped character package in memory.
///
/// # Errors
///
/// Returns an error when any component, image, semantic plan, material,
/// texture, encode, or manifest stage fails. No output file is written.
pub fn prepare_semantic_character(
    request: &SemanticTextureRequest
) -> Result<PreparedSemanticCharacter, SemanticTextureArtifactError> {
    let character_name = request
        .character_name
        .trim();
    if character_name.is_empty() {
        return Err(SemanticTextureArtifactError::BlankCharacterName);
    }
    if request
        .skin_paths
        .is_empty()
    {
        return Err(SemanticTextureArtifactError::MissingSkinPaths);
    }
    let skin_paths = request
        .skin_paths
        .iter()
        .map(std::path::PathBuf::as_path)
        .collect::<Vec<_>>();
    let mesh_paths = request
        .mesh_paths
        .iter()
        .map(std::path::PathBuf::as_path)
        .collect::<Vec<_>>();
    let composite_paths = request
        .composite_paths
        .iter()
        .map(std::path::PathBuf::as_path)
        .collect::<Vec<_>>();
    let character = load_character(
        character_name,
        &request.skeleton_path,
        &skin_paths,
        &mesh_paths,
        &composite_paths,
    )
    .map_err(
        |error| SemanticTextureArtifactError::Character(format!("{error:?}")),
    )?;
    let animations = load_requested_animations(
        request,
        &character.bones,
    )?;
    let body_source = decode_image(
        "body-texture",
        &request.body_texture_path,
    )?;
    let body_mode = request
        .body_texture_mode()
        .map_err(
            |error| SemanticTextureArtifactError::Request(format!("{error:?}")),
        )?;
    let recipe = request
        .body_recipe()
        .map_err(
            |error| SemanticTextureArtifactError::Request(format!("{error:?}")),
        )?;
    let body = match body_mode {
        BodyTextureMode::PreserveSource => preserve_body_texture(
            &character,
            &body_source,
            &recipe.groups,
        )?,
        BodyTextureMode::SemanticAtlas => plan_body_texture(
            &character,
            &body_source,
            &recipe,
        )
        .map_err(
            |error| SemanticTextureArtifactError::Body(format!("{error:?}")),
        )?,
    };
    let eye = prepare_eye(
        &character, request,
    )?;
    let package = package::build(
        request,
        &body.remapped_character,
    )?;
    let artifacts = artifacts::assemble(
        request,
        &body,
        eye.as_ref(),
        animations.len(),
        package.extra_textures,
    )?;
    Ok(
        PreparedSemanticCharacter {
            character: body.remapped_character,
            materials: package.materials,
            animations,
            artifacts,
        },
    )
}

/// Load all requested skeletal clips and require unique stable names.
fn load_requested_animations(
    request: &SemanticTextureRequest,
    bones: &[crate::domain::skeleton::Bone],
) -> Result<Vec<AnimationClip>, SemanticTextureArtifactError> {
    let paths = request
        .general_animation_paths
        .iter()
        .chain(&request.character_animation_paths)
        .map(std::path::PathBuf::as_path)
        .collect::<Vec<_>>();
    let animations = load_animation_clips(
        &paths, bones,
    )
    .map_err(
        |error| SemanticTextureArtifactError::Animation(format!("{error:?}")),
    )?;
    let mut names = BTreeSet::new();
    for animation in &animations {
        if !names.insert(
            animation
                .name
                .clone(),
        ) {
            return Err(
                SemanticTextureArtifactError::DuplicateAnimationClip(
                    animation
                        .name
                        .clone(),
                ),
            );
        }
    }
    Ok(animations)
}

/// Decode and analyze the four eye frames when one eye group exists.
fn prepare_eye(
    character: &crate::domain::character::CharacterAsset,
    request: &SemanticTextureRequest,
) -> Result<
    Option<crate::domain::texture::semantic::EyeSemanticPlan>,
    SemanticTextureArtifactError,
> {
    let (eye_group_request, eye_frame_paths) = match (
        request.eye_group,
        request
            .eye_frame_paths
            .as_ref(),
    ) {
        (None, None) => return Ok(None),
        (Some(group_request), Some(frame_paths)) => (
            group_request,
            frame_paths,
        ),
        (None, Some(_)) | (Some(_), None) => {
            return Err(
                SemanticTextureArtifactError::Request(
                    format!(
                        "{:?}",
                        request::RequestError::IncompleteEyeSelection
                    ),
                ),
            );
        }
    };
    let eye_group_address: GroupAddress = eye_group_request.into();
    let eye_group = group(
        character,
        eye_group_address,
    )?;
    let eye_source_images = eye_frame_paths
        .iter()
        .enumerate()
        .map(
            |(index, path)| {
                decode_image(
                    &format!("eye-frame-{index}"),
                    path,
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let eye_sources: [_; 4] = eye_source_images
        .try_into()
        .map_err(
            |_frames: Vec<_>| SemanticTextureArtifactError::EyeFrameCount,
        )?;
    analyze_eye_frames(
        eye_group,
        &eye_sources,
        request.eye_output_size,
    )
    .map(Some)
    .map_err(|error| SemanticTextureArtifactError::Eye(format!("{error:?}")))
}

/// Preserve source body UVs and pixels while validating selected groups.
fn preserve_body_texture(
    character: &crate::domain::character::CharacterAsset,
    source: &RgbaImage,
    groups: &[GroupAddress],
) -> Result<BodyTexturePlan, SemanticTextureArtifactError> {
    let mut vertex_count = 0_usize;
    let mut triangle_count = 0_usize;
    for address in groups {
        let part = character
            .parts
            .get(address.part_index)
            .ok_or_else(
                || {
                    SemanticTextureArtifactError::Body(
                        format!("missing body part: {address:?}"),
                    )
                },
            )?;
        let group = part
            .mesh
            .groups
            .get(address.group_index)
            .ok_or_else(
                || {
                    SemanticTextureArtifactError::Body(
                        format!("missing body group: {address:?}"),
                    )
                },
            )?;
        if group
            .uvs
            .len()
            != group
                .positions
                .len()
        {
            return Err(
                SemanticTextureArtifactError::Body(
                    format!("body group has incomplete UVs: {address:?}"),
                ),
            );
        }
        vertex_count = vertex_count
            .checked_add(
                group
                    .positions
                    .len(),
            )
            .ok_or_else(
                || {
                    SemanticTextureArtifactError::Body(
                        "body count overflow".to_owned(),
                    )
                },
            )?;
        triangle_count = triangle_count
            .checked_add(
                group
                    .triangles
                    .len(),
            )
            .ok_or_else(
                || {
                    SemanticTextureArtifactError::Body(
                        "body count overflow".to_owned(),
                    )
                },
            )?;
    }
    Ok(
        BodyTexturePlan {
            atlas: source.clone(),
            remapped_character: character.clone(),
            color_assignments: Vec::new(),
            charts: Vec::new(),
            source_vertex_count: vertex_count,
            source_triangle_count: triangle_count,
        },
    )
}

/// Build only the texture artifacts for callers that do not publish an FBX.
///
/// # Errors
///
/// Returns the same failures as [`prepare_semantic_character`].
pub fn build_semantic_texture_artifacts(
    request: &SemanticTextureRequest
) -> Result<SemanticTextureArtifacts, SemanticTextureArtifactError> {
    Ok(prepare_semantic_character(request)?.artifacts)
}

/// Decode one explicit input PNG through repository-owned I/O and PNG adapters.
fn decode_image(
    role: &str,
    path: &Path,
) -> Result<RgbaImage, SemanticTextureArtifactError> {
    let bytes = read_bytes(path).map_err(
        |error| SemanticTextureArtifactError::Read {
            role: role.to_owned(),
            message: error.to_string(),
        },
    )?;
    decode_png_bytes(&bytes).map_err(
        |error| SemanticTextureArtifactError::Png(format!("{error:?}")),
    )
}

/// Resolve one explicit primitive group without discovery.
fn group(
    character: &crate::domain::character::CharacterAsset,
    address: GroupAddress,
) -> Result<&PrimitiveGroup, SemanticTextureArtifactError> {
    character
        .parts
        .get(address.part_index)
        .and_then(
            |part| {
                part.mesh
                    .groups
                    .get(address.group_index)
            },
        )
        .ok_or(SemanticTextureArtifactError::MissingEyeGroup(address))
}
