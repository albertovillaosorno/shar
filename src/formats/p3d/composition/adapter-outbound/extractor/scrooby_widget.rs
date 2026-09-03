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
//   - Typed recovery of observed Scrooby layer-widget chunks.
// - Must-Not:
//   - Invent runtime behavior, layout policy, or resource resolution.
// - Allows:
//   - Exact contract-backed scalar, string, point, colour, and child recovery.
// - Split-When:
//   - One widget family gains an independent semantic compiler.
// - Merge-When:
//   - Another extractor module owns the identical Scrooby widget boundary.
// - Summary:
//   - Scrooby layer-widget schema recovery.
// - Description:
//   - Preserves the observed authored UI widget fields without mapping them to
//     an Unreal widget model.
// - Usage:
//   - Invoked by the extractor schema dispatcher for typed Scrooby children.
// - Defaults:
//   - Malformed bounds, non-finite floats, or trailing header bytes fail
//     closed.
//

//! Scrooby layer-widget schema recovery.

use super::auxiliary::scrooby_container_children_json;
use super::{
    ChunkRecord, RecoveredComponent, escape_json, raw_component_bytes,
    read_u32, render, render_f32, schema,
};

struct WidgetFrame {
    name: String,
    version: u32,
    position: [u32; 2],
    dimensions: [u32; 2],
    justification: [u32; 2],
    color: u32,
    translucency: u32,
    rotation: f32,
}

pub(super) fn recover_group_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_word(chunk, &mut cursor)?;
    let alpha = read_word(chunk, &mut cursor)?;
    if cursor != component.header_size {
        return None;
    }
    let children = scrooby_container_children_json(
        chunk,
        component.header_size,
        component.total_size,
        &[
            0x0001_8004,
            0x0001_8006,
            0x0001_8007,
            0x0001_8008,
            0x0001_8009,
        ],
    )?;
    Some(render_named(
        component,
        kind_index,
        &name,
        format!(
            concat!(
                r#"{{"schema":"scrooby_group","name":"{}","#,
                r#""version":{},"alpha":{},"children":[{}]}}"#,
            ),
            escape_json(&name),
            version,
            alpha,
            children,
        ),
    ))
}

pub(super) fn recover_multi_sprite_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let frame = read_widget_frame(chunk, &mut cursor)?;
    let image_count = usize::try_from(read_word(chunk, &mut cursor)?).ok()?;
    let mut images = Vec::with_capacity(image_count);
    for _ in 0..image_count {
        images.push(schema::read_pascal_at(chunk, &mut cursor)?);
    }
    if cursor != component.header_size
        || component.header_size != component.total_size
    {
        return None;
    }
    let image_names = string_array_json(&images);
    Some(render_frame(
        component,
        kind_index,
        &frame,
        "scrooby_multi_sprite",
        &format!(
            r#","image_count":{image_count},"image_names":[{image_names}]"#,
        ),
        "",
    ))
}

pub(super) fn recover_multi_text_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let frame = read_widget_frame(chunk, &mut cursor)?;
    if frame.version <= 16 {
        return None;
    }
    let text_style = schema::read_pascal_at(chunk, &mut cursor)?;
    let shadow_enabled = *chunk.get(cursor)?;
    cursor = cursor.checked_add(1)?;
    let shadow_color = read_word(chunk, &mut cursor)?;
    let shadow_offset_x = read_word(chunk, &mut cursor)?;
    let shadow_offset_y = read_word(chunk, &mut cursor)?;
    let current_text = read_word(chunk, &mut cursor)?;
    if cursor != component.header_size {
        return None;
    }
    let children = scrooby_container_children_json(
        chunk,
        component.header_size,
        component.total_size,
        &[0x0001_800b, 0x0001_800c],
    )?;
    Some(render_frame(
        component,
        kind_index,
        &frame,
        "scrooby_multi_text",
        &format!(
            concat!(
                r#","text_style":"{}","shadow_enabled":{},"#,
                r#""shadow_color":{},"shadow_offset":[{},{}],"#,
                r#""current_text":{}"#,
            ),
            escape_json(&text_style),
            shadow_enabled,
            shadow_color,
            shadow_offset_x,
            shadow_offset_y,
            current_text,
        ),
        &format!(r#","children":[{children}]"#),
    ))
}

pub(super) fn recover_pure3d_object_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let frame = read_widget_frame(chunk, &mut cursor)?;
    let filename = schema::read_pascal_at(chunk, &mut cursor)?;
    if cursor != component.header_size
        || component.header_size != component.total_size
    {
        return None;
    }
    Some(render_frame(
        component,
        kind_index,
        &frame,
        "scrooby_pure3d_object",
        &format!(r#","filename":"{}""#, escape_json(&filename)),
        "",
    ))
}

pub(super) fn recover_polygon_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_word(chunk, &mut cursor)?;
    let translucency = read_word(chunk, &mut cursor)?;
    let point_count = usize::try_from(read_word(chunk, &mut cursor)?).ok()?;
    let mut points = Vec::with_capacity(point_count);
    for _ in 0..point_count {
        points.push(schema::read_point(chunk, &mut cursor)?);
    }
    let mut colors = Vec::with_capacity(point_count);
    for _ in 0..point_count {
        colors.push(read_word(chunk, &mut cursor)?);
    }
    if cursor != component.header_size
        || component.header_size != component.total_size
    {
        return None;
    }
    let points = points.iter().map(float3_json).collect::<Vec<_>>().join(",");
    let colors = colors
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",");
    Some(render_named(
        component,
        kind_index,
        &name,
        format!(
            concat!(
                r#"{{"schema":"scrooby_polygon","name":"{}","#,
                r#""version":{},"translucency":{},"point_count":{},"#,
                r#""points":[{}],"colors":[{}]}}"#,
            ),
            escape_json(&name),
            version,
            translucency,
            point_count,
            points,
            colors,
        ),
    ))
}

pub(super) fn recover_string_text_bible_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let bible_name = schema::read_pascal_at(chunk, &mut cursor)?;
    let string_id = schema::read_pascal_at(chunk, &mut cursor)?;
    if cursor != component.header_size
        || component.header_size != component.total_size
    {
        return None;
    }
    let name = format!("{bible_name}:{string_id}");
    Some(render_named(
        component,
        kind_index,
        &name,
        format!(
            concat!(
                r#"{{"schema":"scrooby_string_text_bible","#,
                r#""bible_name":"{}","string_id":"{}"}}"#,
            ),
            escape_json(&bible_name),
            escape_json(&string_id),
        ),
    ))
}

pub(super) fn recover_string_hardcoded_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let value = schema::read_pascal_at(chunk, &mut cursor)?;
    if cursor != component.header_size
        || component.header_size != component.total_size
    {
        return None;
    }
    Some(render_named(
        component,
        kind_index,
        &value,
        format!(
            r#"{{"schema":"scrooby_string_hardcoded","value":"{}"}}"#,
            escape_json(&value),
        ),
    ))
}

fn read_widget_frame(bytes: &[u8], cursor: &mut usize) -> Option<WidgetFrame> {
    let name = schema::read_pascal_at(bytes, cursor)?;
    let version = read_word(bytes, cursor)?;
    let position = [read_word(bytes, cursor)?, read_word(bytes, cursor)?];
    let dimensions = [read_word(bytes, cursor)?, read_word(bytes, cursor)?];
    let justification = [read_word(bytes, cursor)?, read_word(bytes, cursor)?];
    let color = read_word(bytes, cursor)?;
    let translucency = read_word(bytes, cursor)?;
    let rotation = schema::read_f32(bytes, *cursor)?;
    if !rotation.is_finite() {
        return None;
    }
    *cursor = cursor.checked_add(4)?;
    Some(WidgetFrame {
        name,
        version,
        position,
        dimensions,
        justification,
        color,
        translucency,
        rotation,
    })
}

fn read_word(bytes: &[u8], cursor: &mut usize) -> Option<u32> {
    let value = read_u32(bytes, *cursor)?;
    *cursor = cursor.checked_add(4)?;
    Some(value)
}

fn render_frame(
    component: &ChunkRecord,
    kind_index: usize,
    frame: &WidgetFrame,
    schema_name: &str,
    fields: &str,
    children: &str,
) -> RecoveredComponent {
    let mut json = format!(
        concat!(
            r#"{{"schema":"{}","name":"{}","version":{},"#,
            r#""position":[{},{}],"dimensions":[{},{}],"#,
            r#""justification":[{},{}],"color":{},"translucency":{},"#,
            r#""rotation":{}{}{}"#,
        ),
        schema_name,
        escape_json(&frame.name),
        frame.version,
        frame.position[0],
        frame.position[1],
        frame.dimensions[0],
        frame.dimensions[1],
        frame.justification[0],
        frame.justification[1],
        frame.color,
        frame.translucency,
        render_f32(frame.rotation, frame.rotation.to_string()),
        fields,
        children,
    );
    json.push('}');
    render_named(component, kind_index, &frame.name, json)
}

fn render_named(
    component: &ChunkRecord,
    kind_index: usize,
    name: &str,
    json: String,
) -> RecoveredComponent {
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, name);
    render::json_component(
        kind,
        &file_name,
        name.to_owned(),
        json,
        "decoded_schema_payload",
    )
}

fn string_array_json(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!("\"{}\"", escape_json(value)))
        .collect::<Vec<_>>()
        .join(",")
}

fn float3_json(values: &[f32; 3]) -> String {
    format!(
        "[{},{},{}]",
        render_f32(values[0], values[0].to_string()),
        render_f32(values[1], values[1].to_string()),
        render_f32(values[2], values[2].to_string()),
    )
}
