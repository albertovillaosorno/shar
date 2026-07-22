// File:
//   - modulation.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     structural_guide/atlas/modulation.rs
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
//   - Opaque texture, material-color, and source-wide vertex-color baking.
// - Must-Not:
//   - Pack rectangles, change UV0, resize images, or serialize files.
// - Allows:
//   - Straight-alpha flattening and deterministic RGB multiplication.
// - Summary:
//   - Produces one bounded RGB atlas variant per material/wrap identity.
//
// Large file:
//   - false

//! Visible-color modulation for structural-guide atlas variants.

use crate::domain::PipelineError;

/// Bake one source image through material and vertex-color modulation.
pub(super) fn bake(
    source_pixels: &[[u8; 4]],
    width: u32,
    height: u32,
    material_tint: [u8; 4],
    vertex_tint: [u8; 4],
) -> Result<Vec<[u8; 3]>, PipelineError> {
    validate_pixel_count(
        source_pixels,
        width,
        height,
    )?;
    Ok(
        source_pixels
            .iter()
            .map(
                |source| {
                    multiply_rgb(
                        *source,
                        material_tint,
                        vertex_tint,
                    )
                },
            )
            .collect(),
    )
}

fn validate_pixel_count(
    source_pixels: &[[u8; 4]],
    width: u32,
    height: u32,
) -> Result<(), PipelineError> {
    if width == 0 || height == 0 {
        return Err(
            PipelineError::new(
                "structural-guide source texture has zero dimensions",
            ),
        );
    }
    let expected = usize::try_from(u64::from(width) * u64::from(height))
        .map_err(|error| PipelineError::new(error.to_string()))?;
    if source_pixels.len() != expected {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide source pixel count changed: expected \
                     {expected}, found {}",
                    source_pixels.len()
                ),
            ),
        );
    }
    Ok(())
}

fn multiply_rgb(
    source: [u8; 4],
    material: [u8; 4],
    vertex: [u8; 4],
) -> [u8; 3] {
    let [
        source_red,
        source_green,
        source_blue,
        source_alpha,
    ] = source;
    let [
        material_red,
        material_green,
        material_blue,
        material_alpha,
    ] = material;
    let [
        vertex_red,
        vertex_green,
        vertex_blue,
        vertex_alpha,
    ] = vertex;
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
mod tests {
    use super::*;

    #[test]
    fn material_average_tint_is_baked_into_opaque_rgb() {
        let result = bake(
            &[
                [
                    255, 128, 64, 128,
                ],
            ],
            1,
            1,
            [
                255, 128, 255, 255,
            ],
            [
                128, 255, 64, 255,
            ],
        );
        assert_eq!(
            result,
            Ok(
                vec![
                    [
                        64, 32, 8
                    ]
                ]
            )
        );
    }

    #[test]
    fn source_pixel_count_must_match_dimensions() {
        let result = bake(
            &[],
            1,
            1,
            [255; 4],
            [255; 4],
        );
        assert!(result.is_err());
    }
}
