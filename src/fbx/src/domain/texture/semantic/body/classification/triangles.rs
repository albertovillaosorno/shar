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
//   - Triangle chart ownership and patterned-detail acceptance.
// - Must-Not:
//   - Duplicate vertices, alter topology, or guess an unanchored pattern.
// - Allows:
//   - Attaching one isolated mixed triangle to a uniform shared anchor chart.
// - Split-When:
//   - Patterned components require more than one shared anchor contract.
// - Merge-When:
//   - Primitive-group classification owns chart raster policy directly.
// - Summary:
//   - Flat and anchored-pattern triangle classification boundary.
// - Description:
//   - Preserves isolated source patterns without changing vertex ownership.
// - Usage:
//   - Called after every selected vertex has a color and semantic region.
// - Defaults:
//   - Mixed triangles fail unless exactly one safe anchor proves ownership.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Flat and anchored-pattern triangle classification boundary.
use super::super::super::color::Rgba8;
use super::super::super::region::BodyRegion;
use super::super::error::SemanticTextureError;
use super::super::recipe::GroupAddress;
use super::TriangleClassification;
use crate::domain::mesh::PrimitiveGroup;

/// Classify chart ownership and raster policy for every source triangle.
pub(super) fn classify(
    address: GroupAddress,
    group: &PrimitiveGroup,
    colors: &[Rgba8],
    regions: &[BodyRegion],
) -> Result<Vec<TriangleClassification>, SemanticTextureError> {
    let uses = vertex_use_counts(group)?;
    group
        .triangles
        .iter()
        .enumerate()
        .map(
            |(triangle, indices)| {
                classify_triangle(
                    address, triangle, *indices, group, colors, regions, &uses,
                )
            },
        )
        .collect()
}

/// Classify one triangle as flat or as a safe anchored source pattern.
fn classify_triangle(
    address: GroupAddress,
    triangle: usize,
    indices: [u32; 3],
    group: &PrimitiveGroup,
    colors: &[Rgba8],
    regions: &[BodyRegion],
    uses: &[u32],
) -> Result<TriangleClassification, SemanticTextureError> {
    let vertices = [
        index(indices[0])?,
        index(indices[1])?,
        index(indices[2])?,
    ];
    if let Some((region, color)) = uniform_key(
        vertices, colors, regions,
    )? {
        return Ok(
            TriangleClassification {
                color,
                region,
                sample_source: false,
            },
        );
    }
    let Some(anchor) = patterned_anchor(
        triangle, indices, vertices, group, colors, regions, uses,
    )?
    else {
        return Err(
            mixed_error(
                address, triangle, vertices, colors,
            )?,
        );
    };
    Ok(
        TriangleClassification {
            color: value(
                colors, anchor,
            )?,
            region: value(
                regions, anchor,
            )?,
            sample_source: true,
        },
    )
}

/// Resolve the only safe shared anchor for one isolated patterned triangle.
fn patterned_anchor(
    triangle: usize,
    indices: [u32; 3],
    vertices: [usize; 3],
    group: &PrimitiveGroup,
    colors: &[Rgba8],
    regions: &[BodyRegion],
    uses: &[u32],
) -> Result<Option<usize>, SemanticTextureError> {
    let shared = vertices
        .iter()
        .copied()
        .filter(
            |vertex| {
                value(
                    uses, *vertex,
                )
                .is_ok_and(|count| count > 1)
            },
        )
        .collect::<Vec<_>>();
    if shared.len() != 1 {
        return Ok(None);
    }
    let anchor = shared[0];
    if vertices
        .iter()
        .copied()
        .filter(|vertex| *vertex != anchor)
        .any(
            |vertex| {
                value(
                    uses, vertex,
                ) != Ok(1)
            },
        )
    {
        return Ok(None);
    }
    let anchor_color = value(
        colors, anchor,
    )?;
    let anchor_region = value(
        regions, anchor,
    )?;
    let anchor_raw = indices
        .iter()
        .copied()
        .find(|candidate| index(*candidate) == Ok(anchor))
        .ok_or(SemanticTextureError::NumericOverflow)?;
    let mut neighbor_count = 0_u32;
    for (neighbor, neighbor_indices) in group
        .triangles
        .iter()
        .enumerate()
    {
        if neighbor == triangle || !neighbor_indices.contains(&anchor_raw) {
            continue;
        }
        let neighbor_vertices = [
            index(neighbor_indices[0])?,
            index(neighbor_indices[1])?,
            index(neighbor_indices[2])?,
        ];
        let Some((region, color)) = uniform_key(
            neighbor_vertices,
            colors,
            regions,
        )?
        else {
            return Ok(None);
        };
        if region != anchor_region || color != anchor_color {
            return Ok(None);
        }
        neighbor_count = neighbor_count
            .checked_add(1)
            .ok_or(SemanticTextureError::NumericOverflow)?;
    }
    Ok((neighbor_count > 0).then_some(anchor))
}

/// Return one uniform semantic/color key or `None` for a mixed triangle.
fn uniform_key(
    vertices: [usize; 3],
    colors: &[Rgba8],
    regions: &[BodyRegion],
) -> Result<
    Option<(
        BodyRegion,
        Rgba8,
    )>,
    SemanticTextureError,
> {
    let color = value(
        colors,
        vertices[0],
    )?;
    let region = value(
        regions,
        vertices[0],
    )?;
    for vertex in &vertices[1..] {
        if value(
            colors, *vertex,
        )? != color
            || value(
                regions, *vertex,
            )? != region
        {
            return Ok(None);
        }
    }
    Ok(
        Some(
            (
                region, color,
            ),
        ),
    )
}

/// Count triangle ownership for every source vertex.
fn vertex_use_counts(
    group: &PrimitiveGroup
) -> Result<Vec<u32>, SemanticTextureError> {
    let mut uses = vec![
        0_u32;
        group
            .positions
            .len()
    ];
    for indices in &group.triangles {
        for raw in indices {
            let vertex = index(*raw)?;
            let count = uses
                .get_mut(vertex)
                .ok_or(SemanticTextureError::NumericOverflow)?;
            *count = count
                .checked_add(1)
                .ok_or(SemanticTextureError::NumericOverflow)?;
        }
    }
    Ok(uses)
}

/// Select the original fail-closed diagnostic for an unsafe mixed triangle.
fn mixed_error(
    address: GroupAddress,
    triangle: usize,
    vertices: [usize; 3],
    colors: &[Rgba8],
) -> Result<SemanticTextureError, SemanticTextureError> {
    let first = value(
        colors,
        vertices[0],
    )?;
    if vertices[1..]
        .iter()
        .copied()
        .any(
            |vertex| {
                value(
                    colors, vertex,
                )
                .is_ok_and(|color| color != first)
            },
        )
    {
        return Ok(
            SemanticTextureError::MixedSourceColorTriangle {
                group: address,
                triangle,
            },
        );
    }
    Ok(
        SemanticTextureError::MixedSemanticTriangle {
            group: address,
            triangle,
        },
    )
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
