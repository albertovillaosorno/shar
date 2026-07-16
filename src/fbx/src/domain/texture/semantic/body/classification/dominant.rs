// File:
//   - dominant.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/classification/dominant.rs
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
//   - Deterministic reduction of explicit skin influences to one dominant bone
//   - per vertex.
// - Must-Not:
//   - Sample textures, classify colors, or resolve equal strongest weights by
//   - arbitrary ordering.
// - Allows:
//   - Exact weight accumulation and stable tie detection.
// - Split-When:
//   - Multi-bone semantic evidence becomes a supported classification input.
// - Merge-When:
//   - Skin influence validation owns the same dominant-bone contract.
// - Summary:
//   - Dominant skin-bone evidence reduction.
// - Description:
//   - Converts validated explicit influences into conservative semantic
//   - evidence.
// - Usage:
//   - Called by semantic body classification.
// - Defaults:
//   - Missing and tied strongest influences fail closed.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Dominant skin-bone evidence reduction.
use std::cmp::Ordering;
use std::collections::BTreeMap;

use super::super::super::color::Rgba8;
use super::super::super::region::{BodyRegion, BoneFamily};
use super::super::error::SemanticTextureError;
use super::super::recipe::GroupAddress;
use crate::domain::skin::SkinInfluence;

/// Dominant semantic evidence for one source vertex.
#[derive(Clone, Debug)]
pub(super) struct DominantEvidence {
    /// Semantic family of the strongest influence set, or no vote for a tie.
    pub(super) family: Option<BoneFamily>,
    /// Representative bone or tie marker for diagnostics.
    pub(super) bone_id: String,
}

/// Reduce explicit weights to one unambiguous dominant family per vertex.
pub(super) fn dominant_bones(
    address: GroupAddress,
    vertex_count: usize,
    influences: &[SkinInfluence],
    colors: &[Rgba8],
    overridden_colors: &BTreeMap<Rgba8, BodyRegion>,
) -> Result<Vec<DominantEvidence>, SemanticTextureError> {
    let mut weights = vec![BTreeMap::<String, f32>::new(); vertex_count];
    for influence in influences {
        let vertex = usize::try_from(influence.vertex_index)
            .map_err(|_error| SemanticTextureError::NumericOverflow)?;
        let Some(vertex_weights) = weights.get_mut(vertex) else {
            return Err(
                SemanticTextureError::MissingDominantInfluence {
                    group: address,
                    vertex,
                },
            );
        };
        let entry = vertex_weights
            .entry(
                influence
                    .bone_id
                    .clone(),
            )
            .or_default();
        *entry += influence.weight;
    }
    weights
        .into_iter()
        .enumerate()
        .map(
            |(vertex, candidates)| {
                let color = colors
                    .get(vertex)
                    .copied()
                    .ok_or(SemanticTextureError::NumericOverflow)?;
                dominant_bone(
                    address,
                    vertex,
                    color,
                    candidates,
                    overridden_colors,
                )
            },
        )
        .collect()
}

/// Select one strongest semantic family and reject unreviewed cross-family
/// ties.
fn dominant_bone(
    address: GroupAddress,
    vertex: usize,
    color: Rgba8,
    candidates: BTreeMap<String, f32>,
    overridden_colors: &BTreeMap<Rgba8, BodyRegion>,
) -> Result<DominantEvidence, SemanticTextureError> {
    let mut ordered = candidates
        .into_iter()
        .collect::<Vec<_>>();
    ordered.sort_by(
        |left, right| {
            right
                .1
                .total_cmp(&left.1)
                .then_with(
                    || {
                        left.0
                            .cmp(&right.0)
                    },
                )
        },
    );
    let Some((bone_id, weight)) = ordered.first() else {
        return Err(
            SemanticTextureError::MissingDominantInfluence {
                group: address,
                vertex,
            },
        );
    };
    let strongest_family = BoneFamily::from_bone_id(bone_id);
    let cross_family_tie = ordered
        .iter()
        .skip(1)
        .take_while(
            |candidate| {
                candidate
                    .1
                    .total_cmp(weight)
                    == Ordering::Equal
            },
        )
        .any(
            |candidate| {
                BoneFamily::from_bone_id(&candidate.0) != strongest_family
            },
        );
    if cross_family_tie {
        let marker = if overridden_colors.contains_key(&color) {
            "reviewed-color-override"
        } else {
            "cross-family-tie"
        };
        return Ok(
            DominantEvidence {
                family: None,
                bone_id: marker.to_owned(),
            },
        );
    }
    Ok(
        DominantEvidence {
            family: Some(strongest_family),
            bone_id: bone_id.clone(),
        },
    )
}
