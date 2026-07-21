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
//   - Three-zone family grouping and aggregate-bound validation.
// - Must-Not:
//   - Read packages, infer operator placement, own reviewed movements,
//     serialize catalogs, or write FBX files.
// - Allows:
//   - Scope classification, zero-offset recurring-family placement, and finite
//     AABB validation.
// - Summary:
//   - Classifies recurring zones and validates connected world bounds.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Three-zone grouping and aggregate-bound validation.

use std::collections::BTreeMap;

use fbx::domain::mesh::MeshAsset;

use super::transform::{bake_mesh, mesh_bounds, translation};
use crate::domain::PipelineError;

/// One scope's baked placement within a recurring exterior family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MapPlacement {
    /// Stable connected zone-family identity.
    pub(super) group: Option<&'static str>,
    /// Baked source-space translation in whole source units.
    pub(super) offset: [i16; 3],
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
        },
        "level-02" | "level-05" => MapPlacement {
            group: Some("map-02-05"),
            offset: [
                0, 0, 0,
            ],
        },
        "level-03" | "level-06" => MapPlacement {
            group: Some("map-03-06"),
            offset: [
                0, 0, 0,
            ],
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

/// Validate that every recorded zone-family bound is finite and ordered.
///
/// Connected zone families may overlap at authored seams, so overlap is no
/// longer rejected. Only malformed aggregate bounds fail.
///
/// # Errors
///
/// Returns an error when one group bound is non-finite or inverted.
pub(super) fn validate_group_bounds(
    bounds: &BTreeMap<&'static str, MapBounds>
) -> Result<(), PipelineError> {
    for (name, bound) in bounds {
        for (low, high) in bound
            .low
            .iter()
            .zip(&bound.high)
        {
            if !low.is_finite() || !high.is_finite() || low > high {
                return Err(
                    PipelineError::new(
                        format!("world map group has invalid bounds: {name}"),
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

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::{MapBounds, placement_for_scope, validate_group_bounds};

    #[test]
    fn recurring_levels_share_groups_without_artificial_offsets()
    -> Result<(), String> {
        for (levels, group) in [
            (
                &[
                    "level-01", "level-04", "level-07",
                ][..],
                "map-01-04-07",
            ),
            (
                &[
                    "level-02", "level-05",
                ][..],
                "map-02-05",
            ),
            (
                &[
                    "level-03", "level-06",
                ][..],
                "map-03-06",
            ),
        ] {
            for level in levels {
                let placement = placement_for_scope(level)
                    .map_err(|error| error.to_string())?;
                if placement.group != Some(group)
                    || placement.offset
                        != [
                            0, 0, 0,
                        ]
                {
                    return Err(format!("invalid zone grouping for {level}"));
                }
            }
        }
        Ok(())
    }

    #[test]
    fn connected_overlap_is_allowed_but_invalid_bounds_fail()
    -> Result<(), String> {
        let mut bounds = BTreeMap::new();
        let _ = bounds.insert(
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
        let _ = bounds.insert(
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
        validate_group_bounds(&bounds).map_err(|error| error.to_string())?;
        let _ = bounds.insert(
            "map-03-06",
            MapBounds {
                low: [
                    5.0, 0.0, 0.0,
                ],
                high: [
                    4.0, 10.0, 10.0,
                ],
            },
        );
        if validate_group_bounds(&bounds).is_ok() {
            return Err(String::from("inverted bounds were accepted"));
        }
        Ok(())
    }
}
