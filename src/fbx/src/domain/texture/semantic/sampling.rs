// File:
//   - sampling.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/sampling.rs
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
//   - V-up UV sampling and deterministic nearest-neighbor image scaling.
// - Must-Not:
//   - Read or write files, change color values, or classify mesh semantics.
// - Allows:
//   - Checked coordinate conversion and exact source-texel selection.
// - Split-When:
//   - Split when another resampling filter becomes a supported contract.
// - Merge-When:
//   - RGBA image storage becomes the sole owner of sampling behavior.
// - Summary:
//   - Deterministic image sampling for semantic texture preparation.
// - Description:
//   - Bridges V-up mesh UVs to top-left-origin image rows without adapters.
// - Usage:
//   - Used by body classification and eye-frame modernization.
// - Defaults:
//   - Sampling uses source texel ownership and scaling uses nearest neighbor.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Deterministic V-up UV sampling and nearest-neighbor scaling.
use super::color::Rgba8;
use super::image::{RgbaImage, RgbaImageError, checked_pixel_count};

/// Texture-address behavior applied before source-texel selection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextureAddressMode {
    /// Reject coordinates outside the closed zero-through-one interval.
    Clamp,
    /// Repeat coordinates modulo one, matching tiled runtime sampling.
    Tile,
}

impl RgbaImage {
    /// Sample one V-up UV coordinate using source texel ownership.
    ///
    /// The image rows remain top-left-origin, so the V coordinate is inverted
    /// exactly once before selecting the source texel.
    ///
    /// # Errors
    ///
    /// Returns an error when either UV component is non-finite or outside the
    /// closed zero-through-one interval.
    pub fn sample_uv_v_up(
        &self,
        uv: [f32; 2],
    ) -> Result<Rgba8, RgbaImageError> {
        let x = unit_coordinate_to_index(
            f64::from(uv[0]),
            self.width,
        )?;
        let y = unit_coordinate_to_index(
            1.0 - f64::from(uv[1]),
            self.height,
        )?;
        self.pixel(
            x, y,
        )
    }

    /// Sample one V-up UV coordinate through an explicit address mode.
    ///
    /// # Errors
    ///
    /// Returns an error when either coordinate is non-finite or clamp mode
    /// receives a coordinate outside the closed zero-through-one interval.
    pub fn sample_uv_v_up_with_address_mode(
        &self,
        uv: [f32; 2],
        address_mode: TextureAddressMode,
    ) -> Result<Rgba8, RgbaImageError> {
        match address_mode {
            TextureAddressMode::Clamp => self.sample_uv_v_up(uv),
            TextureAddressMode::Tile => {
                let u = tiled_coordinate(f64::from(uv[0]))?;
                let v = tiled_coordinate(f64::from(uv[1]))?;
                let x = unit_coordinate_to_index(
                    u, self.width,
                )?;
                let y = unit_coordinate_to_index(
                    1.0 - v,
                    self.height,
                )?;
                self.pixel(
                    x, y,
                )
            }
        }
    }

    /// Scale the image with deterministic nearest-neighbor sampling.
    ///
    /// # Errors
    ///
    /// Returns an error when output dimensions are zero or overflow.
    pub fn scale_nearest(
        &self,
        width: u32,
        height: u32,
    ) -> Result<Self, RgbaImageError> {
        let count = checked_pixel_count(
            width, height,
        )?;
        let mut pixels = Vec::with_capacity(count);
        for y in 0..height {
            let source_y = proportional_index(
                y,
                height,
                self.height,
            );
            for x in 0..width {
                let source_x = proportional_index(
                    x, width, self.width,
                );
                pixels.push(
                    self.pixel(
                        source_x, source_y,
                    )?,
                );
            }
        }
        Self::new(
            width, height, pixels,
        )
    }
}

/// Normalize one finite coordinate for tiled texture addressing.
fn tiled_coordinate(value: f64) -> Result<f64, RgbaImageError> {
    if !value.is_finite() {
        return Err(RgbaImageError::InvalidUv);
    }
    Ok(value.rem_euclid(1.0))
}

/// Convert one unit coordinate into source-texel ownership.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "The value is finite, non-negative, floored, and bounded below \
              the declared image dimension before conversion."
)]
fn unit_coordinate_to_index(
    value: f64,
    size: u32,
) -> Result<u32, RgbaImageError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(RgbaImageError::InvalidUv);
    }
    let scaled = (value * f64::from(size)).floor();
    if scaled >= f64::from(size) {
        return Ok(size - 1);
    }
    Ok(scaled as u32)
}

/// Resolve a nearest-neighbor source coordinate without floating point.
fn proportional_index(
    destination: u32,
    destination_size: u32,
    source_size: u32,
) -> u32 {
    let numerator = u64::from(destination) * u64::from(source_size);
    let source = numerator / u64::from(destination_size);
    u32::try_from(source)
        .unwrap_or(source_size - 1)
        .min(source_size - 1)
}
