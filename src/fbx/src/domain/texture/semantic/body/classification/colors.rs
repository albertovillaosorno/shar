// File:
//   - colors.rs
// Path:
//   - src/fbx/src/domain/texture/semantic/body/classification/colors.rs
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
//   - Exact source-color classification through unique bone-family votes,
//   - reviewed overrides, and a bounded exposed-color hair threshold.
// - Must-Not:
//   - Sample images, inspect triangles, or infer an answer from tied evidence.
// - Allows:
//   - Deterministic minimum-region coverage checks.
// - Split-When:
//   - Pattern, alpha, or accessory classifications require new evidence types.
// - Merge-When:
//   - The parent classification transaction owns the same color-voting logic.
// - Summary:
//   - Strict source-color semantic voting.
// - Description:
//   - Converts exact flat-color evidence into stable parent body regions.
// - Usage:
//   - Called after all selected body vertices have been sampled.
// - Defaults:
//   - Reviewed overrides are explicit and all automatic ties fail closed.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Strict source-color semantic voting.
use std::collections::{BTreeMap, BTreeSet};

use super::super::super::color::Rgba8;
use super::super::super::region::{BodyRegion, BoneFamily};
use super::super::error::SemanticTextureError;
use super::super::recipe::BodySemanticRecipe;
use super::super::types::SourceColorAssignment;

/// Classify exact colors and prove every minimum semantic region is present.
pub(super) fn classify_colors(
    counts: &BTreeMap<Rgba8, BTreeMap<BoneFamily, u32>>,
    recipe: &BodySemanticRecipe,
) -> Result<Vec<SourceColorAssignment>, SemanticTextureError> {
    let brightest_exposed = counts
        .iter()
        .filter(
            |(_color, families)| {
                unique_winner(families) == Some(BoneFamily::Exposed)
            },
        )
        .map(|(color, _families)| color.relative_luminance())
        .max_by(f32::total_cmp)
        .unwrap_or(0.0);
    let mut assignments = Vec::with_capacity(counts.len());
    let mut present = BTreeSet::new();
    for (color, family_counts) in counts {
        let (region, overridden) = match recipe
            .color_overrides
            .get(color)
        {
            Some(region) => (
                *region, true,
            ),
            None => (
                automatic_region(
                    *color,
                    family_counts,
                    brightest_exposed,
                    recipe.hair_luminance_ratio,
                )?,
                false,
            ),
        };
        present.insert(region);
        assignments.push(
            SourceColorAssignment {
                color: *color,
                region,
                family_counts: family_counts.clone(),
                overridden,
            },
        );
    }
    for region in BodyRegion::ALL {
        if !present.contains(&region) {
            return Err(SemanticTextureError::MissingRequiredRegion(region));
        }
    }
    Ok(assignments)
}

/// Convert one uniquely winning bone family into a semantic region.
fn automatic_region(
    color: Rgba8,
    family_counts: &BTreeMap<BoneFamily, u32>,
    brightest_exposed: f32,
    hair_luminance_ratio: f32,
) -> Result<BodyRegion, SemanticTextureError> {
    let family = unique_winner(family_counts)
        .ok_or(SemanticTextureError::AmbiguousColorEvidence(color))?;
    match family {
        BoneFamily::Exposed => {
            if color.relative_luminance()
                <= brightest_exposed * hair_luminance_ratio
            {
                Ok(BodyRegion::Hair)
            } else {
                Ok(BodyRegion::Skin)
            }
        }
        BoneFamily::Torso => Ok(BodyRegion::Torso),
        BoneFamily::LowerBody => Ok(BodyRegion::Legs),
        BoneFamily::Foot => Ok(BodyRegion::Shoes),
        BoneFamily::Unsupported => {
            Err(SemanticTextureError::AmbiguousColorEvidence(color))
        }
    }
}

/// Return one family only when it has a unique highest count.
fn unique_winner(counts: &BTreeMap<BoneFamily, u32>) -> Option<BoneFamily> {
    let mut ordered = counts
        .iter()
        .map(
            |(family, count)| {
                (
                    *family, *count,
                )
            },
        )
        .collect::<Vec<_>>();
    ordered.sort_by(
        |left, right| {
            right
                .1
                .cmp(&left.1)
                .then_with(
                    || {
                        left.0
                            .cmp(&right.0)
                    },
                )
        },
    );
    let first = ordered.first()?;
    if ordered
        .get(1)
        .is_some_and(|second| second.1 == first.1)
    {
        return None;
    }
    Some(first.0)
}
