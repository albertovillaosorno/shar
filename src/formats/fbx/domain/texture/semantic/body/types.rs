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
//   - Types domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Types domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Types domain module.

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
    pub const ALL: [Self; 3] = [Self::Xy, Self::Xz, Self::Yz];

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
    pub const fn project(self, position: [f32; 3]) -> [f32; 2] {
        match self {
            Self::Xy | Self::SourceUv => [position[0], position[1]],
            Self::Xz => [position[0], position[2]],
            Self::Yz => [position[1], position[2]],
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
        self.x.checked_add(self.width.checked_sub(1)?)
    }

    /// Return the inclusive bottom coordinate.
    ///
    /// # Errors
    ///
    /// Returns `None` when checked arithmetic overflows.
    #[must_use]
    pub fn bottom(self) -> Option<u32> {
        self.y.checked_add(self.height.checked_sub(1)?)
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
