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
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::types::{EyeComponent, EyeSide, EyeTextureError};
use crate::domain::mesh::PrimitiveGroup;

/// Discover exactly two connected components and assign stable side identities.
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
    if components.len() != 2 {
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
    if components[0]
        .0
        .total_cmp(&components[1].0)
        .is_eq()
    {
        return Err(EyeTextureError::AmbiguousComponentSides);
    }
    Ok(
        vec![
            EyeComponent {
                side: EyeSide::NegativeX,
                vertex_indices: components[0]
                    .1
                    .clone(),
                centroid_x: components[0].0,
            },
            EyeComponent {
                side: EyeSide::PositiveX,
                vertex_indices: components[1]
                    .1
                    .clone(),
                centroid_x: components[1].0,
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
