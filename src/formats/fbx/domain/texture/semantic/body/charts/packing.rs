// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Packing domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Packing domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Packing domain module.

use super::super::super::region::BodyRegion;
use super::super::error::SemanticTextureError;
use super::super::recipe::AtlasConfig;
use super::super::types::ProjectionAxis;
use super::model::{PlacedChart, ProjectedChart};

#[path = "packing/grid.rs"]
mod grid;
#[path = "packing/mapping.rs"]
mod mapping;

/// Convert one destination pixel position into V-up atlas UV coordinates.
pub(super) fn atlas_uv(
    position: [f32; 2],
    config: &AtlasConfig,
    projection: ProjectionAxis,
) -> [f32; 2] {
    mapping::atlas_uv(position, config, projection)
}

/// Place every chart in deterministic semantic and chart order.
pub(super) fn place(
    charts: &[ProjectedChart],
    config: &AtlasConfig,
    source_texture_size: [u32; 2],
) -> Result<Vec<PlacedChart>, SemanticTextureError> {
    let mut placed = Vec::with_capacity(charts.len());
    for region in BodyRegion::ALL {
        let region_charts = charts
            .iter()
            .filter(|chart| chart.region == region)
            .collect::<Vec<_>>();
        if region_charts.is_empty() {
            continue;
        }
        let region_rect = grid::semantic_column(config, region)?;
        let layout = grid::choose(
            region,
            region_rect,
            region_charts.len(),
            config.padding,
        )?;
        for (index, chart) in region_charts.into_iter().enumerate() {
            let cell = grid::cell(region_rect, layout, index)?;
            placed.push(mapping::map_chart(
                chart,
                cell,
                config,
                source_texture_size,
            )?);
        }
    }
    Ok(placed)
}
