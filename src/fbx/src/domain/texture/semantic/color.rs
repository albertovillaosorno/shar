// File:
//   - color.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/color.rs
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
//   - Exact RGBA values and sRGB-to-linear-light conversion.
// - Must-Not:
//   - Store images, sample UVs, read files, or classify semantic regions.
// - Allows:
//   - Deterministic color-channel access and relative luminance calculation.
// - Split-When:
//   - Split when another color space becomes an independent public contract.
// - Merge-When:
//   - Another texture-domain module owns the same color conversion invariant.
// - Summary:
//   - Repository-owned color values for semantic texture preparation.
// - Description:
//   - Keeps color-space behavior independent from image and PNG adapters.
// - Usage:
//   - Used by semantic body and eye planners and RGBA images.
// - Defaults:
//   - RGB channels are encoded as sRGB and alpha is straight eight-bit data.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Exact RGBA and linear-light color values.
/// One eight-bit sRGB color with straight alpha.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Rgba8 {
    /// Red sRGB channel.
    pub red: u8,
    /// Green sRGB channel.
    pub green: u8,
    /// Blue sRGB channel.
    pub blue: u8,
    /// Straight alpha channel.
    pub alpha: u8,
}

impl Rgba8 {
    /// Build one exact eight-bit RGBA color.
    #[must_use]
    pub const fn new(
        red: u8,
        green: u8,
        blue: u8,
        alpha: u8,
    ) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Return the four channels in file order.
    #[must_use]
    pub const fn channels(self) -> [u8; 4] {
        [
            self.red, self.green, self.blue, self.alpha,
        ]
    }

    /// Convert the sRGB channels to linear light without changing alpha.
    #[must_use]
    pub fn linear_rgb(self) -> LinearRgb {
        LinearRgb {
            red: srgb_channel_to_linear(self.red),
            green: srgb_channel_to_linear(self.green),
            blue: srgb_channel_to_linear(self.blue),
        }
    }

    /// Return the relative linear-light luminance.
    #[must_use]
    pub fn relative_luminance(self) -> f32 {
        let linear = self.linear_rgb();
        linear
            .red
            .mul_add(
                0.2126,
                linear
                    .green
                    .mul_add(
                        0.7152,
                        linear.blue * 0.0722,
                    ),
            )
    }
}

/// One normalized linear-light RGB value.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LinearRgb {
    /// Linear red channel from zero through one.
    pub red: f32,
    /// Linear green channel from zero through one.
    pub green: f32,
    /// Linear blue channel from zero through one.
    pub blue: f32,
}

/// Convert one eight-bit sRGB channel into linear light.
fn srgb_channel_to_linear(channel: u8) -> f32 {
    let normalized = f32::from(channel) / 255.0;
    if normalized <= 0.04045 {
        normalized / 12.92
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4)
    }
}
