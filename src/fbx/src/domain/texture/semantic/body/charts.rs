// File:
//   - charts.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/charts.rs
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
//   - The ordered connected-chart, atlas-placement, raster, and UV-only remap
//   - transaction for one classified character body.
// - Must-Not:
//   - Reclassify source evidence, alter topology, bones, weights, normals, or
//   - invoke filesystem or external authoring applications.
// - Allows:
//   - Focused discovery, projection, packing, and raster modules.
// - Split-When:
//   - Another texture lane cannot reuse the chart transaction.
// - Merge-When:
//   - The body facade can own this transaction without duplicated stages.
// - Summary:
//   - Semantic body chart transaction.
// - Description:
//   - Produces the modern atlas and a character clone with only UV changes.
// - Usage:
//   - Called after strict body classification succeeds.
// - Defaults:
//   - Every chart is rasterized and every selected vertex receives one UV.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: chart-stage ordering, atlas allocation, UV-only mutation, and
//   - final plan assembly form one bounded transaction.
//

//! Semantic body chart, atlas, and UV-remapping transaction.
use super::super::image::RgbaImage;
use super::classification::{Classification, selected_group};
use super::error::SemanticTextureError;
use super::raster;
use super::recipe::{AtlasConfig, BodySemanticRecipe, GroupAddress};
use super::types::BodyTexturePlan;
use crate::domain::character::CharacterAsset;

#[path = "charts/discovery.rs"]
mod discovery;
#[path = "charts/model.rs"]
pub(in crate::domain::texture::semantic::body) mod model;
#[path = "charts/packing.rs"]
mod packing;
#[path = "charts/projection.rs"]
mod projection;

/// Build the complete atlas and UV-remapped character from classification.
pub(super) fn build_plan(
    character: &CharacterAsset,
    recipe: &BodySemanticRecipe,
    classification: Classification,
) -> Result<BodyTexturePlan, SemanticTextureError> {
    let projected = discovery::discover(
        character,
        &classification,
    )?;
    let placed = packing::place(
        &projected,
        &recipe.atlas,
    )?;
    let mut atlas = RgbaImage::filled(
        recipe
            .atlas
            .width,
        recipe
            .atlas
            .height,
        recipe
            .atlas
            .background,
    )?;
    let mut coverage = vec![
        false;
        atlas
            .pixels()
            .len()
    ];
    let mut remapped_character = character.clone();
    for chart in &placed {
        let (source_group, _influences) = selected_group(
            character,
            chart
                .public
                .group,
        )?;
        raster::rasterize(
            &mut atlas,
            &mut coverage,
            source_group,
            chart,
            recipe
                .atlas
                .padding,
        )?;
        apply_uvs(
            &mut remapped_character,
            chart,
            &recipe.atlas,
        )?;
    }
    Ok(
        BodyTexturePlan {
            atlas,
            remapped_character,
            color_assignments: classification.assignments,
            charts: placed
                .into_iter()
                .map(|chart| chart.public)
                .collect(),
            source_vertex_count: classification.vertex_count,
            source_triangle_count: classification.triangle_count,
        },
    )
}

/// Apply one chart's destination UVs to the cloned character only.
fn apply_uvs(
    character: &mut CharacterAsset,
    chart: &model::PlacedChart,
    config: &AtlasConfig,
) -> Result<(), SemanticTextureError> {
    let group = mutable_group(
        character,
        chart
            .public
            .group,
    )?;
    for (vertex, position) in &chart.pixel_positions {
        let uv = group
            .uvs
            .get_mut(*vertex)
            .ok_or(SemanticTextureError::NumericOverflow)?;
        *uv = packing::atlas_uv(
            *position, config,
        );
    }
    Ok(())
}

/// Resolve one mutable selected primitive group in the cloned character.
fn mutable_group(
    character: &mut CharacterAsset,
    address: GroupAddress,
) -> Result<&mut crate::domain::mesh::PrimitiveGroup, SemanticTextureError> {
    character
        .parts
        .get_mut(address.part_index)
        .ok_or(SemanticTextureError::MissingPart(address))?
        .mesh
        .groups
        .get_mut(address.group_index)
        .ok_or(SemanticTextureError::MissingGroup(address))
}
