// File:
//   - discovery.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/charts/discovery.rs
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
//   - Connected-component discovery for exact-color semantic body triangles.
// - Must-Not:
//   - Split vertices, change topology, choose atlas rectangles, or rasterize.
// - Allows:
//   - Stable chart identities, complete vertex coverage checks, and projection
//   - delegation.
// - Split-When:
//   - Edge-based seam discovery becomes distinct from shared-vertex components.
// - Merge-When:
//   - Projection selection becomes the sole owner of chart discovery.
// - Summary:
//   - Connected flat-color semantic chart discovery.
// - Description:
//   - Groups already uniform triangles by exact color, region, and
//   - connectivity.
// - Usage:
//   - Called after strict semantic classification succeeds.
// - Defaults:
//   - Every selected vertex must belong to exactly one discovered chart.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: keyed triangle grouping, connectivity, coverage, identity, and
//   - projection delegation form one chart-discovery transaction.
//

//! Connected flat-color semantic chart discovery.
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::super::super::color::Rgba8;
use super::super::super::region::BodyRegion;
use super::super::classification::{
    Classification, GroupClassification, selected_group,
};
use super::super::error::SemanticTextureError;
use super::super::recipe::GroupAddress;
use super::model::ProjectedChart;
use super::projection;
use crate::domain::character::CharacterAsset;
use crate::domain::mesh::PrimitiveGroup;

/// Discover and project every selected connected semantic chart.
pub(super) fn discover(
    character: &CharacterAsset,
    classification: &Classification,
) -> Result<Vec<ProjectedChart>, SemanticTextureError> {
    let mut charts = Vec::new();
    for (address, group_classification) in &classification.groups {
        let (group, _influences) = selected_group(
            character, *address,
        )?;
        let discovered = discover_group(
            *address,
            group,
            group_classification,
        )?;
        charts.extend(discovered);
    }
    charts.sort_by(
        |left, right| {
            left.region
                .cmp(&right.region)
                .then_with(
                    || {
                        left.group
                            .cmp(&right.group)
                    },
                )
                .then_with(
                    || {
                        left.source_color
                            .cmp(&right.source_color)
                    },
                )
                .then_with(
                    || {
                        left.triangle_indices
                            .cmp(&right.triangle_indices)
                    },
                )
        },
    );
    Ok(charts)
}

/// Discover all connected charts in one selected primitive group.
fn discover_group(
    address: GroupAddress,
    group: &PrimitiveGroup,
    classification: &GroupClassification,
) -> Result<Vec<ProjectedChart>, SemanticTextureError> {
    let mut by_key: BTreeMap<
        (
            BodyRegion,
            Rgba8,
        ),
        Vec<usize>,
    > = BTreeMap::new();
    for (triangle, indices) in group
        .triangles
        .iter()
        .enumerate()
    {
        let vertex = usize::try_from(indices[0])
            .map_err(|_error| SemanticTextureError::NumericOverflow)?;
        let region = classification
            .regions
            .get(vertex)
            .copied()
            .ok_or(SemanticTextureError::NumericOverflow)?;
        let color = classification
            .colors
            .get(vertex)
            .copied()
            .ok_or(SemanticTextureError::NumericOverflow)?;
        by_key
            .entry(
                (
                    region, color,
                ),
            )
            .or_default()
            .push(triangle);
    }
    let mut charts = Vec::new();
    let mut covered_vertices = BTreeSet::new();
    for ((region, color), triangles) in by_key {
        for (ordinal, component) in connected_components(
            group, &triangles,
        )?
        .into_iter()
        .enumerate()
        {
            let vertices = component_vertices(
                group, &component,
            )?;
            for vertex in &vertices {
                if !covered_vertices.insert(*vertex) {
                    return Err(
                        SemanticTextureError::ConflictingVertexChart {
                            group: address,
                            vertex: *vertex,
                        },
                    );
                }
            }
            let id = chart_id(
                address, region, color, ordinal,
            );
            charts.push(
                projection::project(
                    id, address, region, color, group, component, vertices,
                )?,
            );
        }
    }
    for vertex in 0..group
        .positions
        .len()
    {
        if !covered_vertices.contains(&vertex) {
            return Err(
                SemanticTextureError::UncoveredVertex {
                    group: address,
                    vertex,
                },
            );
        }
    }
    Ok(charts)
}

/// Partition one exact-color triangle set by shared-vertex connectivity.
fn connected_components(
    group: &PrimitiveGroup,
    triangle_indices: &[usize],
) -> Result<Vec<Vec<usize>>, SemanticTextureError> {
    let mut by_vertex: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for triangle in triangle_indices {
        let indices = group
            .triangles
            .get(*triangle)
            .ok_or(SemanticTextureError::NumericOverflow)?;
        for index in indices {
            let vertex = usize::try_from(*index)
                .map_err(|_error| SemanticTextureError::NumericOverflow)?;
            by_vertex
                .entry(vertex)
                .or_default()
                .push(*triangle);
        }
    }
    let mut pending = triangle_indices
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let mut components = Vec::new();
    while let Some(seed) = pending.pop_first() {
        let mut queue = VecDeque::from([seed]);
        let mut component = Vec::new();
        while let Some(triangle) = queue.pop_front() {
            component.push(triangle);
            let indices = group
                .triangles
                .get(triangle)
                .ok_or(SemanticTextureError::NumericOverflow)?;
            for index in indices {
                let vertex = usize::try_from(*index)
                    .map_err(|_error| SemanticTextureError::NumericOverflow)?;
                for adjacent in by_vertex
                    .get(&vertex)
                    .into_iter()
                    .flatten()
                {
                    if pending.remove(adjacent) {
                        queue.push_back(*adjacent);
                    }
                }
            }
        }
        component.sort_unstable();
        components.push(component);
    }
    components.sort();
    Ok(components)
}

/// Collect sorted vertices referenced by one triangle component.
fn component_vertices(
    group: &PrimitiveGroup,
    triangles: &[usize],
) -> Result<Vec<usize>, SemanticTextureError> {
    let mut vertices = BTreeSet::new();
    for triangle in triangles {
        let indices = group
            .triangles
            .get(*triangle)
            .ok_or(SemanticTextureError::NumericOverflow)?;
        for index in indices {
            vertices.insert(
                usize::try_from(*index)
                    .map_err(|_error| SemanticTextureError::NumericOverflow)?,
            );
        }
    }
    Ok(
        vertices
            .into_iter()
            .collect(),
    )
}

/// Build one stable chart identity without source file names.
fn chart_id(
    address: GroupAddress,
    region: BodyRegion,
    color: Rgba8,
    ordinal: usize,
) -> String {
    format!(
        "part-{:04}-group-{:04}-{}-{:02x}{:02x}{:02x}{:02x}-chart-{:04}",
        address.part_index,
        address.group_index,
        region.as_str(),
        color.red,
        color.green,
        color.blue,
        color.alpha,
        ordinal,
    )
}
