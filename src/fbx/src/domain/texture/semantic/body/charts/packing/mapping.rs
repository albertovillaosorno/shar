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
use super::super::super::types::{AtlasChart, PixelRect, ProjectionAxis};
use super::super::model::{PlacedChart, ProjectedChart, SourceUvPlacement};

/// Fit one projected chart into one padded cell and calculate pixel positions.
pub(super) fn map_chart(
    chart: &ProjectedChart,
    cell: PixelRect,
    config: &AtlasConfig,
    source_texture_size: [u32; 2],
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
    if chart.projection == ProjectionAxis::SourceUv {
        return map_source_uv_chart(
            chart,
            cell,
            inner,
            source_texture_size,
        );
    }
    map_orthographic_chart(
        chart, cell, inner,
    )
}

/// Fit one orthographically projected chart into the padded cell.
#[expect(
    clippy::cast_possible_truncation,
    reason = "Mapped coordinates are finite and bounded by checked atlas \
              pixel rectangles before conversion to f32."
)]
fn map_orthographic_chart(
    chart: &ProjectedChart,
    cell: PixelRect,
    inner: PixelRect,
) -> Result<PlacedChart, SemanticTextureError> {
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
        let x = (f64::from(projected[0])
            - f64::from(
                chart
                    .bounds
                    .minimum[0],
            ))
        .mul_add(
            scale, origin[0],
        );
        let y = (f64::from(
            chart
                .bounds
                .maximum[1],
        ) - f64::from(projected[1]))
        .mul_add(
            scale, origin[1],
        );
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
            public: public_chart(
                chart, cell, content,
            ),
            pixel_positions,
            source_uv_placement: None,
        },
    )
}

/// Map one source-UV chart into an exact integer texel-multiple block.
#[expect(
    clippy::cast_precision_loss,
    reason = "Checked u32 atlas coordinates are represented as f32 UV source \
              positions to match the mesh domain."
)]
fn map_source_uv_chart(
    chart: &ProjectedChart,
    cell: PixelRect,
    inner: PixelRect,
    source_texture_size: [u32; 2],
) -> Result<PlacedChart, SemanticTextureError> {
    let [
        source_width,
        source_height,
    ] = source_texture_size;
    if source_width == 0 || source_height == 0 {
        return Err(SemanticTextureError::NumericOverflow);
    }
    let scale = (inner.width / source_width).min(inner.height / source_height);
    if scale == 0 {
        return Err(SemanticTextureError::RegionGridTooSmall(chart.region));
    }
    let width = source_width
        .checked_mul(scale)
        .ok_or(SemanticTextureError::NumericOverflow)?;
    let height = source_height
        .checked_mul(scale)
        .ok_or(SemanticTextureError::NumericOverflow)?;
    let origin = [
        inner
            .x
            .checked_add((inner.width - width) / 2)
            .ok_or(SemanticTextureError::NumericOverflow)?,
        inner
            .y
            .checked_add((inner.height - height) / 2)
            .ok_or(SemanticTextureError::NumericOverflow)?,
    ];
    let mut pixel_positions = BTreeMap::new();
    for (vertex, uv) in &chart.projected_positions {
        pixel_positions.insert(
            *vertex,
            [
                origin[0] as f32
                    + source_axis(
                        uv[0], width,
                    )?,
                origin[1] as f32
                    + source_axis(
                        1.0 - uv[1],
                        height,
                    )?,
            ],
        );
    }
    Ok(
        PlacedChart {
            public: public_chart(
                chart,
                cell,
                PixelRect {
                    x: origin[0],
                    y: origin[1],
                    width,
                    height,
                },
            ),
            pixel_positions,
            source_uv_placement: Some(
                SourceUvPlacement {
                    origin,
                    scale,
                },
            ),
        },
    )
}

/// Build the public chart evidence shared by every placement policy.
fn public_chart(
    chart: &ProjectedChart,
    cell: PixelRect,
    content: PixelRect,
) -> AtlasChart {
    AtlasChart {
        id: chart
            .id
            .clone(),
        group: chart.group,
        region: chart.region,
        source_color: chart.source_color,
        sample_source: chart.sample_source,
        source_sampled_triangles: chart
            .source_sampled_triangles
            .clone(),
        triangle_indices: chart
            .triangle_indices
            .clone(),
        vertex_indices: chart
            .vertex_indices
            .clone(),
        projection: chart.projection,
        cell,
        content,
    }
}

/// Map one normalized source coordinate inside an exact texel block.
fn source_axis(
    value: f32,
    extent: u32,
) -> Result<f32, SemanticTextureError> {
    const OWNERSHIP_BIAS: f32 = 1.0e-4;
    let extent = u16::try_from(extent)
        .map(f32::from)
        .map_err(|_error| SemanticTextureError::NumericOverflow)?;
    let maximum = extent - OWNERSHIP_BIAS;
    Ok(
        value
            .mul_add(
                extent,
                OWNERSHIP_BIAS,
            )
            .clamp(
                OWNERSHIP_BIAS,
                maximum,
            ),
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
    projection: ProjectionAxis,
) -> [f32; 2] {
    if projection == ProjectionAxis::SourceUv {
        return [
            position[0] / config.width as f32,
            1.0 - position[1] / config.height as f32,
        ];
    }
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
