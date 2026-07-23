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
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxError, CharacterBinaryFbxSummary, ModelExportRootPolicy,
    ModelUvPolicy, write_binary_model_fbx,
    write_binary_model_fbx_with_policies,
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

fn remove_if_present(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(
            format!(
                "temporary FBX cleanup failed for {}: {error}",
                path.display()
            ),
        ),
    }
}

fn output_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-binary-model-{label}-{}.fbx",
            std::process::id()
        ),
    )
}

fn find_token(bytes: &[u8], token: &str) -> Option<usize> {
    bytes
        .windows(token.len())
        .position(|window| window == token.as_bytes())
}

fn contains_token(bytes: &[u8], token: &str) -> bool {
    find_token(
        bytes, token,
    )
    .is_some()
}

fn encoded_vector(value: [f64; 3]) -> Vec<u8> {
    value
        .into_iter()
        .flat_map(
            |component| std::iter::once(b'D').chain(component.to_le_bytes()),
        )
        .collect()
}

fn export_root_bytes(bytes: &[u8]) -> Result<&[u8], String> {
    let start = find_token(
        bytes,
        "SHAR_Export_Root",
    )
    .ok_or_else(|| "static FBX export root is missing".to_owned())?;
    let end = bytes[start..]
        .windows("Geometry".len())
        .position(|window| window == b"Geometry")
        .map(|relative| start + relative)
        .ok_or_else(
            || "static FBX geometry after export root is missing".to_owned(),
        )?;
    Ok(&bytes[start..end])
}

#[test]
fn static_model_is_deterministic_and_has_no_rig_objects() -> Result<(), String>
{
    let first = output_path("first");
    let second = output_path("second");
    remove_if_present(&first)?;
    remove_if_present(&second)?;
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
    let expected = CharacterBinaryFbxSummary {
        geometries: 1,
        bones: 0,
        clusters: 0,
        materials: 1,
        textures: 0,
        animations: 0,
    };
    if first_summary != expected {
        return Err(format!("unexpected static summary: {first_summary:?}"));
    }
    if first_summary != second_summary {
        return Err(
            format!(
                "static summaries differ: {first_summary:?} != \
                 {second_summary:?}"
            ),
        );
    }
    let first_bytes = fs::read(&first)
        .map_err(|error| format!("first static read failed: {error}"))?;
    let second_bytes = fs::read(&second)
        .map_err(|error| format!("second static read failed: {error}"))?;
    if first_bytes != second_bytes {
        return Err("static FBX bytes are not deterministic".to_owned());
    }
    if !first_bytes.starts_with(BINARY_MAGIC) {
        return Err("static FBX binary magic is missing".to_owned());
    }
    for required in [
        "Geometry",
        "Model",
        "Material",
        "ColorSet_1",
    ] {
        if !contains_token(
            &first_bytes,
            required,
        ) {
            return Err(format!("static FBX is missing {required}"));
        }
    }
    for forbidden in [
        "Deformer",
        "Pose",
        "NodeAttribute",
        "LimbNode",
        "AnimationStack",
        "AnimationCurve",
    ] {
        if contains_token(
            &first_bytes,
            forbidden,
        ) {
            return Err(format!("static FBX contains forbidden {forbidden}"));
        }
    }
    remove_if_present(&first)?;
    remove_if_present(&second)?;
    Ok(())
}

#[test]
fn world_reflection_is_shared_by_exterior_and_interior() -> Result<(), String> {
    let legacy_path = output_path("legacy-root");
    let exterior_path = output_path("exterior-root");
    let interior_path = output_path("interior-root");
    for path in [
        &legacy_path,
        &exterior_path,
        &interior_path,
    ] {
        remove_if_present(path)?;
    }
    let mesh = model_mesh()?;
    let material = material()?;
    write_binary_model_fbx(
        "legacy-root-model",
        std::slice::from_ref(&mesh),
        std::slice::from_ref(&material),
        &legacy_path,
    )
    .map_err(|error| format!("legacy-root write failed: {error:?}"))?;
    write_binary_model_fbx_with_policies(
        "exterior-root-model",
        std::slice::from_ref(&mesh),
        std::slice::from_ref(&material),
        ModelUvPolicy::Preserve,
        ModelExportRootPolicy::ReflectX,
        &exterior_path,
    )
    .map_err(|error| format!("exterior-root write failed: {error:?}"))?;
    write_binary_model_fbx_with_policies(
        "interior-root-model",
        &[mesh],
        &[material],
        ModelUvPolicy::Preserve,
        ModelExportRootPolicy::ReflectX,
        &interior_path,
    )
    .map_err(|error| format!("interior-root write failed: {error:?}"))?;

    let legacy_bytes = fs::read(&legacy_path)
        .map_err(|error| format!("legacy-root read failed: {error}"))?;
    let exterior_bytes = fs::read(&exterior_path)
        .map_err(|error| format!("exterior-root read failed: {error}"))?;
    let interior_bytes = fs::read(&interior_path)
        .map_err(|error| format!("interior-root read failed: {error}"))?;
    let legacy_root = export_root_bytes(&legacy_bytes)?;
    let exterior_root = export_root_bytes(&exterior_bytes)?;
    let interior_root = export_root_bytes(&interior_bytes)?;

    let identity_scale = encoded_vector(
        [
            1.0, 1.0, 1.0,
        ],
    );
    let zero_rotation = encoded_vector(
        [
            0.0, 0.0, 0.0,
        ],
    );
    let legacy_rotation = encoded_vector(
        [
            0.0, 180.0, 0.0,
        ],
    );
    let reflected_scale = encoded_vector(
        [
            -1.0, 1.0, 1.0,
        ],
    );

    if !legacy_root
        .windows(legacy_rotation.len())
        .any(|window| window == legacy_rotation)
        || !legacy_root
            .windows(identity_scale.len())
            .any(|window| window == identity_scale)
    {
        return Err(
            "legacy static root no longer preserves character orientation"
                .to_owned(),
        );
    }
    if !exterior_root
        .windows(zero_rotation.len())
        .any(|window| window == zero_rotation)
        || !exterior_root
            .windows(reflected_scale.len())
            .any(|window| window == reflected_scale)
    {
        return Err("exterior world root lacks the X reflection".to_owned());
    }
    if exterior_root
        .windows(legacy_rotation.len())
        .any(|window| window == legacy_rotation)
    {
        return Err(
            "exterior world root inherited the character rotation".to_owned(),
        );
    }
    if !interior_root
        .windows(zero_rotation.len())
        .any(|window| window == zero_rotation)
        || !interior_root
            .windows(reflected_scale.len())
            .any(|window| window == reflected_scale)
    {
        return Err("interior static root lacks the X reflection".to_owned());
    }
    if interior_root
        .windows(legacy_rotation.len())
        .any(|window| window == legacy_rotation)
    {
        return Err(
            "interior static root inherited the character rotation".to_owned(),
        );
    }

    if exterior_root != interior_root {
        return Err("exterior and interior world roots diverged".to_owned());
    }

    assert_eq!(
        ModelExportRootPolicy::ReflectX
            .relative_matrix_to(ModelExportRootPolicy::ReflectX),
        [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0,
        ],
    );

    for path in [
        &legacy_path,
        &exterior_path,
        &interior_path,
    ] {
        remove_if_present(path)?;
    }
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

    if result != Err(CharacterBinaryFbxError::InvalidModelName) {
        return Err(format!("unexpected invalid-name result: {result:?}"));
    }
    if path.exists() {
        return Err("invalid static model created an artifact".to_owned());
    }
    Ok(())
}
