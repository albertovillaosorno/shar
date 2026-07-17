// File:
//   - inventory_common.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/inventory_common.rs
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
//   - Shared normalized-component identity and JSON inventory helpers.
// - Must-Not:
//   - Select category policy, serialize FBX, or publish final artifacts.
// - Allows:
//   - Component enumeration, JSON decoding, identity cleanup, and member maps.
// - Split-When:
//   - Ledger and component helpers gain independent consumers.
// - Merge-When:
//   - Mission and world inventories no longer share these exact rules.
// - Summary:
//   - Keeps fixed-width Pure3D identities and component paths deterministic.
// - Description:
//   - Rejects duplicate identities and malformed member paths visibly.
// - Usage:
//   - Imported by both mission and terrain-world prop inventories.
// - Defaults:
//   - JSON components are sorted by portable path before decoding.
//
// ADRs:
// - docs/adr/fbx/extraction/source-discovery-boundary.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Shared normalized-component inventory helpers.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::domain::PipelineError;

/// Composite identity, skeleton, and rigid prop references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CompositeEvidence {
    /// Normalized composite member id without family or extension.
    pub(super) member_id: String,
    /// Cleaned composite identity.
    pub(super) name: String,
    /// Cleaned referenced skeleton identity.
    pub(super) skeleton_name: String,
    /// Cleaned rigid prop identities owned by the composite.
    pub(super) prop_names: BTreeSet<String>,
}

/// Return all JSON component paths in stable file-name order.
pub(super) fn component_paths(
    root: &Path,
    family: &str,
) -> Result<Vec<PathBuf>, PipelineError> {
    let directory = root
        .join("components")
        .join(family);
    if !directory.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths = fs::read_dir(&directory)
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "prop component directory read failed for {}: {error}",
                        directory.display()
                    ),
                )
            },
        )?
        .map(
            |entry| {
                entry
                    .map(|value| value.path())
                    .map_err(|error| PipelineError::new(error.to_string()))
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    paths.retain(
        |path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        },
    );
    paths.sort();
    Ok(paths)
}

/// Map decoded component identity to its normalized member id.
pub(super) fn component_name_map(
    root: &Path,
    family: &str,
) -> Result<BTreeMap<String, String>, PipelineError> {
    let mut result = BTreeMap::new();
    for path in component_paths(
        root, family,
    )? {
        let name = read_component_name(&path)?;
        let id = component_member_id(&path)?;
        if result
            .insert(
                name.clone(),
                id,
            )
            .is_some()
        {
            return Err(
                PipelineError::new(
                    format!(
                        "prop component identity is duplicated: \
                         {family}/{name}"
                    ),
                ),
            );
        }
    }
    Ok(result)
}

/// Read one component's normalized name.
pub(super) fn read_component_name(
    path: &Path
) -> Result<String, PipelineError> {
    let value = read_json(path)?;
    Ok(
        clean_identity(
            &required_string(
                &value, "name",
            )?,
        ),
    )
}

/// Read one composite and its rigid mesh references.
pub(super) fn read_composite(
    path: &Path
) -> Result<CompositeEvidence, PipelineError> {
    let value = read_json(path)?;
    let props = value
        .get("props")
        .and_then(Value::as_array)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "prop composite has no props array: {}",
                        path.display()
                    ),
                )
            },
        )?;
    let prop_names = props
        .iter()
        .map(
            |prop| {
                prop.get("name")
                    .and_then(Value::as_str)
                    .map(clean_identity)
                    .ok_or_else(
                        || {
                            PipelineError::new(
                                format!(
                                    "prop composite has malformed prop name: \
                                     {}",
                                    path.display()
                                ),
                            )
                        },
                    )
            },
        )
        .collect::<Result<BTreeSet<_>, _>>()?;
    Ok(
        CompositeEvidence {
            member_id: component_member_id(path)?,
            name: clean_identity(
                &required_string(
                    &value, "name",
                )?,
            ),
            skeleton_name: clean_identity(
                &required_string(
                    &value,
                    "skeleton_name",
                )?,
            ),
            prop_names,
        },
    )
}

/// Decode one complete JSON component file.
pub(super) fn read_json(path: &Path) -> Result<Value, PipelineError> {
    let bytes = fs::read(path).map_err(
        |error| {
            PipelineError::new(
                format!(
                    "prop component read failed for {}: {error}",
                    path.display()
                ),
            )
        },
    )?;
    serde_json::from_slice(&bytes).map_err(
        |error| {
            PipelineError::new(
                format!(
                    "prop component JSON failed for {}: {error}",
                    path.display()
                ),
            )
        },
    )
}

/// Build one lowercase portable asset name with a deterministic limit.
///
/// # Errors
///
/// Returns the supplied error when the identity has no portable characters.
pub(super) fn portable_asset_name(
    value: &str,
    maximum: usize,
    empty_error: &str,
) -> Result<String, PipelineError> {
    let mut output = String::new();
    let mut previous_dash = false;
    for character in value
        .chars()
        .flat_map(char::to_lowercase)
    {
        let normalized = if character.is_ascii_alphanumeric() {
            character
        } else {
            '-'
        };
        if normalized == '-' {
            if previous_dash || output.is_empty() {
                continue;
            }
            previous_dash = true;
        } else {
            previous_dash = false;
        }
        output.push(normalized);
        if output.len() == maximum {
            break;
        }
    }
    while output.ends_with('-') {
        let _removed_character = output.pop();
    }
    if output.is_empty() {
        return Err(PipelineError::new(empty_error));
    }
    Ok(output)
}

/// Read one required JSON string field.
pub(super) fn required_string(
    value: &Value,
    field: &str,
) -> Result<String, PipelineError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_owned)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!("prop JSON field is not a string: {field}"),
                )
            },
        )
}

/// Read one required non-negative integer field.
pub(super) fn required_usize(
    value: &Value,
    field: &str,
) -> Result<usize, PipelineError> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|number| usize::try_from(number).ok())
        .ok_or_else(
            || {
                PipelineError::new(
                    format!("prop JSON field is not a usize: {field}"),
                )
            },
        )
}

/// Convert one ledger path into a member id for the required family.
pub(super) fn ledger_member_id(
    path: &str,
    family: &str,
) -> Result<String, PipelineError> {
    let prefix = format!("{family}/");
    let member = path
        .strip_prefix(&prefix)
        .and_then(|value| value.strip_suffix(".json"))
        .ok_or_else(
            || {
                PipelineError::new(
                    format!("prop ledger path does not match {family}: {path}"),
                )
            },
        )?;
    if member.is_empty() || member.contains('/') || member.contains('\\') {
        return Err(
            PipelineError::new(
                format!("prop member id is not one path segment: {member}"),
            ),
        );
    }
    Ok(member.to_owned())
}

/// Return one component member id from its JSON path.
pub(super) fn component_member_id(
    path: &Path
) -> Result<String, PipelineError> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "prop component path has no UTF-8 member id: {}",
                        path.display()
                    ),
                )
            },
        )
}

/// Remove fixed-width decoded identity padding without changing inner text.
pub(super) fn clean_identity(value: &str) -> String {
    let mut cleaned = value
        .trim()
        .to_owned();
    loop {
        let next = cleaned
            .trim_end_matches(r"\x00")
            .trim_end_matches(r"\u0000")
            .trim_end_matches('\0')
            .trim_end()
            .to_owned();
        if next == cleaned {
            return cleaned;
        }
        cleaned = next;
    }
}

#[cfg(test)]
mod tests {
    use super::clean_identity;

    #[test]
    fn decoded_identity_padding_is_removed() {
        assert_eq!(
            clean_identity("PTRN_flag\x00\x00"),
            "PTRN_flag"
        );
        assert_eq!(
            clean_identity("flag\0\0"),
            "flag"
        );
    }
}
