// File:
//   - blender_review_helper.rs
// Path:
//   - src/fbx/src/adapters/driven/blender_review_helper.rs
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
//   - Deterministic optional Blender review-sidecar generation for FBX files.
// - Must-Not:
//   - Invoke Blender, mutate FBX bytes, or alter source animation semantics.
// - Allows:
//   - Materialize one validated Python template beside an FBX artifact.
// - Split-When:
//   - Template validation and artifact writing gain separate release cycles.
// - Merge-When:
//   - Another Blender adapter owns the same review-sidecar contract.
// - Summary:
//   - Writes typed Blender review helpers from native clip timing evidence.
// - Description:
//   - Substitutes only portable identity and integer timing constants in a
//   - Python template validated independently by Ruff and BasedPyright.
// - Usage:
//   - Requested explicitly by phase-three `fbx-export`.
// - Defaults:
//   - Native FBX timing and imported keyframes remain unchanged.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Deterministic Blender review-sidecar generation.
//!
//! The adapter materializes a tracked Python template beside one FBX artifact
//! only when the caller explicitly requests the experimental unsupported
//! Blender convenience. The template is independently validated by Ruff and
//! BasedPyright, while this module owns only portable identity and integer
//! frame-rate substitution.
//!
//! Native FBX clip timing remains authoritative. The generated helper imports
//! the sibling artifact, selects its armature and one Action, and preserves the
//! imported keyframes, interpolation, frame ranges, and source frame rate. The
//! standalone file carries one explicit `INP001` exception instead of emitting
//! repeated package-marker boilerplate.

use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::domain::animation::AnimationClip;
use crate::domain::animation::clip::frame_rates_match;
use crate::domain::scene::identity::is_windows_reserved_name;

/// Strict typed Python template included in each generated helper.
const HELPER_TEMPLATE: &str =
    include_str!("../../../python/shar_blender_review/helper_template.py");
/// Template line replaced with the portable sibling FBX name.
const FILE_NAME_MARKER: &str =
    "FBX_FILE_NAME: str = \"__SHAR_FBX_FILE_NAME__\"";
/// Template line replaced with the native integer frame rate.
const SOURCE_FPS_MARKER: &str = "SOURCE_FPS: int = 1";

/// Deterministic helper-generation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HelperError {
    /// The FBX path had no supported portable file name.
    InvalidFbxFileName,
    /// No skeletal clips were available to establish source timing.
    MissingAnimations,
    /// Source clips did not share one frame rate.
    MixedFrameRate,
    /// Source timing could not be represented safely.
    InvalidFrameRate,
    /// The validated template no longer contains its substitution contract.
    TemplateContract,
    /// One generated Python file could not be written.
    Write {
        /// Destination path rendered for diagnostics.
        path: String,
        /// Filesystem failure rendered without platform-specific types.
        source: String,
    },
}

/// Deterministic summary of one generated standalone Blender helper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HelperSummary {
    /// Integer source frame rate preserved by the FBX.
    pub source_fps: u32,
    /// Standalone Python files written by the helper.
    pub files: usize,
}

/// Write one typed standalone Blender helper beside an FBX artifact.
///
/// # Errors
///
/// Returns an error when timing evidence is absent or mixed, the FBX name is
/// not portable, the template contract drifts, or the helper cannot be
/// written.
pub fn write_review_helper(
    fbx_path: &Path,
    clips: &[AnimationClip],
    output_path: &Path,
) -> Result<HelperSummary, HelperError> {
    let fbx_file_name = portable_fbx_file_name(fbx_path)?;
    let source_fps = shared_integer_frame_rate(clips)?;
    let script = render_script(
        fbx_file_name,
        source_fps,
    )?;
    write_python(
        output_path,
        &script,
    )?;
    Ok(
        HelperSummary {
            source_fps,
            files: 1,
        },
    )
}

/// Return one portable FBX file name safe for Python substitution.
fn portable_fbx_file_name(path: &Path) -> Result<&str, HelperError> {
    let value = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(HelperError::InvalidFbxFileName)?;
    let extension_is_fbx = path
        .extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("fbx"));
    let valid = extension_is_fbx
        && !value.starts_with('.')
        && !is_windows_reserved_name(value)
        && value
            .chars()
            .all(
                |character| {
                    character.is_ascii_alphanumeric()
                        || matches!(
                            character,
                            '.' | '-' | '_'
                        )
                },
            );
    if !valid {
        return Err(HelperError::InvalidFbxFileName);
    }
    Ok(value)
}

/// Require one positive integer frame rate shared by every clip.
fn shared_integer_frame_rate(
    clips: &[AnimationClip]
) -> Result<u32, HelperError> {
    let first = clips
        .first()
        .ok_or(HelperError::MissingAnimations)?
        .frame_rate;
    if clips
        .iter()
        .any(
            |clip| {
                !frame_rates_match(
                    clip.frame_rate,
                    first,
                )
            },
        )
    {
        return Err(HelperError::MixedFrameRate);
    }
    if !first.is_finite()
        || first <= 0.0_f64
        || first
            .fract()
            .abs()
            > f64::EPSILON
        || first > f64::from(u32::MAX)
    {
        return Err(HelperError::InvalidFrameRate);
    }
    match first
        .to_string()
        .parse::<u32>()
    {
        Ok(frame_rate) => Ok(frame_rate),
        Err(_error) => Err(HelperError::InvalidFrameRate),
    }
}

/// Substitute only the two typed constants owned by the generator.
fn render_script(
    fbx_file_name: &str,
    source_fps: u32,
) -> Result<String, HelperError> {
    if !HELPER_TEMPLATE.contains(FILE_NAME_MARKER)
        || !HELPER_TEMPLATE.contains(SOURCE_FPS_MARKER)
    {
        return Err(HelperError::TemplateContract);
    }
    let file_name = format!("FBX_FILE_NAME: str = {fbx_file_name:?}");
    let source = format!("SOURCE_FPS: int = {source_fps}");
    Ok(
        HELPER_TEMPLATE
            .replacen(
                FILE_NAME_MARKER,
                &file_name,
                1,
            )
            .replacen(
                SOURCE_FPS_MARKER,
                &source,
                1,
            ),
    )
}

/// Write one complete generated Python document with stable line endings.
fn write_python(
    path: &Path,
    text: &str,
) -> Result<(), HelperError> {
    local::write_text(
        path, text, true,
    )
    .map_err(
        |error| HelperError::Write {
            path: path
                .display()
                .to_string(),
            source: error.to_string(),
        },
    )
}

#[cfg(test)]
#[test]
fn rejects_near_equal_clip_rates() {
    let clips = [
        AnimationClip {
            name: "first".to_owned(),
            frame_rate: 30.0_f64,
            cyclic: false,
            frame_count: 1,
            tracks: Vec::new(),
            ignored_group_ids: Vec::new(),
        },
        AnimationClip {
            name: "second".to_owned(),
            frame_rate: 30.000_000_000_5_f64,
            cyclic: false,
            frame_count: 1,
            tracks: Vec::new(),
            ignored_group_ids: Vec::new(),
        },
    ];

    assert_eq!(
        shared_integer_frame_rate(&clips),
        Err(HelperError::MixedFrameRate)
    );
}

#[cfg(test)]
#[test]
fn rejects_windows_reserved_fbx_name() {
    assert_eq!(
        portable_fbx_file_name(Path::new("CON.fbx")),
        Err(HelperError::InvalidFbxFileName)
    );
}
