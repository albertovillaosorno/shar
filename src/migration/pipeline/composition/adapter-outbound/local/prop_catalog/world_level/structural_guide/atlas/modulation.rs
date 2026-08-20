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
//   - Modulation outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Modulation outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Modulation outbound adapter.

#![expect(
    clippy::arithmetic_side_effects,
    clippy::integer_division,
    // jig-ignore-next-line: exact syntax is indivisible
    reason = "Bounded integer modulation implements exact reviewed eight-bit rounding formulas."
)]

use crate::domain::PipelineError;

/// Bake one source image through material and vertex-color modulation.
pub(super) fn bake(
    source_pixels: &[[u8; 4]],
    width: u32,
    height: u32,
    material_tint: [u8; 4],
    vertex_tint: [u8; 4],
) -> Result<Vec<[u8; 3]>, PipelineError> {
    validate_pixel_count(source_pixels, width, height)?;
    Ok(source_pixels
        .iter()
        .map(|source| multiply_rgb(*source, material_tint, vertex_tint))
        .collect())
}

fn validate_pixel_count(
    source_pixels: &[[u8; 4]],
    width: u32,
    height: u32,
) -> Result<(), PipelineError> {
    if width == 0 || height == 0 {
        return Err(PipelineError::new(
            "structural-guide source texture has zero dimensions",
        ));
    }
    let expected = usize::try_from(u64::from(width) * u64::from(height))
        .map_err(|error| PipelineError::new(error.to_string()))?;
    if source_pixels.len() != expected {
        return Err(PipelineError::new(format!(
            "structural-guide source pixel count changed: expected \
                     {expected}, found {}",
            source_pixels.len()
        )));
    }
    Ok(())
}

fn multiply_rgb(
    source: [u8; 4],
    material: [u8; 4],
    vertex: [u8; 4],
) -> [u8; 3] {
    let [source_red, source_green, source_blue, source_alpha] = source;
    let [material_red, material_green, material_blue, material_alpha] =
        material;
    let [vertex_red, vertex_green, vertex_blue, vertex_alpha] = vertex;
    [
        multiply_channel(
            source_red,
            material_red,
            vertex_red,
            source_alpha,
            material_alpha,
            vertex_alpha,
        ),
        multiply_channel(
            source_green,
            material_green,
            vertex_green,
            source_alpha,
            material_alpha,
            vertex_alpha,
        ),
        multiply_channel(
            source_blue,
            material_blue,
            vertex_blue,
            source_alpha,
            material_alpha,
            vertex_alpha,
        ),
    ]
}

fn multiply_channel(
    source: u8,
    material: u8,
    vertex: u8,
    source_alpha: u8,
    material_alpha: u8,
    vertex_alpha: u8,
) -> u8 {
    let product = u64::from(source)
        .saturating_mul(u64::from(material))
        .saturating_mul(u64::from(vertex))
        .saturating_mul(u64::from(source_alpha))
        .saturating_mul(u64::from(material_alpha))
        .saturating_mul(u64::from(vertex_alpha));
    let denominator = u64::from(u8::MAX).pow(5);
    u8::try_from(product / denominator).unwrap_or(u8::MAX)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/structural_guide/atlas/modulation/tests.rs"]
mod tests;
