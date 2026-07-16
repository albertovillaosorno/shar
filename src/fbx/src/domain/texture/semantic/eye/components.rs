// File:
//   - components.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/eye/components.rs
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
//   - Shared-vertex connected-component discovery and stable side assignment
//   - for one two-eye primitive group.
// - Must-Not:
//   - Analyze texture frames, infer anatomical left or right labels, or change
//   - source geometry.
// - Allows:
//   - Complete vertex coverage checks and horizontal centroid ordering.
// - Split-When:
//   - More than two supported eye components require a different identity
//   - model.
// - Merge-When:
//   - Another eye module owns the same connected-component contract.
// - Summary:
//   - Two-eye mesh component discovery.
// - Description:
//   - Identifies the negative-X and positive-X components without depending on
//   - source object names.
// - Usage:
//   - Called by the eye semantic facade before frame analysis.
// - Defaults:
//   - Exactly two nonempty components with distinct horizontal centroids.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: adjacency construction, traversal, coverage, centroid
//   - calculation, and side assignment form one component identity
//   - transaction.
//

//! Deterministic two-eye connected-component discovery.
#![expect(
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing,
    clippy::shadow_reuse,
    clippy::too_many_lines,
    unused_results,
    reason = "Validated topology bounds component indices; fixture assertions \
              document the side-splitting contract."
)]

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::types::{EyeComponent, EyeSide, EyeTextureError};
use crate::domain::mesh::PrimitiveGroup;

/// Discover two eye sides from one or more disconnected islands per side.
pub(super) fn discover(
    group: &PrimitiveGroup
) -> Result<Vec<EyeComponent>, EyeTextureError> {
    let mut adjacency: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    let mut referenced = BTreeSet::new();
    for triangle in &group.triangles {
        let vertices = [
            index(triangle[0])?,
            index(triangle[1])?,
            index(triangle[2])?,
        ];
        referenced.extend(vertices);
        connect(
            &mut adjacency,
            vertices[0],
            vertices[1],
        );
        connect(
            &mut adjacency,
            vertices[1],
            vertices[2],
        );
        connect(
            &mut adjacency,
            vertices[2],
            vertices[0],
        );
    }
    validate_coverage(
        group,
        &referenced,
    )?;
    let mut pending = referenced;
    let mut components = Vec::new();
    while let Some(seed) = pending.pop_first() {
        let mut queue = VecDeque::from([seed]);
        let mut vertices = Vec::new();
        while let Some(vertex) = queue.pop_front() {
            vertices.push(vertex);
            for adjacent in adjacency
                .get(&vertex)
                .into_iter()
                .flatten()
            {
                if pending.remove(adjacent) {
                    queue.push_back(*adjacent);
                }
            }
        }
        vertices.sort_unstable();
        components.push(
            (
                centroid_x(
                    group, &vertices,
                )?,
                vertices,
            ),
        );
    }
    if components.len() < 2 {
        return Err(
            EyeTextureError::ComponentCount {
                actual: components.len(),
            },
        );
    }
    components.sort_by(
        |left, right| {
            left.0
                .total_cmp(&right.0)
        },
    );
    let mut split = None;
    let mut largest_gap = None;
    let mut ambiguous = false;
    for position in 1..components.len() {
        let gap = components[position].0 - components[position - 1].0;
        match largest_gap {
            None => {
                largest_gap = Some(gap);
                split = Some(position);
                ambiguous = false;
            }
            Some(current)
                if gap
                    .total_cmp(&current)
                    .is_gt() =>
            {
                largest_gap = Some(gap);
                split = Some(position);
                ambiguous = false;
            }
            Some(current)
                if gap
                    .total_cmp(&current)
                    .is_eq() =>
            {
                ambiguous = true;
            }
            Some(_current) => {}
        }
    }
    if ambiguous || largest_gap.is_none_or(|gap| gap <= 0.0) {
        return Err(EyeTextureError::AmbiguousComponentSides);
    }
    let split = split.ok_or(EyeTextureError::AmbiguousComponentSides)?;
    let (negative_components, positive_components) = components.split_at(split);
    let mut negative_vertices = negative_components
        .iter()
        .flat_map(
            |component| {
                component
                    .1
                    .iter()
                    .copied()
            },
        )
        .collect::<Vec<_>>();
    let mut positive_vertices = positive_components
        .iter()
        .flat_map(
            |component| {
                component
                    .1
                    .iter()
                    .copied()
            },
        )
        .collect::<Vec<_>>();
    negative_vertices.sort_unstable();
    positive_vertices.sort_unstable();
    let negative_centroid = centroid_x(
        group,
        &negative_vertices,
    )?;
    let positive_centroid = centroid_x(
        group,
        &positive_vertices,
    )?;
    if negative_centroid
        .total_cmp(&positive_centroid)
        .is_eq()
    {
        return Err(EyeTextureError::AmbiguousComponentSides);
    }
    Ok(
        vec![
            EyeComponent {
                side: EyeSide::NegativeX,
                vertex_indices: negative_vertices,
                centroid_x: negative_centroid,
            },
            EyeComponent {
                side: EyeSide::PositiveX,
                vertex_indices: positive_vertices,
                centroid_x: positive_centroid,
            },
        ],
    )
}

/// Require every mesh vertex to belong to one eye component.
fn validate_coverage(
    group: &PrimitiveGroup,
    referenced: &BTreeSet<usize>,
) -> Result<(), EyeTextureError> {
    for vertex in 0..group
        .positions
        .len()
    {
        if !referenced.contains(&vertex) {
            return Err(
                EyeTextureError::UncoveredVertex {
                    vertex,
                },
            );
        }
    }
    Ok(())
}

/// Add one undirected adjacency edge.
fn connect(
    adjacency: &mut BTreeMap<usize, BTreeSet<usize>>,
    first: usize,
    second: usize,
) {
    adjacency
        .entry(first)
        .or_default()
        .insert(second);
    adjacency
        .entry(second)
        .or_default()
        .insert(first);
}

/// Calculate one component's horizontal centroid.
fn centroid_x(
    group: &PrimitiveGroup,
    vertices: &[usize],
) -> Result<f32, EyeTextureError> {
    let mut sum = 0.0_f32;
    for vertex in vertices {
        let position = group
            .positions
            .get(*vertex)
            .ok_or(EyeTextureError::NumericOverflow)?;
        sum += position[0];
    }
    let count = u16::try_from(vertices.len())
        .map(f32::from)
        .map_err(|_error| EyeTextureError::NumericOverflow)?;
    if count == 0.0 {
        return Err(
            EyeTextureError::ComponentCount {
                actual: 0,
            },
        );
    }
    Ok(sum / count)
}

/// Convert one domain vertex index into the host index type.
fn index(value: u32) -> Result<usize, EyeTextureError> {
    usize::try_from(value).map_err(|_error| EyeTextureError::NumericOverflow)
}

#[cfg(test)]
mod tests {
    use super::discover;
    use crate::domain::mesh::PrimitiveGroup;

    #[test]
    fn merges_multiple_disconnected_islands_per_eye_side() {
        let positions = vec![
            [
                -2.2, 0.0, 0.0,
            ],
            [
                -2.0, 0.2, 0.0,
            ],
            [
                -1.8, 0.0, 0.0,
            ],
            [
                -1.7, 0.0, 0.0,
            ],
            [
                -1.5, 0.2, 0.0,
            ],
            [
                -1.3, 0.0, 0.0,
            ],
            [
                1.3, 0.0, 0.0,
            ],
            [
                1.5, 0.2, 0.0,
            ],
            [
                1.7, 0.0, 0.0,
            ],
            [
                1.8, 0.0, 0.0,
            ],
            [
                2.0, 0.2, 0.0,
            ],
            [
                2.2, 0.0, 0.0,
            ],
        ];
        let indices = [
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11,
        ];
        let group_result = PrimitiveGroup::new(
            0,
            "eye",
            positions,
            Vec::new(),
            &indices,
        );
        assert!(
            group_result.is_ok(),
            "fixture group failed: {group_result:?}"
        );
        let Ok(group) = group_result else {
            return;
        };
        let discovery_result = discover(&group);
        assert!(
            discovery_result.is_ok(),
            "eye discovery failed: {discovery_result:?}"
        );
        let Ok(components) = discovery_result else {
            return;
        };
        let [
            left,
            right,
        ] = components.as_slice()
        else {
            assert_eq!(
                components.len(),
                2
            );
            return;
        };

        assert_eq!(
            left.vertex_indices
                .len(),
            6
        );
        assert_eq!(
            right
                .vertex_indices
                .len(),
            6
        );
        assert!(left.centroid_x < right.centroid_x);
    }
}
