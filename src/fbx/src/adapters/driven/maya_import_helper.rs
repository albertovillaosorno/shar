// File:
//   - maya_import_helper.rs
// Path:
//   - src/fbx/src/adapters/driven/maya_import_helper.rs
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
//   - Deterministic optional Maya import-script generation for one FBX file.
// - Must-Not:
//   - Invoke Maya, rewrite FBX bytes, or emit alternate FBX/DAE/MA/MB assets.
// - Allows:
//   - Materialize one portable Python script beside the canonical binary FBX
//   - and configure Maya to the exported animation frame rate.
// - Split-When:
//   - Maya import policy and script persistence gain independent lifecycles.
// - Merge-When:
//   - Another Maya adapter owns the same sibling-FBX import contract.
// - Summary:
//   - Writes a Maya helper that imports the canonical binary FBX 7.7 artifact.
// - Description:
//   - Substitutes one validated sibling file name and optional animation time
//   - unit into a fixed Maya Python template without invoking Maya.
// - Usage:
//   - Requested explicitly through the phase-three `fbx-export --maya` option.
// - Defaults:
//   - The normal export remains FBX plus textures plus JSON only.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Deterministic optional Maya import-script generation.
//!
//! The helper loads Maya's FBX plug-in and imports the sibling canonical binary
//! FBX 7.7 file. It never creates a second FBX representation and does not save
//! a Maya-native scene automatically.

use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use crate::domain::animation::clip::frame_rates_match;
use crate::domain::scene::identity::is_windows_reserved_name;

/// Marker replaced with the validated sibling FBX file name.
const FILE_NAME_MARKER: &str = "__SHAR_FBX_FILE_NAME__";
/// Marker replaced with the validated Maya animation time unit.
const TIME_UNIT_MARKER: &str = "__SHAR_MAYA_TIME_UNIT__";
/// Standard Maya time-unit names keyed by frames per second.
const STANDARD_TIME_UNITS: [(
    f64,
    &str,
); 7] = [
    (
        15.0, "game",
    ),
    (
        24.0, "film",
    ),
    (
        25.0, "pal",
    ),
    (
        30.0, "ntsc",
    ),
    (
        48.0, "show",
    ),
    (
        50.0, "palf",
    ),
    (
        60.0, "ntscf",
    ),
];
/// Fixed import script kept independent from machine-local Maya paths.
const MAYA_TEMPLATE: &str = r#"from pathlib import Path

import maya.cmds as cmds

FBX_FILE_NAME = "__SHAR_FBX_FILE_NAME__"
MAYA_TIME_UNIT = "__SHAR_MAYA_TIME_UNIT__"


def import_shar_fbx() -> None:
    """Import the canonical sibling FBX without saving a Maya scene."""
    script_directory = Path(__file__).resolve().parent
    fbx_path = script_directory / FBX_FILE_NAME
    if not fbx_path.is_file():
        raise FileNotFoundError(f"FBX artifact not found: {fbx_path}")
    if not cmds.pluginInfo("fbxmaya", query=True, loaded=True):
        cmds.loadPlugin("fbxmaya", quiet=True)
    if MAYA_TIME_UNIT:
        cmds.currentUnit(time=MAYA_TIME_UNIT, updateAnimation=False)
    cmds.file(
        str(fbx_path),
        i=True,
        type="FBX",
        ignoreVersion=True,
        mergeNamespacesOnClash=False,
        namespace=":",
        options="fbx",
        preserveReferences=True,
    )


import_shar_fbx()
"#;

/// Deterministic Maya helper generation failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The FBX path had no supported portable file name.
    InvalidFbxFileName,
    /// The requested animation frame rate cannot map to a Maya time unit.
    InvalidFrameRate,
    /// The fixed script template lost its substitution marker.
    TemplateContract,
    /// The generated Python file could not be written.
    Write {
        /// Destination path rendered for diagnostics.
        path: String,
        /// Filesystem failure rendered without platform-specific types.
        source: String,
    },
}

/// Deterministic summary of one generated Maya import helper.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Summary {
    /// Standalone Python files written by the helper.
    pub files: usize,
}

/// Write one standalone Maya import helper beside an FBX artifact.
///
/// # Errors
///
/// Returns an error when the FBX file name or frame rate is invalid, the
/// template drifts, or the helper cannot be written.
pub fn write(
    fbx_path: &Path,
    frame_rate: Option<f64>,
    output_path: &Path,
) -> Result<Summary, Error> {
    let fbx_file_name = portable_fbx_file_name(fbx_path)?;
    let time_unit = maya_time_unit(frame_rate)?;
    let script = render_script(
        fbx_file_name,
        &time_unit,
    )?;
    local::write_text(
        output_path,
        &script,
        true,
    )
    .map_err(
        |error| Error::Write {
            path: output_path
                .display()
                .to_string(),
            source: error.to_string(),
        },
    )?;
    Ok(
        Summary {
            files: 1,
        },
    )
}

/// Return one portable sibling FBX file name safe for Python substitution.
fn portable_fbx_file_name(path: &Path) -> Result<&str, Error> {
    let value = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or(Error::InvalidFbxFileName)?;
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
        return Err(Error::InvalidFbxFileName);
    }
    Ok(value)
}

/// Convert one optional positive frame rate to a Maya time-unit name.
fn maya_time_unit(frame_rate: Option<f64>) -> Result<String, Error> {
    let Some(source_frame_rate) = frame_rate else {
        return Ok(String::new());
    };
    if !source_frame_rate.is_finite() || source_frame_rate <= 0.0_f64 {
        return Err(Error::InvalidFrameRate);
    }
    for (standard_rate, unit) in STANDARD_TIME_UNITS {
        if frame_rates_match(
            source_frame_rate,
            standard_rate,
        ) {
            return Ok(unit.to_owned());
        }
    }
    Ok(format!("{source_frame_rate}fps"))
}

/// Substitute only the validated sibling FBX identity and Maya time unit.
fn render_script(
    fbx_file_name: &str,
    time_unit: &str,
) -> Result<String, Error> {
    if MAYA_TEMPLATE
        .matches(FILE_NAME_MARKER)
        .count()
        != 1
        || MAYA_TEMPLATE
            .matches(TIME_UNIT_MARKER)
            .count()
            != 1
    {
        return Err(Error::TemplateContract);
    }
    let with_file_name = MAYA_TEMPLATE.replacen(
        FILE_NAME_MARKER,
        fbx_file_name,
        1,
    );
    Ok(
        with_file_name.replacen(
            TIME_UNIT_MARKER,
            time_unit,
            1,
        ),
    )
}

#[cfg(test)]
#[test]
fn near_standard_rate_does_not_use_builtin_unit() {
    assert_ne!(
        maya_time_unit(Some(24.000_000_000_5_f64)),
        Ok("film".to_owned())
    );
}

#[cfg(test)]
#[test]
fn custom_rate_preserves_round_trip_precision() {
    assert_eq!(
        maya_time_unit(Some(23.976_543_21_f64)),
        Ok("23.97654321fps".to_owned())
    );
}

#[cfg(test)]
#[test]
fn rejects_windows_reserved_fbx_name() {
    assert_eq!(
        portable_fbx_file_name(Path::new("CON.fbx")),
        Err(Error::InvalidFbxFileName)
    );
}
