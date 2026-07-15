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
) -> Result<Vec<u8>, String> {
    let selected_uvs = request
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
        .collect::<Result<Vec<Value>, String>>()?;
    let assignments = body
        .color_assignments
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
        .collect::<Vec<_>>();
    let charts = body
        .charts
        .iter()
        .map(
            |chart| {
                json!({
                    "id": chart.id,
                    "part_index": chart.group.part_index,
                    "group_index": chart.group.group_index,
                    "region": chart.region.as_str(),
                    "source_rgba": rgba(chart.source_color),
                    "projection": chart.projection.as_str(),
                    "triangle_indices": chart.triangle_indices,
                    "vertex_indices": chart.vertex_indices,
                    "cell": rectangle(chart.cell),
                    "content": rectangle(chart.content),
                })
            },
        )
        .collect::<Vec<_>>();
    let eye_components = eye
        .components
        .iter()
        .map(
            |component| {
                json!({
                    "side": component.side.as_str(),
                    "centroid_x": component.centroid_x,
                    "vertex_indices": component.vertex_indices,
                    "regions": [
                        "upper-lid",
                        "lower-lid",
                        "surface",
                        "pupil-iris",
                    ],
                })
            },
        )
        .collect::<Vec<_>>();
    let eye_frames = eye
        .frame_evidence
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
                    "artifact": format!(
                        "eye-frame-{}.png",
                        evidence.frame_index,
                    ),
                })
            },
        )
        .collect::<Vec<_>>();
    let manifest = json!({
        "schema_version": 1,
        "character_id": request.character_name,
        "topology_policy": {
            "polygon_or_vertex_increase": false,
            "changed_character_fields": ["selected-group-uvs"],
        },
        "body": {
            "artifact": "body-atlas.png",
            "width": body.atlas.width(),
            "height": body.atlas.height(),
            "padding": request.body_atlas_padding,
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
            "lid_rgba": rgba(eye.lid_color),
            "surface_rgba": rgba(eye.surface_color),
            "pupil_iris_rgba": rgba(eye.pupil_color),
            "components": eye_components,
            "frames": eye_frames,
            "animation_mechanism": "source-texture-frame-sequence",
        },
    });
    let mut bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| format!("semantic manifest encode failed: {error}"))?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// Project one exact color into manifest channel order.
fn rgba(color: Rgba8) -> [u8; 4] {
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
