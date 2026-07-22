// File:
//   - model.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     structural_guide/model.rs
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
//   - Structural-guide atlas, surface, layout, and publication records.
// - Must-Not:
//   - Read files, transform geometry, pack pixels, or serialize FBX.
// - Allows:
//   - Stable ordering, manifest projection, and exact atlas assignments.
// - Summary:
//   - Defines the deterministic structural-guide publication model.
//
// LARGE-FILE:
// - owner: Structural-guide publication model
// - reason: Atlas assignments, JSON projections, and source counters form one
//   stable cross-module vocabulary.
// - split: Pixel packing, mesh assembly, and manifest rendering remain
//   separate.
// - validation: Structural-guide model and publication tests.
// - review: Split if another guide artifact consumes a different schema.

//! Structural-guide atlas and publication records.

use std::collections::BTreeMap;

use serde::Serialize;

/// One material and wrap-mode presentation identity.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(super) struct SurfaceKey {
    /// Canonical material binding identity.
    pub(super) material: String,
    /// Whether source UVs repeat outside the unit range.
    pub(super) repeat: bool,
}

/// One useful atlas rectangle and wrap flag assigned to a surface.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct AtlasAssignment {
    /// First useful texel center normalized to atlas dimensions.
    pub(super) offset: [f32; 2],
    /// Useful texel-center span normalized to atlas dimensions.
    pub(super) scale: [f32; 2],
    /// One for repeat and zero for clamp.
    pub(super) repeat: f32,
    /// Whether this surface uses averaged source vertex color.
    pub(super) approximated_vertex_color: bool,
}

/// Complete atlas bytes, deterministic layout, and surface assignments.
pub(super) struct AtlasBuild {
    /// RGB8 sRGB PNG bytes.
    pub(super) png_bytes: Vec<u8>,
    /// Layout JSON payload before final hashing.
    pub(super) layout: AtlasLayout,
    /// Surface-to-tile lookup used while building four UV channels.
    pub(super) assignments: BTreeMap<SurfaceKey, AtlasAssignment>,
}

/// Deterministic atlas layout artifact.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AtlasLayout {
    pub(super) schema_version: u32,
    pub(super) atlas_width: u32,
    pub(super) atlas_height: u32,
    pub(super) padding_pixels: u32,
    pub(super) rotation_allowed: bool,
    pub(super) entries: Vec<AtlasLayoutEntry>,
}

/// One useful atlas tile, excluding its padding border.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct AtlasLayoutEntry {
    pub(super) source_name: String,
    pub(super) source_sha256: String,
    pub(super) variant_sha256: String,
    pub(super) presentation_bake: VertexColorBake,
    pub(super) x: u32,
    pub(super) y: u32,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) wrap_mode: &'static str,
}

/// Layout evidence for exact or approximated source vertex color.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(
    tag = "mode",
    rename_all = "camelCase"
)]
pub(super) enum VertexColorBake {
    /// One exact RGBA8 tint shared by every source vertex using the tile.
    Uniform {
        rgba8: [u8; 4],
        sample_count: u64,
    },
    /// One deterministic average across every use of one source texture.
    SourceAverage {
        rgba8: [u8; 4],
        sample_count: u64,
    },
}

impl VertexColorBake {
    /// Return the RGBA8 tint multiplied into the atlas pixels.
    #[must_use]
    pub(super) const fn rgba8(&self) -> [u8; 4] {
        match self {
            Self::Uniform {
                rgba8,
                ..
            }
            | Self::SourceAverage {
                rgba8,
                ..
            } => *rgba8,
        }
    }

    /// Return whether source vertex-color interpolation is approximated.
    #[must_use]
    pub(super) const fn is_approximate(&self) -> bool {
        matches!(
            self,
            Self::SourceAverage { .. }
        )
    }

    /// Return the number of source color samples represented by the bake.
    #[must_use]
    pub(super) const fn sample_count(&self) -> u64 {
        match self {
            Self::Uniform {
                sample_count,
                ..
            }
            | Self::SourceAverage {
                sample_count,
                ..
            } => *sample_count,
        }
    }

    /// Append a stable identity used for atlas variant hashing.
    pub(super) fn append_identity(
        &self,
        output: &mut Vec<u8>,
    ) {
        match self {
            Self::Uniform {
                rgba8,
                sample_count,
            } => {
                output.push(0);
                output.extend_from_slice(rgba8);
                output.extend_from_slice(&sample_count.to_le_bytes());
            }
            Self::SourceAverage {
                rgba8,
                sample_count,
            } => {
                output.push(1);
                output.extend_from_slice(rgba8);
                output.extend_from_slice(&sample_count.to_le_bytes());
            }
        }
    }
}

/// Exact affine evidence baked from source coordinates into Unreal space.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) struct GuidePlacement {
    /// Row-major row-vector matrix including axis, scale, centering, and sea
    /// level.
    pub(super) source_to_unreal_matrix_row_major: [f32; 16],
}

/// Geometry and source-coverage evidence used by the final manifest.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(super) struct GuideSourceCounts {
    pub(super) input_meshes: usize,
    pub(super) input_groups: usize,
    pub(super) input_triangles: usize,
    pub(super) removed_duplicate_triangles: usize,
    /// Zero-area source triangles omitted from the visual guide.
    pub(super) removed_degenerate_triangles: usize,
    /// Retained triangles whose unusable source normals used the face normal.
    pub(super) repaired_normal_triangles: usize,
    pub(super) wasp_meshes: usize,
    pub(super) prop_like_meshes: usize,
    /// Retained triangles using documented source-texture-wide vertex-color
    /// averages.
    pub(super) approximated_vertex_color_triangles: usize,
}
