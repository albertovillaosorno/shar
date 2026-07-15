// File:
//   - mapping.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/charts/packing/mapping.rs
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
//   - Aspect-preserving mapping from projected chart coordinates into one
//   - padded atlas cell and V-up destination UVs.
// - Must-Not:
//   - Select grids, discover charts, alter topology, or rasterize pixels.
// - Allows:
//   - Checked coordinate conversion and public chart placement metadata.
// - Split-When:
//   - A different coordinate normalization policy becomes supported.
// - Merge-When:
//   - Grid selection becomes the sole owner of chart mapping.
// - Summary:
//   - Projected chart coordinate mapping.
// - Description:
//   - Centers each projection in its cell while preserving chart aspect ratio.
// - Usage:
//   - Called by the packing facade and UV-remapping transaction.
// - Defaults:
//   - Image rows are top-down and destination mesh UV coordinates are V-up.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: checked cell inset, aspect fit, pixel bounds, and UV conversion
//   - form one coordinate-mapping contract.
//

//! Aspect-preserving projected chart coordinate mapping.
use std::collections::BTreeMap;

use super::super::super::error::SemanticTextureError;
use super::super::super::recipe::AtlasConfig;
use super::super::super::types::{AtlasChart, PixelRect};
use super::super::model::{PlacedChart, ProjectedChart};

/// Fit one projected chart into one padded cell and calculate pixel positions.
#[expect(
    clippy::cast_possible_truncation,
    reason = "Mapped coordinates are finite and bounded by checked atlas \
              pixel rectangles before conversion to f32."
)]
pub(super) fn map_chart(
    chart: &ProjectedChart,
    cell: PixelRect,
    config: &AtlasConfig,
) -> Result<PlacedChart, SemanticTextureError> {
    let inset = config
        .padding
        .checked_mul(2)
        .ok_or(SemanticTextureError::NumericOverflow)?;
    let inner = PixelRect {
        x: cell
            .x
            .checked_add(config.padding)
            .ok_or(SemanticTextureError::NumericOverflow)?,
        y: cell
            .y
            .checked_add(config.padding)
            .ok_or(SemanticTextureError::NumericOverflow)?,
        width: cell
            .width
            .checked_sub(inset)
            .ok_or(SemanticTextureError::RegionGridTooSmall(chart.region))?,
        height: cell
            .height
            .checked_sub(inset)
            .ok_or(SemanticTextureError::RegionGridTooSmall(chart.region))?,
    };
    let available = [
        f64::from(inner.width - 1),
        f64::from(inner.height - 1),
    ];
    let spans = [
        f64::from(
            chart
                .bounds
                .width(),
        ),
        f64::from(
            chart
                .bounds
                .height(),
        ),
    ];
    if spans[0] <= 0.0 || spans[1] <= 0.0 {
        return Err(
            SemanticTextureError::DegenerateChartProjection(
                chart
                    .id
                    .clone(),
            ),
        );
    }
    let scale = (available[0] / spans[0]).min(available[1] / spans[1]);
    let mapped_size = [
        spans[0] * scale,
        spans[1] * scale,
    ];
    let origin = [
        f64::from(inner.x) + (available[0] - mapped_size[0]) / 2.0,
        f64::from(inner.y) + (available[1] - mapped_size[1]) / 2.0,
    ];
    let mut pixel_positions = BTreeMap::new();
    for (vertex, projected) in &chart.projected_positions {
        let x = origin[0]
            + (f64::from(projected[0])
                - f64::from(
                    chart
                        .bounds
                        .minimum[0],
                ))
                * scale;
        let y = origin[1]
            + (f64::from(
                chart
                    .bounds
                    .maximum[1],
            ) - f64::from(projected[1]))
                * scale;
        pixel_positions.insert(
            *vertex,
            [
                x as f32, y as f32,
            ],
        );
    }
    let content = pixel_bounds(&pixel_positions)?;
    Ok(
        PlacedChart {
            public: AtlasChart {
                id: chart
                    .id
                    .clone(),
                group: chart.group,
                region: chart.region,
                source_color: chart.source_color,
                triangle_indices: chart
                    .triangle_indices
                    .clone(),
                vertex_indices: chart
                    .vertex_indices
                    .clone(),
                projection: chart.projection,
                cell,
                content,
            },
            pixel_positions,
        },
    )
}

/// Convert one destination pixel position into V-up atlas UV coordinates.
#[expect(
    clippy::cast_precision_loss,
    reason = "Atlas dimensions are bounded u32 pixel counts and conversion to \
              f32 matches the existing mesh UV representation."
)]
pub(super) fn atlas_uv(
    position: [f32; 2],
    config: &AtlasConfig,
) -> [f32; 2] {
    [
        position[0] / (config.width - 1) as f32,
        1.0 - position[1] / (config.height - 1) as f32,
    ]
}

/// Calculate integer bounds that contain every mapped destination vertex.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "Mapped chart coordinates are finite and non-negative inside one \
              checked atlas cell before floor or ceiling conversion."
)]
fn pixel_bounds(
    positions: &BTreeMap<usize, [f32; 2]>
) -> Result<PixelRect, SemanticTextureError> {
    let mut minimum = [f32::INFINITY; 2];
    let mut maximum = [f32::NEG_INFINITY; 2];
    for position in positions.values() {
        minimum[0] = minimum[0].min(position[0]);
        minimum[1] = minimum[1].min(position[1]);
        maximum[0] = maximum[0].max(position[0]);
        maximum[1] = maximum[1].max(position[1]);
    }
    if !minimum[0].is_finite()
        || !minimum[1].is_finite()
        || !maximum[0].is_finite()
        || !maximum[1].is_finite()
    {
        return Err(SemanticTextureError::NumericOverflow);
    }
    let left = minimum[0].floor() as u32;
    let top = minimum[1].floor() as u32;
    let right = maximum[0].ceil() as u32;
    let bottom = maximum[1].ceil() as u32;
    Ok(
        PixelRect {
            x: left,
            y: top,
            width: right
                .checked_sub(left)
                .and_then(|value| value.checked_add(1))
                .ok_or(SemanticTextureError::NumericOverflow)?,
            height: bottom
                .checked_sub(top)
                .and_then(|value| value.checked_add(1))
                .ok_or(SemanticTextureError::NumericOverflow)?,
        },
    )
}
