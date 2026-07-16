// File:
//   - semantic_character_texture_cli.rs
// Path:
//   - src/fbx/src/semantic_character_texture_cli.rs
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
//   - Command-line composition and transactional filesystem publication for one
//   - explicit semantic character texture request.
// - Must-Not:
//   - Discover assets, overwrite output, classify semantics, bulk-export
//   - characters, or invoke external content-authoring applications.
// - Allows:
//   - JSON request parsing, in-memory artifact building, hidden staging writes,
//   - one directory rename, cleanup after failure, and summary output.
// - Split-When:
//   - Publication policy becomes reusable by another artifact family.
// - Merge-When:
//   - Another composition root owns the same command and transaction.
// - Summary:
//   - Transactional semantic character texture CLI.
// - Description:
//   - Publishes one external-texture FBX, one body texture, three eye images,
//   - optional explicit material textures, and one manifest atomically.
// - Usage:
//   - `semantic-character-texture <request.json> <new-output-directory>`.
// - Defaults:
//   - Existing output or staging directories are rejected.
//
// ADRs:
// - docs/adr/fbx/export/character-semantic-texture-rig-and-outfit-contract.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - true
//   - Reason: CLI parsing, staging identity, write set, rename, cleanup, and
//   - summary output form one bounded composition-root transaction.
//

//! Transactional semantic character texture artifact CLI.
use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use fbx::adapters::driven::binary_character_writer::write_binary_character_fbx;
use fbx::adapters::driven::semantic_character_texture::{
    PreparedSemanticCharacter, SemanticTextureRequest,
    prepare_semantic_character,
};
use schoenwald_filesystem::adapters::driving::local::{
    create_dir_all, path_kind, read_utf8, write_bytes,
};
use schoenwald_filesystem::domain::PathKind;

/// Fixed CLI usage contract.
const USAGE: &str =
    "semantic-character-texture <request.json> <new-output-directory>";

fn main() -> ExitCode {
    match run() {
        Ok(summary) => {
            println!("{summary}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("semantic-character-texture: {error}");
            ExitCode::FAILURE
        }
    }
}

/// Parse arguments, build all bytes, and publish one new output directory.
fn run() -> Result<String, String> {
    let arguments = env::args_os()
        .skip(1)
        .collect::<Vec<_>>();
    if arguments.len() != 2 {
        return Err(format!("usage: {USAGE}"));
    }
    let request_path = PathBuf::from(&arguments[0]);
    let output_path = PathBuf::from(&arguments[1]);
    let request_text = read_utf8(&request_path)
        .map_err(|error| format!("request read failed: {error}"))?;
    let request: SemanticTextureRequest =
        serde_json::from_str(&request_text)
            .map_err(|error| format!("request JSON failed: {error}"))?;
    let prepared = prepare_semantic_character(&request)
        .map_err(|error| format!("preparation failed: {error:?}"))?;
    publish(
        &output_path,
        &prepared,
    )?;
    let artifacts = &prepared.artifacts;
    serde_json::to_string(
        &serde_json::json!({
            "character_id": artifacts.summary.character_id,
            "body_vertex_count": artifacts.summary.body_vertex_count,
            "body_triangle_count": artifacts.summary.body_triangle_count,
            "body_chart_count": artifacts.summary.body_chart_count,
            "eye_region_count": artifacts.summary.eye_region_count,
            "animation_count": artifacts.summary.animation_count,
            "body_texture_size": artifacts.summary.body_texture_size,
            "eye_frame_size": artifacts.summary.eye_frame_size,
            "eye_profile_sha256": artifacts.eye_profile_sha256,
            "fbx": format!("{}.fbx", artifacts.summary.character_id),
            "output": output_path,
        }),
    )
    .map_err(|error| format!("summary JSON failed: {error}"))
}

/// Publish all artifacts through one hidden staging directory rename.
fn publish(
    output: &Path,
    prepared: &PreparedSemanticCharacter,
) -> Result<(), String> {
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
        |()| {
            std::fs::rename(
                &staging, output,
            )
            .map_err(|error| format!("output publish failed: {error}"))
        },
    );
    if result.is_err() {
        let _cleanup_result = std::fs::remove_dir_all(&staging);
    }
    result
}

/// Write the complete fixed artifact set below one staging directory.
fn write_artifacts(
    staging: &Path,
    prepared: &PreparedSemanticCharacter,
) -> Result<(), String> {
    let artifacts = &prepared.artifacts;
    let textures = staging.join("textures");
    create_dir_all(&textures)
        .map_err(|error| format!("texture directory create failed: {error}"))?;
    write(
        &textures.join("body-atlas.png"),
        &artifacts.body_texture_png,
    )?;
    for (file_name, bytes) in [
        (
            "eye.png",
            &artifacts.eye_layer_pngs[0],
        ),
        (
            "eye-pupil.png",
            &artifacts.eye_layer_pngs[1],
        ),
        (
            "eye-lids.png",
            &artifacts.eye_layer_pngs[2],
        ),
    ] {
        write(
            &textures.join(file_name),
            bytes,
        )?;
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
    write_binary_character_fbx(
        &prepared.character,
        &prepared.materials,
        &prepared.animations,
        &fbx_path,
    )
    .map_err(|error| format!("prepared FBX write failed: {error:?}"))?;
    write(
        &staging.join("texture-plan.json"),
        &artifacts.manifest_json,
    )
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
