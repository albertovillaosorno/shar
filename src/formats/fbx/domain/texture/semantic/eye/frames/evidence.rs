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
//   - Evidence domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Evidence domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Evidence domain module.

#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    reason = "Validated frame dimensions bound evidence accumulation and \
              private carriers."
)]

use std::collections::{BTreeMap, BTreeSet};

use super::super::super::color::Rgba8;
use super::super::super::image::RgbaImage;
use super::super::types::{EyeFrameEvidence, EyeTextureError};
use super::closure;

/// Complete source evidence consumed by frame modernization.
pub(super) struct SourceEvidence {
    /// Ordered source-frame measurements used by modernization.
    pub(super) frames: Vec<EyeFrameEvidence>,
    /// Dominant eyelid color derived from source evidence.
    pub(super) lid_color: Rgba8,
    /// Dominant eye-surface color derived from source evidence.
    pub(super) surface_color: Rgba8,
    /// Dominant pupil color derived from source evidence.
    pub(super) pupil_color: Rgba8,
}

/// Analyze all source-frame evidence before any image scaling.
pub(super) fn analyze(
    frames: &[RgbaImage; 4],
    output_size: u32,
) -> Result<SourceEvidence, EyeTextureError> {
    validate_dimensions(frames, output_size)?;
    let (lid_color, surface_color, pupil_indices, pupil_color) =
        source_palette(frames)?;
    let lid_sets = frames
        .iter()
        .map(|frame| exact_color_indices(frame.pixels(), lid_color))
        .collect::<Vec<_>>();
    closure::validate(&lid_sets, frames[0].width(), frames[0].height())?;
    for frame_index in [1_usize, 2] {
        if pupil_indices.iter().any(|index| {
            let pixel = frames[frame_index].pixels()[*index];
            pixel != frames[0].pixels()[*index] && pixel != lid_color
        }) {
            return Err(EyeTextureError::PupilChangedBeforeClosure {
                frame: frame_index,
            });
        }
    }
    let evidence = lid_sets
        .iter()
        .enumerate()
        .map(|(frame_index, indices)| {
            closure::frame_evidence(
                frame_index,
                indices,
                &pupil_indices,
                frames,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(SourceEvidence {
        frames: evidence,
        lid_color,
        surface_color,
        pupil_color,
    })
}

/// Resolve closed-lid, sclera, and pupil evidence from source frames.
fn source_palette(
    frames: &[RgbaImage; 4],
) -> Result<(Rgba8, Rgba8, BTreeSet<usize>, Rgba8), EyeTextureError> {
    let closed_pixels = frames[3].pixels();
    let lid_color = closed_pixels
        .first()
        .copied()
        .ok_or(EyeTextureError::InvalidFrameDimensions)?;
    if closed_pixels.iter().any(|color| *color != lid_color) {
        return Err(EyeTextureError::NonUniformClosedFrame);
    }
    let open_counts = color_counts(frames[0].pixels());
    let surface_color = open_counts
        .iter()
        .filter(|(color, _count)| **color != lid_color)
        .max_by(|left, right| {
            left.1.cmp(right.1).then_with(|| right.0.cmp(left.0))
        })
        .map(|(color, _count)| *color)
        .ok_or(EyeTextureError::MissingEyeSurface)?;
    let pupil_indices =
        pupil_indices(frames[0].pixels(), lid_color, surface_color)?;
    let pupil_color = pupil_indices
        .iter()
        .map(|index| frames[0].pixels()[*index])
        .min_by(|left, right| {
            left.relative_luminance()
                .total_cmp(&right.relative_luminance())
                .then_with(|| left.cmp(right))
        })
        .ok_or(EyeTextureError::MissingPupilEvidence)?;
    Ok((lid_color, surface_color, pupil_indices, pupil_color))
}

/// Validate shared, even source dimensions and a non-downsampling output size.
fn validate_dimensions(
    frames: &[RgbaImage; 4],
    output_size: u32,
) -> Result<(), EyeTextureError> {
    let width = frames[0].width();
    let height = frames[0].height();
    if width == 0 || height < 2 || !height.is_multiple_of(2) {
        return Err(EyeTextureError::InvalidFrameDimensions);
    }
    if frames
        .iter()
        .any(|frame| frame.width() != width || frame.height() != height)
    {
        return Err(EyeTextureError::FrameDimensionMismatch);
    }
    if output_size < width.max(height) {
        return Err(EyeTextureError::InvalidOutputSize);
    }
    Ok(())
}

/// Count exact colors in deterministic color order.
fn color_counts(pixels: &[Rgba8]) -> BTreeMap<Rgba8, usize> {
    let mut counts = BTreeMap::new();
    for color in pixels {
        *counts.entry(*color).or_default() += 1;
    }
    counts
}

/// Discover dark pupil or iris pixels relative to the dominant eye surface.
fn pupil_indices(
    pixels: &[Rgba8],
    lid_color: Rgba8,
    surface_color: Rgba8,
) -> Result<BTreeSet<usize>, EyeTextureError> {
    let threshold = surface_color.relative_luminance() * 0.15;
    let indices = pixels
        .iter()
        .enumerate()
        .filter(|(_index, color)| {
            **color != lid_color && color.relative_luminance() <= threshold
        })
        .map(|(index, _color)| index)
        .collect::<BTreeSet<_>>();
    if indices.is_empty() {
        return Err(EyeTextureError::MissingPupilEvidence);
    }
    if indices.len().saturating_mul(4) > pixels.len() {
        return Err(EyeTextureError::ExcessivePupilEvidence);
    }
    Ok(indices)
}

/// Return exact indices matching one color.
fn exact_color_indices(pixels: &[Rgba8], color: Rgba8) -> BTreeSet<usize> {
    pixels
        .iter()
        .enumerate()
        .filter(|(_index, pixel)| **pixel == color)
        .map(|(index, _pixel)| index)
        .collect()
}
