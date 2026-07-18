// File:
//   - scenegraph.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     scenegraph.rs
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
//   - Authored drawable placement discovery from normalized scenegraphs.
// - Must-Not:
//   - Load mesh payloads, infer missing transforms, or mutate geometry.
// - Allows:
//   - Recursive scenegraph traversal and exact drawable-name grouping.
// - Summary:
//   - Maps drawable identities to row-vector world matrices.
//
// ADRs:
// - docs/adr/pipeline/unreal/world-assembly-from-normalized-chunks.md
//
// Large file:
//   - false
//

//! Authored drawable placement discovery from normalized scenegraphs.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

use super::super::inventory_common::clean_identity;
use super::transform::{Matrix, identity, matrix_key, multiply};
use crate::domain::PipelineError;

/// Read every normalized scenegraph and group exact drawable transforms.
///
/// # Errors
///
/// Returns an error when a candidate JSON document or transform matrix is
/// malformed.
pub(super) fn placement_map(
    package_root: &Path
) -> Result<BTreeMap<String, Vec<Matrix>>, PipelineError> {
    let components = package_root.join("components");
    let mut placements = BTreeMap::<String, Vec<Matrix>>::new();
    for family in [
        "scenegraph",
        "srr_insta_entity_dsg",
        "srr_insta_static_phys_dsg",
        "srr_dyna_phys_dsg",
        "srr_insta_anim_dyna_phys_dsg",
        "srr_static_phys_dsg",
    ] {
        let directory = components.join(family);
        if !directory.is_dir() {
            continue;
        }
        for path in json_files(&directory)? {
            let bytes = fs::read(&path).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "world level scenegraph read failed for {}: \
                             {error}",
                            path.display()
                        ),
                    )
                },
            )?;
            let value: Value = serde_json::from_slice(&bytes).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "world level scenegraph JSON failed for {}: \
                             {error}",
                            path.display()
                        ),
                    )
                },
            )?;
            collect_scenegraphs(
                &value,
                &mut placements,
            )?;
        }
    }
    for matrices in placements.values_mut() {
        matrices.sort_by_key(matrix_key);
        matrices.dedup_by_key(|matrix| matrix_key(matrix));
    }
    Ok(placements)
}

/// List canonical JSON files in one normalized component family.
fn json_files(directory: &Path) -> Result<Vec<PathBuf>, PipelineError> {
    let mut files = fs::read_dir(directory)
        .map_err(|error| PipelineError::new(error.to_string()))?
        .map(
            |entry| {
                entry
                    .map(|value| value.path())
                    .map_err(|error| PipelineError::new(error.to_string()))
            },
        )
        .collect::<Result<Vec<_>, _>>()?;
    files.retain(
        |path| {
            path.extension()
                .and_then(|value| value.to_str())
                == Some("json")
        },
    );
    files.sort();
    Ok(files)
}

/// Recursively discover every embedded scenegraph value.
fn collect_scenegraphs(
    value: &Value,
    placements: &mut BTreeMap<String, Vec<Matrix>>,
) -> Result<(), PipelineError> {
    match value {
        Value::Object(object) => {
            if object
                .get("schema")
                .and_then(Value::as_str)
                == Some("scenegraph")
            {
                if let Some(roots) = object
                    .get("roots")
                    .and_then(Value::as_array)
                {
                    for root in roots {
                        walk_node(
                            root,
                            &identity(),
                            placements,
                        )?;
                    }
                }
                return Ok(());
            }
            for child in object.values() {
                collect_scenegraphs(
                    child, placements,
                )?;
            }
        }
        Value::Array(values) => {
            for child in values {
                collect_scenegraphs(
                    child, placements,
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

/// Walk one scenegraph node and collect drawable transforms.
fn walk_node(
    node: &Value,
    parent: &Matrix,
    placements: &mut BTreeMap<String, Vec<Matrix>>,
) -> Result<(), PipelineError> {
    let object = node
        .as_object()
        .ok_or_else(
            || PipelineError::new("world scenegraph node is not an object"),
        )?;
    let kind = object
        .get("kind")
        .and_then(Value::as_str)
        .ok_or_else(
            || PipelineError::new("world scenegraph node has no kind"),
        )?;
    let current = if kind == "transform" {
        multiply(
            &matrix_value(
                object
                    .get("matrix")
                    .ok_or_else(
                        || PipelineError::new("world transform has no matrix"),
                    )?,
            )?,
            parent,
        )
    } else {
        *parent
    };
    if kind == "drawable" {
        let drawable = object
            .get("drawable_name")
            .or_else(|| object.get("name"))
            .and_then(Value::as_str)
            .ok_or_else(
                || PipelineError::new("world drawable has no identity"),
            )?;
        placements
            .entry(clean_identity(drawable))
            .or_default()
            .push(current);
    }
    if let Some(children) = object
        .get("children")
        .and_then(Value::as_array)
    {
        for child in children {
            walk_node(
                child, &current, placements,
            )?;
        }
    }
    Ok(())
}

/// Decode one exact sixteen-component affine matrix.
fn matrix_value(value: &Value) -> Result<Matrix, PipelineError> {
    let values = value
        .as_array()
        .ok_or_else(
            || PipelineError::new("world transform matrix is not an array"),
        )?;
    if values.len() != 16 {
        return Err(PipelineError::new("world transform matrix is not 4x4"));
    }
    let mut matrix = [0.0_f32; 16];
    for (target, source) in matrix
        .iter_mut()
        .zip(values.iter())
    {
        let component = source
            .as_f64()
            .ok_or_else(
                || {
                    PipelineError::new(
                        "world transform component is not numeric",
                    )
                },
            )?;
        let rendered = component.to_string();
        *target = rendered
            .parse::<f32>()
            .map_err(
                |error| {
                    PipelineError::new(
                        format!("world transform component failed: {error}"),
                    )
                },
            )?;
    }
    Ok(matrix)
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use serde_json::json;

    use super::collect_scenegraphs;

    #[test]
    fn nested_transform_places_one_drawable() -> Result<(), String> {
        let value = json!({
            "schema": "scenegraph",
            "roots": [{
                "kind": "transform",
                "matrix": [
                    1_i32, 0_i32, 0_i32, 0_i32,
                    0_i32, 1_i32, 0_i32, 0_i32,
                    0_i32, 0_i32, 1_i32, 0_i32,
                    4_i32, 5_i32, 6_i32, 1_i32
                ],
                "children": [{
                    "kind": "drawable",
                    "drawable_name": "house"
                }]
            }]
        });
        let mut placements = BTreeMap::new();
        collect_scenegraphs(
            &value,
            &mut placements,
        )
        .map_err(|error| error.to_string())?;
        let [matrix] = placements
            .get("house")
            .map(Vec::as_slice)
            .ok_or_else(|| "house placement is missing".to_owned())?
        else {
            return Err("house placement count is not one".to_owned());
        };
        if matrix[12..15]
            != [
                4.0, 5.0, 6.0,
            ]
        {
            return Err("house translation was not preserved".to_owned());
        }
        Ok(())
    }
}
