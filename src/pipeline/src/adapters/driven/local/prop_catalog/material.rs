// File:
//   - material.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/material.rs
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
//   - Canonical diffuse-material and external-texture preparation for prop
//     models.
// - Must-Not:
//   - Infer normal/specular roles, serialize FBX, or retain source file names.
// - Allows:
//   - Decoded shader resolution, texture hashing, and group shader renaming.
// - Split-When:
//   - Additional material channels gain proven typed source evidence.
// - Merge-When:
//   - A shared model material planner owns identical canonicalization rules.
// - Summary:
//   - Deduplicates materials and textures by the bytes represented in FBX.
// - Description:
//   - Converts source shader identities into content-derived portable names.
// - Usage:
//   - Applied to static and rigid-animated prepared prop geometry.
// - Defaults:
//   - Untextured groups share one neutral `material-none` binding.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md
//
// Large file:
//   - false
//

//! Canonical diffuse-material and texture preparation for prop models.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use fbx::adapters::driven::decoded_component_source::DecodedComponentSource;
use fbx::domain::character::CharacterAsset;
use fbx::domain::mesh::MeshAsset;
use fbx::domain::texture::MaterialBinding;
use fbx::ports::component_source::ComponentSource as _;
use shar_sha256::digest_hex;

use super::prepared::PreparedTexture;
use crate::domain::PipelineError;

/// Canonicalize static mesh shaders and return deduplicated bindings/payloads.
pub(super) fn canonicalize_static_materials(
    meshes: &mut [MeshAsset],
    package_root: &Path,
    scratch: &Path,
) -> Result<
    (
        Vec<MaterialBinding>,
        Vec<PreparedTexture>,
    ),
    PipelineError,
> {
    let shaders = meshes
        .iter()
        .flat_map(
            |mesh| {
                mesh.groups
                    .iter()
            },
        )
        .map(
            |group| {
                group
                    .shader
                    .clone()
            },
        )
        .collect::<BTreeSet<_>>();
    let (renames, materials, textures) = resolve_materials(
        shaders,
        package_root,
        scratch,
    )?;
    for group in meshes
        .iter_mut()
        .flat_map(
            |mesh| {
                mesh.groups
                    .iter_mut()
            },
        )
    {
        group.shader = renames
            .get(&group.shader)
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "prop material rename is missing for {}",
                            group.shader
                        ),
                    )
                },
            )?
            .clone();
    }
    Ok(
        (
            materials, textures,
        ),
    )
}

/// Canonicalize rigid-animated mesh shaders and return bindings/payloads.
pub(super) fn canonicalize_animated_materials(
    asset: &mut CharacterAsset,
    package_root: &Path,
    scratch: &Path,
) -> Result<
    (
        Vec<MaterialBinding>,
        Vec<PreparedTexture>,
    ),
    PipelineError,
> {
    let shaders = asset
        .parts
        .iter()
        .flat_map(
            |part| {
                part.mesh
                    .groups
                    .iter()
            },
        )
        .map(
            |group| {
                group
                    .shader
                    .clone()
            },
        )
        .collect::<BTreeSet<_>>();
    let (renames, materials, textures) = resolve_materials(
        shaders,
        package_root,
        scratch,
    )?;
    for group in asset
        .parts
        .iter_mut()
        .flat_map(
            |part| {
                part.mesh
                    .groups
                    .iter_mut()
            },
        )
    {
        group.shader = renames
            .get(&group.shader)
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "prop material rename is missing for {}",
                            group.shader
                        ),
                    )
                },
            )?
            .clone();
    }
    Ok(
        (
            materials, textures,
        ),
    )
}

/// Resolve source shaders and replace source names with content-derived names.
type MaterialPlan = (
    BTreeMap<String, String>,
    Vec<MaterialBinding>,
    Vec<PreparedTexture>,
);

fn resolve_materials(
    shaders: BTreeSet<String>,
    package_root: &Path,
    scratch: &Path,
) -> Result<MaterialPlan, PipelineError> {
    fs::create_dir_all(scratch).map_err(
        |error| {
            PipelineError::new(
                format!("prop material scratch creation failed: {error}"),
            )
        },
    )?;
    let source = DecodedComponentSource::new(
        package_root,
        scratch,
    );
    let mut renames = BTreeMap::new();
    let mut bindings = BTreeMap::new();
    let mut textures = BTreeMap::new();
    for shader in shaders {
        let binding = source
            .resolve_material(&shader)
            .map_err(
                |error| {
                    PipelineError::new(
                        format!("prop material {shader} failed: {error:?}"),
                    )
                },
            )?;
        let (canonical_material, canonical_texture) =
            match binding.texture_file_name {
                Some(source_name) => {
                    let bytes = fs::read(scratch.join(&source_name)).map_err(
                        |error| {
                            PipelineError::new(
                                format!(
                                    "prop staged texture read failed for \
                                     {source_name}: {error}"
                                ),
                            )
                        },
                    )?;
                    let digest = digest_hex(&bytes);
                    let file_name = format!("texture-{digest}.png");
                    textures
                        .entry(file_name.clone())
                        .or_insert(
                            PreparedTexture {
                                file_name: file_name.clone(),
                                bytes,
                                sha256: digest.clone(),
                            },
                        );
                    (
                        format!("material-{digest}"),
                        Some(file_name),
                    )
                }
                None => (
                    "material-none".to_owned(),
                    None,
                ),
            };
        renames.insert(
            shader,
            canonical_material.clone(),
        );
        let material = MaterialBinding::new(
            canonical_material.clone(),
            canonical_texture,
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!("canonical prop material failed: {error:?}"),
                )
            },
        )?;
        bindings
            .entry(canonical_material)
            .or_insert(material);
    }
    Ok(
        (
            renames,
            bindings
                .into_values()
                .collect(),
            textures
                .into_values()
                .collect(),
        ),
    )
}
