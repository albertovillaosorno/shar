// File:
//   - packing.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/charts/packing.rs
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
//   - The ordered semantic chart placement transaction.
// - Must-Not:
//   - Discover charts, change topology, rasterize pixels, or sample source
//   - color.
// - Allows:
//   - Focused integer-grid and aspect-preserving mapping modules.
// - Split-When:
//   - Another packing family cannot reuse region grouping and stable ordering.
// - Merge-When:
//   - The chart facade can own placement directly without duplication.
// - Summary:
//   - Semantic chart packing facade.
// - Description:
//   - Places charts by fixed region order and delegates exact rectangle
//   - mapping.
// - Usage:
//   - Called after chart discovery and before UV mutation or rasterization.
// - Defaults:
//   - Region columns follow the fixed BodyRegion order.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Ordered semantic chart atlas placement.
use super::super::super::region::BodyRegion;
use super::super::error::SemanticTextureError;
use super::super::recipe::AtlasConfig;
use super::model::{PlacedChart, ProjectedChart};

#[path = "packing/grid.rs"]
mod grid;
#[path = "packing/mapping.rs"]
mod mapping;

/// Convert one destination pixel position into V-up atlas UV coordinates.
pub(super) fn atlas_uv(
    position: [f32; 2],
    config: &AtlasConfig,
) -> [f32; 2] {
    mapping::atlas_uv(
        position, config,
    )
}

/// Place every chart in deterministic semantic and chart order.
pub(super) fn place(
    charts: &[ProjectedChart],
    config: &AtlasConfig,
) -> Result<Vec<PlacedChart>, SemanticTextureError> {
    let mut placed = Vec::with_capacity(charts.len());
    for region in BodyRegion::ALL {
        let region_charts = charts
            .iter()
            .filter(|chart| chart.region == region)
            .collect::<Vec<_>>();
        if region_charts.is_empty() {
            return Err(SemanticTextureError::MissingRequiredRegion(region));
        }
        let region_rect = grid::semantic_column(
            config, region,
        )?;
        let layout = grid::choose(
            region,
            region_rect,
            region_charts.len(),
            config.padding,
        )?;
        for (index, chart) in region_charts
            .into_iter()
            .enumerate()
        {
            let cell = grid::cell(
                region_rect,
                layout,
                index,
            )?;
            placed.push(
                mapping::map_chart(
                    chart, cell, config,
                )?,
            );
        }
    }
    Ok(placed)
}
