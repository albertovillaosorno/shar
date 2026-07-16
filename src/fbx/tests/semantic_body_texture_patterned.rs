// File:
//   - semantic_body_texture_patterned.rs
// Path:
//   - src/fbx/tests/semantic_body_texture_patterned.rs
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
//   - Anchored patterned-detail semantic atlas behavior regression.
// - Must-Not:
//   - Read extracted assets or weaken arbitrary mixed-triangle rejection.
// - Allows:
//   - Source-versus-atlas sampling and topology-invariance assertions.
// - Split-When:
//   - More source-UV chart families require independent test transactions.
// - Merge-When:
//   - General semantic body tests own patterned-detail behavior completely.
// - Summary:
//   - Patterned semantic body atlas regression.
// - Description:
//   - Proves one safe anchored pattern survives atlas remapping exactly.
// - Usage:
//   - Runs with the complete FBX package integration test suite.
// - Defaults:
//   - Every interior sample must retain exact source texel ownership.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Patterned semantic body atlas regression.
#[path = "common/semantic_patterned_body.rs"]
mod semantic_patterned_body;

use fbx::domain::texture::semantic::{
    ProjectionAxis, TextureAddressMode, plan_body_texture,
};
use png as _;
use schoenwald_filesystem as _;
use semantic_patterned_body::patterned_body_fixture;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

#[test]
#[expect(
    clippy::indexing_slicing,
    reason = "Fixture literals are constructor-validated."
)]
fn preserves_one_anchored_pattern_without_topology_changes()
-> Result<(), String> {
    let (character, source, recipe) = patterned_body_fixture()?;
    let source_group = &character.parts[0]
        .mesh
        .groups[0];
    let planned = plan_body_texture(
        &character, &source, &recipe,
    )
    .map_err(|error| format!("patterned plan failed: {error:?}"))?;
    let remapped_group = &planned
        .remapped_character
        .parts[0]
        .mesh
        .groups[0];
    if remapped_group.positions != source_group.positions
        || remapped_group.triangles != source_group.triangles
    {
        return Err("patterned planning changed topology".to_owned());
    }
    let sampled = planned
        .charts
        .iter()
        .filter(|chart| chart.sample_source)
        .collect::<Vec<_>>();
    if sampled.len() != 1
        || sampled[0].projection != ProjectionAxis::SourceUv
        || sampled[0].source_sampled_triangles != [1]
    {
        return Err(format!("unexpected patterned charts: {sampled:?}"));
    }
    let mut sample_count = 0_usize;
    for triangle in &source_group.triangles {
        let vertices = [
            usize::try_from(triangle[0]).map_err(|error| error.to_string())?,
            usize::try_from(triangle[1]).map_err(|error| error.to_string())?,
            usize::try_from(triangle[2]).map_err(|error| error.to_string())?,
        ];
        let source_uvs = [
            source_group.uvs[vertices[0]],
            source_group.uvs[vertices[1]],
            source_group.uvs[vertices[2]],
        ];
        let atlas_uvs = [
            remapped_group.uvs[vertices[0]],
            remapped_group.uvs[vertices[1]],
            remapped_group.uvs[vertices[2]],
        ];
        for first in 1_u8..=6 {
            for second in 1_u8..=(7 - first) {
                let third = 8 - first - second;
                if third == 0 {
                    continue;
                }
                let weights = [
                    f32::from(first) / 8.0,
                    f32::from(second) / 8.0,
                    f32::from(third) / 8.0,
                ];
                let expected = source
                    .sample_uv_v_up_with_address_mode(
                        interpolate(
                            source_uvs, weights,
                        ),
                        TextureAddressMode::Clamp,
                    )
                    .map_err(
                        |error| format!("source sample failed: {error:?}"),
                    )?;
                let actual = planned
                    .atlas
                    .sample_uv_v_up_with_address_mode(
                        interpolate(
                            atlas_uvs, weights,
                        ),
                        TextureAddressMode::Clamp,
                    )
                    .map_err(
                        |error| format!("atlas sample failed: {error:?}"),
                    )?;
                if actual != expected {
                    return Err(
                        format!(
                            "patterned sample changed: expected={expected:?}, \
                             actual={actual:?}, weights={weights:?}",
                        ),
                    );
                }
                sample_count = sample_count
                    .checked_add(1)
                    .ok_or_else(|| "sample count overflow".to_owned())?;
            }
        }
    }
    if sample_count != 42 {
        return Err(format!("unexpected sample count: {sample_count}"));
    }
    Ok(())
}

fn interpolate(
    values: [[f32; 2]; 3],
    weights: [f32; 3],
) -> [f32; 2] {
    [
        values[0][0].mul_add(
            weights[0],
            values[1][0].mul_add(
                weights[1],
                values[2][0] * weights[2],
            ),
        ),
        values[0][1].mul_add(
            weights[0],
            values[1][1].mul_add(
                weights[1],
                values[2][1] * weights[2],
            ),
        ),
    ]
}
