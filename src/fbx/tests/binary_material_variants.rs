// File:
//   - binary_material_variants.rs
// Path:
//   - src/fbx/tests/binary_material_variants.rs
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
//   - Regression coverage for shared source shaders with different geometry
//   - semantics.
// - Must-Not:
//   - Depend on private assets, generated catalogs, or installed DCC software.
// - Allows:
//   - Synthetic static geometry and binary material-name assertions.
// - Summary:
//   - Proves light geometry cannot make sibling body geometry emissive.
//
// Large file:
//   - false
//

//! Shared-source material variant regression coverage.

use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::write_binary_model_fbx;
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::texture::MaterialBinding;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn mesh(name: &str) -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "shared_m",
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
    .map_err(|error| format!("primitive group failed: {error:?}"))?;
    MeshAsset::new(
        name,
        vec![group],
    )
    .map_err(|error| format!("mesh failed: {error:?}"))
}

fn output_path() -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-binary-material-variants-{}.fbx",
            std::process::id()
        ),
    )
}

fn remove_if_present(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("temporary FBX cleanup failed: {error}")),
    }
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
fn shared_shader_creates_isolated_light_material_variant() -> Result<(), String>
{
    let path = output_path();
    remove_if_present(&path)?;
    let material = MaterialBinding::new(
        "shared_m", None,
    )
    .map_err(|error| format!("material failed: {error:?}"))?;
    let summary = write_binary_model_fbx(
        "material-variants",
        &[
            mesh("vehicle-body")?,
            mesh("lightsShape")?,
        ],
        &[material],
        &path,
    )
    .map_err(|error| format!("FBX write failed: {error:?}"))?;
    if summary.materials != 2 {
        return Err(
            format!(
                "shared shader did not split into two semantic variants: \
                 {summary:?}"
            ),
        );
    }
    let bytes =
        fs::read(&path).map_err(|error| format!("FBX read failed: {error}"))?;
    if !contains_token(
        &bytes,
        "shared_m__light-emitter",
    ) {
        return Err("light material variant identity is missing".to_owned());
    }
    remove_if_present(&path)?;
    Ok(())
}
