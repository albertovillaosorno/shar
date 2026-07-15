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
//   - Produces one body atlas, four eye frames, and one manifest in memory.
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
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local::read_bytes;

use crate::adapters::driven::decoded_skin_source::load_character;
use crate::adapters::driven::semantic_texture_png::decode_png_bytes;
use crate::domain::mesh::PrimitiveGroup;
use crate::domain::texture::semantic::{
    GroupAddress, analyze_eye_frames, plan_body_texture,
};

#[path = "semantic_character_texture/artifacts.rs"]
mod artifacts;
#[path = "semantic_character_texture/manifest.rs"]
mod manifest;
#[path = "semantic_character_texture/request.rs"]
pub mod request;

pub use request::SemanticTextureRequest;

/// Complete deterministic artifact byte bundle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticTextureArtifacts {
    /// Modern body atlas PNG bytes.
    pub body_atlas_png: Vec<u8>,
    /// Four modern eye texture-frame PNG byte sequences.
    pub eye_frame_pngs: [Vec<u8>; 4],
    /// Deterministic JSON manifest bytes with one trailing newline.
    pub manifest_json: Vec<u8>,
    /// Compact observable generation summary.
    pub summary: SemanticTextureSummary,
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
    /// Modern body atlas dimensions.
    pub body_atlas_size: [u32; 2],
    /// Modern square eye frame dimension.
    pub eye_frame_size: u32,
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
    /// Exactly four eye images could not be assembled.
    EyeFrameCount,
    /// Manifest rendering failed.
    Manifest(String),
}

/// Build every semantic texture artifact in memory from one explicit request.
///
/// # Errors
///
/// Returns an error when any component, image, semantic plan, encode, or
/// manifest stage fails. No output file is written by this function.
pub fn build_semantic_texture_artifacts(
    request: &SemanticTextureRequest
) -> Result<SemanticTextureArtifacts, SemanticTextureArtifactError> {
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
    let composite_paths = request
        .composite_paths
        .iter()
        .map(std::path::PathBuf::as_path)
        .collect::<Vec<_>>();
    let character = load_character(
        character_name,
        &request.skeleton_path,
        &skin_paths,
        &composite_paths,
    )
    .map_err(
        |error| SemanticTextureArtifactError::Character(format!("{error:?}")),
    )?;
    let body_source = decode_image(
        "body-texture",
        &request.body_texture_path,
    )?;
    let recipe = request
        .body_recipe()
        .map_err(
            |error| SemanticTextureArtifactError::Request(format!("{error:?}")),
        )?;
    let body = plan_body_texture(
        &character,
        &body_source,
        &recipe,
    )
    .map_err(
        |error| SemanticTextureArtifactError::Body(format!("{error:?}")),
    )?;
    let eye_group_address: GroupAddress = request
        .eye_group
        .into();
    let eye_group = group(
        &character,
        eye_group_address,
    )?;
    let eye_sources = request
        .eye_frame_paths
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
    let eye_sources: [_; 4] = eye_sources
        .try_into()
        .map_err(
            |_frames: Vec<_>| SemanticTextureArtifactError::EyeFrameCount,
        )?;
    let eye = analyze_eye_frames(
        eye_group,
        &eye_sources,
        request.eye_output_size,
    )
    .map_err(|error| SemanticTextureArtifactError::Eye(format!("{error:?}")))?;
    artifacts::assemble(
        request, &body, &eye,
    )
}

/// Decode one explicit input PNG through repository-owned I/O and PNG adapters.
fn decode_image(
    role: &str,
    path: &Path,
) -> Result<
    crate::domain::texture::semantic::RgbaImage,
    SemanticTextureArtifactError,
> {
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
