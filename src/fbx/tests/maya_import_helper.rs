// File:
//   - maya_import_helper.rs
// Path:
//   - src/fbx/tests/maya_import_helper.rs
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
//   - Synthetic regressions for deterministic Maya import helpers.
// - Must-Not:
//   - Invoke Maya, read private assets, or save Maya-native scenes.
// - Allows:
//   - Generate scripts in temporary directories and inspect their text.
// - Split-When:
//   - Import behavior gains multiple independently selectable policies.
// - Merge-When:
//   - Another test owns the same script-generation assertions.
// - Summary:
//   - Proves Maya support imports the sole canonical binary FBX artifact.
// - Description:
//   - Guards deterministic script bytes, portable identities, and the absence
//   - of alternate FBX, DAE, MA, or MB output paths.
// - Usage:
//   - Run through the fbx crate test suite.
// - Defaults:
//   - Maya itself remains outside this synthetic test boundary.
//
// ADRs:
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Synthetic Maya import-helper generation regressions.
//!
//! The tests materialize scripts in temporary directories, compare repeated
//! output byte-for-byte, and verify that the helper imports only the canonical
//! sibling FBX. Maya itself and private game assets remain outside the test.
//!
//! Negative cases reject non-FBX and non-portable identities before any output
//! file is created.

use std::fs;
use std::path::{Path, PathBuf};

use fbx::adapters::driven::maya_import_helper::{
    Error as MayaImportHelperError, Summary as MayaImportHelperSummary,
    write as write_maya_import_helper,
};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

fn temp_root(label: &str) -> PathBuf {
    std::env::temp_dir().join(
        format!(
            "fbx-maya-helper-{label}-{}",
            std::process::id()
        ),
    )
}

#[test]
fn writes_deterministic_import_script_for_canonical_fbx() -> Result<(), String>
{
    let first_root = temp_root("first");
    let second_root = temp_root("second");
    fs::create_dir_all(&first_root)
        .map_err(|error| format!("first temp root failed: {error}"))?;
    fs::create_dir_all(&second_root)
        .map_err(|error| format!("second temp root failed: {error}"))?;
    let fbx_path = Path::new("characters-apu-base-model.fbx");
    let first_path = first_root.join("characters-apu-base-model.maya.py");
    let second_path = second_root.join("characters-apu-base-model.maya.py");
    let first = write_maya_import_helper(
        fbx_path,
        Some(30.0_f64),
        &first_path,
    );
    let second = write_maya_import_helper(
        fbx_path,
        Some(30.0_f64),
        &second_path,
    );
    let first_script = fs::read_to_string(&first_path)
        .map_err(|error| format!("first helper read failed: {error}"))?;
    let second_script = fs::read_to_string(&second_path)
        .map_err(|error| format!("second helper read failed: {error}"))?;
    fs::remove_dir_all(&first_root)
        .map_err(|error| format!("first cleanup failed: {error}"))?;
    fs::remove_dir_all(&second_root)
        .map_err(|error| format!("second cleanup failed: {error}"))?;

    let expected = Ok(
        MayaImportHelperSummary {
            files: 1,
        },
    );
    if first != expected || second != expected {
        return Err(
            format!("unexpected Maya helper summaries: {first:?} {second:?}"),
        );
    }
    if first_script != second_script {
        return Err("repeated Maya scripts must be byte-identical".to_owned());
    }
    for required in [
        "FBX_FILE_NAME = \"characters-apu-base-model.fbx\"",
        "MAYA_TIME_UNIT = \"ntsc\"",
        "cmds.pluginInfo(\"fbxmaya\", query=True, loaded=True)",
        "cmds.loadPlugin(\"fbxmaya\", quiet=True)",
        "cmds.currentUnit(time=MAYA_TIME_UNIT, updateAnimation=False)",
        "type=\"FBX\"",
        "ignoreVersion=True",
        "preserveReferences=True",
    ] {
        if !first_script.contains(required) {
            return Err(format!("missing Maya import contract: {required}"));
        }
    }
    let time_unit_position = first_script
        .find("cmds.currentUnit(time=MAYA_TIME_UNIT, updateAnimation=False)")
        .ok_or_else(|| "Maya time-unit command is missing".to_owned())?;
    let import_position = first_script
        .find("cmds.file(")
        .ok_or_else(|| "Maya FBX import command is missing".to_owned())?;
    if time_unit_position >= import_position {
        return Err(
            "Maya time unit must be configured before import".to_owned(),
        );
    }
    for forbidden in [
        "__SHAR_FBX_FILE_NAME__",
        "__SHAR_MAYA_TIME_UNIT__",
        "maya.standalone",
        "cmds.file(rename=",
        "cmds.file(save=True",
        ".maya.fbx",
        ".dae",
        ".ma\"",
        ".mb\"",
    ] {
        if first_script.contains(forbidden) {
            return Err(format!("forbidden Maya helper contract: {forbidden}"));
        }
    }
    Ok(())
}

#[test]
fn rejects_nonportable_or_non_fbx_identity() {
    for path in [
        "character.dae",
        ".character.fbx",
        "character with spaces.fbx",
    ] {
        let error = write_maya_import_helper(
            Path::new(path),
            None,
            Path::new("unused.maya.py"),
        );
        assert_eq!(
            error,
            Err(MayaImportHelperError::InvalidFbxFileName)
        );
    }
}

#[test]
fn rejects_invalid_animation_frame_rates() {
    for frame_rate in [
        0.0_f64,
        -1.0_f64,
        f64::NAN,
        f64::INFINITY,
    ] {
        let error = write_maya_import_helper(
            Path::new("character.fbx"),
            Some(frame_rate),
            Path::new("unused.maya.py"),
        );
        assert_eq!(
            error,
            Err(MayaImportHelperError::InvalidFrameRate)
        );
    }
}
