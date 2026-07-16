// File:
//   - image.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/image.rs
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
//   - Checked top-left-origin RGBA image storage and pixel access.
// - Must-Not:
//   - Decode files, sample UVs, scale images, or classify semantic regions.
// - Allows:
//   - Exact construction, fill, indexed access, mutation, and byte flattening.
// - Split-When:
//   - Split when mutable image operations need an independent contract.
// - Merge-When:
//   - Another texture-domain module owns the same checked image storage.
// - Summary:
//   - File-container-independent RGBA image storage.
// - Description:
//   - Preserves exact pixels behind checked dimensions and coordinates.
// - Usage:
//   - Extended by sampling behavior and consumed by PNG adapters.
// - Defaults:
//   - Pixels use row-major top-left-origin order.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: construction, checked indexing, mutation, and flattening enforce
//   - one image-storage invariant set.
//

//! Checked file-container-independent RGBA image storage.
#![expect(
    clippy::indexing_slicing,
    clippy::module_name_repetitions,
    clippy::shadow_reuse,
    reason = "Validated image dimensions bound storage access; explicit names \
              preserve the domain boundary."
)]

use super::color::Rgba8;

/// Image value validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RgbaImageError {
    /// Width or height was zero.
    ZeroDimensions,
    /// Width and height could not be represented as one pixel count.
    PixelCountOverflow,
    /// Caller-provided pixels did not match the declared dimensions.
    PixelCountMismatch {
        /// Expected number of pixels.
        expected: usize,
        /// Actual number of pixels.
        actual: usize,
    },
    /// Pixel coordinate was outside the image.
    PixelOutOfBounds {
        /// Requested horizontal coordinate.
        x: u32,
        /// Requested vertical coordinate.
        y: u32,
    },
    /// UV coordinate was non-finite or outside zero through one.
    InvalidUv,
}

/// One top-left-origin RGBA image independent from a file container.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RgbaImage {
    /// Image width in pixels.
    pub(super) width: u32,
    /// Image height in pixels.
    pub(super) height: u32,
    /// Row-major top-left-origin RGBA pixel storage.
    pub(super) pixels: Vec<Rgba8>,
}

impl RgbaImage {
    /// Build an image from exact dimensions and row-major pixels.
    ///
    /// # Errors
    ///
    /// Returns an error when dimensions are zero, overflow, or disagree with
    /// the supplied pixel count.
    pub fn new(
        width: u32,
        height: u32,
        pixels: Vec<Rgba8>,
    ) -> Result<Self, RgbaImageError> {
        let expected = checked_pixel_count(
            width, height,
        )?;
        if pixels.len() != expected {
            return Err(
                RgbaImageError::PixelCountMismatch {
                    expected,
                    actual: pixels.len(),
                },
            );
        }
        Ok(
            Self {
                width,
                height,
                pixels,
            },
        )
    }

    /// Build an image filled with one exact color.
    ///
    /// # Errors
    ///
    /// Returns an error when dimensions are zero or overflow.
    pub fn filled(
        width: u32,
        height: u32,
        color: Rgba8,
    ) -> Result<Self, RgbaImageError> {
        let count = checked_pixel_count(
            width, height,
        )?;
        Self::new(
            width,
            height,
            vec![color; count],
        )
    }

    /// Return the image width.
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// Return the image height.
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// Return the immutable row-major pixel storage.
    #[must_use]
    pub fn pixels(&self) -> &[Rgba8] {
        &self.pixels
    }

    /// Return one exact pixel.
    ///
    /// # Errors
    ///
    /// Returns an error when the coordinate is outside the image.
    pub fn pixel(
        &self,
        x: u32,
        y: u32,
    ) -> Result<Rgba8, RgbaImageError> {
        let index = self.pixel_index(
            x, y,
        )?;
        Ok(self.pixels[index])
    }

    /// Replace one exact pixel.
    ///
    /// # Errors
    ///
    /// Returns an error when the coordinate is outside the image.
    pub fn set_pixel(
        &mut self,
        x: u32,
        y: u32,
        color: Rgba8,
    ) -> Result<(), RgbaImageError> {
        let index = self.pixel_index(
            x, y,
        )?;
        self.pixels[index] = color;
        Ok(())
    }

    /// Flatten pixels into deterministic RGBA file order.
    #[must_use]
    pub fn rgba_bytes(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(|color| color.channels())
            .collect()
    }

    /// Resolve one row-major pixel index.
    pub(super) fn pixel_index(
        &self,
        x: u32,
        y: u32,
    ) -> Result<usize, RgbaImageError> {
        if x >= self.width || y >= self.height {
            return Err(
                RgbaImageError::PixelOutOfBounds {
                    x,
                    y,
                },
            );
        }
        let row = usize::try_from(y)
            .map_err(|_error| RgbaImageError::PixelCountOverflow)?;
        let width = usize::try_from(self.width)
            .map_err(|_error| RgbaImageError::PixelCountOverflow)?;
        let column = usize::try_from(x)
            .map_err(|_error| RgbaImageError::PixelCountOverflow)?;
        row.checked_mul(width)
            .and_then(|offset| offset.checked_add(column))
            .ok_or(RgbaImageError::PixelCountOverflow)
    }
}

/// Validate and calculate one image pixel count.
pub(super) fn checked_pixel_count(
    width: u32,
    height: u32,
) -> Result<usize, RgbaImageError> {
    if width == 0 || height == 0 {
        return Err(RgbaImageError::ZeroDimensions);
    }
    let width = usize::try_from(width)
        .map_err(|_error| RgbaImageError::PixelCountOverflow)?;
    let height = usize::try_from(height)
        .map_err(|_error| RgbaImageError::PixelCountOverflow)?;
    width
        .checked_mul(height)
        .ok_or(RgbaImageError::PixelCountOverflow)
}
