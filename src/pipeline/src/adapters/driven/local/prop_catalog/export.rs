// File:
//   - export.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/export.rs
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
//   - Semantic prop deduplication and unique FBX/texture artifact publication.
// - Must-Not:
//   - Re-extract sources, infer non-model assets, or publish the root catalog.
// - Allows:
//   - Sequential preparation, content-key lookup, and binary model writing.
// - Split-When:
//   - Static and animated artifact publication gain different transactions.
// - Merge-When:
//   - Catalog rendering owns the same artifact publication lifecycle.
// - Summary:
//   - Writes one FBX per unique canonical mission or terrain-world model.
// - Description:
//   - Duplicate occurrences become source aliases instead of duplicate files.
// - Usage:
//   - Called once after complete candidate discovery.
// - Defaults:
//   - Mission and terrain-world signatures deduplicate within their own family.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Semantic prop deduplication and unique artifact publication.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use fbx::adapters::driven::binary_character_writer::{
    write_binary_character_fbx, write_binary_model_fbx,
};
use shar_sha256::digest_hex;

use super::model::{ExportedProp, PropAlias, PropCandidate, TextureRecord};
use super::prepare::prepare_candidate;
use super::prepared::{PreparedGeometry, PreparedProp};
use crate::domain::PipelineError;

/// Prepare, deduplicate, and publish every discovered prop occurrence.
///
/// # Errors
///
/// Returns an error when preparation, duplicate accounting, or artifact writing
/// fails.
pub(super) fn export_unique_props(
    candidates: &[PropCandidate],
    normalized_root: &Path,
    scratch_root: &Path,
    output_root: &Path,
) -> Result<Vec<ExportedProp>, PipelineError> {
    let mut assets = Vec::new();
    let mut identities = BTreeMap::new();
    let mut public_names = BTreeMap::new();
    for (ordinal, candidate) in candidates
        .iter()
        .enumerate()
    {
        let prepared = prepare_candidate(
            candidate,
            normalized_root,
            scratch_root,
            ordinal,
        )?;
        let key = (
            candidate.family,
            prepared
                .signature
                .clone(),
        );
        if let Some(existing) = identities
            .get(&key)
            .copied()
        {
            let asset: &mut ExportedProp = assets
                .get_mut(existing)
                .ok_or_else(
                    || {
                        PipelineError::new(
                            "prop duplicate index escaped asset list",
                        )
                    },
                )?;
            asset
                .aliases
                .push(PropAlias::from(candidate));
        } else {
            let public_name = asset_name(&candidate.owner_name)?;
            if let Some(existing_signature) = public_names.get(&public_name)
                && existing_signature != &prepared.signature
            {
                return Err(
                    PipelineError::new(
                        format!(
                            "mission prop name collision for {public_name}"
                        ),
                    ),
                );
            }
            let index = assets.len();
            let exported = write_unique_prop(
                candidate,
                prepared,
                &public_name,
                output_root,
            )?;
            public_names.insert(
                public_name,
                exported
                    .signature
                    .clone(),
            );
            identities.insert(
                key, index,
            );
            assets.push(exported);
        }
        let _cleanup_result = fs::remove_dir_all(
            scratch_root.join(format!("candidate-{ordinal:06}")),
        );
    }
    for asset in &mut assets {
        asset
            .aliases
            .sort_by(
                |left, right| {
                    (
                        &left.package_id,
                        &left.owner_name,
                        &left.container_key,
                    )
                        .cmp(
                            &(
                                &right.package_id,
                                &right.owner_name,
                                &right.container_key,
                            ),
                        )
                },
            );
    }
    assets.sort_by(
        |left, right| {
            (
                left.family,
                &left.asset_id,
            )
                .cmp(
                    &(
                        right.family,
                        &right.asset_id,
                    ),
                )
        },
    );
    Ok(assets)
}

/// Write one unique prepared model and construct its catalog record.
fn write_unique_prop(
    candidate: &PropCandidate,
    prepared: PreparedProp,
    asset_id: &str,
    output_root: &Path,
) -> Result<ExportedProp, PipelineError> {
    let relative_dir = asset_id.to_owned();
    let directory = output_root.join(&relative_dir);
    let texture_dir = directory.join("textures");
    fs::create_dir_all(&texture_dir).map_err(
        |error| {
            PipelineError::new(format!("prop asset directory failed: {error}"))
        },
    )?;
    let mut texture_records = Vec::new();
    for texture in &prepared.textures {
        fs::write(
            texture_dir.join(&texture.file_name),
            &texture.bytes,
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "prop texture publication failed for {}: {error}",
                        texture.file_name
                    ),
                )
            },
        )?;
        texture_records.push(
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
                    |_| {
                        PipelineError::new("prop texture byte count overflowed")
                    },
                )?,
                sha256: texture
                    .sha256
                    .clone(),
            },
        );
    }
    let fbx_path = directory.join(format!("{asset_id}.fbx"));
    let summary = match &prepared.geometry {
        PreparedGeometry::Static(meshes) => write_binary_model_fbx(
            asset_id,
            meshes,
            &prepared.materials,
            &fbx_path,
        ),
        PreparedGeometry::RigidAnimated {
            asset,
            animations,
        } => write_binary_character_fbx(
            asset,
            &prepared.materials,
            animations,
            &fbx_path,
        ),
    }
    .map_err(
        |error| PipelineError::new(format!("prop FBX write failed: {error:?}")),
    )?;
    let fbx_bytes = fs::read(&fbx_path).map_err(
        |error| {
            PipelineError::new(format!("prop FBX read-back failed: {error}"))
        },
    )?;
    Ok(
        ExportedProp {
            asset_id: asset_id.to_owned(),
            family: candidate.family,
            route: prepared.route,
            signature: prepared.signature,
            fbx_path: format!("{relative_dir}/{asset_id}.fbx"),
            fbx_bytes: u64::try_from(fbx_bytes.len()).map_err(
                |_| PipelineError::new("prop FBX byte count overflowed"),
            )?,
            fbx_sha256: digest_hex(&fbx_bytes),
            summary,
            textures: texture_records,
            aliases: vec![PropAlias::from(candidate)],
        },
    )
}

/// Build one readable portable mission prop name.
///
/// # Errors
///
/// Returns an error when the source identity has no portable characters.
fn asset_name(owner_name: &str) -> Result<String, PipelineError> {
    let mut slug = String::new();
    let mut previous_dash = false;
    for character in owner_name
        .chars()
        .flat_map(char::to_lowercase)
    {
        let normalized = if character.is_ascii_alphanumeric() {
            character
        } else {
            '-'
        };
        if normalized == '-' {
            if previous_dash || slug.is_empty() {
                continue;
            }
            previous_dash = true;
        } else {
            previous_dash = false;
        }
        slug.push(normalized);
        if slug.len() == 48 {
            break;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        return Err(
            PipelineError::new("mission prop identity has no portable name"),
        );
    }
    Ok(slug)
}

#[cfg(test)]
mod tests {
    use super::asset_name;

    #[test]
    fn asset_name_is_readable_without_a_hash_suffix() {
        assert_eq!(
            asset_name("Finish Line!"),
            Ok("finish-line".to_owned())
        );
    }

    #[test]
    fn asset_name_rejects_empty_portable_identity() {
        assert!(asset_name("___").is_err());
    }
}
