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

use super::inventory_common::portable_asset_name;
use super::model::{
    ExportedProp, PropAlias, PropCandidate, PropFamily, TextureRecord,
};
use super::prepare::prepare_candidate;
use super::prepared::{PreparedGeometry, PreparedProp};
use crate::domain::PipelineError;

/// Mutable publication indexes for one complete non-world prop batch.
#[derive(Debug, Default)]
struct ExportState {
    /// Published assets in discovery order.
    assets: Vec<ExportedProp>,
    /// Semantic identity to published asset index.
    identities: BTreeMap<
        (
            PropFamily,
            String,
        ),
        usize,
    >,
    /// Family-local public name to semantic signature.
    public_names: BTreeMap<
        (
            PropFamily,
            String,
        ),
        String,
    >,
}

impl ExportState {
    /// Prepare and account for one discovered source occurrence.
    fn publish_candidate(
        &mut self,
        candidate: &PropCandidate,
        normalized_root: &Path,
        scratch_root: &Path,
        output_root: &Path,
        ordinal: usize,
    ) -> Result<(), PipelineError> {
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
        if let Some(existing) = self
            .identities
            .get(&key)
            .copied()
        {
            let asset = self
                .assets
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
            self.publish_unique(
                candidate,
                prepared,
                key,
                output_root,
            )?;
        }
        fs::remove_dir_all(scratch_root.join(format!("candidate-{ordinal:06}")))
            .map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "prop material scratch cleanup failed: {error}"
                        ),
                    )
                },
            )
    }

    /// Publish one semantic identity and register its family-local name.
    fn publish_unique(
        &mut self,
        candidate: &PropCandidate,
        prepared: PreparedProp,
        identity: (
            PropFamily,
            String,
        ),
        output_root: &Path,
    ) -> Result<(), PipelineError> {
        let public_name = asset_name(&candidate.owner_name)?;
        let public_key = (
            candidate.family,
            public_name.clone(),
        );
        if let Some(existing_signature) = self
            .public_names
            .get(&public_key)
            && existing_signature != &prepared.signature
        {
            return Err(
                PipelineError::new(
                    format!(
                        "non-world prop name collision for {public_name} in {}",
                        candidate
                            .family
                            .as_str()
                    ),
                ),
            );
        }
        let index = self
            .assets
            .len();
        let exported = write_unique_prop(
            candidate,
            prepared,
            &public_name,
            output_root,
        )?;
        let _previous_public_name = self
            .public_names
            .insert(
                public_key,
                exported
                    .signature
                    .clone(),
            );
        let _previous_identity = self
            .identities
            .insert(
                identity, index,
            );
        self.assets
            .push(exported);
        Ok(())
    }

    /// Sort aliases and assets into canonical catalog order.
    fn finish(mut self) -> Vec<ExportedProp> {
        for asset in &mut self.assets {
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
        self.assets
            .sort_by(
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
        self.assets
    }
}

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
    let mut state = ExportState::default();
    for (ordinal, candidate) in candidates
        .iter()
        .enumerate()
    {
        state.publish_candidate(
            candidate,
            normalized_root,
            scratch_root,
            output_root,
            ordinal,
        )?;
    }
    Ok(state.finish())
}

/// Publish every external texture referenced by one prepared asset.
fn publish_textures(
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
                    format!(
                        "prop texture publication failed for {}: {error}",
                        texture.file_name
                    ),
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
                                "prop texture byte count overflowed: {error}"
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

/// Write one unique prepared model and construct its catalog record.
fn write_unique_prop(
    candidate: &PropCandidate,
    prepared: PreparedProp,
    asset_id: &str,
    output_root: &Path,
) -> Result<ExportedProp, PipelineError> {
    let relative_dir = format!(
        "{}/{asset_id}",
        candidate
            .family
            .as_str()
    );
    let directory = output_root
        .join(
            candidate
                .family
                .as_str(),
        )
        .join(asset_id);
    let texture_dir = directory.join("textures");
    fs::create_dir_all(&texture_dir).map_err(
        |error| {
            PipelineError::new(format!("prop asset directory failed: {error}"))
        },
    )?;
    let texture_records = publish_textures(
        &prepared,
        &texture_dir,
    )?;
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
                |error| {
                    PipelineError::new(
                        format!("prop FBX byte count overflowed: {error}"),
                    )
                },
            )?,
            fbx_sha256: digest_hex(&fbx_bytes),
            summary,
            textures: texture_records,
            aliases: vec![PropAlias::from(candidate)],
        },
    )
}

/// Build one readable portable non-world prop name.
///
/// # Errors
///
/// Returns an error when the source identity has no portable characters.
fn asset_name(owner_name: &str) -> Result<String, PipelineError> {
    portable_asset_name(
        owner_name,
        48,
        "non-world prop identity has no portable name",
    )
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
