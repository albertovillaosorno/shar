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
    /// Exact material bindings for every primitive-group shader.
    pub(super) materials: Vec<MaterialBinding>,
    /// Extra external PNG artifacts in stable file-name order.
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
    let eye_group = request
        .eye_group
        .map(Into::into);
    if eye_group.is_some_and(|address| body_groups.contains(&address)) {
        return Err(package_error("eye group also appears in body groups"));
    }
    let mut bindings = BTreeMap::<String, Option<String>>::new();
    for address in &body_groups {
        bind_group(
            character,
            *address,
            "body-atlas.png",
            &mut bindings,
        )?;
    }
    if let Some(address) = eye_group {
        bind_group(
            character,
            address,
            "eye.png",
            &mut bindings,
        )?;
    }
    let extra_textures = assemble_extra_materials(
        request,
        &mut bindings,
    )?;
    assemble_untextured_materials(
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
                    material, texture,
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
    bindings: &mut BTreeMap<String, Option<String>>,
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
        let artifact = ExternalTextureArtifact {
            file_name: extra
                .output_file_name
                .clone(),
            png,
        };
        if let Some(existing) = textures.get(&extra.output_file_name) {
            if existing != &artifact {
                return Err(
                    package_error(
                        format!(
                            "conflicting extra texture output: {}",
                            extra.output_file_name
                        ),
                    ),
                );
            }
        } else {
            let _previous = textures.insert(
                extra
                    .output_file_name
                    .clone(),
                artifact,
            );
        }
        let _previous = bindings.insert(
            binding.material_name,
            Some(
                extra
                    .output_file_name
                    .clone(),
            ),
        );
    }
    Ok(textures)
}

/// Bind explicit shader-only materials without inventing texture artifacts.
fn assemble_untextured_materials(
    request: &SemanticTextureRequest,
    bindings: &mut BTreeMap<String, Option<String>>,
) -> Result<(), SemanticTextureArtifactError> {
    for material_name in &request.untextured_materials {
        let binding = MaterialBinding::new(
            material_name.clone(),
            None,
        )
        .map_err(
            |error| {
                package_error(format!("invalid untextured material: {error:?}"))
            },
        )?;
        if bindings
            .insert(
                binding
                    .material_name
                    .clone(),
                None,
            )
            .is_some()
        {
            return Err(
                package_error(
                    format!(
                        "untextured material shadows another binding: {}",
                        binding.material_name
                    ),
                ),
            );
        }
    }
    Ok(())
}

/// Bind one selected group shader to one generated texture identity.
fn bind_group(
    character: &CharacterAsset,
    address: GroupAddress,
    texture: &str,
    bindings: &mut BTreeMap<String, Option<String>>,
) -> Result<(), SemanticTextureArtifactError> {
    let shader = group_shader(
        character, address,
    )?;
    let selected = Some(texture.to_owned());
    if let Some(existing) = bindings.insert(
        shader.clone(),
        selected.clone(),
    ) && existing != selected
    {
        return Err(
            package_error(
                format!("shader {shader} maps to conflicting texture policies"),
            ),
        );
    }
    Ok(())
}

/// Require every group to be selected or explicitly bound exactly once.
fn validate_group_coverage(
    character: &CharacterAsset,
    body_groups: &BTreeSet<GroupAddress>,
    eye_group: Option<GroupAddress>,
    bindings: &BTreeMap<String, Option<String>>,
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
    let eye_shader = eye_group
        .map(
            |address| {
                group_shader(
                    character, address,
                )
            },
        )
        .transpose()?;
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
            if eye_shader
                .as_ref()
                .is_some_and(
                    |shader| {
                        group.shader == *shader && Some(address) != eye_group
                    },
                )
            {
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
            let _inserted = used.insert(
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
