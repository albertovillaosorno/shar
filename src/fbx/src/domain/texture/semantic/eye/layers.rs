// File:
//   - layers.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/eye/layers.rs
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
//   - Deterministic construction of canonical sclera, pupil, and lid textures.
// - Must-Not:
//   - Change eye geometry, animation, frame order, or access files.
// - Allows:
//   - Exact-color masks and transparent RGBA layer construction.
// - Split-When:
//   - One eye layer gains an independent generation contract.
// - Merge-When:
//   - Frame analysis owns the same canonical layer construction.
// - Summary:
//   - Canonical three-texture eye layer generation.
// - Description:
//   - Derives editable eye SSOT layers while retaining compatibility frames.
// - Usage:
//   - Called after validated frame analysis and nearest-neighbor scaling.
// - Defaults:
//   - The lid texture is split evenly into upper and lower semantic halves.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Canonical three-texture eye layer generation.
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
    let composite = RgbaImage::filled(
        width,
        height,
        Rgba8::new(
            255, 255, 255, 255,
        ),
    )?;
    let transparent = Rgba8::new(
        0, 0, 0, 0,
    );
    let pupil_pixels = open_frame
        .pixels()
        .iter()
        .map(
            |color| {
                if *color == pupil_color {
                    pupil_color
                } else {
                    transparent
                }
            },
        )
        .collect();
    let pupil = RgbaImage::new(
        width,
        height,
        pupil_pixels,
    )?;
    let lids = RgbaImage::filled(
        width, height, lid_color,
    )?;
    Ok(
        EyeTextureLayers {
            composite,
            pupil,
            lids,
        },
    )
}
