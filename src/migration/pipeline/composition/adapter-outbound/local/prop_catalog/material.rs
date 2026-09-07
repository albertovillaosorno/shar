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
use std::path::{Path, PathBuf};

use fbx::adapters::driven::decoded_component_source::{
    DecodedComponentError, DecodedComponentSource, ShaderSourceEvidence,
    read_shader_source_evidence,
};
use fbx::domain::character::CharacterAsset;
use fbx::domain::mesh::{MeshAsset, PrimitiveGroup};
use fbx::domain::texture::{MaterialBinding, MaterialSemantics};
use fbx::ports::component_source::ComponentSource as _;
use serde_json::{Value, json};
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

/// One exact runtime-visible shader source contributing a presentation state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CanonicalShaderPresentationSource {
    /// Owning generated package identity when phase-three evidence exists.
    pub(super) package_id: Option<String>,
    /// Exact package-level source chunk ordinal when the ledger publishes it.
    pub(super) source_ordinal: Option<usize>,
    /// Normalized shader member file name when the ledger publishes it.
    pub(super) member: Option<String>,
    /// Complete validated decoded shader evidence.
    pub(super) evidence: ShaderSourceEvidence,
}

/// Exact source presentation retained beside one canonical material binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CanonicalMaterialPresentation {
    /// Canonical material identity referenced by FBX primitive groups.
    pub(super) material_name: String,
    /// SHA-256 over presentation plus the runtime-addressable shader identity.
    pub(super) binding_sha256: String,
    /// SHA-256 over texture content, decoded shader state, tint, and semantics.
    pub(super) presentation_sha256: String,
    /// Exact normalized texture payload SHA-256 when this material is textured.
    pub(super) texture_sha256: Option<String>,
    /// Runtime-visible occurrences of this independently addressable shader.
    pub(super) source_shaders: Vec<CanonicalShaderPresentationSource>,
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
    let plan = canonicalize_static_material_plan_with_authority(
        meshes,
        package_root,
        scratch,
        authority,
        package,
        mesh_sources,
        source_subcategory,
    )?;
    Ok((plan.materials, plan.textures))
}

/// Canonical world material outputs required by FBX and native projection.
pub(super) struct WorldStaticMaterialProjection {
    /// Canonical FBX material bindings.
    pub(super) materials: Vec<MaterialBinding>,
    /// Canonical normalized texture payloads.
    pub(super) textures: Vec<PreparedTexture>,
    /// Runtime-visible source shader evidence grouped by binding identity.
    pub(super) presentations: Vec<CanonicalMaterialPresentation>,
}

/// Canonicalize one world mesh batch and retain native projection evidence.
pub(super) fn canonicalize_world_static_materials_with_presentations(
    meshes: &mut [MeshAsset],
    package_root: &Path,
    scratch: &Path,
    authority: &SharedTextureAuthority,
    package: Option<&PhaseThreePackageRow>,
    mesh_sources: Option<&[WorldMeshSourceCoordinate<'_>]>,
    source_subcategory: &str,
) -> Result<WorldStaticMaterialProjection, PipelineError> {
    let plan = canonicalize_static_material_plan_with_authority(
        meshes,
        package_root,
        scratch,
        Some(authority),
        package,
        mesh_sources,
        source_subcategory,
    )?;
    Ok(WorldStaticMaterialProjection {
        materials: plan.materials,
        textures: plan.textures,
        presentations: plan.presentations,
    })
}

/// Build the complete static material plan before projecting wrapper outputs.
fn canonicalize_static_material_plan_with_authority(
    meshes: &mut [MeshAsset],
    package_root: &Path,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
    package: Option<&PhaseThreePackageRow>,
    mesh_sources: Option<&[WorldMeshSourceCoordinate<'_>]>,
    source_subcategory: &str,
) -> Result<MaterialPlanWithPresentation, PipelineError> {
    let shader_sources =
        shader_consumer_provenance(meshes, mesh_sources, package);
    let shaders = shader_sources.keys().cloned().collect::<BTreeSet<_>>();
    let plan = resolve_material_plan(
        shaders,
        &shader_sources,
        package_root,
        scratch,
        authority,
        package,
        source_subcategory,
    )?;
    for group in meshes.iter_mut().flat_map(|mesh| mesh.groups.iter_mut()) {
        group.shader = plan
            .renames
            .get(&group.shader)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "prop material rename is missing for {}",
                    group.shader
                ))
            })?
            .clone();
    }
    Ok(plan)
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
    package_root: &Path,
    shader: &str,
    consumer_provenance: Option<&ShaderConsumerProvenance>,
    authority: Option<&SharedTextureAuthority>,
    package: Option<&PhaseThreePackageRow>,
    source_subcategory: &str,
) -> Result<MaterialBinding, PipelineError> {
    match source.resolve_material(shader) {
        Ok(binding) => Ok(binding),
        Err(DecodedComponentError::MissingTexture {
            texture, searched, ..
        }) if authority.is_some() => {
            let external = authority
                .ok_or_else(|| {
                    PipelineError::new("shared texture authority is missing")
                })?
                .resolve_authoritative(&texture, source_subcategory)?
                .ok_or_else(|| {
                    PipelineError::new(format!(
                        concat!(
                            "prop material {} has no scoped texture ",
                            "authority for {}; local search was {}"
                        ),
                        shader, texture, searched
                    ))
                })?;
            source
                .resolve_material_with_authoritative_external_texture(
                    shader,
                    &external.logical,
                    external.path,
                )
                .map_err(|error| {
                    PipelineError::new(format!(
                        "shared prop material failed: {error:?}"
                    ))
                })
        },
        Err(DecodedComponentError::AmbiguousTextureMember { texture, .. }) => {
            resolve_runtime_first_texture_material(
                source,
                package_root,
                shader,
                &texture,
                consumer_provenance,
                package,
            )?
            .map_or_else(
                || {
                    Err(PipelineError::new(format!(
                        concat!(
                            "prop material {} has ambiguous local ",
                            "texture {}"
                        ),
                        shader, texture
                    )))
                },
                Ok,
            )
        },
        Err(DecodedComponentError::MissingShaderMember { .. })
            if authority.is_some()
                && runtime_missing_shader_has_package_consumers(
                    consumer_provenance,
                    package,
                ) =>
        {
            runtime_missing_shader_material()
        },
        Err(error @ DecodedComponentError::AmbiguousShaderMember { .. }) => {
            let context = RuntimeMaterialContext {
                package_root,
                consumer_provenance,
                authority,
                package,
                source_subcategory,
            };
            resolve_runtime_first_material(source, shader, context, &error)?
                .map_or_else(
                    || {
                        Err(material_resolution_error(
                            shader,
                            consumer_provenance,
                            &error,
                            package,
                        ))
                    },
                    Ok,
                )
        },
        Err(error) => Err(material_resolution_error(
            shader,
            consumer_provenance,
            &error,
            package,
        )),
    }
}

/// Require exact package evidence before applying the runtime missing-shader
/// fallback.
fn runtime_missing_shader_has_package_consumers(
    provenance: Option<&ShaderConsumerProvenance>,
    package: Option<&PhaseThreePackageRow>,
) -> bool {
    let (Some(provenance), Some(package)) = (provenance, package) else {
        return false;
    };
    if provenance.source_ordinals.is_empty()
        || provenance.model_member_ids.is_empty()
    {
        return false;
    }
    provenance.model_member_ids.iter().all(|member_id| {
        package.members().iter().any(|member| {
            member.id == *member_id
                && member.role == PackageRole::Model
                && member.kind == "p3d-mesh"
                && member.source_chunk_kind == "mesh"
                && member.source_chunk_ordinal.is_some()
        })
    })
}

/// Reproduce the shipped primitive-group fallback for a missing shader.
fn runtime_missing_shader_material() -> Result<MaterialBinding, PipelineError> {
    MaterialBinding::new("error", None).map_err(|error| {
        PipelineError::new(format!(
            "runtime missing-shader material failed: {error:?}"
        ))
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RuntimeLedgerOccurrence {
    member: String,
    source_ordinal: usize,
    path: PathBuf,
}

#[derive(Clone, Copy)]
struct RuntimeMaterialContext<'source> {
    package_root: &'source Path,
    consumer_provenance: Option<&'source ShaderConsumerProvenance>,
    authority: Option<&'source SharedTextureAuthority>,
    package: Option<&'source PhaseThreePackageRow>,
    source_subcategory: &'source str,
}

/// Resolve duplicate local textures for one uniquely identified shader.
fn resolve_runtime_first_texture_material(
    source: &DecodedComponentSource,
    package_root: &Path,
    shader: &str,
    texture: &str,
    consumer_provenance: Option<&ShaderConsumerProvenance>,
    package: Option<&PhaseThreePackageRow>,
) -> Result<Option<MaterialBinding>, PipelineError> {
    let (Some(provenance), Some(package)) =
        (consumer_provenance, package)
    else {
        return Ok(None);
    };
    let shaders = top_level_ledger_occurrences(package_root, "shader", shader)?;
    let [shader_occurrence] = shaders.as_slice() else {
        return Ok(None);
    };
    if !runtime_shader_package_member_matches(package, shader_occurrence)
        || !runtime_shader_precedes_consumers(
            shader_occurrence.source_ordinal,
            provenance,
            package,
        )
    {
        return Ok(None);
    }
    let Some(first_texture) = runtime_first_texture_occurrence(
        package_root,
        texture,
        shader_occurrence.source_ordinal,
    )? else {
        return Ok(None);
    };
    source
        .resolve_indexed_material_with_texture_occurrence(
            &shader_occurrence.path,
            &first_texture.path,
        )
        .map(Some)
        .map_err(|source| {
            PipelineError::new(format!(
                "runtime-first unique-shader texture failed: {source:?}"
            ))
        })
}

/// Resolve a duplicate shader exactly as the shipped inventory would.
fn resolve_runtime_first_material(
    source: &DecodedComponentSource,
    shader: &str,
    context: RuntimeMaterialContext<'_>,
    error: &DecodedComponentError,
) -> Result<Option<MaterialBinding>, PipelineError> {
    let Some(first_shader) = runtime_first_shader_occurrence(
        context.package_root,
        shader,
        context.consumer_provenance,
        context.package,
        error,
    )? else {
        return Ok(None);
    };
    let shader_evidence =
        read_shader_source_evidence(&first_shader.path, shader)
        .map_err(|source| {
            PipelineError::new(format!(
                "runtime-first shader evidence failed: {source:?}"
            ))
        })?;
    if let Some(texture) = shader_evidence.texture_reference.as_deref()
        && let Some(first_texture) = runtime_first_texture_occurrence(
            context.package_root,
            texture,
            first_shader.source_ordinal,
        )?
    {
        return source
            .resolve_indexed_material_with_texture_occurrence(
                &first_shader.path,
                &first_texture.path,
            )
            .map(Some)
            .map_err(|source| {
                PipelineError::new(format!(
                    "runtime-first prop texture failed: {source:?}"
                ))
            });
    }
    match source.resolve_indexed_material(&first_shader.path) {
        Ok(binding) => Ok(Some(binding)),
        Err(DecodedComponentError::MissingTexture {
            texture, searched, ..
        }) if context.authority.is_some() => {
            let external = context.authority
                .ok_or_else(|| {
                    PipelineError::new("shared texture authority is missing")
                })?
                .resolve_authoritative(&texture, context.source_subcategory)?
                .ok_or_else(|| {
                    PipelineError::new(format!(
                        concat!(
                            "runtime-first prop material {} has no scoped ",
                            "texture authority for {}; local search was {}"
                        ),
                        shader, texture, searched
                    ))
                })?;
            source
                .resolve_indexed_material_with_authoritative_external_texture(
                    &first_shader.path,
                    &external.logical,
                    external.path,
                )
                .map(Some)
                .map_err(|source| {
                    PipelineError::new(format!(
                        "runtime-first shared prop material failed: {source:?}"
                    ))
                })
        },
        Err(source) => Err(PipelineError::new(format!(
            "runtime-first prop material failed: {source:?}"
        ))),
    }
}

/// Select the first same-section shader only with complete source coordinates.
fn runtime_first_shader_occurrence(
    package_root: &Path,
    shader: &str,
    consumer_provenance: Option<&ShaderConsumerProvenance>,
    package: Option<&PhaseThreePackageRow>,
    error: &DecodedComponentError,
) -> Result<Option<RuntimeLedgerOccurrence>, PipelineError> {
    let DecodedComponentError::AmbiguousShaderMember { occurrences, .. } = error
    else {
        return Ok(None);
    };
    let (Some(provenance), Some(package)) =
        (consumer_provenance, package)
    else {
        return Ok(None);
    };
    if occurrences.len() < 2
        || ambiguous_shader_package_member_ids(package, error).is_none()
    {
        return Ok(None);
    }
    let ledger = top_level_ledger_occurrences(package_root, "shader", shader)?;
    let expected = occurrences
        .iter()
        .map(|occurrence| {
            occurrence
                .source_ordinal
                .map(|ordinal| (occurrence.member.clone(), ordinal))
        })
        .collect::<Option<BTreeSet<_>>>();
    let actual = ledger
        .iter()
        .map(|occurrence| {
            (occurrence.member.clone(), occurrence.source_ordinal)
        })
        .collect::<BTreeSet<_>>();
    if expected.as_ref() != Some(&actual) || ledger.len() != occurrences.len() {
        return Ok(None);
    }
    let Some(first) = ledger.first() else {
        return Ok(None);
    };
    if !runtime_shader_precedes_consumers(
        first.source_ordinal,
        provenance,
        package,
    ) {
        return Ok(None);
    }
    Ok(Some(first.clone()))
}

/// Verify one physical shader against its phase-three material coordinate.
fn runtime_shader_package_member_matches(
    package: &PhaseThreePackageRow,
    occurrence: &RuntimeLedgerOccurrence,
) -> bool {
    let path = format!(
        "{}/components/shader/{}",
        package.package_root, occurrence.member
    );
    package
        .find_member_by_source_coordinate(&path, occurrence.source_ordinal)
        .is_some_and(|member| {
            member.role == PackageRole::Material
                && member.kind == "p3d-shader"
                && member.source_chunk_kind == "shader"
        })
}

/// Verify that the selected shader existed before every proven consumer.
fn runtime_shader_precedes_consumers(
    shader_ordinal: usize,
    provenance: &ShaderConsumerProvenance,
    package: &PhaseThreePackageRow,
) -> bool {
    let mut consumer_ordinals = provenance.source_ordinals.clone();
    for member_id in &provenance.model_member_ids {
        let Some(source_ordinal) = package
            .members()
            .iter()
            .find(|member| {
                member.id == *member_id
                    && member.role == PackageRole::Model
                    && member.source_chunk_kind == "mesh"
            })
            .and_then(|member| member.source_chunk_ordinal)
        else {
            return false;
        };
        let _inserted = consumer_ordinals.insert(source_ordinal);
    }
    !consumer_ordinals.is_empty()
        && consumer_ordinals
            .iter()
            .all(|ordinal| *ordinal > shader_ordinal)
}

/// Select the first local texture that existed when the shader was loaded.
fn runtime_first_texture_occurrence(
    package_root: &Path,
    texture: &str,
    shader_ordinal: usize,
) -> Result<Option<RuntimeLedgerOccurrence>, PipelineError> {
    let occurrences =
        top_level_ledger_occurrences(package_root, "texture", texture)?;
    let Some(first) = occurrences.first() else {
        return Ok(None);
    };
    if first.source_ordinal >= shader_ordinal {
        return Err(PipelineError::new(format!(
            concat!(
                "prop texture {} first appears at source ordinal {} after ",
                "runtime-first shader ordinal {}"
            ),
            texture, first.source_ordinal, shader_ordinal
        )));
    }
    Ok(Some(first.clone()))
}

/// Read top-level occurrences for one runtime inventory identity.
fn top_level_ledger_occurrences(
    package_root: &Path,
    kind: &str,
    logical: &str,
) -> Result<Vec<RuntimeLedgerOccurrence>, PipelineError> {
    let manifest = package_root.join("components.jsonl");
    let text = fs::read_to_string(&manifest).map_err(|source| {
        PipelineError::new(format!(
            "runtime inventory ledger read failed: {source}"
        ))
    })?;
    let mut occurrences = Vec::new();
    let prefix = format!("{kind}/");
    for line in text.lines().skip(1) {
        let value = serde_json::from_str::<Value>(line).map_err(|source| {
            PipelineError::new(format!(
                "runtime inventory ledger JSON failed: {source}"
            ))
        })?;
        if value.get("kind").and_then(Value::as_str) != Some(kind) {
            continue;
        }
        let Some(name) = value.get("name").and_then(Value::as_str) else {
            continue;
        };
        if !name
            .trim_end_matches(char::from(0))
            .eq_ignore_ascii_case(logical)
        {
            continue;
        }
        if value.get("depth").and_then(Value::as_u64) != Some(1)
            || value.get("parent_ordinal").and_then(Value::as_u64) != Some(0)
        {
            return Ok(Vec::new());
        }
        let source_ordinal = value
            .get("ordinal")
            .and_then(Value::as_u64)
            .and_then(|ordinal| usize::try_from(ordinal).ok())
            .ok_or_else(|| {
                PipelineError::new(
                    "runtime inventory occurrence has no source ordinal",
                )
            })?;
        let relative = value
            .get("path")
            .and_then(Value::as_str)
            .and_then(|path| path.strip_prefix(&prefix))
            .filter(|member| {
                !member.is_empty()
                    && !member.contains('/')
                    && !member.contains('\\')
            })
            .ok_or_else(|| {
                PipelineError::new(
                    "runtime inventory occurrence path is unsafe",
                )
            })?;
        occurrences.push(RuntimeLedgerOccurrence {
            member: relative.to_owned(),
            source_ordinal,
            path: package_root.join("components").join(kind).join(relative),
        });
    }
    occurrences.sort_by_key(|occurrence| occurrence.source_ordinal);
    Ok(occurrences)
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

/// Resolve source shaders and replace source names with content-derived names.
type MaterialPlan = (
    BTreeMap<String, String>,
    Vec<MaterialBinding>,
    Vec<PreparedTexture>,
);

/// Canonical material plan with exact source presentation evidence.
struct MaterialPlanWithPresentation {
    renames: BTreeMap<String, String>,
    materials: Vec<MaterialBinding>,
    textures: Vec<PreparedTexture>,
    presentations: Vec<CanonicalMaterialPresentation>,
}

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
    let plan = resolve_material_plan(
        shaders,
        shader_sources,
        package_root,
        scratch,
        authority,
        package,
        source_subcategory,
    )?;
    Ok((plan.renames, plan.materials, plan.textures))
}

/// Resolve bindings and retain exact runtime-visible shader state.
fn resolve_material_plan(
    shaders: BTreeSet<String>,
    shader_sources: &BTreeMap<String, ShaderConsumerProvenance>,
    package_root: &Path,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
    package: Option<&PhaseThreePackageRow>,
    source_subcategory: &str,
) -> Result<MaterialPlanWithPresentation, PipelineError> {
    fs::create_dir_all(scratch).map_err(|error| {
        PipelineError::new(format!(
            "prop material scratch creation failed: {error}"
        ))
    })?;
    let mut renames = BTreeMap::new();
    let mut bindings = BTreeMap::new();
    let mut textures = BTreeMap::new();
    let mut presentations = BTreeMap::new();
    for (shader_index, shader) in shaders.into_iter().enumerate() {
        let shader_scratch = scratch.join(format!("shader-{shader_index:04}"));
        fs::create_dir_all(&shader_scratch).map_err(|error| {
            PipelineError::new(format!(
                "prop shader scratch creation failed: {error}"
            ))
        })?;
        let source = DecodedComponentSource::new(package_root, &shader_scratch);
        let provenance = shader_sources.get(&shader);
        let binding = resolve_source_material(
            &source,
            package_root,
            &shader,
            provenance,
            authority,
            package,
            source_subcategory,
        )?;
        let shader_source = runtime_visible_shader_evidence(
            &source,
            package_root,
            &shader,
            provenance,
            authority,
            package,
        )?;
        let source_semantics = binding.semantics;
        let source_base_color = binding.base_color_rgba8;
        let (texture_digest, canonical_texture) =
            match binding.texture_file_name.as_deref() {
                Some(source_name) => {
                    let source_bytes =
                        fs::read(shader_scratch.join(source_name)).map_err(
                            |error| {
                                PipelineError::new(format!(
                                    "prop staged texture read failed for \
                                     {source_name}: {error}"
                                ))
                            },
                        )?;
                    let prepared = prepare_source_texture(source_bytes);
                    let digest = prepared.sha256.clone();
                    let file_name = prepared.file_name.clone();
                    match textures.get(&file_name) {
                        Some(existing) if existing != &prepared => {
                            return Err(PipelineError::new(format!(
                                "prop texture identity conflicts: {file_name}"
                            )));
                        },
                        Some(_) => {},
                        None => {
                            let _previous =
                                textures.insert(file_name.clone(), prepared);
                        },
                    }
                    (Some(digest), Some(file_name))
                },
                None => (None, None),
            };
        let presentation_sha256 = material_presentation_digest(
            texture_digest.as_deref(),
            source_base_color,
            source_semantics,
            shader_source.as_ref().map(|source| &source.evidence),
        )?;
        let source_shader_identity = shader_source
            .as_ref()
            .map(|source| source.evidence.identity.as_str());
        let binding_sha256 = material_binding_digest(
            &presentation_sha256,
            source_shader_identity,
        )?;
        let canonical_material =
            canonical_material_identity(&binding_sha256, source_semantics);
        let _previous_rename =
            renames.insert(shader, canonical_material.clone());
        let material =
            MaterialBinding::new(canonical_material.clone(), canonical_texture)
                .map(|material| {
                    material
                        .with_semantics(source_semantics)
                        .with_base_color_rgba8(source_base_color)
                })
                .map_err(|error| {
                    PipelineError::new(format!(
                        "canonical prop material failed: {error:?}"
                    ))
                })?;
        match bindings.get(&canonical_material) {
            Some(existing) if existing != &material => {
                return Err(PipelineError::new(format!(
                    "canonical material conflicts: {canonical_material}"
                )));
            },
            Some(_) => {},
            None => {
                let _previous =
                    bindings.insert(canonical_material.clone(), material);
            },
        }
        let presentation = CanonicalMaterialPresentation {
            material_name: canonical_material.clone(),
            binding_sha256,
            presentation_sha256,
            texture_sha256: texture_digest,
            source_shaders: shader_source.into_iter().collect(),
        };
        match presentations.get_mut(&canonical_material) {
            Some(existing) => {
                merge_material_presentation_evidence(existing, presentation)?;
            },
            None => {
                let _previous =
                    presentations.insert(canonical_material, presentation);
            },
        }
    }
    Ok(MaterialPlanWithPresentation {
        renames,
        materials: bindings.into_values().collect(),
        textures: textures.into_values().collect(),
        presentations: presentations.into_values().collect(),
    })
}

/// Read the exact shader occurrence selected by the runtime-first policy.
/// Material resolution uses the same policy.
fn runtime_visible_shader_evidence(
    source: &DecodedComponentSource,
    package_root: &Path,
    shader: &str,
    consumer_provenance: Option<&ShaderConsumerProvenance>,
    authority: Option<&SharedTextureAuthority>,
    package: Option<&PhaseThreePackageRow>,
) -> Result<Option<CanonicalShaderPresentationSource>, PipelineError> {
    match source.shader_source_evidence(shader) {
        Ok(evidence) => {
            let ledger = package_root.join("components.jsonl");
            let occurrences = if ledger.is_file() {
                top_level_ledger_occurrences(package_root, "shader", shader)?
            } else {
                Vec::new()
            };
            let occurrence = match occurrences.as_slice() {
                [only] => Some(only),
                _ => None,
            };
            Ok(Some(CanonicalShaderPresentationSource {
                package_id: package.map(|value| value.package_id.clone()),
                source_ordinal: occurrence.map(|value| value.source_ordinal),
                member: occurrence.map(|value| value.member.clone()),
                evidence,
            }))
        },
        Err(DecodedComponentError::MissingShaderMember { .. })
            if authority.is_some()
                && runtime_missing_shader_has_package_consumers(
                    consumer_provenance,
                    package,
                ) =>
        {
            Ok(None)
        },
        Err(error @ DecodedComponentError::AmbiguousShaderMember { .. }) => {
            let Some(first_shader) = runtime_first_shader_occurrence(
                package_root,
                shader,
                consumer_provenance,
                package,
                &error,
            )? else {
                return Err(material_resolution_error(
                    shader,
                    consumer_provenance,
                    &error,
                    package,
                ));
            };
            let shader_path = &first_shader.path;
            let evidence = read_shader_source_evidence(shader_path, shader)
                .map_err(|source| {
                    PipelineError::new(format!(
                        "runtime-visible shader evidence failed: {source:?}"
                    ))
                })?;
            Ok(Some(CanonicalShaderPresentationSource {
                package_id: package.map(|value| value.package_id.clone()),
                source_ordinal: Some(first_shader.source_ordinal),
                member: Some(first_shader.member),
                evidence,
            }))
        },
        Err(error) => Err(material_resolution_error(
            shader,
            consumer_provenance,
            &error,
            package,
        )),
    }
}

/// Merge equivalent visual state while retaining independently addressable
/// source shader identity.
pub(super) fn merge_material_presentation_evidence(
    target: &mut CanonicalMaterialPresentation,
    incoming: CanonicalMaterialPresentation,
) -> Result<(), PipelineError> {
    if target.material_name != incoming.material_name
        || target.binding_sha256 != incoming.binding_sha256
        || target.presentation_sha256 != incoming.presentation_sha256
        || target.texture_sha256 != incoming.texture_sha256
    {
        return Err(PipelineError::new(format!(
            "canonical prop presentation identity conflicts: {}",
            target.material_name
        )));
    }
    match (target.source_shaders.first(), incoming.source_shaders.first()) {
        (None, None) => {},
        (Some(left), Some(right))
            if left.evidence.identity == right.evidence.identity
                && same_shader_presentation_state(
                    &left.evidence,
                    &right.evidence,
                ) => {},
        _ => {
            return Err(PipelineError::new(format!(
                "canonical prop presentation digest conflicts: {}",
                target.material_name
            )));
        },
    }
    for source in incoming.source_shaders {
        if !target.source_shaders.contains(&source) {
            target.source_shaders.push(source);
        }
    }
    target.source_shaders.sort_by(|left, right| {
        left.package_id
            .cmp(&right.package_id)
            .then(left.source_ordinal.cmp(&right.source_ordinal))
            .then(left.member.cmp(&right.member))
            .then(left.evidence.identity.cmp(&right.evidence.identity))
    });
    Ok(())
}

/// Compare presentation state while deliberately ignoring logical identity.
fn same_shader_presentation_state(
    left: &ShaderSourceEvidence,
    right: &ShaderSourceEvidence,
) -> bool {
    left.schema == right.schema
        && left.version == right.version
        && left.platform_shader_name == right.platform_shader_name
        && left.translucency == right.translucency
        && left.vertex_needs == right.vertex_needs
        && left.vertex_mask == right.vertex_mask
        && left.parameter_count == right.parameter_count
        && left.texture_reference == right.texture_reference
        && left.params == right.params
}

/// Hash exact presentation state without the authored logical shader name.
pub(super) fn material_presentation_digest(
    texture_sha256: Option<&str>,
    base_color_rgba8: [u8; 4],
    semantics: MaterialSemantics,
    shader: Option<&ShaderSourceEvidence>,
) -> Result<String, PipelineError> {
    let shader_value = shader.map(|evidence| {
        json!({
            "schema": evidence.schema,
            "version": evidence.version,
            "platform_shader_name": evidence.platform_shader_name,
            "translucency": evidence.translucency,
            "vertex_needs": evidence.vertex_needs,
            "vertex_mask": evidence.vertex_mask,
            "parameter_count": evidence.parameter_count,
            "texture_reference": evidence.texture_reference,
            "params": evidence.params.iter().map(|parameter| json!({
                "kind": parameter.kind,
                "param": parameter.param,
                "value": parameter.value
            })).collect::<Vec<_>>()
        })
    });
    let value = json!({
        "texture_sha256": texture_sha256,
        "base_color_rgba8": base_color_rgba8,
        "semantics": {
            "transparent": semantics.is_transparent(),
            "glass": semantics.is_glass(),
            "mirror": semantics.is_mirror(),
            "reflective": semantics.is_reflective(),
            "light_emitter": semantics.is_light_emitter(),
            "visual_effect": semantics.is_visual_effect()
        },
        "shader": shader_value
    });
    let bytes = serde_json::to_vec(&value).map_err(|error| {
        PipelineError::new(format!(
            "prop material presentation serialization failed: {error}"
        ))
    })?;
    Ok(digest_hex(&bytes))
}

/// Hash one FBX material binding without collapsing runtime shader targets.
fn material_binding_digest(
    presentation_sha256: &str,
    source_shader_identity: Option<&str>,
) -> Result<String, PipelineError> {
    let value = json!({
        "presentation_sha256": presentation_sha256,
        "source_shader_identity": source_shader_identity
    });
    let bytes = serde_json::to_vec(&value).map_err(|error| {
        PipelineError::new(format!(
            "prop material binding serialization failed: {error}"
        ))
    })?;
    Ok(digest_hex(&bytes))
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

/// Build a content-derived identity from one runtime-addressable binding.
fn canonical_material_identity(
    binding_digest: &str,
    semantics: MaterialSemantics,
) -> String {
    let base = format!("material-{binding_digest}");
    semantics
        .suffix()
        .map_or_else(|| base.clone(), |suffix| format!("{base}-{suffix}"))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/material/tests.rs"]
mod tests;
