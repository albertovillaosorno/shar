// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Material outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Material outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Material outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource,
};
use fbx::domain::character::CharacterAsset;
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::texture::{MaterialBinding, MaterialSemantics};
use fbx::ports::component_source::ComponentSource as _;
use shar_sha256::digest_hex;

use super::prepared::PreparedTexture;
use super::texture_authority::SharedTextureAuthority;
use crate::domain::package::{PackageRole, PhaseThreePackageRow};
use crate::domain::PipelineError;

/// Exact normalized mesh occurrence aligned with one loaded world mesh.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WorldMeshSourceCoordinate<'source> {
    /// Normalized mesh member id without family or extension.
    pub(super) member_id: &'source str,
    /// Exact package-level mesh source chunk ordinal.
    pub(super) source_ordinal: usize,
}

/// Non-selecting provenance for every primitive-group consumer of one shader.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ShaderConsumerProvenance {
    /// Exact package-level primitive-group source chunk ordinals.
    source_ordinals: BTreeSet<usize>,
    /// Stable phase-three model members owning those primitive groups.
    model_member_ids: BTreeSet<String>,
}

/// Canonicalize static mesh shaders and return deduplicated bindings/payloads.
pub(super) fn canonicalize_static_materials(
    meshes: &mut [MeshAsset],
    package_root: &Path,
    scratch: &Path,
) -> Result<(Vec<MaterialBinding>, Vec<PreparedTexture>), PipelineError> {
    canonicalize_static_materials_with_authority(
        meshes,
        package_root,
        scratch,
        None,
        None,
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
    package: Option<&PhaseThreePackageRow>,
    mesh_sources: Option<&[WorldMeshSourceCoordinate<'_>]>,
    source_subcategory: &str,
) -> Result<(Vec<MaterialBinding>, Vec<PreparedTexture>), PipelineError> {
    canonicalize_static_materials_with_authority(
        meshes,
        package_root,
        scratch,
        Some(authority),
        package,
        mesh_sources,
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
    package: Option<&PhaseThreePackageRow>,
    mesh_sources: Option<&[WorldMeshSourceCoordinate<'_>]>,
    source_subcategory: &str,
) -> Result<(Vec<MaterialBinding>, Vec<PreparedTexture>), PipelineError> {
    let shader_sources =
        shader_consumer_provenance(meshes, mesh_sources, package);
    let shaders = shader_sources.keys().cloned().collect::<BTreeSet<_>>();
    let (renames, materials, textures) = resolve_materials(
        shaders,
        &shader_sources,
        package_root,
        scratch,
        authority,
        package,
        source_subcategory,
    )?;
    for group in meshes.iter_mut().flat_map(|mesh| mesh.groups.iter_mut()) {
        group.shader = renames
            .get(&group.shader)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "prop material rename is missing for {}",
                    group.shader
                ))
            })?
            .clone();
    }
    Ok((materials, textures))
}

/// Canonicalize rigid-animated mesh shaders and return bindings/payloads.
pub(super) fn canonicalize_animated_materials(
    asset: &mut CharacterAsset,
    package_root: &Path,
    scratch: &Path,
) -> Result<(Vec<MaterialBinding>, Vec<PreparedTexture>), PipelineError> {
    canonicalize_animated_materials_with_authority(
        asset,
        package_root,
        scratch,
        None,
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
    package: Option<&PhaseThreePackageRow>,
    source_subcategory: &str,
) -> Result<(Vec<MaterialBinding>, Vec<PreparedTexture>), PipelineError> {
    canonicalize_animated_materials_with_authority(
        asset,
        package_root,
        scratch,
        Some(authority),
        package,
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
    package: Option<&PhaseThreePackageRow>,
    source_subcategory: &str,
) -> Result<(Vec<MaterialBinding>, Vec<PreparedTexture>), PipelineError> {
    let shader_sources = shader_consumer_provenance_from_groups(
        asset.parts.iter().flat_map(|part| part.mesh.groups.iter()),
    );
    let shaders = shader_sources.keys().cloned().collect::<BTreeSet<_>>();
    let (renames, materials, textures) = resolve_materials(
        shaders,
        &shader_sources,
        package_root,
        scratch,
        authority,
        package,
        source_subcategory,
    )?;
    for group in asset
        .parts
        .iter_mut()
        .flat_map(|part| part.mesh.groups.iter_mut())
    {
        group.shader = renames
            .get(&group.shader)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "prop material rename is missing for {}",
                    group.shader
                ))
            })?
            .clone();
    }
    Ok((materials, textures))
}

/// Resolve one shader locally or through the scoped shared authority.
///
/// # Errors
///
/// Returns an error when shader evidence or fallback texture scope is invalid.
fn resolve_source_material(
    source: &DecodedComponentSource,
    shader: &str,
    consumer_provenance: Option<&ShaderConsumerProvenance>,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
    package: Option<&PhaseThreePackageRow>,
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
                .ok_or_else(|| {
                    PipelineError::new("shared texture authority is missing")
                })?
                .resolve(&texture, source_subcategory)?
                .ok_or_else(|| {
                    PipelineError::new(format!(
                        concat!(
                            "prop material {} has no scoped texture ",
                            "authority for {}; local search was {}"
                        ),
                        shader, texture, searched
                    ))
                })?;
            let file_name = external
                .file_name()
                .and_then(|value| value.to_str())
                .ok_or_else(|| {
                    PipelineError::new(format!(
                        "shared prop texture has no UTF-8 file name: \
                                 {}",
                        external.display()
                    ))
                })?
                .to_owned();
            let _copied_bytes = fs::copy(external, scratch.join(&file_name))
                .map_err(|error| {
                    PipelineError::new(format!(
                        "shared prop texture staging failed for {}: \
                             {error}",
                        external.display()
                    ))
                })?;
            MaterialBinding::new(material_name, Some(file_name)).map_err(
                |error| {
                    PipelineError::new(format!(
                        "shared prop material failed: {error:?}"
                    ))
                },
            )
        },
        Err(DecodedComponentError::Read { path, source: _source })
            if authority.is_some()
                && is_world_analysis_default_shader(shader)
                && !Path::new(&path).is_file() =>
        {
            MaterialBinding::new(shader, None).map_err(|error| {
                PipelineError::new(format!(
                    "world default material fallback failed: {error:?}"
                ))
            })
        },
        Err(error) => Err(material_resolution_error(
            shader,
            consumer_provenance,
            &error,
            package,
        )),
    }
}

/// Collect exact primitive-group provenance without an owning-mesh coordinate.
fn shader_consumer_provenance_from_groups<'group>(
    groups: impl Iterator<Item = &'group PrimitiveGroup>,
) -> BTreeMap<String, ShaderConsumerProvenance> {
    let mut sources = BTreeMap::<String, ShaderConsumerProvenance>::new();
    for group in groups {
        let provenance = sources.entry(group.shader.clone()).or_default();
        if let Some(source_ordinal) = group.source_ordinal {
            let _inserted = provenance.source_ordinals.insert(source_ordinal);
        }
    }
    sources
}

/// Collect exact primitive-group and owning-mesh provenance by logical shader.
fn shader_consumer_provenance(
    meshes: &[MeshAsset],
    mesh_sources: Option<&[WorldMeshSourceCoordinate<'_>]>,
    package: Option<&PhaseThreePackageRow>,
) -> BTreeMap<String, ShaderConsumerProvenance> {
    let mut sources = BTreeMap::<String, ShaderConsumerProvenance>::new();
    for (mesh_index, mesh) in meshes.iter().enumerate() {
        let model_member_id = mesh_sources
            .and_then(|coordinates| coordinates.get(mesh_index))
            .and_then(|coordinate| {
                package.and_then(|package| {
                    model_package_member_id(package, *coordinate)
                })
            });
        for group in &mesh.groups {
            let provenance = sources.entry(group.shader.clone()).or_default();
            if let Some(source_ordinal) = group.source_ordinal {
                let _inserted =
                    provenance.source_ordinals.insert(source_ordinal);
            }
            if let Some(model_member_id) = &model_member_id {
                let _inserted =
                    provenance.model_member_ids.insert(model_member_id.clone());
            }
        }
    }
    sources
}

/// Resolve one exact world mesh coordinate to its stable phase-three model id.
fn model_package_member_id(
    package: &PhaseThreePackageRow,
    coordinate: WorldMeshSourceCoordinate<'_>,
) -> Option<String> {
    let path = format!(
        "{}/components/mesh/{}.json",
        package.package_root, coordinate.member_id
    );
    package
        .find_member_by_source_coordinate(&path, coordinate.source_ordinal)
        .filter(|member| {
            member.role == PackageRole::Model
                && member.kind == "p3d-mesh"
                && member.source_chunk_kind == "mesh"
        })
        .map(|member| member.id.clone())
}

/// Preserve exact consumer coordinates when a logical shader is ambiguous.
fn material_resolution_error(
    shader: &str,
    consumer_provenance: Option<&ShaderConsumerProvenance>,
    error: &DecodedComponentError,
    package: Option<&PhaseThreePackageRow>,
) -> PipelineError {
    if matches!(error, DecodedComponentError::AmbiguousShaderMember { .. })
        && let Some(provenance) = consumer_provenance
        && !provenance.source_ordinals.is_empty()
    {
        let ordinals = &provenance.source_ordinals;
        if let Some(member_ids) = package.and_then(|package| {
            ambiguous_shader_package_member_ids(package, error)
        }) {
            if !provenance.model_member_ids.is_empty() {
                return PipelineError::new(format!(
                    concat!(
                        "prop material {} used by primitive-group source ",
                        "ordinals {:?} in phase-three model members {:?} ",
                        "failed: {:?}; phase-three material members {:?}"
                    ),
                    shader,
                    ordinals,
                    provenance.model_member_ids,
                    error,
                    member_ids
                ));
            }
            return PipelineError::new(format!(
                concat!(
                    "prop material {} used by primitive-group source ",
                    "ordinals {:?} failed: {:?}; phase-three material ",
                    "members {:?}"
                ),
                shader, ordinals, error, member_ids
            ));
        }
        return PipelineError::new(format!(
            "prop material {shader} used by primitive-group source ordinals \
             {ordinals:?} failed: {error:?}"
        ));
    }
    PipelineError::new(format!("prop material {shader} failed: {error:?}"))
}

/// Resolve every ambiguous shader occurrence to a stable phase-three member.
fn ambiguous_shader_package_member_ids(
    package: &PhaseThreePackageRow,
    error: &DecodedComponentError,
) -> Option<Vec<String>> {
    let DecodedComponentError::AmbiguousShaderMember { occurrences, .. } = error
    else {
        return None;
    };
    let mut member_ids = Vec::with_capacity(occurrences.len());
    for occurrence in occurrences {
        let source_ordinal = occurrence.source_ordinal?;
        let path = format!(
            "{}/components/shader/{}",
            package.package_root, occurrence.member
        );
        let member = package
            .find_member_by_source_coordinate(&path, source_ordinal)
            .filter(|member| {
                member.role == PackageRole::Material
                    && member.kind == "p3d-shader"
                    && member.source_chunk_kind == "shader"
            })?;
        member_ids.push(member.id.clone());
    }
    Some(member_ids)
}

/// Return whether one missing shader has proven neutral analysis evidence.
fn is_world_analysis_default_shader(shader: &str) -> bool {
    matches!(
        shader.to_ascii_lowercase().as_str(),
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
    shader_sources: &BTreeMap<String, ShaderConsumerProvenance>,
    package_root: &Path,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
    package: Option<&PhaseThreePackageRow>,
    source_subcategory: &str,
) -> Result<MaterialPlan, PipelineError> {
    fs::create_dir_all(scratch).map_err(|error| {
        PipelineError::new(format!(
            "prop material scratch creation failed: {error}"
        ))
    })?;
    let source = DecodedComponentSource::new(package_root, scratch);
    let mut renames = BTreeMap::new();
    let mut bindings = BTreeMap::new();
    let mut textures = BTreeMap::new();
    for shader in shaders {
        let binding = resolve_source_material(
            &source,
            &shader,
            shader_sources.get(&shader),
            scratch,
            authority,
            package,
            source_subcategory,
        )?;
        let source_semantics = binding.semantics;
        let (canonical_material, canonical_texture) = match binding
            .texture_file_name
        {
            Some(source_name) => {
                let source_bytes = fs::read(scratch.join(&source_name))
                    .map_err(|error| {
                        PipelineError::new(format!(
                            "prop staged texture read failed for \
                                         {source_name}: {error}"
                        ))
                    })?;
                let prepared = prepare_source_texture(source_bytes);
                let digest = prepared.sha256.clone();
                let file_name = prepared.file_name.clone();
                let _published_texture =
                    textures.entry(file_name.clone()).or_insert(prepared);
                (
                    canonical_material_identity(
                        Some(&digest),
                        source_semantics,
                    ),
                    Some(file_name),
                )
            },
            None => (canonical_material_identity(None, source_semantics), None),
        };
        let _previous_rename =
            renames.insert(shader, canonical_material.clone());
        let material =
            MaterialBinding::new(canonical_material.clone(), canonical_texture)
                .map(|material| material.with_semantics(source_semantics))
                .map_err(|error| {
                    PipelineError::new(format!(
                        "canonical prop material failed: {error:?}"
                    ))
                })?;
        let _published_material =
            bindings.entry(canonical_material).or_insert(material);
    }
    Ok((
        renames,
        bindings.into_values().collect(),
        textures.into_values().collect(),
    ))
}


/// Preserve one recovered source texture and derive its content identity.
fn prepare_source_texture(bytes: Vec<u8>) -> PreparedTexture {
    let sha256 = digest_hex(&bytes);
    PreparedTexture {
        file_name: format!("texture-{sha256}.png"),
        bytes,
        sha256,
    }
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
        .map_or_else(|| base.clone(), |suffix| format!("{base}-{suffix}"))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/material/tests.rs"]
mod tests;
