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
//   - Frames domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Frames domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Frames domain module.

use super::super::color::Rgba8;
use super::super::image::RgbaImage;
use super::types::{EyeFrameEvidence, EyeTextureError};

#[path = "frames/closure.rs"]
mod closure;
#[path = "frames/evidence.rs"]
mod evidence;

/// Complete internal frame analysis returned to the eye facade.
pub(super) struct AnalyzedFrames {
    /// Ordered evidence records for the four authored eye frames.
    pub(super) evidence: Vec<EyeFrameEvidence>,
    /// Modernized open-through-closed frames in canonical order.
    pub(super) modern_frames: [RgbaImage; 4],
    /// Dominant authored eyelid color.
    pub(super) lid_color: Rgba8,
    /// Dominant authored sclera or eye-surface color.
    pub(super) surface_color: Rgba8,
    /// Dominant authored pupil color.
    pub(super) pupil_color: Rgba8,
}

/// Analyze exact source frames and create modernized nearest-neighbor outputs.
pub(super) fn analyze(
    frames: &[RgbaImage; 4],
    output_size: u32,
) -> Result<AnalyzedFrames, EyeTextureError> {
    let source = evidence::analyze(frames, output_size)?;
    let modern_frames = [
        frames[0].scale_nearest(output_size, output_size)?,
        frames[1].scale_nearest(output_size, output_size)?,
        frames[2].scale_nearest(output_size, output_size)?,
        frames[3].scale_nearest(output_size, output_size)?,
    ];
    Ok(AnalyzedFrames {
        evidence: source.frames,
        modern_frames,
        lid_color: source.lid_color,
        surface_color: source.surface_color,
        pupil_color: source.pupil_color,
    })
}
