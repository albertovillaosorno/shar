// File:
//   - artifact.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/wrench/artifact.rs
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
//   - Wrench model-scope, material, summary, and inventory verification.
// - Must-Not:
//   - Accept collection presentation, effects, or non-model package members.
// - Allows:
//   - Decoded material resolution and deterministic file accounting.
// - Split-When:
//   - Material staging or scope validation becomes independently reusable.
// - Merge-When:
//   - A stable generic model-prop artifact verifier owns the same contract.
// - Summary:
//   - Proves the Wrench FBX contains only the visible original model.
// - Description:
//   - Fixes exact counts for mesh, rig, material, texture, cluster, and clip.
// - Usage:
//   - Called before and after Wrench binary FBX serialization.
// - Defaults:
//   - One mesh group, four retained bones, one material, and one texture.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Wrench model-scope and artifact verification.

use std::collections::BTreeSet;
use std::path::Path;

use fbx::adapters::driven::binary_character_writer::CharacterBinaryFbxSummary;
use fbx::adapters::driven::decoded_component_source::DecodedComponentSource;
use fbx::domain::character::CharacterAsset;
use fbx::domain::texture::MaterialBinding;
use fbx::ports::component_source::ComponentSource as _;
use schoenwald_filesystem::adapters::driving::local::file_len;

use super::{
    BODY_MESH_MEMBERS, EXPECTED_BONES, EXPECTED_CLUSTERS, EXPECTED_GEOMETRIES,
    EXPECTED_MATERIALS, EXPECTED_TEXTURE_BINDINGS,
};
use crate::domain::PipelineError;

/// Resolve every shader used by the selected model mesh.
pub(super) fn resolve_materials(
    asset: &CharacterAsset,
    package_root: &Path,
    texture_dir: &Path,
) -> Result<Vec<MaterialBinding>, PipelineError> {
    let source = DecodedComponentSource::new(
        package_root,
        texture_dir,
    );
    asset
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
                    .as_str()
            },
        )
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(
            |shader| {
                source
                    .resolve_material(shader)
                    .map_err(
                        |error| {
                            PipelineError::new(
                                format!(
                                    "Wrench material {shader} failed: \
                                     {error:?}"
                                ),
                            )
                        },
                    )
            },
        )
        .collect()
}

/// Verify the selected model and pruned rig before serialization.
pub(super) fn verify_model_scope(
    asset: &CharacterAsset
) -> Result<(), PipelineError> {
    if asset
        .parts
        .len()
        != BODY_MESH_MEMBERS.len()
        || asset
            .parts
            .first()
            .is_none_or(
                |part| {
                    part.mesh
                        .name
                        != "wrench7Shape"
                },
            )
    {
        return Err(PipelineError::new("Wrench model selection changed"));
    }
    if asset
        .bones
        .len()
        != EXPECTED_BONES
    {
        return Err(
            PipelineError::new(
                format!(
                    "Wrench retained bone count changed: {} {:?}",
                    asset
                        .bones
                        .len(),
                    asset
                        .bones
                        .iter()
                        .map(
                            |bone| bone
                                .id
                                .as_str()
                        )
                        .collect::<Vec<_>>()
                ),
            ),
        );
    }
    let banned = [
        "collect", "glow", "particle", "effect", "quad",
    ];
    if asset
        .parts
        .iter()
        .any(
            |part| {
                contains_banned_token(
                    &part
                        .mesh
                        .name,
                    &banned,
                )
            },
        )
        || asset
            .bones
            .iter()
            .any(
                |bone| {
                    contains_banned_token(
                        &bone.id, &banned,
                    )
                },
            )
    {
        return Err(
            PipelineError::new(
                "Wrench model scope included collection or effect evidence",
            ),
        );
    }
    Ok(())
}

/// Return whether one identity contains a forbidden standalone-export token.
fn contains_banned_token(
    identity: &str,
    banned: &[&str],
) -> bool {
    let normalized = identity.to_ascii_lowercase();
    banned
        .iter()
        .any(|token| normalized.contains(token))
}

/// Verify the written binary FBX summary remains inside the model scope.
pub(super) fn verify_summary(
    summary: &CharacterBinaryFbxSummary
) -> Result<(), PipelineError> {
    if summary.animations != 1
        || summary.bones != EXPECTED_BONES
        || summary.geometries != EXPECTED_GEOMETRIES
        || summary.clusters != EXPECTED_CLUSTERS
        || summary.materials != EXPECTED_MATERIALS
        || summary.textures != EXPECTED_TEXTURE_BINDINGS
    {
        return Err(
            PipelineError::new(
                format!("Wrench FBX summary changed: {summary:?}"),
            ),
        );
    }
    Ok(())
}

/// Inventory the FBX and each unique referenced external texture.
pub(super) fn inventory(
    fbx_path: &Path,
    texture_dir: &Path,
    materials: &[MaterialBinding],
) -> Result<
    (
        usize,
        u64,
    ),
    PipelineError,
> {
    let mut files = 1_usize;
    let mut bytes = file_len(fbx_path).map_err(
        |error| {
            PipelineError::new(format!("Wrench FBX inventory failed: {error}"))
        },
    )?;
    let texture_names = materials
        .iter()
        .filter_map(
            |binding| {
                binding
                    .texture_file_name
                    .as_deref()
            },
        )
        .collect::<BTreeSet<_>>();
    for texture_name in texture_names {
        files = files
            .checked_add(1)
            .ok_or_else(
                || PipelineError::new("Wrench file count overflowed"),
            )?;
        bytes = bytes
            .checked_add(
                file_len(&texture_dir.join(texture_name)).map_err(
                    |error| {
                        PipelineError::new(
                            format!("Wrench texture inventory failed: {error}"),
                        )
                    },
                )?,
            )
            .ok_or_else(
                || PipelineError::new("Wrench byte total overflowed"),
            )?;
    }
    Ok(
        (
            files, bytes,
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::contains_banned_token;

    #[test]
    fn standalone_scope_rejects_collection_effect_identities() {
        let banned = [
            "collect", "glow", "particle", "effect", "quad",
        ];
        assert!(
            contains_banned_token(
                "wrench_collect",
                &banned
            )
        );
        assert!(
            contains_banned_token(
                "circelglowShape",
                &banned
            )
        );
        assert!(
            !contains_banned_token(
                "wrench7Shape",
                &banned
            )
        );
        assert!(
            !contains_banned_token(
                "wrench46", &banned
            )
        );
    }
}
