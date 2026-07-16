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
    pub(in crate::domain::texture::semantic::body) minimum: [f32; 2],
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
    pub(super) id: String,
    pub(super) group: GroupAddress,
    pub(super) region: BodyRegion,
    pub(super) source_color: Rgba8,
    pub(super) sample_source: bool,
    pub(super) source_sampled_triangles: Vec<usize>,
    pub(super) triangle_indices: Vec<usize>,
    pub(super) vertex_indices: Vec<usize>,
    pub(super) projection: ProjectionAxis,
    pub(super) projected_positions: BTreeMap<usize, [f32; 2]>,
    pub(super) bounds: ProjectionBounds,
}

/// Exact atlas placement for one full source-UV texture block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain::texture::semantic::body) struct SourceUvPlacement {
    pub(in crate::domain::texture::semantic::body) origin: [u32; 2],
    pub(in crate::domain::texture::semantic::body) scale: u32,
}

/// One atlas-placed chart with exact destination pixel positions.
#[derive(Clone, Debug)]
pub(in crate::domain::texture::semantic::body) struct PlacedChart {
    pub(in crate::domain::texture::semantic::body) public: AtlasChart,
    pub(in crate::domain::texture::semantic::body) pixel_positions:
        BTreeMap<usize, [f32; 2]>,
    pub(in crate::domain::texture::semantic::body) source_uv_placement:
        Option<SourceUvPlacement>,
}
