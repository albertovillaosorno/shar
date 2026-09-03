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
    const CAMERA: u32 = 0x0000_2200;
    if component.id != CAMERA || component.header_size != component.total_size {
        return None;
    }
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let mut values = [0_f32; 13];
    for value in &mut values {
        *value = schema::read_f32(chunk, cursor)?;
        cursor = cursor.checked_add(4)?;
    }
    if version != 2
        || cursor != component.header_size
        || values.iter().any(|value| !value.is_finite())
    {
        return None;
    }
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            "{{\"schema\":\"camera\",\"name\":\"{}\",",
            "\"version\":{},\"fov\":{},\"aspect_ratio\":{},",
            "\"near_clip\":{},\"far_clip\":{},",
            "\"position\":[{},{},{}],\"look\":[{},{},{}],",
            "\"up\":[{},{},{}]}}\n"
        ),
        escape_json(&name),
        version,
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
        values[12]
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
    const SYSTEM: u32 = 0x0001_5801;
    if component.id != SYSTEM || component.header_size != component.total_size {
        return None;
    }
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let factory = schema::read_pascal_at(chunk, &mut cursor)?;
    if version != 0 || cursor != component.header_size {
        return None;
    }
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
    const FACTORY: u32 = 0x0001_5800;
    if component.id != FACTORY {
        return None;
    }
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let version = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let frame_rate = schema::read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let anim_frames = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let ol_frames = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let cycle_anim = auxiliary::read_u16(chunk, cursor)?;
    cursor = cursor.checked_add(2)?;
    let enable_sorting = auxiliary::read_u16(chunk, cursor)?;
    cursor = cursor.checked_add(2)?;
    let num_emitters = usize::try_from(read_u32(chunk, cursor)?).ok()?;
    cursor = cursor.checked_add(4)?;
    if version != 0
        || !frame_rate.is_finite()
        || frame_rate <= 0.
        || num_emitters == 0
        || cursor != component.header_size
    {
        return None;
    }
    let (instancing_version, max_instances) =
        particle_factory_children_evidence(
            chunk,
            component.header_size,
            component.total_size,
            num_emitters,
        )?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            "{{\"schema\":\"particle_system_factory\",",
            "\"name\":\"{}\",\"version\":{},\"frame_rate\":{},",
            "\"num_anim_frames\":{},\"num_ol_frames\":{},",
            "\"cycle_anim\":{},\"enable_sorting\":{},",
            "\"num_emitters\":{},\"instancing_version\":{},",
            "\"max_instances\":{}}}\n"
        ),
        escape_json(&name),
        version,
        frame_rate,
        anim_frames,
        ol_frames,
        cycle_anim,
        enable_sorting,
        num_emitters,
        instancing_version,
        max_instances
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover exact particle-factory instancing evidence and emitter cardinality.
fn particle_factory_children_evidence(
    chunk: &[u8],
    header_size: usize,
    total_size: usize,
    num_emitters: usize,
) -> Option<(u32, u32)> {
    const INSTANCING_INFO: u32 = 0x0001_580b;
    const BASE_EMITTER: u32 = 0x0001_5805;
    const SPRITE_EMITTER: u32 = 0x0001_5806;
    const DRAWABLE_EMITTER: u32 = 0x0001_5807;
    let mut cursor = header_size;
    let mut child_index = 0_usize;
    let mut emitter_count = 0_usize;
    let mut instancing_evidence = None;
    while cursor < total_size {
        let (id, child_header, child_total) = read_chunk_header(chunk, cursor)?;
        let next = cursor.checked_add(child_total)?;
        if next > total_size {
            return None;
        }
        if child_index == 0 {
            if id != INSTANCING_INFO
                || child_header != 20
                || child_total != child_header
            {
                return None;
            }
            let instancing_version = read_u32(chunk, cursor.checked_add(12)?)?;
            let max_instances = read_u32(chunk, cursor.checked_add(16)?)?;
            if max_instances == 0 {
                return None;
            }
            instancing_evidence = Some((instancing_version, max_instances));
        } else if matches!(id, BASE_EMITTER | SPRITE_EMITTER | DRAWABLE_EMITTER)
        {
            emitter_count = emitter_count.checked_add(1)?;
        } else {
            return None;
        }
        child_index = child_index.checked_add(1)?;
        cursor = next;
    }
    if cursor != total_size || emitter_count != num_emitters {
        return None;
    }
    instancing_evidence
}

/// Recover light group json.
pub(super) fn recover_light_group_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    const LIGHT_GROUP: u32 = 0x0000_2380;
    if component.id != LIGHT_GROUP
        || component.header_size != component.total_size
    {
        return None;
    }
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let num_lights = usize::try_from(read_u32(chunk, cursor)?).ok()?;
    cursor = cursor.checked_add(4)?;
    let mut lights = Vec::with_capacity(num_lights);
    for _ in 0..num_lights {
        lights.push(schema::read_pascal_at(chunk, &mut cursor)?);
    }
    if cursor != component.header_size {
        return None;
    }
    let lights_json = lights
        .iter()
        .map(|light| format!("\"{}\"", escape_json(light)))
        .collect::<Vec<_>>()
        .join(",");
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            "{{\"schema\":\"light_group\",\"name\":\"{}\",",
            "\"num_lights\":{},\"lights\":[{}]}}\n"
        ),
        escape_json(&name),
        num_lights,
        lights_json
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
    chunks: Option<&[ChunkRecord]>,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let num_meshes = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let num_billboards = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    if version != 0 || cursor != component.header_size {
        return None;
    }
    let children = world_sphere_children_json(
        component,
        source,
        chunks?,
        usize::try_from(num_meshes).ok()?,
        usize::try_from(num_billboards).ok()?,
    )?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            "{{\"schema\":\"world_sphere_dsg\",",
            "\"name\":\"{}\",\"version\":{},",
            "\"num_meshes\":{},\"num_billboard_quads\":{},",
            "\"children\":[{}]}}\n"
        ),
        escape_json(&name),
        version,
        num_meshes,
        num_billboards,
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

/// Preserve and validate direct world-sphere child relationships.
fn world_sphere_children_json(
    component: &ChunkRecord,
    source: &[u8],
    chunks: &[ChunkRecord],
    declared_meshes: usize,
    declared_billboards: usize,
) -> Option<String> {
    const MESH: u32 = 0x0001_0000;
    const SKELETON: u32 = 0x0000_4500;
    const COMPOSITE_DRAWABLE: u32 = 0x0000_4512;
    const MULTI_CONTROLLER: u32 = 0x0000_48a0;
    const QUAD_GROUP: u32 = 0x0001_7002;
    const ANIMATION: u32 = 0x0012_1000;
    const FRAME_CONTROLLER: u32 = 0x0012_1200;
    const LENS_FLARE_DSG: u32 = 0x03f0_000d;

    let direct = chunks
        .iter()
        .filter(|child| child.parent_ordinal == Some(component.ordinal))
        .collect::<Vec<_>>();
    let mut physical_cursor =
        component.offset.checked_add(component.header_size)?;
    let physical_end = component.offset.checked_add(component.total_size)?;
    let mut meshes = 0_usize;
    let mut billboards = 0_usize;
    let mut rendered = Vec::with_capacity(direct.len());
    for child in direct {
        if child.offset != physical_cursor
            || !matches!(
                child.id,
                MESH | SKELETON
                    | COMPOSITE_DRAWABLE
                    | MULTI_CONTROLLER
                    | QUAD_GROUP
                    | ANIMATION
                    | FRAME_CONTROLLER
                    | LENS_FLARE_DSG
            )
        {
            return None;
        }
        physical_cursor = physical_cursor.checked_add(child.total_size)?;
        meshes += usize::from(child.id == MESH);
        billboards += usize::from(child.id == QUAD_GROUP);
        let child_bytes = raw_component_bytes(child, source).ok()?;
        let mut name_cursor = 12;
        let child_name = schema::read_pascal_at(child_bytes, &mut name_cursor)?;
        rendered.push(format!(
            concat!(
                "{{\"source_ordinal\":{},\"kind\":\"{}\",",
                "\"name\":\"{}\"}}"
            ),
            child.ordinal,
            escape_json(child.kind.label()),
            escape_json(&child_name)
        ));
    }
    if physical_cursor != physical_end
        || meshes != declared_meshes
        || billboards != declared_billboards
    {
        return None;
    }
    Some(rendered.join(","))
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
    if position.iter().any(|value| !value.is_finite()) {
        return None;
    }
    let num_triggers = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    if cursor != component.header_size {
        return None;
    }
    let locator_type_name =
        crate::adapters::driven::decoders::locator::type_name(locator_type)?;
    let data_interpretation =
        crate::adapters::driven::decoders::locator::data_interpretation_json(
            locator_type,
            &data,
            num_triggers,
        )?;
    let (triggers, decoded_trigger_count) = trigger_volumes_json(
        chunk,
        component.header_size,
        component.total_size,
    )?;
    if decoded_trigger_count != usize::try_from(num_triggers).ok()? {
        return None;
    }
    let extra_matrices = extra_matrices_json(
        chunk,
        component.header_size,
        component.total_size,
    )?;
    let splines = locator_splines_json(
        chunk,
        component.header_size,
        component.total_size,
    )?;
    let kind = component.kind.label();
    let file_name = schema::fallback_name(kind, kind_index, &name);
    let json = format!(
        "{{\"schema\":\"locator\",\"name\":\"{}\",\"locator_type\":{},\"\
         locator_type_name\":\"{}\",\"position\":[{},{},{}],\"\
         num_data_elements\":{},\"data_elements_u32\":[{}],\"\
         data_elements_f32\":[{}],\"data_ascii_lossy\":\"{}\",\"\
         data_interpretation\":{},\"num_triggers\":{},\"trigger_volumes\":\
         [{}],\"extra_matrices\":[{}],\"splines\":[{}]}}\n",
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
        triggers,
        extra_matrices,
        splines
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
) -> Option<(String, usize)> {
    const TRIGGER_VOLUME: u32 = 0x0300_0006;
    const SPLINE: u32 = 0x0300_0007;
    const EXTRA_MATRIX: u32 = 0x0300_000c;
    let mut triggers = Vec::new();
    while cursor < end {
        let (id, header_size, total_size) = read_chunk_header(chunk, cursor)?;
        let next = cursor.checked_add(total_size)?;
        if total_size < header_size || next > end {
            return None;
        }
        match id {
            TRIGGER_VOLUME => {
                triggers.push(trigger_volume_json(
                    chunk,
                    cursor,
                    header_size,
                    total_size,
                )?);
            },
            SPLINE | EXTRA_MATRIX => {},
            _ => return None,
        }
        cursor = next;
    }
    let count = triggers.len();
    Some((triggers.join(","), count))
}

/// Preserve every direct locator spline child in source order.
pub(super) fn locator_splines_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> Option<String> {
    const TRIGGER_VOLUME: u32 = 0x0300_0006;
    const SPLINE: u32 = 0x0300_0007;
    const EXTRA_MATRIX: u32 = 0x0300_000c;
    let mut splines = Vec::new();
    while cursor < end {
        let (id, header_size, total_size) = read_chunk_header(chunk, cursor)?;
        let next = cursor.checked_add(total_size)?;
        if total_size < header_size || next > end {
            return None;
        }
        match id {
            SPLINE => splines.push(locator_spline_json(
                chunk,
                cursor,
                header_size,
                total_size,
            )?),
            TRIGGER_VOLUME | EXTRA_MATRIX => {},
            _ => return None,
        }
        cursor = next;
    }
    Some(splines.join(","))
}

/// Decode one locator spline and its one observed nested rail record.
fn locator_spline_json(
    chunk: &[u8],
    offset: usize,
    header_size: usize,
    total_size: usize,
) -> Option<String> {
    const RAIL: u32 = 0x0300_000a;
    let end = offset.checked_add(total_size)?;
    let mut cursor = offset.checked_add(12)?;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let num_control_points = usize::try_from(read_u32(chunk, cursor)?).ok()?;
    cursor = cursor.checked_add(4)?;
    let mut control_points = Vec::with_capacity(num_control_points);
    for _ in 0..num_control_points {
        let point = schema::read_point(chunk, &mut cursor)?;
        if point.iter().any(|value| !value.is_finite()) {
            return None;
        }
        control_points
            .push(format!("[{},{},{}]", point[0], point[1], point[2]));
    }
    if cursor != offset.checked_add(header_size)? {
        return None;
    }
    let (rail_id, rail_header, rail_total) = read_chunk_header(chunk, cursor)?;
    if rail_id != RAIL || cursor.checked_add(rail_total)? != end {
        return None;
    }
    let rail = locator_rail_json(chunk, cursor, rail_header, rail_total)?;
    Some(format!(
        concat!(
            r#"{{"name":"{}","num_control_points":{},"#,
            r#""control_points":[{}],"rail":{}}}"#,
        ),
        escape_json(&name),
        num_control_points,
        control_points.join(","),
        rail
    ))
}

/// Decode one schema-declared rail record without assigning camera semantics.
fn locator_rail_json(
    chunk: &[u8],
    offset: usize,
    header_size: usize,
    total_size: usize,
) -> Option<String> {
    if header_size != total_size {
        return None;
    }
    let mut cursor = offset.checked_add(12)?;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let behaviour = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let min_radius = schema::read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let max_radius = schema::read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let track_rail = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let track_dist = schema::read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let reverse_sense = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let fov = schema::read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let target_offset = schema::read_point(chunk, &mut cursor)?;
    let axis_play = schema::read_point(chunk, &mut cursor)?;
    let position_lag = schema::read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let target_lag = schema::read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let floats = [
        min_radius,
        max_radius,
        track_dist,
        fov,
        position_lag,
        target_lag,
    ];
    if cursor != offset.checked_add(header_size)?
        || floats.iter().any(|value| !value.is_finite())
        || target_offset.iter().any(|value| !value.is_finite())
        || axis_play.iter().any(|value| !value.is_finite())
    {
        return None;
    }
    Some(format!(
        concat!(
            r#"{{"name":"{}","behaviour":{},"min_radius":{},"#,
            r#""max_radius":{},"track_rail":{},"track_dist":{},"#,
            r#""reverse_sense":{},"fov":{},"target_offset":[{},{},{}],"#,
            r#""axis_play":[{},{},{}],"position_lag":{},"target_lag":{}}}"#,
        ),
        escape_json(&name),
        behaviour,
        min_radius,
        max_radius,
        track_rail,
        track_dist,
        reverse_sense,
        fov,
        target_offset[0],
        target_offset[1],
        target_offset[2],
        axis_play[0],
        axis_play[1],
        axis_play[2],
        position_lag,
        target_lag
    ))
}

/// Preserve every direct locator extra-matrix child in source order.
pub(super) fn extra_matrices_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> Option<String> {
    const TRIGGER_VOLUME: u32 = 0x0300_0006;
    const SPLINE: u32 = 0x0300_0007;
    const EXTRA_MATRIX: u32 = 0x0300_000c;
    let mut matrices = Vec::new();
    while cursor < end {
        let (id, header_size, total_size) = read_chunk_header(chunk, cursor)?;
        let next = cursor.checked_add(total_size)?;
        if total_size < header_size || next > end {
            return None;
        }
        match id {
            EXTRA_MATRIX => matrices.push(extra_matrix_json(
                chunk,
                cursor,
                header_size,
                total_size,
            )?),
            TRIGGER_VOLUME | SPLINE => {},
            _ => return None,
        }
        cursor = next;
    }
    Some(matrices.join(","))
}

/// Decode one schema-declared 4-by-4 extra matrix without interpretation.
fn extra_matrix_json(
    chunk: &[u8],
    offset: usize,
    header_size: usize,
    total_size: usize,
) -> Option<String> {
    if header_size != total_size {
        return None;
    }
    let mut cursor = offset.checked_add(12)?;
    let mut matrix = [[0_f32; 4]; 4];
    for row in &mut matrix {
        for value in row {
            *value = schema::read_f32(chunk, cursor)?;
            if !value.is_finite() {
                return None;
            }
            cursor = cursor.checked_add(4)?;
        }
    }
    (cursor == offset.checked_add(header_size)?).then(|| matrix_json(&matrix))
}

/// Trigger volume json.
pub(super) fn trigger_volume_json(
    chunk: &[u8],
    offset: usize,
    header_size: usize,
    total_size: usize,
) -> Option<String> {
    if header_size != total_size {
        return None;
    }
    let mut cursor = offset.checked_add(12)?;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let volume_type = read_u32(chunk, cursor)?;
    if !matches!(volume_type, 0 | 1) {
        return None;
    }
    cursor = cursor.checked_add(4)?;
    let scale = [
        schema::read_f32(chunk, cursor)?,
        schema::read_f32(chunk, cursor.checked_add(4)?)?,
        schema::read_f32(chunk, cursor.checked_add(8)?)?,
    ];
    if scale.iter().any(|value| !value.is_finite()) {
        return None;
    }
    cursor = cursor.checked_add(12)?;
    let mut matrix = [[0f32; 4]; 4];
    for row in &mut matrix {
        for value in row {
            *value = schema::read_f32(chunk, cursor)?;
            if !value.is_finite() {
                return None;
            }
            cursor = cursor.checked_add(4)?;
        }
    }
    if cursor != offset.checked_add(header_size)? {
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
    mut cursor: usize,
    end: usize,
    expected: usize,
) -> Option<String> {
    const DEFINITION: u32 = 0x0001_1001;
    const TEXTURE: u32 = 0x0001_1002;
    const INT: u32 = 0x0001_1003;
    const FLOAT: u32 = 0x0001_1004;
    const COLOUR: u32 = 0x0001_1005;
    const VECTOR: u32 = 0x0001_1006;
    const MATRIX: u32 = 0x0001_1007;
    let mut params = Vec::with_capacity(expected);
    while cursor < end {
        let (id, header_size, total_size) = read_chunk_header(chunk, cursor)?;
        let next = cursor.checked_add(total_size)?;
        if total_size < header_size || next > end {
            return None;
        }
        match id {
            DEFINITION => {
                if !shader_definition_is_exact(
                    chunk,
                    cursor,
                    header_size,
                    total_size,
                )? {
                    return None;
                }
            },
            TEXTURE | INT | FLOAT | COLOUR | VECTOR | MATRIX => {
                params.push(shader_param_json(
                    chunk,
                    cursor,
                    id,
                    header_size,
                    total_size,
                )?);
            },
            _ => return None,
        }
        cursor = next;
    }
    (cursor == end && params.len() == expected).then(|| params.join(","))
}

/// Validate a childless shader-definition record against its schema fields.
fn shader_definition_is_exact(
    chunk: &[u8],
    offset: usize,
    header_size: usize,
    total_size: usize,
) -> Option<bool> {
    let header_end = offset.checked_add(header_size)?;
    let total_end = offset.checked_add(total_size)?;
    let mut cursor = offset.checked_add(12)?;
    let _name = schema::read_pascal_at(chunk, &mut cursor)?;
    let length = usize::try_from(read_u32(chunk, cursor)?).ok()?;
    cursor = cursor.checked_add(4)?;
    let definition_end = cursor.checked_add(length)?;
    let definition = chunk.get(cursor..definition_end)?;
    let _definition_text = std::str::from_utf8(definition).ok()?;
    cursor = definition_end;
    Some(cursor == header_end && header_end == total_end)
}

/// Shader param json.
pub(super) fn shader_param_json(
    chunk: &[u8],
    offset: usize,
    id: u32,
    header_size: usize,
    total_size: usize,
) -> Option<String> {
    let header_end = offset.checked_add(header_size)?;
    let total_end = offset.checked_add(total_size)?;
    let mut cursor = offset.checked_add(12)?;
    let value = match id {
        0x0001_1002 => {
            let param = read_fourcc(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            let value = schema::read_pascal_at(chunk, &mut cursor)?;
            format!(
                r#"{{"kind":"texture","param":"{}","value":"{}"}}"#,
                escape_json(&param),
                escape_json(&value),
            )
        },
        0x0001_1003 => {
            let param = read_fourcc(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            let value = read_u32(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            format!(
                r#"{{"kind":"int","param":"{}","value":{}}}"#,
                escape_json(&param),
                value,
            )
        },
        0x0001_1004 => {
            let param = read_fourcc(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            let value = schema::read_f32(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            if !value.is_finite() {
                return None;
            }
            format!(
                r#"{{"kind":"float","param":"{}","value":{}}}"#,
                escape_json(&param),
                value,
            )
        },
        0x0001_1005 => {
            let param = read_fourcc(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            let value = read_u32(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            format!(
                r#"{{"kind":"colour","param":"{}","value":{}}}"#,
                escape_json(&param),
                value,
            )
        },
        0x0001_1006 => {
            let param = read_fourcc(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            let values = [
                schema::read_f32(chunk, cursor)?,
                schema::read_f32(chunk, cursor.checked_add(4)?)?,
                schema::read_f32(chunk, cursor.checked_add(8)?)?,
            ];
            cursor = cursor.checked_add(12)?;
            if values.iter().any(|value| !value.is_finite()) {
                return None;
            }
            format!(
                r#"{{"kind":"vector","param":"{}","value":[{},{},{}]}}"#,
                escape_json(&param),
                values[0],
                values[1],
                values[2],
            )
        },
        0x0001_1007 => {
            let param = read_fourcc(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            let mut values = [0_f32; 16];
            for value in &mut values {
                *value = schema::read_f32(chunk, cursor)?;
                cursor = cursor.checked_add(4)?;
                if !value.is_finite() {
                    return None;
                }
            }
            format!(
                "{{\"kind\":\"matrix\",\"param\":\"{}\",\"value\":{}}}",
                escape_json(&param),
                matrix_values_array_json(&values),
            )
        },
        _ => return None,
    };
    (cursor == header_end && header_end == total_end).then_some(value)
}

/// Render an already-decoded 4x4 matrix in source order.
fn matrix_values_array_json(values: &[f32; 16]) -> String {
    format!(
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
        values[15],
    )
}

/// Game attr params json.
pub(super) fn game_attr_params_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
    expected: usize,
) -> Option<String> {
    let mut params = Vec::with_capacity(expected);
    for _ in 0..expected {
        let (id, header_size, total_size) = read_chunk_header(chunk, cursor)?;
        let value_size = match id {
            0x0001_2001..=0x0001_2003 => 4_usize,
            0x0001_2004 => 12,
            0x0001_2005 => 64,
            _ => return None,
        };
        let next = cursor.checked_add(total_size)?;
        if header_size != total_size || next > end {
            return None;
        }
        let mut field_cursor = cursor.checked_add(12)?;
        let _param_name = schema::read_pascal_at(chunk, &mut field_cursor)?;
        let expected_end = field_cursor.checked_add(value_size)?;
        if expected_end != cursor.checked_add(header_size)? {
            return None;
        }
        params.push(game_attr_param_json(chunk, cursor, id)?);
        cursor = next;
    }
    (cursor == end).then(|| params.join(","))
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

/// Decode every direct light child with exact schema framing.
pub(super) fn light_children_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> Option<String> {
    let mut extras = Vec::new();
    let mut seen = Vec::new();
    while cursor < end {
        let (id, header_size, total_size) = read_chunk_header(chunk, cursor)?;
        let next = cursor.checked_add(total_size)?;
        if total_size < header_size || next > end || seen.contains(&id) {
            return None;
        }
        extras.push(light_child_json(
            chunk,
            cursor,
            id,
            header_size,
            total_size,
        )?);
        seen.push(id);
        cursor = next;
    }
    Some(extras.join(","))
}

/// Decode one schema-declared direct light child.
pub(super) fn light_child_json(
    chunk: &[u8],
    offset: usize,
    id: u32,
    header_size: usize,
    total_size: usize,
) -> Option<String> {
    let mut cursor = offset.checked_add(12)?;
    match id {
        0x0001_3001 | 0x0001_3002 => {
            if header_size != 24 || total_size != header_size {
                return None;
            }
            let p = schema::read_point(chunk, &mut cursor)?;
            if cursor != offset.checked_add(header_size)?
                || p.iter().any(|value| !value.is_finite())
            {
                return None;
            }
            let kind = if id == 0x0001_3001 {
                "direction"
            } else {
                "position"
            };
            Some(format!(
                r#"{{"kind":"{}","value":[{},{},{}]}}"#,
                kind, p[0], p[1], p[2]
            ))
        },
        0x0001_3003 => {
            if header_size != 28 || total_size != header_size {
                return None;
            }
            let phi = schema::read_f32(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            let theta = schema::read_f32(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            let falloff = schema::read_f32(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            let range = schema::read_f32(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            if cursor != offset.checked_add(header_size)?
                || [phi, theta, falloff, range]
                    .iter()
                    .any(|value| !value.is_finite())
            {
                return None;
            }
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
        0x0001_3004 | 0x0001_3008 => {
            if header_size != 16 || total_size != header_size {
                return None;
            }
            let value = read_u32(chunk, cursor)?;
            cursor = cursor.checked_add(4)?;
            if cursor != offset.checked_add(header_size)? {
                return None;
            }
            let kind = if id == 0x0001_3004 {
                "shadow"
            } else {
                "illumination_type"
            };
            Some(format!(r#"{{"kind":"{kind}","value":{value}}}"#))
        },
        0x0001_3006 => light_decay_json(chunk, offset, header_size, total_size),
        _ => None,
    }
}

/// Decode one decay-range child and its optional authored Y rotation.
pub(super) fn light_decay_json(
    chunk: &[u8],
    offset: usize,
    header_size: usize,
    total_size: usize,
) -> Option<String> {
    const ROTATION_Y: u32 = 0x0001_3007;
    if header_size != 40 || total_size < header_size {
        return None;
    }
    let mut cursor = offset.checked_add(12)?;
    let decay_type = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let inner = schema::read_point(chunk, &mut cursor)?;
    let outer = schema::read_point(chunk, &mut cursor)?;
    if cursor != offset.checked_add(header_size)?
        || inner
            .iter()
            .chain(outer.iter())
            .any(|value| !value.is_finite())
    {
        return None;
    }
    let end = offset.checked_add(total_size)?;
    let rotation = if cursor == end {
        None
    } else {
        let (id, child_header, child_total) = read_chunk_header(chunk, cursor)?;
        if id != ROTATION_Y
            || child_header != 16
            || child_total != child_header
            || cursor.checked_add(child_total)? != end
        {
            return None;
        }
        let value = schema::read_f32(chunk, cursor.checked_add(12)?)?;
        if !value.is_finite() {
            return None;
        }
        Some(value)
    };
    let rotation_json = rotation
        .map_or_else(String::new, |value| format!(r#", "rotation_y":{value}"#));
    Some(format!(
        concat!(
            r#"{{"kind":"decay_range","#,
            r#""type":{},"#,
            r#""inner":[{},{},{}],"#,
            r#""outer":[{},{},{}]{} }}"#,
        ),
        decay_type,
        inner[0],
        inner[1],
        inner[2],
        outer[0],
        outer[1],
        outer[2],
        rotation_json
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
    if component.id == 0x0012_1200
        && (cursor != component.header_size
            || component.total_size != component.header_size
            || !frame_offset.is_finite())
    {
        return None;
    }
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
    chunks: Option<&[ChunkRecord]>,
) -> Option<RecoveredComponent> {
    const SPRITE: u32 = 0x0001_9005;
    if component.id != SPRITE {
        return None;
    }
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = schema::read_pascal_at(chunk, &mut cursor)?;
    let native_x = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let native_y = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let shader = schema::read_pascal_at(chunk, &mut cursor)?;
    let image_width = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let image_height = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let image_count = usize::try_from(read_u32(chunk, cursor)?).ok()?;
    cursor = cursor.checked_add(4)?;
    let blit_border = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    if cursor != component.header_size {
        return None;
    }
    let images = sprite_images_json(component, source, chunks?, image_count)?;
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
            r#""blit_border":{},"#,
            r#""images":[{}]}}"#,
        ),
        escape_json(&name),
        native_x,
        native_y,
        escape_json(&shader),
        image_width,
        image_height,
        image_count,
        blit_border,
        images
    );
    Some(json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Preserve exact direct image relationships for one sprite.
fn sprite_images_json(
    component: &ChunkRecord,
    source: &[u8],
    chunks: &[ChunkRecord],
    declared_images: usize,
) -> Option<String> {
    const IMAGE: u32 = 0x0001_9001;
    let direct = chunks
        .iter()
        .filter(|child| child.parent_ordinal == Some(component.ordinal))
        .collect::<Vec<_>>();
    if direct.len() != declared_images {
        return None;
    }
    let mut physical_cursor =
        component.offset.checked_add(component.header_size)?;
    let physical_end = component.offset.checked_add(component.total_size)?;
    let mut rendered = Vec::with_capacity(direct.len());
    for child in direct {
        if child.id != IMAGE || child.offset != physical_cursor {
            return None;
        }
        physical_cursor = physical_cursor.checked_add(child.total_size)?;
        let child_bytes = raw_component_bytes(child, source).ok()?;
        let mut name_cursor = 12;
        let child_name = schema::read_pascal_at(child_bytes, &mut name_cursor)?;
        rendered.push(format!(
            concat!("{{\"source_ordinal\":{},", "\"authored_name\":\"{}\"}}"),
            child.ordinal,
            escape_json(&child_name)
        ));
    }
    if physical_cursor != physical_end {
        return None;
    }
    Some(rendered.join(","))
}
