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
//   - P3D world schema recovery helpers.
// - Must-Not:
//   - Own filesystem publication or command-line composition.
// - Allows:
//   - Exact bounded decoding and JSON recovery for its schema family.
// - Split-When:
//   - Split when one decoder family exceeds the fixed declaration limit.
// - Merge-When:
//   - Merge when another file owns the identical decoder family.
// - Summary:
//   - P3D world schema recovery helpers.
// - Description:
//   - Implements world and simulation schema recovery inside the extractor
//     module scope.
// - Usage:
//   - Included by the owning extractor adapter.
// - Defaults:
//   - Unsupported or malformed payloads fail closed through `Option`.
//

//! P3D world schema recovery helpers.

use super::{
    ChunkRecord, PathBuf, RecoveredComponent, auxiliary, component_name,
    escape_json, kind_schema, raw_component_bytes, read_chunk_header, read_u32,
    render, sanitize, schema_recovery,
};

/// Recovers one schema through the first matching decoder family.
pub(super) fn recover_schema_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
    chunks: Option<&[ChunkRecord]>,
) -> Option<RecoveredComponent> {
    if matches!(component.kind.label(), "srr_anim_dsg" | "srr_anim_coll_dsg") {
        return recover_anim_dsg_json(component, source, kind_index, chunks?);
    }
    if component.kind.label() == "srr_tree_dsg" {
        return recover_tree_json(component, source, kind_index, chunks?);
    }
    recover_world_schema_json(component, source, kind_index)
        .or_else(|| {
            schema_recovery::recover_render_schema_json(
                component, source, kind_index, chunks,
            )
        })
        .or_else(|| {
            recover_auxiliary_schema_json(component, source, kind_index)
        })
}

/// Routes world, simulation, and foundational schema families.
pub(super) fn recover_world_schema_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    match component.kind.label() {
        "text_bible" => {
            render::recover_text_bible_json(component, source, kind_index)
        },
        "shader" => recover_shader_json(component, source, kind_index),
        "game_attr" => recover_game_attr_json(component, source, kind_index),
        "light" => recover_light_json(component, source, kind_index),
        "srr_locator" => {
            render::recover_srr_locator_json(component, source, kind_index)
        },
        "srr_fence_dsg" => recover_fence_json(component, source, kind_index),
        "srr_entity_dsg" => {
            render::recover_entity_dsg_json(component, source, kind_index)
        },
        "srr_insta_entity_dsg" => {
            render::recover_insta_entity_dsg_json(component, source, kind_index)
        },
        "srr_dyna_phys_dsg"
        | "srr_insta_anim_dyna_phys_dsg"
        | "srr_static_phys_dsg"
        | "srr_insta_static_phys_dsg" => {
            recover_physics_dsg_json(component, source, kind_index)
        },
        "srr_road_segment_data" => {
            recover_road_segment_json(component, source, kind_index)
        },
        "srr_road" => recover_road_json(component, source, kind_index),
        "srr_intersection" => {
            recover_intersection_json(component, source, kind_index)
        },
        "srr_intersect_dsg" => {
            recover_intersect_json(component, source, kind_index)
        },
        "simulation_collision_object" => {
            recover_collision_object_json(component, source, kind_index)
        },
        "simulation_physics_object" => {
            recover_physics_object_json(component, source, kind_index)
        },
        "srr_chunk_set" => {
            recover_chunk_set_json(component, source, kind_index)
        },
        _ => None,
    }
}

/// Routes auxiliary, UI, and remaining specialized schema families.
pub(super) fn recover_auxiliary_schema_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    match component.kind.label() {
        "locator" => {
            auxiliary::recover_locator_json(component, source, kind_index)
        },
        "srr_ped_path" => {
            auxiliary::recover_ped_path_json(component, source, kind_index)
        },
        "srr_follow_cam" => {
            auxiliary::recover_follow_cam_json(component, source, kind_index)
        },
        "export_info" => {
            auxiliary::recover_export_info_json(component, source, kind_index)
        },
        "srr_breakable_object" => auxiliary::recover_breakable_object_json(
            component, source, kind_index,
        ),
        "srr_attribute_table" => auxiliary::recover_attribute_table_json(
            component, source, kind_index,
        ),
        "srr_lens_flare_dsg" => {
            auxiliary::recover_lens_flare_json(component, source, kind_index)
        },
        "animated_object" => auxiliary::recover_animated_object_json(
            component, source, kind_index,
        ),
        "animated_object_factory" => {
            auxiliary::recover_animated_object_factory_json(
                component, source, kind_index,
            )
        },
        "state_prop" => {
            auxiliary::recover_state_prop_json(component, source, kind_index)
        },
        "vertex_expression_group" | "vertex_expression_mixer" => {
            auxiliary::recover_vertex_expression_json(
                component, source, kind_index,
            )
        },
        "quad_group" => {
            auxiliary::recover_quad_group_json(component, source, kind_index)
        },
        "texture_font" => {
            auxiliary::recover_texture_font_json(component, source, kind_index)
        },
        "scrooby_project" => auxiliary::recover_scrooby_project_json(
            component, source, kind_index,
        ),
        "scrooby_screen" => auxiliary::recover_scrooby_screen_json(
            component, source, kind_index,
        ),
        "scrooby_page" => {
            auxiliary::recover_scrooby_page_json(component, source, kind_index)
        },
        "scrooby_layer" => {
            auxiliary::recover_scrooby_layer_json(component, source, kind_index)
        },
        "scrooby_group" => super::scrooby_widget::recover_group_json(
            component, source, kind_index,
        ),
        "scrooby_multi_sprite" => {
            super::scrooby_widget::recover_multi_sprite_json(
                component, source, kind_index,
            )
        },
        "scrooby_multi_text" => super::scrooby_widget::recover_multi_text_json(
            component, source, kind_index,
        ),
        "scrooby_pure3d_object" => {
            super::scrooby_widget::recover_pure3d_object_json(
                component, source, kind_index,
            )
        },
        "scrooby_polygon" => super::scrooby_widget::recover_polygon_json(
            component, source, kind_index,
        ),
        "scrooby_string_text_bible" => {
            super::scrooby_widget::recover_string_text_bible_json(
                component, source, kind_index,
            )
        },
        "scrooby_string_hardcoded" => {
            super::scrooby_widget::recover_string_hardcoded_json(
                component, source, kind_index,
            )
        },
        "scrooby_image_resource" => {
            auxiliary::recover_scrooby_image_resource_json(
                component, source, kind_index,
            )
        },
        "scrooby_pure3d_resource" => {
            auxiliary::recover_scrooby_pure3d_resource_json(
                component, source, kind_index,
            )
        },
        "scrooby_text_style_resource" => {
            auxiliary::recover_scrooby_text_style_resource_json(
                component, source, kind_index,
            )
        },
        "scrooby_text_bible_resource" => {
            auxiliary::recover_scrooby_text_bible_resource_json(
                component, source, kind_index,
            )
        },
        "srr_inst_particle_system" => {
            auxiliary::recover_inst_particle_system_json(
                component, source, kind_index,
            )
        },
        _ => None,
    }
}

/// Recover shader json.
pub(super) fn recover_shader_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let pddi_shader_name = read_pascal_at(chunk, &mut cursor)?;
    let has_translucency = read_u32(chunk, cursor)?;
    cursor += 4;
    let vertex_needs = read_u32(chunk, cursor)?;
    cursor += 4;
    let vertex_mask = read_u32(chunk, cursor)?;
    cursor += 4;
    let num_params = read_u32(chunk, cursor)?;
    let fallback = format!("shader_{kind_index:04}");
    let file_name = sanitize(if name.is_empty() {
        &fallback
    } else {
        &name
    });
    let params = render::shader_params_json(
        chunk,
        component.header_size,
        component.total_size,
    );
    let json = format!(
        concat!(
            r#"{{"schema":"shader","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""pddi_shader_name":"{}","#,
            r#""has_translucency":{},"#,
            r#""vertex_needs":{},"#,
            r#""vertex_mask":{},"#,
            r#""num_params":{},"#,
            r#""params":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        escape_json(&pddi_shader_name),
        has_translucency,
        vertex_needs,
        vertex_mask,
        num_params,
        params
    );
    Some(RecoveredComponent {
        relative_path: PathBuf::from("shader")
            .join(format!("{file_name}.json")),
        name,
        bytes: json.into_bytes(),
        payload_format: "schema_json".to_owned(),
        recovery_status: "decoded_schema_payload".to_owned(),
    })
}

/// Recover game attr json.
pub(super) fn recover_game_attr_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor += 4;
    let num_params = usize::try_from(read_u32(chunk, cursor)?).ok()?;
    cursor = cursor.checked_add(4)?;
    if cursor != component.header_size {
        return None;
    }
    let fallback = format!("game_attr_{kind_index:04}");
    let file_name = sanitize(if name.is_empty() {
        &fallback
    } else {
        &name
    });
    let params = render::game_attr_params_json(
        chunk,
        component.header_size,
        component.total_size,
        num_params,
    )?;
    let json = format!(
        concat!(
            r#"{{"schema":"game_attr","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""num_params":{},"#,
            r#""params":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        num_params,
        params
    );
    Some(RecoveredComponent {
        relative_path: PathBuf::from("game_attr")
            .join(format!("{file_name}.json")),
        name,
        bytes: json.into_bytes(),
        payload_format: "schema_json".to_owned(),
        recovery_status: "decoded_schema_payload".to_owned(),
    })
}

/// Recover light json.
pub(super) fn recover_light_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    const LIGHT: u32 = 0x0001_3000;
    if component.id != LIGHT {
        return None;
    }
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let light_type = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let colour = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let constant = read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let linear = read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let squared = read_f32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let enabled = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    if cursor != component.header_size
        || [constant, linear, squared]
            .iter()
            .any(|value| !value.is_finite())
    {
        return None;
    }
    let fallback = format!("light_{kind_index:04}");
    let file_name = sanitize(if name.is_empty() {
        &fallback
    } else {
        &name
    });
    let extras = render::light_children_json(
        chunk,
        component.header_size,
        component.total_size,
    )?;
    let json = format!(
        concat!(
            r#"{{"schema":"light","#,
            r#""name":"{}","#,
            r#""version":{},"#,
            r#""type":{},"#,
            r#""type_name":"{}","#,
            r#""colour":{},"#,
            r#""attenuation":{{"#,
            r#""constant":{},"#,
            r#""linear":{},"#,
            r#""squared":{}}},"#,
            r#""enabled":{},"#,
            r#""extras":[{}]}}"#,
        ),
        escape_json(&name),
        version,
        light_type,
        render::light_type_name(light_type),
        colour,
        constant,
        linear,
        squared,
        enabled,
        extras
    );
    Some(RecoveredComponent {
        relative_path: PathBuf::from("light").join(format!("{file_name}.json")),
        name,
        bytes: json.into_bytes(),
        payload_format: "schema_json".to_owned(),
        recovery_status: "decoded_schema_payload".to_owned(),
    })
}

/// Recover fence json.
pub(super) fn recover_fence_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    const WALL: u32 = 0x0300_0000;
    let chunk = raw_component_bytes(component, source).ok()?;
    if component.header_size != 12 {
        return None;
    }
    let wall_offset = component.header_size;
    let (wall_id, wall_header, wall_total) =
        read_chunk_header(chunk, wall_offset)?;
    let wall_end = wall_offset.checked_add(wall_total)?;
    if wall_id != WALL
        || wall_header != 48
        || wall_total != wall_header
        || wall_end != component.total_size
    {
        return None;
    }
    let mut cursor = wall_offset.checked_add(12)?;
    let start = read_point(chunk, &mut cursor)?;
    let end = read_point(chunk, &mut cursor)?;
    let normal = read_point(chunk, &mut cursor)?;
    if cursor != wall_end
        || start
            .iter()
            .chain(end.iter())
            .chain(normal.iter())
            .any(|value| !value.is_finite())
    {
        return None;
    }
    let name = format!("srr_fence_dsg_{kind_index:04}");
    let json = format!(
        "{{\"schema\":\"tlFenceDSGChunk.sc/tlWallChunk.sc\",\"name\":\"{}\",\"\
         start\":[{},{},{}],\"end\":[{},{},{}],\"normal\":[{},{},{}]}}\n",
        name,
        start[0],
        start[1],
        start[2],
        end[0],
        end[1],
        end[2],
        normal[0],
        normal[1],
        normal[2]
    );
    Some(RecoveredComponent {
        relative_path: PathBuf::from("srr_fence_dsg")
            .join(format!("{name}.json")),
        name,
        bytes: json.into_bytes(),
        payload_format: "schema_json".to_owned(),
        recovery_status: "decoded_schema_payload".to_owned(),
    })
}

/// Read point.
pub(super) fn read_point(bytes: &[u8], cursor: &mut usize) -> Option<[f32; 3]> {
    let x = read_f32(bytes, *cursor)?;
    *cursor += 4;
    let y = read_f32(bytes, *cursor)?;
    *cursor += 4;
    let z = read_f32(bytes, *cursor)?;
    *cursor += 4;
    Some([x, y, z])
}

/// Read pascal at.
pub(super) fn read_pascal_at(
    bytes: &[u8],
    cursor: &mut usize,
) -> Option<String> {
    let length = usize::from(*bytes.get(*cursor)?);
    let start = (*cursor).checked_add(1)?;
    let end = start.checked_add(length)?;
    let raw = bytes.get(start..end)?;
    let value = std::str::from_utf8(raw).ok()?.to_owned();
    *cursor = end;
    Some(value)
}

/// Read f32.
pub(super) fn read_f32(bytes: &[u8], offset: usize) -> Option<f32> {
    let end = offset.checked_add(4)?;
    let slice = bytes.get(offset..end)?;
    Some(f32::from_le_bytes([slice[0], slice[1], slice[2], slice[3]]))
}

/// Recover one animated DSG container with direct source relationships.
pub(super) fn recover_anim_dsg_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
    chunks: &[ChunkRecord],
) -> Option<RecoveredComponent> {
    const ANIM_DSG: u32 = 0x03f0_000c;
    const ANIM_COLL_DSG: u32 = 0x03f0_0008;
    const COMPOSITE_DRAWABLE: u32 = 0x0000_4512;
    const MULTI_CONTROLLER: u32 = 0x0000_48a0;
    const QUAD_GROUP: u32 = 0x0001_7002;
    const FRAME_CONTROLLER: u32 = 0x0012_1200;
    const COLLISION_OBJECT: u32 = 0x0701_0000;

    if !matches!(component.id, ANIM_DSG | ANIM_COLL_DSG) {
        return None;
    }
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(chunk, &mut cursor)?;
    let version = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    let has_alpha = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    if version != 0 || cursor != component.header_size {
        return None;
    }

    let direct = chunks
        .iter()
        .filter(|child| child.parent_ordinal == Some(component.ordinal))
        .collect::<Vec<_>>();
    let mut physical_cursor =
        component.offset.checked_add(component.header_size)?;
    let physical_end = component.offset.checked_add(component.total_size)?;
    let mut composites = 0_usize;
    let mut controllers = 0_usize;
    let mut frames = 0_usize;
    let mut collisions = 0_usize;
    let mut children = Vec::with_capacity(direct.len());
    for child in direct {
        if child.offset != physical_cursor
            || !matches!(
                child.id,
                COMPOSITE_DRAWABLE
                    | MULTI_CONTROLLER
                    | QUAD_GROUP
                    | FRAME_CONTROLLER
                    | COLLISION_OBJECT
            )
        {
            return None;
        }
        if child.id == COLLISION_OBJECT && component.id != ANIM_COLL_DSG {
            return None;
        }
        physical_cursor = physical_cursor.checked_add(child.total_size)?;
        composites += usize::from(child.id == COMPOSITE_DRAWABLE);
        controllers += usize::from(child.id == MULTI_CONTROLLER);
        frames += usize::from(child.id == FRAME_CONTROLLER);
        collisions += usize::from(child.id == COLLISION_OBJECT);
        let child_chunk = raw_component_bytes(child, source).ok()?;
        let mut name_cursor = 12;
        let child_name = read_pascal_at(child_chunk, &mut name_cursor)?;
        children.push(format!(
            concat!(
                "{{\"source_ordinal\":{},\"kind\":\"{}\",",
                "\"name\":\"{}\"}}"
            ),
            child.ordinal,
            escape_json(child.kind.label()),
            escape_json(&child_name)
        ));
    }
    let expected_collisions = usize::from(component.id == ANIM_COLL_DSG);
    if physical_cursor != physical_end
        || composites != 1
        || controllers != 1
        || frames == 0
        || collisions != expected_collisions
    {
        return None;
    }

    let kind = component.kind.label();
    let file_name = fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            "{{\"schema\":\"{}\",\"name\":\"{}\",",
            "\"version\":{},\"has_alpha\":{},\"children\":[{}]}}\n"
        ),
        escape_json(kind_schema(kind)),
        escape_json(&name),
        version,
        has_alpha,
        children.join(",")
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover road segment json.
pub(super) fn recover_road_segment_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(chunk, &mut cursor)?;
    let road_type = read_u32(chunk, cursor)?;
    cursor += 4;
    let lanes = read_u32(chunk, cursor)?;
    cursor += 4;
    let shoulder = read_u32(chunk, cursor)?;
    cursor += 4;
    let direction = read_point(chunk, &mut cursor)?;
    let top = read_point(chunk, &mut cursor)?;
    let bottom = read_point(chunk, &mut cursor)?;
    if cursor != component.header_size
        || component.header_size != component.total_size
        || direction
            .iter()
            .chain(&top)
            .chain(&bottom)
            .any(|value| !value.is_finite())
    {
        return None;
    }
    let kind = component.kind.label();
    let file_name = fallback_name(kind, kind_index, &name);
    let json = format!(
        "{{\"schema\":\"road_segment_data\",\"name\":\"{}\",\"type\":{},\"\
         num_lanes\":{},\"has_shoulder\":{},\"direction\":[{},{},{}],\"top\":\
         [{},{},{}],\"bottom\":[{},{},{}]}}\n",
        escape_json(&name),
        road_type,
        lanes,
        shoulder,
        direction[0],
        direction[1],
        direction[2],
        top[0],
        top[1],
        top[2],
        bottom[0],
        bottom[1],
        bottom[2]
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Decode direct road-segment children without assigning runtime semantics.
fn road_segments_json(
    chunk: &[u8],
    mut cursor: usize,
    end: usize,
) -> Option<Vec<String>> {
    const ROAD_SEGMENT: u32 = 0x0300_0002;
    let mut segments = Vec::new();
    while cursor < end {
        let (id, header_size, total_size) = read_chunk_header(chunk, cursor)?;
        if id != ROAD_SEGMENT || header_size != total_size {
            return None;
        }
        let next = cursor.checked_add(total_size)?;
        if next > end {
            return None;
        }
        let mut field_cursor = cursor.checked_add(12)?;
        let name = read_pascal_at(chunk, &mut field_cursor)?;
        let data_name = read_pascal_at(chunk, &mut field_cursor)?;
        let hierarchy = read_finite_matrix(chunk, &mut field_cursor)?;
        let scale = read_finite_matrix(chunk, &mut field_cursor)?;
        if field_cursor != cursor.checked_add(header_size)? {
            return None;
        }
        segments.push(format!(
            concat!(
                "{{\"name\":\"{}\",",
                "\"road_segment_data\":\"{}\",",
                "\"hierarchy_matrix\":{},",
                "\"scale_matrix\":{}}}"
            ),
            escape_json(&name),
            escape_json(&data_name),
            render::matrix_json(&hierarchy),
            render::matrix_json(&scale)
        ));
        cursor = next;
    }
    (cursor == end).then_some(segments)
}

/// Read one finite source matrix for road-segment evidence.
fn read_finite_matrix(
    chunk: &[u8],
    cursor: &mut usize,
) -> Option<[[f32; 4]; 4]> {
    let mut matrix = [[0_f32; 4]; 4];
    for row in &mut matrix {
        for value in row {
            *value = read_f32(chunk, *cursor)?;
            if !value.is_finite() {
                return None;
            }
            *cursor = (*cursor).checked_add(4)?;
        }
    }
    Some(matrix)
}

/// Recover road json.
pub(super) fn recover_road_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(chunk, &mut cursor)?;
    let road_type = read_u32(chunk, cursor)?;
    cursor += 4;
    let start = read_pascal_at(chunk, &mut cursor)?;
    let end = read_pascal_at(chunk, &mut cursor)?;
    let density = read_u32(chunk, cursor)?;
    cursor += 4;
    let speed = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    if cursor != component.header_size {
        return None;
    }
    let segments =
        road_segments_json(chunk, component.header_size, component.total_size)?;
    let kind = component.kind.label();
    let file_name = fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            "{{\"schema\":\"road\",\"name\":\"{}\",",
            "\"type\":{},",
            "\"start_intersection\":\"{}\",",
            "\"end_intersection\":\"{}\",",
            "\"density\":{},\"speed\":{},",
            "\"num_segments\":{},\"segments\":[{}]}}\n"
        ),
        escape_json(&name),
        road_type,
        escape_json(&start),
        escape_json(&end),
        density,
        speed,
        segments.len(),
        segments.join(",")
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover intersection json.
pub(super) fn recover_intersection_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let name = read_pascal_at(chunk, &mut cursor)?;
    let centre = read_point(chunk, &mut cursor)?;
    let radius = read_f32(chunk, cursor)?;
    cursor += 4;
    let intersection_type = read_u32(chunk, cursor)?;
    cursor = cursor.checked_add(4)?;
    if cursor != component.header_size
        || component.header_size != component.total_size
        || centre.iter().any(|value| !value.is_finite())
        || !radius.is_finite()
    {
        return None;
    }
    let kind = component.kind.label();
    let file_name = fallback_name(kind, kind_index, &name);
    let json = format!(
        concat!(
            "{{\"schema\":\"intersection\",",
            "\"name\":\"{}\",",
            "\"centre\":[{},{},{}],",
            "\"radius\":{},",
            "\"type\":{}}}\n",
        ),
        escape_json(&name),
        centre[0],
        centre[1],
        centre[2],
        radius,
        intersection_type
    );
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover one spatial tree with exact physical node relationships.
pub(super) fn recover_tree_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
    chunks: &[ChunkRecord],
) -> Option<RecoveredComponent> {
    const TREE_DSG: u32 = 0x03f0_0004;
    const CONTIGUOUS_BIN_NODE: u32 = 0x03f0_0005;
    const SPATIAL_NODE: u32 = 0x03f0_0006;

    if component.id != TREE_DSG || component.header_size != 40 {
        return None;
    }
    let chunk = raw_component_bytes(component, source).ok()?;
    let mut cursor = 12;
    let declared_nodes = usize::try_from(read_u32(chunk, cursor)?).ok()?;
    cursor = cursor.checked_add(4)?;
    let bounds_min = read_point(chunk, &mut cursor)?;
    let bounds_max = read_point(chunk, &mut cursor)?;
    if cursor != component.header_size
        || bounds_min
            .iter()
            .chain(bounds_max.iter())
            .any(|value| !value.is_finite())
    {
        return None;
    }

    let direct = chunks
        .iter()
        .filter(|child| child.parent_ordinal == Some(component.ordinal))
        .collect::<Vec<_>>();
    if direct.len() != declared_nodes {
        return None;
    }
    let mut physical_cursor =
        component.offset.checked_add(component.header_size)?;
    let physical_end = component.offset.checked_add(component.total_size)?;
    let mut nodes = Vec::with_capacity(direct.len());
    for bin in direct {
        if bin.id != CONTIGUOUS_BIN_NODE
            || bin.offset != physical_cursor
            || bin.header_size != 20
        {
            return None;
        }
        physical_cursor = physical_cursor.checked_add(bin.total_size)?;
        let bin_chunk = raw_component_bytes(bin, source).ok()?;
        let subtree_size = read_u32(bin_chunk, 12)?;
        let parent_offset = read_u32(bin_chunk, 16)?;
        let spatial_children = chunks
            .iter()
            .filter(|child| child.parent_ordinal == Some(bin.ordinal))
            .collect::<Vec<_>>();
        if spatial_children.len() != 1 {
            return None;
        }
        let spatial = spatial_children.first()?;
        if spatial.id != SPATIAL_NODE
            || spatial.offset != bin.offset.checked_add(bin.header_size)?
            || spatial.header_size != 49
            || spatial.total_size != 49
            || bin.total_size != 69
        {
            return None;
        }
        let spatial_chunk = raw_component_bytes(spatial, source).ok()?;
        let plane_axis = *spatial_chunk.get(12)?;
        let plane_position = read_f32(spatial_chunk, 13)?;
        if !plane_position.is_finite() {
            return None;
        }
        let mut count_cursor = 17_usize;
        let mut counts = [0_u32; 8];
        for count in &mut counts {
            *count = read_u32(spatial_chunk, count_cursor)?;
            count_cursor = count_cursor.checked_add(4)?;
        }
        if count_cursor != spatial.header_size {
            return None;
        }
        nodes.push(format!(
            concat!(
                "{{\"source_ordinal\":{},\"spatial_source_ordinal\":{},",
                "\"subtree_size\":{},\"parent_offset\":{},",
                "\"plane_axis\":{},\"plane_position\":{},",
                "\"counts\":[{},{},{},{},{},{},{},{}]}}"
            ),
            bin.ordinal,
            spatial.ordinal,
            subtree_size,
            parent_offset,
            plane_axis,
            plane_position,
            counts[0],
            counts[1],
            counts[2],
            counts[3],
            counts[4],
            counts[5],
            counts[6],
            counts[7]
        ));
    }
    if physical_cursor != physical_end {
        return None;
    }

    let kind = component.kind.label();
    let name = format!("{kind}_{kind_index:04}");
    let json = format!(
        concat!(
            "{{\"schema\":\"tree_dsg\",\"name\":\"{}\",",
            "\"num_nodes\":{},\"bounds_min\":[{},{},{}],",
            "\"bounds_max\":[{},{},{}],\"nodes\":[{}]}}\n"
        ),
        escape_json(&name),
        declared_nodes,
        bounds_min[0],
        bounds_min[1],
        bounds_min[2],
        bounds_max[0],
        bounds_max[1],
        bounds_max[2],
        nodes.join(",")
    );
    Some(render::json_component(
        kind,
        &name,
        name.clone(),
        json,
        "decoded_schema_payload",
    ))
}

/// Recover intersect json.
pub(super) fn recover_intersect_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let kind = component.kind.label();
    let name = format!("{kind}_{kind_index:04}");
    let json = crate::adapters::driven::decoders::intersect::dsg_json(chunk)?;
    Some(render::json_component(
        kind,
        &name,
        name.clone(),
        json,
        "decoded_schema_payload",
    ))
}

/// Recover simulation collision object json.
pub(super) fn recover_collision_object_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_collision_decoded_json(
        component,
        source,
        kind_index,
        crate::adapters::driven::decoders::collision::object_json,
    )
}

/// Recover simulation physics object json.
pub(super) fn recover_physics_object_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_collision_decoded_json(
        component,
        source,
        kind_index,
        crate::adapters::driven::decoders::collision::physics_json,
    )
}

/// Recover physics DSG wrapper json.
pub(super) fn recover_physics_dsg_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let kind = component.kind.label();
    let name = component_name(component, source, kind_index);
    let file_name = fallback_name(kind, kind_index, &name);
    let json = crate::adapters::driven::decoders::collision::dsg_json(
        chunk,
        kind_schema(kind),
    )?;
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Recover chunk-set json.
pub(super) fn recover_chunk_set_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
) -> Option<RecoveredComponent> {
    recover_collision_decoded_json(
        component,
        source,
        kind_index,
        crate::adapters::driven::decoders::collision::chunk_set_json,
    )
}

/// Recover collision-family decoded json.
pub(super) fn recover_collision_decoded_json(
    component: &ChunkRecord,
    source: &[u8],
    kind_index: usize,
    decoder: fn(&[u8]) -> Option<String>,
) -> Option<RecoveredComponent> {
    let chunk = raw_component_bytes(component, source).ok()?;
    let kind = component.kind.label();
    let name = component_name(component, source, kind_index);
    let file_name = fallback_name(kind, kind_index, &name);
    let json = decoder(chunk)?;
    Some(render::json_component(
        kind,
        &file_name,
        name,
        json,
        "decoded_schema_payload",
    ))
}

/// Fallback name.
pub(super) fn fallback_name(
    kind: &str,
    kind_index: usize,
    name: &str,
) -> String {
    if name.is_empty() {
        format!("{kind}_{kind_index:04}")
    } else {
        sanitize(name)
    }
}

/// Build a fallback component name from the kind and per-kind index.
pub(super) fn fallback_component_name(
    component: &ChunkRecord,
    kind_index: usize,
) -> String {
    format!("{}_{kind_index:04}", component.kind.label())
}
