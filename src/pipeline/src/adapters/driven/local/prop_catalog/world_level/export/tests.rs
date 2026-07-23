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

use fbx::adapters::driven::binary_character_writer::{
    ModelExportRootPolicy, ModelUvPolicy,
};
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::texture::MaterialBinding;
use shar_sha256::digest_hex;

use super::{
    MasterContent, PreparedTexture, WORLD_ROOT_POLICY, WORLD_UV_POLICY,
    append_world_fbx_to_guide, write_content_fbx,
};

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

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
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
            ModelExportRootPolicy::ReflectX,
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

#[test]
fn guide_append_preserves_exterior_world_fbx_geometry_exactly()
-> Result<(), String> {
    let source = textured_content()?;
    let source_mesh = source
        .meshes
        .first()
        .ok_or_else(|| "world FBX fixture mesh is missing".to_owned())?;
    let source_group = source_mesh
        .groups
        .first()
        .ok_or_else(|| "world FBX fixture group is missing".to_owned())?;
    let expected_positions = source_group
        .positions
        .clone();
    let expected_normals = source_group
        .normals
        .clone();
    let expected_uvs = source_group
        .uvs
        .clone();
    let expected_triangles = source_group
        .triangles
        .clone();
    let mut guide = MasterContent::default();
    append_world_fbx_to_guide(
        &source, &mut guide,
    )
    .map_err(|error| error.to_string())?;
    let combined_mesh = guide
        .meshes
        .first()
        .ok_or_else(|| "combined guide mesh is missing".to_owned())?;
    let combined_group = combined_mesh
        .groups
        .first()
        .ok_or_else(|| "combined guide group is missing".to_owned())?;
    assert_eq!(
        combined_group.positions,
        expected_positions,
    );
    assert_eq!(
        combined_group.normals,
        expected_normals,
    );
    assert_eq!(
        combined_group.uvs,
        expected_uvs,
    );
    assert_eq!(
        combined_group.triangles,
        expected_triangles,
    );
    assert_eq!(
        guide
            .materials
            .len(),
        source
            .materials
            .len(),
    );
    assert_eq!(
        guide
            .textures
            .len(),
        source
            .textures
            .len(),
    );
    Ok(())
}

#[test]
fn guide_append_preserves_interior_world_fbx_geometry_exactly()
-> Result<(), String> {
    let mut source = textured_content()?;
    let group = source
        .meshes
        .first_mut()
        .and_then(
            |mesh| {
                mesh.groups
                    .first_mut()
            },
        )
        .ok_or_else(|| "interior FBX fixture group is missing".to_owned())?;
    group.positions = vec![
        [
            1.0, 2.0, 3.0,
        ],
        [
            4.0, 5.0, 6.0,
        ],
        [
            7.0, 8.0, 9.0,
        ],
    ];
    group.normals = vec![
        [
            1.0, 0.0, 0.0,
        ];
        3
    ];
    let expected_positions = group
        .positions
        .clone();
    let expected_normals = group
        .normals
        .clone();
    let expected_uvs = group
        .uvs
        .clone();
    let expected_triangles = group
        .triangles
        .clone();
    let mut guide = MasterContent::default();
    append_world_fbx_to_guide(
        &source, &mut guide,
    )
    .map_err(|error| error.to_string())?;
    let combined = guide
        .meshes
        .first()
        .and_then(
            |mesh| {
                mesh.groups
                    .first()
            },
        )
        .ok_or_else(|| "combined interior guide group is missing".to_owned())?;
    assert_eq!(
        combined.positions,
        expected_positions
    );
    assert_eq!(
        combined.normals,
        expected_normals
    );
    assert_eq!(
        combined.uvs,
        expected_uvs
    );
    assert_eq!(
        combined.triangles,
        expected_triangles
    );
    Ok(())
}

#[test]
fn world_fbx_policy_reflects_x_once_and_preserves_authored_uvs() {
    assert_eq!(
        WORLD_ROOT_POLICY,
        ModelExportRootPolicy::ReflectX
    );
    assert_eq!(
        WORLD_UV_POLICY,
        ModelUvPolicy::Preserve
    );
}
