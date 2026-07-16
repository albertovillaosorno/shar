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
use std::path::PathBuf;
use std::process::ExitCode;

use fbx::adapters::driven::semantic_character_texture::{
    SemanticTextureRequest, prepare_semantic_character,
    publish_prepared_semantic_character,
};
use png as _;
use schoenwald_filesystem::adapters::driving::local::read_utf8;
use serde as _;
use shar_sha256 as _;

/// Fixed CLI usage contract.
const USAGE: &str =
    "semantic-character-texture <request.json> <new-output-directory>";

#[expect(
    clippy::print_stdout,
    clippy::print_stderr,
    reason = "The CLI contract writes success and failure diagnostics to \
              standard streams."
)]
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
    let [
        request_argument,
        output_argument,
    ] = arguments.as_slice()
    else {
        return Err(format!("usage: {USAGE}"));
    };
    let request_path = PathBuf::from(request_argument);
    let output_path = PathBuf::from(output_argument);
    let request_text = read_utf8(&request_path)
        .map_err(|error| format!("request read failed: {error}"))?;
    let request: SemanticTextureRequest =
        serde_json::from_str(&request_text)
            .map_err(|error| format!("request JSON failed: {error}"))?;
    let prepared = prepare_semantic_character(&request)
        .map_err(|error| format!("preparation failed: {error:?}"))?;
    let _summary = publish_prepared_semantic_character(
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
