// File:
//   - package.rs
// Path:
//   - src/fbx/src/adapters/driven/semantic_character_texture/package.rs
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
//   - Complete prepared-character material bindings and explicit extra PNGs.
// - Must-Not:
//   - Discover packages, infer missing textures, change geometry, or publish.
// - Allows:
//   - Exact group-address ownership, strict material coverage, and PNG
//   - normalization.
// - Split-When:
//   - Extra-texture normalization and material coverage diverge independently.
// - Merge-When:
//   - Another adapter owns the same prepared-package binding transaction.
// - Summary:
//   - Prepared semantic character package binding assembly.
// - Description:
//   - Maps every shader to a generated or explicit external PNG and fails
//   - closed.
// - Usage:
//   - Called after semantic body and eye analysis succeeds.
// - Defaults:
//   - Body and eye outputs use fixed portable identities below `textures/`.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Strict prepared-character material and extra-texture assembly.
use std::collections::{BTreeMap, BTreeSet};

use schoenwald_filesystem::adapters::driving::local::read_bytes;

use super::request::SemanticTextureRequest;
use super::{ExternalTextureArtifact, SemanticTextureArtifactError};
use crate::adapters::driven::semantic_texture_png::{
    decode_png_bytes, encode_png_bytes,
};
use crate::domain::character::CharacterAsset;
use crate::domain::texture::MaterialBinding;
use crate::domain::texture::semantic::GroupAddress;

/// Complete package binding result.
pub(super) struct PackageBindings {
    pub(super) materials: Vec<MaterialBinding>,
    pub(super) extra_textures: Vec<ExternalTextureArtifact>,
}

/// Build exact external material bindings for every primitive-group shader.
pub(super) fn build(
    request: &SemanticTextureRequest,
    character: &CharacterAsset,
) -> Result<PackageBindings, SemanticTextureArtifactError> {
    let body_groups = request
        .body_groups
        .iter()
        .copied()
        .map(Into::into)
        .collect::<BTreeSet<GroupAddress>>();
    let eye_group: GroupAddress = request
        .eye_group
        .into();
    if body_groups.contains(&eye_group) {
        return Err(package_error("eye group also appears in body groups"));
    }
    let mut bindings = BTreeMap::<String, String>::new();
    for address in &body_groups {
        bind_group(
            character,
            *address,
            "body-atlas.png",
            &mut bindings,
        )?;
    }
    bind_group(
        character,
        eye_group,
        "eye.png",
        &mut bindings,
    )?;
    let extra_textures = assemble_extra_materials(
        request,
        &mut bindings,
    )?;
    validate_group_coverage(
        character,
        &body_groups,
        eye_group,
        &bindings,
    )?;
    let materials = bindings
        .into_iter()
        .map(
            |(material, texture)| {
                MaterialBinding::new(
                    material,
                    Some(texture),
                )
                .map_err(
                    |error| {
                        package_error(format!("material failed: {error:?}"))
                    },
                )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    Ok(
        PackageBindings {
            materials,
            extra_textures: extra_textures
                .into_values()
                .collect(),
        },
    )
}

/// Decode and bind every explicit extra-material texture.
fn assemble_extra_materials(
    request: &SemanticTextureRequest,
    bindings: &mut BTreeMap<String, String>,
) -> Result<
    BTreeMap<String, ExternalTextureArtifact>,
    SemanticTextureArtifactError,
> {
    let mut textures = BTreeMap::new();
    for extra in &request.extra_materials {
        if bindings.contains_key(&extra.material_name) {
            return Err(
                package_error(
                    format!(
                        "extra material shadows generated binding: {}",
                        extra.material_name
                    ),
                ),
            );
        }
        let binding = MaterialBinding::new(
            extra
                .material_name
                .clone(),
            Some(
                extra
                    .output_file_name
                    .clone(),
            ),
        )
        .map_err(
            |error| package_error(format!("invalid extra material: {error:?}")),
        )?;
        let bytes = read_bytes(&extra.texture_path).map_err(
            |error| {
                package_error(
                    format!(
                        "extra texture read failed for {}: {error}",
                        extra.output_file_name
                    ),
                )
            },
        )?;
        let image = decode_png_bytes(&bytes).map_err(
            |error| {
                package_error(
                    format!(
                        "extra texture decode failed for {}: {error:?}",
                        extra.output_file_name
                    ),
                )
            },
        )?;
        let png = encode_png_bytes(&image).map_err(
            |error| {
                package_error(
                    format!(
                        "extra texture encode failed for {}: {error:?}",
                        extra.output_file_name
                    ),
                )
            },
        )?;
        if textures
            .insert(
                extra
                    .output_file_name
                    .clone(),
                ExternalTextureArtifact {
                    file_name: extra
                        .output_file_name
                        .clone(),
                    png,
                },
            )
            .is_some()
        {
            return Err(
                package_error(
                    format!(
                        "duplicate extra texture output: {}",
                        extra.output_file_name
                    ),
                ),
            );
        }
        bindings.insert(
            binding.material_name,
            extra
                .output_file_name
                .clone(),
        );
    }
    Ok(textures)
}

/// Bind one selected group shader to one generated texture identity.
fn bind_group(
    character: &CharacterAsset,
    address: GroupAddress,
    texture: &str,
    bindings: &mut BTreeMap<String, String>,
) -> Result<(), SemanticTextureArtifactError> {
    let shader = group_shader(
        character, address,
    )?;
    if let Some(existing) = bindings.insert(
        shader.clone(),
        texture.to_owned(),
    ) && existing != texture
    {
        return Err(
            package_error(
                format!(
                    "shader {shader} maps to both {existing} and {texture}"
                ),
            ),
        );
    }
    Ok(())
}

/// Require every group to be selected or explicitly bound exactly once.
fn validate_group_coverage(
    character: &CharacterAsset,
    body_groups: &BTreeSet<GroupAddress>,
    eye_group: GroupAddress,
    bindings: &BTreeMap<String, String>,
) -> Result<(), SemanticTextureArtifactError> {
    let body_shaders = body_groups
        .iter()
        .map(
            |address| {
                group_shader(
                    character, *address,
                )
            },
        )
        .collect::<Result<BTreeSet<_>, _>>()?;
    let eye_shader = group_shader(
        character, eye_group,
    )?;
    let mut used = BTreeSet::new();
    for (part_index, part) in character
        .parts
        .iter()
        .enumerate()
    {
        for (group_index, group) in part
            .mesh
            .groups
            .iter()
            .enumerate()
        {
            let address = GroupAddress {
                part_index,
                group_index,
            };
            if body_shaders.contains(&group.shader)
                && !body_groups.contains(&address)
            {
                return Err(
                    package_error(
                        format!(
                            "body shader group was not selected: \
                             part={part_index}, group={group_index}"
                        ),
                    ),
                );
            }
            if group.shader == eye_shader && address != eye_group {
                return Err(
                    package_error(
                        format!(
                            "eye shader group was not selected: \
                             part={part_index}, group={group_index}"
                        ),
                    ),
                );
            }
            if !bindings.contains_key(&group.shader) {
                return Err(
                    package_error(
                        format!(
                            "missing explicit texture binding for shader {}",
                            group.shader
                        ),
                    ),
                );
            }
            used.insert(
                group
                    .shader
                    .clone(),
            );
        }
    }
    if let Some(unused) = bindings
        .keys()
        .find(|shader| !used.contains(*shader))
    {
        return Err(
            package_error(
                format!(
                    "texture binding is not used by the character: {unused}"
                ),
            ),
        );
    }
    Ok(())
}

/// Resolve one exact group shader.
fn group_shader(
    character: &CharacterAsset,
    address: GroupAddress,
) -> Result<String, SemanticTextureArtifactError> {
    character
        .parts
        .get(address.part_index)
        .and_then(
            |part| {
                part.mesh
                    .groups
                    .get(address.group_index)
            },
        )
        .map(
            |group| {
                group
                    .shader
                    .clone()
            },
        )
        .ok_or_else(
            || {
                package_error(
                    format!(
                        "group is missing: part={}, group={}",
                        address.part_index, address.group_index
                    ),
                )
            },
        )
}

/// Construct one package-level failure.
fn package_error(message: impl Into<String>) -> SemanticTextureArtifactError {
    SemanticTextureArtifactError::Package(message.into())
}
