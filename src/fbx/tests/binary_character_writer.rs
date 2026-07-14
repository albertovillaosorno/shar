// File:
//   - binary_character_writer.rs
// Path:
//   - src/fbx/tests/binary_character_writer.rs
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
//   - Regression coverage for deterministic binary character FBX artifacts.
// - Must-Not:
//   - Read private assets, invoke Blender, or depend on machine-local paths.
// - Allows:
//   - Synthetic character evidence and process-unique temporary output files.
// - Split-When:
//   - Binary container parsing needs an independent conformance test surface.
// - Merge-When:
//   - Character writer formats share one stable conformance fixture.
// - Summary:
//   - Protects binary FBX 7.7 character artifact generation.
// - Description:
//   - Verifies binary identity, footer structure, offsets, and deterministic
//   - bytes from one synthetic skinned character.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - Temporary artifacts are process-unique and removed after each test.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: One ordered binary FBX conformance regression keeps the 7.7
//   - header, 64-bit node records, graph markers, UV bytes, and footer evidence
//   - together so byte offsets remain auditable as one artifact contract.
//

//! Regression coverage for deterministic binary character FBX artifacts.
//!
//! Synthetic geometry and skin data prove the binary container without
//! depending on private extracted assets or an installed DCC application.

/// Shared paired-artifact test helper.
#[path = "common/binary_artifact.rs"]
pub mod binary_artifact;

use std::mem::size_of;
use std::path::PathBuf;

use binary_artifact::read_binary_pair;
use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxSummary, EmbeddedTexture, write_binary_character_fbx,
};
use fbx::domain::character::{CharacterAsset, SkinnedPart};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::skeleton::Bone;
use fbx::domain::skin::SkinInfluence;
use fbx::domain::texture::MaterialBinding;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;

const BINARY_MAGIC: &[u8; 23] = b"Kaydara FBX Binary  \x00\x1a\x00";
const FBX_VERSION: u32 = 7_700;
const NODE_RECORD_WIDTH: usize = 25;
const ROOT_NODE_OFFSET: usize = 27;
const IDENTITY_MATRIX: [f64; 16] = [
    1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64,
    0.0_f64, 0.0_f64, 1.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 0.0_f64, 1.0_f64,
];
const FOOTER_ID: [u8; 16] = [
    0xfa, 0xbc, 0xab, 0x09, 0xd0, 0xc8, 0xd4, 0x66, 0xb1, 0x76, 0xfb, 0x83,
    0x1c, 0xf7, 0x26, 0x7e,
];
const FINAL_MAGIC: [u8; 16] = [
    0xf8, 0x5a, 0x8c, 0x6a, 0xde, 0xf5, 0xd9, 0x7e, 0xec, 0xe9, 0x0c, 0xe3,
    0x75, 0x8f, 0x29, 0x0b,
];

fn synthetic_character() -> Result<CharacterAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "skin",
        vec![
            [
                0.0, 0.0, 0.0,
            ],
            [
                1.0, 0.0, 0.0,
            ],
            [
                0.0, 1.0, 0.0,
            ],
        ],
        vec![
            [
                0.25, 0.75,
            ],
            [
                1.0, 0.0,
            ],
            [
                0.0, 1.0,
            ],
        ],
        &[
            0, 1, 2,
        ],
    )
    .and_then(
        |group| {
            group.with_normals(
                vec![
                    [
                        0.0, 0.0, 1.0,
                    ],
                    [
                        0.0, 0.0, 1.0,
                    ],
                    [
                        0.0, 0.0, 1.0,
                    ],
                ],
            )
        },
    )
    .map_err(|error| format!("synthetic primitive group failed: {error:?}"))?;
    let mesh = MeshAsset::new(
        "body",
        vec![group],
    )
    .map_err(|error| format!("synthetic mesh failed: {error:?}"))?;
    let influences = (0_u32..3)
        .map(
            |vertex_index| SkinInfluence {
                vertex_index,
                bone_id: "root".to_owned(),
                weight: 1.0,
            },
        )
        .collect();
    CharacterAsset::new(
        "synthetic",
        vec![
            Bone {
                id: "root".to_owned(),
                parent_id: None,
                rest_matrix: [
                    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0,
                    0.0, 0.0, 0.0, 1.0,
                ],
            },
        ],
        vec![
            SkinnedPart {
                mesh,
                group_influences: vec![influences],
            },
        ],
    )
    .map_err(|error| format!("synthetic character failed: {error:?}"))
}

fn materials() -> Result<Vec<MaterialBinding>, String> {
    let material = MaterialBinding::new(
        "skin",
        Some("skin.png".to_owned()),
    )
    .map_err(|error| format!("synthetic material failed: {error:?}"))?;
    Ok(vec![material])
}

fn embedded_textures() -> Vec<EmbeddedTexture> {
    vec![
        EmbeddedTexture {
            file_name: "skin.png".to_owned(),
            content: b"\x89PNG\r\n\x1a\nsynthetic-texture".to_vec(),
        },
    ]
}

fn output_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-binary-character-{label}-{}.fbx",
            std::process::id()
        ),
    )
}

/// Encode one exact FBX integer property payload for byte-level assertions.
fn integer_property_token(
    name: &str,
    value: i32,
) -> Option<Vec<u8>> {
    let mut token = Vec::new();
    for field in [
        name, "int", "Integer", "",
    ] {
        let length = u32::try_from(field.len()).ok()?;
        token.push(b'S');
        token.extend_from_slice(&length.to_le_bytes());
        token.extend_from_slice(field.as_bytes());
    }
    token.push(b'I');
    token.extend_from_slice(&value.to_le_bytes());
    Some(token)
}

/// Read one exact signed integer leaf-node value by its unique node name.
fn i32_leaf_value(
    document: &[u8],
    name: &str,
) -> Option<i32> {
    let name_start = document
        .windows(name.len())
        .position(|window| window == name.as_bytes())?;
    let property_start = name_start.checked_add(name.len())?;
    if document.get(property_start) != Some(&b'I') {
        return None;
    }
    let value_start = property_start.checked_add(1)?;
    let value_end = value_start.checked_add(size_of::<i32>())?;
    let bytes = document.get(value_start..value_end)?;
    Some(
        i32::from_le_bytes(
            bytes
                .try_into()
                .ok()?,
        ),
    )
}

/// Encode one exact uncompressed FBX double-array property payload.
fn f64_array_token(values: &[f64]) -> Option<Vec<u8>> {
    let count = u32::try_from(values.len()).ok()?;
    let payload_byte_count = values
        .len()
        .checked_mul(size_of::<f64>())?;
    let encoded_byte_count = u32::try_from(payload_byte_count).ok()?;
    let mut token = Vec::new();
    token.push(b'd');
    token.extend_from_slice(&count.to_le_bytes());
    token.extend_from_slice(&0_u32.to_le_bytes());
    token.extend_from_slice(&encoded_byte_count.to_le_bytes());
    for value in values {
        token.extend_from_slice(&value.to_le_bytes());
    }
    Some(token)
}

/// Count exact byte-window matches.
fn byte_window_count(
    haystack: &[u8],
    needle: &[u8],
) -> usize {
    if needle.is_empty() {
        return 0;
    }
    haystack
        .windows(needle.len())
        .filter(|window| *window == needle)
        .count()
}

// One ordered byte-level regression verifies header, graph tokens, footer, and
// deterministic repeated output without splitting shared artifact evidence.
#[expect(
    clippy::too_many_lines,
    reason = "One binary FBX artifact regression preserves ordered byte \
              evidence."
)]
#[test]
fn writes_deterministic_binary_fbx_7700_with_standard_footer() {
    const FOOTER_VERSION_DISTANCE: usize = 140;

    let first_path = output_path("first");
    let second_path = output_path("second");
    let character_result = synthetic_character();
    assert!(
        character_result.is_ok(),
        "synthetic character should build: {character_result:?}"
    );
    let Some(character) = character_result.ok() else {
        return;
    };
    let materials_result = materials();
    assert!(
        materials_result.is_ok(),
        "synthetic materials should build: {materials_result:?}"
    );
    let Some(materials) = materials_result.ok() else {
        return;
    };

    let textures = embedded_textures();
    let first_summary = write_binary_character_fbx(
        &character,
        &materials,
        &textures,
        &[],
        &first_path,
    );
    let second_summary = write_binary_character_fbx(
        &character,
        &materials,
        &textures,
        &[],
        &second_path,
    );
    let artifacts = read_binary_pair(
        &first_path,
        &second_path,
        "binary FBX",
    );
    let Some((first, second)) = artifacts else {
        return;
    };

    assert_eq!(
        first_summary,
        Ok(
            CharacterBinaryFbxSummary {
                geometries: 1,
                bones: 1,
                clusters: 1,
                materials: 1,
                textures: 1,
                animations: 0,
            }
        )
    );
    assert_eq!(
        second_summary,
        first_summary
    );
    assert_eq!(
        first,
        second
    );
    assert_eq!(
        first.get(..BINARY_MAGIC.len()),
        Some(BINARY_MAGIC.as_slice())
    );
    let version = first
        .get(23..27)
        .and_then(|bytes| <[u8; 4]>::try_from(bytes).ok())
        .map(u32::from_le_bytes);
    assert_eq!(
        version,
        Some(FBX_VERSION)
    );
    let root_header_end_result =
        ROOT_NODE_OFFSET.checked_add(NODE_RECORD_WIDTH);
    assert!(root_header_end_result.is_some());
    let Some(root_header_end) = root_header_end_result else {
        return;
    };
    let root_header_result = first.get(ROOT_NODE_OFFSET..root_header_end);
    assert!(root_header_result.is_some());
    let Some(root_header) = root_header_result else {
        return;
    };
    let root_property_count = root_header
        .get(8..16)
        .and_then(|bytes| <[u8; 8]>::try_from(bytes).ok())
        .map(u64::from_le_bytes);
    assert_eq!(
        root_property_count,
        Some(0)
    );
    let root_property_bytes = root_header
        .get(16..24)
        .and_then(|bytes| <[u8; 8]>::try_from(bytes).ok())
        .map(u64::from_le_bytes);
    assert_eq!(
        root_property_bytes,
        Some(0)
    );
    assert_eq!(
        root_header.get(24),
        Some(&18)
    );
    assert_eq!(
        first.get(root_header_end..root_header_end + 18),
        Some(b"FBXHeaderExtension".as_slice())
    );
    let final_magic_start_result = first
        .len()
        .checked_sub(FINAL_MAGIC.len());
    assert!(final_magic_start_result.is_some());
    let Some(final_magic_start) = final_magic_start_result else {
        return;
    };
    assert_eq!(
        first.get(final_magic_start..),
        Some(FINAL_MAGIC.as_slice())
    );
    let footer_id_start_result = first
        .windows(FOOTER_ID.len())
        .rposition(|window| window == FOOTER_ID);
    assert!(footer_id_start_result.is_some());
    let Some(footer_id_start) = footer_id_start_result else {
        return;
    };
    let null_record_start_result =
        footer_id_start.checked_sub(NODE_RECORD_WIDTH);
    assert!(null_record_start_result.is_some());
    let Some(null_record_start) = null_record_start_result else {
        return;
    };
    assert_eq!(
        first.get(null_record_start..footer_id_start),
        Some([0_u8; NODE_RECORD_WIDTH].as_slice())
    );
    assert!(
        first
            .windows(b"body_0\0\x01Geometry".len())
            .any(|window| window == b"body_0\0\x01Geometry")
    );
    assert!(
        first
            .windows(b"root\0\x01Model".len())
            .any(|window| window == b"root\0\x01Model")
    );
    assert!(
        first
            .windows(b"TransformAssociateModel".len())
            .any(|window| window == b"TransformAssociateModel")
    );
    assert!(
        first
            .windows(b"DefaultAttributeIndex".len())
            .any(|window| window == b"DefaultAttributeIndex")
    );
    assert!(
        first
            .windows(b"ShadingCT".len())
            .any(|window| window == b"ShadingCT"),
        "FBX booleans must use Maya-compatible C plus T/F encoding"
    );
    assert!(
        !first
            .windows(b"ShadingB".len())
            .any(|window| window == b"ShadingB"),
        "retired nonstandard B boolean tags must not return"
    );
    for property_name in [
        "SpecularColor",
        "SpecularFactor",
        "Shininess",
        "ReflectionColor",
        "ReflectionFactor",
    ] {
        assert!(
            first
                .windows(property_name.len())
                .any(|window| window == property_name.as_bytes()),
            "missing zero-highlight property {property_name}"
        );
    }
    assert!(
        first
            .windows(b"SHAR_Export_Root\0\x01Model".len())
            .any(|window| window == b"SHAR_Export_Root\0\x01Model"),
        concat!(
            "binary FBX must rotate the completed character through one ",
            "export root"
        )
    );
    assert_eq!(
        i32_leaf_value(
            &first,
            "NbPoseNodes",
        ),
        Some(2_i32),
        concat!(
            "bind pose must remain in source bind space and exclude the ",
            "export-only parent"
        )
    );
    let identity_token_result = f64_array_token(&IDENTITY_MATRIX);
    assert!(
        identity_token_result.is_some(),
        "identity bind matrix should fit the FBX array contract"
    );
    let Some(identity_token) = identity_token_result else {
        return;
    };
    let identity_count = byte_window_count(
        &first,
        &identity_token,
    );
    assert!(
        identity_count >= 4_usize,
        concat!(
            "mesh, bone, and cluster bind records must retain their source ",
            "space matrices instead of duplicating the export rotation; ",
            "found {}"
        ),
        identity_count
    );
    let Some(texture) = textures.first() else {
        return;
    };
    let embedded_png = &texture.content;
    assert!(
        first
            .windows(embedded_png.len())
            .any(|window| window == embedded_png),
        "binary FBX must store exact PNG bytes in Video.Content"
    );
    assert!(
        first
            .windows(b"Filename".len())
            .any(|window| window == b"Filename"),
        "embedded Video nodes must use Maya-compatible Filename spelling"
    );
    assert!(
        !first
            .windows(b"textures/skin.png".len())
            .any(|window| window == b"textures/skin.png"),
        "embedded textures must not depend on a sibling textures directory"
    );
    for (name, value) in [
        (
            "UpAxis", 1_i32,
        ),
        (
            "UpAxisSign",
            1_i32,
        ),
        (
            "FrontAxis",
            2_i32,
        ),
        (
            "FrontAxisSign",
            1_i32,
        ),
        (
            "CoordAxis",
            0_i32,
        ),
        (
            "CoordAxisSign",
            1_i32,
        ),
        (
            "OriginalUpAxis",
            1_i32,
        ),
        (
            "OriginalUpAxisSign",
            1_i32,
        ),
    ] {
        let token_result = integer_property_token(
            name, value,
        );
        assert!(
            token_result.is_some(),
            "axis property token should fit u32: {name}"
        );
        let Some(token) = token_result else {
            return;
        };
        assert!(
            first
                .windows(token.len())
                .any(|window| window == token),
            "binary FBX axis property must match Maya 7.7: {name}={value}"
        );
    }
    let mut source_uv_bytes = Vec::new();
    source_uv_bytes.extend_from_slice(&0.25_f64.to_le_bytes());
    source_uv_bytes.extend_from_slice(&0.75_f64.to_le_bytes());
    assert!(
        first
            .windows(source_uv_bytes.len())
            .any(|window| window == source_uv_bytes),
        "binary FBX must preserve the authored palette row selected by source \
         V"
    );
    let mut flipped_uv_bytes = Vec::new();
    flipped_uv_bytes.extend_from_slice(&0.25_f64.to_le_bytes());
    flipped_uv_bytes.extend_from_slice(&0.25_f64.to_le_bytes());
    assert!(
        !first
            .windows(flipped_uv_bytes.len())
            .any(|window| window == flipped_uv_bytes),
        "binary FBX must not vertically flip palette UV coordinates"
    );
    let footer_version_offset_result = first
        .len()
        .checked_sub(FOOTER_VERSION_DISTANCE);
    assert!(footer_version_offset_result.is_some());
    let Some(footer_version_offset) = footer_version_offset_result else {
        return;
    };
    let footer_version_end_result = footer_version_offset.checked_add(4);
    assert!(footer_version_end_result.is_some());
    let Some(footer_version_end) = footer_version_end_result else {
        return;
    };
    let footer_version = first
        .get(footer_version_offset..footer_version_end)
        .and_then(|bytes| <[u8; 4]>::try_from(bytes).ok())
        .map(u32::from_le_bytes);
    assert_eq!(
        footer_version,
        Some(FBX_VERSION)
    );
}
