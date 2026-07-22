// File:
//   - tests.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/export/
//     tests.rs
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
//   - Regression coverage for world FBX publication boundaries.
// - Must-Not:
//   - Read extracted game assets or depend on generated repository outputs.
// - Allows:
//   - Build one minimal textured scene and verify its portable publication.
// - Summary:
//   - Proves nested world FBXs carry resolvable external textures.
//
// Large file:
//   - false
//

//! World FBX publication regression coverage.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_character_writer::ModelUvPolicy;
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::texture::MaterialBinding;
use shar_sha256::digest_hex;

use super::{MasterContent, PreparedTexture, write_content_fbx};

const TEXTURE_FILE_NAME: &str = "interior-test.png";
const TEXTURE_BYTES: &[u8] = b"canonical-interior-texture-payload";

fn textured_mesh() -> Result<MeshAsset, String> {
    let group = PrimitiveGroup::new(
        0,
        "interior-material",
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
    .map_err(|error| format!("interior fixture group failed: {error:?}"))?;
    MeshAsset::new(
        "interior-mesh",
        vec![group],
    )
    .map_err(|error| format!("interior fixture mesh failed: {error:?}"))
}

fn textured_content() -> Result<MasterContent, String> {
    let material = MaterialBinding::new(
        "interior-material",
        Some(TEXTURE_FILE_NAME.to_owned()),
    )
    .map_err(|error| format!("interior fixture material failed: {error:?}"))?;
    let texture = PreparedTexture {
        file_name: TEXTURE_FILE_NAME.to_owned(),
        bytes: TEXTURE_BYTES.to_vec(),
        sha256: digest_hex(TEXTURE_BYTES),
    };
    Ok(
        MasterContent {
            meshes: vec![textured_mesh()?],
            review: Vec::new(),
            materials: BTreeMap::from(
                [
                    (
                        material
                            .material_name
                            .clone(),
                        material,
                    ),
                ],
            ),
            textures: BTreeMap::from(
                [
                    (
                        texture
                            .file_name
                            .clone(),
                        texture,
                    ),
                ],
            ),
            packages: Vec::new(),
        },
    )
}

fn temporary_root() -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "shar-world-nested-texture-test-{}",
            std::process::id()
        ),
    )
}

fn contains_bytes(
    haystack: &[u8],
    needle: &[u8],
) -> bool {
    haystack
        .windows(needle.len())
        .any(|window| window == needle)
}

fn remove_if_present(path: &Path) -> Result<(), String> {
    match fs::remove_dir_all(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(
            format!(
                "interior fixture cleanup failed for {}: {error}",
                path.display()
            ),
        ),
    }
}

#[test]
fn nested_fbx_publishes_adjacent_external_textures() -> Result<(), String> {
    let root = temporary_root();
    remove_if_present(&root)?;
    fs::create_dir_all(&root).map_err(
        |error| format!("interior fixture root creation failed: {error}"),
    )?;
    let result = (|| {
        let relative = "interiors/i00-test/i00-test.fbx";
        let mut content = textured_content()?;
        let record = write_content_fbx(
            "i00-test",
            relative,
            &mut content,
            &root,
            ModelUvPolicy::Preserve,
        )
        .map_err(|error| error.to_string())?
        .ok_or_else(|| String::from("interior fixture FBX was not written"))?;
        if record
            .summary
            .textures
            != 1
        {
            return Err(
                format!(
                    "interior fixture texture count changed: {}",
                    record
                        .summary
                        .textures
                ),
            );
        }
        let fbx_path = root.join(relative);
        let fbx_bytes = fs::read(&fbx_path).map_err(
            |error| format!("interior fixture FBX read failed: {error}"),
        )?;
        if !contains_bytes(
            &fbx_bytes,
            format!("textures/{TEXTURE_FILE_NAME}").as_bytes(),
        ) {
            return Err(String::from("interior FBX texture reference changed"));
        }
        let texture_path = root
            .join("interiors/i00-test/textures")
            .join(TEXTURE_FILE_NAME);
        let published = fs::read(&texture_path).map_err(
            |error| {
                format!(
                    "nested interior texture is not resolvable beside the \
                     FBX: {error}"
                )
            },
        )?;
        if published != TEXTURE_BYTES {
            return Err(String::from("nested interior texture bytes changed"));
        }
        Ok(())
    })();
    let cleanup = remove_if_present(&root);
    result.and(cleanup)
}
