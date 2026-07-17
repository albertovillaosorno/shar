// File:
//   - world_export.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_export.rs
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
//   - Name consolidation, compatible clip merging, and world-prop publication.
// - Must-Not:
//   - Export terrain placement, collision, particles, or gameplay state.
// - Allows:
//   - One canonical FBX per readable name and retained variant evidence.
// - Summary:
//   - Publishes hash-free world props without silently overwriting collisions.
//
// Large file:
//   - true

//! Consolidated world-prop publication.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use fbx::adapters::driven::binary_character_writer::{
    write_binary_character_fbx, write_binary_model_fbx,
};
use fbx::domain::animation::AnimationClip;
use shar_sha256::digest_hex;

use super::inventory_common::portable_asset_name;
use super::model::{PropAlias, PropCandidate, PropRoute, TextureRecord};
use super::prepare::{
    prepare_world_candidate, prepared_signature, rig_signature,
    structural_signature, visual_signature,
};
use super::prepared::{PreparedGeometry, PreparedProp, PreparedTexture};
use super::texture_authority::SharedTextureAuthority;
use super::world_model::{ExportedWorldProp, OmittedWorldVariant};
use crate::domain::PipelineError;

/// One prepared same-name world-prop variant before consolidation.
#[derive(Debug)]
struct WorldVariant {
    /// Canonical prepared geometry, materials, textures, and clips.
    prepared: PreparedProp,
    /// Source occurrences represented by this semantic variant.
    aliases: Vec<PropAlias>,
    /// Geometry signature including presentation channels.
    visual_sha256: String,
    /// Position and topology signature excluding presentation channels.
    structural_sha256: String,
    /// Optional rigid-binding signature.
    rig_sha256: Option<String>,
}

/// Prepare, consolidate, and publish every discovered world prop.
///
/// # Errors
///
/// Returns an error when preparation, consolidation, or publication fails.
pub(super) fn export_world_props(
    candidates: &[PropCandidate],
    normalized_root: &Path,
    scratch_root: &Path,
    authority: &SharedTextureAuthority,
    output_root: &Path,
) -> Result<Vec<ExportedWorldProp>, PipelineError> {
    let mut groups: BTreeMap<String, Vec<WorldVariant>> = BTreeMap::new();
    for (ordinal, candidate) in candidates
        .iter()
        .enumerate()
    {
        let prepared = prepare_world_candidate(
            candidate,
            normalized_root,
            scratch_root,
            authority,
            ordinal,
        )?;
        let name = portable_name(&candidate.owner_name)?;
        let variants = groups
            .entry(name)
            .or_default();
        if let Some(existing) = variants
            .iter_mut()
            .find(
                |value| {
                    value
                        .prepared
                        .signature
                        == prepared.signature
                },
            )
        {
            existing
                .aliases
                .push(PropAlias::from(candidate));
        } else {
            variants.push(
                WorldVariant {
                    visual_sha256: visual_signature(&prepared),
                    structural_sha256: structural_signature(&prepared),
                    rig_sha256: rig_signature(&prepared),
                    prepared,
                    aliases: vec![PropAlias::from(candidate)],
                },
            );
        }
        fs::remove_dir_all(
            scratch_root.join(format!("candidate-{ordinal:06}")),
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "world prop material scratch cleanup failed: {error}"
                    ),
                )
            },
        )?;
    }

    let mut output = Vec::new();
    for (name, variants) in groups {
        output.push(
            publish_group(
                &name,
                variants,
                output_root,
            )?,
        );
    }
    output.sort_by(
        |left, right| {
            left.asset_id
                .cmp(&right.asset_id)
        },
    );
    Ok(output)
}

/// Select, consolidate, and publish one readable same-name variant group.
///
/// # Errors
///
/// Returns an error when the group is empty or artifact publication fails.
fn publish_group(
    name: &str,
    mut variants: Vec<WorldVariant>,
    output_root: &Path,
) -> Result<ExportedWorldProp, PipelineError> {
    variants.sort_by(compare_variants);
    let mut iterator = variants.into_iter();
    let mut canonical = iterator
        .next()
        .ok_or_else(
            || PipelineError::new("world prop variant group is empty"),
        )?;
    let canonical_structure = canonical
        .structural_sha256
        .clone();
    let canonical_rig = canonical
        .rig_sha256
        .clone();
    let mut compatible = Vec::new();
    let mut omitted = Vec::new();
    for variant in iterator {
        if variant.structural_sha256 == canonical_structure
            && variant.rig_sha256 == canonical_rig
        {
            compatible.push(variant);
        } else {
            omitted.push(
                OmittedWorldVariant {
                    semantic_sha256: variant
                        .prepared
                        .signature,
                    visual_sha256: variant.visual_sha256,
                    structural_sha256: variant.structural_sha256,
                    route: variant
                        .prepared
                        .route,
                    source_count: variant
                        .aliases
                        .len(),
                },
            );
            canonical
                .aliases
                .extend(variant.aliases);
        }
    }
    let merged_count = compatible.len();
    merge_compatible(
        &mut canonical,
        compatible,
    );
    write_world_prop(
        name,
        canonical,
        omitted,
        merged_count,
        output_root,
    )
}

/// Return the first stable source package id or an empty fallback.
fn first_package_id(variant: &WorldVariant) -> &str {
    variant
        .aliases
        .first()
        .map_or(
            "",
            |alias| {
                alias
                    .package_id
                    .as_str()
            },
        )
}

/// Order same-name variants by completeness and stable provenance.
fn compare_variants(
    left: &WorldVariant,
    right: &WorldVariant,
) -> Ordering {
    richness(right)
        .cmp(&richness(left))
        .then_with(|| first_package_id(left).cmp(first_package_id(right)))
        .then_with(
            || {
                left.prepared
                    .signature
                    .cmp(
                        &right
                            .prepared
                            .signature,
                    )
            },
        )
}

/// Return one deterministic completeness score for canonical selection.
fn richness(
    variant: &WorldVariant
) -> (
    bool,
    usize,
    usize,
    usize,
) {
    let (groups, bones) = match &variant
        .prepared
        .geometry
    {
        PreparedGeometry::Static(meshes) => (
            meshes
                .iter()
                .map(
                    |mesh| {
                        mesh.groups
                            .len()
                    },
                )
                .sum(),
            0,
        ),
        PreparedGeometry::RigidAnimated {
            asset,
            ..
        } => (
            asset
                .parts
                .iter()
                .map(
                    |part| {
                        part.mesh
                            .groups
                            .len()
                    },
                )
                .sum(),
            asset
                .bones
                .len(),
        ),
    };
    (
        variant
            .prepared
            .route
            == PropRoute::RigidAnimated,
        groups,
        bones,
        variant
            .aliases
            .len(),
    )
}

/// Merge structurally compatible aliases, textures, and distinct clips.
fn merge_compatible(
    canonical: &mut WorldVariant,
    variants: Vec<WorldVariant>,
) {
    let mut textures: BTreeMap<String, PreparedTexture> = canonical
        .prepared
        .textures
        .drain(..)
        .map(
            |texture| {
                (
                    texture
                        .file_name
                        .clone(),
                    texture,
                )
            },
        )
        .collect();
    let mut clips = take_clips(&mut canonical.prepared);
    let mut clip_keys = clips
        .iter()
        .map(animation_key)
        .collect::<BTreeSet<_>>();
    for mut variant in variants {
        canonical
            .aliases
            .append(&mut variant.aliases);
        for texture in std::mem::take(
            &mut variant
                .prepared
                .textures,
        ) {
            let _published_texture = textures
                .entry(
                    texture
                        .file_name
                        .clone(),
                )
                .or_insert(texture);
        }
        for clip in take_clips(&mut variant.prepared) {
            let key = animation_key(&clip);
            if clip_keys.insert(key) {
                clips.push(clip);
            }
        }
    }
    clips.sort_by_key(animation_key);
    for (ordinal, clip) in clips
        .iter_mut()
        .enumerate()
    {
        clip.name = format!("animation-{ordinal:04}");
    }
    if let PreparedGeometry::RigidAnimated {
        animations,
        ..
    } = &mut canonical
        .prepared
        .geometry
    {
        *animations = clips;
    }
    canonical
        .prepared
        .textures = textures
        .into_values()
        .collect();
    canonical
        .prepared
        .signature = prepared_signature(
        canonical
            .prepared
            .route,
        &canonical
            .prepared
            .geometry,
        &canonical
            .prepared
            .materials,
        &canonical
            .prepared
            .textures,
    );
    canonical
        .aliases
        .sort_by(
            |left, right| {
                (
                    &left.package_id,
                    &left.container_key,
                )
                    .cmp(
                        &(
                            &right.package_id,
                            &right.container_key,
                        ),
                    )
            },
        );
}

/// Remove and return authored clips from one prepared variant.
fn take_clips(prepared: &mut PreparedProp) -> Vec<AnimationClip> {
    match &mut prepared.geometry {
        PreparedGeometry::Static(_) => Vec::new(),
        PreparedGeometry::RigidAnimated {
            animations,
            ..
        } => std::mem::take(animations),
    }
}

/// Hash one animation independently of its publication name.
fn animation_key(clip: &AnimationClip) -> String {
    let mut value = clip.clone();
    value
        .name
        .clear();
    digest_hex(format!("{value:?}").as_bytes())
}

/// Publish every external texture referenced by one prepared world prop.
///
/// # Errors
///
/// Returns an error when texture writing or byte-count conversion fails.
fn publish_world_textures(
    prepared: &PreparedProp,
    texture_dir: &Path,
) -> Result<Vec<TextureRecord>, PipelineError> {
    let mut records = Vec::new();
    for texture in &prepared.textures {
        fs::write(
            texture_dir.join(&texture.file_name),
            &texture.bytes,
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!("world prop texture write failed: {error}"),
                )
            },
        )?;
        records.push(
            TextureRecord {
                file_name: texture
                    .file_name
                    .clone(),
                bytes: u64::try_from(
                    texture
                        .bytes
                        .len(),
                )
                .map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "world prop texture size overflowed: {error}"
                            ),
                        )
                    },
                )?,
                sha256: texture
                    .sha256
                    .clone(),
            },
        );
    }
    Ok(records)
}

/// Write one canonical world prop and construct its catalog record.
///
/// # Errors
///
/// Returns an error when directories, textures, FBX, or hashes fail.
fn write_world_prop(
    name: &str,
    variant: WorldVariant,
    omitted: Vec<OmittedWorldVariant>,
    merged_count: usize,
    output_root: &Path,
) -> Result<ExportedWorldProp, PipelineError> {
    let directory = output_root.join(name);
    let texture_dir = directory.join("textures");
    fs::create_dir_all(&texture_dir).map_err(
        |error| {
            PipelineError::new(format!("world prop directory failed: {error}"))
        },
    )?;
    let textures = publish_world_textures(
        &variant.prepared,
        &texture_dir,
    )?;
    let path = directory.join(format!("{name}.fbx"));
    let summary = match &variant
        .prepared
        .geometry
    {
        PreparedGeometry::Static(meshes) => write_binary_model_fbx(
            name,
            meshes,
            &variant
                .prepared
                .materials,
            &path,
        ),
        PreparedGeometry::RigidAnimated {
            asset,
            animations,
        } => write_binary_character_fbx(
            asset,
            &variant
                .prepared
                .materials,
            animations,
            &path,
        ),
    }
    .map_err(
        |error| {
            PipelineError::new(
                format!("world prop FBX write failed: {error:?}"),
            )
        },
    )?;
    let bytes = fs::read(&path).map_err(
        |error| {
            PipelineError::new(
                format!("world prop FBX read-back failed: {error}"),
            )
        },
    )?;
    Ok(
        ExportedWorldProp {
            asset_id: name.to_owned(),
            route: variant
                .prepared
                .route,
            semantic_sha256: variant
                .prepared
                .signature,
            visual_sha256: variant.visual_sha256,
            structural_sha256: variant.structural_sha256,
            rig_sha256: variant.rig_sha256,
            fbx_path: format!("{name}/{name}.fbx"),
            fbx_bytes: u64::try_from(bytes.len()).map_err(
                |error| {
                    PipelineError::new(
                        format!("world prop FBX size overflowed: {error}"),
                    )
                },
            )?,
            fbx_sha256: digest_hex(&bytes),
            summary,
            textures,
            aliases: variant.aliases,
            merged_compatible_variants: merged_count,
            omitted_visual_variants: omitted,
        },
    )
}

/// Build one readable portable world-prop name.
///
/// # Errors
///
/// Returns an error when the source identity has no portable characters.
fn portable_name(value: &str) -> Result<String, PipelineError> {
    portable_asset_name(
        value,
        64,
        "world prop identity has no portable name",
    )
}

#[cfg(test)]
mod tests {
    use super::portable_name;

    #[test]
    fn portable_world_name_has_no_hash_suffix() {
        assert_eq!(
            portable_name("Cypress Tree!"),
            Ok("cypress-tree".to_owned())
        );
    }
}
