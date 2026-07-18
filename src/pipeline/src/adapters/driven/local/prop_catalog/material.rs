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

use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource,
};
use fbx::domain::character::CharacterAsset;
use fbx::domain::mesh::MeshAsset;
use fbx::domain::texture::{MaterialBinding, MaterialSemantics};
use fbx::ports::component_source::ComponentSource as _;
use shar_sha256::digest_hex;

use super::prepared::PreparedTexture;
use super::texture_authority::SharedTextureAuthority;
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
    canonicalize_static_materials_with_authority(
        meshes,
        package_root,
        scratch,
        None,
        "",
    )
}

/// Canonicalize world static materials with shared texture authority.
///
/// # Errors
///
/// Returns an error when local or shared material evidence is malformed.
pub(super) fn canonicalize_world_static_materials(
    meshes: &mut [MeshAsset],
    package_root: &Path,
    scratch: &Path,
    authority: &SharedTextureAuthority,
    source_subcategory: &str,
) -> Result<
    (
        Vec<MaterialBinding>,
        Vec<PreparedTexture>,
    ),
    PipelineError,
> {
    canonicalize_static_materials_with_authority(
        meshes,
        package_root,
        scratch,
        Some(authority),
        source_subcategory,
    )
}

/// Canonicalize static materials with optional shared texture fallback.
///
/// # Errors
///
/// Returns an error when shader resolution or canonical renaming fails.
fn canonicalize_static_materials_with_authority(
    meshes: &mut [MeshAsset],
    package_root: &Path,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
    source_subcategory: &str,
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
        authority,
        source_subcategory,
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
    canonicalize_animated_materials_with_authority(
        asset,
        package_root,
        scratch,
        None,
        "",
    )
}

/// Canonicalize world animated materials with shared texture authority.
///
/// # Errors
///
/// Returns an error when local or shared material evidence is malformed.
pub(super) fn canonicalize_world_animated_materials(
    asset: &mut CharacterAsset,
    package_root: &Path,
    scratch: &Path,
    authority: &SharedTextureAuthority,
    source_subcategory: &str,
) -> Result<
    (
        Vec<MaterialBinding>,
        Vec<PreparedTexture>,
    ),
    PipelineError,
> {
    canonicalize_animated_materials_with_authority(
        asset,
        package_root,
        scratch,
        Some(authority),
        source_subcategory,
    )
}

/// Canonicalize animated materials with optional shared texture fallback.
///
/// # Errors
///
/// Returns an error when shader resolution or canonical renaming fails.
fn canonicalize_animated_materials_with_authority(
    asset: &mut CharacterAsset,
    package_root: &Path,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
    source_subcategory: &str,
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
        authority,
        source_subcategory,
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

/// Resolve one shader locally or through the scoped shared authority.
///
/// # Errors
///
/// Returns an error when shader evidence or fallback texture scope is invalid.
fn resolve_source_material(
    source: &DecodedComponentSource,
    shader: &str,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
    source_subcategory: &str,
) -> Result<MaterialBinding, PipelineError> {
    match source.resolve_material(shader) {
        Ok(binding) => Ok(binding),
        Err(DecodedComponentError::MissingTexture {
            shader: material_name,
            texture,
            searched,
        }) if authority.is_some() => {
            let external = authority
                .ok_or_else(
                    || {
                        PipelineError::new(
                            "shared texture authority is missing",
                        )
                    },
                )?
                .resolve(
                    &texture,
                    source_subcategory,
                )?
                .ok_or_else(
                    || {
                        PipelineError::new(
                            format!(
                                concat!(
                                    "prop material {} has no scoped texture ",
                                    "authority for {}; local search was {}"
                                ),
                                shader, texture, searched
                            ),
                        )
                    },
                )?;
            let file_name = external
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or_else(
                    || {
                        PipelineError::new(
                            format!(
                                "shared prop texture has no UTF-8 file name: \
                                 {}",
                                external.display()
                            ),
                        )
                    },
                )?
                .to_owned();
            let _copied_bytes = fs::copy(
                external,
                scratch.join(&file_name),
            )
            .map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "shared prop texture staging failed for {}: \
                             {error}",
                            external.display()
                        ),
                    )
                },
            )?;
            MaterialBinding::new(
                material_name,
                Some(file_name),
            )
            .map_err(
                |error| {
                    PipelineError::new(
                        format!("shared prop material failed: {error:?}"),
                    )
                },
            )
        }
        Err(DecodedComponentError::Read {
            path,
            source: _source,
        }) if authority.is_some()
            && is_world_analysis_default_shader(shader)
            && !Path::new(&path).is_file() =>
        {
            MaterialBinding::new(
                shader, None,
            )
            .map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "world default material fallback failed: {error:?}"
                        ),
                    )
                },
            )
        }
        Err(error) => Err(
            PipelineError::new(
                format!("prop material {shader} failed: {error:?}"),
            ),
        ),
    }
}

/// Return whether one missing shader has proven neutral analysis evidence.
fn is_world_analysis_default_shader(shader: &str) -> bool {
    matches!(
        shader
            .to_ascii_lowercase()
            .as_str(),
        "lambert1" | "pure3dsimpleshader15"
    )
}

/// Resolve source shaders and replace source names with content-derived names.
type MaterialPlan = (
    BTreeMap<String, String>,
    Vec<MaterialBinding>,
    Vec<PreparedTexture>,
);

/// Resolve and content-canonicalize one complete shader identity set.
///
/// # Errors
///
/// Returns an error when material, texture, hashing, or staging work fails.
fn resolve_materials(
    shaders: BTreeSet<String>,
    package_root: &Path,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
    source_subcategory: &str,
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
        let binding = resolve_source_material(
            &source,
            &shader,
            scratch,
            authority,
            source_subcategory,
        )?;
        let source_semantics = binding.semantics;
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
                    let _published_texture = textures
                        .entry(file_name.clone())
                        .or_insert_with(
                            || PreparedTexture {
                                file_name: file_name.clone(),
                                bytes,
                                sha256: digest.clone(),
                            },
                        );
                    (
                        canonical_material_identity(
                            Some(&digest),
                            source_semantics,
                        ),
                        Some(file_name),
                    )
                }
                None => (
                    canonical_material_identity(
                        None,
                        source_semantics,
                    ),
                    None,
                ),
            };
        let _previous_rename = renames.insert(
            shader,
            canonical_material.clone(),
        );
        let material = MaterialBinding::new(
            canonical_material.clone(),
            canonical_texture,
        )
        .map(|material| material.with_semantics(source_semantics))
        .map_err(
            |error| {
                PipelineError::new(
                    format!("canonical prop material failed: {error:?}"),
                )
            },
        )?;
        let _published_material = bindings
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

/// Build one content-derived material identity without merging semantic
/// classes.
fn canonical_material_identity(
    texture_digest: Option<&str>,
    semantics: MaterialSemantics,
) -> String {
    let base = texture_digest.map_or_else(
        || "material-none".to_owned(),
        |digest| format!("material-{digest}"),
    );
    semantics
        .suffix()
        .map_or_else(
            || base.clone(),
            |suffix| format!("{base}-{suffix}"),
        )
}

#[cfg(test)]
mod tests {
    use fbx::domain::texture::MaterialSemantics;

    use super::{
        canonical_material_identity, is_world_analysis_default_shader,
    };

    #[test]
    fn recognizes_only_evidence_backed_neutral_defaults() {
        assert!(is_world_analysis_default_shader("lambert1"));
        assert!(is_world_analysis_default_shader("Pure3DSimpleShader15"));
        assert!(!is_world_analysis_default_shader("lambert"));
        assert!(!is_world_analysis_default_shader("pure3dSimpleShader14"));
        assert!(!is_world_analysis_default_shader("world_button_m"));
    }

    #[test]
    fn canonical_material_identity_separates_surface_semantics() {
        let opaque = canonical_material_identity(
            Some("abc123"),
            MaterialSemantics::default(),
        );
        let glass = canonical_material_identity(
            Some("abc123"),
            MaterialSemantics::default().with_glass(true),
        );
        let emitter = canonical_material_identity(
            Some("abc123"),
            MaterialSemantics::default()
                .with_transparent(true)
                .with_light_emitter(true),
        );
        assert_eq!(
            opaque,
            "material-abc123"
        );
        assert_eq!(
            glass,
            "material-abc123-glass"
        );
        assert_eq!(
            emitter,
            "material-abc123-transparent-light-emitter"
        );
        assert_ne!(
            opaque,
            glass
        );
        assert_ne!(
            glass,
            emitter
        );
    }
}
