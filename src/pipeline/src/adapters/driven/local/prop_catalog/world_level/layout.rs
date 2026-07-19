// File:
//   - layout.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/layout.rs
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
//   - Three-map variant grouping, baked map offsets, and overlap rejection.
// - Must-Not:
//   - Read packages, select geometry, serialize catalogs, or write FBX files.
// - Allows:
//   - Scope classification, static geometry translation, and AABB validation.
// - Summary:
//   - Aligns level variants while separating independent narrative maps.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Three-map layout and disjoint-bound validation.

use std::collections::BTreeMap;

use fbx::domain::mesh::MeshAsset;

use super::transform::{bake_mesh, mesh_bounds, translation};
use crate::domain::PipelineError;

/// Horizontal distance between independent map origins.
const MAP_GROUP_SPACING: i16 = 8_192;

/// One scope's baked map placement and import classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MapPlacement {
    /// Stable narrative map identity, absent for auxiliary bonus areas.
    pub(super) group: Option<&'static str>,
    /// Baked source-space translation in whole source units.
    pub(super) offset: [i16; 3],
    /// Whether the artifact belongs in the normal root-FBX import set.
    pub(super) normal_import: bool,
}

/// One post-placement map-group axis-aligned bound.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct MapBounds {
    /// Minimum position on each source-space axis.
    pub(super) low: [f32; 3],
    /// Maximum position on each source-space axis.
    pub(super) high: [f32; 3],
}

/// Resolve one exported package scope into exactly one map placement.
///
/// # Errors
///
/// Returns an error when a narrative level has no declared map group.
pub(super) fn placement_for_scope(
    scope: &str
) -> Result<MapPlacement, PipelineError> {
    let placement = match scope {
        "level-01" | "level-04" | "level-07" => MapPlacement {
            group: Some("map-01-04-07"),
            offset: [
                0, 0, 0,
            ],
            normal_import: true,
        },
        "level-02" | "level-05" => MapPlacement {
            group: Some("map-02-05"),
            offset: [
                MAP_GROUP_SPACING,
                0,
                0,
            ],
            normal_import: true,
        },
        "level-03" | "level-06" => MapPlacement {
            group: Some("map-03-06"),
            offset: [
                MAP_GROUP_SPACING.saturating_mul(2),
                0,
                0,
            ],
            normal_import: true,
        },
        "bonus-area" => MapPlacement {
            group: None,
            offset: [
                0, 0, 0,
            ],
            normal_import: false,
        },
        _ => {
            return Err(
                PipelineError::new(
                    format!("world scope has no declared map layout: {scope}"),
                ),
            );
        }
    };
    Ok(placement)
}

/// Bake one scope placement into all normally imported package meshes.
///
/// # Errors
///
/// Returns an error when one translated mesh becomes invalid.
pub(super) fn apply_placement(
    meshes: &mut [MeshAsset],
    placement: MapPlacement,
) -> Result<(), PipelineError> {
    if placement.offset
        == [
            0, 0, 0,
        ]
    {
        return Ok(());
    }
    let matrix = translation(
        placement
            .offset
            .map(f32::from),
    );
    for mesh in meshes {
        let name = mesh
            .name
            .clone();
        bake_mesh(
            mesh, &matrix, name,
        )?;
    }
    Ok(())
}

/// Return one aggregate bound for a non-empty mesh collection.
#[must_use]
pub(super) fn collection_bounds(meshes: &[MeshAsset]) -> Option<MapBounds> {
    let mut aggregate: Option<MapBounds> = None;
    for mesh in meshes {
        let (low, high) = mesh_bounds(mesh);
        aggregate = Some(
            aggregate.map_or(
                MapBounds {
                    low,
                    high,
                },
                |current| {
                    merge_bounds(
                        current,
                        MapBounds {
                            low,
                            high,
                        },
                    )
                },
            ),
        );
    }
    aggregate
}

/// Merge one package bound into its narrative map group.
pub(super) fn record_group_bounds(
    bounds: &mut BTreeMap<&'static str, MapBounds>,
    placement: MapPlacement,
    package_bounds: Option<MapBounds>,
) {
    let Some(group) = placement.group else {
        return;
    };
    let Some(package) = package_bounds else {
        return;
    };
    let _entry = bounds
        .entry(group)
        .and_modify(
            |current| {
                *current = merge_bounds(
                    *current, package,
                );
            },
        )
        .or_insert(package);
}

/// Reject any overlap between independent narrative-map bounds.
///
/// # Errors
///
/// Returns an error when two different map groups overlap on all three axes.
pub(super) fn validate_disjoint_groups(
    bounds: &BTreeMap<&'static str, MapBounds>
) -> Result<(), PipelineError> {
    let entries = bounds
        .iter()
        .collect::<Vec<_>>();
    for (index, (left_name, left)) in entries
        .iter()
        .enumerate()
    {
        for (right_name, right) in entries
            .iter()
            .skip(index.saturating_add(1))
        {
            if overlaps(
                **left, **right,
            ) {
                return Err(
                    PipelineError::new(
                        format!(
                            "independent world map groups overlap: \
                             {left_name} and {right_name}"
                        ),
                    ),
                );
            }
        }
    }
    Ok(())
}

/// Merge two axis-aligned bounds.
const fn merge_bounds(
    left: MapBounds,
    right: MapBounds,
) -> MapBounds {
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
    MapBounds {
        low: [
            minimum(
                left_low_x,
                right_low_x,
            ),
            minimum(
                left_low_y,
                right_low_y,
            ),
            minimum(
                left_low_z,
                right_low_z,
            ),
        ],
        high: [
            maximum(
                left_high_x,
                right_high_x,
            ),
            maximum(
                left_high_y,
                right_high_y,
            ),
            maximum(
                left_high_z,
                right_high_z,
            ),
        ],
    }
}

/// Return the lower of two finite source-space coordinates.
const fn minimum(
    left: f32,
    right: f32,
) -> f32 {
    if left < right {
        left
    } else {
        right
    }
}

/// Return the higher of two finite source-space coordinates.
const fn maximum(
    left: f32,
    right: f32,
) -> f32 {
    if left > right {
        left
    } else {
        right
    }
}

/// Return whether two closed bounds overlap with positive volume.
fn overlaps(
    left: MapBounds,
    right: MapBounds,
) -> bool {
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
    left_low_x < right_high_x
        && left_high_x > right_low_x
        && left_low_y < right_high_y
        && left_high_y > right_low_y
        && left_low_z < right_high_z
        && left_high_z > right_low_z
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{
        MAP_GROUP_SPACING, MapBounds, placement_for_scope,
        validate_disjoint_groups,
    };

    #[test]
    fn variants_share_one_map_offset_and_independent_maps_do_not()
    -> Result<(), String> {
        let level_one = placement_for_scope("level-01")
            .map_err(|error| error.to_string())?;
        let level_four = placement_for_scope("level-04")
            .map_err(|error| error.to_string())?;
        let level_seven = placement_for_scope("level-07")
            .map_err(|error| error.to_string())?;
        if level_one != level_four || level_one != level_seven {
            return Err(
                "levels 1, 4, and 7 did not share one placement".to_owned(),
            );
        }
        let level_two = placement_for_scope("level-02")
            .map_err(|error| error.to_string())?;
        let level_three = placement_for_scope("level-03")
            .map_err(|error| error.to_string())?;
        if level_two.offset
            != [
                MAP_GROUP_SPACING,
                0,
                0,
            ]
            || level_three.offset
                != [
                    MAP_GROUP_SPACING.saturating_mul(2),
                    0,
                    0,
                ]
        {
            return Err(
                "independent map offsets were not deterministic".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn independent_group_overlap_is_a_hard_error() {
        let mut bounds = BTreeMap::new();
        let _first = bounds.insert(
            "map-01-04-07",
            MapBounds {
                low: [
                    0.0, 0.0, 0.0,
                ],
                high: [
                    10.0, 10.0, 10.0,
                ],
            },
        );
        let _second = bounds.insert(
            "map-02-05",
            MapBounds {
                low: [
                    9.0, 0.0, 0.0,
                ],
                high: [
                    20.0, 10.0, 10.0,
                ],
            },
        );
        assert!(validate_disjoint_groups(&bounds).is_err());
    }

    #[test]
    fn bonus_areas_are_auxiliary_not_a_fourth_normal_map() -> Result<(), String>
    {
        let bonus = placement_for_scope("bonus-area")
            .map_err(|error| error.to_string())?;
        if bonus
            .group
            .is_some()
            || bonus.normal_import
        {
            return Err(
                "bonus area entered the normal three-map import set".to_owned(),
            );
        }
        Ok(())
    }
}
