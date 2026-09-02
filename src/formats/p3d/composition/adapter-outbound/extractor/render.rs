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
//   - P3D render schema recovery helpers.
// - Must-Not:
//   - Own filesystem publication or command-line composition.
// - Allows:
//   - Exact bounded decoding and JSON recovery for its schema family.
// - Split-When:
//   - Split when one decoder family exceeds the fixed declaration limit.
// - Merge-When:
//   - Merge when another file owns the identical decoder family.
// - Summary:
//   - P3D render schema recovery helpers.
// - Description:
//   - Implements render and parameter schema recovery inside the extractor
//     module scope.
// - Usage:
//   - Included by the owning extractor adapter.
// - Defaults:
//   - Unsupported or malformed payloads fail closed through `Option`.
//

//! P3D render schema recovery helpers.

use super::{
    ChunkRecord, PathBuf, RecoveredComponent, auxiliary, component_name,
    escape_json, raw_component_bytes, read_chunk_header, read_pascal_name,
    read_u32, render_f32, sanitize, schema,
};

/// Wrap decoded schema JSON in package component metadata.
pub(super) fn schema_component(
    component: &ChunkRecord,
    kind_index: usize,
    name: String,
    json: String,
) -> RecoveredComponent {
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    json_component(kind, &file_name, name, json, "decoded_schema_payload")
}

/// Json component.
pub(super) fn json_component(
    kind: &str,
    file_name: &str,
    name: String,
    json: String,
    recovery_status: &str,
) -> RecoveredComponent {
    RecoveredComponent {
        relative_path: PathBuf::from(kind)
            .join(format!("{}.json", sanitize(file_name))),
        name,
        bytes: json.into_bytes(),
        payload_format: "schema_json".to_owned(),
        recovery_status: recovery_status.to_owned(),
    }
}

/// Recover mesh json.
pub(super) fn recover_mesh_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
    chunks: Option<&[ChunkRecord]>,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let name = read_pascal_name(component, source).unwrap_or_else(|| {
        format!("{}_{kind_index:04}", component.kind.label())
    });
    let ordinals = primitive_group_source_ordinals(component, chunks);
    let json = crate::adapters::driven::decoders::mesh::
        mesh_json_with_source_ordinals(chunk, ordinals.as_deref())?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover skin json.
pub(super) fn recover_skin_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
    chunks: Option<&[ChunkRecord]>,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let name = read_pascal_name(component, source).unwrap_or_else(|| {
        format!("{}_{kind_index:04}", component.kind.label())
    });
    let ordinals = primitive_group_source_ordinals(component, chunks);
    let json = crate::adapters::driven::decoders::mesh::
        skin_json_with_source_ordinals(chunk, ordinals.as_deref())?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Return direct primitive-group source ordinals when package context exists.
fn primitive_group_source_ordinals(
    component: &ChunkRecord,
    chunks: Option<&[ChunkRecord]>,
) -> Option<Vec<usize>> {
    let chunks = chunks?;
    let ordinals = chunks
        .iter()
        .filter(|child| child.parent_ordinal == Some(component.ordinal))
        .filter(|child| child.id == 0x0001_0002)
        .map(|child| child.ordinal)
        .collect::<Vec<_>>();
    Some(ordinals)
}

/// Recover skeleton json.
pub(super) fn recover_skeleton_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    auxiliary::recover_rig_decoded_json(
        component,
        source,
        kind_index,
        crate::adapters::driven::decoders::rig::skeleton_json,
    )
}

/// Recover camera json.
pub(super) fn recover_camera_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let fov = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let aspect_ratio = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let near_clip = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let far_clip = schema::read_f32(chunk, cursor)?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        "{{\"schema\":\"camera\",\"name\":\"{}\",\"version\":{},\"fov\":{},\"\
         aspect_ratio\":{},\"near_clip\":{},\"far_clip\":{}}}\n",
        escape_json(&name),
        version,
        fov,
        aspect_ratio,
        near_clip,
        far_clip
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover composite json.
pub(super) fn recover_composite_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let name = read_pascal_name(component, source).unwrap_or_else(|| {
        schema::fallback_component_name(component, kind_index)
    });
    let json =
        crate::adapters::driven::decoders::scene::composite_drawable_json(
            chunk,
        )?;
    Some(schema_component(component, kind_index, name, json))
}

/// Recover scenegraph json.
pub(super) fn recover_scenegraph_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let name = read_pascal_name(component, source).unwrap_or_else(|| {
        schema::fallback_component_name(component, kind_index)
    });
    let json =
        crate::adapters::driven::decoders::scene::scenegraph_json(chunk)?;
    Some(schema_component(component, kind_index, name, json))
}

/// Recover entity DSG json.
pub(super) fn recover_entity_dsg_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let name = read_pascal_name(component, source).unwrap_or_else(|| {
        schema::fallback_component_name(component, kind_index)
    });
    let json =
        crate::adapters::driven::decoders::scene::entity_dsg_json(chunk)?;
    Some(schema_component(component, kind_index, name, json))
}

/// Recover insta entity DSG json.
pub(super) fn recover_insta_entity_dsg_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let name = read_pascal_name(component, source).unwrap_or_else(|| {
        schema::fallback_component_name(component, kind_index)
    });
    let json =
        crate::adapters::driven::decoders::scene::insta_entity_dsg_json(chunk)?;
    Some(schema_component(component, kind_index, name, json))
}

/// Recover animation json.
pub(super) fn recover_animation_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    auxiliary::recover_rig_decoded_json(
        component,
        source,
        kind_index,
        crate::adapters::driven::decoders::rig::animation_json,
    )
}

/// Recover particle system json.
pub(super) fn recover_particle_system_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let factory = schema::read_pascal_at(chunk, &mut cursor)?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        "{{\"schema\":\"particle_system\",\"name\":\"{}\",\"version\":{},\"\
         factory_name\":\"{}\"}}\n",
        escape_json(&name),
        version,
        escape_json(&factory)
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover particle factory json.
pub(super) fn recover_particle_factory_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let frame_rate = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let anim_frames = read_u32(chunk, cursor)?;
    cursor += 4;
    let ol_frames = read_u32(chunk, cursor)?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        "{{\"schema\":\"particle_system_factory\",\"name\":\"{}\",\"version\":\
         {},\"frame_rate\":{},\"num_anim_frames\":{},\"num_ol_frames\":{}}}\n",
        escape_json(&name),
        version,
        frame_rate,
        anim_frames,
        ol_frames
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover light group json.
pub(super) fn recover_light_group_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let num_lights = read_u32(chunk, cursor)?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        "{{\"schema\":\"light_group\",\"name\":\"{}\",\"num_lights\":{}}}\n",
        escape_json(&name),
        num_lights
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover world sphere json.
pub(super) fn recover_world_sphere_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let num_meshes = read_u32(chunk, cursor)?;
    cursor += 4;
    let num_billboards = read_u32(chunk, cursor)?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        "{{\"schema\":\"world_sphere_dsg\",\"name\":\"{}\",\"version\":{},\"\
         num_meshes\":{},\"num_billboard_quads\":{}}}\n",
        escape_json(&name),
        version,
        num_meshes,
        num_billboards
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Read fourcc.
pub(super) fn read_fourcc(bytes: &[u8], offset: usize) -> Option<String> {
    let end = offset.checked_add(4)?;
    let slice = bytes.get(offset..end)?;
    Some(
        std::str::from_utf8(slice)
            .ok()?
            .trim_matches(char::from(0))
            .to_owned(),
    )
}

/// Recover text bible json.
pub(super) fn recover_text_bible_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let name = component_name(component, source, kind_index);
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let children = auxiliary::child_chunks_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"text_bible","#,
            r#""name":"{}","#,
            r#""payload_size":{},"#,
            r#""child_count":{},"#,
            r#""language_chunks":[{}]}}"#,
        ),
        escape_json(&name),
        component.payload_size,
        component.child_count,
        children
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover srr locator json.
pub(super) fn recover_srr_locator_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let locator_type = read_u32(chunk, cursor)?;
    cursor += 4;
    let num_data = read_u32(chunk, cursor)? as usize;
    cursor += 4;
    let mut data = Vec::with_capacity(num_data);
    for _ in 0..num_data {
        data.push(read_u32(chunk, cursor)?);
        cursor += 4;
    }
    let position = schema::read_point(chunk, &mut cursor)?;
    let num_triggers = read_u32(chunk, cursor)?;
    let locator_type_name =
        crate::adapters::driven::decoders::locator::type_name(locator_type)?;
    let data_interpretation =
        crate::adapters::driven::decoders::locator::data_interpretation_json(
            locator_type,
            &data,
            num_triggers,
        )?;
    let triggers = trigger_volumes_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        "{{\"schema\":\"locator\",\"name\":\"{}\",\"locator_type\":{},\"\
         locator_type_name\":\"{}\",\"position\":[{},{},{}],\"\
         num_data_elements\":{},\"data_elements_u32\":[{}],\"\
         data_elements_f32\":[{}],\"data_ascii_lossy\":\"{}\",\"\
         data_interpretation\":{},\"num_triggers\":{},\"trigger_volumes\":\
         [{}]}}\n",
        escape_json(&name),
        locator_type,
        locator_type_name,
        position[0],
        position[1],
        position[2],
        num_data,
        u32_list_json(&data),
        f32_list_json(&data),
        escape_json(&ascii_from_u32_words(&data)),
        data_interpretation,
        num_triggers,
        triggers
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// U32 list json.
pub(super) fn u32_list_json(values: &[u32]) -> String {
    values
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// F32 list json.
pub(super) fn f32_list_json(values: &[u32]) -> String {
    values
        .iter()
        .map(|value| {
            let decoded = f32::from_bits(*value);
            render_f32(decoded, decoded.to_string())
        })
        .collect::<Vec<_>>()
        .join(",")
}

/// Ascii from u32 words.
pub(super) fn ascii_from_u32_words(values: &[u32]) -> String {
    let mut bytes = Vec::with_capacity(values.len() * 4);
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    String::from_utf8_lossy(&bytes)
        .trim_matches(char::from(0))
        .to_owned()
}

/// Trigger volumes json.
pub(super) fn trigger_volumes_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> String {
    let mut triggers = Vec::new();
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
        if id == 0x0300_0006
            && let Some(trigger) =
                trigger_volume_json(chunk, cursor, header_size)
        {
            triggers.push(trigger);
        }
        cursor = next;
    }
    triggers.join(",")
}

/// Trigger volume json.
pub(super) fn trigger_volume_json(
    chunk: &[u8],
    offset: usize,
    header_size: usize,
) -> Option<String> {
    let mut cursor = offset + 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let volume_type = read_u32(chunk, cursor)?;
    cursor += 4;
    let scale = [
        schema::read_f32(chunk, cursor)?,
        schema::read_f32(chunk, cursor + 4)?,
        schema::read_f32(chunk, cursor + 8)?,
    ];
    cursor += 12;
    let mut matrix = [[0f32; 4]; 4];
    for row in &mut matrix {
        for value in row {
            *value = schema::read_f32(chunk, cursor)?;
            cursor += 4;
        }
    }
    if cursor > offset + header_size {
        return None;
    }
    let position = [matrix[3][0], matrix[3][1], matrix[3][2]];
    Some(format!(
        "{{\"name\":\"{}\",\"type\":{},\"type_name\":\"{}\",\"scale\":[{},\
             {},{}],\"position\":[{},{},{}],\"matrix\":{}}}",
        escape_json(&name),
        volume_type,
        trigger_volume_type_name(volume_type),
        scale[0],
        scale[1],
        scale[2],
        position[0],
        position[1],
        position[2],
        matrix_json(&matrix)
    ))
}

/// Trigger volume type name.
pub(super) fn trigger_volume_type_name(volume_type: u32) -> &'static str {
    match volume_type {
        0 => "sphere",
        1 => "rectangle",
        _ => "unknown_trigger_volume_type",
    }
}

/// Matrix json.
pub(super) fn matrix_json(matrix: &[[f32; 4]; 4]) -> String {
    format!(
        "[[{},{},{},{}],[{},{},{},{}],[{},{},{},{}],[{},{},{},{}]]",
        matrix[0][0],
        matrix[0][1],
        matrix[0][2],
        matrix[0][3],
        matrix[1][0],
        matrix[1][1],
        matrix[1][2],
        matrix[1][3],
        matrix[2][0],
        matrix[2][1],
        matrix[2][2],
        matrix[2][3],
        matrix[3][0],
        matrix[3][1],
        matrix[3][2],
        matrix[3][3]
    )
}

/// Shader params json.
pub(super) fn shader_params_json(
    chunk: &[u8],
    cursor: usize,
    end: usize,
) -> String {
    param_chunks_json(chunk, cursor, end, true)
}

/// Game attr params json.
pub(super) fn game_attr_params_json(
    chunk: &[u8],
    cursor: usize,
    end: usize,
) -> String {
    param_chunks_json(chunk, cursor, end, false)
}

/// Param chunks json.
pub(super) fn param_chunks_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
    shader_params: bool,
) -> String {
    let mut params = Vec::new();
    while cursor + 12 <= end {
        let header = read_chunk_header(chunk, cursor);
        let Some((id, header_size, total_size)) = header else {
            break;
        };
        let next = cursor.saturating_add(total_size);
        if total_size < header_size || next > end {
            break;
        }
        let parsed = if shader_params {
            shader_param_json(chunk, cursor, id)
        } else {
            game_attr_param_json(chunk, cursor, id)
        };
        if let Some(value) = parsed {
            params.push(value);
        }
        cursor = next;
    }
    params.join(",")
}

/// Shader param json.
pub(super) fn shader_param_json(
    chunk: &[u8],
    offset: usize,
    id: u32,
) -> Option<String> {
    let mut cursor = offset + 12;
    match id {
        0x0001_1002 => {
            let param = read_fourcc(chunk, cursor)?;
            cursor += 4;
            let value = schema::read_pascal_at(chunk, &mut cursor)?;
            Some(format!(
                r#"{{"kind":"texture","param":"{}","value":"{}"}}"#,
                escape_json(&param),
                escape_json(&value)
            ))
        },
        0x0001_1003 => shader_number_param(chunk, cursor, "int"),
        0x0001_1004 => shader_float_param(chunk, cursor),
        0x0001_1005 => shader_colour_param(chunk, cursor),
        0x0001_1006 => shader_vector_param(chunk, cursor),
        0x0001_1007 => shader_matrix_param(chunk, cursor),
        _ => None,
    }
}

/// Game attr param json.
pub(super) fn game_attr_param_json(
    chunk: &[u8],
    offset: usize,
    id: u32,
) -> Option<String> {
    let mut cursor = offset + 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    match id {
        0x0001_2001 => {
            let value = read_u32(chunk, cursor)?;
            Some(format!(
                r#"{{"kind":"int","name":"{}","value":{}}}"#,
                escape_json(&name),
                value
            ))
        },
        0x0001_2002 => {
            let value = schema::read_f32(chunk, cursor)?;
            Some(format!(
                r#"{{"kind":"float","name":"{}","value":{}}}"#,
                escape_json(&name),
                value
            ))
        },
        0x0001_2003 => {
            let value = read_u32(chunk, cursor)?;
            Some(format!(
                r#"{{"kind":"colour","name":"{}","value":{}}}"#,
                escape_json(&name),
                value
            ))
        },
        0x0001_2004 => game_attr_vector_param(chunk, cursor, &name),
        0x0001_2005 => game_attr_matrix_param(chunk, cursor, &name),
        _ => None,
    }
}

/// Shader number param.
pub(super) fn shader_number_param(
    chunk: &[u8],
    cursor: usize,
    kind: &str,
) -> Option<String> {
    let param = read_fourcc(chunk, cursor)?;
    let value = read_u32(chunk, cursor + 4)?;
    Some(format!(
        r#"{{"kind":"{}","param":"{}","value":{}}}"#,
        kind,
        escape_json(&param),
        value
    ))
}

/// Shader float param.
pub(super) fn shader_float_param(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let param = read_fourcc(chunk, cursor)?;
    let value = schema::read_f32(chunk, cursor + 4)?;
    Some(format!(
        r#"{{"kind":"float","param":"{}","value":{}}}"#,
        escape_json(&param),
        value
    ))
}

/// Shader colour param.
pub(super) fn shader_colour_param(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let param = read_fourcc(chunk, cursor)?;
    let value = read_u32(chunk, cursor + 4)?;
    Some(format!(
        r#"{{"kind":"colour","param":"{}","value":{}}}"#,
        escape_json(&param),
        value
    ))
}

/// Shader vector param.
pub(super) fn shader_vector_param(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let param = read_fourcc(chunk, cursor)?;
    let x = schema::read_f32(chunk, cursor + 4)?;
    let y = schema::read_f32(chunk, cursor + 8)?;
    let z = schema::read_f32(chunk, cursor + 12)?;
    Some(format!(
        r#"{{"kind":"vector","param":"{}","value":[{},{},{}]}}"#,
        escape_json(&param),
        x,
        y,
        z
    ))
}

/// Shader matrix param.
pub(super) fn shader_matrix_param(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let param = read_fourcc(chunk, cursor)?;
    let matrix = matrix_values_json(chunk, cursor + 4)?;
    Some(format!(
        r#"{{"kind":"matrix","param":"{}","value":{}}}"#,
        escape_json(&param),
        matrix
    ))
}

/// Game attr vector param.
pub(super) fn game_attr_vector_param(
    chunk: &[u8],
    cursor: usize,
    name: &str,
) -> Option<String> {
    let x = schema::read_f32(chunk, cursor)?;
    let y = schema::read_f32(chunk, cursor + 4)?;
    let z = schema::read_f32(chunk, cursor + 8)?;
    Some(format!(
        r#"{{"kind":"vector","name":"{}","value":[{},{},{}]}}"#,
        escape_json(name),
        x,
        y,
        z
    ))
}

/// Game attr matrix param.
pub(super) fn game_attr_matrix_param(
    chunk: &[u8],
    cursor: usize,
    name: &str,
) -> Option<String> {
    let matrix = matrix_values_json(chunk, cursor)?;
    Some(format!(
        r#"{{"kind":"matrix","name":"{}","value":{}}}"#,
        escape_json(name),
        matrix
    ))
}

/// Matrix values json.
pub(super) fn matrix_values_json(
    chunk: &[u8],
    cursor: usize,
) -> Option<String> {
    let mut values = Vec::with_capacity(16);
    for index in 0..16 {
        values.push(schema::read_f32(chunk, cursor + index * 4)?);
    }
    Some(format!(
        "[[{},{},{},{}],[{},{},{},{}],[{},{},{},{}],[{},{},{},{}]]",
        values[0],
        values[1],
        values[2],
        values[3],
        values[4],
        values[5],
        values[6],
        values[7],
        values[8],
        values[9],
        values[10],
        values[11],
        values[12],
        values[13],
        values[14],
        values[15]
    ))
}

/// Light type name.
pub(super) fn light_type_name(value: u32) -> &'static str {
    match value {
        0 => "ambient",
        1 => "directional",
        2 => "point",
        3 => "spot",
        _ => "unknown_light_type",
    }
}

/// Light children json.
pub(super) fn light_children_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> String {
    let mut extras = Vec::new();
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
        if let Some(extra) = light_child_json(chunk, cursor, id) {
            extras.push(extra);
        }
        cursor = next;
    }
    extras.join(",")
}

/// Light child json.
pub(super) fn light_child_json(
    chunk: &[u8],
    offset: usize,
    id: u32,
) -> Option<String> {
    let mut cursor = offset + 12;
    match id {
        0x0001_3001 => {
            let p = schema::read_point(chunk, &mut cursor)?;
            Some(format!(
                r#"{{"kind":"direction","value":[{},{},{}]}}"#,
                p[0], p[1], p[2]
            ))
        },
        0x0001_3002 => {
            let p = schema::read_point(chunk, &mut cursor)?;
            Some(format!(
                r#"{{"kind":"position","value":[{},{},{}]}}"#,
                p[0], p[1], p[2]
            ))
        },
        0x0001_3003 => {
            let phi = schema::read_f32(chunk, cursor)?;
            let theta = schema::read_f32(chunk, cursor + 4)?;
            let falloff = schema::read_f32(chunk, cursor + 8)?;
            let range = schema::read_f32(chunk, cursor + 12)?;
            Some(format!(
                concat!(
                    r#"{{"kind":"cone","#,
                    r#""phi":{},"#,
                    r#""theta":{},"#,
                    r#""falloff":{},"#,
                    r#""range":{}}}"#,
                ),
                phi, theta, falloff, range
            ))
        },
        0x0001_3004 => {
            let value = read_u32(chunk, cursor)?;
            Some(format!(r#"{{"kind":"shadow","value":{value}}}"#))
        },
        0x0001_3006 => light_decay_json(chunk, cursor),
        0x0001_3008 => {
            let value = read_u32(chunk, cursor)?;
            Some(format!(r#"{{"kind":"illumination_type","value":{value}}}"#))
        },
        _ => None,
    }
}

/// Light decay json.
pub(super) fn light_decay_json(
    chunk: &[u8],
    mut cursor: usize,
) -> Option<String> {
    let decay_type = read_u32(chunk, cursor)?;
    cursor += 4;
    let inner = schema::read_point(chunk, &mut cursor)?;
    let outer = schema::read_point(chunk, &mut cursor)?;
    Some(format!(
        concat!(
            r#"{{"kind":"decay_range","#,
            r#""type":{},"#,
            r#""inner":[{},{},{}],"#,
            r#""outer":[{},{},{}]}}"#,
        ),
        decay_type, inner[0], inner[1], inner[2], outer[0], outer[1], outer[2]
    ))
}

/// Recover frame controller json.
pub(super) fn recover_frame_controller_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let controller_type = read_fourcc(chunk, cursor)?;
    cursor += 4;
    let frame_offset = schema::read_f32(chunk, cursor)?;
    cursor += 4;
    let hierarchy_name = schema::read_pascal_at(chunk, &mut cursor)?;
    let animation_name = schema::read_pascal_at(chunk, &mut cursor)?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"frame_controller","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""type":"{}","#,
            r#""frame_offset":{},"#,
            r#""hierarchy_name":"{}","#,
            r#""animation_name":"{}"}}"#,
        ),
        escape_json(&name),
        version,
        escape_json(&controller_type),
        frame_offset,
        escape_json(&hierarchy_name),
        escape_json(&animation_name)
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover sprite json.
pub(super) fn recover_sprite_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let native_x = read_u32(chunk, cursor)?;
    cursor += 4;
    let native_y = read_u32(chunk, cursor)?;
    cursor += 4;
    let shader = schema::read_pascal_at(chunk, &mut cursor)?;
    let image_width = read_u32(chunk, cursor)?;
    cursor += 4;
    let image_height = read_u32(chunk, cursor)?;
    cursor += 4;
    let image_count = read_u32(chunk, cursor)?;
    cursor += 4;
    let blit_border = read_u32(chunk, cursor)?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            r#"{{"schema":"sprite","#,
            r#""name":"{}","#,
            r#""native_size":[{},{}],"#,
            r#""shader":"{}","#,
            r#""image_size":[{},{}],"#,
            r#""image_count":{},"#,
            r#""blit_border":{}}}"#,
        ),
        escape_json(&name),
        native_x,
        native_y,
        escape_json(&shader),
        image_width,
        image_height,
        image_count,
        blit_border
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}
