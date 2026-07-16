// File:
//   - manifest.rs
// Path:
//   - src/fbx/src/adapters/driven/semantic_character_texture/manifest.rs
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
//   - Deterministic JSON projection of semantic body and eye planning evidence.
// - Must-Not:
//   - Read files, classify assets, encode PNG, or include local source paths.
// - Allows:
//   - Stable public identities, counts, rectangles, colors, selected UVs, and
//   - artifact file names.
// - Split-When:
//   - Body and eye manifests become independent published artifacts.
// - Merge-When:
//   - Another adapter owns the same semantic texture manifest schema.
// - Summary:
//   - Semantic character texture manifest renderer.
// - Description:
//   - Records enough evidence to compare repeated preparation and later bind
//   - the generated textures without publishing workstation routes.
// - Usage:
//   - Called after both pure-domain plans and PNG encodes succeed.
// - Defaults:
//   - Schema version is explicit and output ends with one newline.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: body evidence, chart placement, remapped UVs, eye evidence, and
//   - artifact identities form one deterministic manifest schema.
//

//! Deterministic semantic character texture manifest renderer.
use serde_json::{Value, json};

use super::request::SemanticTextureRequest;
use crate::domain::texture::semantic::{
    BodyTexturePlan, EyeSemanticPlan, PixelRect, Rgba8,
};

/// Render one deterministic manifest without local source paths.
///
/// # Errors
///
/// Returns an error when selected group UVs cannot be resolved or JSON
/// serialization fails.
pub(super) fn render(
    request: &SemanticTextureRequest,
    body: &BodyTexturePlan,
    eye: &EyeSemanticPlan,
    eye_profile_sha256: &str,
) -> Result<Vec<u8>, String> {
    let selected_uvs = selected_uvs(
        request, body,
    )?;
    let assignments = assignments(body);
    let charts = charts(body);
    let eye_components = eye_components(eye);
    let eye_frames = eye_frames(eye);
    let changed_character_fields =
        if request.body_texture_mode == "semantic-atlas" {
            vec!["selected-group-uvs"]
        } else {
            Vec::new()
        };
    let manifest = json!({
        "schema_version": 1,
        "character_id": request.character_name,
        "topology_policy": {
            "polygon_or_vertex_increase": false,
            "changed_character_fields": changed_character_fields,
        },
        "body": {
            "mode": request.body_texture_mode,
            "artifact": "textures/body-atlas.png",
            "width": body.atlas.width(),
            "height": body.atlas.height(),
            "padding": request.body_atlas_padding,
            "texture_address_mode": request.body_texture_address_mode,
            "source_vertex_count": body.source_vertex_count,
            "source_triangle_count": body.source_triangle_count,
            "semantic_region_count": 5,
            "color_assignments": assignments,
            "charts": charts,
            "selected_group_uvs": selected_uvs,
        },
        "eyes": {
            "eye_group": {
                "part_index": request.eye_group.part_index,
                "group_index": request.eye_group.group_index,
            },
            "semantic_region_count": eye.semantic_region_count,
            "profile_sha256": eye_profile_sha256,
            "canonical_layers": {
                "sclera_rgba": rgba(eye.surface_color),
                "pupil": "textures/eye-pupil.png",
                "lids": "textures/eye-lids.png",
                "upper_lid_uv_rect": [0.0, 0.0, 1.0, 0.5],
                "lower_lid_uv_rect": [0.0, 0.5, 1.0, 1.0],
            },
            "derived_open_eye": "textures/eye.png",
            "lid_rgba": rgba(eye.lid_color),
            "sclera_rgba": rgba(eye.surface_color),
            "pupil_rgba": rgba(eye.pupil_color),
            "components": eye_components,
            "derived_compatibility_frames": eye_frames,
            "animation_changes": false,
        },
    });
    let mut bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| format!("semantic manifest encode failed: {error}"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// Render selected group UV arrays without exposing local source paths.
fn selected_uvs(
    request: &SemanticTextureRequest,
    body: &BodyTexturePlan,
) -> Result<Vec<Value>, String> {
    request
        .body_groups
        .iter()
        .map(
            |address| {
                let group = body
                    .remapped_character
                    .parts
                    .get(address.part_index)
                    .and_then(
                        |part| {
                            part.mesh
                                .groups
                                .get(address.group_index)
                        },
                    )
                    .ok_or_else(
                        || {
                            format!(
                                "manifest group missing: part={}, group={}",
                                address.part_index, address.group_index,
                            )
                        },
                    )?;
                Ok(
                    json!({
                        "part_index": address.part_index,
                        "group_index": address.group_index,
                        "uvs": group.uvs,
                    }),
                )
            },
        )
        .collect()
}

/// Render deterministic source-color assignments.
fn assignments(body: &BodyTexturePlan) -> Vec<Value> {
    body.color_assignments
        .iter()
        .map(
            |assignment| {
                let families = assignment
                    .family_counts
                    .iter()
                    .map(
                        |(family, count)| {
                            json!({
                                "family": family.as_str(),
                                "vertex_count": count,
                            })
                        },
                    )
                    .collect::<Vec<_>>();
                json!({
                    "rgba": rgba(assignment.color),
                    "region": assignment.region.as_str(),
                    "overridden": assignment.overridden,
                    "family_counts": families,
                })
            },
        )
        .collect()
}

/// Render deterministic body chart placement and raster evidence.
fn charts(body: &BodyTexturePlan) -> Vec<Value> {
    body.charts
        .iter()
        .map(
            |chart| {
                json!({
                    "id": chart.id,
                    "part_index": chart.group.part_index,
                    "group_index": chart.group.group_index,
                    "region": chart.region.as_str(),
                    "source_rgba": rgba(chart.source_color),
                    "raster_mode": if chart.sample_source {
                        "source-sampled"
                    } else {
                        "flat-color"
                    },
                    "source_sampled_triangle_indices":
                        chart.source_sampled_triangles,
                    "projection": chart.projection.as_str(),
                    "triangle_indices": chart.triangle_indices,
                    "vertex_indices": chart.vertex_indices,
                    "cell": rectangle(chart.cell),
                    "content": rectangle(chart.content),
                })
            },
        )
        .collect()
}

/// Render the two disconnected semantic eye components.
fn eye_components(eye: &EyeSemanticPlan) -> Vec<Value> {
    eye.components
        .iter()
        .map(
            |component| {
                json!({
                    "side": component.side.as_str(),
                    "centroid_x": component.centroid_x,
                    "vertex_indices": component.vertex_indices,
                    "regions": [
                        "sclera",
                        "pupil",
                        "upper-lid",
                        "lower-lid",
                    ],
                })
            },
        )
        .collect()
}

/// Render deterministic four-frame blink evidence.
fn eye_frames(eye: &EyeSemanticPlan) -> Vec<Value> {
    eye.frame_evidence
        .iter()
        .map(
            |evidence| {
                json!({
                    "frame_index": evidence.frame_index,
                    "lid_pixel_count": evidence.lid_pixel_count,
                    "upper_lid_pixel_count": evidence.upper_lid_pixel_count,
                    "lower_lid_pixel_count": evidence.lower_lid_pixel_count,
                    "preserved_pupil_pixel_count":
                        evidence.preserved_pupil_pixel_count,
                })
            },
        )
        .collect()
}

/// Project one exact color into manifest channel order.
const fn rgba(color: Rgba8) -> [u8; 4] {
    color.channels()
}

/// Project one integer rectangle into named manifest fields.
fn rectangle(rectangle: PixelRect) -> Value {
    json!({
        "x": rectangle.x,
        "y": rectangle.y,
        "width": rectangle.width,
        "height": rectangle.height,
    })
}
