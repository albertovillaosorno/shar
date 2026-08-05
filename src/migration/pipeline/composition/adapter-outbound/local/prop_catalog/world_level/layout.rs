// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Source-authored world grouping and aggregate-bound validation.
// - Must-Not:
//   - Translate, rotate, scale, recenter, or otherwise alter source geometry.
// - Allows:
//   - Inputs: exported package scopes and source-space mesh bounds.
//   - Outputs: narrative grouping metadata and validated aggregate bounds.
//   - Side effects: none.
// - Split-When:
//   - Split when another grouping scheme gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns identical source-space grouping policy.
// - Summary:
//   - Source-preserving world layout metadata.
// - Description:
//   - Groups recurring narrative levels without baking artificial placement
//     offsets into their source-authored geometry.
// - Usage:
//   - Used by world FBX export for catalog metadata and bounds validation.
// - Defaults:
//   - Unknown scopes and malformed bounds fail explicitly.
//

//! Source-preserving world layout metadata.

use std::collections::BTreeMap;

use fbx::domain::mesh::MeshAsset;

use super::transform::mesh_bounds;
use crate::domain::PipelineError;

/// One scope's recurring narrative-family identity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct MapPlacement {
    /// Stable recurring narrative-family identity.
    pub(super) group: &'static str,
}

/// One source-space map-group axis-aligned bound.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct MapBounds {
    /// Minimum position on each source-space axis.
    pub(super) low: [f32; 3],
    /// Maximum position on each source-space axis.
    pub(super) high: [f32; 3],
}

/// Resolve one exported package scope into exactly one narrative family.
///
/// # Errors
///
/// Returns an error when a narrative level has no declared family.
pub(super) fn placement_for_scope(
    scope: &str,
) -> Result<MapPlacement, PipelineError> {
    let group = match scope {
        "level-01" | "level-04" | "level-07" => "map-01-04-07",
        "level-02" | "level-05" => "map-02-05",
        "level-03" | "level-06" => "map-03-06",
        _ => {
            return Err(PipelineError::new(format!(
                "world scope has no declared narrative family: {scope}"
            )));
        },
    };
    Ok(MapPlacement { group })
}

/// Return one aggregate bound for a non-empty mesh collection.
#[must_use]
pub(super) fn collection_bounds(meshes: &[MeshAsset]) -> Option<MapBounds> {
    let mut aggregate: Option<MapBounds> = None;
    for mesh in meshes {
        let (low, high) = mesh_bounds(mesh);
        aggregate =
            Some(aggregate.map_or(MapBounds { low, high }, |current| {
                merge_bounds(current, MapBounds { low, high })
            }));
    }
    aggregate
}

/// Merge one source-space package bound into its narrative family.
pub(super) fn record_group_bounds(
    bounds: &mut BTreeMap<&'static str, MapBounds>,
    placement: MapPlacement,
    package_bounds: Option<MapBounds>,
) {
    let Some(package) = package_bounds else {
        return;
    };
    let _entry = bounds
        .entry(placement.group)
        .and_modify(|current| {
            *current = merge_bounds(*current, package);
        })
        .or_insert(package);
}

/// Validate that every source-space narrative-family bound is finite and
/// ordered.
///
/// # Errors
///
/// Returns an error when one group bound is non-finite or inverted.
pub(super) fn validate_group_bounds(
    bounds: &BTreeMap<&'static str, MapBounds>,
) -> Result<(), PipelineError> {
    for (name, bound) in bounds {
        for (low, high) in bound.low.iter().zip(&bound.high) {
            if !low.is_finite() || !high.is_finite() || low > high {
                return Err(PipelineError::new(format!(
                    "world narrative family has invalid bounds: {name}"
                )));
            }
        }
    }
    Ok(())
}

/// Merge two axis-aligned bounds.
const fn merge_bounds(left: MapBounds, right: MapBounds) -> MapBounds {
    let [left_low_x, left_low_y, left_low_z] = left.low;
    let [left_high_x, left_high_y, left_high_z] = left.high;
    let [right_low_x, right_low_y, right_low_z] = right.low;
    let [right_high_x, right_high_y, right_high_z] = right.high;
    MapBounds {
        low: [
            minimum(left_low_x, right_low_x),
            minimum(left_low_y, right_low_y),
            minimum(left_low_z, right_low_z),
        ],
        high: [
            maximum(left_high_x, right_high_x),
            maximum(left_high_y, right_high_y),
            maximum(left_high_z, right_high_z),
        ],
    }
}

/// Return the lower of two finite source-space coordinates.
const fn minimum(left: f32, right: f32) -> f32 {
    if left < right {
        left
    } else {
        right
    }
}

/// Return the higher of two finite source-space coordinates.
const fn maximum(left: f32, right: f32) -> f32 {
    if left > right {
        left
    } else {
        right
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/layout/tests.rs"]
mod tests;
