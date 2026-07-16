// File:
//   - semantic_patterned_body.rs
// Path:
//   - src/fbx/tests/common/semantic_patterned_body.rs
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
//   - Redistributable anchored patterned-body fixture construction.
// - Must-Not:
//   - Depend on extracted game data or duplicate production algorithms.
// - Allows:
//   - One uniform triangle and one isolated multicolor anchored triangle.
// - Split-When:
//   - More patterned topology families require independent fixtures.
// - Merge-When:
//   - The general body fixture owns patterned evidence directly.
// - Summary:
//   - Synthetic patterned semantic-body fixture.
// - Description:
//   - Reproduces the safe shared-anchor topology used by source-UV charts.
// - Usage:
//   - Consumed by the patterned semantic atlas integration regression.
// - Defaults:
//   - Polygon and vertex counts remain immutable.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - false
//

//! Redistributable anchored patterned-body fixture.
use std::collections::BTreeMap;

use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;
use fbx::domain::texture::semantic::{
    AtlasConfig, BodySemanticRecipe, GroupAddress, Rgba8, RgbaImage,
    TextureAddressMode,
};

/// Build one safe anchored patterned chart fixture.
#[expect(
    clippy::redundant_pub_crate,
    reason = "Parent tests import this fixture; broader visibility is not \
              public API."
)]
pub(super) fn patterned_body_fixture() -> Result<
    (
        CharacterAsset,
        RgbaImage,
        BodySemanticRecipe,
    ),
    String,
> {
    Ok(
        (
            patterned_character()?,
            patterned_source()?,
            patterned_recipe()?,
        ),
    )
}

/// Build the anchored patterned mesh and stable skin evidence.
fn patterned_character() -> Result<CharacterAsset, String> {
    let positions = vec![
        [
            0.0, 0.0, 0.0,
        ],
        [
            1.0, 0.0, 0.0,
        ],
        [
            0.0, 1.0, 0.0,
        ],
        [
            1.0, 1.0, 0.0,
        ],
        [
            0.5, 1.5, 0.0,
        ],
    ];
    let uvs = vec![
        [
            0.125, 0.875,
        ],
        [
            0.125, 0.875,
        ],
        [
            0.125, 0.875,
        ],
        [
            0.375, 0.625,
        ],
        [
            0.625, 0.375,
        ],
    ];
    let group = PrimitiveGroup::new(
        0,
        "patterned-body",
        positions,
        uvs,
        &[
            0, 1, 2, 2, 3, 4,
        ],
    )
    .map_err(|error| format!("patterned group failed: {error:?}"))?;
    let mesh = MeshAsset::new(
        "patterned-body",
        vec![group],
    )
    .map_err(|error| format!("patterned mesh failed: {error:?}"))?;
    let influences = (0_u32..5)
        .map(
            |vertex_index| SkinInfluence {
                vertex_index,
                bone_id: "head".to_owned(),
                weight: 1.0,
            },
        )
        .collect();
    CharacterAsset::new(
        "patterned-character",
        patterned_skeleton(),
        vec![
            SkinnedPart {
                mesh,
                group_influences: vec![influences],
            },
        ],
    )
    .map_err(|error| format!("patterned character failed: {error:?}"))
}

/// Build the two-bone skeleton used by the patterned fixture.
fn patterned_skeleton() -> Vec<Bone> {
    let identity = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 1.0,
    ];
    vec![
        Bone {
            id: "root".to_owned(),
            parent_id: None,
            rest_matrix: identity,
        },
        Bone {
            id: "head".to_owned(),
            parent_id: Some("root".to_owned()),
            rest_matrix: identity,
        },
    ]
}

/// Build the exact 4x4 source pattern used by the fixture.
#[expect(
    clippy::indexing_slicing,
    reason = "Fixed fixture constructors validate literals before indexing."
)]
fn patterned_source() -> Result<RgbaImage, String> {
    let yellow = Rgba8::new(
        255, 210, 0, 255,
    );
    let purple = Rgba8::new(
        115, 16, 123, 255,
    );
    let blue = Rgba8::new(
        0, 33, 123, 255,
    );
    let mut pixels = vec![yellow; 16];
    pixels[5] = purple;
    pixels[10] = blue;
    RgbaImage::new(
        4, 4, pixels,
    )
    .map_err(|error| format!("patterned image failed: {error:?}"))
}

/// Build the deterministic semantic atlas recipe for the fixture.
fn patterned_recipe() -> Result<BodySemanticRecipe, String> {
    let atlas = AtlasConfig::new(
        320,
        160,
        4,
        Rgba8::new(
            128, 128, 128, 255,
        ),
    )
    .map_err(|error| format!("patterned atlas failed: {error:?}"))?;
    BodySemanticRecipe::new(
        vec![
            GroupAddress {
                part_index: 0,
                group_index: 0,
            },
        ],
        BTreeMap::new(),
        TextureAddressMode::Clamp,
        0.20,
        atlas,
    )
    .map_err(|error| format!("patterned recipe failed: {error:?}"))
}
