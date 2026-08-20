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
//   - Filesystem intake of decoded mission camera and multi-controller
//     evidence.
// - Must-Not:
//   - Derive identity from filenames or infer cross-level lookup precedence.
// - Allows:
//   - Read indexed decoded JSON and build the level-scoped camera catalog.
// - Split-When:
//   - Camera and multi-controller decoding require independent intake policies.
// - Merge-When:
//   - Prepare-Unreal directly owns this exact decoded component intake.
// - Summary:
//   - Local mission camera catalog adapter.
// - Description:
//   - Reads exact embedded component names from mission camera packages and
//     binds them to package/member provenance before semantic reference checks.
// - Usage:
//   - Called once after authoritative phase-three package-index intake.
// - Defaults:
//   - Malformed paths, schema drift, or ambiguous referenced identities fail
//     closed.
//

//! Filesystem-backed decoded mission camera catalog intake.

use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local::read_utf8;
use serde_json::Value;

use crate::domain::{
    MissionCameraCatalog, MissionCameraCatalogEntry,
    MissionCameraComponentKind, PackageRole, PhaseThreePackageIndex,
    PipelineError, PipelineOutcome,
};

/// Build the canonical level-scoped camera catalog from indexed evidence.
pub(super) fn load_mission_camera_catalog(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
) -> PipelineOutcome<MissionCameraCatalog> {
    let mut entries = Vec::new();
    for package in index.packages() {
        if package.category() != "missions"
            || !package
                .package_root
                .starts_with("extracted/art/missions/level")
        {
            continue;
        }
        for member in package.members() {
            let Some(kind) = camera_member_kind(
                member.role,
                member.unit_type.as_str(),
                member.kind.as_str(),
                member.source_chunk_kind.as_str(),
            )? else {
                continue;
            };
            let physical = resolve_member_path(extracted_root, &member.path)?;
            let text = read_utf8(&physical).map_err(|error| {
                PipelineError::new(format!(
                    "mission camera catalog read failed: {error}"
                ))
            })?;
            let name = parse_component_name(&text, kind)?;
            entries.push(
                MissionCameraCatalogEntry::new(
                    name,
                    kind,
                    member.id.clone(),
                    package.package_id.clone(),
                    package.package_root.clone(),
                    member.path.clone(),
                )
                .map_err(|error| {
                    PipelineError::new(format!(
                        "mission camera catalog entry failed: {error}"
                    ))
                })?,
            );
        }
    }
    Ok(MissionCameraCatalog::from_entries(entries))
}

fn camera_member_kind(
    role: PackageRole,
    unit_type: &str,
    kind: &str,
    source_chunk_kind: &str,
) -> PipelineOutcome<Option<MissionCameraComponentKind>> {
    match (role, unit_type, kind, source_chunk_kind) {
        (PackageRole::Camera, "camera", "p3d-camera", "camera") => {
            Ok(Some(MissionCameraComponentKind::Camera))
        },
        (
            PackageRole::Controller,
            "controller",
            "p3d-controller",
            "multi_controller",
        ) => Ok(Some(MissionCameraComponentKind::MultiController)),
        (PackageRole::Camera, ..) => Err(PipelineError::new(
            "mission camera package member classification drifted",
        )),
        (PackageRole::Controller, _, _, "multi_controller") => {
            Err(PipelineError::new(concat!(
                "mission multi-controller package member classification ",
                "drifted"
            )))
        },
        _ => Ok(None),
    }
}

fn parse_component_name(
    json: &str,
    kind: MissionCameraComponentKind,
) -> PipelineOutcome<String> {
    let value = serde_json::from_str::<Value>(json).map_err(|error| {
        PipelineError::new(format!(
            "invalid decoded mission camera JSON: {error}"
        ))
    })?;
    let object = value.as_object().ok_or_else(|| {
        PipelineError::new("decoded mission camera must be a JSON object")
    })?;
    let expected_schema = match kind {
        MissionCameraComponentKind::Camera => "camera",
        MissionCameraComponentKind::MultiController => "multi_controller",
    };
    let schema = object
        .get("schema")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            PipelineError::new(
                "decoded mission camera schema must be a string",
            )
        })?;
    if schema != expected_schema {
        return Err(PipelineError::new(
            "decoded mission camera schema does not match member kind",
        ));
    }
    let raw_name = object
        .get("name")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            PipelineError::new(
                "decoded mission camera name must be a string",
            )
        })?;
    let name = raw_name.trim_end_matches(char::from(0)).to_owned();
    if name.is_empty() || name.chars().any(char::is_control) {
        return Err(PipelineError::new(concat!(
            "decoded mission camera name is empty or contains ",
            "interior control data"
        )));
    }
    Ok(name)
}

fn resolve_member_path(
    extracted_root: &Path,
    member_path: &str,
) -> PipelineOutcome<PathBuf> {
    let root_name = extracted_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new(
                "mission camera extracted root has no portable basename",
            )
        })?;
    let prefix = format!("{root_name}/");
    let relative = member_path.strip_prefix(&prefix).ok_or_else(|| {
        PipelineError::new(
            "mission camera member path is outside extracted root",
        )
    })?;
    if relative.is_empty()
        || relative.starts_with('/')
        || relative.contains(char::from(92))
        || relative.contains(':')
        || relative.chars().any(char::is_control)
        || relative
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(PipelineError::new(
            "unsafe mission camera member relative path",
        ));
    }
    Ok(extracted_root.join(relative))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/mission_camera_catalog/tests.rs"]
mod tests;
