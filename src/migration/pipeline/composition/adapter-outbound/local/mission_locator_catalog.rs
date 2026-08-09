// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Filesystem-backed intake of decoded mission locator package evidence.
// - Must-Not:
//   - Derive locator identity from filenames or infer package load precedence.
// - Allows:
//   - Read indexed decoded locator JSON and build the pure domain catalog.
// - Split-When:
//   - Locator decoding gains a second physical source format.
// - Merge-When:
//   - Prepare-Unreal owns this exact decoded locator intake directly.
// - Summary:
//   - Local mission locator catalog adapter.
// - Description:
//   - Verifies indexed srr_locator members against decoded JSON before exposing
//     exact names and source types to mission semantic resolution.
// - Usage:
//   - Called after authoritative phase-three package-index intake.
// - Defaults:
//   - Malformed paths, schema drift, or invalid decoded identities fail closed.
//

//! Filesystem-backed decoded mission locator catalog intake.

use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local::read_utf8;
use serde_json::{Map, Value};

use crate::domain::{
    MissionLocatorCatalog, MissionLocatorCatalogEntry, PackageRole,
    PhaseThreePackageIndex, PipelineError, PipelineOutcome,
};

/// Build the canonical decoded locator catalog from indexed package evidence.
pub(super) fn load_mission_locator_catalog(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
) -> PipelineOutcome<MissionLocatorCatalog> {
    let mut entries = Vec::new();
    for package in index.packages() {
        for member in package
            .members()
            .iter()
            .filter(|member| member.role == PackageRole::Locator)
        {
            validate_locator_member(
                member.unit_type.as_str(),
                member.kind.as_str(),
                member.source_chunk_kind.as_str(),
            )?;
            if member.source_chunk_kind == "locator" {
                continue;
            }
            let physical = resolve_locator_path(extracted_root, &member.path)?;
            let text = read_utf8(&physical).map_err(|error| {
                PipelineError::new(format!(
                    "mission locator catalog read failed: {error}"
                ))
            })?;
            let decoded = parse_decoded_locator(&text)?;
            entries.push(
                MissionLocatorCatalogEntry::new(
                    decoded.name,
                    decoded.locator_type,
                    decoded.locator_type_name,
                    member.id.clone(),
                    package.package_id.clone(),
                    package.package_root.clone(),
                    member.path.clone(),
                )
                .map_err(|error| {
                    PipelineError::new(format!(
                        "mission locator catalog entry failed: {error}"
                    ))
                })?,
            );
        }
    }
    MissionLocatorCatalog::from_entries(entries).map_err(|error| {
        PipelineError::new(format!(
            "mission locator catalog intake failed: {error}"
        ))
    })
}

#[derive(Debug, Eq, PartialEq)]
struct DecodedLocatorIdentity {
    name: String,
    locator_type: u32,
    locator_type_name: String,
}

fn parse_decoded_locator(json: &str) -> PipelineOutcome<DecodedLocatorIdentity> {
    let value = serde_json::from_str::<Value>(json).map_err(|error| {
        PipelineError::new(format!("invalid decoded locator JSON: {error}"))
    })?;
    let object = value.as_object().ok_or_else(|| {
        PipelineError::new("decoded locator must be a JSON object")
    })?;
    if required_string(object, "schema")? != "locator" {
        return Err(PipelineError::new(
            "decoded locator schema is not supported",
        ));
    }
    let raw_name = required_string(object, "name")?;
    let name = raw_name.trim_end_matches(char::from(0)).to_owned();
    let locator_type = required_u32(object, "locator_type")?;
    let locator_type_name = required_string(object, "locator_type_name")?;
    if name.is_empty() || name.chars().any(char::is_control) {
        return Err(PipelineError::new(
            "decoded locator name is empty or contains interior control data",
        ));
    }
    Ok(DecodedLocatorIdentity {
        name,
        locator_type,
        locator_type_name,
    })
}

fn validate_locator_member(
    unit_type: &str,
    kind: &str,
    source_chunk_kind: &str,
) -> PipelineOutcome<()> {
    if unit_type != "locator"
        || kind != "p3d-locator"
        || !matches!(source_chunk_kind, "locator" | "srr_locator")
    {
        return Err(PipelineError::new(
            "locator package member classification drifted",
        ));
    }
    Ok(())
}

fn resolve_locator_path(
    extracted_root: &Path,
    member_path: &str,
) -> PipelineOutcome<PathBuf> {
    let root_name = extracted_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new("extracted root has no portable basename")
        })?;
    let prefix = format!("{root_name}/");
    let relative = member_path.strip_prefix(&prefix).ok_or_else(|| {
        PipelineError::new("locator member path is outside extracted root")
    })?;
    validate_relative_path(relative)?;
    Ok(extracted_root.join(relative))
}

fn validate_relative_path(path: &str) -> PipelineOutcome<()> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains(char::from(92))
        || path.contains(':')
        || path.chars().any(char::is_control)
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(PipelineError::new("unsafe locator member relative path"));
    }
    Ok(())
}

fn required_string(
    object: &Map<String, Value>,
    field: &str,
) -> PipelineOutcome<String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "decoded locator field `{field}` must be a string"
            ))
        })
}

fn required_u32(
    object: &Map<String, Value>,
    field: &str,
) -> PipelineOutcome<u32> {
    let value = object
        .get(field)
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "decoded locator field `{field}` must be an unsigned integer"
            ))
        })?;
    u32::try_from(value).map_err(|_conversion_error| {
        PipelineError::new(format!(
            "decoded locator field `{field}` exceeds u32"
        ))
    })
}

#[cfg(test)]
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/mission_locator_catalog/tests.rs"]
mod tests;
