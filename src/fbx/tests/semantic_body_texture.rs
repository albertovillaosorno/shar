// File:
//   - semantic_body_texture.rs
// Path:
//   - src/fbx/tests/semantic_body_texture.rs
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
//   - Behavioral regression coverage for deterministic body-atlas planning,
//   - chart packing, rasterization, and UV-only character mutation.
// - Must-Not:
//   - Read extracted assets, invoke external authoring applications, or assert
//   - private implementation functions.
// - Allows:
//   - Synthetic public-domain aggregate construction and public API assertions.
// - Split-When:
//   - Atlas planning and UV invariance need independent test contracts.
// - Merge-When:
//   - Another integration test owns the same body-planning behavior.
// - Summary:
//   - Semantic body texture behavioral regression.
// - Description:
//   - Proves five-region output without topology or deformation-data changes.
// - Usage:
//   - Runs through the standard fbx integration test suite.
// - Defaults:
//   - Every assertion uses synthetic redistributable evidence.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Behavioral regression for semantic body texture planning.
#[path = "common/semantic_body.rs"]
mod semantic_body;

use std::collections::BTreeSet;

use fbx::domain::character::CharacterAsset;
use fbx::domain::texture::semantic::{
    BodyRegion, BodyTexturePlan, BoneFamily, Rgba8, RgbaImage, RgbaImageError,
    TextureAddressMode, plan_body_texture,
};
use semantic_body::{BODY_COLORS, body_fixture};

#[test]
fn plans_five_regions_deterministically_and_changes_only_uvs()
-> Result<(), String> {
    let (character, source, recipe) = body_fixture()?;
    let first = plan_body_texture(
        &character, &source, &recipe,
    )
    .map_err(|error| format!("first plan failed: {error:?}"))?;
    let second = plan_body_texture(
        &character, &source, &recipe,
    )
    .map_err(|error| format!("second plan failed: {error:?}"))?;
    if first != second {
        return Err("equivalent planning was not deterministic".to_owned());
    }
    validate_plan_shape(&first)?;
    validate_atlas_colors(&first)?;
    validate_uv_only_change(
        &character, first,
    )
}

/// Validate source counts, semantic lanes, and chart containment.
fn validate_plan_shape(plan: &BodyTexturePlan) -> Result<(), String> {
    if plan.source_vertex_count != 15 || plan.source_triangle_count != 5 {
        return Err(
            format!(
                "unexpected source counts: vertices={}, triangles={}",
                plan.source_vertex_count, plan.source_triangle_count,
            ),
        );
    }
    let regions = plan
        .color_assignments
        .iter()
        .map(|assignment| assignment.region)
        .collect::<BTreeSet<_>>();
    if regions
        != BodyRegion::ALL
            .into_iter()
            .collect()
    {
        return Err(format!("unexpected semantic regions: {regions:?}"));
    }
    if plan
        .charts
        .len()
        != 5
    {
        return Err(
            format!(
                "expected five charts, got {}",
                plan.charts
                    .len()
            ),
        );
    }
    for (index, first) in plan
        .charts
        .iter()
        .enumerate()
    {
        for second in plan
            .charts
            .iter()
            .skip(index + 1)
        {
            if overlaps(
                first.cell,
                second.cell,
            ) {
                return Err(
                    format!(
                        "chart cells overlap: {} and {}",
                        first.id, second.id,
                    ),
                );
            }
        }
        let escaped = first
            .content
            .x
            < first
                .cell
                .x
            || first
                .content
                .y
                < first
                    .cell
                    .y
            || first
                .content
                .right()
                > first
                    .cell
                    .right()
            || first
                .content
                .bottom()
                > first
                    .cell
                    .bottom();
        if escaped {
            return Err(
                format!(
                    "chart content escaped its cell: {}",
                    first.id
                ),
            );
        }
    }
    Ok(())
}

/// Require every synthetic source color to survive atlas rasterization.
fn validate_atlas_colors(plan: &BodyTexturePlan) -> Result<(), String> {
    let colors = plan
        .atlas
        .pixels()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    for color in BODY_COLORS {
        if !colors.contains(&color) {
            return Err(format!("atlas omitted source color {color:?}"));
        }
    }
    Ok(())
}

/// Prove semantic preparation changed only selected UV coordinates.
fn validate_uv_only_change(
    character: &CharacterAsset,
    plan: BodyTexturePlan,
) -> Result<(), String> {
    let original_uvs = character.parts[0]
        .mesh
        .groups[0]
        .uvs
        .clone();
    let remapped_uvs = &plan
        .remapped_character
        .parts[0]
        .mesh
        .groups[0]
        .uvs;
    if original_uvs == *remapped_uvs {
        return Err("semantic planning did not remap UVs".to_owned());
    }
    if remapped_uvs
        .iter()
        .any(
            |uv| !(0.0..=1.0).contains(&uv[0]) || !(0.0..=1.0).contains(&uv[1]),
        )
    {
        return Err("remapped UV escaped the atlas".to_owned());
    }
    let mut normalized = plan.remapped_character;
    normalized.parts[0]
        .mesh
        .groups[0]
        .uvs = original_uvs;
    if normalized != *character {
        return Err("semantic planning changed data outside UVs".to_owned());
    }
    Ok(())
}

#[test]
fn tiled_sampling_wraps_before_v_up_texel_selection() -> Result<(), String> {
    let blue = Rgba8::new(
        0, 0, 255, 255,
    );
    let image = RgbaImage::new(
        2,
        2,
        vec![
            Rgba8::new(
                255, 0, 0, 255,
            ),
            Rgba8::new(
                0, 255, 0, 255,
            ),
            blue,
            Rgba8::new(
                255, 255, 0, 255,
            ),
        ],
    )
    .map_err(|error| format!("sampling fixture failed: {error:?}"))?;
    let base = image
        .sample_uv_v_up_with_address_mode(
            [
                0.25, 0.25,
            ],
            TextureAddressMode::Tile,
        )
        .map_err(|error| format!("base sample failed: {error:?}"))?;
    let wrapped = image
        .sample_uv_v_up_with_address_mode(
            [
                0.25, 1.25,
            ],
            TextureAddressMode::Tile,
        )
        .map_err(|error| format!("wrapped sample failed: {error:?}"))?;
    if base != blue || wrapped != base {
        return Err(
            format!(
                "tiled sampling did not preserve repeated texel: {wrapped:?}"
            ),
        );
    }
    if image.sample_uv_v_up_with_address_mode(
        [
            0.25, 1.25,
        ],
        TextureAddressMode::Clamp,
    ) != Err(RgbaImageError::InvalidUv)
    {
        return Err("clamp mode accepted an out-of-range UV".to_owned());
    }
    Ok(())
}

#[test]
fn classifies_legacy_pelvis_support_joint_as_lower_body() {
    assert_eq!(
        BoneFamily::from_bone_id("Ass_Joint"),
        BoneFamily::LowerBody,
    );
}

#[test]
fn semantic_atlas_allows_an_empty_hair_lane() -> Result<(), String> {
    let (character, source, mut recipe) = body_fixture()?;
    recipe
        .color_overrides
        .insert(
            BODY_COLORS[1],
            BodyRegion::Skin,
        );
    let planned = plan_body_texture(
        &character, &source, &recipe,
    )
    .map_err(|error| format!("optional lane planning failed: {error:?}"))?;
    if planned
        .charts
        .iter()
        .any(|chart| chart.region == BodyRegion::Hair)
    {
        return Err("hair lane was populated without hair evidence".to_owned());
    }
    Ok(())
}

/// Return whether two nonempty integer rectangles overlap.
fn overlaps(
    first: fbx::domain::texture::semantic::PixelRect,
    second: fbx::domain::texture::semantic::PixelRect,
) -> bool {
    let Some(first_right) = first.right() else {
        return true;
    };
    let Some(first_bottom) = first.bottom() else {
        return true;
    };
    let Some(second_right) = second.right() else {
        return true;
    };
    let Some(second_bottom) = second.bottom() else {
        return true;
    };
    first.x <= second_right
        && second.x <= first_right
        && first.y <= second_bottom
        && second.y <= first_bottom
}
