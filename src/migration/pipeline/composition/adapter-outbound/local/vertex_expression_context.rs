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
//   - Package-local runtime context for vertex-expression mixers.
// - Must-Not:
//   - Reject legal expression keys merely because no offset list matches.
//   - Require expression-group target names to equal mixer target names.
// - Allows:
//   - Resolve exact normalized mixer, group, skin, and key-index relationships.
// - Split-When:
//   - Morph-target construction gains an independent publication lifecycle.
// - Merge-When:
//   - Another adapter owns the identical expression-to-offset runtime join.
// - Summary:
//   - Bind vertex-expression keys to their runtime-scanned skin offset lists.
// - Description:
//   - Mirrors shipped VertexOffset mixer lookup without changing source JSON.
// - Usage:
//   - Called by prepare-unreal before semantic package planning is published.
// - Defaults:
//   - Missing or ambiguous package-local mixer targets fail migration
//     preflight.
//

//! Vertex-expression runtime relationship preflight.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::resolve_under;
use serde_json::Value;

use crate::domain::{
    PhaseThreePackageIndex, PipelineError, PipelineOutcome,
};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct VertexExpressionKeyContext {
    pub(super) package_id: String,
    pub(super) mixer_ordinal: usize,
    pub(super) group_ordinal: usize,
    pub(super) skin_ordinal: usize,
    pub(super) expression_index: usize,
    pub(super) key_ordinal: usize,
    pub(super) key_index: u32,
    pub(super) key_value: f64,
    pub(super) offset_list_indices: Vec<usize>,
    pub(super) offset_count: usize,
    pub(super) group_target_matches_mixer: bool,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct VertexExpressionPreflight {
    package_ids: BTreeSet<String>,
    mixer_count: usize,
    keys: Vec<VertexExpressionKeyContext>,
}

impl VertexExpressionPreflight {
    pub(super) fn package_count(&self) -> usize {
        self.package_ids.len()
    }

    pub(super) const fn mixer_count(&self) -> usize {
        self.mixer_count
    }

    pub(super) const fn key_count(&self) -> usize {
        self.keys.len()
    }

    pub(super) fn matched_key_count(&self) -> usize {
        self.keys
            .iter()
            .filter(|key| !key.offset_list_indices.is_empty())
            .count()
    }
}

#[derive(Clone, Debug)]
struct Component {
    ordinal: usize,
    kind: String,
    payload: Value,
}

/// Resolve every indexed package that contains a vertex-expression mixer.
///
/// # Errors
///
/// Returns an error when normalized component evidence is malformed or a mixer
/// cannot resolve its group and target skin in the package migration context.
pub(super) fn preflight_vertex_expression_context(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
) -> PipelineOutcome<VertexExpressionPreflight> {
    let mut report = VertexExpressionPreflight::default();
    for package in index.packages() {
        let indexed_mixers = package
            .members()
            .iter()
            .filter(|member| {
                member.source_chunk_kind == "vertex_expression_mixer"
            })
            .count();
        if indexed_mixers == 0 {
            continue;
        }
        let root = resolve_normalized_package_root(
            extracted_root,
            &package.package_root,
        )?;
        let mut package_report = preflight_vertex_expression_package(
            &root,
            &package.package_id,
        )?;
        if package_report.mixer_count != indexed_mixers {
            return Err(PipelineError::new(
                "vertex-expression package index disagrees with mixer evidence",
            ));
        }
        let _inserted = report.package_ids.insert(package.package_id.clone());
        report.mixer_count = report
            .mixer_count
            .checked_add(package_report.mixer_count)
            .ok_or_else(|| {
                PipelineError::new("vertex-expression mixer count overflowed")
            })?;
        report.keys.append(&mut package_report.keys);
    }
    report.keys.sort_by_key(|key| {
        (
            key.package_id.clone(),
            key.mixer_ordinal,
            key.expression_index,
            key.key_ordinal,
        )
    });
    Ok(report)
}

fn preflight_vertex_expression_package(
    root: &Path,
    package_id: &str,
) -> PipelineOutcome<VertexExpressionPreflight> {
    let components = read_components(root)?;
    let groups = identity_map(&components, "vertex_expression_group")?;
    let skins = identity_map(&components, "skin")?;
    let mut report = VertexExpressionPreflight::default();
    for mixer in components
        .iter()
        .filter(|component| component.kind == "vertex_expression_mixer")
    {
        if required_u32(&mixer.payload, "type")? != 3 {
            return Err(PipelineError::new(
                "vertex-expression mixer is not a VertexOffset mixer",
            ));
        }
        let group_name = clean_identity(required_string(
            &mixer.payload,
            "expression_group_name",
        )?)?;
        let target_name = clean_identity(required_string(
            &mixer.payload,
            "target_name",
        )?)?;
        let group = resolve_unique(&groups, &group_name, "expression group")?;
        let skin = resolve_unique(&skins, &target_name, "target skin")?;
        push_mixer_keys(package_id, mixer, group, skin, &mut report.keys)?;
        report.mixer_count = report.mixer_count.checked_add(1).ok_or_else(|| {
            PipelineError::new("vertex-expression mixer count overflowed")
        })?;
    }
    if report.mixer_count > 0 {
        let _inserted = report.package_ids.insert(package_id.to_owned());
    }
    Ok(report)
}

fn push_mixer_keys(
    package_id: &str,
    mixer: &Component,
    group: &Component,
    skin: &Component,
    out: &mut Vec<VertexExpressionKeyContext>,
) -> PipelineOutcome<()> {
    let group_target = clean_identity(required_string(
        &group.payload,
        "target_name",
    )?)?;
    let mixer_target = clean_identity(required_string(
        &mixer.payload,
        "target_name",
    )?)?;
    let expressions = required_array(&group.payload, "expressions")?;
    let offsets = skin
        .payload
        .get("expression_offsets")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(
                "vertex-expression target skin has no expression offsets",
            )
        })?;
    let lists = offsets
        .get("offset_lists")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new("skin expression offsets have no offset lists")
        })?;
    if lists.is_empty() {
        return Err(PipelineError::new(
            "vertex-expression target skin has no offset lists",
        ));
    }
    for (expression_index, expression) in expressions.iter().enumerate() {
        let keys = required_array(expression, "keys")?;
        let indices = required_array(expression, "indices")?;
        if keys.len() != indices.len() {
            return Err(PipelineError::new(
                "vertex-expression key values and indices disagree",
            ));
        }
        for (key_ordinal, (key, index)) in
            keys.iter().zip(indices).enumerate()
        {
            let key_value = key.as_f64().ok_or_else(|| {
                PipelineError::new("vertex-expression key is not numeric")
            })?;
            let key_index = index
                .as_u64()
                .and_then(|value| u32::try_from(value).ok())
                .ok_or_else(|| {
                    PipelineError::new("vertex-expression key index is invalid")
                })?;
            let mut offset_list_indices = Vec::new();
            let mut offset_count = 0usize;
            for (list_index, list) in lists.iter().enumerate() {
                if required_u32(list, "key_index")? != key_index {
                    continue;
                }
                offset_list_indices.push(list_index);
                let count = required_array(list, "offsets")?.len();
                offset_count = offset_count.checked_add(count).ok_or_else(|| {
                    PipelineError::new(
                        "vertex-expression offset count overflowed",
                    )
                })?;
            }
            out.push(VertexExpressionKeyContext {
                package_id: package_id.to_owned(),
                mixer_ordinal: mixer.ordinal,
                group_ordinal: group.ordinal,
                skin_ordinal: skin.ordinal,
                expression_index,
                key_ordinal,
                key_index,
                key_value,
                offset_list_indices,
                offset_count,
                group_target_matches_mixer: group_target == mixer_target,
            });
        }
    }
    Ok(())
}

fn read_components(root: &Path) -> PipelineOutcome<Vec<Component>> {
    let ledger_path = root.join("components.jsonl");
    let ledger = fs::read_to_string(&ledger_path).map_err(|error| {
        PipelineError::new(format!(
            "vertex-expression ledger read failed: {error}"
        ))
    })?;
    let components_root = root.join("components");
    let mut components = Vec::new();
    let mut ordinals = BTreeSet::new();
    for line in ledger.lines().skip(1) {
        let row = serde_json::from_str::<Value>(line).map_err(|error| {
            PipelineError::new(format!(
                "vertex-expression ledger JSON failed: {error}"
            ))
        })?;
        let kind = required_string(&row, "kind")?.to_owned();
        if !matches!(
            kind.as_str(),
            "vertex_expression_mixer" | "vertex_expression_group" | "skin"
        ) {
            continue;
        }
        let ordinal = required_usize(&row, "ordinal")?;
        if !ordinals.insert(ordinal) {
            return Err(PipelineError::new(
                "vertex-expression component ordinal is duplicated",
            ));
        }
        let relative = required_string(&row, "path")?;
        let path = resolve_under(&components_root, Path::new(relative))
            .map_err(|_error| {
                PipelineError::new(
                    "vertex-expression component path escapes package",
                )
            })?;
        let payload = serde_json::from_slice::<Value>(
            &fs::read(&path).map_err(|error| {
                PipelineError::new(format!(
                    "vertex-expression component read failed: {error}"
                ))
            })?,
        )
        .map_err(|error| {
            PipelineError::new(format!(
                "vertex-expression component JSON failed: {error}"
            ))
        })?;
        if required_string(&payload, "schema")? != kind {
            return Err(PipelineError::new(
                "vertex-expression component schema disagrees with ledger",
            ));
        }
        components.push(Component { ordinal, kind, payload });
    }
    Ok(components)
}

fn identity_map<'a>(
    components: &'a [Component],
    kind: &str,
) -> PipelineOutcome<BTreeMap<String, Vec<&'a Component>>> {
    let mut by_name = BTreeMap::<String, Vec<&Component>>::new();
    for component in components.iter().filter(|item| item.kind == kind) {
        let name = clean_identity(required_string(
            &component.payload,
            "name",
        )?)?;
        by_name.entry(name).or_default().push(component);
    }
    Ok(by_name)
}

fn resolve_unique<'a>(
    identities: &'a BTreeMap<String, Vec<&'a Component>>,
    name: &str,
    label: &str,
) -> PipelineOutcome<&'a Component> {
    let Some(matches) = identities.get(name) else {
        return Err(PipelineError::new(format!(
            "vertex-expression migration context has no package-local {label}"
        )));
    };
    match matches.as_slice() {
        [component] => Ok(*component),
        _ => Err(PipelineError::new(format!(
            "vertex-expression package-local {label} is ambiguous"
        ))),
    }
}

fn clean_identity(value: &str) -> PipelineOutcome<String> {
    let cleaned = value.trim_end_matches(char::from(0));
    if cleaned.is_empty()
        || cleaned != cleaned.trim()
        || cleaned.chars().any(char::is_control)
    {
        return Err(PipelineError::new(
            "vertex-expression identity is malformed",
        ));
    }
    Ok(cleaned.to_owned())
}

fn resolve_normalized_package_root(
    extracted_root: &Path,
    published_root: &str,
) -> PipelineOutcome<PathBuf> {
    let root_name = extracted_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new(
                "vertex-expression extracted root has no portable basename",
            )
        })?;
    let prefix = format!("{root_name}/");
    let relative = published_root.strip_prefix(&prefix).ok_or_else(|| {
        PipelineError::new(
            "vertex-expression package root is outside extracted evidence",
        )
    })?;
    resolve_under(extracted_root, Path::new(relative)).map_err(|_error| {
        PipelineError::new("vertex-expression package root escapes evidence")
    })
}

fn required_string<'a>(
    value: &'a Value,
    field: &str,
) -> PipelineOutcome<&'a str> {
    value.get(field).and_then(Value::as_str).ok_or_else(|| {
        PipelineError::new(format!(
            "vertex-expression {field} is not a string"
        ))
    })
}

fn required_array<'a>(
    value: &'a Value,
    field: &str,
) -> PipelineOutcome<&'a Vec<Value>> {
    value.get(field).and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new(format!(
            "vertex-expression {field} is not an array"
        ))
    })
}

fn required_usize(value: &Value, field: &str) -> PipelineOutcome<usize> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| {
            PipelineError::new(format!(
                "vertex-expression {field} is not an integer"
            ))
        })
}

fn required_u32(value: &Value, field: &str) -> PipelineOutcome<u32> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| {
            PipelineError::new(format!(
                "vertex-expression {field} is not a u32"
            ))
        })
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/vertex_expression_context_tests.rs"]
mod tests;
