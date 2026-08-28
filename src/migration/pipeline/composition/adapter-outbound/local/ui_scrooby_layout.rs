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
//   - Deterministic Scrooby runtime-layout evidence publication.
// - Must-Not:
//   - Copy authored names or text, invent Unreal widgets, or reinterpret fields
//     the source runtime ignores.
// - Allows:
//   - Preserve source ordering and source-runtime screen-space semantics.
// - Split-When:
//   - Unreal WidgetBlueprint construction gains an independent lifecycle.
// - Merge-When:
//   - Another adapter owns the identical normalized Scrooby layout artifact.
// - Summary:
//   - Publish public-safe Scrooby layout semantics.
// - Description:
//   - Compiles normalized Scrooby hierarchy into package-scoped ordered layout
//     rows while retaining raw fields beside runtime interpretations.
// - Usage:
//   - Called by prepare-unreal after Scrooby reference preflight.
// - Defaults:
//   - Unsupported justification or malformed screen geometry fails closed.
//

//! Scrooby runtime-layout evidence publication.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::resolve_under;
use serde_json::{Map, Value, json};

use crate::domain::{PhaseThreePackageIndex, PipelineError, PipelineOutcome};

const SCHEMA: &str = "shar-schoenwald.scrooby-layout-catalog.v2";
const FILE: &str = "layout.jsonl";

#[derive(Clone, Debug)]
struct Component {
    ordinal: usize,
    parent_ordinal: Option<usize>,
    kind: String,
    payload: Value,
}

pub(super) fn publish_scrooby_layout_catalog(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
    output_root: &Path,
) -> PipelineOutcome<usize> {
    let rows = collect_layout_catalog(index, extracted_root)?;
    let rendered = render_catalog(&rows)?;
    publish_rendered(output_root, &rendered)?;
    Ok(rows.len())
}

fn collect_layout_catalog(
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
) -> PipelineOutcome<Vec<Value>> {
    let mut rows = Vec::new();
    for package in index.packages() {
        if !package
            .members()
            .iter()
            .any(|member| member.source_chunk_kind == "scrooby_project")
        {
            continue;
        }
        let root = resolve_package_root(extracted_root, &package.package_root)?;
        for mut row in collect_package_layout(&root)? {
            let object = row.as_object_mut().ok_or_else(|| {
                PipelineError::new("Scrooby layout row is not an object")
            })?;
            let _previous = object.insert(
                "package_id".to_owned(),
                Value::String(package.package_id.clone()),
            );
            rows.push(row);
        }
    }
    rows.sort_by(|left, right| {
        let left_package = left
            .get("package_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let right_package = right
            .get("package_id")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let left_ordinal = left
            .get("ordinal")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        let right_ordinal = right
            .get("ordinal")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        (left_package, left_ordinal).cmp(&(right_package, right_ordinal))
    });
    Ok(rows)
}

fn collect_package_layout(root: &Path) -> PipelineOutcome<Vec<Value>> {
    let components = read_components(root)?;
    let by_ordinal = components
        .iter()
        .map(|component| (component.ordinal, component))
        .collect::<BTreeMap<_, _>>();
    let mut siblings = BTreeMap::<usize, Vec<&Component>>::new();
    for component in &components {
        if let Some(parent) = component.parent_ordinal {
            siblings.entry(parent).or_default().push(component);
        }
    }
    for children in siblings.values_mut() {
        children.sort_by_key(|component| component.ordinal);
    }

    let mut rows = Vec::new();
    for component in components
        .iter()
        .filter(|item| is_layout_kind(&item.kind))
    {
        let source_sibling_index = source_sibling_index(component, &siblings)?;
        let runtime_index = runtime_index(component, &by_ordinal, &siblings)?;
        let mut row = Map::new();
        let _previous = row.insert(
            "kind".to_owned(),
            Value::String(component.kind.clone()),
        );
        let _previous =
            row.insert("ordinal".to_owned(), json!(component.ordinal));
        let _previous = row.insert(
            "parent_ordinal".to_owned(),
            json!(component.parent_ordinal),
        );
        let _previous = row.insert(
            "source_sibling_index".to_owned(),
            json!(source_sibling_index),
        );
        let _previous =
            row.insert("runtime_index".to_owned(), json!(runtime_index));
        add_semantics(component, &mut row)?;
        rows.push(Value::Object(row));
    }
    Ok(rows)
}

fn add_semantics(
    component: &Component,
    row: &mut Map<String, Value>,
) -> PipelineOutcome<()> {
    match component.kind.as_str() {
        "scrooby_project" | "scrooby_page" => {
            let resolution =
                required_u32_pair(&component.payload, "resolution")?;
            let _previous = row.insert("canvas".to_owned(), json!(resolution));
        },
        "scrooby_screen" => {},
        "scrooby_layer" => {
            let visible = required_u32(&component.payload, "visible")?;
            if visible > 1 {
                return Err(PipelineError::new(
                    "Scrooby layer visibility is invalid",
                ));
            }
            let _previous =
                row.insert("visible".to_owned(), json!(visible == 1));
            let _previous = row.insert(
                "raw_editable_u32".to_owned(),
                json!(required_u32(&component.payload, "editable")?),
            );
            let _previous = row.insert(
                "raw_alpha_u32".to_owned(),
                json!(required_u32(&component.payload, "alpha")?),
            );
        },
        "scrooby_group" => {
            let _previous = row.insert(
                "raw_version_u32".to_owned(),
                json!(required_u32(&component.payload, "version")?),
            );
            let _previous = row.insert(
                "raw_alpha_u32".to_owned(),
                json!(required_u32(&component.payload, "alpha")?),
            );
            let _previous = row.insert(
                "origin_policy".to_owned(),
                Value::String("min-child-bounds-on-first-show".to_owned()),
            );
        },
        "scrooby_multi_sprite" => {
            add_widget_frame(&component.payload, row)?;
            let _previous = row.insert("initial_index".to_owned(), json!(0));
            let _previous = row.insert(
                "image_count".to_owned(),
                json!(required_u32(&component.payload, "image_count")?),
            );
        },
        "scrooby_multi_text" => {
            add_widget_frame(&component.payload, row)?;
            let shadow = required_u32(&component.payload, "shadow_enabled")?;
            if shadow > 1 {
                return Err(PipelineError::new(
                    "Scrooby text shadow flag is invalid",
                ));
            }
            let _previous =
                row.insert("shadow_enabled".to_owned(), json!(shadow == 1));
            let shadow_color =
                required_u32(&component.payload, "shadow_color")?;
            let _previous = row.insert(
                "shadow_color_raw_u32".to_owned(),
                json!(shadow_color),
            );
            let _previous = row.insert(
                "shadow_color_rgba_u8".to_owned(),
                json!(packed_rgba_u8(shadow_color)),
            );
            let shadow_offset =
                required_u32_pair(&component.payload, "shadow_offset")?;
            let _previous = row.insert(
                "shadow_offset_i32".to_owned(),
                json!([
                    semantic_i32(shadow_offset[0]),
                    semantic_i32(shadow_offset[1]),
                ]),
            );
            let _previous = row.insert(
                "current_text_i32".to_owned(),
                json!(semantic_i32(required_u32(
                    &component.payload,
                    "current_text",
                )?)),
            );
        },
        "scrooby_pure3d_object" => add_widget_frame(&component.payload, row)?,
        "scrooby_polygon" => add_polygon(&component.payload, row)?,
        "scrooby_string_text_bible" | "scrooby_string_hardcoded" => {},
        _ => {
            return Err(PipelineError::new("unsupported Scrooby layout kind"));
        },
    }
    Ok(())
}

fn add_widget_frame(
    payload: &Value,
    row: &mut Map<String, Value>,
) -> PipelineOutcome<()> {
    let position = required_u32_pair(payload, "position")?;
    let dimensions = required_u32_pair(payload, "dimensions")?;
    let justification = required_u32_pair(payload, "justification")?;
    let signed_dimensions = [
        semantic_i32(dimensions[0]),
        semantic_i32(dimensions[1]),
    ];
    if signed_dimensions[0] <= 0 || signed_dimensions[1] <= 0 {
        return Err(PipelineError::new(
            "Scrooby widget dimensions are non-positive",
        ));
    }
    let _previous = row.insert("position_raw_u32".to_owned(), json!(position));
    let _previous = row.insert(
        "position_i32".to_owned(),
        json!([semantic_i32(position[0]), semantic_i32(position[1])]),
    );
    let _previous =
        row.insert("dimensions_raw_u32".to_owned(), json!(dimensions));
    let _previous =
        row.insert("dimensions_i32".to_owned(), json!(signed_dimensions));
    let _previous = row.insert(
        "justification_raw_u32".to_owned(),
        json!(justification),
    );
    let _previous = row.insert(
        "justification".to_owned(),
        json!([
            horizontal_justification(justification[0])?,
            vertical_justification(justification[1])?,
        ]),
    );
    let color = required_u32(payload, "color")?;
    let _previous =
        row.insert("color_raw_u32".to_owned(), json!(color));
    let _previous =
        row.insert("color_rgba_u8".to_owned(), json!(packed_rgba_u8(color)));
    let _previous = row.insert(
        "raw_translucency_u32".to_owned(),
        json!(required_u32(payload, "translucency")?),
    );
    let _previous = row.insert(
        "raw_rotation_f64".to_owned(),
        json!(required_f64(payload, "rotation")?),
    );
    Ok(())
}

fn add_polygon(
    payload: &Value,
    row: &mut Map<String, Value>,
) -> PipelineOutcome<()> {
    let _previous = row.insert(
        "raw_translucency_u32".to_owned(),
        json!(required_u32(payload, "translucency")?),
    );
    let points = payload.get("points").and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new("Scrooby polygon points are not an array")
    })?;
    let mut raw_points = Vec::with_capacity(points.len());
    let mut screen_points = Vec::with_capacity(points.len());
    for point in points {
        let values = point.as_array().ok_or_else(|| {
            PipelineError::new("Scrooby polygon point is not an array")
        })?;
        let [x, y, z] = values.as_slice() else {
            return Err(PipelineError::new("Scrooby polygon point is not 3D"));
        };
        let x = number_f64(x)?;
        let y = number_f64(y)?;
        let z = number_f64(z)?;
        raw_points.push(json!([x, y, z]));
        screen_points.push(json!([screen_i32(x)?, screen_i32(y)?]));
    }
    if points.len() < 3 {
        return Err(PipelineError::new(
            "Scrooby polygon has fewer than 3 points",
        ));
    }
    let colors = payload.get("colors").and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new("Scrooby polygon colors are not an array")
    })?;
    if colors.len() != points.len() {
        return Err(PipelineError::new("Scrooby polygon color count differs"));
    }
    let colors = colors
        .iter()
        .map(|value| {
            value
                .as_u64()
                .and_then(|value| u32::try_from(value).ok())
                .ok_or_else(|| {
                PipelineError::new("Scrooby polygon color is not u32")
            })
        })
        .collect::<PipelineOutcome<Vec<_>>>()?;
    let _previous =
        row.insert("points_raw".to_owned(), Value::Array(raw_points));
    let _previous = row.insert(
        "screen_points_i32".to_owned(),
        Value::Array(screen_points),
    );
    let rgba = colors
        .iter()
        .copied()
        .map(packed_rgba_u8)
        .collect::<Vec<_>>();
    let _previous = row.insert("colors_raw_u32".to_owned(), json!(colors));
    let _previous = row.insert("colors_rgba_u8".to_owned(), json!(rgba));
    Ok(())
}

fn read_components(root: &Path) -> PipelineOutcome<Vec<Component>> {
    let ledger = fs::read_to_string(root.join("components.jsonl"))
        .map_err(|error| io_error("read Scrooby layout ledger", &error))?;
    let components_root = root.join("components");
    let mut components = Vec::new();
    for line in ledger.lines().skip(1) {
        let value = serde_json::from_str::<Value>(line).map_err(|error| {
            PipelineError::new(format!(
                "Scrooby layout ledger JSON failed: {error}"
            ))
        })?;
        let kind = required_string(&value, "kind")?.to_owned();
        if !kind.starts_with("scrooby_") {
            continue;
        }
        let ordinal = required_usize(&value, "ordinal")?;
        let parent_ordinal = value
            .get("parent_ordinal")
            .and_then(Value::as_u64)
            .and_then(|value| usize::try_from(value).ok());
        let relative = required_string(&value, "path")?;
        let path = resolve_under(&components_root, Path::new(relative))
            .map_err(|_error| {
            PipelineError::new("Scrooby layout component escapes package")
        })?;
        let payload = serde_json::from_slice::<Value>(
            &fs::read(&path).map_err(|error| {
            io_error("read Scrooby layout component", &error)
        })?)
        .map_err(|error| {
            PipelineError::new(format!(
                "Scrooby layout component JSON failed: {error}"
            ))
        })?;
        if payload.get("schema").and_then(Value::as_str)
            != Some(kind.as_str())
        {
            return Err(PipelineError::new(
                "Scrooby layout schema differs from ledger",
            ));
        }
        components.push(Component { ordinal, parent_ordinal, kind, payload });
    }
    components.sort_by_key(|component| component.ordinal);
    Ok(components)
}

fn source_sibling_index(
    component: &Component,
    siblings: &BTreeMap<usize, Vec<&Component>>,
) -> PipelineOutcome<Option<usize>> {
    let Some(parent) = component.parent_ordinal else { return Ok(None) };
    let Some(children) = siblings.get(&parent) else { return Ok(None) };
    children
        .iter()
        .position(|child| child.ordinal == component.ordinal)
        .map(Some)
        .ok_or_else(|| PipelineError::new("Scrooby source sibling is missing"))
}

fn runtime_index(
    component: &Component,
    by_ordinal: &BTreeMap<usize, &Component>,
    siblings: &BTreeMap<usize, Vec<&Component>>,
) -> PipelineOutcome<Option<usize>> {
    let Some(parent_ordinal) = component.parent_ordinal else {
        return Ok(None);
    };
    let Some(parent) = by_ordinal.get(&parent_ordinal) else { return Ok(None) };
    if !runtime_child(&parent.kind, &component.kind) {
        return Ok(None);
    }
    let children = siblings.get(&parent_ordinal).ok_or_else(|| {
        PipelineError::new("Scrooby runtime parent has no children")
    })?;
    Ok(children
        .iter()
        .filter(|child| runtime_child(&parent.kind, &child.kind))
        .position(|child| child.ordinal == component.ordinal))
}

fn runtime_child(parent: &str, child: &str) -> bool {
    match parent {
        "scrooby_project" => child == "scrooby_screen",
        "scrooby_page" => child == "scrooby_layer",
        "scrooby_layer" | "scrooby_group" => matches!(
            child,
            "scrooby_group"
                | "scrooby_multi_sprite"
                | "scrooby_multi_text"
                | "scrooby_pure3d_object"
                | "scrooby_polygon"
        ),
        "scrooby_multi_text" => matches!(
            child,
            "scrooby_string_text_bible" | "scrooby_string_hardcoded"
        ),
        _ => false,
    }
}

fn is_layout_kind(kind: &str) -> bool {
    matches!(
        kind,
        "scrooby_project"
            | "scrooby_screen"
            | "scrooby_page"
            | "scrooby_layer"
            | "scrooby_group"
            | "scrooby_multi_sprite"
            | "scrooby_multi_text"
            | "scrooby_pure3d_object"
            | "scrooby_polygon"
            | "scrooby_string_text_bible"
            | "scrooby_string_hardcoded"
    )
}

const fn semantic_i32(value: u32) -> i32 {
    i32::from_le_bytes(value.to_le_bytes())
}

const fn packed_rgba_u8(value: u32) -> [u8; 4] {
    let [alpha, red, green, blue] = value.to_be_bytes();
    [red, green, blue, alpha]
}

fn horizontal_justification(value: u32) -> PipelineOutcome<&'static str> {
    match value {
        0 => Ok("left"),
        1 => Ok("right"),
        4 => Ok("centre"),
        _ => Err(PipelineError::new("unsupported horizontal justification")),
    }
}

fn vertical_justification(value: u32) -> PipelineOutcome<&'static str> {
    match value {
        2 => Ok("top"),
        3 => Ok("bottom"),
        4 => Ok("centre"),
        _ => Err(PipelineError::new("unsupported vertical justification")),
    }
}

fn screen_i32(value: f64) -> PipelineOutcome<i32> {
    if !value.is_finite()
        || value < f64::from(i32::MIN)
        || value > f64::from(i32::MAX)
    {
        return Err(PipelineError::new(
            "Scrooby screen coordinate is outside i32",
        ));
    }
    let integral = if value.is_sign_negative() {
        value.ceil()
    } else {
        value.floor()
    };
    integral
        .to_string()
        .parse::<i32>()
        .map_err(|_error| {
            PipelineError::new("Scrooby screen coordinate is not integral i32")
        })
}

fn required_u32_pair(value: &Value, field: &str) -> PipelineOutcome<[u32; 2]> {
    let array = value.get(field).and_then(Value::as_array).ok_or_else(|| {
        PipelineError::new(format!("Scrooby {field} is not an array"))
    })?;
    let [first, second] = array.as_slice() else {
        return Err(PipelineError::new(format!(
            "Scrooby {field} is not a pair"
        )));
    };
    Ok([value_u32(first, field)?, value_u32(second, field)?])
}

fn required_u32(value: &Value, field: &str) -> PipelineOutcome<u32> {
    value.get(field).ok_or_else(|| {
        PipelineError::new(format!("Scrooby {field} is missing"))
    }).and_then(|value| value_u32(value, field))
}

fn value_u32(value: &Value, field: &str) -> PipelineOutcome<u32> {
    value.as_u64().and_then(|value| u32::try_from(value).ok()).ok_or_else(|| {
        PipelineError::new(format!("Scrooby {field} is not u32"))
    })
}

fn required_f64(value: &Value, field: &str) -> PipelineOutcome<f64> {
    value.get(field).ok_or_else(|| {
        PipelineError::new(format!("Scrooby {field} is missing"))
    }).and_then(number_f64)
}

fn number_f64(value: &Value) -> PipelineOutcome<f64> {
    value.as_f64().filter(|value| value.is_finite()).ok_or_else(|| {
        PipelineError::new("Scrooby numeric value is not finite")
    })
}

fn required_string<'value>(
    value: &'value Value,
    field: &str,
) -> PipelineOutcome<&'value str> {
    value.get(field).and_then(Value::as_str).ok_or_else(|| {
        PipelineError::new(format!("Scrooby {field} is not a string"))
    })
}

fn required_usize(value: &Value, field: &str) -> PipelineOutcome<usize> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|value| usize::try_from(value).ok())
        .ok_or_else(|| {
        PipelineError::new(format!("Scrooby {field} is not an integer"))
    })
}

fn resolve_package_root(
    extracted_root: &Path,
    published_root: &str,
) -> PipelineOutcome<PathBuf> {
    let root_name = extracted_root
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
        PipelineError::new("Scrooby layout root has no portable name")
    })?;
    let prefix = format!("{root_name}/");
    let relative = published_root.strip_prefix(&prefix).ok_or_else(|| {
        PipelineError::new("Scrooby layout package is outside extracted root")
    })?;
    resolve_under(extracted_root, Path::new(relative)).map_err(|_error| {
        PipelineError::new("Scrooby layout package escapes extracted root")
    })
}

fn render_catalog(rows: &[Value]) -> PipelineOutcome<String> {
    let mut output = String::new();
    output.push_str(&serde_json::to_string(&json!({
        "schema": SCHEMA,
        "record_type": "header",
        "status": "complete",
        "layout_count": rows.len(),
    }))
    .map_err(|error| {
        PipelineError::new(format!("Scrooby layout JSON failed: {error}"))
    })?);
    output.push('\n');
    for row in rows {
        let mut object = row.as_object().cloned().ok_or_else(|| {
            PipelineError::new("Scrooby layout output row is not an object")
        })?;
        let _previous = object.insert(
            "schema".to_owned(),
            Value::String(SCHEMA.to_owned()),
        );
        let _previous = object.insert(
            "record_type".to_owned(),
            Value::String("layout".to_owned()),
        );
        output.push_str(&serde_json::to_string(&Value::Object(object))
            .map_err(|error| {
            PipelineError::new(format!("Scrooby layout JSON failed: {error}"))
        })?);
        output.push('\n');
    }
    Ok(output)
}

fn publish_rendered(output_root: &Path, rendered: &str) -> PipelineOutcome<()> {
    let name = output_root
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            PipelineError::new("Scrooby layout output has no portable name")
        })?;
    let parent = output_root.parent().unwrap_or_else(|| Path::new("."));
    fs::create_dir_all(parent)
        .map_err(|error| io_error("create Scrooby layout parent", &error))?;
    let staging = parent.join(format!(".{name}.complete-staging"));
    let backup = parent.join(format!(".{name}.complete-backup"));
    ensure_absent(&staging, "Scrooby layout staging")?;
    ensure_absent(&backup, "Scrooby layout backup")?;
    let catalog = output_root.join(FILE);
    if let Ok(metadata) = fs::symlink_metadata(output_root) {
        if !metadata.is_dir() || metadata.file_type().is_symlink() {
            return Err(PipelineError::new(
                "Scrooby layout output is not a regular directory",
            ));
        }
        if fs::read_to_string(&catalog).ok().as_deref() == Some(rendered) {
            return Ok(());
        }
    }

    fs::create_dir_all(&staging)
        .map_err(|error| io_error("create Scrooby layout staging", &error))?;
    let staged_catalog = staging.join(FILE);
    if let Err(error) = fs::write(&staged_catalog, rendered) {
        let _cleanup = fs::remove_dir_all(&staging);
        return Err(io_error("write Scrooby layout catalog", &error));
    }
    if fs::read_to_string(&staged_catalog)
        .map_err(|error| io_error("read staged Scrooby layout", &error))?
        != rendered
    {
        let _cleanup = fs::remove_dir_all(&staging);
        return Err(PipelineError::new(
            "staged Scrooby layout changed during read-back",
        ));
    }

    let had_output = match fs::symlink_metadata(output_root) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            let _cleanup = fs::remove_dir_all(&staging);
            return Err(io_error("inspect Scrooby layout output", &error));
        },
        Ok(metadata) => {
            if !metadata.is_dir() || metadata.file_type().is_symlink() {
                let _cleanup = fs::remove_dir_all(&staging);
                return Err(PipelineError::new(
                    "Scrooby layout output is not a regular directory",
                ));
            }
            if let Err(error) = fs::rename(output_root, &backup) {
                let _cleanup = fs::remove_dir_all(&staging);
                return Err(io_error("back up Scrooby layout output", &error));
            }
            true
        },
    };
    if let Err(error) = fs::rename(&staging, output_root) {
        let publish_error = io_error("publish Scrooby layout catalog", &error);
        let cleanup_error = fs::remove_dir_all(&staging).err();
        let rollback_error = had_output
            .then(|| fs::rename(&backup, output_root).err())
            .flatten();
        return match (rollback_error, cleanup_error) {
            (None, None) => Err(publish_error),
            (rollback, cleanup) => Err(PipelineError::new(format!(
                "{publish_error}; rollback={:?}; cleanup={:?}",
                rollback.map(|value| value.kind()),
                cleanup.map(|value| value.kind()),
            ))),
        };
    }

    let published = fs::read_to_string(output_root.join(FILE))
        .map_err(|error| io_error("read published Scrooby layout", &error))?;
    if published != rendered {
        rollback_layout_catalog(output_root, &backup, had_output)?;
        return Err(PipelineError::new(
            "published Scrooby layout changed during read-back",
        ));
    }
    if had_output {
        fs::remove_dir_all(&backup)
            .map_err(|error| io_error("remove Scrooby layout backup", &error))?;
    }
    Ok(())
}

fn ensure_absent(path: &Path, label: &str) -> PipelineOutcome<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(io_error(
            "inspect Scrooby layout transaction",
            &error,
        )),
        Ok(_metadata) => Err(PipelineError::new(format!(
            "{label} already exists"
        ))),
    }
}

fn rollback_layout_catalog(
    output_root: &Path,
    backup: &Path,
    had_output: bool,
) -> PipelineOutcome<()> {
    fs::remove_dir_all(output_root)
        .map_err(|error| io_error("remove invalid Scrooby layout", &error))?;
    if had_output {
        fs::rename(backup, output_root).map_err(|error| {
            io_error("restore previous Scrooby layout", &error)
        })?;
    }
    Ok(())
}

fn io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed: {error}"))
}

#[cfg(test)]
// jig-ignore-next-line: canonical test module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/ui_scrooby_layout_tests.rs"]
mod tests;
