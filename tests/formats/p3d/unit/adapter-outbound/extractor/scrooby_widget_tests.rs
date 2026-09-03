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
//   - Scrooby layer-widget extraction regressions.
// - Must-Not:
//   - Invent runtime widget behavior or resource resolution.
// - Allows:
//   - Synthetic contract-shaped widget chunks and hierarchy assertions.
// - Split-When:
//   - One widget family gains an independent semantic compiler.
// - Merge-When:
//   - Another test module owns the identical widget recovery boundary.
// - Summary:
//   - Pins typed Scrooby widget recovery below layers and groups.
// - Description:
//   - Proves observed authored fields and nested text references remain typed
//     normalized evidence.
// - Usage:
//   - Included only by the P3D extractor test configuration.
// - Defaults:
//   - Malformed bounds and invalid hierarchy fail closed.
//

//! Scrooby layer-widget extraction regressions.

use super::*;
use crate::ChunkKind;

fn chunk(
    ordinal: usize,
    parent_ordinal: Option<usize>,
    kind: ChunkKind,
) -> ChunkRecord {
    ChunkRecord {
        ordinal,
        depth: ordinal,
        parent_ordinal,
        id: 0,
        kind,
        offset: 0,
        header_size: 12,
        total_size: 12,
        payload_offset: 12,
        payload_size: 0,
        child_count: 0,
    }
}

fn record(
    kind: ChunkKind,
    id: u32,
    header_size: usize,
    total_size: usize,
    child_count: usize,
) -> ChunkRecord {
    ChunkRecord {
        ordinal: 4,
        depth: 4,
        parent_ordinal: Some(3),
        id,
        kind,
        offset: 0,
        header_size,
        total_size,
        payload_offset: header_size,
        payload_size: total_size.saturating_sub(header_size),
        child_count,
    }
}

fn push_pascal(bytes: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let length = u8::try_from(value.len()).map_err(|error| {
        format!("fixture Pascal string is too long: {error}")
    })?;
    bytes.push(length);
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

fn finish_chunk(
    source: &mut [u8],
    id: u32,
    header_size: usize,
    total_size: usize,
) -> Result<(), String> {
    source[0..4].copy_from_slice(&id.to_le_bytes());
    source[4..8].copy_from_slice(
        &u32::try_from(header_size)
            .map_err(|error| format!("fixture header is too large: {error}"))?
            .to_le_bytes(),
    );
    source[8..12].copy_from_slice(
        &u32::try_from(total_size)
            .map_err(|error| format!("fixture chunk is too large: {error}"))?
            .to_le_bytes(),
    );
    Ok(())
}

fn push_widget_frame(bytes: &mut Vec<u8>, name: &str) -> Result<(), String> {
    push_pascal(bytes, name)?;
    for value in [1_u32, 10, 20, 30, 40, 1, 2, 0x1122_3344, 7] {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
    bytes.extend_from_slice(&0.5_f32.to_le_bytes());
    Ok(())
}

fn recovered_json(
    kind: ChunkKind,
    id: u32,
    source: &[u8],
    header_size: usize,
    child_count: usize,
) -> Result<String, String> {
    let component = record(kind, id, header_size, source.len(), child_count);
    let recovered = recover_component(&component, source, 1)
        .map_err(|error| error.to_string())?;
    String::from_utf8(recovered.bytes).map_err(|error| error.to_string())
}

#[test]
fn publishes_widgets_only_below_scrooby_layout_parents() {
    let chunks = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::ScroobyProject),
        chunk(2, Some(1), ChunkKind::ScroobyPage),
        chunk(3, Some(2), ChunkKind::ScroobyLayer),
        chunk(4, Some(3), ChunkKind::ScroobyGroup),
        chunk(5, Some(4), ChunkKind::ScroobyMultiText),
        chunk(6, Some(5), ChunkKind::ScroobyStringTextBible),
    ];
    for component in &chunks[4..] {
        assert!(should_publish_component(component, &chunks));
    }

    let invalid = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::ScroobyPage),
        chunk(2, Some(1), ChunkKind::ScroobyMultiSprite),
    ];
    assert!(!should_publish_component(&invalid[2], &invalid));
}

#[test]
fn recovers_multi_sprite_and_pure3d_object_fields() -> Result<(), String> {
    let mut sprite = vec![0_u8; 12];
    push_widget_frame(&mut sprite, "Icons")?;
    sprite.extend_from_slice(&2_u32.to_le_bytes());
    push_pascal(&mut sprite, "a")?;
    push_pascal(&mut sprite, "b")?;
    let sprite_size = sprite.len();
    finish_chunk(&mut sprite, 0x0001_8006, sprite_size, sprite_size)?;
    let json = recovered_json(
        ChunkKind::ScroobyMultiSprite,
        0x0001_8006,
        &sprite,
        sprite_size,
        0,
    )?;
    if !json.contains(r#""position":[10,20]"#)
        || !json.contains(r#""image_names":["a","b"]"#)
    {
        return Err("multi-sprite recovery lost authored fields".to_owned());
    }

    let mut object = vec![0_u8; 12];
    push_widget_frame(&mut object, "Scene")?;
    push_pascal(&mut object, "pure3d\\scene.p3d")?;
    let object_size = object.len();
    finish_chunk(&mut object, 0x0001_8008, object_size, object_size)?;
    let json = recovered_json(
        ChunkKind::ScroobyPure3dObject,
        0x0001_8008,
        &object,
        object_size,
        0,
    )?;
    if !json.contains(r#""filename":"pure3d\\scene.p3d""#) {
        return Err("Pure3D-object recovery lost filename".to_owned());
    }
    Ok(())
}

#[test]
fn recovers_multi_text_and_nested_bible_reference() -> Result<(), String> {
    let mut text = vec![0_u8; 12];
    push_widget_frame(&mut text, "Caption")?;
    push_pascal(&mut text, "font0_16")?;
    text.push(1);
    text.extend_from_slice(&0x1122_3344_u32.to_le_bytes());
    text.extend_from_slice(&3_u32.to_le_bytes());
    text.extend_from_slice(&4_u32.to_le_bytes());
    text.extend_from_slice(&0_u32.to_le_bytes());
    let header_size = text.len();

    let child_start = text.len();
    text.extend_from_slice(&[0_u8; 12]);
    push_pascal(&mut text, "srr2")?;
    push_pascal(&mut text, "MISSION_OBJECTIVE")?;
    let child_size = text.len().saturating_sub(child_start);
    finish_chunk(
        text.get_mut(child_start..).ok_or("child slice missing")?,
        0x0001_800b,
        child_size,
        child_size,
    )?;
    let total_size = text.len();
    finish_chunk(&mut text, 0x0001_8007, header_size, total_size)?;

    let json = recovered_json(
        ChunkKind::ScroobyMultiText,
        0x0001_8007,
        &text,
        header_size,
        1,
    )?;
    if !json.contains(r#""text_style":"font0_16""#)
        || !json.contains(r#""shadow_enabled":1"#)
        || !json.contains(r#""id_hex":"0x0001800b""#)
    {
        return Err(
            "multi-text recovery lost style or child inventory".to_owned()
        );
    }

    let child = text.get(child_start..).ok_or("child slice missing")?;
    let json = recovered_json(
        ChunkKind::ScroobyStringTextBible,
        0x0001_800b,
        child,
        child_size,
        0,
    )?;
    if !json.contains(r#""bible_name":"srr2""#)
        || !json.contains(r#""string_id":"MISSION_OBJECTIVE""#)
    {
        return Err("text-bible reference recovery lost identity".to_owned());
    }
    Ok(())
}

#[test]
fn recovers_polygon_points_and_colours() -> Result<(), String> {
    let mut source = vec![0_u8; 12];
    push_pascal(&mut source, "Shape")?;
    source.extend_from_slice(&1_u32.to_le_bytes());
    source.extend_from_slice(&2_u32.to_le_bytes());
    source.extend_from_slice(&2_u32.to_le_bytes());
    for point in [[1f32, 2., 3.], [4., 5., 6.]] {
        for value in point {
            source.extend_from_slice(&value.to_le_bytes());
        }
    }
    source.extend_from_slice(&0x0102_0304_u32.to_le_bytes());
    source.extend_from_slice(&0xa0b0_c0d0_u32.to_le_bytes());
    let size = source.len();
    finish_chunk(&mut source, 0x0001_8009, size, size)?;
    let json = recovered_json(
        ChunkKind::ScroobyPolygon,
        0x0001_8009,
        &source,
        size,
        0,
    )?;
    if !json.contains(r#""points":[[1,2,3],[4,5,6]]"#)
        || !json.contains(r#""colors":[16909060,2695938256]"#)
    {
        return Err("polygon recovery lost points or colours".to_owned());
    }
    Ok(())
}

#[test]
fn identical_nested_widgets_keep_distinct_occurrences() -> Result<(), String> {
    let mut paths = BTreeMap::new();
    let mut first = RecoveredComponent {
        name: "Caption".to_owned(),
        relative_path: PathBuf::from("scrooby_multi_text/Caption.json"),
        bytes: b"same".to_vec(),
        payload_format: "json".to_owned(),
        recovery_status: "decoded_schema_payload".to_owned(),
    };
    let mut second = RecoveredComponent {
        name: first.name.clone(),
        relative_path: first.relative_path.clone(),
        bytes: first.bytes.clone(),
        payload_format: first.payload_format.clone(),
        recovery_status: first.recovery_status.clone(),
    };
    let mut first_chunk = chunk(10, Some(3), ChunkKind::ScroobyMultiText);
    first_chunk.depth = 4;
    let mut second_chunk = chunk(20, Some(4), ChunkKind::ScroobyMultiText);
    second_chunk.depth = 5;
    if !register_recovered_path(&mut paths, &first_chunk, &mut first)
        .map_err(|error| error.to_string())?
        || !register_recovered_path(&mut paths, &second_chunk, &mut second)
            .map_err(|error| error.to_string())?
    {
        return Err("Scrooby widget occurrence was deduplicated".to_owned());
    }
    if second.relative_path
        != Path::new("scrooby_multi_text/Caption__ordinal_0020.json")
    {
        return Err("Scrooby widget duplicate lacked ordinal alias".to_owned());
    }
    Ok(())
}


fn scrooby_layer_fixture() -> Result<(Vec<u8>, usize), String> {
    let mut source = vec![0_u8; 12];
    push_pascal(&mut source, "HudLayer")?;
    for value in [1_u32, 1, 0, 255] {
        source.extend_from_slice(&value.to_le_bytes());
    }
    let header_size = source.len();
    finish_chunk(&mut source, 0x0001_8003, header_size, header_size)?;
    Ok((source, header_size))
}

#[test]
fn layer_rejects_scrooby_child_drift() -> Result<(), String> {
    let (source, header_size) = scrooby_layer_fixture()?;

    let mut unknown_child = source.clone();
    unknown_child.extend_from_slice(&0xdead_beef_u32.to_le_bytes());
    unknown_child.extend_from_slice(&12_u32.to_le_bytes());
    unknown_child.extend_from_slice(&12_u32.to_le_bytes());
    let unknown_total = unknown_child.len();
    finish_chunk(
        &mut unknown_child,
        0x0001_8003,
        header_size,
        unknown_total,
    )?;
    let component = record(
        ChunkKind::ScroobyLayer,
        0x0001_8003,
        header_size,
        unknown_total,
        1,
    );
    if auxiliary::recover_scrooby_layer_json(&component, &unknown_child, 1)
        .is_some()
    {
        return Err("layer accepted an undeclared direct child".to_owned());
    }

    let mut truncated_child = source;
    truncated_child.extend_from_slice(&0x0001_8004_u32.to_le_bytes());
    let truncated_total = truncated_child.len();
    finish_chunk(
        &mut truncated_child,
        0x0001_8003,
        header_size,
        truncated_total,
    )?;
    let component = record(
        ChunkKind::ScroobyLayer,
        0x0001_8003,
        header_size,
        truncated_total,
        1,
    );
    if auxiliary::recover_scrooby_layer_json(&component, &truncated_child, 1)
        .is_some()
    {
        return Err("layer ignored a truncated direct child".to_owned());
    }
    Ok(())
}

#[test]
fn layer_preserves_scrooby_child_inventory() -> Result<(), String> {
    let (mut source, header_size) = scrooby_layer_fixture()?;
    for id in [
        0x0001_8004_u32,
        0x0001_8006,
        0x0001_8007,
        0x0001_8008,
        0x0001_8009,
    ] {
        source.extend_from_slice(&id.to_le_bytes());
        source.extend_from_slice(&12_u32.to_le_bytes());
        source.extend_from_slice(&12_u32.to_le_bytes());
    }
    let total_size = source.len();
    finish_chunk(&mut source, 0x0001_8003, header_size, total_size)?;
    let component = record(
        ChunkKind::ScroobyLayer,
        0x0001_8003,
        header_size,
        total_size,
        5,
    );
    let recovered = auxiliary::recover_scrooby_layer_json(
        &component,
        &source,
        1,
    )
    .ok_or_else(|| "declared layer children should decode".to_owned())?;
    let json = String::from_utf8(recovered.bytes)
        .map_err(|error| error.to_string())?;
    for id in ["00018004", "00018006", "00018007", "00018008", "00018009"] {
        if !json.contains(&format!(r#""id_hex":"0x{id}""#)) {
            return Err(format!("layer lost declared child 0x{id}"));
        }
    }
    Ok(())
}
