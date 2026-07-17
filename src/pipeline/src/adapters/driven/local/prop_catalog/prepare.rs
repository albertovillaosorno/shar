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
use fbx::ports::component_source::ComponentSource as _;
use shar_sha256::digest_hex;

use super::canonical::{
    canonicalize_animated_asset, canonicalize_static_meshes,
};
use super::material::{
    canonicalize_animated_materials, canonicalize_static_materials,
};
use super::model::{PropCandidate, PropRoute};
use super::prepared::{PreparedGeometry, PreparedProp};
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
        ),
        PropRoute::RigidAnimated => prepare_animated(
            candidate,
            &package_root,
            &scratch,
        ),
    }
}

/// Prepare one static model candidate.
fn prepare_static(
    candidate: &PropCandidate,
    package_root: &Path,
    scratch: &Path,
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
    let (materials, textures) = canonicalize_static_materials(
        &mut meshes,
        package_root,
        scratch,
    )?;
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
    if animations.len() != 1 {
        return Err(
            PipelineError::new(
                "rigid prop must publish exactly one exact PTRN clip",
            ),
        );
    }
    let (materials, textures) = canonicalize_animated_materials(
        &mut asset,
        package_root,
        scratch,
    )?;
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
    let member = member.ok_or_else(
        || {
            PipelineError::new(
                format!("rigid prop is missing required {family} member"),
            )
        },
    )?;
    Ok(
        component_path(
            root, family, member,
        ),
    )
}

/// Hash canonical domain values and texture digests for semantic deduplication.
fn prepared_signature(
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
