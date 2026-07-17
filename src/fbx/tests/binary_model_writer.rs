// File:
//   - binary_model_writer.rs
// Path:
//   - src/fbx/tests/binary_model_writer.rs
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
//   - Deterministic static-model binary FBX regression coverage.
// - Must-Not:
//   - Depend on private extracted assets or installed DCC applications.
// - Allows:
//   - Synthetic geometry, material, byte-level family checks, and paired
//     writes.
// - Split-When:
//   - Static model material or geometry families need independent fixtures.
// - Merge-When:
//   - Character and static writers share one identical public acceptance test.
// - Summary:
//   - Proves static FBX output contains no synthetic rig or animation objects.
// - Description:
//   - Verifies determinism, summary counts, and model-input validation.
// - Usage:
//   - Run through the canonical FBX crate test gate.
// - Defaults:
//   - One triangle, one material, no texture, no skeleton, and no animation.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Deterministic static-model binary FBX regression coverage.

use std::fs;
use std::path::PathBuf;

use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxError, CharacterBinaryFbxSummary, write_binary_model_fbx,
};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::texture::MaterialBinding;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

const BINARY_MAGIC: &[u8; 23] = b"Kaydara FBX Binary  \x00\x1a\x00";

fn model_mesh() -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "material",
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
                0.0, 0.0,
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
    .and_then(
        |group| {
            group.with_colors(
                vec![
                    [
                        1.0, 0.0, 0.0, 1.0,
                    ],
                    [
                        0.0, 1.0, 0.0, 1.0,
                    ],
                    [
                        0.0, 0.0, 1.0, 1.0,
                    ],
                ],
            )
        },
    )
    .map_err(|error| format!("static primitive group failed: {error:?}"))?;
    MeshAsset::new(
        "model",
        vec![group],
    )
    .map_err(|error| format!("static mesh failed: {error:?}"))
}

fn material() -> Result<MaterialBinding, String> {
    MaterialBinding::new(
        "material", None,
    )
    .map_err(|error| format!("static material failed: {error:?}"))
}

fn output_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-binary-model-{label}-{}.fbx",
            std::process::id()
        ),
    )
}

fn contains_token(
    bytes: &[u8],
    token: &str,
) -> bool {
    bytes
        .windows(token.len())
        .any(|window| window == token.as_bytes())
}

#[test]
fn static_model_is_deterministic_and_has_no_rig_objects() -> Result<(), String>
{
    let first = output_path("first");
    let second = output_path("second");
    let _ = fs::remove_file(&first);
    let _ = fs::remove_file(&second);
    let mesh = model_mesh()?;
    let material = material()?;

    let first_summary = write_binary_model_fbx(
        "static-model",
        std::slice::from_ref(&mesh),
        std::slice::from_ref(&material),
        &first,
    )
    .map_err(|error| format!("first static write failed: {error:?}"))?;
    let second_summary = write_binary_model_fbx(
        "static-model",
        &[mesh],
        &[material],
        &second,
    )
    .map_err(|error| format!("second static write failed: {error:?}"))?;
    assert_eq!(
        first_summary,
        CharacterBinaryFbxSummary {
            geometries: 1,
            bones: 0,
            clusters: 0,
            materials: 1,
            textures: 0,
            animations: 0,
        }
    );
    assert_eq!(
        first_summary,
        second_summary
    );
    let first_bytes = fs::read(&first)
        .map_err(|error| format!("first static read failed: {error}"))?;
    let second_bytes = fs::read(&second)
        .map_err(|error| format!("second static read failed: {error}"))?;
    assert_eq!(
        first_bytes,
        second_bytes
    );
    assert!(first_bytes.starts_with(BINARY_MAGIC));
    for required in [
        "Geometry",
        "Model",
        "Material",
        "ColorSet_1",
    ] {
        assert!(
            contains_token(
                &first_bytes,
                required
            )
        );
    }
    for forbidden in [
        "Deformer",
        "Pose",
        "NodeAttribute",
        "LimbNode",
        "AnimationStack",
        "AnimationCurve",
    ] {
        assert!(
            !contains_token(
                &first_bytes,
                forbidden
            )
        );
    }
    let _ = fs::remove_file(first);
    let _ = fs::remove_file(second);
    Ok(())
}

#[test]
fn static_model_rejects_invalid_aggregate_identity() -> Result<(), String> {
    let mesh = model_mesh()?;
    let material = material()?;
    let path = output_path("invalid-name");
    let result = write_binary_model_fbx(
        " invalid ",
        &[mesh],
        &[material],
        &path,
    );

    assert_eq!(
        result,
        Err(CharacterBinaryFbxError::InvalidModelName)
    );
    assert!(!path.exists());
    Ok(())
}
