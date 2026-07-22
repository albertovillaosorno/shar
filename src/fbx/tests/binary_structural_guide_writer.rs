// File:
//   - binary_structural_guide_writer.rs
// Path:
//   - src/fbx/tests/binary_structural_guide_writer.rs
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
//   - Structural-guide FBX 7.7 byte and public validation regression coverage.
// - Must-Not:
//   - Depend on extracted assets, Blender, Unreal, or network access.
// - Allows:
//   - Synthetic one-triangle payloads, paired writes, and marker inspection.
// - Summary:
//   - Proves the one-mesh, one-material, four-UV writer contract.
//
// Large file:
//   - false

//! Structural-guide FBX writer regression coverage.

use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_structural_guide_writer::{
    STRUCTURAL_GUIDE_ASSET_NAME, STRUCTURAL_GUIDE_MATERIAL_NAME,
    STRUCTURAL_GUIDE_TEXTURE_PATH, STRUCTURAL_GUIDE_UV_NAMES,
    StructuralGuideFbxError, StructuralGuideFbxSummary, StructuralGuideMesh,
    write_binary_structural_guide_fbx,
};

const BINARY_MAGIC: &[u8; 23] = b"Kaydara FBX Binary  \x00\x1a\x00";

fn guide_mesh() -> StructuralGuideMesh {
    StructuralGuideMesh {
        positions: vec![
            [
                0.0, 0.0, 0.0,
            ],
            [
                100.0, 0.0, 0.0,
            ],
            [
                0.0, 100.0, 0.0,
            ],
        ],
        normals: vec![
            [
                0.0, 0.0, 1.0
            ];
            3
        ],
        triangles: vec![
            [
                0, 1, 2,
            ],
        ],
        source_uvs: vec![
            [
                0.0, 0.0,
            ],
            [
                2.0, 0.0,
            ],
            [
                0.0, 2.0,
            ],
        ],
        atlas_offsets: vec![[0.000_610_351_56; 2]; 3],
        atlas_scales: vec![[0.015_380_859; 2]; 3],
        atlas_flags: vec![
            [
                1.0, 0.0
            ];
            3
        ],
    }
}

fn output_path(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-structural-guide-{label}-{}.fbx",
            std::process::id()
        ),
    )
}

fn remove_if_present(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("temporary cleanup failed: {error}")),
    }
}

fn contains(
    bytes: &[u8],
    token: &str,
) -> bool {
    bytes
        .windows(token.len())
        .any(|window| window == token.as_bytes())
}

#[test]
fn guide_is_deterministic_fbx_7700_with_four_uv_channels() -> Result<(), String>
{
    let first = output_path("first");
    let second = output_path("second");
    remove_if_present(&first)?;
    remove_if_present(&second)?;
    let mesh = guide_mesh();
    let first_summary = write_binary_structural_guide_fbx(
        &mesh, &first,
    )
    .map_err(|error| format!("first write failed: {error:?}"))?;
    let second_summary = write_binary_structural_guide_fbx(
        &mesh, &second,
    )
    .map_err(|error| format!("second write failed: {error:?}"))?;
    let expected = StructuralGuideFbxSummary {
        vertices: 3,
        triangles: 1,
        bounds_min_cm: [
            0.0, 0.0, 0.0,
        ],
        bounds_max_cm: [
            100.0, 100.0, 0.0,
        ],
    };
    if first_summary != expected || second_summary != expected {
        return Err(
            format!(
                "unexpected summaries: {first_summary:?} {second_summary:?}"
            ),
        );
    }
    let first_bytes = fs::read(&first).map_err(|error| error.to_string())?;
    let second_bytes = fs::read(&second).map_err(|error| error.to_string())?;
    if first_bytes != second_bytes {
        return Err("structural-guide FBX bytes differ".to_owned());
    }
    if !first_bytes.starts_with(BINARY_MAGIC) {
        return Err("binary FBX magic is missing".to_owned());
    }
    let version = first_bytes
        .get(BINARY_MAGIC.len()..BINARY_MAGIC.len() + 4)
        .and_then(|bytes| <[u8; 4]>::try_from(bytes).ok())
        .map(u32::from_le_bytes)
        .ok_or_else(|| "FBX version bytes are missing".to_owned())?;
    if version != 7_700 {
        return Err(format!("unexpected FBX version: {version}"));
    }
    for required in [
        STRUCTURAL_GUIDE_ASSET_NAME,
        STRUCTURAL_GUIDE_MATERIAL_NAME,
        STRUCTURAL_GUIDE_TEXTURE_PATH,
        STRUCTURAL_GUIDE_UV_NAMES[0],
        STRUCTURAL_GUIDE_UV_NAMES[1],
        STRUCTURAL_GUIDE_UV_NAMES[2],
        STRUCTURAL_GUIDE_UV_NAMES[3],
    ] {
        if !contains(
            &first_bytes,
            required,
        ) {
            return Err(format!("required FBX marker is missing: {required}"));
        }
    }
    for forbidden in [
        "SHAR_Export_Root",
        "AnimationStack",
        "AnimationCurve",
        "Deformer",
        "Pose",
        "LimbNode",
    ] {
        if contains(
            &first_bytes,
            forbidden,
        ) {
            return Err(format!("forbidden FBX marker exists: {forbidden}"));
        }
    }
    remove_if_present(&first)?;
    remove_if_present(&second)?;
    Ok(())
}

#[test]
fn guide_rejects_misaligned_uv_channel() -> Result<(), String> {
    let path = output_path("invalid-uv");
    remove_if_present(&path)?;
    let mut mesh = guide_mesh();
    let _removed = mesh
        .atlas_flags
        .pop();
    let result = write_binary_structural_guide_fbx(
        &mesh, &path,
    );
    let expected = StructuralGuideFbxError::ChannelLengthMismatch {
        channel: STRUCTURAL_GUIDE_UV_NAMES[3],
        positions: 3,
        values: 2,
    };
    if result != Err(expected) {
        return Err(format!("unexpected invalid UV result: {result:?}"));
    }
    if path.exists() {
        return Err("invalid guide created an FBX".to_owned());
    }
    Ok(())
}

#[test]
fn guide_refuses_to_overwrite_existing_artifact() -> Result<(), String> {
    let path = output_path("existing");
    remove_if_present(&path)?;
    let mesh = guide_mesh();
    let _summary = write_binary_structural_guide_fbx(
        &mesh, &path,
    )
    .map_err(|error| format!("initial write failed: {error:?}"))?;
    let result = write_binary_structural_guide_fbx(
        &mesh, &path,
    );
    if !matches!(
        result,
        Err(StructuralGuideFbxError::OutputExists(_))
    ) {
        return Err(format!("unexpected overwrite result: {result:?}"));
    }
    remove_if_present(&path)?;
    Ok(())
}
