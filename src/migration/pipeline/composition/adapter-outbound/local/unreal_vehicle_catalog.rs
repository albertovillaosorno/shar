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
//   - Verification of generated vehicle FBX catalog evidence for Unreal plans.
// - Must-Not:
//   - Infer gameplay construction or collapse vehicle support packages.
// - Allows:
//   - Read the deterministic vehicle catalog and verify its FBX payload bytes.
// - Split-When:
//   - Split when vehicle plan promotion gains an independent lifecycle.
// - Merge-When:
//   - Merge when another adapter owns identical vehicle artifact verification.
// - Summary:
//   - Generated vehicle catalog verifier.
// - Description:
//   - Converts verified vehicle-catalog FBX rows into generic Unreal FBX
//     artifact evidence without claiming that the full semantic package is
//     ready.
// - Usage:
//   - Used by prepare-unreal before any future vehicle plan promotion.
// - Defaults:
//   - Missing roots remain absent; malformed or stale roots fail closed.
//

//! Generated vehicle FBX catalog verification.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde_json::Value;
use shar_sha256::digest_hex;

use super::unreal_fbx_catalog::{
    FBX_VERSION, binary_fbx_version, io_error, validate_ancestor_chain,
    validate_digest, validate_directory_metadata, validate_public_identifier,
    validate_regular_file, validate_relative_path,
};
use crate::domain::{
    PipelineError, PipelineOutcome, UnrealFbxArtifactEvidence,
};

const CATALOG_FILE: &str = "vehicles.catalog.json";
const CATALOG_SCHEMA: &str = "shar.vehicle-catalog.v5";
const LOGICAL_ROOT: &str = "vehicle-assets";

/// One verified vehicle FBX plus its exact package subcategory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct VerifiedVehicleFbxArtifact {
    pub evidence: UnrealFbxArtifactEvidence,
    pub subcategory: String,
}

/// Verify the generated vehicle FBX rows when the catalog root exists.
///
/// This intentionally verifies only the FBX presentation boundary. Vehicle
/// textures, materials, Physics Assets, wheels, and runtime construction remain
/// separate semantic work and are not promoted by this adapter.
///
/// # Errors
///
/// Returns an error for malformed, duplicated, unsafe, stale, linked, or
/// unsupported vehicle FBX evidence.
pub(super) fn verified_vehicle_fbx_catalog(
    root: &Path,
) -> PipelineOutcome<Option<Vec<VerifiedVehicleFbxArtifact>>> {
    let metadata = match fs::symlink_metadata(root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        },
        Err(error) => {
            return Err(io_error(
                "inspect generated vehicle catalog root",
                &error,
            ));
        },
        Ok(metadata) => metadata,
    };
    validate_directory_metadata(&metadata)?;
    let catalog_path = root.join(CATALOG_FILE);
    validate_regular_file(&catalog_path, "generated vehicle catalog")?;
    let text = fs::read_to_string(&catalog_path)
        .map_err(|error| io_error("read generated vehicle catalog", &error))?;
    let root_value = serde_json::from_str::<Value>(&text).map_err(|_error| {
        PipelineError::new("generated vehicle catalog contains invalid JSON")
    })?;
    let object = root_value.as_object().ok_or_else(|| {
        PipelineError::new("generated vehicle catalog must be a JSON object")
    })?;
    if object.get("schema").and_then(Value::as_str) != Some(CATALOG_SCHEMA) {
        return Err(PipelineError::new(
            "generated vehicle catalog schema is not supported",
        ));
    }
    let vehicles = object
        .get("vehicles")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new("generated vehicle catalog has no vehicle rows")
        })?;
    let declared = object
        .get("counts")
        .and_then(Value::as_object)
        .and_then(|counts| counts.get("vehicles"))
        .and_then(Value::as_u64)
        .ok_or_else(|| {
            PipelineError::new("generated vehicle catalog has no vehicle count")
        })?;
    if declared != u64::try_from(vehicles.len()).unwrap_or(u64::MAX) {
        return Err(PipelineError::new(
            "generated vehicle catalog vehicle count is stale",
        ));
    }

    let mut package_ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut result = Vec::with_capacity(vehicles.len());
    for row in vehicles {
        let row = row.as_object().ok_or_else(|| {
            PipelineError::new("generated vehicle catalog row is not an object")
        })?;
        let package_id = required_string(row, "package_id")?;
        validate_public_identifier(&package_id)?;
        if !package_ids.insert(package_id.clone()) {
            return Err(PipelineError::new(
                "generated vehicle catalog contains a duplicate package",
            ));
        }
        let subcategory = required_string(row, "subcategory")?;
        validate_vehicle_subcategory(&subcategory)?;
        let vehicle = required_string(row, "vehicle")?;
        validate_vehicle_name(&vehicle)?;
        let fbx = row.get("fbx").and_then(Value::as_object).ok_or_else(|| {
            PipelineError::new(
                "generated vehicle catalog row has no FBX record",
            )
        })?;
        let relative_path = required_string(fbx, "path")?;
        validate_relative_path(&relative_path)?;
        let expected_path = format!("{vehicle}/{vehicle}.fbx");
        if relative_path != expected_path
            || !paths.insert(relative_path.clone())
        {
            return Err(PipelineError::new(
                "generated vehicle FBX path is not canonical or unique",
            ));
        }
        let size_bytes = required_u64(fbx, "bytes")?;
        let expected_sha256 = required_string(fbx, "sha256")?;
        validate_digest(&expected_sha256)?;
        let path = root.join(&relative_path);
        validate_regular_file(&path, "generated vehicle FBX")?;
        validate_ancestor_chain(root, &path)?;
        let bytes = fs::read(&path)
            .map_err(|error| io_error("read generated vehicle FBX", &error))?;
        let actual_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if actual_size != size_bytes || digest_hex(&bytes) != expected_sha256 {
            return Err(PipelineError::new(
                "generated vehicle FBX bytes do not match the catalog",
            ));
        }
        let version = binary_fbx_version(&bytes)?;
        if version != FBX_VERSION {
            return Err(PipelineError::new(
                "generated vehicle FBX version is not supported",
            ));
        }
        result.push(VerifiedVehicleFbxArtifact {
            evidence: UnrealFbxArtifactEvidence {
                package_id,
                path: format!("{LOGICAL_ROOT}/{relative_path}"),
                size_bytes: actual_size,
                sha256: expected_sha256,
                fbx_version: version,
            },
            subcategory,
        });
    }
    result.sort_by(|left, right| {
        left.evidence.package_id.cmp(&right.evidence.package_id)
    });
    Ok(Some(result))
}

fn required_string(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> PipelineOutcome<String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "generated vehicle catalog is missing string field {field}"
            ))
        })
}

fn required_u64(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> PipelineOutcome<u64> {
    object.get(field).and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new(format!(
            "generated vehicle catalog is missing unsigned field {field}"
        ))
    })
}

fn validate_vehicle_subcategory(value: &str) -> PipelineOutcome<()> {
    if !value.starts_with("cars/")
        || value.contains(char::from(92))
        || value.contains(':')
        || value.chars().any(char::is_control)
        || value
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(PipelineError::new(
            "generated vehicle subcategory is not canonical",
        ));
    }
    Ok(())
}

fn validate_vehicle_name(value: &str) -> PipelineOutcome<()> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
    {
        return Err(PipelineError::new(
            "generated vehicle identity is not canonical",
        ));
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: repository test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_vehicle_catalog/tests.rs"]
mod tests;
