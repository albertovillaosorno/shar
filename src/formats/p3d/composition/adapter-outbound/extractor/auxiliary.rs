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
//   - P3D auxiliary schema recovery helpers.
// - Must-Not:
//   - Own filesystem publication or command-line composition.
// - Allows:
//   - Exact bounded decoding and JSON recovery for its schema family.
// - Split-When:
//   - Split when one decoder family exceeds the fixed declaration limit.
// - Merge-When:
//   - Merge when another file owns the identical decoder family.
// - Summary:
//   - P3D auxiliary schema recovery helpers.
// - Description:
//   - Implements auxiliary and presentation schema recovery inside the
//     extractor module scope.
// - Usage:
//   - Included by the owning extractor adapter.
// - Defaults:
//   - Unsupported or malformed payloads fail closed through `Option`.
//

//! P3D auxiliary schema recovery helpers.

use super::{
    ChunkRecord, RecoveredComponent, component_name, escape_json,
    raw_component_bytes, read_chunk_header, read_u32, render, render_f32,
    schema, vertex_expression_json,
};

/// Recover billboard quad group json.
pub(super) fn recover_quad_group_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let shader = schema::read_pascal_at(chunk, &mut cursor)?;
    let z_test = read_u32(chunk, cursor)?;
    cursor += 4;
    let z_write = read_u32(chunk, cursor)?;
    cursor += 4;
    let fog = read_u32(chunk, cursor)?;
    cursor += 4;
    let num_quads = read_u32(chunk, cursor)?;
    let quads = billboard_quads_json(
        chunk,
        component.header_size,
        component.total_size,
    )?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"quad_group","#,
            r#""version":{},"#,
            r#""name":"{}","#,
            r#""shader":"{}","#,
            r#""z_test":{},"#,
            r#""z_write":{},"#,
            r#""fog":{},"#,
            r#""num_quads":{},"#,
            r#""quads":[{}]}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&shader),
        z_test,
        z_write,
        fog,
        num_quads,
        quads
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Decode every billboard quad child in one authored group.
pub(super) fn billboard_quads_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> Option<String> {
    const QUAD: u32 = 0x0001_7001;
    let mut quads = Vec::new();
    while cursor + 12 <= end {
        let (id, header_size, total_size) = read_chunk_header(chunk, cursor)?;
        let next = cursor.checked_add(total_size)?;
        if id != QUAD || total_size < header_size || next > end {
            return None;
        }
        quads.push(billboard_quad_json(
            chunk.get(cursor..next)?,
            header_size,
            total_size,
        )?);
        cursor = next;
    }
    (cursor == end).then(|| quads.join(","))
}

/// Core billboard-quad values decoded from one chunk header.
pub(super) struct BillboardQuadFields {
    /// Authored quad identity.
    name: String,
    /// Source chunk schema version.
    version: u32,
    /// Authored billboard orientation mode.
    billboard_mode: String,
    /// Quad translation in source coordinates.
    translation: [f32; 3],
    /// Packed source vertex colour.
    colour: u32,
    /// Four authored UV corners in winding order.
    uvs: [[f32; 2]; 4],
    /// Authored quad width.
    width: f32,
    /// Authored quad height.
    height: f32,
    /// Authored camera-distance parameter.
    distance: f32,
    /// Authored UV offset.
    uv_offset: [f32; 2],
}

/// Optional display and perspective values decoded from child chunks.
pub(super) struct BillboardQuadDisplay {
    /// Authored display rotation quaternion in WXYZ order.
    rotation: [f32; 4],
    /// Authored display cutoff mode.
    cutoff_mode: String,
    /// Authored animated UV offset range.
    uv_offset_range: [f32; 2],
    /// Source-side display range.
    source_range: f32,
    /// Edge-fade display range.
    edge_range: f32,
    /// Whether perspective scaling remains enabled.
    perspective: bool,
}

/// Decode one billboard quad plus optional display and perspective evidence.
pub(super) fn billboard_quad_json(
    quad: &[u8],
    header_size: usize,
    total_size: usize,
) -> Option<String> {
    let fields = billboard_quad_fields(quad, header_size, total_size)?;
    let display = billboard_quad_display(quad, header_size, total_size)?;
    Some(render_billboard_quad_json(&fields, &display))
}

/// Decode the fixed billboard-quad header without reading child chunks.
pub(super) fn billboard_quad_fields(
    quad: &[u8],
    header_size: usize,
    total_size: usize,
) -> Option<BillboardQuadFields> {
    let mut cursor = 12_usize;
    let version = read_u32(quad, cursor)?;
    cursor += 4;
    let name = schema::read_pascal_at(quad, &mut cursor)?;
    let billboard_mode = render::read_fourcc(quad, cursor)?;
    cursor += 4;
    let translation = read_f32_array::<3>(quad, &mut cursor)?;
    let colour = read_u32(quad, cursor)?;
    cursor += 4;
    let uvs = [
        read_f32_array::<2>(quad, &mut cursor)?,
        read_f32_array::<2>(quad, &mut cursor)?,
        read_f32_array::<2>(quad, &mut cursor)?,
        read_f32_array::<2>(quad, &mut cursor)?,
    ];
    let width = schema::read_f32(quad, cursor)?;
    cursor += 4;
    let height = schema::read_f32(quad, cursor)?;
    cursor += 4;
    let distance = schema::read_f32(quad, cursor)?;
    cursor += 4;
    let uv_offset = read_f32_array::<2>(quad, &mut cursor)?;
    if cursor != header_size || total_size != quad.len() {
        return None;
    }
    Some(BillboardQuadFields {
        name,
        version,
        billboard_mode,
        translation,
        colour,
        uvs,
        width,
        height,
        distance,
        uv_offset,
    })
}

/// Decode optional display and perspective child chunks.
pub(super) fn billboard_quad_display(
    quad: &[u8],
    header_size: usize,
    total_size: usize,
) -> Option<BillboardQuadDisplay> {
    const DISPLAY_INFO: u32 = 0x0001_7003;
    const PERSPECTIVE_INFO: u32 = 0x0001_7004;
    let mut display = BillboardQuadDisplay {
        rotation: [0f32, 0., 0., 1.],
        cutoff_mode: String::new(),
        uv_offset_range: [0f32; 2],
        source_range: 0f32,
        edge_range: 0f32,
        perspective: true,
    };
    let mut child = header_size;
    while child + 12 <= total_size {
        let (id, child_header, child_total) = read_chunk_header(quad, child)?;
        let next = child.checked_add(child_total)?;
        if child_total < child_header || next > total_size {
            return None;
        }
        let mut field = child + 12;
        match id {
            DISPLAY_INFO => {
                read_billboard_display_info(
                    quad,
                    child,
                    child_header,
                    &mut field,
                    &mut display,
                )?;
            },
            PERSPECTIVE_INFO => {
                read_billboard_perspective_info(
                    quad,
                    child,
                    child_header,
                    &mut field,
                    &mut display,
                )?;
            },
            _ => return None,
        }
        child = next;
    }
    (child == total_size).then_some(display)
}

/// Decode one display-info child into the accumulated billboard evidence.
pub(super) fn read_billboard_display_info(
    quad: &[u8],
    child: usize,
    child_header: usize,
    field: &mut usize,
    display: &mut BillboardQuadDisplay,
) -> Option<()> {
    let _version = read_u32(quad, *field)?;
    *field += 4;
    display.rotation = read_f32_array::<4>(quad, field)?;
    display.cutoff_mode = render::read_fourcc(quad, *field)?;
    *field += 4;
    display.uv_offset_range = read_f32_array::<2>(quad, field)?;
    display.source_range = schema::read_f32(quad, *field)?;
    *field += 4;
    display.edge_range = schema::read_f32(quad, *field)?;
    *field += 4;
    (*field == child + child_header).then_some(())
}

/// Decode one perspective-info child into the accumulated billboard evidence.
pub(super) fn read_billboard_perspective_info(
    quad: &[u8],
    child: usize,
    child_header: usize,
    field: &mut usize,
    display: &mut BillboardQuadDisplay,
) -> Option<()> {
    let _version = read_u32(quad, *field)?;
    *field += 4;
    display.perspective = read_u32(quad, *field)? != 0;
    *field += 4;
    (*field == child + child_header).then_some(())
}

/// Render one decoded billboard quad to the canonical JSON schema.
pub(super) fn render_billboard_quad_json(
    fields: &BillboardQuadFields,
    display: &BillboardQuadDisplay,
) -> String {
    format!(
        concat!(
            r#"{{"name":"{}","version":{},"billboard_mode":"{}","#,
            r#""translation":{},"colour":{},"uvs":[{},{},{},{}],"#,
            r#""width":{},"height":{},"distance":{},"uv_offset":{},"#,
            r#""rotation_wxyz":{},"cutoff_mode":"{}","#,
            r#""uv_offset_range":{},"source_range":{},"edge_range":{},"#,
            r#""perspective":{}}}"#,
        ),
        escape_json(&fields.name),
        fields.version,
        escape_json(&fields.billboard_mode),
        f32_array_json(&fields.translation),
        fields.colour,
        f32_array_json(&fields.uvs[0]),
        f32_array_json(&fields.uvs[1]),
        f32_array_json(&fields.uvs[2]),
        f32_array_json(&fields.uvs[3]),
        render_f32(fields.width, fields.width.to_string()),
        render_f32(fields.height, fields.height.to_string()),
        render_f32(fields.distance, fields.distance.to_string()),
        f32_array_json(&fields.uv_offset),
        f32_array_json(&display.rotation),
        escape_json(&display.cutoff_mode),
        f32_array_json(&display.uv_offset_range),
        render_f32(display.source_range, display.source_range.to_string()),
        render_f32(display.edge_range, display.edge_range.to_string()),
        display.perspective,
    )
}

/// Read a fixed-width finite float array while advancing one checked cursor.
pub(super) fn read_f32_array<const N: usize>(
    bytes: &[u8],
    cursor: &mut usize,
) -> Option<[f32; N]> {
    let mut output = [0f32; N];
    for value in &mut output {
        *value = schema::read_f32(bytes, *cursor)?;
        if !value.is_finite() {
            return None;
        }
        *cursor = cursor.checked_add(4)?;
    }
    Some(output)
}

/// Render one fixed float array as compact deterministic JSON.
pub(super) fn f32_array_json<const N: usize>(values: &[f32; N]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| render_f32(*value, value.to_string()))
            .collect::<Vec<_>>()
            .join(",")
    )
}

/// Recover texture font json.
pub(super) fn recover_texture_font_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let shader = schema::read_pascal_at(chunk, &mut cursor)?;
    let font_size = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let font_width = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let font_height = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let baseline = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let num_textures = read_u32(chunk, cursor)?;
    let (children, glyph_count, glyph_records) = texture_font_children_json(
        chunk,
        component.header_size,
        component.total_size,
        num_textures,
    )?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"texture_font","version":{},"name":"{}","#,
            r#""shader":"{}","font_size":{},"font_width":{},"#,
            r#""font_height":{},"baseline":{},"num_textures":{},"#,
            r#""glyph_count":{},"glyph_record_stride_bytes":40,"#,
            r#""glyph_records_u32":[{}],"children":[{}]}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&shader),
        font_size,
        font_width,
        font_height,
        baseline,
        num_textures,
        glyph_count,
        glyph_records,
        children
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Decode the contract-declared texture and glyph-list children of one texture
/// font. Glyph records remain ten raw little-endian u32 words because the
/// generated Pure3D schema identifies the fixed-width record type but does not
/// expose authoritative field names for `tlTextureGlyph`.
fn texture_font_children_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
    declared_textures: u32,
) -> Option<(String, u32, String)> {
    const TEXTURE: u32 = 0x0001_9000;
    const TEXTURE_GLYPH_LIST: u32 = 0x0002_2001;
    let mut children = Vec::new();
    let mut texture_count = 0_u32;
    let mut glyph_evidence = None;
    while cursor < end {
        let (id, header_size, total_size) = read_chunk_header(chunk, cursor)?;
        let next = cursor.checked_add(total_size)?;
        if total_size < header_size || next > end {
            return None;
        }
        children.push(format!(
            concat!(
                r#"{{"id_hex":"0x{:08x}","header_size":{},"#,
                r#""total_size":{},"payload_size":{}}}"#,
            ),
            id,
            header_size,
            total_size,
            total_size.checked_sub(header_size)?
        ));
        match id {
            TEXTURE => {
                texture_count = texture_count.checked_add(1)?;
            },
            TEXTURE_GLYPH_LIST if glyph_evidence.is_none() => {
                glyph_evidence = Some(texture_glyph_records_json(
                    chunk.get(cursor..next)?,
                    header_size,
                    total_size,
                )?);
            },
            _ => return None,
        }
        cursor = next;
    }
    if cursor != end || texture_count != declared_textures {
        return None;
    }
    let (glyph_count, glyph_records) = glyph_evidence?;
    Some((children.join(","), glyph_count, glyph_records))
}

/// Decode one `tlTextureGlyphListChunk` without assigning semantics to the ten
/// words in each fixed-width record.
fn texture_glyph_records_json(
    glyph_chunk: &[u8],
    header_size: usize,
    total_size: usize,
) -> Option<(u32, String)> {
    const CHUNK_HEADER_BYTES: usize = 12;
    const GLYPH_COUNT_BYTES: usize = 4;
    const GLYPH_WORDS: usize = 10;
    const WORD_BYTES: usize = 4;
    const GLYPH_RECORD_BYTES: usize = GLYPH_WORDS * WORD_BYTES;
    if header_size != total_size || total_size != glyph_chunk.len() {
        return None;
    }
    let glyph_count = read_u32(glyph_chunk, CHUNK_HEADER_BYTES)?;
    let count = usize::try_from(glyph_count).ok()?;
    let records_bytes = count.checked_mul(GLYPH_RECORD_BYTES)?;
    let expected = CHUNK_HEADER_BYTES
        .checked_add(GLYPH_COUNT_BYTES)?
        .checked_add(records_bytes)?;
    if expected != total_size {
        return None;
    }
    let mut cursor = CHUNK_HEADER_BYTES.checked_add(GLYPH_COUNT_BYTES)?;
    let mut records = Vec::with_capacity(count);
    for _ in 0..count {
        let mut words = Vec::with_capacity(GLYPH_WORDS);
        for _ in 0..GLYPH_WORDS {
            words.push(read_u32(glyph_chunk, cursor)?.to_string());
            cursor = cursor.checked_add(WORD_BYTES)?;
        }
        records.push(format!("[{}]", words.join(",")));
    }
    (cursor == total_size).then(|| (glyph_count, records.join(",")))
}

/// Recover scrooby project json.
pub(super) fn recover_scrooby_project_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let resolution_x = read_u32(chunk, cursor)?;
    cursor += 4;
    let resolution_y = read_u32(chunk, cursor)?;
    cursor += 4;
    let platform = schema::read_pascal_at(chunk, &mut cursor)?;
    let page_path = schema::read_pascal_at(chunk, &mut cursor)?;
    let resource_path = schema::read_pascal_at(chunk, &mut cursor)?;
    let screen_path = schema::read_pascal_at(chunk, &mut cursor)?;
    let children =
        child_chunks_json(chunk, component.header_size, component.total_size);
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"scrooby_project","name":"{}","version":{},"#,
            r#""resolution":[{},{}],"platform":"{}","#,
            r#""page_path":"{}","resource_path":"{}","screen_path":"{}","#,
            r#""children":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        resolution_x,
        resolution_y,
        escape_json(&platform),
        escape_json(&page_path),
        escape_json(&resource_path),
        escape_json(&screen_path),
        children
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover instanced particle system json.
pub(super) fn recover_inst_particle_system_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let particle_type = read_u32(chunk, 12)?;
    let max_instances = read_u32(chunk, 16)?;
    let children =
        child_chunks_json(chunk, component.header_size, component.total_size);
    let kind = component.kind.label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        concat!(
            r#"{{"schema":"inst_particle_system","#,
            r#""particle_type":{},"#,
            r#""max_instances":{},"#,
            r#""children":[{}]}}"#,
        ),
        particle_type, max_instances, children
    );
    Some(render::json_component(
        kind,
        &name,
        name.clone(),
        json,
        "decoded_schema_payload",
    ))
}

/// Recover multi controller json.
pub(super) fn recover_multi_controller_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_rig_decoded_json(
        component,
        source,
        kind_index,
        crate::adapters::driven::decoders::rig::multi_controller_json,
    )
}

/// Recover vertex animation key json.
pub(super) fn recover_vertex_anim_key_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_rig_decoded_json(
        component,
        source,
        kind_index,
        crate::adapters::driven::decoders::rig::vertex_key_json,
    )
}

/// Recover rig-family decoded json.
pub(super) fn recover_rig_decoded_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
    decoder: fn(&[u8]) -> Option<String>,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let kind = component.kind.label();
    let name = component_name(component, source, kind_index);
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = decoder(chunk)?;
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover history json.
pub(super) fn recover_history_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let num_lines = read_u16(chunk, cursor)? as usize;
    cursor += 2;
    let mut lines = Vec::new();
    for _ in 0..num_lines {
        lines.push(format!(
            "\"{}\"",
            escape_json(&schema::read_pascal_at(chunk, &mut cursor)?)
        ));
    }
    let kind = component.kind.label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        r#"{{"schema":"history","num_lines":{},"history":[{}]}}"#,
        num_lines,
        lines.join(",")
    );
    Some(render::json_component(
        kind,
        &name,
        name.clone(),
        json,
        "decoded_schema_payload",
    ))
}

/// Recover locator json.
pub(super) fn recover_locator_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let position = schema::read_point(chunk, &mut cursor)?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"locator","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""position":[{},{},{}]}}"#,
        ),
        escape_json(&name),
        version,
        position[0],
        position[1],
        position[2]
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover ped path json.
pub(super) fn recover_ped_path_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let count = read_u32(chunk, cursor)? as usize;
    cursor += 4;
    let mut points = Vec::new();
    for _ in 0..count {
        let p = schema::read_point(chunk, &mut cursor)?;
        points.push(format!("[{},{},{}]", p[0], p[1], p[2]));
    }
    let kind = component.kind.label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        r#"{{"schema":"ped_path","num_points":{},"points":[{}]}}"#,
        count,
        points.join(",")
    );
    Some(render::json_component(
        kind,
        &name,
        name.clone(),
        json,
        "decoded_schema_payload",
    ))
}

/// Recover follow cam json.
pub(super) fn recover_follow_cam_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let id = read_u32(chunk, cursor)?;
    cursor += 4;
    let rotation = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let elevation = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let magnitude = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let target_offset = schema::read_point(chunk, &mut cursor)?;
    let kind = component.kind.label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        concat!(
            r#"{{"schema":"follow_cam","#,
            r#""id":{},"#,
            r#""rotation":{},"#,
            r#""elevation":{},"#,
            r#""magnitude":{},"#,
            r#""target_offset":[{},{},{}]}}"#,
        ),
        id,
        rotation,
        elevation,
        magnitude,
        target_offset[0],
        target_offset[1],
        target_offset[2]
    );
    Some(render::json_component(
        kind,
        &name,
        name.clone(),
        json,
        "decoded_schema_payload",
    ))
}

/// Recover export info json.
pub(super) fn recover_export_info_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let children =
        child_chunks_json(chunk, component.header_size, component.total_size);
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        r#"{{"schema":"export_info","name":"{}","entries":[{}]}}"#,
        escape_json(&name),
        children
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover breakable object json.
pub(super) fn recover_breakable_object_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let breakable_type = read_u32(chunk, 12)?;
    let max_instances = read_u32(chunk, 16)?;
    let children =
        child_chunks_json(chunk, component.header_size, component.total_size);
    let kind = component.kind.label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        concat!(
            r#"{{"schema":"breakable_object","#,
            r#""breakable_type":{},"#,
            r#""max_instances":{},"#,
            r#""children":[{}]}}"#,
        ),
        breakable_type, max_instances, children
    );
    Some(render::json_component(
        kind,
        &name,
        name.clone(),
        json,
        "decoded_schema_payload",
    ))
}

/// Recover lens flare json.
pub(super) fn recover_lens_flare_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let num_billboard_quads = read_u32(chunk, cursor)?;
    let children =
        child_chunks_json(chunk, component.header_size, component.total_size);
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"lens_flare_dsg","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""num_billboard_quads":{},"#,
            r#""children":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        num_billboard_quads,
        children
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover attribute table json.
pub(super) fn recover_attribute_table_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let num_rows = read_u32(chunk, cursor)? as usize;
    cursor += 4;
    let mut rows = Vec::new();
    for _ in 0..num_rows {
        let sound = schema::read_pascal_at(chunk, &mut cursor)?;
        let particle = schema::read_pascal_at(chunk, &mut cursor)?;
        let animation = schema::read_pascal_at(chunk, &mut cursor)?;
        let friction = schema::read_f32(chunk, cursor)?;
        cursor += 4;
        let mass = schema::read_f32(chunk, cursor)?;
        cursor += 4;
        let elasticity = schema::read_f32(chunk, cursor)?;
        cursor += 4;
        rows.push(format!(
            concat!(
                r#"{{"sound":"{}","#,
                r#""particle":"{}","#,
                r#""animation":"{}","#,
                r#""friction":{},"#,
                r#""mass":{},"#,
                r#""elasticity":{}}}"#,
            ),
            escape_json(&sound),
            escape_json(&particle),
            escape_json(&animation),
            friction,
            mass,
            elasticity,
        ));
    }
    let kind = component.kind.label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        r#"{{"schema":"attribute_table","num_rows":{},"rows":[{}]}}"#,
        num_rows,
        rows.join(",")
    );
    Some(render::json_component(
        kind,
        &name,
        name.clone(),
        json,
        "decoded_schema_payload",
    ))
}

/// Recover animated object json.
pub(super) fn recover_animated_object_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let factory_name = schema::read_pascal_at(chunk, &mut cursor)?;
    let starting_animation = read_u32(chunk, cursor)?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"animated_object","#,
            r#""version":{},"#,
            r#""name":"{}","#,
            r#""factory_name":"{}","#,
            r#""starting_animation":{}}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&factory_name),
        starting_animation
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover animated object factory json.
pub(super) fn recover_animated_object_factory_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let base_object_name = schema::read_pascal_at(chunk, &mut cursor)?;
    let num_animations = read_u32(chunk, cursor)?;
    let children =
        child_chunks_json(chunk, component.header_size, component.total_size);
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"animated_object_factory","#,
            r#""version":{},"#,
            r#""name":"{}","#,
            r#""base_object_name":"{}","#,
            r#""num_animations":{},"#,
            r#""children":[{}]}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&base_object_name),
        num_animations,
        children
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover state prop json.
pub(super) fn recover_state_prop_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let object_factory_name = schema::read_pascal_at(chunk, &mut cursor)?;
    let num_states = read_u32(chunk, cursor)?;
    let children =
        child_chunks_json(chunk, component.header_size, component.total_size);
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"state_prop","#,
            r#""version":{},"#,
            r#""name":"{}","#,
            r#""object_factory_name":"{}","#,
            r#""num_states":{},"#,
            r#""children":[{}]}}"#,
        ),
        version,
        escape_json(&name),
        escape_json(&object_factory_name),
        num_states,
        children
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover vertex expression json.
pub(super) fn recover_vertex_expression_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let kind = component.kind.label();
    let name = component_name(component, source, kind_index);
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = vertex_expression_json(kind, chunk)?;
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Child chunks json.
pub(super) fn child_chunks_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> String {
    let mut children = Vec::new();
    while cursor + 12 <= end {
        let Some((id, header_size, total_size)) =
            read_chunk_header(chunk, cursor)
        else {
            break;
        };
        let next = cursor.saturating_add(total_size);
        if total_size < header_size || next > end {
            break;
        }
        children.push(format!(
            concat!(
                r#"{{"id_hex":"0x{:08x}","#,
                r#""header_size":{},"#,
                r#""total_size":{},"#,
                r#""payload_size":{}}}"#,
            ),
            id,
            header_size,
            total_size,
            total_size.saturating_sub(header_size)
        ));
        cursor = next;
    }
    children.join(",")
}

/// Read u16.
pub(super) fn read_u16(bytes: &[u8], offset: usize) -> Option<u16> {
    let end = offset.checked_add(2)?;
    let slice = bytes.get(offset..end)?;
    Some(u16::from_le_bytes([slice[0], slice[1]]))
}
