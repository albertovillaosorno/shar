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
//   - Filesystem batch artifact outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem batch artifact outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem batch artifact outbound adapter.

use std::path::{Path, PathBuf};

#[cfg(test)]
use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;
use shar_sha256::digest_hex;

use super::image::detect_image_extension;

/// Returns whether every manifest row references a complete component file.
#[cfg(test)]
pub(super) fn manifest_component_files_exist(
    output_dir: &Path,
    text: &str,
) -> bool {
    let mut has_header = false;
    let mut has_rows = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !has_header {
            has_header = true;
            continue;
        }
        let parsed = serde_json::from_str::<serde_json::Value>(trimmed);
        let Ok(value) = parsed else {
            return false;
        };
        let Some(object) = value.as_object() else {
            return false;
        };
        let Some(path_value) = object.get("path") else {
            return false;
        };
        let Some(relative_path) = path_value.as_str() else {
            return false;
        };
        let Some(payload_format_value) = object.get("payload_format") else {
            return false;
        };
        let Some(payload_format) = payload_format_value.as_str() else {
            return false;
        };
        if relative_path.is_empty()
            || !cache_component_is_complete(
                output_dir,
                relative_path,
                payload_format,
            )
        {
            return false;
        }
        has_rows = true;
    }
    has_header && has_rows
}

/// Returns whether every current manifest artifact matches its exact digest.
pub(super) fn manifest_component_files_match_digests(
    output_dir: &Path,
    text: &str,
) -> bool {
    let mut has_header = false;
    let mut has_rows = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if !has_header {
            has_header = true;
            continue;
        }
        let Ok(value) = serde_json::from_str::<serde_json::Value>(trimmed)
        else {
            return false;
        };
        let Some(object) = value.as_object() else {
            return false;
        };
        let Some(relative_path) =
            object.get("path").and_then(serde_json::Value::as_str)
        else {
            return false;
        };
        let Some(payload_format) = object
            .get("payload_format")
            .and_then(serde_json::Value::as_str)
        else {
            return false;
        };
        let Some(expected_sha256) =
            object.get("sha256").and_then(serde_json::Value::as_str)
        else {
            return false;
        };
        let Some(component_path) =
            cache_component_path(output_dir, relative_path)
        else {
            return false;
        };
        let Ok(bytes) = local::read_bytes(&component_path) else {
            return false;
        };
        if !payload_bytes_are_complete(&bytes, payload_format)
            || digest_hex(&bytes) != expected_sha256
        {
            return false;
        }
        has_rows = true;
    }
    has_header && has_rows
}

/// Returns whether one manifest component resolves to a nonempty file.
#[cfg(test)]
pub(super) fn cache_component_exists(
    output_dir: &Path,
    relative_path: &str,
) -> bool {
    let Some(component_path) = cache_component_path(output_dir, relative_path)
    else {
        return false;
    };
    component_file_has_data(&component_path)
}

/// Returns whether one cached component contains valid payload evidence.
#[cfg(test)]
fn cache_component_is_complete(
    output_dir: &Path,
    relative_path: &str,
    payload_format: &str,
) -> bool {
    let Some(component_path) = cache_component_path(output_dir, relative_path)
    else {
        return false;
    };
    let Ok(bytes) = local::read_bytes(&component_path) else {
        return false;
    };
    payload_bytes_are_complete(&bytes, payload_format)
}

/// Returns whether artifact bytes satisfy their declared payload encoding.
fn payload_bytes_are_complete(bytes: &[u8], payload_format: &str) -> bool {
    if bytes.is_empty() {
        return false;
    }
    if payload_format == "schema_json" {
        return serde_json::from_slice::<serde_json::Value>(bytes).is_ok();
    }
    let Some(subtype) = payload_format.strip_prefix("image/") else {
        return false;
    };
    detect_image_extension(bytes) == Some(subtype)
}

/// Resolves one manifest path beneath the package components directory.
fn cache_component_path(
    output_dir: &Path,
    relative_path: &str,
) -> Option<PathBuf> {
    let components_root = output_dir.join("components");
    schoenwald_filesystem::resolve_under(
        &components_root,
        Path::new(relative_path),
    )
    .ok()
}

/// Returns whether one resolved component path is a nonempty regular file.
#[cfg(test)]
fn component_file_has_data(component_path: &Path) -> bool {
    if !matches!(local::path_kind(component_path), Ok(PathKind::File)) {
        return false;
    }
    matches!(
        local::file_len(component_path),
        Ok(length) if length > 0
    )
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-outbound/filesystem_batch_artifact_tests.rs"]
mod tests;
