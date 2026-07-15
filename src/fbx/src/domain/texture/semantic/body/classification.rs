// File:
//   - classification.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/classification.rs
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
//   - The ordered transaction that samples selected body groups and assembles
//   - deterministic semantic classification evidence.
// - Must-Not:
//   - Pack charts, rasterize atlases, mutate UVs, or guess ambiguous evidence.
// - Allows:
//   - Focused dominant-influence, color-voting, and triangle-check modules.
// - Split-When:
//   - Sampling and evidence assembly no longer share one failure boundary.
// - Merge-When:
//   - Another body module owns the same classification transaction.
// - Summary:
//   - Flat-color semantic body classification transaction.
// - Description:
//   - Correlates source texels with dominant skin bones before UV changes.
// - Usage:
//   - Called only by the body-planning facade.
// - Defaults:
//   - Transparency, unsupported bones, ties, and mixed triangles fail closed.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: selected-group access, source sampling, and result assembly form
//   - one bounded transaction while complex rules live in submodules.
//

//! Ordered flat-color semantic body classification transaction.
use std::collections::BTreeMap;

use super::super::color::Rgba8;
use super::super::image::RgbaImage;
use super::super::region::{BodyRegion, BoneFamily};
use super::error::SemanticTextureError;
use super::recipe::{BodySemanticRecipe, GroupAddress};
use super::types::SourceColorAssignment;
use crate::domain::character::CharacterAsset;
use crate::domain::mesh::PrimitiveGroup;
use crate::domain::skin::SkinInfluence;

#[path = "classification/colors.rs"]
mod colors;
#[path = "classification/dominant.rs"]
mod dominant;
#[path = "classification/triangles.rs"]
mod triangles;

/// Per-group immutable classification used by chart generation.
#[derive(Clone, Debug)]
pub(super) struct GroupClassification {
    pub(super) colors: Vec<Rgba8>,
    pub(super) regions: Vec<BodyRegion>,
}

/// Complete deterministic classification output.
#[derive(Clone, Debug)]
pub(super) struct Classification {
    pub(super) groups: BTreeMap<GroupAddress, GroupClassification>,
    pub(super) assignments: Vec<SourceColorAssignment>,
    pub(super) vertex_count: usize,
    pub(super) triangle_count: usize,
}

/// Classify every selected body vertex and validate flat-color triangles.
pub(super) fn classify(
    character: &CharacterAsset,
    source_texture: &RgbaImage,
    recipe: &BodySemanticRecipe,
) -> Result<Classification, SemanticTextureError> {
    let mut sampled = BTreeMap::new();
    let mut counts: BTreeMap<Rgba8, BTreeMap<BoneFamily, u32>> =
        BTreeMap::new();
    let mut vertex_count = 0_usize;
    let mut triangle_count = 0_usize;
    for address in &recipe.groups {
        let (group, influences) = selected_group(
            character, *address,
        )?;
        if group
            .uvs
            .len()
            != group
                .positions
                .len()
        {
            return Err(SemanticTextureError::MissingGroupUvs(*address));
        }
        let group_colors = sample_group_colors(
            group,
            source_texture,
        )?;
        let dominant_evidence = dominant::dominant_bones(
            *address,
            group
                .positions
                .len(),
            influences,
            &group_colors,
            &recipe.color_overrides,
        )?;
        record_family_counts(
            &group_colors,
            &dominant_evidence,
            recipe,
            &mut counts,
        )?;
        vertex_count = vertex_count
            .checked_add(
                group
                    .positions
                    .len(),
            )
            .ok_or(SemanticTextureError::NumericOverflow)?;
        triangle_count = triangle_count
            .checked_add(
                group
                    .triangles
                    .len(),
            )
            .ok_or(SemanticTextureError::NumericOverflow)?;
        sampled.insert(
            *address,
            group_colors,
        );
    }
    let assignments = colors::classify_colors(
        &counts, recipe,
    )?;
    let by_color = assignments
        .iter()
        .map(
            |assignment| {
                (
                    assignment.color,
                    assignment.region,
                )
            },
        )
        .collect::<BTreeMap<_, _>>();
    let mut groups = BTreeMap::new();
    for address in &recipe.groups {
        let (group, _influences) = selected_group(
            character, *address,
        )?;
        let group_colors = sampled
            .remove(address)
            .ok_or(SemanticTextureError::MissingGroup(*address))?;
        let regions = group_colors
            .iter()
            .map(
                |color| {
                    by_color
                        .get(color)
                        .copied()
                        .ok_or(
                            SemanticTextureError::AmbiguousColorEvidence(
                                *color,
                            ),
                        )
                },
            )
            .collect::<Result<Vec<_>, _>>()?;
        triangles::validate(
            *address,
            group,
            &group_colors,
            &regions,
        )?;
        groups.insert(
            *address,
            GroupClassification {
                colors: group_colors,
                regions,
            },
        );
    }
    Ok(
        Classification {
            groups,
            assignments,
            vertex_count,
            triangle_count,
        },
    )
}

/// Resolve one selected primitive group and its explicit influences.
pub(super) fn selected_group(
    character: &CharacterAsset,
    address: GroupAddress,
) -> Result<
    (
        &PrimitiveGroup,
        &[SkinInfluence],
    ),
    SemanticTextureError,
> {
    let part = character
        .parts
        .get(address.part_index)
        .ok_or(SemanticTextureError::MissingPart(address))?;
    let group = part
        .mesh
        .groups
        .get(address.group_index)
        .ok_or(SemanticTextureError::MissingGroup(address))?;
    let influences = part
        .group_influences
        .get(address.group_index)
        .ok_or(SemanticTextureError::MissingGroup(address))?;
    Ok(
        (
            group, influences,
        ),
    )
}

/// Sample exact source colors before reducing skin-bone evidence.
fn sample_group_colors(
    group: &PrimitiveGroup,
    source_texture: &RgbaImage,
) -> Result<Vec<Rgba8>, SemanticTextureError> {
    let mut group_colors = Vec::with_capacity(
        group
            .uvs
            .len(),
    );
    for uv in &group.uvs {
        let color = source_texture.sample_uv_v_up(*uv)?;
        if color.alpha != u8::MAX {
            return Err(
                SemanticTextureError::TransparentSourceBodyColor(color),
            );
        }
        group_colors.push(color);
    }
    Ok(group_colors)
}

/// Record exact color-to-family evidence after reviewed tie handling.
fn record_family_counts(
    colors: &[Rgba8],
    evidence: &[dominant::DominantEvidence],
    recipe: &BodySemanticRecipe,
    counts: &mut BTreeMap<Rgba8, BTreeMap<BoneFamily, u32>>,
) -> Result<(), SemanticTextureError> {
    if colors.len() != evidence.len() {
        return Err(SemanticTextureError::NumericOverflow);
    }
    for (color, dominant) in colors
        .iter()
        .copied()
        .zip(evidence)
    {
        if dominant.family == BoneFamily::Unsupported
            && !recipe
                .color_overrides
                .contains_key(&color)
        {
            return Err(
                SemanticTextureError::UnsupportedBoneEvidence {
                    color,
                    bone_id: dominant
                        .bone_id
                        .clone(),
                },
            );
        }
        let entry = counts
            .entry(color)
            .or_default()
            .entry(dominant.family)
            .or_default();
        *entry = entry
            .checked_add(1)
            .ok_or(SemanticTextureError::NumericOverflow)?;
    }
    Ok(())
}
