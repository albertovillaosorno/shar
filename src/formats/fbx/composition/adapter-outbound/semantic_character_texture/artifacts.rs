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
//   - Artifacts outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Artifacts outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Artifacts outbound adapter.

use shar_sha256::digest_hex;

use super::{
    ExternalTextureArtifact, SemanticTextureArtifactError,
    SemanticTextureArtifacts, SemanticTextureRequest, SemanticTextureSummary,
    manifest,
};
use crate::adapters::driven::semantic_texture_png::encode_png_bytes;
use crate::domain::texture::semantic::{BodyTexturePlan, EyeSemanticPlan};

/// Encode images, render the manifest, and assemble one complete byte bundle.
pub(super) fn assemble(
    request: &SemanticTextureRequest,
    body: &BodyTexturePlan,
    eye: Option<&EyeSemanticPlan>,
    animation_count: usize,
    extra_textures: Vec<ExternalTextureArtifact>,
) -> Result<SemanticTextureArtifacts, SemanticTextureArtifactError> {
    let body_texture_png = encode_png_bytes(&body.atlas).map_err(|error| {
        SemanticTextureArtifactError::Png(format!("{error:?}"))
    })?;
    let (eye_layer_pngs, eye_profile_sha256) = if let Some(eye_plan) = eye {
        let eye_pngs = [
            &eye_plan.layers.composite,
            &eye_plan.layers.pupil,
            &eye_plan.layers.lids,
        ]
        .into_iter()
        .map(encode_png_bytes)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| {
            SemanticTextureArtifactError::Png(format!("{error:?}"))
        })?;
        let layers: [Vec<u8>; 3] =
            eye_pngs.try_into().map_err(|_layers: Vec<_>| {
                SemanticTextureArtifactError::EyeLayerCount
            })?;
        let profile = eye_profile_sha256(&layers, eye_plan.surface_color);
        (Some(layers), Some(profile))
    } else {
        (None, None)
    };
    let manifest_json =
        manifest::render(request, body, eye, eye_profile_sha256.as_deref())
            .map_err(SemanticTextureArtifactError::Manifest)?;
    Ok(SemanticTextureArtifacts {
        body_texture_png,
        eye_layer_pngs,
        eye_profile_sha256,
        extra_textures,
        manifest_json,
        summary: SemanticTextureSummary {
            character_id: request.character_name.trim().to_owned(),
            body_vertex_count: body.source_vertex_count,
            body_triangle_count: body.source_triangle_count,
            body_chart_count: body.charts.len(),
            eye_region_count: eye
                .map_or(0, |eye_plan| eye_plan.semantic_region_count),
            animation_count,
            body_texture_size: [body.atlas.width(), body.atlas.height()],
            eye_frame_size: eye
                .map(|eye_plan| eye_plan.modern_frames[0].width()),
        },
    })
}

/// Hash the sclera color plus independent pupil and lid textures.
fn eye_profile_sha256(
    layers: &[Vec<u8>; 3],
    sclera_color: crate::domain::texture::semantic::Rgba8,
) -> String {
    let mut evidence = Vec::new();
    evidence.extend_from_slice(b"sclera-rgba");
    evidence.push(0);
    evidence.extend_from_slice(&sclera_color.channels());
    evidence.push(0xff);
    for (role, bytes) in [("pupil", &layers[1]), ("lids", &layers[2])] {
        evidence.extend_from_slice(role.as_bytes());
        evidence.push(0);
        evidence.extend_from_slice(bytes);
        evidence.push(0xff);
    }
    digest_hex(&evidence)
}
