// File:
//   - manifest.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     structural_guide/manifest.rs
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
//   - The exact structural-guide manifest projection.
// - Must-Not:
//   - Read files, calculate geometry, or write artifacts.
// - Allows:
//   - Real counts, bounds, hashes, coordinate identity, and source coverage.
// - Summary:
//   - Renders deterministic manifest bytes after all other artifacts exist.
//
// Large file:
//   - false
//

//! Deterministic structural-guide manifest projection.

use fbx::adapters::driven::binary_structural_guide_writer::{
    STRUCTURAL_GUIDE_ASSET_NAME, STRUCTURAL_GUIDE_MATERIAL_NAME,
    STRUCTURAL_GUIDE_TEXTURE_PATH, StructuralGuideFbxSummary,
};
use serde::Serialize;

use super::atlas::{ATLAS_PADDING, ATLAS_SIZE};
use super::model::{GuidePlacement, GuideSourceCounts};
use crate::domain::PipelineError;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Manifest<'hashes> {
    schema_version: u32,
    asset_name: &'static str,
    fbx_version: &'static str,
    source_canonical: bool,
    reference_artifact: ReferenceArtifact,
    mirror_correction_applied: bool,
    final_transform_determinant_positive: bool,
    winding_corrected: bool,
    coordinate_system: CoordinateSystem,
    placement: Placement,
    mesh: Mesh,
    uv_channels: UvChannels,
    atlas: Atlas,
    source_to_unreal_matrix_row_major: [f32; 16],
    source_coverage: SourceCoverage,
    hashes: Hashes<'hashes>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReferenceArtifact {
    purpose: &'static str,
    fidelity_policy: &'static str,
    known_limitations: [&'static str; 4],
}

#[derive(Serialize)]
struct CoordinateSystem {
    units: &'static str,
    forward: &'static str,
    right: &'static str,
    up: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Placement {
    location_cm: [f32; 3],
    rotation_degrees: [f32; 3],
    scale: [f32; 3],
    sea_level_z_cm: f32,
    horizontal_center_cm: [f32; 2],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Mesh {
    object_count: u32,
    material_slot_count: u32,
    material_name: &'static str,
    triangulated: bool,
    collision_included: bool,
    lod_count: u32,
    vertex_count: usize,
    triangle_count: usize,
    bounds_min_cm: [f32; 3],
    bounds_max_cm: [f32; 3],
}

#[derive(Serialize)]
struct UvChannels {
    #[serde(rename = "0")]
    zero: &'static str,
    #[serde(rename = "1")]
    one: &'static str,
    #[serde(rename = "2")]
    two: &'static str,
    #[serde(rename = "3")]
    three: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Atlas {
    path: &'static str,
    width: u32,
    height: u32,
    padding_pixels: u32,
    rotated_entries: u32,
    alpha: bool,
    color_space: &'static str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SourceCoverage {
    input_meshes: usize,
    input_groups: usize,
    input_triangles: usize,
    removed_duplicate_triangles: usize,
    removed_degenerate_triangles: usize,
    repaired_normal_triangles: usize,
    wasp_meshes: usize,
    wasp_placements: usize,
    prop_like_meshes: usize,
    approximated_vertex_color_triangles: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Hashes<'hashes> {
    fbx_sha256: &'hashes str,
    atlas_sha256: &'hashes str,
    layout_sha256: &'hashes str,
}

pub(super) fn render(
    summary: StructuralGuideFbxSummary,
    counts: GuideSourceCounts,
    placement: GuidePlacement,
    wasp_placements: usize,
    fbx_sha256: &str,
    atlas_sha256: &str,
    layout_sha256: &str,
) -> Result<Vec<u8>, PipelineError> {
    let manifest = Manifest {
        schema_version: 1,
        asset_name: STRUCTURAL_GUIDE_ASSET_NAME,
        fbx_version: "7.7",
        source_canonical: true,
        reference_artifact: ReferenceArtifact {
            purpose: "Editor-only visual placement and Landscape sculpting \
                      guide",
            fidelity_policy: "Geometry, placement, UV tiling, and atlas \
                              identity are strict; dynamic presentation is \
                              intentionally approximate",
            known_limitations: [
                "Source alpha is flattened into opaque RGB and cannot \
                 reproduce cutout transparency",
                "Lighting, emissive animation, shader parameters, and \
                 time-of-day response are not represented",
                "Varying vertex colors are quantized to RGBA8 and replaced by \
                 one deterministic source-texture-wide average",
                "Zero-area triangles are omitted and unusable source normals \
                 use face normals; the artifact is not shipping content or \
                 gameplay collision authority",
            ],
        },
        mirror_correction_applied: true,
        final_transform_determinant_positive: true,
        winding_corrected: true,
        coordinate_system: CoordinateSystem {
            units: "centimeters",
            forward: "X",
            right: "Y",
            up: "Z",
        },
        placement: Placement {
            location_cm: [
                0.0, 0.0, 0.0,
            ],
            rotation_degrees: [
                0.0, 0.0, 0.0,
            ],
            scale: [
                1.0, 1.0, 1.0,
            ],
            sea_level_z_cm: 0.0,
            horizontal_center_cm: [
                0.0, 0.0,
            ],
        },
        mesh: Mesh {
            object_count: 1,
            material_slot_count: 1,
            material_name: STRUCTURAL_GUIDE_MATERIAL_NAME,
            triangulated: true,
            collision_included: false,
            lod_count: 1,
            vertex_count: summary.vertices,
            triangle_count: summary.triangles,
            bounds_min_cm: summary.bounds_min_cm,
            bounds_max_cm: summary.bounds_max_cm,
        },
        uv_channels: UvChannels {
            zero: "sourceUV",
            one: "atlasOffset",
            two: "atlasScale",
            three: "atlasFlags",
        },
        atlas: Atlas {
            path: STRUCTURAL_GUIDE_TEXTURE_PATH,
            width: ATLAS_SIZE,
            height: ATLAS_SIZE,
            padding_pixels: ATLAS_PADDING,
            rotated_entries: 0,
            alpha: false,
            color_space: "sRGB",
        },
        source_to_unreal_matrix_row_major: placement
            .source_to_unreal_matrix_row_major,
        source_coverage: SourceCoverage {
            input_meshes: counts.input_meshes,
            input_groups: counts.input_groups,
            input_triangles: counts.input_triangles,
            removed_duplicate_triangles: counts.removed_duplicate_triangles,
            removed_degenerate_triangles: counts.removed_degenerate_triangles,
            repaired_normal_triangles: counts.repaired_normal_triangles,
            wasp_meshes: counts.wasp_meshes,
            wasp_placements,
            prop_like_meshes: counts.prop_like_meshes,
            approximated_vertex_color_triangles: counts
                .approximated_vertex_color_triangles,
        },
        hashes: Hashes {
            fbx_sha256,
            atlas_sha256,
            layout_sha256,
        },
    };
    let mut bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    bytes.push(b'\n');
    Ok(bytes)
}
