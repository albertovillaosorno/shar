// File:
//   - triangles.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/classification/triangles.rs
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
//   - Exact source-color and semantic-region uniformity checks per triangle.
// - Must-Not:
//   - Reclassify vertices, duplicate topology, or repair mixed triangles.
// - Allows:
//   - Checked triangle index conversion and fail-closed diagnostics.
// - Split-When:
//   - Patterned triangles gain a separate deterministic resampling contract.
// - Merge-When:
//   - Primitive-group validation owns semantic triangle uniformity.
// - Summary:
//   - Flat-color triangle acceptance boundary.
// - Description:
//   - Proves the first semantic atlas lane needs no vertex duplication.
// - Usage:
//   - Called after every selected vertex has a color and region.
// - Defaults:
//   - Mixed source colors or semantic regions are rejected.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Flat-color triangle acceptance boundary.
use super::super::super::color::Rgba8;
use super::super::super::region::BodyRegion;
use super::super::error::SemanticTextureError;
use super::super::recipe::GroupAddress;
use crate::domain::mesh::PrimitiveGroup;

/// Reject any triangle that would require semantic or color interpolation.
pub(super) fn validate(
    address: GroupAddress,
    group: &PrimitiveGroup,
    colors: &[Rgba8],
    regions: &[BodyRegion],
) -> Result<(), SemanticTextureError> {
    for (triangle, indices) in group
        .triangles
        .iter()
        .enumerate()
    {
        let vertices = [
            index(indices[0])?,
            index(indices[1])?,
            index(indices[2])?,
        ];
        let triangle_colors = [
            value(
                colors,
                vertices[0],
            )?,
            value(
                colors,
                vertices[1],
            )?,
            value(
                colors,
                vertices[2],
            )?,
        ];
        if triangle_colors[0] != triangle_colors[1]
            || triangle_colors[0] != triangle_colors[2]
        {
            return Err(
                SemanticTextureError::MixedSourceColorTriangle {
                    group: address,
                    triangle,
                },
            );
        }
        let triangle_regions = [
            value(
                regions,
                vertices[0],
            )?,
            value(
                regions,
                vertices[1],
            )?,
            value(
                regions,
                vertices[2],
            )?,
        ];
        if triangle_regions[0] != triangle_regions[1]
            || triangle_regions[0] != triangle_regions[2]
        {
            return Err(
                SemanticTextureError::MixedSemanticTriangle {
                    group: address,
                    triangle,
                },
            );
        }
    }
    Ok(())
}

/// Convert one domain vertex index into the host index type.
fn index(value: u32) -> Result<usize, SemanticTextureError> {
    usize::try_from(value)
        .map_err(|_error| SemanticTextureError::NumericOverflow)
}

/// Resolve one checked per-vertex classification value.
fn value<T: Copy>(
    values: &[T],
    vertex: usize,
) -> Result<T, SemanticTextureError> {
    values
        .get(vertex)
        .copied()
        .ok_or(SemanticTextureError::NumericOverflow)
}
