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
//   - Layers domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Layers domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Layers domain module.

use super::super::color::Rgba8;
use super::super::image::RgbaImage;
use super::types::{EyeTextureError, EyeTextureLayers};

/// Build the open-eye compatibility texture, pupil layer, and lid atlas.
pub(super) fn build(
    open_frame: &RgbaImage,
    pupil_color: Rgba8,
    lid_color: Rgba8,
) -> Result<EyeTextureLayers, EyeTextureError> {
    let width = open_frame.width();
    let height = open_frame.height();
    let composite =
        RgbaImage::filled(width, height, Rgba8::new(255, 255, 255, 255))?;
    let transparent = Rgba8::new(0, 0, 0, 0);
    let pupil_pixels = open_frame
        .pixels()
        .iter()
        .map(|color| {
            if *color == pupil_color {
                pupil_color
            } else {
                transparent
            }
        })
        .collect();
    let pupil = RgbaImage::new(width, height, pupil_pixels)?;
    let lids = RgbaImage::filled(width, height, lid_color)?;
    Ok(EyeTextureLayers { composite, pupil, lids })
}
