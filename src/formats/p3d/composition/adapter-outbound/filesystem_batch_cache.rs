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
//   - Filesystem batch cache outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem batch cache outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem batch cache outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local;

use super::filesystem_batch_artifact::manifest_component_files_exist;

/// Stable schema identity for the package header row.
const PACKAGE_SCHEMA: &str = "p3d.package.v1";

/// Count evidence carried by one valid package header row.
struct PackageHeaderEvidence {
    /// Total parsed chunk count, including the root.
    chunk_count: usize,
    /// Number of published component rows.
    component_count: usize,
}

/// Unique identity carried by one complete component row.
struct ComponentIdentity {
    /// Source chunk ordinal.
    ordinal: usize,
    /// Source chunk depth below the root.
    depth: usize,
    /// Immediate source parent ordinal.
    parent_ordinal: usize,
    /// Direct root-child owner ordinal.
    container_ordinal: usize,
    /// Published relative artifact path.
    path: String,
}

/// Returns whether an output directory has a complete component cache.
pub(super) fn is_cache_complete(output_dir: &Path) -> bool {
    let manifest = output_dir.join("components.jsonl");
    let Ok(Some(text)) = local::read_optional_utf8(&manifest) else {
        return false;
    };
    if !manifest_is_complete(&text) {
        return false;
    }
    manifest_component_files_exist(output_dir, &text)
}

/// Returns whether one component manifest contains only complete rows.
pub(super) fn manifest_is_complete(text: &str) -> bool {
    let mut header_evidence = None;
    let mut component_ordinals = BTreeSet::new();
    let mut component_paths = BTreeSet::new();
    let mut component_relations = BTreeMap::new();
    let mut parsed_components = 0_usize;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if header_evidence.is_none() {
            header_evidence = manifest_header_evidence(trimmed);
            if header_evidence.is_none() {
                return false;
            }
            continue;
        }
        let Some(component_identity) = complete_component_identity(trimmed)
        else {
            return false;
        };
        let Some(header) = header_evidence.as_ref() else {
            return false;
        };
        let path_identity = component_identity
            .path
            .chars()
            .flat_map(char::to_uppercase)
            .collect::<String>();
        if component_identity.ordinal >= header.chunk_count
            || component_identity.parent_ordinal >= header.chunk_count
            || component_identity.container_ordinal >= header.chunk_count
            || !component_ordinals.insert(component_identity.ordinal)
            || !component_paths.insert(path_identity)
            || !component_relationship_is_locally_valid(&component_identity)
        {
            return false;
        }
        let previous = component_relations.insert(
            component_identity.ordinal,
            (
                component_identity.depth,
                component_identity.parent_ordinal,
                component_identity.container_ordinal,
            ),
        );
        if previous.is_some() {
            return false;
        }
        let Some(next_count) = parsed_components.checked_add(1) else {
            return false;
        };
        parsed_components = next_count;
    }
    let Some(header) = header_evidence else {
        return false;
    };
    parsed_components > 0
        && parsed_components == header.component_count
        && published_parent_relationships_are_valid(&component_relations)
}

/// Returns count evidence declared by one valid package header row.
fn manifest_header_evidence(line: &str) -> Option<PackageHeaderEvidence> {
    let parsed = serde_json::from_str::<serde_json::Value>(line).ok()?;
    let object = parsed.as_object()?;
    if object.get("schema")?.as_str()? != PACKAGE_SCHEMA {
        return None;
    }
    let byte_len = object.get("byte_len")?.as_u64()?;
    let chunk_count = object.get("chunk_count")?.as_u64()?;
    let count = object.get("component_count")?.as_u64()?;
    let minimum_byte_len = chunk_count.checked_mul(12)?;
    if byte_len < minimum_byte_len || chunk_count == 0 || count >= chunk_count {
        return None;
    }
    Some(PackageHeaderEvidence {
        chunk_count: usize::try_from(chunk_count).ok()?,
        component_count: usize::try_from(count).ok()?,
    })
}

/// Returns the ordinal and artifact path from one complete component row.
fn complete_component_identity(line: &str) -> Option<ComponentIdentity> {
    let value = serde_json::from_str::<serde_json::Value>(line).ok()?;
    let object = value.as_object()?;
    let ordinal = usize::try_from(object.get("ordinal")?.as_u64()?).ok()?;
    let depth = usize::try_from(object.get("depth")?.as_u64()?).ok()?;
    let parent_ordinal =
        usize::try_from(object.get("parent_ordinal")?.as_u64()?).ok()?;
    let container_ordinal =
        usize::try_from(object.get("container_ordinal")?.as_u64()?).ok()?;
    let _name = object.get("name")?.as_str()?;
    let payload_format = object.get("payload_format")?.as_str()?;
    let kind = object.get("kind")?.as_str()?;
    let schema_ref = object.get("schema_ref")?.as_str()?;
    let recovery_status = object.get("recovery_status")?.as_str()?;
    let path = object.get("path")?.as_str()?;
    if ordinal == 0
        || payload_format.is_empty()
        || kind.is_empty()
        || schema_ref.is_empty()
        || kind == "unknown"
        || schema_ref == "unknown"
        || schema_ref != super::package::kind_schema(kind)
        || !recovery_status_matches_payload_format(
            recovery_status,
            payload_format,
        )
        || !artifact_path_matches_payload_format(path, payload_format)
        || path.is_empty()
    {
        return None;
    }
    Some(ComponentIdentity {
        ordinal,
        depth,
        parent_ordinal,
        container_ordinal,
        path: path.to_owned(),
    })
}

/// Returns whether one component's own ancestry fields are self-consistent.
const fn component_relationship_is_locally_valid(component: &ComponentIdentity) -> bool {
    match component.depth {
        0 => false,
        1 => {
            component.parent_ordinal == 0
                && component.container_ordinal == component.ordinal
        },
        _ => component.parent_ordinal != 0 && component.container_ordinal != 0,
    }
}

/// Validate parent links only when both published rows are present.
fn published_parent_relationships_are_valid(
    relations: &BTreeMap<usize, (usize, usize, usize)>,
) -> bool {
    relations.iter().all(
        |(ordinal, (depth, parent_ordinal, container_ordinal))| {
            let Some((parent_depth, _parent_parent, parent_container)) =
                relations.get(parent_ordinal)
            else {
                return true;
            };
            parent_depth.checked_add(1) == Some(*depth)
                && container_ordinal == parent_container
                && ordinal != parent_ordinal
        },
    )
}

/// Returns whether one artifact path extension matches its payload format.
fn artifact_path_matches_payload_format(
    path: &str,
    payload_format: &str,
) -> bool {
    let Some(extension) =
        Path::new(path).extension().and_then(|value| value.to_str())
    else {
        return false;
    };
    if payload_format == "schema_json" {
        return extension == "json";
    }
    let Some(subtype) = payload_format.strip_prefix("image/") else {
        return false;
    };
    extension == subtype
}

/// Returns whether recovery state and payload format prove one complete
/// artifact.
fn recovery_status_matches_payload_format(
    recovery_status: &str,
    payload_format: &str,
) -> bool {
    match recovery_status {
        "decoded_schema_payload" => payload_format == "schema_json",
        "recovered_embedded_image_payload" => {
            is_image_payload_format(payload_format)
        },
        _ => false,
    }
}

/// Returns whether one payload format names a concrete image media subtype.
fn is_image_payload_format(payload_format: &str) -> bool {
    let Some(subtype) = payload_format.strip_prefix("image/") else {
        return false;
    };
    !subtype.is_empty()
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-outbound/filesystem_batch_identity_tests.rs"]
mod identity_tests;

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/p3d/unit/adapter-outbound/filesystem_batch_cache_tests.rs"]
mod tests;
