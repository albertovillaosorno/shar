// File:
//   - structural_guide.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     structural_guide.rs
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
//   - Canonical structural-guide atlas, mesh, manifest, and atomic publication.
// - Must-Not:
//   - Change, reposition, repair, or augment normal world-FBX geometry; invoke
//     Blender; or publish partial files.
// - Allows:
//   - A disposable normal-world FBX transaction and one four-file combined
//     guide output.
// - Summary:
//   - Orchestrates the Unreal Editor-only structural-guide export.
//
// Large file:
//   - false
//

//! Canonical Unreal structural-guide publication.

mod atlas;
mod mesh;
mod model;

use std::fs::{self, OpenOptions};
use std::io::Write as _;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::binary_structural_guide_writer::{
    STRUCTURAL_GUIDE_ASSET_NAME, STRUCTURAL_GUIDE_MATERIAL_NAME,
    STRUCTURAL_GUIDE_TEXTURE_PATH, STRUCTURAL_GUIDE_UV_NAMES,
    write_binary_structural_guide_fbx,
};
use shar_sha256::digest_hex;

use super::collect_structural_guide_source;
use crate::domain::{PipelineError, StageReport};

mod manifest;

/// Stable command/stage identity.
pub(super) const STAGE: &str = "fbx-export-structural-guide";
const FBX_FILE: &str = "SM_SHAR_StructuralGuide_Canonical.fbx";
const MANIFEST_FILE: &str = "SM_SHAR_StructuralGuide_Canonical.manifest.json";
const ATLAS_FILE: &str = "textures/T_SHAR_StructuralGuide_Atlas.png";
const LAYOUT_FILE: &str = "textures/T_SHAR_StructuralGuide_Atlas.layout.json";
/// Export one canonical, opaque, one-section Unreal structural guide.
///
/// # Errors
///
/// Returns an error when canonical world collection, atlas packing, mesh
/// assembly, file validation, or atomic publication fails.
pub(in crate::adapters::driven::local) fn export(
    index_path: &Path,
    game_root: &Path,
    coordinate_root: &Path,
    output_dir: &Path,
) -> Result<StageReport, PipelineError> {
    super::super::ensure_missing(
        output_dir,
        "structural-guide output",
    )?;
    let staging = super::super::staging_path(output_dir)?;
    super::super::ensure_missing(
        &staging,
        "structural-guide staging",
    )?;
    fs::create_dir_all(&staging).map_err(
        |error| {
            PipelineError::new(
                format!("structural-guide staging failed: {error}"),
            )
        },
    )?;
    let result = build(
        index_path,
        game_root,
        coordinate_root,
        &staging,
    )
    .and_then(
        |report| {
            fs::rename(
                &staging, output_dir,
            )
            .map_err(
                |error| {
                    PipelineError::new(
                        format!("structural-guide publication failed: {error}"),
                    )
                },
            )?;
            Ok(report)
        },
    );
    if result.is_err() {
        drop(fs::remove_dir_all(&staging));
    }
    result
}

fn build(
    index_path: &Path,
    game_root: &Path,
    coordinate_root: &Path,
    staging: &Path,
) -> Result<StageReport, PipelineError> {
    let temporary_world = staging.join(".canonical-world");
    let content = collect_structural_guide_source(
        index_path,
        game_root,
        coordinate_root,
        &temporary_world,
    )?;
    // The guide consumes only geometry already owned by generated world FBXs.
    let atlas = atlas::build(&content)?;
    let (guide_mesh, counts) = mesh::build(
        &content, &atlas,
    )?;
    let texture_dir = staging.join("textures");
    fs::create_dir_all(&texture_dir).map_err(
        |error| {
            PipelineError::new(
                format!("structural-guide texture staging failed: {error}"),
            )
        },
    )?;
    let atlas_path = staging.join(ATLAS_FILE);
    write_new(
        &atlas_path,
        &atlas.png_bytes,
    )?;
    let mut layout_bytes = serde_json::to_vec_pretty(&atlas.layout)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    layout_bytes.push(b'\n');
    let layout_path = staging.join(LAYOUT_FILE);
    write_new(
        &layout_path,
        &layout_bytes,
    )?;
    let fbx_path = staging.join(FBX_FILE);
    let summary = write_binary_structural_guide_fbx(
        &guide_mesh,
        &fbx_path,
    )
    .map_err(
        |error| PipelineError::new(format!("guide FBX failed: {error:?}")),
    )?;
    let fbx_bytes = fs::read(&fbx_path).map_err(
        |error| {
            PipelineError::new(
                format!("structural-guide FBX read failed: {error}"),
            )
        },
    )?;
    let fbx_sha256 = digest_hex(&fbx_bytes);
    let atlas_sha256 = digest_hex(&atlas.png_bytes);
    let layout_sha256 = digest_hex(&layout_bytes);
    let manifest_bytes = manifest::render(
        summary,
        counts,
        &fbx_sha256,
        &atlas_sha256,
        &layout_sha256,
    )?;
    let manifest_path = staging.join(MANIFEST_FILE);
    write_new(
        &manifest_path,
        &manifest_bytes,
    )?;
    validate_publication(
        staging,
        &fbx_bytes,
        &atlas.png_bytes,
        &layout_bytes,
        &manifest_bytes,
    )?;
    let (files, bytes) = publication_inventory(staging)?;
    Ok(
        StageReport {
            name: STAGE,
            files,
            bytes,
            note: format!(
                concat!(
                    "combined post-world-FBX geometry without spatial \
                     changes: ",
                    "one mesh, one material, one RGB atlas, four UV channels, ",
                    "{} vertices, {} triangles, {} source meshes, and {} ",
                    "documented vertex-color approximations"
                ),
                summary.vertices,
                summary.triangles,
                counts.input_meshes,
                counts.approximated_vertex_color_triangles,
            ),
        },
    )
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<(), PipelineError> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "structural-guide create-new write failed: {}:{error}",
                        path.display()
                    ),
                )
            },
        )?;
    file.write_all(bytes)
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "structural-guide write failed: {}:{error}",
                        path.display()
                    ),
                )
            },
        )
}

fn validate_publication(
    staging: &Path,
    fbx: &[u8],
    atlas: &[u8],
    layout: &[u8],
    manifest: &[u8],
) -> Result<(), PipelineError> {
    let (files, _bytes) = publication_inventory(staging)?;
    if files != 4 {
        return Err(
            PipelineError::new(
                format!("structural-guide file count changed: {files}"),
            ),
        );
    }
    validate_fbx(fbx)?;
    validate_atlas(atlas)?;
    let layout_value: serde_json::Value = serde_json::from_slice(layout)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    let manifest_value: serde_json::Value = serde_json::from_slice(manifest)
        .map_err(|error| PipelineError::new(error.to_string()))?;
    validate_layout(&layout_value)?;
    validate_manifest(&manifest_value)?;
    for path in [
        FBX_FILE,
        MANIFEST_FILE,
        ATLAS_FILE,
        LAYOUT_FILE,
    ] {
        if !staging
            .join(path)
            .is_file()
        {
            return Err(
                PipelineError::new(
                    format!(
                        "structural-guide required file is missing: {path}"
                    ),
                ),
            );
        }
    }
    Ok(())
}

fn validate_fbx(bytes: &[u8]) -> Result<(), PipelineError> {
    const MAGIC: &[u8] = b"Kaydara FBX Binary  \x00\x1a\x00";
    if !bytes.starts_with(MAGIC) {
        return Err(PipelineError::new("structural-guide FBX is not binary"));
    }
    let version = bytes
        .get(MAGIC.len()..MAGIC.len() + 4)
        .and_then(|value| <[u8; 4]>::try_from(value).ok())
        .map(u32::from_le_bytes)
        .ok_or_else(
            || PipelineError::new("structural-guide FBX version is missing"),
        )?;
    if version != 7_700 {
        return Err(
            PipelineError::new(
                format!("structural-guide FBX version changed: {version}"),
            ),
        );
    }
    for marker in [
        STRUCTURAL_GUIDE_ASSET_NAME,
        STRUCTURAL_GUIDE_MATERIAL_NAME,
        STRUCTURAL_GUIDE_TEXTURE_PATH,
        "SHAR_Export_Root",
        STRUCTURAL_GUIDE_UV_NAMES[0],
        STRUCTURAL_GUIDE_UV_NAMES[1],
        STRUCTURAL_GUIDE_UV_NAMES[2],
        STRUCTURAL_GUIDE_UV_NAMES[3],
    ] {
        if !contains_bytes(
            bytes,
            marker.as_bytes(),
        ) {
            return Err(
                PipelineError::new(
                    format!("structural-guide FBX marker is missing: {marker}"),
                ),
            );
        }
    }
    if contains_bytes(
        bytes,
        b"AnimationStack",
    ) || contains_bytes(
        bytes,
        b"NodeAttribute",
    ) || contains_bytes(
        bytes,
        b"Deformer",
    ) || contains_bytes(
        bytes,
        b"\x89PNG\r\n\x1a\n",
    ) {
        return Err(
            PipelineError::new(
                "structural-guide FBX contains a forbidden helper, runtime \
                 object, or embedded image",
            ),
        );
    }
    Ok(())
}

fn validate_atlas(bytes: &[u8]) -> Result<(), PipelineError> {
    const SIGNATURE: &[u8] = b"\x89PNG\r\n\x1a\n";
    if !bytes.starts_with(SIGNATURE) || bytes.len() < 29 {
        return Err(PipelineError::new("structural-guide atlas is not a PNG"));
    }
    let width = u32::from_be_bytes(
        bytes[16..20]
            .try_into()
            .map_err(
                |error: std::array::TryFromSliceError| {
                    PipelineError::new(error.to_string())
                },
            )?,
    );
    let height = u32::from_be_bytes(
        bytes[20..24]
            .try_into()
            .map_err(
                |error: std::array::TryFromSliceError| {
                    PipelineError::new(error.to_string())
                },
            )?,
    );
    if width != atlas::ATLAS_SIZE || height != atlas::ATLAS_SIZE {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide atlas dimensions changed: \
                     {width}x{height}"
                ),
            ),
        );
    }
    if bytes[24] != 8 || bytes[25] != 2 {
        return Err(
            PipelineError::new(
                format!(
                    "structural-guide atlas must be RGB8 without alpha: {}:{}",
                    bytes[24], bytes[25]
                ),
            ),
        );
    }
    Ok(())
}

fn validate_layout(value: &serde_json::Value) -> Result<(), PipelineError> {
    let width = value
        .get("atlasWidth")
        .and_then(serde_json::Value::as_u64);
    let height = value
        .get("atlasHeight")
        .and_then(serde_json::Value::as_u64);
    let padding = value
        .get("paddingPixels")
        .and_then(serde_json::Value::as_u64);
    let rotation = value
        .get("rotationAllowed")
        .and_then(serde_json::Value::as_bool);
    let entries = value
        .get("entries")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(
            || PipelineError::new("structural-guide atlas entries are missing"),
        )?;
    if width != Some(u64::from(atlas::ATLAS_SIZE))
        || height != Some(u64::from(atlas::ATLAS_SIZE))
        || padding != Some(u64::from(atlas::ATLAS_PADDING))
        || rotation != Some(false)
        || entries.is_empty()
    {
        return Err(
            PipelineError::new(
                "structural-guide atlas layout contract changed",
            ),
        );
    }
    let mut previous = None::<(
        &str,
        &str,
    )>;
    for entry in entries {
        let source = entry
            .get("sourceSha256")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(
                || PipelineError::new("atlas source hash is missing"),
            )?;
        let variant = entry
            .get("variantSha256")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(
                || PipelineError::new("atlas variant hash is missing"),
            )?;
        if source.len() != 64 || variant.len() != 64 {
            return Err(PipelineError::new("atlas hash length changed"));
        }
        if let Some((previous_source, previous_variant)) = previous
            && (
                source, variant,
            ) < (
                previous_source,
                previous_variant,
            )
        {
            return Err(
                PipelineError::new(
                    "atlas entries are not ordered by source SHA-256",
                ),
            );
        }
        previous = Some(
            (
                source, variant,
            ),
        );
    }
    Ok(())
}

fn validate_manifest(value: &serde_json::Value) -> Result<(), PipelineError> {
    let groups_without_normals = value
        .pointer("/sourceCoverage/groupsWithoutNormals")
        .and_then(serde_json::Value::as_u64);
    let normal_layer_included = value
        .pointer("/mesh/normalLayerIncluded")
        .and_then(serde_json::Value::as_bool);
    let normal_contract_changed = match (
        groups_without_normals,
        normal_layer_included,
    ) {
        (Some(groups), Some(included)) => included != (groups == 0),
        _ => true,
    };
    let contract_changed = normal_contract_changed
        || value
            .get("schemaVersion")
            .and_then(serde_json::Value::as_u64)
            != Some(4)
        || value
            .get("assetName")
            .and_then(serde_json::Value::as_str)
            != Some(STRUCTURAL_GUIDE_ASSET_NAME)
        || value
            .get("fbxVersion")
            .and_then(serde_json::Value::as_str)
            != Some("7.7")
        || value
            .get("sourceGeometryPolicy")
            .and_then(serde_json::Value::as_str)
            != Some(manifest::SOURCE_GEOMETRY_POLICY)
        || [
            "/spatialChanges/centered",
            "/spatialChanges/mirroredByGuide",
            "/spatialChanges/scaledByGuide",
            "/spatialChanges/heightAdjustedByGuide",
            "/spatialChanges/triangleDeduplication",
            "/spatialChanges/normalsRepaired",
            "/spatialChanges/guideOnlyGeometryAdded",
        ]
        .iter()
        .any(
            |pointer| {
                value
                    .pointer(pointer)
                    .and_then(serde_json::Value::as_bool)
                    != Some(false)
            },
        )
        || value
            .pointer("/worldFbxScene/units")
            .and_then(serde_json::Value::as_str)
            != Some("meters")
        || value
            .pointer("/worldFbxScene/unitScaleFactor")
            .and_then(serde_json::Value::as_f64)
            != Some(100.0)
        || value
            .pointer("/worldFbxScene/upAxis")
            .and_then(serde_json::Value::as_str)
            != Some("Y")
        || value
            .pointer("/worldFbxScene/frontAxis")
            .and_then(serde_json::Value::as_str)
            != Some("Z")
        || value
            .pointer("/worldFbxScene/coordinateAxis")
            .and_then(serde_json::Value::as_str)
            != Some("X")
        || value
            .pointer("/worldFbxScene/guideExportRootPolicy")
            .and_then(serde_json::Value::as_str)
            != Some("ReflectX")
        || value
            .pointer("/worldFbxScene/guideExportRootRotationDegrees/1")
            .and_then(serde_json::Value::as_f64)
            != Some(0.0)
        || value
            .pointer("/worldFbxScene/guideExportRootScale/0")
            .and_then(serde_json::Value::as_f64)
            != Some(-1.0)
        || value
            .pointer("/worldFbxScene/exteriorExportRootPolicy")
            .and_then(serde_json::Value::as_str)
            != Some("ReflectX")
        || value
            .pointer("/worldFbxScene/interiorExportRootPolicy")
            .and_then(serde_json::Value::as_str)
            != Some("ReflectX")
        || value
            .pointer("/worldFbxScene/worldReflectionAxis")
            .and_then(serde_json::Value::as_str)
            != Some("X")
        || value
            .pointer("/worldFbxScene/sourceRootsFlattenedIntoGuideMesh")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || value
            .pointer("/unrealImport/forceFrontXAxis")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || value
            .pointer("/worldHeight/meters")
            .and_then(serde_json::Value::as_f64)
            != Some(80.0)
        || value
            .pointer("/worldHeight/ownedByNormalWorldFbx")
            .and_then(serde_json::Value::as_bool)
            != Some(true)
        || value
            .pointer("/mesh/objectCount")
            .and_then(serde_json::Value::as_u64)
            != Some(1)
        || value
            .pointer("/mesh/materialSlotCount")
            .and_then(serde_json::Value::as_u64)
            != Some(1)
        || value
            .pointer("/mesh/materialName")
            .and_then(serde_json::Value::as_str)
            != Some(STRUCTURAL_GUIDE_MATERIAL_NAME)
        || value
            .pointer("/atlas/path")
            .and_then(serde_json::Value::as_str)
            != Some(STRUCTURAL_GUIDE_TEXTURE_PATH)
        || value
            .pointer("/atlas/alpha")
            .and_then(serde_json::Value::as_bool)
            != Some(false)
        || value
            .pointer("/uvChannels/0")
            .and_then(serde_json::Value::as_str)
            != Some("finalAtlasUV")
        || value
            .pointer("/uvChannels/1")
            .and_then(serde_json::Value::as_str)
            != Some("sourceUV")
        || value
            .pointer("/uvChannels/2")
            .and_then(serde_json::Value::as_str)
            != Some("atlasOffset")
        || value
            .pointer("/uvChannels/3")
            .and_then(serde_json::Value::as_str)
            != Some("atlasScale");
    if contract_changed {
        return Err(
            PipelineError::new("structural-guide manifest contract changed"),
        );
    }
    for pointer in [
        "/hashes/fbxSha256",
        "/hashes/atlasSha256",
        "/hashes/layoutSha256",
    ] {
        let hash = value
            .pointer(pointer)
            .and_then(serde_json::Value::as_str)
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "structural-guide manifest hash is missing: \
                             {pointer}"
                        ),
                    )
                },
            )?;
        if hash.len() != 64 {
            return Err(
                PipelineError::new(
                    format!(
                        "structural-guide manifest hash length changed: \
                         {pointer}"
                    ),
                ),
            );
        }
    }
    Ok(())
}

fn publication_inventory(
    staging: &Path,
) -> Result<
    (
        usize,
        u64,
    ),
    PipelineError,
> {
    let mut pending = vec![PathBuf::from(staging)];
    let mut files = 0_usize;
    let mut bytes = 0_u64;
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "structural-guide inventory failed: {}:{error}",
                        directory.display()
                    ),
                )
            },
        )? {
            let entry =
                entry.map_err(|error| PipelineError::new(error.to_string()))?;
            let metadata = entry
                .metadata()
                .map_err(|error| PipelineError::new(error.to_string()))?;
            if metadata.is_dir() {
                pending.push(entry.path());
            } else if metadata.is_file() {
                files = files
                    .checked_add(1)
                    .ok_or_else(
                        || PipelineError::new("guide file count overflowed"),
                    )?;
                bytes = bytes
                    .checked_add(metadata.len())
                    .ok_or_else(
                        || PipelineError::new("guide byte count overflowed"),
                    )?;
            } else {
                return Err(
                    PipelineError::new(
                        "structural-guide publication contains a non-file \
                         entry",
                    ),
                );
            }
        }
    }
    Ok(
        (
            files, bytes,
        ),
    )
}

fn contains_bytes(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

#[cfg(test)]
mod tests {
    use fbx::adapters::driven::binary_structural_guide_writer as fbx_guide;

    use super::model::GuideSourceCounts;
    use super::{manifest, validate_manifest};

    #[test]
    fn rendered_manifest_satisfies_publication_validator() -> Result<(), String>
    {
        let bytes = manifest::render(
            fbx_guide::StructuralGuideFbxSummary {
                vertices: 3,
                triangles: 1,
                bounds_min_meters: [
                    -1.0, 79.0, -2.0,
                ],
                bounds_max_meters: [
                    1.0, 81.0, 2.0,
                ],
            },
            GuideSourceCounts {
                input_meshes: 2,
                input_groups: 2,
                groups_without_normals: 0,
                input_triangles: 1,
                removed_duplicate_triangles: 0,
                removed_degenerate_triangles: 0,
                repaired_normal_triangles: 0,
                wasp_meshes: 0,
                prop_like_meshes: 0,
                approximated_vertex_color_triangles: 0,
            },
            &"a".repeat(64),
            &"b".repeat(64),
            &"c".repeat(64),
        )
        .map_err(|error| error.to_string())?;
        let value: serde_json::Value = serde_json::from_slice(&bytes)
            .map_err(|error| error.to_string())?;
        assert_eq!(
            value
                .pointer("/worldFbxScene/guideExportRootPolicy")
                .and_then(serde_json::Value::as_str),
            Some("ReflectX"),
        );
        assert_eq!(
            value
                .pointer("/worldFbxScene/guideExportRootScale/0")
                .and_then(serde_json::Value::as_f64),
            Some(-1.0),
        );
        assert_eq!(
            value
                .pointer("/worldFbxScene/exteriorExportRootPolicy")
                .and_then(serde_json::Value::as_str),
            Some("ReflectX"),
        );
        assert_eq!(
            value
                .pointer("/worldFbxScene/interiorExportRootPolicy")
                .and_then(serde_json::Value::as_str),
            Some("ReflectX"),
        );
        assert_eq!(
            value
                .pointer("/worldFbxScene/worldReflectionAxis")
                .and_then(serde_json::Value::as_str),
            Some("X"),
        );
        assert_eq!(
            value
                .pointer("/worldFbxScene/sourceRootsFlattenedIntoGuideMesh")
                .and_then(serde_json::Value::as_bool),
            Some(false),
        );
        validate_manifest(&value).map_err(|error| error.to_string())
    }
}
