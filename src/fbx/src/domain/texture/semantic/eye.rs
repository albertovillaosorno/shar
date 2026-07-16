// File:
//   - eye.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/eye.rs
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
//   - The public pure-domain facade for two-eye component and four-frame
//   - texture animation analysis.
// - Must-Not:
//   - Invent eyelid geometry, change animation mechanisms, read files, or
//   - encode output artifacts.
// - Allows:
//   - Focused component, frame, and result modules.
// - Split-When:
//   - Another eye-animation mechanism needs an independent public contract.
// - Merge-When:
//   - The parent semantic facade can expose the same API directly.
// - Summary:
//   - Evidence-driven semantic eye analysis facade.
// - Description:
//   - Preserves source texture-frame blinking while exposing eight semantic eye
//   - regions across two connected mesh components.
// - Usage:
//   - Called by repository-owned character texture artifact adapters.
// - Defaults:
//   - Exactly two eye components and four monotonic closure frames are
//   - required.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Evidence-driven semantic eye analysis.
use super::image::RgbaImage;
use crate::domain::mesh::PrimitiveGroup;

#[path = "eye/components.rs"]
mod components;
#[path = "eye/frames.rs"]
mod frames;
#[path = "eye/layers.rs"]
mod layers;
#[path = "eye/types.rs"]
mod types;

pub use types::{
    EyeComponent, EyeFrameEvidence, EyeRegion, EyeSemanticPlan, EyeSide,
    EyeTextureError, EyeTextureLayers,
};

/// Analyze two eye mesh components and the source four-frame blink sequence.
///
/// # Errors
///
/// Returns an error when component identity, frame dimensions, closure order,
/// pupil preservation, or output dimensions are unsupported.
pub fn analyze_eye_frames(
    group: &PrimitiveGroup,
    source_frames: &[RgbaImage; 4],
    output_size: u32,
) -> Result<EyeSemanticPlan, EyeTextureError> {
    let components = components::discover(group)?;
    let analyzed = frames::analyze(
        source_frames,
        output_size,
    )?;
    let layers = layers::build(
        &analyzed.modern_frames[0],
        analyzed.pupil_color,
        analyzed.lid_color,
    )?;
    Ok(
        EyeSemanticPlan {
            components,
            frame_evidence: analyzed.evidence,
            modern_frames: analyzed.modern_frames,
            layers,
            lid_color: analyzed.lid_color,
            surface_color: analyzed.surface_color,
            pupil_color: analyzed.pupil_color,
            semantic_region_count: 8,
        },
    )
}
