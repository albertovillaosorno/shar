// File:
//   - semantic_body_texture_rejections.rs
// Path:
//   - src/fbx/tests/semantic_body_texture_rejections.rs
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
//   - Behavioral regression coverage for mixed-color triangle rejection and
//   - reviewed exact-color resolution of cross-family influence ties.
// - Must-Not:
//   - Read extracted assets, invoke external authoring applications, or assert
//   - private implementation functions.
// - Allows:
//   - Synthetic public-domain evidence mutation through public domain APIs.
// - Split-When:
//   - Color uniformity and influence-tie rejection need independent suites.
// - Merge-When:
//   - The success-path body suite can own these failures below the file limit.
// - Summary:
//   - Semantic body texture rejection regression.
// - Description:
//   - Proves ambiguous or mixed evidence fails closed unless explicitly
//   - reviewed.
// - Usage:
//   - Runs through the standard fbx integration test suite.
// - Defaults:
//   - Every fixture is synthetic and redistributable.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Behavioral rejection regression for semantic body texture planning.
#[path = "common/semantic_body.rs"]
mod semantic_body;

use fbx::domain::skin::SkinInfluence;
use fbx::domain::texture::semantic::{
    BodyRegion, BoneFamily, SemanticTextureError, plan_body_texture,
};
use semantic_body::{BODY_COLORS, body_fixture};

#[test]
fn rejects_a_triangle_that_samples_more_than_one_source_color()
-> Result<(), String> {
    let (mut character, source, recipe) = body_fixture()?;
    character.parts[0]
        .mesh
        .groups[0]
        .uvs[1] = [
        0.3, 0.5,
    ];
    let result = plan_body_texture(
        &character, &source, &recipe,
    );
    match result {
        Err(SemanticTextureError::MixedSourceColorTriangle {
            group,
            triangle,
        }) if group.part_index == 0
            && group.group_index == 0
            && triangle == 0 =>
        {
            Ok(())
        }
        other => Err(
            format!("expected mixed source-color rejection, got {other:?}",),
        ),
    }
}

#[test]
fn requires_reviewed_color_for_cross_family_ties() -> Result<(), String> {
    let (mut character, source, mut recipe) = body_fixture()?;
    let influences = &mut character.parts[0].group_influences[0];
    let head = influences
        .iter_mut()
        .find(|influence| influence.vertex_index == 0)
        .ok_or_else(|| "missing synthetic head influence".to_owned())?;
    head.weight = 0.5;
    influences.push(
        SkinInfluence {
            vertex_index: 0,
            bone_id: "spine".to_owned(),
            weight: 0.5,
        },
    );
    match plan_body_texture(
        &character, &source, &recipe,
    ) {
        Err(SemanticTextureError::AmbiguousDominantInfluence {
            group,
            vertex,
        }) if group.part_index == 0
            && group.group_index == 0
            && vertex == 0 => {}
        other => {
            return Err(
                format!("expected cross-family tie rejection, got {other:?}",),
            );
        }
    }
    recipe
        .color_overrides
        .insert(
            BODY_COLORS[0],
            BodyRegion::Skin,
        );
    let planned = plan_body_texture(
        &character, &source, &recipe,
    )
    .map_err(|error| format!("reviewed override failed: {error:?}"))?;
    let assignment = planned
        .color_assignments
        .iter()
        .find(|assignment| assignment.color == BODY_COLORS[0])
        .ok_or_else(|| "missing reviewed skin assignment".to_owned())?;
    if !assignment.overridden
        || assignment
            .family_counts
            .get(&BoneFamily::Unsupported)
            != Some(&1)
    {
        return Err(
            format!("reviewed tie evidence was not preserved: {assignment:?}",),
        );
    }
    Ok(())
}
