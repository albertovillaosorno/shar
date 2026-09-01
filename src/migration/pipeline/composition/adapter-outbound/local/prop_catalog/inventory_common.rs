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
//   - Inventory common outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Inventory common outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Inventory common outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::domain::PipelineError;

/// One authored composite prop relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CompositePropEvidence {
    /// Cleaned referenced render identity.
    pub(super) name: String,
    /// Authored skeleton joint index.
    pub(super) skeleton_joint_id: usize,
    /// Authored translucency flag.
    pub(super) is_translucent: bool,
    /// Exact authored optional sort order at source f32 width.
    pub(super) sort_order_bits: Option<u32>,
}

/// One authored composite effect relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CompositeEffectEvidence {
    /// Cleaned effect identity.
    pub(super) name: String,
    /// Authored skeleton joint index.
    pub(super) skeleton_joint_id: usize,
    /// Authored translucency flag.
    pub(super) is_translucent: bool,
    /// Exact authored optional sort order at source f32 width.
    pub(super) sort_order_bits: Option<u32>,
}

/// Composite identity, skeleton, and rigid prop/effect references.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CompositeEvidence {
    /// Normalized composite member id without family or extension.
    pub(super) member_id: String,
    /// Cleaned composite identity.
    pub(super) name: String,
    /// Cleaned referenced skeleton identity.
    pub(super) skeleton_name: String,
    /// Cleaned rigid prop identities in authored composite order.
    pub(super) prop_names: Vec<String>,
    /// Authored prop relationships in the same composite order.
    pub(super) prop_bindings: Vec<CompositePropEvidence>,
    /// Authored effect relationships in exact composite effect order.
    pub(super) effect_bindings: Vec<CompositeEffectEvidence>,
}

/// Return all JSON component paths in stable file-name order.
pub(super) fn component_paths(
    root: &Path,
    family: &str,
) -> Result<Vec<PathBuf>, PipelineError> {
    let directory = root.join("components").join(family);
    if !directory.is_dir() {
        return Ok(Vec::new());
    }
    let mut paths = fs::read_dir(&directory)
        .map_err(|error| {
            PipelineError::new(format!(
                "prop component directory read failed for {}: {error}",
                directory.display()
            ))
        })?
        .map(|entry| {
            entry
                .map(|value| value.path())
                .map_err(|error| PipelineError::new(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    paths.retain(|path| {
        path.extension()
            .is_some_and(|extension| extension == "json")
    });
    paths.sort();
    Ok(paths)
}

/// Map decoded component identity to its normalized member id.
pub(super) fn component_name_map(
    root: &Path,
    family: &str,
) -> Result<BTreeMap<String, String>, PipelineError> {
    let mut result = BTreeMap::new();
    for path in component_paths(root, family)? {
        let name = read_component_name(&path)?;
        let id = component_member_id(&path)?;
        if result.insert(name.clone(), id).is_some() {
            return Err(PipelineError::new(format!(
                "prop component identity is duplicated: \
                         {family}/{name}"
            )));
        }
    }
    Ok(result)
}

/// Read one component's normalized name.
pub(super) fn read_component_name(
    path: &Path,
) -> Result<String, PipelineError> {
    let value = read_json(path)?;
    clean_identity(&required_string(&value, "name")?)
}

/// Read one composite and its rigid mesh references.
pub(super) fn read_composite(
    path: &Path,
) -> Result<CompositeEvidence, PipelineError> {
    let value = read_json(path)?;
    let props =
        value
            .get("props")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "prop composite has no props array: {}",
                    path.display()
                ))
            })?;
    let mut seen = BTreeSet::new();
    let mut prop_names = Vec::with_capacity(props.len());
    let mut prop_bindings = Vec::with_capacity(props.len());
    for prop in props {
        let raw_name = prop
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "prop composite has malformed prop name: \
                                     {}",
                    path.display()
                ))
            })?;
        let name = clean_identity(raw_name)?;
        if !seen.insert(name.clone()) {
            return Err(PipelineError::new(format!(
                "prop composite repeats prop identity {name}: {}",
                path.display()
            )));
        }
        let skeleton_joint_id = required_usize(prop, "skeleton_joint_id")?;
        let translucent = required_usize(prop, "is_translucent")?;
        let sort_order_bits = optional_f32_bits(prop, "sort_order")?;
        if translucent > 1 {
            return Err(PipelineError::new(format!(
                "prop composite translucency is not a boolean: {}",
                path.display()
            )));
        }
        prop_names.push(name.clone());
        prop_bindings.push(CompositePropEvidence {
            name,
            skeleton_joint_id,
            is_translucent: translucent == 1,
            sort_order_bits,
        });
    }
    let effects = value
        .get("effects")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "prop composite has no effects array: {}",
                path.display()
            ))
        })?;
    let mut effect_bindings = Vec::with_capacity(effects.len());
    for effect in effects {
        if required_string(effect, "kind")? != "effect" {
            return Err(PipelineError::new(format!(
                "prop composite has unsupported effect kind: {}",
                path.display()
            )));
        }
        let name = clean_identity(&required_string(effect, "name")?)?;
        let skeleton_joint_id = required_usize(effect, "skeleton_joint_id")?;
        let translucent = required_usize(effect, "is_translucent")?;
        if translucent > 1 {
            return Err(PipelineError::new(format!(
                "prop composite effect translucency is not a boolean: {}",
                path.display()
            )));
        }
        effect_bindings.push(CompositeEffectEvidence {
            name,
            skeleton_joint_id,
            is_translucent: translucent == 1,
            sort_order_bits: optional_f32_bits(effect, "sort_order")?,
        });
    }
    Ok(CompositeEvidence {
        member_id: component_member_id(path)?,
        name: clean_identity(&required_string(&value, "name")?)?,
        skeleton_name: clean_identity(&required_string(
            &value,
            "skeleton_name",
        )?)?,
        prop_names,
        prop_bindings,
        effect_bindings,
    })
}

/// Decode one complete JSON component file.
pub(super) fn read_json(path: &Path) -> Result<Value, PipelineError> {
    let bytes = fs::read(path).map_err(|error| {
        PipelineError::new(format!(
            "prop component read failed for {}: {error}",
            path.display()
        ))
    })?;
    serde_json::from_slice(&bytes).map_err(|error| {
        PipelineError::new(format!(
            "prop component JSON failed for {}: {error}",
            path.display()
        ))
    })
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
    for character in value.chars().flat_map(char::to_lowercase) {
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
        .ok_or_else(|| {
            PipelineError::new(format!(
                "prop JSON field is not a string: {field}"
            ))
        })
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
        .ok_or_else(|| {
            PipelineError::new(format!(
                "prop JSON field is not a usize: {field}"
            ))
        })
}

/// Read one optional finite JSON number at exact source f32 width.
fn optional_f32_bits(
    value: &Value,
    field: &str,
) -> Result<Option<u32>, PipelineError> {
    let Some(raw) = value.get(field) else {
        return Ok(None);
    };
    let parsed = raw
        .to_string()
        .parse::<f32>()
        .map_err(|error| {
            PipelineError::new(format!(
                "prop JSON field is not an f32: {field}: {error}"
            ))
        })?;
    if !parsed.is_finite() {
        return Err(PipelineError::new(format!(
            "prop JSON field is not finite: {field}"
        )));
    }
    Ok(Some(parsed.to_bits()))
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
        .ok_or_else(|| {
            PipelineError::new(format!(
                "prop ledger path does not match {family}: {path}"
            ))
        })?;
    if member.is_empty() || member.contains('/') || member.contains('\\') {
        return Err(PipelineError::new(format!(
            "prop member id is not one path segment: {member}"
        )));
    }
    Ok(member.to_owned())
}

/// Return one component member id from its JSON path.
pub(super) fn component_member_id(
    path: &Path,
) -> Result<String, PipelineError> {
    path.file_stem()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "prop component path has no UTF-8 member id: {}",
                path.display()
            ))
        })
}

/// Remove fixed-width decoded NUL padding without repairing source identity.
///
/// # Errors
///
/// Returns an error when the unpadded identity is empty, space-padded, or
/// contains a control character.
pub(super) fn clean_identity(value: &str) -> Result<String, PipelineError> {
    let mut cleaned = value.to_owned();
    loop {
        let next = cleaned
            .trim_end_matches(r"\x00")
            .trim_end_matches(r"\u0000")
            .trim_end_matches('\0')
            .to_owned();
        if next == cleaned {
            break;
        }
        cleaned = next;
    }
    if cleaned.is_empty()
        || cleaned != cleaned.trim()
        || cleaned.chars().any(char::is_control)
    {
        return Err(PipelineError::new(
            "prop source identity is non-canonical",
        ));
    }
    Ok(cleaned)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/inventory_common/tests.rs"]
mod tests;
