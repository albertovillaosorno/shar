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
//   - Read-only verification of the complete generated FBX catalog.
// - Must-Not:
//   - Generate FBX bytes, mutate extraction, or accept partial catalogs.
// - Allows:
//   - Validate catalog JSONL, inventory, hashes, FBX headers, and PNG evidence.
// - Split-When:
//   - Split when catalog publication gains an independent lifecycle.
// - Merge-When:
//   - Merge when another adapter owns identical generated-FBX verification.
// - Summary:
//   - Generated FBX catalog verification adapter.
// - Description:
//   - Promotes generated model artifacts only after exact read-back evidence.
// - Usage:
//   - Read by prepare-unreal before aggregate plan generation.
// - Defaults:
//   - An absent catalog leaves FBX operations pending; an existing invalid or
//     partial catalog fails closed.
//

//! Generated FBX catalog verification adapter.

use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Map, Value};
use shar_sha256::digest_hex;

use crate::domain::{
    PipelineError, PipelineOutcome, UnrealFbxArtifactEvidence,
};

/// Stable logical FBX artifact prefix stored in generated plans.
const FBX_ARTIFACT_LOGICAL_ROOT: &str = "fbx-assets";
const CATALOG_FILE: &str = "catalog.jsonl";
pub(super) const CATALOG_SCHEMA: &str = "shar-schoenwald.fbx-catalog.v2";
const PNG_MAGIC: &[u8] = &[0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
pub(super) const FBX_VERSION: u32 = 7700;
const FBX_HEADER_SIZE: usize = 27;
const FBX_MAGIC: &[u8] = b"Kaydara FBX Binary  \0\x1a\0";

/// Read and verify one complete generated FBX catalog when it exists.
///
/// # Errors
///
/// Returns an error when an existing catalog is malformed, partial, stale,
/// unsafe, contains an unexpected file, or references invalid FBX bytes.
pub(super) fn verified_fbx_catalog(
    root: &Path,
) -> PipelineOutcome<Option<Vec<UnrealFbxArtifactEvidence>>> {
    match fs::symlink_metadata(root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(None);
        },
        Err(error) => {
            return Err(io_error("inspect generated FBX catalog root", &error));
        },
        Ok(metadata) => validate_directory_metadata(&metadata)?,
    }

    let catalog_path = root.join(CATALOG_FILE);
    validate_regular_file(&catalog_path, "generated FBX catalog")?;
    let catalog = fs::read_to_string(&catalog_path)
        .map_err(|error| io_error("read generated FBX catalog", &error))?;
    if !catalog.ends_with('\n') || catalog.contains('\r') {
        return Err(PipelineError::new(
            "generated FBX catalog line endings are not canonical",
        ));
    }
    let mut lines = catalog.lines();
    let header_line = lines
        .next()
        .ok_or_else(|| PipelineError::new("generated FBX catalog is empty"))?;
    if header_line.trim().is_empty() {
        return Err(PipelineError::new(
            "generated FBX catalog header is blank",
        ));
    }
    let header = parse_object(header_line, "generated FBX catalog header")?;
    require_exact_fields(
        &header,
        &[
            "schema",
            "record_type",
            "status",
            "package_count",
            "file_count",
        ],
        "generated FBX catalog header",
    )?;
    if required_string(&header, "schema", "generated FBX catalog header")?
        != CATALOG_SCHEMA
        || required_string(
            &header,
            "record_type",
            "generated FBX catalog header",
        )? != "header"
        || required_string(&header, "status", "generated FBX catalog header")?
            != "complete"
    {
        return Err(PipelineError::new(
            "generated FBX catalog header is not canonical",
        ));
    }
    let declared_package_count =
        required_u64(&header, "package_count", "generated FBX catalog header")?;
    let declared_file_count =
        required_u64(&header, "file_count", "generated FBX catalog header")?;

    let mut package_ids = BTreeSet::new();
    let mut texture_package_ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut evidence = Vec::new();
    let mut record_count = 0_u64;
    for (offset, line) in lines.enumerate() {
        let line_number = offset.saturating_add(2);
        if line.trim().is_empty() {
            return Err(PipelineError::new(format!(
                "generated FBX catalog line {line_number} is blank"
            )));
        }
        record_count = record_count.saturating_add(1);
        let label = format!("generated FBX catalog line {line_number}");
        let row = parse_object(line, &label)?;
        if required_string(&row, "schema", &label)? != CATALOG_SCHEMA {
            return Err(PipelineError::new(format!(
                "{label} uses an unsupported catalog schema"
            )));
        }
        let record_type = required_string(&row, "record_type", &label)?;
        match record_type.as_str() {
            "fbx" => {
                require_exact_fields(
                    &row,
                    &[
                        "schema",
                        "record_type",
                        "package_id",
                        "path",
                        "size_bytes",
                        "sha256",
                        "fbx_version",
                    ],
                    &label,
                )?;
                let package_id = required_string(&row, "package_id", &label)?;
                validate_public_identifier(&package_id)?;
                if !package_ids.insert(package_id.clone()) {
                    return Err(PipelineError::new(
                        "generated FBX catalog contains a duplicate package",
                    ));
                }
                let relative_path = required_string(&row, "path", &label)?;
                validate_fbx_catalog_path(&package_id, &relative_path)?;
                claim_catalog_path(&mut paths, &relative_path)?;
                let size_bytes = required_u64(&row, "size_bytes", &label)?;
                let expected_sha256 = required_string(&row, "sha256", &label)?;
                validate_digest(&expected_sha256)?;
                let declared_version =
                    required_u64(&row, "fbx_version", &label)?;
                if declared_version != u64::from(FBX_VERSION) {
                    return Err(PipelineError::new(
                        "generated FBX catalog declares an unsupported version",
                    ));
                }
                evidence.push(verify_fbx(
                    root,
                    &package_id,
                    &relative_path,
                    size_bytes,
                    &expected_sha256,
                )?);
            },
            "texture" => {
                require_exact_fields(
                    &row,
                    &[
                        "schema",
                        "record_type",
                        "package_id",
                        "path",
                        "size_bytes",
                        "sha256",
                    ],
                    &label,
                )?;
                let package_id = required_string(&row, "package_id", &label)?;
                validate_public_identifier(&package_id)?;
                let _inserted = texture_package_ids.insert(package_id.clone());
                let relative_path = required_string(&row, "path", &label)?;
                validate_texture_catalog_path(&package_id, &relative_path)?;
                claim_catalog_path(&mut paths, &relative_path)?;
                let size_bytes = required_u64(&row, "size_bytes", &label)?;
                let expected_sha256 = required_string(&row, "sha256", &label)?;
                validate_digest(&expected_sha256)?;
                verify_texture(
                    root,
                    &relative_path,
                    size_bytes,
                    &expected_sha256,
                )?;
            },
            _ => {
                return Err(PipelineError::new(format!(
                    "{label} has an unsupported record type"
                )));
            },
        }
    }

    let actual_package_count =
        u64::try_from(evidence.len()).unwrap_or(u64::MAX);
    if declared_package_count != actual_package_count {
        return Err(PipelineError::new(
            "generated FBX catalog package count is stale",
        ));
    }
    if declared_file_count != record_count {
        return Err(PipelineError::new(
            "generated FBX catalog file count is stale",
        ));
    }
    if !texture_package_ids.is_subset(&package_ids) {
        return Err(PipelineError::new(
            "generated FBX texture has no owning package",
        ));
    }
    verify_inventory(root, &paths)?;
    evidence.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    Ok(Some(evidence))
}

fn verify_fbx(
    root: &Path,
    package_id: &str,
    relative_path: &str,
    expected_size: u64,
    expected_sha256: &str,
) -> PipelineOutcome<UnrealFbxArtifactEvidence> {
    let path = root.join(relative_path);
    validate_regular_file(&path, "generated FBX artifact")?;
    validate_ancestor_chain(root, &path)?;
    let bytes = fs::read(&path)
        .map_err(|error| io_error("read generated FBX artifact", &error))?;
    let actual_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if actual_size != expected_size {
        return Err(PipelineError::new(
            "generated FBX artifact size does not match its catalog",
        ));
    }
    let actual_sha256 = digest_hex(&bytes);
    if actual_sha256 != expected_sha256 {
        return Err(PipelineError::new(
            "generated FBX artifact digest does not match its catalog",
        ));
    }
    let version = binary_fbx_version(&bytes)?;
    Ok(UnrealFbxArtifactEvidence {
        package_id: package_id.to_owned(),
        path: format!("{FBX_ARTIFACT_LOGICAL_ROOT}/{relative_path}"),
        size_bytes: actual_size,
        sha256: actual_sha256,
        fbx_version: version,
    })
}

fn verify_texture(
    root: &Path,
    relative_path: &str,
    expected_size: u64,
    expected_sha256: &str,
) -> PipelineOutcome<()> {
    let path = root.join(relative_path);
    validate_regular_file(&path, "generated FBX texture")?;
    validate_ancestor_chain(root, &path)?;
    let bytes = fs::read(&path)
        .map_err(|error| io_error("read generated FBX texture", &error))?;
    let actual_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
    if actual_size != expected_size {
        return Err(PipelineError::new(
            "generated FBX texture size does not match its catalog",
        ));
    }
    if digest_hex(&bytes) != expected_sha256 {
        return Err(PipelineError::new(
            "generated FBX texture digest does not match its catalog",
        ));
    }
    if !bytes.starts_with(PNG_MAGIC) {
        return Err(PipelineError::new(
            "generated FBX texture is not a PNG artifact",
        ));
    }
    Ok(())
}

fn binary_fbx_version(bytes: &[u8]) -> PipelineOutcome<u32> {
    if bytes.len() < FBX_HEADER_SIZE || !bytes.starts_with(FBX_MAGIC) {
        return Err(PipelineError::new(
            "generated FBX artifact has an invalid binary header",
        ));
    }
    let version_slice =
        bytes.get(FBX_MAGIC.len()..FBX_HEADER_SIZE).ok_or_else(|| {
            PipelineError::new(
                "generated FBX artifact has an invalid version field",
            )
        })?;
    let version_bytes: [u8; 4] =
        version_slice.try_into().map_err(|_error| {
            PipelineError::new(
                "generated FBX artifact has an invalid version field",
            )
        })?;
    let version = u32::from_le_bytes(version_bytes);
    if version != FBX_VERSION {
        return Err(PipelineError::new(
            "generated FBX artifact version is not supported",
        ));
    }
    Ok(version)
}

fn validate_fbx_catalog_path(
    package_id: &str,
    relative_path: &str,
) -> PipelineOutcome<()> {
    validate_relative_path(relative_path)?;
    let package_name = package_id.replace('-', "_");
    let expected = format!("packages/{package_name}/{package_name}.fbx");
    if relative_path != expected {
        return Err(PipelineError::new(
            "generated FBX catalog path does not match its package identity",
        ));
    }
    Ok(())
}

fn validate_texture_catalog_path(
    package_id: &str,
    relative_path: &str,
) -> PipelineOutcome<()> {
    validate_relative_path(relative_path)?;
    let package_name = package_id.replace('-', "_");
    let prefix = format!("packages/{package_name}/textures/");
    let Some(file_name) = relative_path.strip_prefix(&prefix) else {
        return Err(PipelineError::new(
            "generated FBX texture path does not match its package identity",
        ));
    };
    let extension_is_png = Path::new(file_name)
        .extension()
        .is_some_and(|extension| extension == "png");
    if file_name.is_empty() || file_name.contains('/') || !extension_is_png {
        return Err(PipelineError::new(
            "generated FBX texture path is not canonical",
        ));
    }
    Ok(())
}

fn claim_catalog_path(
    paths: &mut BTreeSet<String>,
    relative_path: &str,
) -> PipelineOutcome<()> {
    if !paths.insert(relative_path.to_owned()) {
        return Err(PipelineError::new(
            "generated FBX catalog contains a duplicate path",
        ));
    }
    Ok(())
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
        return Err(PipelineError::new("generated FBX catalog path is unsafe"));
    }
    Ok(())
}

fn validate_public_identifier(value: &str) -> PipelineOutcome<()> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || bytes.windows(2).any(|pair| pair == b"--")
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
    {
        return Err(PipelineError::new(
            "generated FBX package identity is not canonical",
        ));
    }
    Ok(())
}

fn validate_digest(value: &str) -> PipelineOutcome<()> {
    if value.len() != 64
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || matches!(byte, b'a'..=b'f'))
    {
        return Err(PipelineError::new(
            "generated FBX catalog digest is not canonical",
        ));
    }
    Ok(())
}

fn verify_inventory(
    root: &Path,
    declared_paths: &BTreeSet<String>,
) -> PipelineOutcome<()> {
    let mut actual = BTreeSet::new();
    collect_inventory(root, root, &mut actual)?;
    let mut expected = declared_paths.clone();
    let _inserted = expected.insert(CATALOG_FILE.to_owned());
    if actual != expected {
        return Err(PipelineError::new(
            "generated FBX catalog inventory is not exact",
        ));
    }
    Ok(())
}

fn collect_inventory(
    root: &Path,
    directory: &Path,
    inventory: &mut BTreeSet<String>,
) -> PipelineOutcome<()> {
    validate_directory(directory, "generated FBX catalog directory")?;
    let entries = fs::read_dir(directory)
        .map_err(|error| io_error("list generated FBX catalog", &error))?;
    for entry in entries {
        let entry = entry.map_err(|error| {
            io_error("read generated FBX catalog entry", &error)
        })?;
        let path = entry.path();
        let metadata = fs::symlink_metadata(&path).map_err(|error| {
            io_error("inspect generated FBX catalog entry", &error)
        })?;
        reject_reparse_or_symlink(&metadata)?;
        if metadata.is_dir() {
            collect_inventory(root, &path, inventory)?;
        } else if metadata.is_file() {
            let relative = portable_relative_path(root, &path)?;
            if !inventory.insert(relative) {
                return Err(PipelineError::new(
                    "generated FBX catalog inventory contains a collision",
                ));
            }
        } else {
            return Err(PipelineError::new(
                "generated FBX catalog contains an unsupported entry",
            ));
        }
    }
    Ok(())
}

fn validate_ancestor_chain(root: &Path, path: &Path) -> PipelineOutcome<()> {
    let relative = path.strip_prefix(root).map_err(|_error| {
        PipelineError::new("generated FBX artifact escaped its catalog root")
    })?;
    let mut current = PathBuf::from(root);
    let mut components = relative.components().peekable();
    while let Some(component) = components.next() {
        current.push(component);
        if components.peek().is_some() {
            validate_directory(&current, "generated FBX artifact ancestor")?;
        }
    }
    Ok(())
}

fn portable_relative_path(root: &Path, path: &Path) -> PipelineOutcome<String> {
    let relative = path.strip_prefix(root).map_err(|_error| {
        PipelineError::new("generated FBX inventory escaped its catalog root")
    })?;
    let mut parts = Vec::new();
    for component in relative.components() {
        let part = component.as_os_str().to_str().ok_or_else(|| {
            PipelineError::new(
                "generated FBX inventory path is not portable UTF-8",
            )
        })?;
        parts.push(part);
    }
    let portable = parts.join("/");
    validate_relative_path(&portable)?;
    Ok(portable)
}

fn validate_directory(path: &Path, label: &str) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_error("inspect generated FBX directory", &error))?;
    validate_directory_metadata(&metadata).map_err(|_error| {
        PipelineError::new(format!("{label} is not a regular directory"))
    })
}

fn validate_directory_metadata(metadata: &fs::Metadata) -> PipelineOutcome<()> {
    reject_reparse_or_symlink(metadata)?;
    if !metadata.is_dir() {
        return Err(PipelineError::new(
            "generated FBX catalog root is not a regular directory",
        ));
    }
    Ok(())
}

fn validate_regular_file(path: &Path, label: &str) -> PipelineOutcome<()> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| io_error("inspect generated FBX file", &error))?;
    reject_reparse_or_symlink(&metadata)?;
    if !metadata.is_file() {
        return Err(PipelineError::new(format!(
            "{label} is not a regular file"
        )));
    }
    Ok(())
}

fn reject_reparse_or_symlink(metadata: &fs::Metadata) -> PipelineOutcome<()> {
    if metadata.file_type().is_symlink() {
        return Err(PipelineError::new(
            "generated FBX catalog crosses a symbolic link",
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt as _;
        const REPARSE_POINT: u32 = 0x400;
        if metadata.file_attributes() & REPARSE_POINT != 0 {
            return Err(PipelineError::new(
                "generated FBX catalog crosses a reparse point",
            ));
        }
    }
    Ok(())
}

fn parse_object(
    json: &str,
    label: &str,
) -> PipelineOutcome<Map<String, Value>> {
    let value = serde_json::from_str::<Value>(json).map_err(|_error| {
        PipelineError::new(format!("{label} contains invalid JSON"))
    })?;
    value.as_object().cloned().ok_or_else(|| {
        PipelineError::new(format!("{label} must be a JSON object"))
    })
}

fn require_exact_fields(
    object: &Map<String, Value>,
    fields: &[&str],
    label: &str,
) -> PipelineOutcome<()> {
    let actual = object.keys().map(String::as_str).collect::<HashSet<_>>();
    let expected = fields.iter().copied().collect::<HashSet<_>>();
    if actual != expected {
        return Err(PipelineError::new(format!(
            "{label} fields are not canonical"
        )));
    }
    Ok(())
}

fn required_string(
    object: &Map<String, Value>,
    field: &str,
    label: &str,
) -> PipelineOutcome<String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{label} is missing string field {field}"
            ))
        })
}

fn required_u64(
    object: &Map<String, Value>,
    field: &str,
    label: &str,
) -> PipelineOutcome<u64> {
    object.get(field).and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new(format!(
            "{label} is missing unsigned integer field {field}"
        ))
    })
}

fn io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed ({:?})", error.kind()))
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_fbx_catalog/tests.rs"]
mod tests;
