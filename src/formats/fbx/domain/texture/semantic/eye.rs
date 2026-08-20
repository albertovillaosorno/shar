// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Eye domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Eye domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Eye domain module.

#![expect(
    clippy::module_name_repetitions,
    reason = "Eye semantic types retain explicit names at the public domain \
              boundary."
)]

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
    let analyzed = frames::analyze(source_frames, output_size)?;
    let layers = layers::build(
        &analyzed.modern_frames[0],
        analyzed.pupil_color,
        analyzed.lid_color,
    )?;
    Ok(EyeSemanticPlan {
        components,
        frame_evidence: analyzed.evidence,
        modern_frames: analyzed.modern_frames,
        layers,
        lid_color: analyzed.lid_color,
        surface_color: analyzed.surface_color,
        pupil_color: analyzed.pupil_color,
        semantic_region_count: 8,
    })
}
