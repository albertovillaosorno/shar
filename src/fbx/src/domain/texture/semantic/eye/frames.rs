// File:
//   - frames.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/eye/frames.rs
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
//   - Ordered source-evidence analysis and deterministic four-frame scaling.
// - Must-Not:
//   - Reimplement source color or closure rules, change frame order, or access
//   - files.
// - Allows:
//   - Delegation to focused evidence modules and nearest-neighbor scaling.
// - Split-When:
//   - Another source-supported frame modernization policy is introduced.
// - Merge-When:
//   - The eye facade can own orchestration without duplicating frame behavior.
// - Summary:
//   - Four-frame eye texture modernization orchestration.
// - Description:
//   - Validates source evidence before scaling every authoritative frame.
// - Usage:
//   - Called by the semantic eye facade after component discovery.
// - Defaults:
//   - Source frame order and pixel ownership remain unchanged.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Four-frame eye texture modernization orchestration.

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
    let source = evidence::analyze(
        frames,
        output_size,
    )?;
    let modern_frames = [
        frames[0].scale_nearest(
            output_size,
            output_size,
        )?,
        frames[1].scale_nearest(
            output_size,
            output_size,
        )?,
        frames[2].scale_nearest(
            output_size,
            output_size,
        )?,
        frames[3].scale_nearest(
            output_size,
            output_size,
        )?,
    ];
    Ok(
        AnalyzedFrames {
            evidence: source.frames,
            modern_frames,
            lid_color: source.lid_color,
            surface_color: source.surface_color,
            pupil_color: source.pupil_color,
        },
    )
}
