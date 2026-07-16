// File:
//   - semantic_body.rs
// Path:
//   - src/fbx/tests/common/semantic_body.rs
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
//   - Redistributable synthetic five-region character, palette, and recipe
//   - fixtures for semantic body tests.
// - Must-Not:
//   - Read extracted assets, contain proprietary pixels, or duplicate planner
//   - implementation logic.
// - Allows:
//   - Explicit public-domain domain construction.
// - Split-When:
//   - Character aggregate and palette recipes require independent fixtures.
// - Merge-When:
//   - Another test-support module owns the same body fixtures.
// - Summary:
//   - Synthetic semantic body fixtures.
// - Description:
//   - Builds one real five-region skinned aggregate through public
//   - constructors.
// - Usage:
//   - Imported by semantic body integration tests.
// - Defaults:
//   - Skin, hair, torso, legs, and shoes are always present.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
//
// Large file:
//   - true
//   - Reason: body mesh, skin, skeleton, palette, and recipe form one complete
//   - redistributable fixture.
//

//! Redistributable synthetic semantic body fixtures.
use std::collections::BTreeMap;

use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;
use fbx::domain::texture::semantic::{
    AtlasConfig, BodySemanticRecipe, GroupAddress, Rgba8, RgbaImage,
    TextureAddressMode,
};

/// Stable synthetic colors in body-region order.
#[expect(
    clippy::redundant_pub_crate,
    reason = "Parent tests import this fixture; broader visibility is not \
              public API."
)]
pub(super) const BODY_COLORS: [Rgba8; 5] = [
    Rgba8::new(
        255, 210, 0, 255,
    ),
    Rgba8::new(
        8, 8, 8, 255,
    ),
    Rgba8::new(
        220, 40, 30, 255,
    ),
    Rgba8::new(
        40, 90, 220, 255,
    ),
    Rgba8::new(
        90, 45, 15, 255,
    ),
];

/// Build one five-region synthetic character, source image, and recipe.
#[expect(
    clippy::redundant_pub_crate,
    reason = "Parent tests import this fixture; broader visibility is not \
              public API."
)]
pub(super) fn body_fixture() -> Result<
    (
        CharacterAsset,
        RgbaImage,
        BodySemanticRecipe,
    ),
    String,
> {
    let group = body_group()?;
    let character = body_character(group)?;
    let image = RgbaImage::new(
        5,
        1,
        BODY_COLORS.to_vec(),
    )
    .map_err(|error| format!("palette failed: {error:?}"))?;
    Ok(
        (
            character,
            image,
            body_recipe()?,
        ),
    )
}

/// Build the five disconnected flat-color primitive groups as one group.
#[expect(
    clippy::arithmetic_side_effects,
    reason = "Fixture literals are constructor-validated."
)]
fn body_group() -> Result<PrimitiveGroup, String> {
    let mut positions = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();
    for region in 0_u16..5 {
        let x = f32::from(region) * 2.0;
        positions.extend(
            [
                [
                    x, 0.0, 0.0,
                ],
                [
                    x + 1.0,
                    0.0,
                    0.0,
                ],
                [
                    x, 1.0, 0.0,
                ],
            ],
        );
        let u = (f32::from(region) + 0.5) / 5.0;
        uvs.extend(
            [[
                u, 0.5,
            ]; 3],
        );
        let base = u32::from(region) * 3;
        indices.extend(
            [
                base,
                base + 1,
                base + 2,
            ],
        );
    }
    PrimitiveGroup::new(
        0,
        "synthetic-body",
        positions,
        uvs,
        &indices,
    )
    .map_err(|error| format!("body group failed: {error:?}"))
}

/// Build one skinned synthetic character around the body group.
#[expect(
    clippy::arithmetic_side_effects,
    reason = "Fixture literals are constructor-validated."
)]
fn body_character(group: PrimitiveGroup) -> Result<CharacterAsset, String> {
    let mesh = MeshAsset::new(
        "synthetic-body",
        vec![group],
    )
    .map_err(|error| format!("body mesh failed: {error:?}"))?;
    let bones = [
        "head", "head", "spine", "knee", "foot",
    ];
    let mut influences = Vec::new();
    for (region, bone_id) in bones
        .iter()
        .enumerate()
    {
        for offset in 0_usize..3 {
            let vertex_index = u32::try_from(region * 3 + offset).map_err(
                |_error| "synthetic vertex index overflow".to_owned(),
            )?;
            influences.push(
                SkinInfluence {
                    vertex_index,
                    bone_id: (*bone_id).to_owned(),
                    weight: 1.0,
                },
            );
        }
    }
    CharacterAsset::new(
        "synthetic-character",
        body_skeleton(),
        vec![
            SkinnedPart {
                mesh,
                group_influences: vec![influences],
            },
        ],
    )
    .map_err(|error| format!("character failed: {error:?}"))
}

/// Build the shared synthetic body skeleton.
fn body_skeleton() -> Vec<Bone> {
    let identity = [
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0,
        0.0, 1.0,
    ];
    let mut skeleton = vec![
        Bone {
            id: "root".to_owned(),
            parent_id: None,
            rest_matrix: identity,
        },
    ];
    for id in [
        "head", "spine", "knee", "foot",
    ] {
        skeleton.push(
            Bone {
                id: id.to_owned(),
                parent_id: Some("root".to_owned()),
                rest_matrix: identity,
            },
        );
    }
    skeleton
}

/// Build the stable five-column semantic atlas recipe.
fn body_recipe() -> Result<BodySemanticRecipe, String> {
    let atlas = AtlasConfig::new(
        250,
        100,
        2,
        Rgba8::new(
            128, 128, 128, 255,
        ),
    )
    .map_err(|error| format!("atlas failed: {error:?}"))?;
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
    .map_err(|error| format!("recipe failed: {error:?}"))
}
