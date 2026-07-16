// File:
//   - model.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/charts/model.rs
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
//   - Internal projected and placed chart values shared by body planning
//   - stages.
// - Must-Not:
//   - Discover topology, select projections, pack rectangles, or mutate images.
// - Allows:
//   - Deterministic chart metadata and per-vertex projected or pixel positions.
// - Split-When:
//   - Public chart values need a different representation from planner state.
// - Merge-When:
//   - One chart stage becomes the only consumer of these internal values.
// - Summary:
//   - Internal semantic chart planning values.
// - Description:
//   - Carries exact chart evidence between discovery, packing, and
//   - rasterization.
// - Usage:
//   - Private to semantic body chart planning.
// - Defaults:
//   - Vertex and triangle collections remain sorted.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Internal semantic chart planning values.

use std::collections::BTreeMap;

use super::super::super::color::Rgba8;
use super::super::super::region::BodyRegion;
use super::super::recipe::GroupAddress;
use super::super::types::{AtlasChart, ProjectionAxis};

/// Projected two-dimensional bounds for one chart.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::domain::texture::semantic::body) struct ProjectionBounds {
    /// Minimum projected coordinate on each chart axis.
    pub(in crate::domain::texture::semantic::body) minimum: [f32; 2],
    /// Maximum projected coordinate on each chart axis.
    pub(in crate::domain::texture::semantic::body) maximum: [f32; 2],
}

impl ProjectionBounds {
    /// Return the projected horizontal span.
    #[must_use]
    pub(super) fn width(self) -> f32 {
        self.maximum[0] - self.minimum[0]
    }

    /// Return the projected vertical span.
    #[must_use]
    pub(super) fn height(self) -> f32 {
        self.maximum[1] - self.minimum[1]
    }
}

/// One connected flat-color chart before atlas placement.
#[derive(Clone, Debug)]
pub(super) struct ProjectedChart {
    /// Stable chart identity rendered into the semantic manifest.
    pub(super) id: String,
    /// Source primitive-group address owned by this chart.
    pub(super) group: GroupAddress,
    /// Semantic body region assigned to this chart.
    pub(super) region: BodyRegion,
    /// Authored source color preserved by flat-color rasterization.
    pub(super) source_color: Rgba8,
    /// Whether pixels sample the source texture instead of a flat color.
    pub(super) sample_source: bool,
    /// Triangle indices that retain source-texture sampling.
    pub(super) source_sampled_triangles: Vec<usize>,
    /// Complete source triangle membership for the chart.
    pub(super) triangle_indices: Vec<usize>,
    /// Complete source vertex membership for the chart.
    pub(super) vertex_indices: Vec<usize>,
    /// Projection axis selected by deterministic geometric scoring.
    pub(super) projection: ProjectionAxis,
    /// Projected position for every source vertex in the chart.
    pub(super) projected_positions: BTreeMap<usize, [f32; 2]>,
    /// Projected bounds used for atlas scaling and placement.
    pub(super) bounds: ProjectionBounds,
}

/// Exact atlas placement for one full source-UV texture block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain::texture::semantic::body) struct SourceUvPlacement {
    /// Atlas pixel origin of the preserved source-UV block.
    pub(in crate::domain::texture::semantic::body) origin: [u32; 2],
    /// Integer scale applied uniformly to preserved source UVs.
    pub(in crate::domain::texture::semantic::body) scale: u32,
}

/// One atlas-placed chart with exact destination pixel positions.
#[derive(Clone, Debug)]
pub(in crate::domain::texture::semantic::body) struct PlacedChart {
    /// Public manifest chart rendered from this internal placement.
    pub(in crate::domain::texture::semantic::body) public: AtlasChart,
    /// Exact destination pixel position for every chart vertex.
    pub(in crate::domain::texture::semantic::body) pixel_positions:
        BTreeMap<usize, [f32; 2]>,
    /// Preserved source-UV placement for source-sampled charts.
    pub(in crate::domain::texture::semantic::body) source_uv_placement:
        Option<SourceUvPlacement>,
}
