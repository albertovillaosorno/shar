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

use fbx::domain::texture::semantic::{BodyRegion, plan_body_texture};
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
    if first.source_vertex_count != 15 || first.source_triangle_count != 5 {
        return Err(
            format!(
                "unexpected source counts: vertices={}, triangles={}",
                first.source_vertex_count, first.source_triangle_count,
            ),
        );
    }
    let regions = first
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
    if first
        .charts
        .len()
        != 5
    {
        return Err(
            format!(
                "expected five charts, got {}",
                first
                    .charts
                    .len()
            ),
        );
    }
    for (index, first_chart) in first
        .charts
        .iter()
        .enumerate()
    {
        for second_chart in first
            .charts
            .iter()
            .skip(index + 1)
        {
            if overlaps(
                first_chart.cell,
                second_chart.cell,
            ) {
                return Err(
                    format!(
                        "chart cells overlap: {} and {}",
                        first_chart.id, second_chart.id,
                    ),
                );
            }
        }
        if first_chart
            .content
            .x
            < first_chart
                .cell
                .x
            || first_chart
                .content
                .y
                < first_chart
                    .cell
                    .y
            || first_chart
                .content
                .right()
                > first_chart
                    .cell
                    .right()
            || first_chart
                .content
                .bottom()
                > first_chart
                    .cell
                    .bottom()
        {
            return Err(
                format!(
                    "chart content escaped its cell: {}",
                    first_chart.id,
                ),
            );
        }
    }
    let atlas_colors = first
        .atlas
        .pixels()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    for color in BODY_COLORS {
        if !atlas_colors.contains(&color) {
            return Err(format!("atlas omitted source color {color:?}"));
        }
    }
    let original_uvs = character.parts[0]
        .mesh
        .groups[0]
        .uvs
        .clone();
    let remapped_uvs = first
        .remapped_character
        .parts[0]
        .mesh
        .groups[0]
        .uvs
        .clone();
    if original_uvs == remapped_uvs {
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
    let mut normalized = first
        .remapped_character
        .clone();
    normalized.parts[0]
        .mesh
        .groups[0]
        .uvs = original_uvs;
    if normalized != character {
        return Err("semantic planning changed data outside UVs".to_owned());
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
