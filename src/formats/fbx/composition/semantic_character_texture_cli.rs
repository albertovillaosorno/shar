// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Semantic character texture cli composition module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Semantic character texture cli composition module.
// - Description:
//   - Implements the declared composition module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Semantic character texture cli composition module.

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
        },
        Err(error) => {
            eprintln!("semantic-character-texture: {error}");
            ExitCode::FAILURE
        },
    }
}

/// Parse arguments, build all bytes, and publish one new output directory.
fn run() -> Result<String, String> {
    let arguments = env::args_os().skip(1).collect::<Vec<_>>();
    let [request_argument, output_argument] = arguments.as_slice() else {
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
    let _summary =
        publish_prepared_semantic_character(&output_path, &prepared)?;
    let artifacts = &prepared.artifacts;
    serde_json::to_string(&serde_json::json!({
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
    }))
    .map_err(|error| format!("summary JSON failed: {error}"))
}
