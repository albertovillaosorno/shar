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
//   - Error domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Error domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Error domain module.

use super::super::color::Rgba8;
use super::super::image::RgbaImageError;
use super::super::region::BodyRegion;
use super::recipe::GroupAddress;

/// Semantic texture planning failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SemanticTextureError {
    /// Image-domain validation failed.
    Image(RgbaImageError),
    /// Recipe contained no body primitive groups.
    MissingBodyGroups,
    /// Recipe repeated one body primitive group.
    DuplicateBodyGroup,
    /// Hair luminance ratio was non-finite or outside zero through one.
    InvalidHairLuminanceRatio,
    /// Atlas dimensions overflowed checked arithmetic.
    AtlasDimensionsOverflow,
    /// Atlas dimensions could not contain required regions and padding.
    AtlasTooSmall {
        /// Requested width.
        width: u32,
        /// Requested height.
        height: u32,
        /// Requested chart padding.
        padding: u32,
    },
    /// Opaque body materials cannot use a transparent neutral background.
    TransparentBodyAtlasBackground,
    /// Character part address did not exist.
    MissingPart(GroupAddress),
    /// Primitive-group address did not exist.
    MissingGroup(GroupAddress),
    /// Selected group did not contain a complete UV channel.
    MissingGroupUvs(GroupAddress),
    /// One selected vertex did not have a dominant skin influence.
    MissingDominantInfluence {
        /// Source group.
        group: GroupAddress,
        /// Source vertex ordinal.
        vertex: usize,
    },
    /// One dominant bone could not support semantic classification.
    UnsupportedBoneEvidence {
        /// Source color under classification.
        color: Rgba8,
        /// Unsupported bone identity.
        bone_id: String,
    },
    /// Equal evidence supported more than one semantic family.
    AmbiguousColorEvidence(Rgba8),
    /// The first opaque body-atlas lane encountered source transparency.
    TransparentSourceBodyColor(Rgba8),
    /// Triangle vertices sampled more than one exact source color.
    MixedSourceColorTriangle {
        /// Source group.
        group: GroupAddress,
        /// Source triangle ordinal.
        triangle: usize,
    },
    /// Triangle vertices resolved to more than one semantic region.
    MixedSemanticTriangle {
        /// Source group.
        group: GroupAddress,
        /// Source triangle ordinal.
        triangle: usize,
    },
    /// One source vertex was not covered by any selected triangle chart.
    UncoveredVertex {
        /// Source group.
        group: GroupAddress,
        /// Source vertex ordinal.
        vertex: usize,
    },
    /// One source vertex was assigned to conflicting charts.
    ConflictingVertexChart {
        /// Source group.
        group: GroupAddress,
        /// Source vertex ordinal.
        vertex: usize,
    },
    /// No orthographic projection preserved every triangle in one chart.
    DegenerateChartProjection(String),
    /// One region cell could not hold its chart grid and padding.
    RegionGridTooSmall(BodyRegion),
    /// Rasterization produced no covered destination pixel for one chart.
    EmptyRasterizedChart(String),
    /// Checked integer or index conversion failed.
    NumericOverflow,
}

impl From<RgbaImageError> for SemanticTextureError {
    fn from(error: RgbaImageError) -> Self {
        Self::Image(error)
    }
}
