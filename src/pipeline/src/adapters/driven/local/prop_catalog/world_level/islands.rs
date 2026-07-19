// File:
//   - islands.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/islands.
//     rs
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
//   - Deterministic separation of spatially distant world-geometry islands.
// - Must-Not:
//   - Read packages, infer gameplay state, transform coordinates, or write FBX.
// - Allows:
//   - Exact-position triangle connectivity and proximity-based object
//     clustering.
// - Summary:
//   - Prevents unrelated placed props from sharing one Blender object.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - true
//   - Reason: connectivity, attribute compaction, and proximity clustering form
//     one geometry-separation invariant.
//   - Split: extract a reusable mesh-island domain only when another lane needs
//     it.
//   - Validation: focused synthetic island tests and canonical pipeline gates.
//   - Review: required when the proximity contract changes.
//

//! Attribute-preserving separation of distant world mesh islands.

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::ops::Sub;

use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};

use crate::domain::PipelineError;

/// Maximum axis-aligned gap that still belongs to one visual object.
const OBJECT_JOIN_DISTANCE: f32 = 1.0;

/// One extracted connected primitive component.
#[derive(Clone, Debug)]
struct GroupComponent {
    /// Attribute-preserving primitive group.
    group: PrimitiveGroup,
    /// Component axis-aligned minimum.
    low: [f32; 3],
    /// Component axis-aligned maximum.
    high: [f32; 3],
}

/// Compact attributes and source-to-compact vertex identity.
#[derive(Clone, Debug)]
struct CompactAttributes {
    /// Source vertex to compact vertex mapping.
    remap: BTreeMap<u32, u32>,
    /// Compact positions.
    positions: Vec<[f32; 3]>,
    /// Compact UVs when present.
    uvs: Vec<[f32; 2]>,
    /// Compact normals when present.
    normals: Vec<[f32; 3]>,
    /// Compact vertex colors when present.
    colors: Vec<[f32; 4]>,
}

/// Split one mesh only when it contains spatially distant geometry clusters.
///
/// # Errors
///
/// Returns an error when source indices or reconstructed primitive attributes
/// are malformed.
pub(super) fn split_distant_islands(
    mesh: MeshAsset
) -> Result<Vec<MeshAsset>, PipelineError> {
    let mut components = Vec::new();
    for group in &mesh.groups {
        components.extend(group_components(group)?);
    }
    if components.len() <= 1 {
        return Ok(vec![mesh]);
    }
    let clusters = proximity_clusters(&components)?;
    if clusters.len() <= 1 {
        return Ok(vec![mesh]);
    }
    clusters
        .into_iter()
        .enumerate()
        .map(
            |(ordinal, cluster)| {
                rebuild_mesh(
                    &mesh.name,
                    ordinal,
                    &cluster,
                    &components,
                )
            },
        )
        .collect()
}

/// Rebuild one independent object from selected primitive components.
fn rebuild_mesh(
    source_name: &str,
    ordinal: usize,
    cluster: &[usize],
    components: &[GroupComponent],
) -> Result<MeshAsset, PipelineError> {
    let mut groups = Vec::with_capacity(cluster.len());
    for component_index in cluster {
        let component = components
            .get(*component_index)
            .ok_or_else(
                || PipelineError::new("world island component is missing"),
            )?;
        let mut group = component
            .group
            .clone();
        group.index = groups.len();
        groups.push(group);
    }
    MeshAsset::new(
        format!("{source_name}__independent-object-{ordinal:04}"),
        groups,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("world island mesh rebuild failed: {error:?}"),
            )
        },
    )
}

/// Extract exact-position connected components from one primitive group.
fn group_components(
    group: &PrimitiveGroup
) -> Result<Vec<GroupComponent>, PipelineError> {
    if group
        .triangles
        .is_empty()
    {
        return Ok(Vec::new());
    }
    let position_sets = triangle_position_sets(group)?;
    triangle_clusters(&position_sets)?
        .into_iter()
        .map(
            |triangles| {
                extract_component(
                    group, &triangles,
                )
            },
        )
        .collect()
}

/// Resolve one exact authored-position set for every triangle.
fn triangle_position_sets(
    group: &PrimitiveGroup
) -> Result<Vec<BTreeSet<[u32; 3]>>, PipelineError> {
    group
        .triangles
        .iter()
        .map(
            |triangle| {
                triangle
                    .iter()
                    .map(
                        |vertex_index| {
                            let index = usize::try_from(*vertex_index)
                                .map_err(
                                    |error| {
                                        PipelineError::new(
                                            format!(
                                                "world island vertex index \
                                                 overflowed: {error}"
                                            ),
                                        )
                                    },
                                )?;
                            group
                                .positions
                                .get(index)
                                .copied()
                                .map(|position| position.map(f32::to_bits))
                                .ok_or_else(
                                    || {
                                        PipelineError::new(
                                            "world island triangle index is \
                                             invalid",
                                        )
                                    },
                                )
                        },
                    )
                    .collect()
            },
        )
        .collect()
}

/// Cluster triangles that share at least one exact authored position.
fn triangle_clusters(
    position_sets: &[BTreeSet<[u32; 3]>]
) -> Result<Vec<Vec<usize>>, PipelineError> {
    let mut remaining = (0..position_sets.len()).collect::<BTreeSet<_>>();
    let mut clusters = Vec::new();
    while let Some(seed) = remaining.pop_first() {
        let mut queue = VecDeque::from([seed]);
        let mut cluster = Vec::new();
        while let Some(current) = queue.pop_front() {
            cluster.push(current);
            let current_set = position_sets
                .get(current)
                .ok_or_else(
                    || {
                        PipelineError::new(
                            "world triangle position set is missing",
                        )
                    },
                )?;
            let connected = remaining
                .iter()
                .copied()
                .filter(
                    |candidate| {
                        position_sets
                            .get(*candidate)
                            .is_some_and(
                                |other| !current_set.is_disjoint(other),
                            )
                    },
                )
                .collect::<Vec<_>>();
            for candidate in connected {
                let _removed = remaining.remove(&candidate);
                queue.push_back(candidate);
            }
        }
        cluster.sort_unstable();
        clusters.push(cluster);
    }
    Ok(clusters)
}

/// Rebuild one connected triangle set with compact vertex attributes.
fn extract_component(
    source: &PrimitiveGroup,
    triangle_indices: &[usize],
) -> Result<GroupComponent, PipelineError> {
    let used_vertices = collect_used_vertices(
        source,
        triangle_indices,
    )?;
    let compact = compact_attributes(
        source,
        &used_vertices,
    )?;
    let indices = remapped_indices(
        source,
        triangle_indices,
        &compact.remap,
    )?;
    let group = rebuild_group(
        source, compact, &indices,
    )?;
    let (low, high) = group_bounds(&group)?;
    Ok(
        GroupComponent {
            group,
            low,
            high,
        },
    )
}

/// Collect every source vertex referenced by selected triangles.
fn collect_used_vertices(
    source: &PrimitiveGroup,
    triangle_indices: &[usize],
) -> Result<BTreeSet<u32>, PipelineError> {
    let mut used_vertices = BTreeSet::new();
    for triangle_index in triangle_indices {
        let triangle = source
            .triangles
            .get(*triangle_index)
            .ok_or_else(
                || PipelineError::new("world island triangle is missing"),
            )?;
        used_vertices.extend(triangle);
    }
    Ok(used_vertices)
}

/// Compact source vertex attributes while preserving optional channel
/// alignment.
fn compact_attributes(
    source: &PrimitiveGroup,
    used_vertices: &BTreeSet<u32>,
) -> Result<CompactAttributes, PipelineError> {
    let mut compact = CompactAttributes {
        remap: BTreeMap::new(),
        positions: Vec::with_capacity(used_vertices.len()),
        uvs: Vec::with_capacity(used_vertices.len()),
        normals: Vec::with_capacity(used_vertices.len()),
        colors: Vec::with_capacity(used_vertices.len()),
    };
    for (compact_index, source_vertex) in used_vertices
        .iter()
        .copied()
        .enumerate()
    {
        let target = u32::try_from(compact_index).map_err(
            |error| {
                PipelineError::new(
                    format!("world island compact index overflowed: {error}"),
                )
            },
        )?;
        let source_index = usize::try_from(source_vertex).map_err(
            |error| {
                PipelineError::new(
                    format!("world island source index overflowed: {error}"),
                )
            },
        )?;
        let _previous = compact
            .remap
            .insert(
                source_vertex,
                target,
            );
        compact
            .positions
            .push(
                copy_required(
                    &source.positions,
                    source_index,
                    "position",
                )?,
            );
        copy_optional(
            &source.uvs,
            source_index,
            &mut compact.uvs,
            "UV",
        )?;
        copy_optional(
            &source.normals,
            source_index,
            &mut compact.normals,
            "normal",
        )?;
        copy_optional(
            &source.colors,
            source_index,
            &mut compact.colors,
            "color",
        )?;
    }
    Ok(compact)
}

/// Copy one required compact vertex attribute.
fn copy_required<T: Copy>(
    source: &[T],
    index: usize,
    label: &str,
) -> Result<T, PipelineError> {
    source
        .get(index)
        .copied()
        .ok_or_else(
            || PipelineError::new(format!("world island {label} is missing")),
        )
}

/// Copy one optional compact vertex attribute when its channel exists.
fn copy_optional<T: Copy>(
    source: &[T],
    index: usize,
    output: &mut Vec<T>,
    label: &str,
) -> Result<(), PipelineError> {
    if source.is_empty() {
        return Ok(());
    }
    output.push(
        copy_required(
            source, index, label,
        )?,
    );
    Ok(())
}

/// Remap selected source triangles to compact vertex indices.
fn remapped_indices(
    source: &PrimitiveGroup,
    triangle_indices: &[usize],
    remap: &BTreeMap<u32, u32>,
) -> Result<Vec<u32>, PipelineError> {
    let capacity = triangle_indices
        .len()
        .checked_mul(3)
        .ok_or_else(
            || PipelineError::new("world island index capacity overflowed"),
        )?;
    let mut indices = Vec::with_capacity(capacity);
    for triangle_index in triangle_indices {
        let triangle = source
            .triangles
            .get(*triangle_index)
            .ok_or_else(
                || PipelineError::new("world island triangle is missing"),
            )?;
        for source_vertex in triangle {
            indices.push(
                *remap
                    .get(source_vertex)
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                "world island vertex remap is missing",
                            )
                        },
                    )?,
            );
        }
    }
    Ok(indices)
}

/// Rebuild one primitive group and restore its optional compact channels.
fn rebuild_group(
    source: &PrimitiveGroup,
    compact: CompactAttributes,
    indices: &[u32],
) -> Result<PrimitiveGroup, PipelineError> {
    let CompactAttributes {
        positions,
        uvs,
        normals,
        colors,
        ..
    } = compact;
    let mut group = PrimitiveGroup::new(
        source.index,
        source
            .shader
            .clone(),
        positions,
        uvs,
        indices,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("world island primitive rebuild failed: {error:?}"),
            )
        },
    )?;
    if !normals.is_empty() {
        group = group
            .with_normals(normals)
            .map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "world island normal rebuild failed: {error:?}"
                        ),
                    )
                },
            )?;
    }
    if !colors.is_empty() {
        group = group
            .with_colors(colors)
            .map_err(
                |error| {
                    PipelineError::new(
                        format!("world island color rebuild failed: {error:?}"),
                    )
                },
            )?;
    }
    Ok(group)
}

/// Cluster connected components whose axis-aligned bounds remain nearby.
fn proximity_clusters(
    components: &[GroupComponent]
) -> Result<Vec<Vec<usize>>, PipelineError> {
    let mut remaining = (0..components.len()).collect::<BTreeSet<_>>();
    let mut clusters = Vec::new();
    while let Some(seed) = remaining.pop_first() {
        let mut queue = VecDeque::from([seed]);
        let mut cluster = Vec::new();
        while let Some(current) = queue.pop_front() {
            cluster.push(current);
            let current_component = components
                .get(current)
                .ok_or_else(
                    || {
                        PipelineError::new(
                            "world island proximity component is missing",
                        )
                    },
                )?;
            let connected = remaining
                .iter()
                .copied()
                .filter(
                    |candidate| {
                        components
                            .get(*candidate)
                            .is_some_and(
                                |other| {
                                    component_gap(
                                        current_component,
                                        other,
                                    ) <= OBJECT_JOIN_DISTANCE
                                },
                            )
                    },
                )
                .collect::<Vec<_>>();
            for candidate in connected {
                let _removed = remaining.remove(&candidate);
                queue.push_back(candidate);
            }
        }
        cluster.sort_unstable();
        clusters.push(cluster);
    }
    Ok(clusters)
}

/// Return the maximum per-axis gap between two component bounds.
fn component_gap(
    left: &GroupComponent,
    right: &GroupComponent,
) -> f32 {
    let [
        left_low_x,
        left_low_y,
        left_low_z,
    ] = left.low;
    let [
        left_high_x,
        left_high_y,
        left_high_z,
    ] = left.high;
    let [
        right_low_x,
        right_low_y,
        right_low_z,
    ] = right.low;
    let [
        right_high_x,
        right_high_y,
        right_high_z,
    ] = right.high;
    [
        axis_gap(
            left_low_x,
            left_high_x,
            right_low_x,
            right_high_x,
        ),
        axis_gap(
            left_low_y,
            left_high_y,
            right_low_y,
            right_high_y,
        ),
        axis_gap(
            left_low_z,
            left_high_z,
            right_low_z,
            right_high_z,
        ),
    ]
    .into_iter()
    .fold(
        0.0_f32,
        f32::max,
    )
}

/// Return one axis gap without using unchecked arithmetic operators.
fn axis_gap(
    left_low: f32,
    left_high: f32,
    right_low: f32,
    right_high: f32,
) -> f32 {
    if left_high < right_low {
        right_low.sub(left_high)
    } else if right_high < left_low {
        left_low.sub(right_high)
    } else {
        0.0
    }
}

/// Return one primitive component's axis-aligned bounds.
fn group_bounds(
    group: &PrimitiveGroup
) -> Result<
    (
        [f32; 3],
        [f32; 3],
    ),
    PipelineError,
> {
    let first = group
        .positions
        .first()
        .copied()
        .ok_or_else(|| PipelineError::new("world island has no positions"))?;
    let mut low = first;
    let mut high = first;
    for position in &group.positions {
        for ((low_axis, high_axis), position_axis) in low
            .iter_mut()
            .zip(high.iter_mut())
            .zip(position.iter())
        {
            *low_axis = low_axis.min(*position_axis);
            *high_axis = high_axis.max(*position_axis);
        }
    }
    Ok(
        (
            low, high,
        ),
    )
}

#[cfg(test)]
mod tests {
    use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};

    use super::split_distant_islands;

    /// Build two disconnected triangles separated on the X axis.
    fn synthetic_mesh(second_origin: f32) -> Result<MeshAsset, String> {
        let group = PrimitiveGroup::new(
            0,
            "material",
            vec![
                [
                    0.0, 0.0, 0.0,
                ],
                [
                    1.0, 0.0, 0.0,
                ],
                [
                    0.0, 1.0, 0.0,
                ],
                [
                    second_origin,
                    0.0,
                    0.0,
                ],
                [
                    second_origin + 1.0,
                    0.0,
                    0.0,
                ],
                [
                    second_origin,
                    1.0,
                    0.0,
                ],
            ],
            vec![
                [
                    0.0, 0.0,
                ],
                [
                    1.0, 0.0,
                ],
                [
                    0.0, 1.0,
                ],
                [
                    0.0, 0.0,
                ],
                [
                    1.0, 0.0,
                ],
                [
                    0.0, 1.0,
                ],
            ],
            &[
                0, 1, 2, 3, 4, 5,
            ],
        )
        .map_err(|error| format!("group failed: {error:?}"))?;
        MeshAsset::new(
            "aggregate",
            vec![group],
        )
        .map_err(|error| format!("mesh failed: {error:?}"))
    }

    #[test]
    fn distant_components_become_independent_objects() -> Result<(), String> {
        let separated = split_distant_islands(synthetic_mesh(20.0)?)
            .map_err(|error| error.to_string())?;
        if separated.len() != 2 {
            return Err(
                format!(
                    "expected two objects, got {}",
                    separated.len()
                ),
            );
        }
        if separated
            .iter()
            .any(
                |mesh| {
                    !mesh
                        .name
                        .contains("__independent-object-")
                },
            )
        {
            return Err("split objects lack stable semantic names".to_owned());
        }
        Ok(())
    }

    #[test]
    fn nearby_components_remain_one_visual_object() -> Result<(), String> {
        let separated = split_distant_islands(synthetic_mesh(1.5)?)
            .map_err(|error| error.to_string())?;
        let first = separated
            .first()
            .ok_or_else(|| "nearby mesh disappeared".to_owned())?;
        if separated.len() != 1 || first.name != "aggregate" {
            return Err("nearby geometry was split apart".to_owned());
        }
        Ok(())
    }
}
