// File:
//   - publication.rs
// Path:
//   - src/fbx/src/adapters/driven/semantic_character_texture/publication.rs
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
//   - Transactional publication of one prepared external-texture character.
// - Must-Not:
//   - Discover assets, infer semantic groups, overwrite outputs, or invoke
//     tools.
// - Allows:
//   - Hidden staging writes, one directory rename, and failure cleanup.
// - Split-When:
//   - Another artifact family requires a distinct publication transaction.
// - Merge-When:
//   - The semantic preparation transaction also owns filesystem publication.
// - Summary:
//   - Prepared semantic character publication adapter.
// - Description:
//   - Publishes the FBX, textures, and texture-plan manifest atomically.
// - Usage:
//   - Called by the focused CLI and the deterministic character catalog batch.
// - Defaults:
//   - Existing output and staging directories are rejected.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Transactional prepared semantic character publication.
use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local::{
    create_dir_all, path_kind, write_bytes,
};
use schoenwald_filesystem::domain::PathKind;

use super::PreparedSemanticCharacter;
use crate::adapters::driven::binary_character_writer::{
    CharacterBinaryFbxSummary, write_binary_character_fbx,
};

/// Publish one prepared character through a hidden sibling staging directory.
///
/// # Errors
///
/// Returns an error when output already exists, any artifact write fails, the
/// FBX serializer rejects the aggregate, or the final rename fails.
pub fn publish_prepared_semantic_character(
    output: &Path,
    prepared: &PreparedSemanticCharacter,
) -> Result<CharacterBinaryFbxSummary, String> {
    ensure_missing(
        output, "output",
    )?;
    let staging = staging_path(output)?;
    ensure_missing(
        &staging, "staging",
    )?;
    create_dir_all(&staging)
        .map_err(|error| format!("staging create failed: {error}"))?;
    let result = write_artifacts(
        &staging, prepared,
    )
    .and_then(
        |summary| {
            std::fs::rename(
                &staging, output,
            )
            .map_err(|error| format!("output publish failed: {error}"))?;
            Ok(summary)
        },
    );
    if result.is_err() {
        let _cleanup_result = std::fs::remove_dir_all(&staging);
    }
    result
}

/// Write the complete prepared artifact set below one staging directory.
fn write_artifacts(
    staging: &Path,
    prepared: &PreparedSemanticCharacter,
) -> Result<CharacterBinaryFbxSummary, String> {
    let artifacts = &prepared.artifacts;
    let textures = staging.join("textures");
    create_dir_all(&textures)
        .map_err(|error| format!("texture directory create failed: {error}"))?;
    write(
        &textures.join("body-atlas.png"),
        &artifacts.body_texture_png,
    )?;
    if let Some(
        [
            eye,
            pupil,
            lids,
        ],
    ) = artifacts
        .eye_layer_pngs
        .as_ref()
    {
        for (file_name, bytes) in [
            (
                "eye.png", eye,
            ),
            (
                "eye-pupil.png",
                pupil,
            ),
            (
                "eye-lids.png",
                lids,
            ),
        ] {
            write(
                &textures.join(file_name),
                bytes,
            )?;
        }
    }
    for extra in &artifacts.extra_textures {
        write(
            &textures.join(&extra.file_name),
            &extra.png,
        )?;
    }
    let fbx_path = staging.join(
        format!(
            "{}.fbx",
            artifacts
                .summary
                .character_id
        ),
    );
    let summary = write_binary_character_fbx(
        &prepared.character,
        &prepared.materials,
        &prepared.animations,
        &fbx_path,
    )
    .map_err(|error| format!("prepared FBX write failed: {error:?}"))?;
    write(
        &staging.join("texture-plan.json"),
        &artifacts.manifest_json,
    )?;
    Ok(summary)
}

/// Write one artifact without parent creation because staging already exists.
fn write(
    path: &Path,
    bytes: &[u8],
) -> Result<(), String> {
    write_bytes(
        path, bytes, false,
    )
    .map_err(|error| format!("artifact write failed: {error}"))
}

/// Require one path to be absent before a fail-closed publication transaction.
fn ensure_missing(
    path: &Path,
    role: &str,
) -> Result<(), String> {
    match path_kind(path)
        .map_err(|error| format!("{role} path inspection failed: {error}"))?
    {
        PathKind::Missing => Ok(()),
        kind => Err(format!("{role} path already exists as {kind:?}")),
    }
}

/// Derive one hidden sibling staging identity from the output leaf name.
fn staging_path(output: &Path) -> Result<PathBuf, String> {
    let name = output
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(
            || "output directory requires a UTF-8 leaf name".to_owned(),
        )?;
    Ok(output.with_file_name(format!(".{name}.textures-staging")))
}
