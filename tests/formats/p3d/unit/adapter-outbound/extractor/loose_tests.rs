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
//   - Extractor loose unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Extractor loose unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Assertions fail explicitly.
//

//! Extractor loose unit tests.

use super::*;

#[test]
fn extractor_u32_reader_rejects_offset_overflow() -> Result<(), String> {
    if read_u32(&[], usize::MAX).is_some() {
        return Err(String::from(
            "extractor u32 reads must reject an offset that cannot \
                 contain four bytes",
        ));
    }
    Ok(())
}

#[test]
fn extractor_f32_reader_rejects_offset_overflow() -> Result<(), String> {
    if schema::read_f32(&[], usize::MAX).is_some() {
        return Err(String::from(
            "extractor f32 reads must reject an offset that cannot \
                 contain four bytes",
        ));
    }
    Ok(())
}

#[test]
fn extractor_fourcc_reader_rejects_offset_overflow() -> Result<(), String> {
    if render::read_fourcc(&[], usize::MAX).is_some() {
        return Err(String::from(
            "extractor FOURCC reads must reject an offset that cannot \
                 contain four bytes",
        ));
    }
    Ok(())
}

#[test]
fn extractor_fourcc_reader_rejects_invalid_utf8() -> Result<(), String> {
    let value = render::read_fourcc(&[b'A', b'B', b'C', 0xff], 0);
    if value.is_some() {
        return Err(String::from(
            "FOURCC reads must reject invalid UTF-8 without replacement",
        ));
    }
    Ok(())
}

#[test]
fn extractor_u16_reader_rejects_offset_overflow() -> Result<(), String> {
    if auxiliary::read_u16(&[], usize::MAX).is_some() {
        return Err(String::from(
            "extractor u16 reads must reject an offset that cannot \
                 contain two bytes",
        ));
    }
    Ok(())
}

#[test]
fn truncated_pascal_read_preserves_cursor() -> Result<(), String> {
    let mut cursor = 0_usize;
    let value = schema::read_pascal_at(&[4, b'a'], &mut cursor);
    if value.is_some() {
        return Err(String::from("truncated Pascal strings must fail"));
    }
    if cursor != 0 {
        return Err(String::from(
            "failed Pascal string reads must preserve the caller cursor",
        ));
    }
    Ok(())
}

#[test]
fn invalid_utf8_pascal_read_preserves_cursor() -> Result<(), String> {
    let mut cursor = 0_usize;
    let value = schema::read_pascal_at(&[1, 0xff], &mut cursor);
    if value.is_some() {
        return Err(String::from("invalid UTF-8 Pascal strings must fail"));
    }
    if cursor != 0 {
        return Err(String::from(
            "invalid UTF-8 Pascal reads must preserve the caller cursor",
        ));
    }
    Ok(())
}

#[test]
fn pascal_read_preserves_significant_whitespace() -> Result<(), String> {
    let mut cursor = 0_usize;
    let value = schema::read_pascal_at(&[3, b' ', b'a', b' '], &mut cursor)
        .ok_or_else(|| String::from("valid Pascal string should decode"))?;
    if value != " a " {
        return Err(String::from(
            "Pascal reads must preserve significant edge whitespace",
        ));
    }
    Ok(())
}

#[test]
fn pascal_read_preserves_declared_null_data() -> Result<(), String> {
    let mut cursor = 0_usize;
    let value = schema::read_pascal_at(&[2, b'a', 0], &mut cursor)
        .ok_or_else(|| String::from("valid Pascal string should decode"))?;
    if value != "a\0" {
        return Err(String::from(
            "Pascal reads must preserve declared trailing null data",
        ));
    }
    Ok(())
}

#[test]
fn pascal_component_name_preserves_edge_spaces() -> Result<(), String> {
    let component = ChunkRecord {
        ordinal: 0,
        depth: 0,
        parent_ordinal: None,
        id: 0,
        kind: crate::ChunkKind::Unknown,
        offset: 0,
        header_size: 16,
        total_size: 16,
        payload_offset: 16,
        payload_size: 0,
        child_count: 0,
    };
    let mut source = vec![0_u8; 12];
    source.extend_from_slice(&[3, b' ', b'a', b' ']);
    let name = read_pascal_name(&component, &source)
        .ok_or_else(|| String::from("valid component name should decode"))?;
    if name != " a " {
        return Err(String::from(
            "component names must preserve significant edge spaces",
        ));
    }
    Ok(())
}

fn push_u32(bytes: &mut Vec<u8>, value: u32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_f32(bytes: &mut Vec<u8>, value: f32) {
    bytes.extend_from_slice(&value.to_le_bytes());
}

fn push_pascal(bytes: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let length = u8::try_from(value.len()).map_err(|error| {
        format!("fixture string length exceeds u8: {error}")
    })?;
    bytes.push(length);
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

fn srr_locator_fixture(
    declared_triggers: u32,
) -> Result<(Vec<u8>, usize), String> {
    const LOCATOR: u32 = 0x0300_0005;
    let mut fields = Vec::new();
    push_pascal(&mut fields, "locator")?;
    push_u32(&mut fields, 2);
    push_u32(&mut fields, 0);
    for value in [0_f32, 0., 0.] {
        push_f32(&mut fields, value);
    }
    push_u32(&mut fields, declared_triggers);
    let header_size = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("locator fixture overflowed"))?;
    let header_u32 =
        u32::try_from(header_size).map_err(|error| error.to_string())?;
    let mut source = Vec::new();
    push_u32(&mut source, LOCATOR);
    push_u32(&mut source, header_u32);
    push_u32(&mut source, header_u32);
    source.extend_from_slice(&fields);
    Ok((source, header_size))
}

#[test]
fn locator_spline_preserves_control_points_and_rail() -> Result<(), String> {
    const SPLINE: u32 = 0x0300_0007;
    const RAIL: u32 = 0x0300_000a;
    let mut spline_fields = Vec::new();
    push_pascal(&mut spline_fields, "path")?;
    push_u32(&mut spline_fields, 2);
    for value in [1_f32, 2., 3., 4., 5., 6.] {
        push_f32(&mut spline_fields, value);
    }
    let spline_header = 12_usize
        .checked_add(spline_fields.len())
        .ok_or_else(|| String::from("spline fixture overflowed"))?;

    let mut rail_fields = Vec::new();
    push_pascal(&mut rail_fields, "rail")?;
    push_u32(&mut rail_fields, 3);
    for value in [1_f32, 5.] {
        push_f32(&mut rail_fields, value);
    }
    push_u32(&mut rail_fields, 1);
    push_f32(&mut rail_fields, 2.5);
    push_u32(&mut rail_fields, 0);
    push_f32(&mut rail_fields, 1.25);
    for value in [7_f32, 8., 9., 0.1, 0.2, 0.3, 0.4, 0.5] {
        push_f32(&mut rail_fields, value);
    }
    let rail_size = 12_usize
        .checked_add(rail_fields.len())
        .ok_or_else(|| String::from("rail fixture overflowed"))?;
    let spline_total = spline_header
        .checked_add(rail_size)
        .ok_or_else(|| String::from("spline total overflowed"))?;

    let mut source = Vec::new();
    push_u32(&mut source, SPLINE);
    push_u32(
        &mut source,
        u32::try_from(spline_header).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(spline_total).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&spline_fields);
    push_u32(&mut source, RAIL);
    let rail_u32 =
        u32::try_from(rail_size).map_err(|error| error.to_string())?;
    push_u32(&mut source, rail_u32);
    push_u32(&mut source, rail_u32);
    source.extend_from_slice(&rail_fields);

    let json = render::locator_splines_json(&source, 0, source.len())
        .ok_or_else(|| String::from("spline fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_str(&format!("[{json}]"))
        .map_err(|error| error.to_string())?;
    if value[0]["num_control_points"] != 2
        || value[0]["control_points"][1] != serde_json::json!([4, 5, 6])
        || value[0]["rail"]["behaviour"] != 3
        || value[0]["rail"]["target_offset"] != serde_json::json!([7, 8, 9])
    {
        return Err(String::from(
            "locator spline or rail evidence was discarded",
        ));
    }
    Ok(())
}

#[test]
fn srr_locator_preserves_extra_matrix_children() -> Result<(), String> {
    const EXTRA_MATRIX: u32 = 0x0300_000c;
    let (mut source, header_size) = srr_locator_fixture(0)?;
    let mut payload = Vec::new();
    for value in [
        1_f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 2., 3., 4., 1.,
    ] {
        push_f32(&mut payload, value);
    }
    let child_size = 12_usize
        .checked_add(payload.len())
        .ok_or_else(|| String::from("extra-matrix fixture overflowed"))?;
    let child_u32 =
        u32::try_from(child_size).map_err(|error| error.to_string())?;
    push_u32(&mut source, EXTRA_MATRIX);
    push_u32(&mut source, child_u32);
    push_u32(&mut source, child_u32);
    source.extend_from_slice(&payload);
    let total_u32 =
        u32::try_from(source.len()).map_err(|error| error.to_string())?;
    source[8..12].copy_from_slice(&total_u32.to_le_bytes());
    let component = ChunkRecord {
        ordinal: 7,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_0005,
        kind: crate::ChunkKind::SrrLocator,
        offset: 0,
        header_size,
        total_size: source.len(),
        payload_offset: header_size,
        payload_size: child_size,
        child_count: 1,
    };
    let recovered = render::recover_srr_locator_json(&component, &source, 1)
        .ok_or_else(|| {
            String::from("locator with extra matrix should decode")
        })?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["extra_matrices"][0][3] != serde_json::json!([2, 3, 4, 1]) {
        return Err(String::from("locator extra matrix was discarded"));
    }
    Ok(())
}

#[test]
fn srr_locator_rejects_declared_trigger_count_drift() -> Result<(), String> {
    let (source, header_size) = srr_locator_fixture(1)?;
    let component = ChunkRecord {
        ordinal: 7,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_0005,
        kind: crate::ChunkKind::SrrLocator,
        offset: 0,
        header_size,
        total_size: source.len(),
        payload_offset: header_size,
        payload_size: 0,
        child_count: 0,
    };
    if render::recover_srr_locator_json(&component, &source, 1).is_some() {
        return Err(String::from(
            "locator recovery replaced the authored trigger count",
        ));
    }
    Ok(())
}

fn billboard_quad_fixture(
    display_version: u32,
) -> Result<(Vec<u8>, usize), String> {
    const QUAD: u32 = 0x0001_7001;
    const DISPLAY_INFO: u32 = 0x0001_7003;
    const PERSPECTIVE_INFO: u32 = 0x0001_7004;

    let mut fields = Vec::new();
    push_u32(&mut fields, 2);
    push_pascal(&mut fields, "quad")?;
    fields.extend_from_slice(b"NOAX");
    for value in [0_f32, 0., 0.] {
        push_f32(&mut fields, value);
    }
    push_u32(&mut fields, u32::MAX);
    for value in [0_f32, 0., 1., 0., 1., 1., 0., 1.] {
        push_f32(&mut fields, value);
    }
    for value in [1_f32, 1., 0., 0., 0.] {
        push_f32(&mut fields, value);
    }
    let header_size = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("billboard-quad header overflowed"))?;

    let mut display = Vec::new();
    push_u32(&mut display, display_version);
    for value in [1_f32, 0., 0., 0.] {
        push_f32(&mut display, value);
    }
    display.extend_from_slice(b"N0NE");
    for value in [0_f32, 0., 0., 0.] {
        push_f32(&mut display, value);
    }
    let display_size = 12_usize
        .checked_add(display.len())
        .ok_or_else(|| String::from("billboard-display fixture overflowed"))?;

    let mut perspective = Vec::new();
    push_u32(&mut perspective, 0);
    push_u32(&mut perspective, 1);
    let perspective_size = 12_usize
        .checked_add(perspective.len())
        .ok_or_else(|| String::from("billboard perspective overflow"))?;

    let total_size = header_size
        .checked_add(display_size)
        .and_then(|value| value.checked_add(perspective_size))
        .ok_or_else(|| String::from("billboard-quad fixture overflowed"))?;
    let mut source = Vec::new();
    push_u32(&mut source, QUAD);
    push_u32(
        &mut source,
        u32::try_from(header_size).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(total_size).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&fields);
    push_u32(&mut source, DISPLAY_INFO);
    push_u32(
        &mut source,
        u32::try_from(display_size).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(display_size).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&display);
    push_u32(&mut source, PERSPECTIVE_INFO);
    push_u32(
        &mut source,
        u32::try_from(perspective_size).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(perspective_size).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&perspective);
    Ok((source, header_size))
}

#[test]
fn billboard_quad_preserves_child_schema_versions() -> Result<(), String> {
    let (source, header_size) = billboard_quad_fixture(1)?;
    let json =
        auxiliary::billboard_quad_json(&source, header_size, source.len())
            .ok_or_else(|| {
                String::from("billboard quad fixture should decode")
            })?;
    let value: serde_json::Value =
        serde_json::from_str(&json).map_err(|error| error.to_string())?;
    if value.get("display_info_version") != Some(&serde_json::json!(1))
        || value.get("perspective_info_version") != Some(&serde_json::json!(0))
    {
        return Err(String::from(
            "billboard child schema versions were not preserved",
        ));
    }
    Ok(())
}

#[test]
fn billboard_quad_rejects_missing_presentation_children() -> Result<(), String>
{
    let (source, header_size) = billboard_quad_fixture(1)?;
    let mut header_only = source[..header_size].to_vec();
    header_only[8..12].copy_from_slice(
        &u32::try_from(header_size)
            .map_err(|error| error.to_string())?
            .to_le_bytes(),
    );
    if auxiliary::billboard_quad_json(
        &header_only,
        header_size,
        header_only.len(),
    )
    .is_some()
    {
        return Err(String::from(
            "billboard quad synthesized both presentation children",
        ));
    }

    let display_total = u32::from_le_bytes([
        source[header_size + 8],
        source[header_size + 9],
        source[header_size + 10],
        source[header_size + 11],
    ]) as usize;
    let display_end = header_size
        .checked_add(display_total)
        .ok_or_else(|| String::from("billboard display end overflow"))?;
    let mut display_only = source[..display_end].to_vec();
    display_only[8..12].copy_from_slice(
        &u32::try_from(display_end)
            .map_err(|error| error.to_string())?
            .to_le_bytes(),
    );
    if auxiliary::billboard_quad_json(
        &display_only,
        header_size,
        display_only.len(),
    )
    .is_some()
    {
        return Err(String::from(
            "billboard quad synthesized a missing perspective child",
        ));
    }
    Ok(())
}

#[test]
fn billboard_quad_rejects_unobserved_child_versions() -> Result<(), String> {
    let (source, header_size) = billboard_quad_fixture(2)?;
    if auxiliary::billboard_quad_json(&source, header_size, source.len())
        .is_some()
    {
        return Err(String::from(
            "billboard quad accepted an unobserved display-info version",
        ));
    }

    let (mut source, header_size) = billboard_quad_fixture(1)?;
    source[12..16].copy_from_slice(&3_u32.to_le_bytes());
    if auxiliary::billboard_quad_json(&source, header_size, source.len())
        .is_some()
    {
        return Err(String::from(
            "billboard quad accepted an unobserved quad version",
        ));
    }

    let (mut source, header_size) = billboard_quad_fixture(1)?;
    let display_total = u32::from_le_bytes([
        source[header_size + 8],
        source[header_size + 9],
        source[header_size + 10],
        source[header_size + 11],
    ]) as usize;
    let perspective = header_size + display_total;
    source[perspective + 12..perspective + 16]
        .copy_from_slice(&1_u32.to_le_bytes());
    if auxiliary::billboard_quad_json(&source, header_size, source.len())
        .is_some()
    {
        return Err(String::from(
            "billboard quad accepted an unobserved perspective-info version",
        ));
    }
    Ok(())
}

fn billboard_group_fixture(declared_quads: u32) -> Result<Vec<u8>, String> {
    const QUAD_GROUP: u32 = 0x0001_7002;
    let mut fields = Vec::new();
    push_u32(&mut fields, 0);
    push_pascal(&mut fields, "group")?;
    push_pascal(&mut fields, "shader")?;
    for value in [1, 0, 0, declared_quads] {
        push_u32(&mut fields, value);
    }
    let total = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("billboard-group fixture overflowed"))?;
    let total = u32::try_from(total).map_err(|error| {
        format!("billboard-group fixture exceeds u32: {error}")
    })?;
    let mut source = Vec::new();
    push_u32(&mut source, QUAD_GROUP);
    push_u32(&mut source, total);
    push_u32(&mut source, total);
    source.extend_from_slice(&fields);
    Ok(source)
}

#[test]
fn billboard_group_rejects_declared_quad_count_drift() -> Result<(), String> {
    let source = billboard_group_fixture(1)?;
    let component = ChunkRecord {
        ordinal: 7,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0001_7002,
        kind: crate::ChunkKind::QuadGroup,
        offset: 0,
        header_size: source.len(),
        total_size: source.len(),
        payload_offset: source.len(),
        payload_size: 0,
        child_count: 0,
    };
    if auxiliary::recover_quad_group_json(&component, &source, 1).is_some() {
        return Err(String::from(
            "billboard recovery replaced the authored quad count",
        ));
    }
    Ok(())
}

fn primitive_group_mesh_fixture() -> Result<(Vec<u8>, usize, usize), String> {
    primitive_group_mesh_fixture_with_contract(0, 1, 0)
}

fn primitive_group_mesh_fixture_with_version(
    version: u32,
) -> Result<(Vec<u8>, usize, usize), String> {
    primitive_group_mesh_fixture_with_contract(version, 1, 0)
}

fn primitive_group_mesh_fixture_with_contract(
    primitive_version: u32,
    declared_groups: u32,
    matrix_count: u32,
) -> Result<(Vec<u8>, usize, usize), String> {
    primitive_group_mesh_fixture_with_lists(
        primitive_version,
        declared_groups,
        matrix_count,
        1,
        0,
        0,
    )
}

fn primitive_group_mesh_fixture_with_lists(
    primitive_version: u32,
    declared_groups: u32,
    matrix_count: u32,
    position_copies: usize,
    index_count: u32,
    index_copies: usize,
) -> Result<(Vec<u8>, usize, usize), String> {
    const MESH: u32 = 0x0001_0000;
    const PRIMITIVE_GROUP: u32 = 0x0001_0002;
    const POSITION_LIST: u32 = 0x0001_0005;
    const INDEX_LIST: u32 = 0x0001_000a;

    let mut group_fields = Vec::new();
    push_u32(&mut group_fields, primitive_version);
    push_pascal(&mut group_fields, "shader")?;
    for value in [0, 0, 1, index_count, matrix_count] {
        push_u32(&mut group_fields, value);
    }
    let group_header = 12_usize
        .checked_add(group_fields.len())
        .ok_or_else(|| String::from("primitive-group fixture overflowed"))?;

    let mut group_children = Vec::new();
    for _ in 0..position_copies {
        let mut payload = Vec::new();
        push_u32(&mut payload, 1);
        for value in [0_f32, 0., 0.] {
            push_f32(&mut payload, value);
        }
        let size = 12_usize
            .checked_add(payload.len())
            .ok_or_else(|| String::from("position-list fixture overflowed"))?;
        let size = u32::try_from(size).map_err(|error| error.to_string())?;
        push_u32(&mut group_children, POSITION_LIST);
        push_u32(&mut group_children, size);
        push_u32(&mut group_children, size);
        group_children.extend_from_slice(&payload);
    }
    for _ in 0..index_copies {
        let mut payload = Vec::new();
        push_u32(&mut payload, index_count);
        for _ in 0..index_count {
            push_u32(&mut payload, 0);
        }
        let size = 12_usize
            .checked_add(payload.len())
            .ok_or_else(|| String::from("index-list fixture overflowed"))?;
        let size = u32::try_from(size).map_err(|error| error.to_string())?;
        push_u32(&mut group_children, INDEX_LIST);
        push_u32(&mut group_children, size);
        push_u32(&mut group_children, size);
        group_children.extend_from_slice(&payload);
    }
    let group_total = group_header
        .checked_add(group_children.len())
        .ok_or_else(|| String::from("primitive-group total overflowed"))?;
    let group_header_u32 = u32::try_from(group_header).map_err(|error| {
        format!("primitive-group header exceeds u32: {error}")
    })?;
    let group_total_u32 = u32::try_from(group_total).map_err(|error| {
        format!("primitive-group total exceeds u32: {error}")
    })?;

    let mut mesh_fields = Vec::new();
    push_pascal(&mut mesh_fields, "mesh")?;
    push_u32(&mut mesh_fields, 3);
    push_u32(&mut mesh_fields, declared_groups);
    let mesh_header = 12_usize
        .checked_add(mesh_fields.len())
        .ok_or_else(|| String::from("mesh fixture header overflowed"))?;
    let mesh_total = mesh_header
        .checked_add(group_total)
        .ok_or_else(|| String::from("mesh fixture total overflowed"))?;
    let mesh_header_u32 = u32::try_from(mesh_header)
        .map_err(|error| format!("mesh header exceeds u32: {error}"))?;
    let mesh_total_u32 = u32::try_from(mesh_total)
        .map_err(|error| format!("mesh total exceeds u32: {error}"))?;

    let mut source = Vec::new();
    push_u32(&mut source, MESH);
    push_u32(&mut source, mesh_header_u32);
    push_u32(&mut source, mesh_total_u32);
    source.extend_from_slice(&mesh_fields);
    push_u32(&mut source, PRIMITIVE_GROUP);
    push_u32(&mut source, group_header_u32);
    push_u32(&mut source, group_total_u32);
    source.extend_from_slice(&group_fields);
    source.extend_from_slice(&group_children);
    Ok((source, mesh_header, group_header))
}

fn primitive_group_mesh_record(
    source: &[u8],
    mesh_header: usize,
) -> ChunkRecord {
    ChunkRecord {
        ordinal: 7,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0001_0000,
        kind: crate::ChunkKind::Mesh,
        offset: 0,
        header_size: mesh_header,
        total_size: source.len(),
        payload_offset: mesh_header,
        payload_size: source.len().saturating_sub(mesh_header),
        child_count: 1,
    }
}

#[test]
fn mesh_recovery_retains_primitive_group_source_ordinal() -> Result<(), String>
{
    let (source, mesh_header, group_size) = primitive_group_mesh_fixture()?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    let group = ChunkRecord {
        ordinal: 42,
        depth: 2,
        parent_ordinal: Some(component.ordinal),
        id: 0x0001_0002,
        kind: crate::ChunkKind::Unknown,
        offset: mesh_header,
        header_size: group_size,
        total_size: source.len().saturating_sub(mesh_header),
        payload_offset: mesh_header.saturating_add(group_size),
        payload_size: source
            .len()
            .saturating_sub(mesh_header)
            .saturating_sub(group_size),
        child_count: 1,
    };
    let chunks = [component, group];
    let recovered = recover_component_with_chunk_table(
        &component,
        &source,
        1,
        Some(&chunks),
    )
    .map_err(|error| error.to_string())?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["prim_groups"][0]["source_ordinal"] != 42 {
        return Err(String::from(
            "mesh primitive group lost its package-level source ordinal",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_declared_primitive_group_count_drift()
-> Result<(), String> {
    let (source, mesh_header, _group_size) =
        primitive_group_mesh_fixture_with_contract(0, 2, 0)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery replaced the authored primitive-group count",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_preserves_packed_normal_bytes() -> Result<(), String> {
    const PACKED_NORMAL_LIST: u32 = 0x0001_0010;
    let (mut source, mesh_header, group_header) =
        primitive_group_mesh_fixture()?;
    let group_total_offset = mesh_header.saturating_add(8);
    let old_group_total = u32::from_le_bytes(
        source[group_total_offset..group_total_offset + 4]
            .try_into()
            .map_err(|error| format!("group total slice failed: {error}"))?,
    );
    let mut payload = Vec::new();
    push_u32(&mut payload, 1);
    payload.push(0xa5);
    let packed_size = 12_usize
        .checked_add(payload.len())
        .ok_or_else(|| String::from("packed-normal fixture overflowed"))?;
    let packed_size_u32 =
        u32::try_from(packed_size).map_err(|error| error.to_string())?;
    push_u32(&mut source, PACKED_NORMAL_LIST);
    push_u32(&mut source, packed_size_u32);
    push_u32(&mut source, packed_size_u32);
    source.extend_from_slice(&payload);
    let group_total = old_group_total
        .checked_add(packed_size_u32)
        .ok_or_else(|| String::from("group total overflowed"))?;
    source[group_total_offset..group_total_offset + 4]
        .copy_from_slice(&group_total.to_le_bytes());
    let mesh_total =
        u32::try_from(source.len()).map_err(|error| error.to_string())?;
    source[8..12].copy_from_slice(&mesh_total.to_le_bytes());

    let component = primitive_group_mesh_record(&source, mesh_header);
    let recovered = render::recover_mesh_json(&component, &source, 1, None)
        .ok_or_else(|| String::from("packed-normal mesh should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["prim_groups"][0]["packed_normals"] != serde_json::json!([165]) {
        return Err(String::from(
            "packed-normal bytes crossed their child boundary",
        ));
    }
    if group_header
        >= usize::try_from(group_total).map_err(|error| error.to_string())?
    {
        return Err(String::from("packed-normal child was not appended"));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_missing_declared_position_list() -> Result<(), String>
{
    let (source, mesh_header, _group_header) =
        primitive_group_mesh_fixture_with_lists(0, 1, 0, 0, 0, 0)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted a missing declared position list",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_duplicate_position_lists() -> Result<(), String> {
    let (source, mesh_header, _group_header) =
        primitive_group_mesh_fixture_with_lists(0, 1, 0, 2, 0, 0)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted duplicate position lists",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_missing_declared_index_list() -> Result<(), String> {
    let (source, mesh_header, _group_header) =
        primitive_group_mesh_fixture_with_lists(0, 1, 0, 1, 3, 0)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted a missing declared index list",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_duplicate_index_lists() -> Result<(), String> {
    let (source, mesh_header, _group_header) =
        primitive_group_mesh_fixture_with_lists(0, 1, 0, 1, 3, 2)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted duplicate index lists",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_zero_count_index_list() -> Result<(), String> {
    let (source, mesh_header, _group_header) =
        primitive_group_mesh_fixture_with_lists(0, 1, 0, 1, 0, 1)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted an unobserved zero-count index list",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_missing_declared_matrix_palette() -> Result<(), String>
{
    let (source, mesh_header, _group_size) =
        primitive_group_mesh_fixture_with_contract(0, 1, 1)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted a missing declared matrix palette",
        ));
    }
    Ok(())
}

#[test]
fn skin_decoder_rejects_declared_primitive_group_count_drift()
-> Result<(), String> {
    const SKIN: u32 = 0x0001_0001;
    let mut fields = Vec::new();
    push_pascal(&mut fields, "skin")?;
    push_u32(&mut fields, 3);
    push_pascal(&mut fields, "skeleton")?;
    push_u32(&mut fields, 1);
    let total = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("skin fixture size overflowed"))?;
    let total = u32::try_from(total)
        .map_err(|error| format!("skin fixture exceeds u32: {error}"))?;
    let mut source = Vec::new();
    push_u32(&mut source, SKIN);
    push_u32(&mut source, total);
    push_u32(&mut source, total);
    source.extend_from_slice(&fields);
    if crate::adapters::driven::decoders::mesh::skin_json(&source).is_some() {
        return Err(String::from(
            "skin decoder replaced the authored primitive-group count",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_unobserved_primitive_group_version()
-> Result<(), String> {
    let (source, mesh_header, _group_size) =
        primitive_group_mesh_fixture_with_version(1)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted an unobserved primitive-group version",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_missing_primitive_group_provenance()
-> Result<(), String> {
    let (source, mesh_header, _group_size) = primitive_group_mesh_fixture()?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if recover_component_with_chunk_table(&component, &source, 1, Some(&[]))
        .is_ok()
    {
        return Err(String::from(
            "mesh recovery accepted incomplete primitive-group provenance",
        ));
    }
    Ok(())
}

fn texture_font_fixture(
    declared_textures: u32,
    glyph_count: u32,
) -> Result<Vec<u8>, String> {
    const TEXTURE_FONT: u32 = 0x0002_2000;
    const TEXTURE_GLYPH_LIST: u32 = 0x0002_2001;
    const GLYPH_RECORD_BYTES: u32 = 40;

    let mut header = Vec::new();
    push_u32(&mut header, 7);
    push_pascal(&mut header, "fixture-font")?;
    push_pascal(&mut header, "simple")?;
    push_f32(&mut header, 16.);
    push_f32(&mut header, 16.);
    push_f32(&mut header, 18.);
    push_f32(&mut header, 14.);
    push_u32(&mut header, declared_textures);

    let glyph_bytes = glyph_count
        .checked_mul(GLYPH_RECORD_BYTES)
        .ok_or_else(|| String::from("fixture glyph byte count overflowed"))?;
    let glyph_header_size = 16_u32
        .checked_add(glyph_bytes)
        .ok_or_else(|| String::from("fixture glyph chunk size overflowed"))?;
    let font_header_len = 12_usize
        .checked_add(header.len())
        .ok_or_else(|| String::from("fixture font header size overflowed"))?;
    let font_header_size = u32::try_from(font_header_len).map_err(|error| {
        format!("fixture font header size exceeds u32: {error}")
    })?;
    let font_total_size = font_header_size
        .checked_add(glyph_header_size)
        .ok_or_else(|| String::from("fixture font total size overflowed"))?;

    let mut bytes = Vec::new();
    push_u32(&mut bytes, TEXTURE_FONT);
    push_u32(&mut bytes, font_header_size);
    push_u32(&mut bytes, font_total_size);
    bytes.extend_from_slice(&header);
    push_u32(&mut bytes, TEXTURE_GLYPH_LIST);
    push_u32(&mut bytes, glyph_header_size);
    push_u32(&mut bytes, glyph_header_size);
    push_u32(&mut bytes, glyph_count);
    for record in 0..glyph_count {
        for word in 0..10_u32 {
            let value = record
                .checked_mul(100)
                .and_then(|base| base.checked_add(word))
                .ok_or_else(|| String::from("fixture glyph word overflowed"))?;
            push_u32(&mut bytes, value);
        }
    }
    Ok(bytes)
}

fn texture_font_record(source: &[u8]) -> Result<ChunkRecord, String> {
    let raw_header = read_u32(source, 4)
        .ok_or_else(|| String::from("fixture font header is missing"))?;
    let header_size = usize::try_from(raw_header).map_err(|error| {
        format!("fixture font header exceeds usize: {error}")
    })?;
    let payload_size = source
        .len()
        .checked_sub(header_size)
        .ok_or_else(|| String::from("fixture font header exceeds source"))?;
    Ok(ChunkRecord {
        ordinal: 1,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0002_2000,
        kind: crate::ChunkKind::TextureFont,
        offset: 0,
        header_size,
        total_size: source.len(),
        payload_offset: header_size,
        payload_size,
        child_count: 1,
    })
}

#[test]
fn texture_font_recovery_preserves_lossless_glyph_words() -> Result<(), String>
{
    let source = texture_font_fixture(0, 2)?;
    let component = texture_font_record(&source)?;
    let recovered = recover_component(&component, &source, 1)
        .map_err(|error| error.to_string())?;
    let json = String::from_utf8(recovered.bytes)
        .map_err(|error| error.to_string())?;
    if !json.contains(r#""glyph_count":2"#)
        || !json.contains(r#""glyph_record_stride_bytes":40"#)
        || !json.contains(
                        // jig-ignore-next-line: literal
            r#""glyph_records_u32":[[0,1,2,3,4,5,6,7,8,9],[100,101,102,103,104,105,106,107,108,109]]"#,
        )
    {
        return Err(String::from(
            "texture-font recovery did not preserve exact glyph words",
        ));
    }
    Ok(())
}

#[test]
fn texture_font_recovery_rejects_declared_texture_mismatch()
-> Result<(), String> {
    let source = texture_font_fixture(1, 1)?;
    let component = texture_font_record(&source)?;
    assert!(recover_component(&component, &source, 1).is_err());
    Ok(())
}

#[test]
fn texture_font_recovery_rejects_glyph_stride_mismatch() -> Result<(), String> {
    let mut source = texture_font_fixture(0, 1)?;
    let component = texture_font_record(&source)?;
    let glyph_count_offset = component
        .header_size
        .checked_add(12)
        .ok_or_else(|| String::from("glyph count offset overflowed"))?;
    let glyph_count_end = glyph_count_offset
        .checked_add(4)
        .ok_or_else(|| String::from("glyph count end overflowed"))?;
    source
        .get_mut(glyph_count_offset..glyph_count_end)
        .ok_or_else(|| String::from("glyph count field is missing"))?
        .copy_from_slice(&2_u32.to_le_bytes());
    assert!(recover_component(&component, &source, 1).is_err());
    Ok(())
}

fn dds_payload_fixture() -> Vec<u8> {
    let mut payload = vec![0_u8; 128];
    payload[..4].copy_from_slice(b"DDS ");
    payload[4..8].copy_from_slice(&124_u32.to_le_bytes());
    payload[12..16].copy_from_slice(&32_u32.to_le_bytes());
    payload[16..20].copy_from_slice(&64_u32.to_le_bytes());
    payload[76..80].copy_from_slice(&32_u32.to_le_bytes());
    payload
}

fn image_fixture(
    payload: &[u8],
    declared_payload_size: usize,
) -> Result<Vec<u8>, String> {
    const IMAGE: u32 = 0x0001_9001;
    const IMAGE_DATA: u32 = 0x0001_9002;
    let mut fields = Vec::new();
    push_pascal(&mut fields, "sprite.png")?;
    for value in [14_000, 64, 32, 32, 0, 1, 10] {
        push_u32(&mut fields, value);
    }
    let image_header = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("fixture image header overflowed"))?;
    let data_total = 16_usize
        .checked_add(payload.len())
        .ok_or_else(|| String::from("fixture image data overflowed"))?;
    let image_total = image_header
        .checked_add(data_total)
        .ok_or_else(|| String::from("fixture image total overflowed"))?;
    let image_header = u32::try_from(image_header).map_err(|error| {
        format!("fixture image header exceeds u32: {error}")
    })?;
    let image_total = u32::try_from(image_total)
        .map_err(|error| format!("fixture image total exceeds u32: {error}"))?;
    let data_total = u32::try_from(data_total)
        .map_err(|error| format!("fixture image data exceeds u32: {error}"))?;
    let declared_payload_size =
        u32::try_from(declared_payload_size).map_err(|error| {
            format!("fixture payload size exceeds u32: {error}")
        })?;

    let mut bytes = Vec::new();
    push_u32(&mut bytes, IMAGE);
    push_u32(&mut bytes, image_header);
    push_u32(&mut bytes, image_total);
    bytes.extend_from_slice(&fields);
    push_u32(&mut bytes, IMAGE_DATA);
    push_u32(&mut bytes, data_total);
    push_u32(&mut bytes, data_total);
    push_u32(&mut bytes, declared_payload_size);
    bytes.extend_from_slice(payload);
    Ok(bytes)
}

fn image_record(source: &[u8]) -> Result<ChunkRecord, String> {
    let header = read_u32(source, 4)
        .ok_or_else(|| String::from("image header missing"))?;
    let header_size = usize::try_from(header)
        .map_err(|error| format!("image header exceeds usize: {error}"))?;
    Ok(ChunkRecord {
        ordinal: 2,
        depth: 2,
        parent_ordinal: Some(1),
        id: 0x0001_9001,
        kind: crate::ChunkKind::Image,
        offset: 0,
        header_size,
        total_size: source.len(),
        payload_offset: header_size,
        payload_size: source.len().saturating_sub(header_size),
        child_count: 1,
    })
}

#[test]
fn embedded_sprite_image_recovery_preserves_exact_dds_payload()
-> Result<(), String> {
    let payload = dds_payload_fixture();
    let source = image_fixture(&payload, payload.len())?;
    let component = image_record(&source)?;
    let recovered = recover_component(&component, &source, 1)
        .map_err(|error| error.to_string())?;
    if recovered.relative_path != Path::new("image/sprite.dds")
        || recovered.payload_format != "image/dds"
        || recovered.recovery_status != "recovered_embedded_image_payload"
        || recovered.bytes != payload
    {
        return Err(
            "embedded sprite DDS recovery changed exact evidence".to_owned()
        );
    }
    Ok(())
}

#[test]
fn embedded_sprite_image_recovery_rejects_oversized_data_claim()
-> Result<(), String> {
    let payload = dds_payload_fixture();
    let declared = payload
        .len()
        .checked_add(1)
        .ok_or_else(|| String::from("fixture payload size overflowed"))?;
    let source = image_fixture(&payload, declared)?;
    let component = image_record(&source)?;
    if recover_component(&component, &source, 1).is_ok() {
        return Err(
            "oversized IMAGE_DATA payload claim was accepted".to_owned()
        );
    }
    Ok(())
}

fn publication_chunk(parent_ordinal: Option<usize>) -> ChunkRecord {
    publication_chunk_at(1, parent_ordinal)
}

fn publication_chunk_at(
    ordinal: usize,
    parent_ordinal: Option<usize>,
) -> ChunkRecord {
    ChunkRecord {
        ordinal,
        depth: 1,
        parent_ordinal,
        id: 0,
        kind: crate::ChunkKind::Mesh,
        offset: 0,
        header_size: 12,
        total_size: 12,
        payload_offset: 12,
        payload_size: 0,
        child_count: 0,
    }
}

fn recovered_publication(path: &str, bytes: &[u8]) -> RecoveredComponent {
    RecoveredComponent {
        name: "fixture".to_owned(),
        relative_path: PathBuf::from(path),
        bytes: bytes.to_vec(),
        payload_format: "schema_json".to_owned(),
        recovery_status: "decoded_schema_payload".to_owned(),
    }
}

#[test]
fn publication_registry_reuses_only_identical_nested_exact_path()
-> Result<(), String> {
    let mut paths = BTreeMap::new();
    let mut first = recovered_publication("mesh/shared.json", b"same");
    let mut nested = recovered_publication("mesh/shared.json", b"same");
    let first_publish = register_recovered_path(
        &mut paths,
        &publication_chunk(Some(0)),
        &mut first,
    )
    .map_err(|error| error.to_string())?;
    let nested_publish = register_recovered_path(
        &mut paths,
        &publication_chunk(Some(1)),
        &mut nested,
    )
    .map_err(|error| error.to_string())?;
    if !first_publish || nested_publish {
        return Err(
            "identical nested exact-path reuse changed publication policy"
                .to_owned(),
        );
    }
    Ok(())
}

#[test]
fn publication_registry_disambiguates_nested_payload_conflict()
-> Result<(), String> {
    let mut paths = BTreeMap::new();
    let mut first = recovered_publication("mesh/shared.json", b"first");
    let mut nested = recovered_publication("mesh/shared.json", b"second");
    let first_publish = register_recovered_path(
        &mut paths,
        &publication_chunk(Some(0)),
        &mut first,
    )
    .map_err(|error| error.to_string())?;
    if !first_publish {
        return Err("first component path claim was skipped".to_owned());
    }
    if !register_recovered_path(
        &mut paths,
        &publication_chunk_at(9, Some(1)),
        &mut nested,
    )
    .map_err(|error| error.to_string())?
    {
        return Err("different nested payload was silently reused".to_owned());
    }
    if nested.relative_path != Path::new("mesh/shared__ordinal_0009.json") {
        return Err("nested payload conflict was not disambiguated".to_owned());
    }
    Ok(())
}

#[test]
fn publication_registry_disambiguates_identical_direct_root_duplicates()
-> Result<(), String> {
    let mut paths = BTreeMap::new();
    let mut first = recovered_publication("mesh/shared.json", b"same");
    let mut second = recovered_publication("mesh/shared.json", b"same");
    if !register_recovered_path(
        &mut paths,
        &publication_chunk_at(1, Some(0)),
        &mut first,
    )
    .map_err(|error| error.to_string())?
    {
        return Err("first component path claim was skipped".to_owned());
    }
    if !register_recovered_path(
        &mut paths,
        &publication_chunk_at(8, Some(0)),
        &mut second,
    )
    .map_err(|error| error.to_string())?
    {
        return Err("identical direct-root duplicate was skipped".to_owned());
    }
    if second.relative_path != Path::new("mesh/shared__ordinal_0008.json") {
        return Err("direct-root duplicate path was not qualified".to_owned());
    }
    if paths.len() != 2 {
        return Err("direct-root duplicate lost source provenance".to_owned());
    }
    Ok(())
}

#[test]
fn publication_registry_disambiguates_identical_case_aliases()
-> Result<(), String> {
    let mut paths = BTreeMap::new();
    let mut first = recovered_publication("mesh/Shared.json", b"same");
    let mut alias = recovered_publication("MESH/shared.json", b"same");
    if !register_recovered_path(
        &mut paths,
        &publication_chunk_at(1, Some(0)),
        &mut first,
    )
    .map_err(|error| error.to_string())?
    {
        return Err("first component path claim was skipped".to_owned());
    }
    if !register_recovered_path(
        &mut paths,
        &publication_chunk_at(27, Some(0)),
        &mut alias,
    )
    .map_err(|error| error.to_string())?
    {
        return Err("case-only source alias was skipped".to_owned());
    }
    if alias.relative_path != Path::new("MESH/shared__ordinal_0027.json") {
        return Err(format!(
            "case alias path was not deterministic: {}",
            alias.relative_path.display()
        ));
    }
    if paths.len() != 2 {
        return Err(
            "case aliases did not retain two provenance rows".to_owned()
        );
    }
    Ok(())
}

#[test]
fn publication_registry_disambiguates_case_alias_payload_conflict()
-> Result<(), String> {
    let mut paths = BTreeMap::new();
    let mut first = recovered_publication("mesh/Shared.json", b"first");
    let mut alias = recovered_publication("MESH/shared.json", b"second");
    let _published = register_recovered_path(
        &mut paths,
        &publication_chunk_at(1, Some(0)),
        &mut first,
    )
    .map_err(|error| error.to_string())?;
    if !register_recovered_path(
        &mut paths,
        &publication_chunk_at(27, Some(0)),
        &mut alias,
    )
    .map_err(|error| error.to_string())?
    {
        return Err("conflicting case alias payload was skipped".to_owned());
    }
    if alias.relative_path != Path::new("MESH/shared__ordinal_0027.json") {
        return Err("conflicting case alias was not disambiguated".to_owned());
    }
    Ok(())
}

fn inst_particle_system_fixture() -> Result<Vec<u8>, String> {
    const INST_PARTICLE_SYSTEM: u32 = 0x0300_1001;
    const SYSTEM_FACTORY: u32 = 0x0001_5800;
    const SYSTEM: u32 = 0x0001_5801;

    let mut factory_header = Vec::new();
    push_u32(&mut factory_header, 0);
    push_pascal(&mut factory_header, "spark")?;
    push_f32(&mut factory_header, 30.);
    push_u32(&mut factory_header, 60);
    push_u32(&mut factory_header, 10);
    factory_header.extend_from_slice(&1_u16.to_le_bytes());
    factory_header.extend_from_slice(&0_u16.to_le_bytes());
    push_u32(&mut factory_header, 0);
    let factory_size = 12_usize
        .checked_add(factory_header.len())
        .ok_or_else(|| String::from("factory fixture size overflowed"))?;
    let factory_size = u32::try_from(factory_size)
        .map_err(|error| format!("factory fixture exceeds u32: {error}"))?;

    let mut system_header = Vec::new();
    push_u32(&mut system_header, 0);
    push_pascal(&mut system_header, "spark")?;
    push_pascal(&mut system_header, "spark")?;
    let system_size = 12_usize
        .checked_add(system_header.len())
        .ok_or_else(|| String::from("system fixture size overflowed"))?;
    let system_size = u32::try_from(system_size)
        .map_err(|error| format!("system fixture exceeds u32: {error}"))?;

    let inst_header_size = 20_u32;
    let inst_total_size = inst_header_size
        .checked_add(factory_size)
        .and_then(|value| value.checked_add(system_size))
        .ok_or_else(|| String::from("inst particle fixture size overflowed"))?;
    let mut bytes = Vec::new();
    push_u32(&mut bytes, INST_PARTICLE_SYSTEM);
    push_u32(&mut bytes, inst_header_size);
    push_u32(&mut bytes, inst_total_size);
    push_u32(&mut bytes, 3);
    push_u32(&mut bytes, 12);
    push_u32(&mut bytes, SYSTEM_FACTORY);
    push_u32(&mut bytes, factory_size);
    push_u32(&mut bytes, factory_size);
    bytes.extend_from_slice(&factory_header);
    push_u32(&mut bytes, SYSTEM);
    push_u32(&mut bytes, system_size);
    push_u32(&mut bytes, system_size);
    bytes.extend_from_slice(&system_header);
    Ok(bytes)
}

fn inst_particle_system_record(source: &[u8]) -> ChunkRecord {
    ChunkRecord {
        ordinal: 1,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_1001,
        kind: crate::ChunkKind::SrrInstParticleSystem,
        offset: 0,
        header_size: 20,
        total_size: source.len(),
        payload_offset: 20,
        payload_size: source.len().saturating_sub(20),
        child_count: 2,
    }
}

#[test]
fn inst_particle_decodes_nested_factory_and_system() -> Result<(), String> {
    let source = inst_particle_system_fixture()?;
    let component = inst_particle_system_record(&source);
    let recovered = recover_component(&component, &source, 1)
        .map_err(|error| error.to_string())?;
    let json: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    let children = json["children"].as_array().ok_or_else(|| {
        String::from("inst particle children must be an array")
    })?;
    if children.len() != 2
        || children[0]["kind"] != "particle_system_factory"
        || children[0]["name"] != "spark"
        || children[0]["version"] != 0
        || children[0]["frame_rate"] != 30.
        || children[0]["num_anim_frames"] != 60
        || children[0]["num_ol_frames"] != 10
        || children[0]["cycle_anim"] != 1
        || children[0]["enable_sorting"] != 0
        || children[0]["num_emitters"] != 0
        || children[1]["kind"] != "particle_system"
        || children[1]["name"] != "spark"
        || children[1]["version"] != 0
        || children[1]["factory_name"] != "spark"
    {
        return Err(format!(
            "nested particle identities were not recovered: {json}"
        ));
    }
    Ok(())
}

#[test]
fn inst_particle_rejects_malformed_nested_system() -> Result<(), String> {
    let mut source = inst_particle_system_fixture()?;
    let factory_size =
        usize::try_from(read_u32(&source, 24).ok_or_else(|| {
            String::from("factory fixture total size is missing")
        })?)
        .map_err(|error| format!("factory size exceeds usize: {error}"))?;
    let system_header = 20_usize
        .checked_add(factory_size)
        .ok_or_else(|| String::from("system fixture offset overflowed"))?;
    let system_name_length = system_header
        .checked_add(16)
        .ok_or_else(|| String::from("system name offset overflowed"))?;
    let system_name_length = source
        .get_mut(system_name_length)
        .ok_or_else(|| String::from("system name length byte is missing"))?;
    *system_name_length = 120;
    let component = inst_particle_system_record(&source);
    if recover_component(&component, &source, 1).is_ok() {
        return Err(String::from(
            "malformed nested particle system header was accepted",
        ));
    }
    Ok(())
}

fn inst_particle_package_fixture() -> Result<Vec<u8>, String> {
    let inst = inst_particle_system_fixture()?;
    let total_size = 12_usize
        .checked_add(inst.len())
        .ok_or_else(|| String::from("root fixture size overflowed"))?;
    let total_size = u32::try_from(total_size)
        .map_err(|error| format!("root fixture exceeds u32: {error}"))?;
    let mut source = Vec::new();
    push_u32(&mut source, 0xff44_3350);
    push_u32(&mut source, 12);
    push_u32(&mut source, total_size);
    source.extend_from_slice(&inst);
    Ok(source)
}

#[test]
fn inst_particle_parser_keeps_nested_resources_parent_local()
-> Result<(), String> {
    let source = inst_particle_package_fixture()?;
    let document = analyze_p3d(&source).map_err(|error| error.to_string())?;
    let inst = document
        .chunks
        .iter()
        .find(|chunk| chunk.kind == crate::ChunkKind::SrrInstParticleSystem)
        .ok_or_else(|| String::from("parsed inst particle chunk is missing"))?;
    let factory = document
        .chunks
        .iter()
        .find(|chunk| chunk.kind == crate::ChunkKind::ParticleSystemFactory)
        .ok_or_else(|| String::from("parsed particle factory is missing"))?;
    let system = document
        .chunks
        .iter()
        .find(|chunk| chunk.kind == crate::ChunkKind::ParticleSystem)
        .ok_or_else(|| String::from("parsed particle system is missing"))?;
    if !should_publish_component(inst, &document.chunks)
        || should_publish_component(factory, &document.chunks)
        || should_publish_component(system, &document.chunks)
    {
        return Err(String::from(
            "nested particle publication escaped the instanced container",
        ));
    }
    let recovered = recover_component(inst, &source, 1)
        .map_err(|error| error.to_string())?;
    let json: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if json["children"][0]["kind"] != "particle_system_factory"
        || json["children"][1]["kind"] != "particle_system"
    {
        return Err(String::from(
            "parent-local particle identities were not retained",
        ));
    }
    Ok(())
}
