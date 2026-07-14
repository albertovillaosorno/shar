// File:
//   - filesystem_batch_artifact.rs
// Path:
//   - src/p3d/src/adapters/driven/filesystem_batch_artifact.rs
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
//   - Physical completeness checks for cached P3D component artifacts.
// - Must-Not:
//   - Parse package-header counts or choose batch input and output roots.
// - Allows:
//   - Resolve manifest paths and validate artifact bytes by payload format.
// - Split-When:
//   - Image and JSON payload verification require independent providers.
// - Merge-When:
//   - Physical artifact evidence no longer differs from manifest identity.
// - Summary:
//   - Validates cached P3D component artifacts.
// - Description:
//   - Requires safe paths, regular nonempty files, and valid cached JSON.
// - Usage:
//   - Called by filesystem_batch_cache after structural manifest validation.
// - Defaults:
//   - Missing, empty, malformed, or unsafe artifacts are incomplete.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Physical completeness checks for cached P3D component artifacts.
//!
//! Manifest paths resolve beneath the package `components` directory, and
//! schema JSON is parsed again before cache reuse.

use std::path::{Path, PathBuf};

#[cfg(test)]
use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;

use super::image::detect_image_extension;

/// Returns whether every manifest row references a complete component file.
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

/// Returns whether one manifest component resolves to a nonempty file.
#[cfg(test)]
pub(super) fn cache_component_exists(
    output_dir: &Path,
    relative_path: &str,
) -> bool {
    let Some(component_path) = cache_component_path(
        output_dir,
        relative_path,
    ) else {
        return false;
    };
    component_file_has_data(&component_path)
}

/// Returns whether one cached component contains valid payload evidence.
fn cache_component_is_complete(
    output_dir: &Path,
    relative_path: &str,
    payload_format: &str,
) -> bool {
    let Some(component_path) = cache_component_path(
        output_dir,
        relative_path,
    ) else {
        return false;
    };
    let Ok(bytes) = local::read_bytes(&component_path) else {
        return false;
    };
    if bytes.is_empty() {
        return false;
    }
    if payload_format == "schema_json" {
        return serde_json::from_slice::<serde_json::Value>(&bytes).is_ok();
    }
    let Some(subtype) = payload_format.strip_prefix("image/") else {
        return false;
    };
    detect_image_extension(&bytes) == Some(subtype)
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
    if !matches!(
        local::path_kind(component_path),
        Ok(PathKind::File)
    ) {
        return false;
    }
    matches!(
        local::file_len(component_path),
        Ok(length) if length > 0
    )
}

#[cfg(test)]
#[path = "filesystem_batch_artifact_tests.rs"]
mod tests;
