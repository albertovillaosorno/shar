// File:
//   - artifacts.rs
// Path:
//   - src/fbx/src/adapters/driven/semantic_character_texture/artifacts.rs
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
//   - Deterministic PNG encoding, manifest rendering, and final semantic
//   - texture artifact bundle assembly.
// - Must-Not:
//   - Read source files, classify character evidence, publish output, or mutate
//   - domain plans.
// - Allows:
//   - Repository-owned PNG encoding and immutable result projection.
// - Split-When:
//   - Body and eye artifact publication become independent transactions.
// - Merge-When:
//   - Another driven module owns the same final byte-bundle assembly.
// - Summary:
//   - Semantic character texture byte-bundle assembly.
// - Description:
//   - Converts complete body and eye plans into one atomic in-memory result.
// - Usage:
//   - Called after every source decode and pure semantic planning stage passes.
// - Defaults:
//   - Produces one body atlas, four eye frames, and one manifest.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Final semantic character texture byte-bundle assembly.
use super::{
    SemanticTextureArtifactError, SemanticTextureArtifacts,
    SemanticTextureRequest, SemanticTextureSummary, manifest,
};
use crate::adapters::driven::semantic_texture_png::encode_png_bytes;
use crate::domain::texture::semantic::{BodyTexturePlan, EyeSemanticPlan};

/// Encode images, render the manifest, and assemble one complete byte bundle.
pub(super) fn assemble(
    request: &SemanticTextureRequest,
    body: &BodyTexturePlan,
    eye: &EyeSemanticPlan,
) -> Result<SemanticTextureArtifacts, SemanticTextureArtifactError> {
    let body_atlas_png = encode_png_bytes(&body.atlas).map_err(
        |error| SemanticTextureArtifactError::Png(format!("{error:?}")),
    )?;
    let eye_pngs = eye
        .modern_frames
        .iter()
        .map(encode_png_bytes)
        .collect::<Result<Vec<_>, _>>()
        .map_err(
            |error| SemanticTextureArtifactError::Png(format!("{error:?}")),
        )?;
    let eye_frame_pngs: [_; 4] = eye_pngs
        .try_into()
        .map_err(
            |_frames: Vec<_>| SemanticTextureArtifactError::EyeFrameCount,
        )?;
    let manifest_json = manifest::render(
        request, body, eye,
    )
    .map_err(SemanticTextureArtifactError::Manifest)?;
    Ok(
        SemanticTextureArtifacts {
            body_atlas_png,
            eye_frame_pngs,
            manifest_json,
            summary: SemanticTextureSummary {
                character_id: request
                    .character_name
                    .trim()
                    .to_owned(),
                body_vertex_count: body.source_vertex_count,
                body_triangle_count: body.source_triangle_count,
                body_chart_count: body
                    .charts
                    .len(),
                eye_region_count: eye.semantic_region_count,
                body_atlas_size: [
                    body.atlas
                        .width(),
                    body.atlas
                        .height(),
                ],
                eye_frame_size: eye.modern_frames[0].width(),
            },
        },
    )
}
