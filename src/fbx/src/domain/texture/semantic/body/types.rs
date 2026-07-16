// File:
//   - types.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/types.rs
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
//   - Stable semantic body color, chart, rectangle, and complete plan values.
// - Must-Not:
//   - Classify evidence, pack charts, rasterize pixels, or mutate characters.
// - Allows:
//   - Immutable deterministic result metadata and one UV-remapped character.
// - Split-When:
//   - Chart geometry and manifest projection need independent public contracts.
// - Merge-When:
//   - Another body module owns the same result identities.
// - Summary:
//   - Semantic body-atlas result values.
// - Description:
//   - Exposes the complete pure-domain planning result to driven adapters.
// - Usage:
//   - Serialized by adapters and asserted by behavioral tests.
// - Defaults:
//   - Chart and color ordering is deterministic.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: result values remain together so adapters consume one stable plan
//   - contract without duplicating chart or evidence fields.
//

//! Deterministic semantic body-atlas result values.
use std::collections::BTreeMap;

use super::super::color::Rgba8;
use super::super::image::RgbaImage;
use super::super::region::{BodyRegion, BoneFamily};
use super::recipe::GroupAddress;
use crate::domain::character::CharacterAsset;

/// One source color classified through bone-family evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceColorAssignment {
    /// Exact source sRGB color.
    pub color: Rgba8,
    /// Assigned parent semantic region.
    pub region: BodyRegion,
    /// Vertex counts by dominant bone family.
    pub family_counts: BTreeMap<BoneFamily, u32>,
    /// True when reviewed recipe evidence resolved the classification.
    pub overridden: bool,
}

/// Orthographic axes used to unwrap one connected flat-color chart.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProjectionAxis {
    /// Project X and Y.
    Xy,
    /// Project X and Z.
    Xz,
    /// Project Y and Z.
    Yz,
    /// Preserve the source texture UV parameterization.
    SourceUv,
}

impl ProjectionAxis {
    /// Fixed projection evaluation order.
    pub const ALL: [Self; 3] = [
        Self::Xy,
        Self::Xz,
        Self::Yz,
    ];

    /// Return the stable manifest identity.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Xy => "xy",
            Self::Xz => "xz",
            Self::Yz => "yz",
            Self::SourceUv => "source-uv",
        }
    }

    /// Project one three-dimensional position into two dimensions.
    #[must_use]
    pub const fn project(
        self,
        position: [f32; 3],
    ) -> [f32; 2] {
        match self {
            Self::Xy | Self::SourceUv => [
                position[0],
                position[1],
            ],
            Self::Xz => [
                position[0],
                position[2],
            ],
            Self::Yz => [
                position[1],
                position[2],
            ],
        }
    }
}

/// One integer pixel rectangle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PixelRect {
    /// Left pixel coordinate.
    pub x: u32,
    /// Top pixel coordinate.
    pub y: u32,
    /// Rectangle width.
    pub width: u32,
    /// Rectangle height.
    pub height: u32,
}

impl PixelRect {
    /// Return the inclusive right coordinate.
    ///
    /// # Errors
    ///
    /// Returns `None` when checked arithmetic overflows.
    #[must_use]
    pub fn right(self) -> Option<u32> {
        self.x
            .checked_add(
                self.width
                    .checked_sub(1)?,
            )
    }

    /// Return the inclusive bottom coordinate.
    ///
    /// # Errors
    ///
    /// Returns `None` when checked arithmetic overflows.
    #[must_use]
    pub fn bottom(self) -> Option<u32> {
        self.y
            .checked_add(
                self.height
                    .checked_sub(1)?,
            )
    }
}

/// One connected semantic flat-color chart in the destination atlas.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtlasChart {
    /// Stable chart identity.
    pub id: String,
    /// Source primitive group.
    pub group: GroupAddress,
    /// Parent semantic region.
    pub region: BodyRegion,
    /// Exact preserved source color for flat charts.
    pub source_color: Rgba8,
    /// True when one or more triangles resample source UV evidence.
    pub sample_source: bool,
    /// Source triangle ordinals that resample original UV evidence.
    pub source_sampled_triangles: Vec<usize>,
    /// Source triangle ordinals included in the chart.
    pub triangle_indices: Vec<usize>,
    /// Source vertex ordinals included in the chart.
    pub vertex_indices: Vec<usize>,
    /// Chosen non-degenerate orthographic projection.
    pub projection: ProjectionAxis,
    /// Reserved chart cell including dilation space.
    pub cell: PixelRect,
    /// Aspect-preserving projected content rectangle.
    pub content: PixelRect,
}

/// Complete semantic body-atlas result.
#[derive(Clone, Debug, PartialEq)]
pub struct BodyTexturePlan {
    /// Opaque modern atlas with preserved source flat colors.
    pub atlas: RgbaImage,
    /// Character clone whose only changed values are selected-group UVs.
    pub remapped_character: CharacterAsset,
    /// Source-color evidence in deterministic color order.
    pub color_assignments: Vec<SourceColorAssignment>,
    /// Connected charts in deterministic atlas order.
    pub charts: Vec<AtlasChart>,
    /// Total source vertices in selected body groups.
    pub source_vertex_count: usize,
    /// Total source triangles in selected body groups.
    pub source_triangle_count: usize,
}
