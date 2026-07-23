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
//   - Claim guide-owned spatial corrections or inspect generated files.
// - Allows:
//   - World-FBX scene identity, lossless-combination evidence, atlas metadata,
//     source counts, bounds, and hashes.
// - Summary:
//   - Proves that the guide only combines normal world-FBX geometry.
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
use super::model::GuideSourceCounts;
use crate::domain::PipelineError;

pub(super) const SOURCE_GEOMETRY_POLICY: &str = concat!(
    "concatenate evaluated normal-world FBX mesh channels under one ",
    "shared ReflectX root without root re-expression",
);

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Manifest<'hashes> {
    schema_version: u32,
    asset_name: &'static str,
    fbx_version: &'static str,
    source_canonical: bool,
    purpose: &'static str,
    source_geometry_policy: &'static str,
    spatial_changes: SpatialChanges,
    world_fbx_scene: WorldFbxScene,
    unreal_import: UnrealImport,
    world_height: WorldHeight,
    mesh: Mesh,
    uv_channels: UvChannels,
    atlas: Atlas,
    source_coverage: SourceCoverage,
    hashes: Hashes<'hashes>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SpatialChanges {
    centered: bool,
    mirrored_by_guide: bool,
    scaled_by_guide: bool,
    height_adjusted_by_guide: bool,
    triangle_deduplication: bool,
    normals_repaired: bool,
    guide_only_geometry_added: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorldFbxScene {
    units: &'static str,
    unit_scale_factor: f32,
    up_axis: &'static str,
    front_axis: &'static str,
    coordinate_axis: &'static str,
    guide_export_root_policy: &'static str,
    guide_export_root_rotation_degrees: [f32; 3],
    guide_export_root_scale: [f32; 3],
    exterior_export_root_policy: &'static str,
    interior_export_root_policy: &'static str,
    world_reflection_axis: &'static str,
    source_roots_flattened_into_guide_mesh: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UnrealImport {
    force_front_x_axis: bool,
    location: [f32; 3],
    rotation: [f32; 3],
    scale: [f32; 3],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WorldHeight {
    meters: f32,
    owned_by_normal_world_fbx: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Mesh {
    object_count: u32,
    material_slot_count: u32,
    material_name: &'static str,
    triangulated: bool,
    normal_layer_included: bool,
    collision_included: bool,
    lod_count: u32,
    vertex_count: usize,
    triangle_count: usize,
    bounds_min_meters: [f32; 3],
    bounds_max_meters: [f32; 3],
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
    groups_without_normals: usize,
    input_triangles: usize,
    wasp_meshes_from_world_fbx: usize,
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
    fbx_sha256: &str,
    atlas_sha256: &str,
    layout_sha256: &str,
) -> Result<Vec<u8>, PipelineError> {
    let manifest = Manifest {
        schema_version: 4,
        asset_name: STRUCTURAL_GUIDE_ASSET_NAME,
        fbx_version: "7.7",
        source_canonical: true,
        purpose: "Editor-only combined view of generated world FBXs",
        source_geometry_policy: SOURCE_GEOMETRY_POLICY,
        spatial_changes: SpatialChanges {
            centered: false,
            mirrored_by_guide: false,
            scaled_by_guide: false,
            height_adjusted_by_guide: false,
            triangle_deduplication: false,
            normals_repaired: false,
            guide_only_geometry_added: false,
        },
        world_fbx_scene: WorldFbxScene {
            units: "meters",
            unit_scale_factor: 100.0,
            up_axis: "Y",
            front_axis: "Z",
            coordinate_axis: "X",
            guide_export_root_policy: "ReflectX",
            guide_export_root_rotation_degrees: [
                0.0, 0.0, 0.0,
            ],
            guide_export_root_scale: [
                -1.0, 1.0, 1.0,
            ],
            exterior_export_root_policy: "ReflectX",
            interior_export_root_policy: "ReflectX",
            world_reflection_axis: "X",
            source_roots_flattened_into_guide_mesh: false,
        },
        unreal_import: UnrealImport {
            force_front_x_axis: false,
            location: [
                0.0, 0.0, 0.0,
            ],
            rotation: [
                0.0, 0.0, 0.0,
            ],
            scale: [
                1.0, 1.0, 1.0,
            ],
        },
        world_height: WorldHeight {
            meters: super::super::movement::WORLD_HEIGHT_OFFSET_METERS,
            owned_by_normal_world_fbx: true,
        },
        mesh: Mesh {
            object_count: 1,
            material_slot_count: 1,
            material_name: STRUCTURAL_GUIDE_MATERIAL_NAME,
            triangulated: true,
            normal_layer_included: counts.groups_without_normals == 0,
            collision_included: false,
            lod_count: 1,
            vertex_count: summary.vertices,
            triangle_count: summary.triangles,
            bounds_min_meters: summary.bounds_min_meters,
            bounds_max_meters: summary.bounds_max_meters,
        },
        uv_channels: UvChannels {
            zero: "finalAtlasUV",
            one: "sourceUV",
            two: "atlasOffset",
            three: "atlasScale",
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
        source_coverage: SourceCoverage {
            input_meshes: counts.input_meshes,
            input_groups: counts.input_groups,
            groups_without_normals: counts.groups_without_normals,
            input_triangles: counts.input_triangles,
            wasp_meshes_from_world_fbx: counts.wasp_meshes,
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
