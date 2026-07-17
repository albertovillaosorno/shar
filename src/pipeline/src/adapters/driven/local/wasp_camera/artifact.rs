// File:
//   - artifact.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/wasp_camera/artifact.rs
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
//   - Wasp Camera material staging, scope verification, and file inventory.
// - Must-Not:
//   - Select index members, assemble the rig, or publish output directories.
// - Allows:
//   - Resolve selected mesh shaders and verify the standalone FBX contract.
// - Split-When:
//   - Material staging and artifact verification gain separate lifecycles.
// - Merge-When:
//   - Generic animated-prop verification owns the same proven invariants.
// - Summary:
//   - Verifies and inventories the canonical Wasp Camera artifact.
// - Description:
//   - Enforces body-only scope, summary cardinality, and external texture
//     files.
// - Usage:
//   - Called by the sibling Wasp Camera assembly transaction.
// - Defaults:
//   - Effects, explosions, shield, ray, state, and placement identities fail.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - true
//   - Reason: Material resolution and observable artifact verification remain
//   - coupled by the exact external-texture FBX output contract.
//

//! Wasp Camera material and artifact verification.
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

/// Resolve every shader used by the selected body meshes.
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
                                    "Wasp Camera material {shader} failed: \
                                     {error:?}"
                                ),
                            )
                        },
                    )
            },
        )
        .collect()
}

/// Verify the selected body and pruned rig before serialization.
pub(super) fn verify_body_scope(
    asset: &CharacterAsset
) -> Result<(), PipelineError> {
    if asset
        .parts
        .len()
        != BODY_MESH_MEMBERS.len()
    {
        return Err(PipelineError::new("Wasp Camera body mesh count changed"));
    }
    if asset
        .bones
        .len()
        != EXPECTED_BONES
    {
        return Err(
            PipelineError::new("Wasp Camera retained bone count changed"),
        );
    }
    let banned = [
        "explosion",
        "shockwave",
        "shield",
        "waspray",
        "particle",
        "glow",
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
                "Wasp Camera body scope included an FX component",
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

/// Verify the written binary FBX summary remains inside the standalone scope.
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
                format!("Wasp Camera FBX summary changed: {summary:?}"),
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
            PipelineError::new(
                format!("Wasp Camera FBX inventory failed: {error}"),
            )
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
                || PipelineError::new("Wasp Camera file count overflowed"),
            )?;
        bytes = bytes
            .checked_add(
                file_len(&texture_dir.join(texture_name)).map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "Wasp Camera texture inventory failed: {error}"
                            ),
                        )
                    },
                )?,
            )
            .ok_or_else(
                || PipelineError::new("Wasp Camera byte total overflowed"),
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
    fn standalone_scope_rejects_effect_identities() {
        let banned = [
            "explosion",
            "shockwave",
            "shield",
            "waspray",
            "particle",
            "glow",
        ];

        assert!(
            contains_banned_token(
                "head_explosionShape",
                &banned
            )
        );
        assert!(
            contains_banned_token(
                "WaspShield",
                &banned
            )
        );
        assert!(
            !contains_banned_token(
                "wasp_armShape5",
                &banned
            )
        );
        assert!(
            !contains_banned_token(
                "Wing_LShape",
                &banned
            )
        );
    }
}
