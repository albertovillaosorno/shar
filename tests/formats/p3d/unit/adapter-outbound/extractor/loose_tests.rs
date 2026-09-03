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

fn trigger_volume_fixture(
    volume_type: u32,
    scale_x: f32,
    trailing_header_word: bool,
    child_word: bool,
) -> Result<(Vec<u8>, usize, usize), String> {
    const TRIGGER_VOLUME: u32 = 0x0300_0006;
    let mut fields = Vec::new();
    push_pascal(&mut fields, "trigger")?;
    push_u32(&mut fields, volume_type);
    for value in [scale_x, 2_f32, 3_f32] {
        push_f32(&mut fields, value);
    }
    for index in 0_usize..16_usize {
        let value = if matches!(index, 0 | 5 | 10 | 15) {
            1_f32
        } else {
            0_f32
        };
        push_f32(&mut fields, value);
    }
    if trailing_header_word {
        push_u32(&mut fields, 99);
    }
    let header_size = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("trigger header overflowed"))?;
    let total_size = header_size
        .checked_add(if child_word {
            4
        } else {
            0
        })
        .ok_or_else(|| String::from("trigger total overflowed"))?;
    let mut source = Vec::new();
    push_u32(&mut source, TRIGGER_VOLUME);
    push_u32(
        &mut source,
        u32::try_from(header_size).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(total_size).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&fields);
    if child_word {
        push_u32(&mut source, 0);
    }
    Ok((source, header_size, total_size))
}

#[test]
fn trigger_volume_preserves_schema_values() -> Result<(), String> {
    let (source, header_size, total_size) =
        trigger_volume_fixture(1, 4_f32, false, false)?;
    let json = render::trigger_volume_json(&source, 0, header_size, total_size)
        .ok_or_else(|| String::from("trigger volume should decode"))?;
    if json.contains("\"type\":1") && json.contains("\"scale\":[4,2,3]") {
        Ok(())
    } else {
        Err(String::from("trigger volume source values were discarded"))
    }
}

#[test]
fn trigger_volume_rejects_unobserved_source_shapes() -> Result<(), String> {
    let (unknown_type, header_size, total_size) =
        trigger_volume_fixture(2, 1_f32, false, false)?;
    if render::trigger_volume_json(&unknown_type, 0, header_size, total_size)
        .is_some()
    {
        return Err(String::from("unobserved trigger type should fail closed"));
    }
    let (trailing, header_size, total_size) =
        trigger_volume_fixture(0, 1_f32, true, false)?;
    if render::trigger_volume_json(&trailing, 0, header_size, total_size)
        .is_some()
    {
        return Err(String::from("trailing trigger header should fail closed"));
    }
    let (child, header_size, total_size) =
        trigger_volume_fixture(0, 1_f32, false, true)?;
    if render::trigger_volume_json(&child, 0, header_size, total_size).is_some()
    {
        return Err(String::from("trigger child data should fail closed"));
    }
    let (nonfinite, header_size, total_size) =
        trigger_volume_fixture(0, f32::NAN, false, false)?;
    if render::trigger_volume_json(&nonfinite, 0, header_size, total_size)
        .is_some()
    {
        return Err(String::from("nonfinite trigger data should fail closed"));
    }
    Ok(())
}

#[test]
fn srr_locator_rejects_trailing_header_data() -> Result<(), String> {
    let (mut source, header_size) = srr_locator_fixture(0)?;
    push_u32(&mut source, 99);
    let extended_header = header_size
        .checked_add(4)
        .ok_or_else(|| String::from("extended locator header overflowed"))?;
    let size_u32 =
        u32::try_from(extended_header).map_err(|error| error.to_string())?;
    source[4..8].copy_from_slice(&size_u32.to_le_bytes());
    source[8..12].copy_from_slice(&size_u32.to_le_bytes());
    let component = ChunkRecord {
        ordinal: 7,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_0005,
        kind: crate::ChunkKind::SrrLocator,
        offset: 0,
        header_size: extended_header,
        total_size: extended_header,
        payload_offset: extended_header,
        payload_size: 0,
        child_count: 0,
    };
    if render::recover_srr_locator_json(&component, &source, 1).is_none() {
        Ok(())
    } else {
        Err(String::from(
            "trailing locator header data should fail closed",
        ))
    }
}

#[test]
fn srr_locator_rejects_nonfinite_position() -> Result<(), String> {
    let (mut source, header_size) = srr_locator_fixture(0)?;
    let position_offset = 28_usize;
    source[position_offset..position_offset + 4]
        .copy_from_slice(&f32::NAN.to_le_bytes());
    let component = ChunkRecord {
        ordinal: 7,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_0005,
        kind: crate::ChunkKind::SrrLocator,
        offset: 0,
        header_size,
        total_size: header_size,
        payload_offset: header_size,
        payload_size: 0,
        child_count: 0,
    };
    if render::recover_srr_locator_json(&component, &source, 1).is_none() {
        Ok(())
    } else {
        Err(String::from(
            "nonfinite locator position should fail closed",
        ))
    }
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

fn road_with_segment_fixture() -> Result<(Vec<u8>, usize), String> {
    const ROAD: u32 = 0x0300_0003;
    const ROAD_SEGMENT: u32 = 0x0300_0002;
    let mut segment_fields = Vec::new();
    push_pascal(&mut segment_fields, "segment")?;
    push_pascal(&mut segment_fields, "segment_data")?;
    for index in 0_usize..16_usize {
        let value = if matches!(index, 0 | 5 | 10 | 15) {
            1_f32
        } else if index == 12 {
            5_f32
        } else {
            0_f32
        };
        push_f32(&mut segment_fields, value);
    }
    for index in 0_usize..16_usize {
        let value = match index {
            0 => 2_f32,
            5 => 3_f32,
            10 => 4_f32,
            15 => 1_f32,
            _ => 0_f32,
        };
        push_f32(&mut segment_fields, value);
    }
    let segment_size = 12_usize
        .checked_add(segment_fields.len())
        .ok_or_else(|| String::from("road segment fixture overflowed"))?;
    let segment_u32 =
        u32::try_from(segment_size).map_err(|error| error.to_string())?;
    let mut segment = Vec::new();
    push_u32(&mut segment, ROAD_SEGMENT);
    push_u32(&mut segment, segment_u32);
    push_u32(&mut segment, segment_u32);
    segment.extend_from_slice(&segment_fields);

    let mut road_fields = Vec::new();
    push_pascal(&mut road_fields, "road")?;
    push_u32(&mut road_fields, 0);
    push_pascal(&mut road_fields, "start")?;
    push_pascal(&mut road_fields, "end")?;
    push_u32(&mut road_fields, 7);
    push_u32(&mut road_fields, 40);
    let road_header = 12_usize
        .checked_add(road_fields.len())
        .ok_or_else(|| String::from("road fixture header overflowed"))?;
    let road_total = road_header
        .checked_add(segment.len())
        .ok_or_else(|| String::from("road fixture total overflowed"))?;
    let mut source = Vec::new();
    push_u32(&mut source, ROAD);
    push_u32(
        &mut source,
        u32::try_from(road_header).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(road_total).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&road_fields);
    source.extend_from_slice(&segment);
    Ok((source, road_header))
}

fn road_component(header_size: usize, total_size: usize) -> ChunkRecord {
    ChunkRecord {
        ordinal: 3,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_0003,
        kind: crate::ChunkKind::SrrRoad,
        offset: 0,
        header_size,
        total_size,
        payload_offset: header_size,
        payload_size: total_size.saturating_sub(header_size),
        child_count: 1,
    }
}

#[test]
fn road_preserves_segment_source_evidence() -> Result<(), String> {
    let (source, header_size) = road_with_segment_fixture()?;
    let component = road_component(header_size, source.len());
    let recovered = schema::recover_road_json(&component, &source, 1)
        .ok_or_else(|| String::from("road fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["num_segments"] == 1
        && value["segments"][0]["road_segment_data"] == "segment_data"
        && value["segments"][0]["hierarchy_matrix"][3][0] == 5
        && value["segments"][0]["scale_matrix"][0][0] == 2
    {
        Ok(())
    } else {
        Err(String::from("road segment source evidence was discarded"))
    }
}

#[test]
fn road_rejects_malformed_segment_evidence() -> Result<(), String> {
    let (mut source, header_size) = road_with_segment_fixture()?;
    let child_offset = header_size;
    let old_size = read_u32(&source, child_offset + 4)
        .ok_or_else(|| String::from("road segment header should exist"))?;
    let new_size = old_size
        .checked_add(4)
        .ok_or_else(|| String::from("road segment size overflowed"))?;
    source[child_offset + 4..child_offset + 8]
        .copy_from_slice(&new_size.to_le_bytes());
    source[child_offset + 8..child_offset + 12]
        .copy_from_slice(&new_size.to_le_bytes());
    push_u32(&mut source, 99);
    let total_u32 =
        u32::try_from(source.len()).map_err(|error| error.to_string())?;
    source[8..12].copy_from_slice(&total_u32.to_le_bytes());
    let component = road_component(header_size, source.len());
    if schema::recover_road_json(&component, &source, 1).is_none() {
        Ok(())
    } else {
        Err(String::from(
            "trailing road segment data should fail closed",
        ))
    }
}

fn ped_path_fixture(
    points: &[[f32; 3]],
    trailing_word: bool,
) -> Result<(Vec<u8>, usize), String> {
    const PED_PATH: u32 = 0x0300_000b;
    let mut fields = Vec::new();
    push_u32(
        &mut fields,
        u32::try_from(points.len()).map_err(|error| error.to_string())?,
    );
    for point in points {
        for value in point {
            push_f32(&mut fields, *value);
        }
    }
    if trailing_word {
        push_u32(&mut fields, 99);
    }
    let size = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("ped path fixture overflowed"))?;
    let size_u32 = u32::try_from(size).map_err(|error| error.to_string())?;
    let mut source = Vec::new();
    push_u32(&mut source, PED_PATH);
    push_u32(&mut source, size_u32);
    push_u32(&mut source, size_u32);
    source.extend_from_slice(&fields);
    Ok((source, size))
}

fn ped_path_component(size: usize) -> ChunkRecord {
    ChunkRecord {
        ordinal: 4,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_000b,
        kind: crate::ChunkKind::SrrPedPath,
        offset: 0,
        header_size: size,
        total_size: size,
        payload_offset: size,
        payload_size: 0,
        child_count: 0,
    }
}

#[test]
fn ped_path_preserves_declared_points() -> Result<(), String> {
    let (source, size) = ped_path_fixture(
        &[[1_f32, 2_f32, 3_f32], [4_f32, 5_f32, 6_f32]],
        false,
    )?;
    let recovered =
        auxiliary::recover_ped_path_json(&ped_path_component(size), &source, 1)
            .ok_or_else(|| String::from("ped path fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["num_points"] == 2
        && value["points"][1] == serde_json::json!([4, 5, 6])
    {
        Ok(())
    } else {
        Err(String::from("ped path source points were discarded"))
    }
}

#[test]
fn ped_path_rejects_unobserved_source_shapes() -> Result<(), String> {
    let (trailing, size) = ped_path_fixture(&[[1_f32, 2_f32, 3_f32]], true)?;
    if auxiliary::recover_ped_path_json(&ped_path_component(size), &trailing, 1)
        .is_some()
    {
        return Err(String::from("trailing ped path data should fail closed"));
    }
    let (nonfinite, size) =
        ped_path_fixture(&[[f32::NAN, 2_f32, 3_f32]], false)?;
    if auxiliary::recover_ped_path_json(
        &ped_path_component(size),
        &nonfinite,
        1,
    )
    .is_some()
    {
        return Err(String::from("nonfinite ped path data should fail closed"));
    }
    Ok(())
}

fn follow_cam_fixture(
    elevation: f32,
    trailing_word: bool,
) -> Result<(Vec<u8>, usize), String> {
    const FOLLOW_CAM: u32 = 0x0300_0100;
    let mut fields = Vec::new();
    push_u32(&mut fields, 81);
    for value in [180_f32, elevation, 9_f32, 0_f32, 0.5_f32, 1.2_f32] {
        push_f32(&mut fields, value);
    }
    if trailing_word {
        push_u32(&mut fields, 99);
    }
    let size = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("follow cam fixture overflowed"))?;
    let size_u32 = u32::try_from(size).map_err(|error| error.to_string())?;
    let mut source = Vec::new();
    push_u32(&mut source, FOLLOW_CAM);
    push_u32(&mut source, size_u32);
    push_u32(&mut source, size_u32);
    source.extend_from_slice(&fields);
    Ok((source, size))
}

fn follow_cam_component(size: usize) -> ChunkRecord {
    ChunkRecord {
        ordinal: 5,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_0100,
        kind: crate::ChunkKind::SrrFollowCam,
        offset: 0,
        header_size: size,
        total_size: size,
        payload_offset: size,
        payload_size: 0,
        child_count: 0,
    }
}

#[test]
fn follow_cam_preserves_source_values() -> Result<(), String> {
    let (source, size) = follow_cam_fixture(12_f32, false)?;
    let recovered = auxiliary::recover_follow_cam_json(
        &follow_cam_component(size),
        &source,
        1,
    )
    .ok_or_else(|| String::from("follow cam fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["id"] == 81
        && value["elevation"] == 12
        && value["target_offset"] == serde_json::json!([0, 0.5, 1.2])
    {
        Ok(())
    } else {
        Err(String::from("follow cam source values were discarded"))
    }
}

#[test]
fn follow_cam_rejects_unobserved_source_shapes() -> Result<(), String> {
    let (trailing, size) = follow_cam_fixture(12_f32, true)?;
    if auxiliary::recover_follow_cam_json(
        &follow_cam_component(size),
        &trailing,
        1,
    )
    .is_some()
    {
        return Err(String::from(
            "trailing follow cam data should fail closed",
        ));
    }
    let (nonfinite, size) = follow_cam_fixture(f32::NAN, false)?;
    if auxiliary::recover_follow_cam_json(
        &follow_cam_component(size),
        &nonfinite,
        1,
    )
    .is_some()
    {
        return Err(String::from(
            "nonfinite follow cam data should fail closed",
        ));
    }
    Ok(())
}

fn attribute_table_fixture(
    mass: f32,
    trailing_word: bool,
) -> Result<(Vec<u8>, usize), String> {
    const ATTRIBUTE_TABLE: u32 = 0x0300_0602;
    let mut fields = Vec::new();
    push_u32(&mut fields, 1);
    push_pascal(&mut fields, "sound")?;
    push_pascal(&mut fields, "particle")?;
    push_pascal(&mut fields, "animation")?;
    for value in [0.5_f32, mass, 0.25_f32] {
        push_f32(&mut fields, value);
    }
    if trailing_word {
        push_u32(&mut fields, 99);
    }
    let size = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("attribute table fixture overflowed"))?;
    let size_u32 = u32::try_from(size).map_err(|error| error.to_string())?;
    let mut source = Vec::new();
    push_u32(&mut source, ATTRIBUTE_TABLE);
    push_u32(&mut source, size_u32);
    push_u32(&mut source, size_u32);
    source.extend_from_slice(&fields);
    Ok((source, size))
}

fn attribute_table_component(size: usize) -> ChunkRecord {
    ChunkRecord {
        ordinal: 6,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_0602,
        kind: crate::ChunkKind::SrrAttributeTable,
        offset: 0,
        header_size: size,
        total_size: size,
        payload_offset: size,
        payload_size: 0,
        child_count: 0,
    }
}

#[test]
fn attribute_table_preserves_source_rows() -> Result<(), String> {
    let (source, size) = attribute_table_fixture(9999_f32, false)?;
    let recovered = auxiliary::recover_attribute_table_json(
        &attribute_table_component(size),
        &source,
        1,
    )
    .ok_or_else(|| String::from("attribute table fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["num_rows"] == 1
        && value["rows"][0]["sound"] == "sound"
        && value["rows"][0]["mass"] == 9999
    {
        Ok(())
    } else {
        Err(String::from("attribute table source row was discarded"))
    }
}

#[test]
fn attribute_table_rejects_unobserved_source_shapes() -> Result<(), String> {
    let (trailing, size) = attribute_table_fixture(1_f32, true)?;
    if auxiliary::recover_attribute_table_json(
        &attribute_table_component(size),
        &trailing,
        1,
    )
    .is_some()
    {
        return Err(String::from(
            "trailing attribute table data should fail closed",
        ));
    }
    let (nonfinite, size) = attribute_table_fixture(f32::NAN, false)?;
    if auxiliary::recover_attribute_table_json(
        &attribute_table_component(size),
        &nonfinite,
        1,
    )
    .is_some()
    {
        return Err(String::from(
            "nonfinite attribute table data should fail closed",
        ));
    }
    Ok(())
}

fn empty_chunk(id: u32) -> Vec<u8> {
    let mut bytes = Vec::new();
    push_u32(&mut bytes, id);
    push_u32(&mut bytes, 12);
    push_u32(&mut bytes, 12);
    bytes
}

fn lens_flare_fixture(
    version: u32,
    declared_quads: u32,
    quad_groups: usize,
) -> Result<(Vec<u8>, usize), String> {
    const LENS_FLARE: u32 = 0x03f0_000d;
    const QUAD_GROUP: u32 = 0x0001_7002;
    const MESH: u32 = 0x0001_0000;
    const COMPOSITE_DRAWABLE: u32 = 0x0000_4512;
    let mut fields = Vec::new();
    push_pascal(&mut fields, "flare")?;
    push_u32(&mut fields, version);
    push_u32(&mut fields, declared_quads);
    let header_size = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("lens flare header overflowed"))?;
    let mut children = Vec::new();
    for _ in 0..quad_groups {
        children.extend_from_slice(&empty_chunk(QUAD_GROUP));
    }
    children.extend_from_slice(&empty_chunk(MESH));
    children.extend_from_slice(&empty_chunk(COMPOSITE_DRAWABLE));
    let total_size = header_size
        .checked_add(children.len())
        .ok_or_else(|| String::from("lens flare total overflowed"))?;
    let mut source = Vec::new();
    push_u32(&mut source, LENS_FLARE);
    push_u32(
        &mut source,
        u32::try_from(header_size).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(total_size).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&fields);
    source.extend_from_slice(&children);
    Ok((source, header_size))
}

fn lens_flare_component(header_size: usize, total_size: usize) -> ChunkRecord {
    ChunkRecord {
        ordinal: 7,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x03f0_000d,
        kind: crate::ChunkKind::SrrLensFlareDsg,
        offset: 0,
        header_size,
        total_size,
        payload_offset: header_size,
        payload_size: total_size.saturating_sub(header_size),
        child_count: 5,
    }
}

#[test]
fn lens_flare_validates_declared_quad_groups() -> Result<(), String> {
    let (source, header_size) = lens_flare_fixture(0, 3, 3)?;
    let recovered = auxiliary::recover_lens_flare_json(
        &lens_flare_component(header_size, source.len()),
        &source,
        1,
    )
    .ok_or_else(|| String::from("lens flare fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["version"] == 0 && value["num_billboard_quads"] == 3 {
        Ok(())
    } else {
        Err(String::from("lens flare source counts were discarded"))
    }
}

#[test]
fn lens_flare_rejects_unobserved_source_shapes() -> Result<(), String> {
    for (version, declared, physical) in [(1, 3, 3), (0, 4, 3)] {
        let (source, header_size) =
            lens_flare_fixture(version, declared, physical)?;
        if auxiliary::recover_lens_flare_json(
            &lens_flare_component(header_size, source.len()),
            &source,
            1,
        )
        .is_some()
        {
            return Err(String::from(
                "unobserved lens flare source shape should fail closed",
            ));
        }
    }
    Ok(())
}

fn breakable_fixture(child_id: u32) -> Vec<u8> {
    const BREAKABLE: u32 = 0x0300_1000;
    let mut source = Vec::new();
    let child = empty_chunk(child_id);
    let total_size = 20_usize.saturating_add(child.len());
    push_u32(&mut source, BREAKABLE);
    push_u32(&mut source, 20);
    push_u32(&mut source, u32::try_from(total_size).unwrap_or(u32::MAX));
    push_u32(&mut source, 24);
    push_u32(&mut source, 3);
    source.extend_from_slice(&child);
    source
}

fn breakable_component(total_size: usize) -> ChunkRecord {
    ChunkRecord {
        ordinal: 8,
        depth: 1,
        parent_ordinal: Some(0),
        id: 0x0300_1000,
        kind: crate::ChunkKind::SrrBreakableObject,
        offset: 0,
        header_size: 20,
        total_size,
        payload_offset: 20,
        payload_size: total_size.saturating_sub(20),
        child_count: 1,
    }
}

#[test]
fn breakable_object_accepts_schema_child() -> Result<(), String> {
    let source = breakable_fixture(0x0001_0000);
    let recovered = auxiliary::recover_breakable_object_json(
        &breakable_component(source.len()),
        &source,
        1,
    )
    .ok_or_else(|| String::from("breakable fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["breakable_type"] == 24 && value["max_instances"] == 3 {
        Ok(())
    } else {
        Err(String::from("breakable source values were discarded"))
    }
}

#[test]
fn breakable_object_rejects_unknown_child() -> Result<(), String> {
    let source = breakable_fixture(0xdead_beef);
    if auxiliary::recover_breakable_object_json(
        &breakable_component(source.len()),
        &source,
        1,
    )
    .is_none()
    {
        Ok(())
    } else {
        Err(String::from("unknown breakable child should fail closed"))
    }
}

fn animated_dsg_fixture(
    parent_id: u32,
    version: u32,
    omit_controller: bool,
    unknown_child: bool,
) -> Result<(Vec<u8>, Vec<ChunkRecord>), String> {
    const ANIM_DSG: u32 = 0x03f0_000c;
    const ANIM_COLL_DSG: u32 = 0x03f0_0008;
    let parent_kind = match parent_id {
        ANIM_COLL_DSG => crate::ChunkKind::SrrAnimCollDsg,
        ANIM_DSG => crate::ChunkKind::SrrAnimDsg,
        _ => crate::ChunkKind::Unknown,
    };
    let mut fields = Vec::new();
    push_pascal(&mut fields, "animated")?;
    push_u32(&mut fields, version);
    push_u32(&mut fields, 9);
    let header_size = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("animated DSG header overflowed"))?;
    let mut children = vec![
        (0x0001_7002_u32, crate::ChunkKind::QuadGroup, "quad"),
        (
            0x0000_4512_u32,
            crate::ChunkKind::CompositeDrawable,
            "composite",
        ),
    ];
    if parent_id == ANIM_COLL_DSG {
        children.push((
            0x0701_0000_u32,
            crate::ChunkKind::SimulationCollisionObject,
            "collision",
        ));
    }
    children.push((
        0x0012_1200_u32,
        crate::ChunkKind::FrameController,
        "frame",
    ));
    if !omit_controller {
        children.push((
            0x0000_48a0_u32,
            crate::ChunkKind::MultiController,
            "controller",
        ));
    }
    if unknown_child {
        children.push((0xdead_beef, crate::ChunkKind::Unknown, "unknown"));
    }

    let mut child_bytes = Vec::new();
    let mut records = Vec::new();
    let mut offset = header_size;
    for (index, (id, kind, name)) in children.into_iter().enumerate() {
        let mut child_fields = Vec::new();
        push_pascal(&mut child_fields, name)?;
        let size = 12_usize
            .checked_add(child_fields.len())
            .ok_or_else(|| String::from("animated DSG child overflowed"))?;
        push_u32(&mut child_bytes, id);
        push_u32(
            &mut child_bytes,
            u32::try_from(size).map_err(|error| error.to_string())?,
        );
        push_u32(
            &mut child_bytes,
            u32::try_from(size).map_err(|error| error.to_string())?,
        );
        child_bytes.extend_from_slice(&child_fields);
        records.push(ChunkRecord {
            ordinal: 2 + index,
            depth: 2,
            parent_ordinal: Some(1),
            id,
            kind,
            offset,
            header_size: size,
            total_size: size,
            payload_offset: offset + size,
            payload_size: 0,
            child_count: 0,
        });
        offset = offset
            .checked_add(size)
            .ok_or_else(|| String::from("animated DSG offset overflowed"))?;
    }
    let total_size = header_size
        .checked_add(child_bytes.len())
        .ok_or_else(|| String::from("animated DSG total overflowed"))?;
    let mut source = Vec::new();
    push_u32(&mut source, parent_id);
    push_u32(
        &mut source,
        u32::try_from(header_size).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(total_size).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&fields);
    source.extend_from_slice(&child_bytes);
    records.insert(0, ChunkRecord {
        ordinal: 1,
        depth: 1,
        parent_ordinal: Some(0),
        id: parent_id,
        kind: parent_kind,
        offset: 0,
        header_size,
        total_size,
        payload_offset: header_size,
        payload_size: total_size.saturating_sub(header_size),
        child_count: records.len(),
    });
    Ok((source, records))
}

#[test]
fn animated_dsg_preserves_direct_child_relationships() -> Result<(), String> {
    let (source, records) = animated_dsg_fixture(0x03f0_0008, 0, false, false)?;
    let parent = records
        .first()
        .ok_or_else(|| String::from("animated DSG parent should exist"))?;
    let recovered = schema::recover_anim_dsg_json(parent, &source, 1, &records)
        .ok_or_else(|| String::from("animated DSG fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["has_alpha"] == 9
        && value["children"][0]["kind"] == "quad_group"
        && value["children"][1]["source_ordinal"] == 3
        && value["children"][2]["kind"] == "simulation_collision_object"
    {
        Ok(())
    } else {
        Err(String::from("animated DSG child topology was discarded"))
    }
}

#[test]
fn animated_dsg_rejects_source_contract_drift() -> Result<(), String> {
    for (parent_id, version, omit_controller, unknown_child) in [
        (0x03f0_000c, 1, false, false),
        (0x03f0_000c, 0, true, false),
        (0x03f0_0008, 0, false, true),
    ] {
        let (source, records) = animated_dsg_fixture(
            parent_id,
            version,
            omit_controller,
            unknown_child,
        )?;
        let parent = records
            .first()
            .ok_or_else(|| String::from("animated DSG parent should exist"))?;
        if schema::recover_anim_dsg_json(parent, &source, 1, &records).is_some()
        {
            return Err(String::from(
                "animated DSG source-contract drift should fail closed",
            ));
        }
    }
    Ok(())
}

fn fence_fixture(
    wall_id: u32,
    start_x: f32,
    trailing_wall_word: bool,
) -> Result<(Vec<u8>, ChunkRecord), String> {
    const FENCE: u32 = 0x03f0_0007;
    let mut wall_fields = Vec::new();
    for value in [
        start_x, 0_f32, 2_f32, 3_f32, 0_f32, 4_f32, 0_f32, 0_f32, 1_f32,
    ] {
        push_f32(&mut wall_fields, value);
    }
    if trailing_wall_word {
        push_u32(&mut wall_fields, 99);
    }
    let wall_size = 12_usize
        .checked_add(wall_fields.len())
        .ok_or_else(|| String::from("fence wall fixture overflowed"))?;
    let total_size = 12_usize
        .checked_add(wall_size)
        .ok_or_else(|| String::from("fence fixture overflowed"))?;
    let mut source = Vec::new();
    push_u32(&mut source, FENCE);
    push_u32(&mut source, 12);
    push_u32(
        &mut source,
        u32::try_from(total_size).map_err(|error| error.to_string())?,
    );
    push_u32(&mut source, wall_id);
    push_u32(
        &mut source,
        u32::try_from(wall_size).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(wall_size).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&wall_fields);
    Ok((source, ChunkRecord {
        ordinal: 1,
        depth: 1,
        parent_ordinal: Some(0),
        id: FENCE,
        kind: crate::ChunkKind::SrrFenceDsg,
        offset: 0,
        header_size: 12,
        total_size,
        payload_offset: 12,
        payload_size: total_size.saturating_sub(12),
        child_count: 1,
    }))
}

#[test]
fn fence_preserves_exact_wall_geometry() -> Result<(), String> {
    let (source, component) = fence_fixture(0x0300_0000, 1_f32, false)?;
    let recovered = schema::recover_fence_json(&component, &source, 1)
        .ok_or_else(|| String::from("fence fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["start"] == serde_json::json!([1, 0, 2])
        && value["end"] == serde_json::json!([3, 0, 4])
        && value["normal"] == serde_json::json!([0, 0, 1])
    {
        Ok(())
    } else {
        Err(String::from("fence wall geometry was discarded"))
    }
}

#[test]
fn fence_rejects_unobserved_wall_shapes() -> Result<(), String> {
    for (wall_id, start_x, trailing) in [
        (0x0300_0001, 1_f32, false),
        (0x0300_0000, f32::NAN, false),
        (0x0300_0000, 1_f32, true),
    ] {
        let (source, component) = fence_fixture(wall_id, start_x, trailing)?;
        if schema::recover_fence_json(&component, &source, 1).is_some() {
            return Err(String::from(
                "unobserved fence shape should fail closed",
            ));
        }
    }
    Ok(())
}

fn world_sphere_fixture(
    version: u32,
    declared_meshes: u32,
    declared_billboards: u32,
) -> Result<(Vec<u8>, Vec<ChunkRecord>), String> {
    const WORLD_SPHERE: u32 = 0x03f0_000b;
    let mut fields = Vec::new();
    push_pascal(&mut fields, "world")?;
    push_u32(&mut fields, version);
    push_u32(&mut fields, declared_meshes);
    push_u32(&mut fields, declared_billboards);
    let header_size = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("world sphere header overflowed"))?;
    let children = [
        (0x0001_0000_u32, crate::ChunkKind::Mesh, "mesh_child"),
        (
            0x0001_7002_u32,
            crate::ChunkKind::QuadGroup,
            "billboard_child",
        ),
        (
            0x0000_4512_u32,
            crate::ChunkKind::CompositeDrawable,
            "composite_child",
        ),
    ];
    let mut source = Vec::new();
    let mut child_bytes = Vec::new();
    let mut records = Vec::new();
    let mut offset = header_size;
    for (index, (id, kind, name)) in children.into_iter().enumerate() {
        let mut child_fields = Vec::new();
        push_pascal(&mut child_fields, name)?;
        let size = 12_usize
            .checked_add(child_fields.len())
            .ok_or_else(|| String::from("world sphere child overflowed"))?;
        push_u32(&mut child_bytes, id);
        push_u32(
            &mut child_bytes,
            u32::try_from(size).map_err(|error| error.to_string())?,
        );
        push_u32(
            &mut child_bytes,
            u32::try_from(size).map_err(|error| error.to_string())?,
        );
        child_bytes.extend_from_slice(&child_fields);
        records.push(ChunkRecord {
            ordinal: 2 + index,
            depth: 2,
            parent_ordinal: Some(1),
            id,
            kind,
            offset,
            header_size: size,
            total_size: size,
            payload_offset: offset + size,
            payload_size: 0,
            child_count: 0,
        });
        offset = offset
            .checked_add(size)
            .ok_or_else(|| String::from("world sphere offset overflowed"))?;
    }
    let total_size = header_size
        .checked_add(child_bytes.len())
        .ok_or_else(|| String::from("world sphere total overflowed"))?;
    push_u32(&mut source, WORLD_SPHERE);
    push_u32(
        &mut source,
        u32::try_from(header_size).map_err(|error| error.to_string())?,
    );
    push_u32(
        &mut source,
        u32::try_from(total_size).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&fields);
    source.extend_from_slice(&child_bytes);
    records.insert(0, ChunkRecord {
        ordinal: 1,
        depth: 1,
        parent_ordinal: Some(0),
        id: WORLD_SPHERE,
        kind: crate::ChunkKind::SrrWorldSphereDsg,
        offset: 0,
        header_size,
        total_size,
        payload_offset: header_size,
        payload_size: total_size.saturating_sub(header_size),
        child_count: 3,
    });
    Ok((source, records))
}

#[test]
fn world_sphere_preserves_direct_child_relationships() -> Result<(), String> {
    let (source, records) = world_sphere_fixture(0, 1, 1)?;
    let parent = records
        .first()
        .ok_or_else(|| String::from("world sphere parent should exist"))?;
    let recovered =
        render::recover_world_sphere_json(parent, &source, 1, Some(&records))
            .ok_or_else(|| String::from("world sphere fixture should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if value["num_meshes"] == 1
        && value["num_billboard_quads"] == 1
        && value["children"][0]["source_ordinal"] == 2
        && value["children"][0]["kind"] == "mesh"
        && value["children"][1]["name"] == "billboard_child"
    {
        Ok(())
    } else {
        Err(String::from("world sphere child topology was discarded"))
    }
}

#[test]
fn world_sphere_rejects_source_contract_drift() -> Result<(), String> {
    for (version, meshes, billboards) in [(1, 1, 1), (0, 2, 1), (0, 1, 2)] {
        let (source, records) =
            world_sphere_fixture(version, meshes, billboards)?;
        let parent = records
            .first()
            .ok_or_else(|| String::from("world sphere parent should exist"))?;
        if render::recover_world_sphere_json(parent, &source, 1, Some(&records))
            .is_some()
        {
            return Err(String::from(
                "world sphere contract drift should fail closed",
            ));
        }
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

fn append_primitive_group_child(
    source: &mut Vec<u8>,
    mesh_header: usize,
    id: u32,
    payload: &[u8],
) -> Result<(), String> {
    let group_total_offset = mesh_header
        .checked_add(8)
        .ok_or_else(|| String::from("group total offset overflowed"))?;
    let old_group_total = u32::from_le_bytes(
        source
            .get(group_total_offset..group_total_offset + 4)
            .ok_or_else(|| String::from("group total field missing"))?
            .try_into()
            .map_err(|error| format!("group total slice failed: {error}"))?,
    );
    let child_size = 12_usize
        .checked_add(payload.len())
        .ok_or_else(|| String::from("primitive child fixture overflowed"))?;
    let child_size =
        u32::try_from(child_size).map_err(|error| error.to_string())?;
    push_u32(source, id);
    push_u32(source, child_size);
    push_u32(source, child_size);
    source.extend_from_slice(payload);
    let group_total = old_group_total
        .checked_add(child_size)
        .ok_or_else(|| String::from("group total overflowed"))?;
    source[group_total_offset..group_total_offset + 4]
        .copy_from_slice(&group_total.to_le_bytes());
    let mesh_total =
        u32::try_from(source.len()).map_err(|error| error.to_string())?;
    source[8..12].copy_from_slice(&mesh_total.to_le_bytes());
    Ok(())
}

fn append_mesh_child(
    source: &mut Vec<u8>,
    id: u32,
    payload: &[u8],
) -> Result<(), String> {
    let child_size = 12_usize
        .checked_add(payload.len())
        .ok_or_else(|| String::from("mesh child fixture overflowed"))?;
    let child_size =
        u32::try_from(child_size).map_err(|error| error.to_string())?;
    push_u32(source, id);
    push_u32(source, child_size);
    push_u32(source, child_size);
    source.extend_from_slice(payload);
    let mesh_total =
        u32::try_from(source.len()).map_err(|error| error.to_string())?;
    source[8..12].copy_from_slice(&mesh_total.to_le_bytes());
    Ok(())
}

fn append_expression_offsets_fixture(
    source: &mut Vec<u8>,
    declared_lists: u32,
) -> Result<(), String> {
    const EXPRESSION_OFFSETS: u32 = 0x0001_0018;
    const OFFSET_LIST: u32 = 0x0001_000e;
    let mut offset_fields = Vec::new();
    push_u32(&mut offset_fields, 1);
    push_u32(&mut offset_fields, 2);
    push_u32(&mut offset_fields, 7);
    for value in [1_f32, 2., 3.] {
        push_f32(&mut offset_fields, value);
    }
    push_u32(&mut offset_fields, 0);
    let offset_size = 12_usize
        .checked_add(offset_fields.len())
        .ok_or_else(|| String::from("offset-list fixture overflowed"))?;
    let offset_size =
        u32::try_from(offset_size).map_err(|error| error.to_string())?;
    let mut expression_fields = Vec::new();
    push_u32(&mut expression_fields, 1);
    push_u32(&mut expression_fields, declared_lists);
    push_u32(&mut expression_fields, 0);
    let expression_header = 12_usize
        .checked_add(expression_fields.len())
        .ok_or_else(|| String::from("expression fixture overflowed"))?;
    let expression_total = expression_header
        .checked_add(
            usize::try_from(offset_size).map_err(|error| error.to_string())?,
        )
        .ok_or_else(|| String::from("expression total overflowed"))?;
    push_u32(source, EXPRESSION_OFFSETS);
    push_u32(
        source,
        u32::try_from(expression_header).map_err(|error| error.to_string())?,
    );
    push_u32(
        source,
        u32::try_from(expression_total).map_err(|error| error.to_string())?,
    );
    source.extend_from_slice(&expression_fields);
    push_u32(source, OFFSET_LIST);
    push_u32(source, offset_size);
    push_u32(source, offset_size);
    source.extend_from_slice(&offset_fields);
    let mesh_total =
        u32::try_from(source.len()).map_err(|error| error.to_string())?;
    source[8..12].copy_from_slice(&mesh_total.to_le_bytes());
    Ok(())
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
fn mesh_recovery_preserves_expression_offset_evidence() -> Result<(), String> {
    let (mut source, mesh_header, _group_header) =
        primitive_group_mesh_fixture()?;
    append_expression_offsets_fixture(&mut source, 1)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    let recovered = render::recover_mesh_json(&component, &source, 1, None)
        .ok_or_else(|| String::from("expression-offset mesh should decode"))?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    let expression = &value["expression_offsets"];
    if expression["num_prim_groups"] != 1
        || expression["num_offset_lists"] != 1
        || expression["prim_group_indices"] != serde_json::json!([0])
        || expression["offset_lists"][0]["num_offsets"] != 1
        || expression["offset_lists"][0]["key_index"] != 2
        || expression["offset_lists"][0]["offsets"][0]["vertex_index"] != 7
        || expression["offset_lists"][0]["offsets"][0]["offset"]
            != serde_json::json!([1, 2, 3])
        || expression["offset_lists"][0]["prim_group_index"] != 0
    {
        return Err(String::from("expression-offset evidence was discarded"));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_expression_offset_count_drift() -> Result<(), String> {
    let (mut source, mesh_header, _group_header) =
        primitive_group_mesh_fixture()?;
    append_expression_offsets_fixture(&mut source, 2)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery replaced the authored expression-list count",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_preserves_bounding_float_bits() -> Result<(), String> {
    const BOUNDING_BOX: u32 = 0x0001_0003;
    const BOUNDING_SPHERE: u32 = 0x0001_0004;
    let (mut source, mesh_header, _group_header) =
        primitive_group_mesh_fixture()?;
    let nan = f32::from_bits(0xffc0_0000);
    let mut box_payload = Vec::new();
    for value in [nan, nan, nan, nan, nan, nan] {
        push_f32(&mut box_payload, value);
    }
    append_mesh_child(&mut source, BOUNDING_BOX, &box_payload)?;
    let mut sphere_payload = Vec::new();
    for value in [nan, nan, nan, 0_f32] {
        push_f32(&mut sphere_payload, value);
    }
    append_mesh_child(&mut source, BOUNDING_SPHERE, &sphere_payload)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    let recovered = render::recover_mesh_json(&component, &source, 1, None)
        .ok_or_else(|| {
            String::from("mesh with non-finite bounds should decode")
        })?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    let nan_bits = serde_json::json!(4_290_772_992_u32);
    if value["bounding_box_f32_bits"]["low"][0] != nan_bits
        || value["bounding_sphere_f32_bits"]["centre"][0] != nan_bits
    {
        return Err(String::from("bounding float payload bits were discarded"));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_duplicate_bounding_box_children() -> Result<(), String>
{
    const BOUNDING_BOX: u32 = 0x0001_0003;
    let (mut source, mesh_header, _group_header) =
        primitive_group_mesh_fixture()?;
    let mut payload = Vec::new();
    for value in [0_f32, 0., 0., 1., 1., 1.] {
        push_f32(&mut payload, value);
    }
    for _ in 0..2 {
        append_mesh_child(&mut source, BOUNDING_BOX, &payload)?;
    }
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted duplicate bounding boxes",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_malformed_bounding_box_size() -> Result<(), String> {
    const BOUNDING_BOX: u32 = 0x0001_0003;
    let (mut source, mesh_header, _group_header) =
        primitive_group_mesh_fixture()?;
    let mut payload = Vec::new();
    for value in [0_f32, 0., 0., 1., 1., 1., 2.] {
        push_f32(&mut payload, value);
    }
    append_mesh_child(&mut source, BOUNDING_BOX, &payload)?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted a malformed bounding-box size",
        ));
    }
    Ok(())
}

#[test]
fn mesh_decoder_rejects_trailing_header_bytes() -> Result<(), String> {
    const MESH: u32 = 0x0001_0000;
    let mut fields = Vec::new();
    push_pascal(&mut fields, "mesh")?;
    push_u32(&mut fields, 3);
    push_u32(&mut fields, 0);
    push_u32(&mut fields, 0xfeed_face);
    let total = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("mesh header fixture overflowed"))?;
    let total = u32::try_from(total).map_err(|error| error.to_string())?;
    let mut source = Vec::new();
    push_u32(&mut source, MESH);
    push_u32(&mut source, total);
    push_u32(&mut source, total);
    source.extend_from_slice(&fields);
    if crate::adapters::driven::decoders::mesh::mesh_json(&source).is_some() {
        return Err(String::from("mesh decoder ignored trailing header bytes"));
    }
    Ok(())
}

#[test]
fn primitive_group_rejects_trailing_header_bytes() -> Result<(), String> {
    let (mut source, mesh_header, group_header) =
        primitive_group_mesh_fixture()?;
    let insert_at = mesh_header
        .checked_add(group_header)
        .ok_or_else(|| String::from("primitive header offset overflowed"))?;
    let _inserted = source
        .splice(insert_at..insert_at, [0xde, 0xad, 0xbe, 0xef])
        .count();
    let group_header_offset = mesh_header.saturating_add(4);
    let group_total_offset = mesh_header.saturating_add(8);
    for offset in [group_header_offset, group_total_offset] {
        let old =
            u32::from_le_bytes(source[offset..offset + 4].try_into().map_err(
                |error| format!("group size slice failed: {error}"),
            )?);
        source[offset..offset + 4]
            .copy_from_slice(&old.saturating_add(4).to_le_bytes());
    }
    let mesh_total =
        u32::try_from(source.len()).map_err(|error| error.to_string())?;
    source[8..12].copy_from_slice(&mesh_total.to_le_bytes());
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "primitive decoder ignored trailing header bytes",
        ));
    }
    Ok(())
}

#[test]
fn skin_decoder_rejects_trailing_header_bytes() -> Result<(), String> {
    const SKIN: u32 = 0x0001_0001;
    let mut fields = Vec::new();
    push_pascal(&mut fields, "skin")?;
    push_u32(&mut fields, 3);
    push_pascal(&mut fields, "skeleton")?;
    push_u32(&mut fields, 0);
    push_u32(&mut fields, 0xfeed_face);
    let total = 12_usize
        .checked_add(fields.len())
        .ok_or_else(|| String::from("skin header fixture overflowed"))?;
    let total = u32::try_from(total).map_err(|error| error.to_string())?;
    let mut source = Vec::new();
    push_u32(&mut source, SKIN);
    push_u32(&mut source, total);
    push_u32(&mut source, total);
    source.extend_from_slice(&fields);
    if crate::adapters::driven::decoders::mesh::skin_json(&source).is_some() {
        return Err(String::from("skin decoder ignored trailing header bytes"));
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
fn mesh_recovery_rejects_per_vertex_list_count_drift() -> Result<(), String> {
    const NORMAL_LIST: u32 = 0x0001_0006;
    let (mut source, mesh_header, _group_header) =
        primitive_group_mesh_fixture()?;
    let mut payload = Vec::new();
    push_u32(&mut payload, 2);
    for value in [0_f32, 0., 1., 0., 1., 0.] {
        push_f32(&mut payload, value);
    }
    append_primitive_group_child(
        &mut source,
        mesh_header,
        NORMAL_LIST,
        &payload,
    )?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted per-vertex list count drift",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_duplicate_vertex_lists() -> Result<(), String> {
    const NORMAL_LIST: u32 = 0x0001_0006;
    let (mut source, mesh_header, _group_header) =
        primitive_group_mesh_fixture()?;
    let mut payload = Vec::new();
    push_u32(&mut payload, 1);
    for value in [0_f32, 0., 1.] {
        push_f32(&mut payload, value);
    }
    for _ in 0..2 {
        append_primitive_group_child(
            &mut source,
            mesh_header,
            NORMAL_LIST,
            &payload,
        )?;
    }
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted duplicate singleton vertex lists",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_zero_count_matrix_palette() -> Result<(), String> {
    const MATRIX_PALETTE: u32 = 0x0001_000d;
    let (mut source, mesh_header, _group_header) =
        primitive_group_mesh_fixture()?;
    let mut payload = Vec::new();
    push_u32(&mut payload, 0);
    append_primitive_group_child(
        &mut source,
        mesh_header,
        MATRIX_PALETTE,
        &payload,
    )?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted an unobserved zero-count matrix palette",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_rejects_duplicate_matrix_palettes() -> Result<(), String> {
    const MATRIX_PALETTE: u32 = 0x0001_000d;
    let (mut source, mesh_header, _group_header) =
        primitive_group_mesh_fixture_with_contract(0, 1, 1)?;
    let mut payload = Vec::new();
    push_u32(&mut payload, 1);
    push_u32(&mut payload, 0);
    for _ in 0..2 {
        append_primitive_group_child(
            &mut source,
            mesh_header,
            MATRIX_PALETTE,
            &payload,
        )?;
    }
    let component = primitive_group_mesh_record(&source, mesh_header);
    if render::recover_mesh_json(&component, &source, 1, None).is_some() {
        return Err(String::from(
            "mesh recovery accepted duplicate matrix palettes",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_preserves_nonfinite_position_bits() -> Result<(), String> {
    let (mut source, mesh_header, group_header) =
        primitive_group_mesh_fixture()?;
    let position_float = mesh_header
        .checked_add(group_header)
        .and_then(|offset| offset.checked_add(16))
        .ok_or_else(|| String::from("position fixture offset overflowed"))?;
    source[position_float..position_float + 4]
        .copy_from_slice(&4_290_772_992_u32.to_le_bytes());
    let component = primitive_group_mesh_record(&source, mesh_header);
    let recovered = render::recover_mesh_json(&component, &source, 1, None)
        .ok_or_else(|| {
            String::from("non-finite position mesh should decode")
        })?;
    let value: serde_json::Value = serde_json::from_slice(&recovered.bytes)
        .map_err(|error| error.to_string())?;
    if !value["prim_groups"][0]["positions"][0][0].is_null()
        || value["prim_groups"][0]["position_nonfinite_f32_bits"][0]["xyz"][0]
            != serde_json::json!(4_290_772_992_u32)
    {
        return Err(String::from(
            "non-finite position payload bits were discarded",
        ));
    }
    Ok(())
}

#[test]
fn mesh_recovery_distinguishes_vertex_shader_child_presence()
-> Result<(), String> {
    const VERTEX_SHADER: u32 = 0x0001_0011;
    let (source, mesh_header, _group_header) = primitive_group_mesh_fixture()?;
    let component = primitive_group_mesh_record(&source, mesh_header);
    let absent = render::recover_mesh_json(&component, &source, 1, None)
        .ok_or_else(|| {
            String::from("mesh without vertex shader should decode")
        })?;
    let absent: serde_json::Value = serde_json::from_slice(&absent.bytes)
        .map_err(|error| error.to_string())?;
    if absent["prim_groups"][0]["vertex_shader_present"] != false {
        return Err(String::from("missing vertex-shader child was invented"));
    }

    let mut present_source = source;
    append_primitive_group_child(
        &mut present_source,
        mesh_header,
        VERTEX_SHADER,
        &[0],
    )?;
    let component = primitive_group_mesh_record(&present_source, mesh_header);
    let present =
        render::recover_mesh_json(&component, &present_source, 1, None)
            .ok_or_else(|| {
                String::from("empty vertex-shader child should decode")
            })?;
    let present: serde_json::Value = serde_json::from_slice(&present.bytes)
        .map_err(|error| error.to_string())?;
    if present["prim_groups"][0]["vertex_shader_present"] != true
        || present["prim_groups"][0]["vertex_shader"] != ""
    {
        return Err(String::from(
            "authored empty vertex-shader child was collapsed into absence",
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
