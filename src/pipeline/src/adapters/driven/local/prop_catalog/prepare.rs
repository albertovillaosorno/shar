// File:
//   - prepare.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/prepare.rs
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
//   - Candidate loading, canonicalization, and semantic prop signatures.
// - Must-Not:
//   - Publish output directories, choose duplicate aliases, or render catalogs.
// - Allows:
//   - Decoded static/rigid source loading and content-derived material
//     planning.
// - Split-When:
//   - Static and animated preparation become independently reusable.
// - Merge-When:
//   - A shared model preparation application service owns the same contract.
// - Summary:
//   - Produces one canonical prepared prop from one source occurrence.
// - Description:
//   - Hashes normalized domain values rather than source component names.
// - Usage:
//   - Called sequentially by the prop batch deduplication loop.
// - Defaults:
//   - One rigid candidate must contain exactly one exact PTRN clip.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/unreal/native-asset-translation-and-no-copy-paste.md
//
// Large file:
//   - false
//

//! Candidate loading, canonicalization, and semantic signatures.

use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::decoded_animation_source::load_animation_clips;
use fbx::adapters::driven::decoded_component_source::DecodedComponentSource;
use fbx::adapters::driven::decoded_rigid_prop_source;
use fbx::domain::animation::AnimationClip;
use fbx::ports::component_source::ComponentSource as _;
use shar_sha256::digest_hex;

use super::canonical::{
    canonicalize_animated_asset, canonicalize_static_meshes,
};
use super::material::{
    canonicalize_animated_materials, canonicalize_static_materials,
    canonicalize_world_animated_materials, canonicalize_world_static_materials,
};
use super::model::{PropCandidate, PropRoute};
use super::prepared::{PreparedGeometry, PreparedProp};
use super::texture_authority::SharedTextureAuthority;
use crate::domain::PipelineError;

/// Load and canonicalize one model-bearing source occurrence.
///
/// # Errors
///
/// Returns an error when component loading, material staging, identity
/// canonicalization, or signature construction fails.
pub(super) fn prepare_candidate(
    candidate: &PropCandidate,
    normalized_root: &Path,
    scratch_root: &Path,
    ordinal: usize,
) -> Result<PreparedProp, PipelineError> {
    prepare_candidate_internal(
        candidate,
        normalized_root,
        scratch_root,
        None,
        ordinal,
    )
}

/// Load and canonicalize one world-prop source occurrence.
///
/// # Errors
///
/// Returns an error when components, shared textures, or signatures fail.
pub(super) fn prepare_world_candidate(
    candidate: &PropCandidate,
    normalized_root: &Path,
    scratch_root: &Path,
    authority: &SharedTextureAuthority,
    ordinal: usize,
) -> Result<PreparedProp, PipelineError> {
    prepare_candidate_internal(
        candidate,
        normalized_root,
        scratch_root,
        Some(authority),
        ordinal,
    )
}

/// Prepare one source occurrence with optional shared texture authority.
///
/// # Errors
///
/// Returns an error when scratch creation or route-specific preparation fails.
fn prepare_candidate_internal(
    candidate: &PropCandidate,
    normalized_root: &Path,
    scratch_root: &Path,
    authority: Option<&SharedTextureAuthority>,
    ordinal: usize,
) -> Result<PreparedProp, PipelineError> {
    let package_root = normalized_root.join(&candidate.relative_root);
    let scratch = scratch_root.join(format!("candidate-{ordinal:06}"));
    fs::create_dir_all(&scratch).map_err(
        |error| {
            PipelineError::new(
                format!("prop preparation scratch failed: {error}"),
            )
        },
    )?;
    match candidate.route {
        PropRoute::Static => prepare_static(
            candidate,
            &package_root,
            &scratch,
            authority,
        ),
        PropRoute::RigidAnimated => prepare_animated(
            candidate,
            &package_root,
            &scratch,
            authority,
        ),
    }
}

/// Prepare one static model candidate.
fn prepare_static(
    candidate: &PropCandidate,
    package_root: &Path,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
) -> Result<PreparedProp, PipelineError> {
    let source = DecodedComponentSource::new(
        package_root,
        scratch,
    );
    let mut meshes = candidate
        .mesh_ids
        .iter()
        .map(
            |member| {
                source
                    .load_mesh(member)
                    .map_err(
                        |error| {
                            PipelineError::new(
                                format!(
                                    "static prop mesh {member} failed: \
                                     {error:?}"
                                ),
                            )
                        },
                    )
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    let (materials, textures) = match authority {
        Some(value) => canonicalize_world_static_materials(
            &mut meshes,
            package_root,
            scratch,
            value,
            &candidate.subcategory,
        ),
        None => canonicalize_static_materials(
            &mut meshes,
            package_root,
            scratch,
        ),
    }?;
    canonicalize_static_meshes(&mut meshes);
    let geometry = PreparedGeometry::Static(meshes);
    let signature = prepared_signature(
        PropRoute::Static,
        &geometry,
        &materials,
        &textures,
    );
    Ok(
        PreparedProp {
            route: PropRoute::Static,
            signature,
            geometry,
            materials,
            textures,
        },
    )
}

/// Prepare one exact rigid-animated model candidate.
fn prepare_animated(
    candidate: &PropCandidate,
    package_root: &Path,
    scratch: &Path,
    authority: Option<&SharedTextureAuthority>,
) -> Result<PreparedProp, PipelineError> {
    let skeleton = required_component(
        candidate
            .skeleton_id
            .as_deref(),
        package_root,
        "skeleton",
    )?;
    let composite = required_component(
        candidate
            .composite_id
            .as_deref(),
        package_root,
        "composite_drawable",
    )?;
    let animation = required_component(
        candidate
            .animation_id
            .as_deref(),
        package_root,
        "animation",
    )?;
    let mesh_paths = candidate
        .mesh_ids
        .iter()
        .map(
            |member| {
                component_path(
                    package_root,
                    "mesh",
                    member,
                )
            },
        )
        .collect::<Vec<_>>();
    let mesh_refs = mesh_paths
        .iter()
        .map(PathBuf::as_path)
        .collect::<Vec<_>>();
    let mut asset = decoded_rigid_prop_source::load_selected_rigid_prop_asset(
        "model", &skeleton, &mesh_refs, &composite,
    )
    .map_err(
        |error| {
            PipelineError::new(format!("rigid prop assembly failed: {error:?}"))
        },
    )?;
    let mut animations = load_animation_clips(
        &[animation.as_path()],
        &asset.bones,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!("rigid prop animation failed: {error:?}"),
            )
        },
    )?;
    ensure_single_animation(&animations)?;
    let (materials, textures) = match authority {
        Some(value) => canonicalize_world_animated_materials(
            &mut asset,
            package_root,
            scratch,
            value,
            &candidate.subcategory,
        ),
        None => canonicalize_animated_materials(
            &mut asset,
            package_root,
            scratch,
        ),
    }?;
    canonicalize_animated_asset(
        &mut asset,
        &mut animations,
    )?;
    let geometry = PreparedGeometry::RigidAnimated {
        asset,
        animations,
    };
    let signature = prepared_signature(
        PropRoute::RigidAnimated,
        &geometry,
        &materials,
        &textures,
    );
    Ok(
        PreparedProp {
            route: PropRoute::RigidAnimated,
            signature,
            geometry,
            materials,
            textures,
        },
    )
}

/// Require exactly one authored model-transform animation.
///
/// # Errors
///
/// Returns an error when the candidate has zero or multiple exact clips.
fn ensure_single_animation(
    animations: &[AnimationClip]
) -> Result<(), PipelineError> {
    if animations.len() == 1 {
        return Ok(());
    }
    Err(
        PipelineError::new(
            "rigid prop must publish exactly one exact PTRN clip",
        ),
    )
}

/// Build one exact normalized component path.
fn component_path(
    root: &Path,
    family: &str,
    member: &str,
) -> PathBuf {
    root.join("components")
        .join(family)
        .join(format!("{member}.json"))
}

/// Require one route-owned component member.
fn required_component(
    member: Option<&str>,
    root: &Path,
    family: &str,
) -> Result<PathBuf, PipelineError> {
    let required_member = member.ok_or_else(
        || {
            PipelineError::new(
                format!("rigid prop is missing required {family} member"),
            )
        },
    )?;
    Ok(
        component_path(
            root,
            family,
            required_member,
        ),
    )
}

/// Hash canonical domain values and texture digests for semantic deduplication.
pub(super) fn prepared_signature(
    route: PropRoute,
    geometry: &PreparedGeometry,
    materials: &[fbx::domain::texture::MaterialBinding],
    textures: &[super::prepared::PreparedTexture],
) -> String {
    let evidence = format!(
        "{route:?}\n{geometry:?}\n{materials:?}\n{:?}",
        textures
            .iter()
            .map(
                |texture| (
                    &texture.file_name,
                    &texture.sha256
                )
            )
            .collect::<Vec<_>>()
    );
    digest_hex(evidence.as_bytes())
}

/// Hash model geometry independently of textures, rig, and animation.
pub(super) fn visual_signature(prepared: &PreparedProp) -> String {
    let mut meshes = match &prepared.geometry {
        PreparedGeometry::Static(value) => value.clone(),
        PreparedGeometry::RigidAnimated {
            asset,
            ..
        } => asset
            .parts
            .iter()
            .map(
                |part| {
                    part.mesh
                        .clone()
                },
            )
            .collect(),
    };
    for group in meshes
        .iter_mut()
        .flat_map(
            |mesh| {
                mesh.groups
                    .iter_mut()
            },
        )
    {
        "material".clone_into(&mut group.shader);
    }
    digest_hex(format!("{meshes:?}").as_bytes())
}

/// Hash positions and topology independently of presentation channels.
pub(super) fn structural_signature(prepared: &PreparedProp) -> String {
    let meshes = match &prepared.geometry {
        PreparedGeometry::Static(value) => value
            .iter()
            .collect::<Vec<_>>(),
        PreparedGeometry::RigidAnimated {
            asset,
            ..
        } => asset
            .parts
            .iter()
            .map(|part| &part.mesh)
            .collect::<Vec<_>>(),
    };
    let evidence = meshes
        .iter()
        .map(
            |mesh| {
                mesh.groups
                    .iter()
                    .map(
                        |group| {
                            (
                                group.index,
                                &group.positions,
                                &group.triangles,
                            )
                        },
                    )
                    .collect::<Vec<_>>()
            },
        )
        .collect::<Vec<_>>();
    digest_hex(format!("{evidence:?}").as_bytes())
}

/// Hash the canonical rigid binding independently of animation samples.
pub(super) fn rig_signature(prepared: &PreparedProp) -> Option<String> {
    match &prepared.geometry {
        PreparedGeometry::Static(_) => None,
        PreparedGeometry::RigidAnimated {
            asset,
            ..
        } => {
            let evidence = (
                &asset.bones,
                asset
                    .parts
                    .iter()
                    .map(|part| &part.group_influences)
                    .collect::<Vec<_>>(),
            );
            Some(digest_hex(format!("{evidence:?}").as_bytes()))
        }
    }
}
