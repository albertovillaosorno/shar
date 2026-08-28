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
//   - Scrooby project-child extraction regressions.
// - Must-Not:
//   - Invent page, layer, or widget semantics absent from the source chunk.
// - Allows:
//   - Synthetic screen/page chunks and publication-policy assertions.
// - Split-When:
//   - Deeper Scrooby child families gain independent typed decoders.
// - Merge-When:
//   - Another test module owns the identical project-child boundary.
// - Summary:
//   - Pins typed Scrooby screen/page recovery under project containers.
// - Description:
//   - Proves direct project children become typed normalized evidence while
//     unrelated nesting stays hidden.
// - Usage:
//   - Included only by the P3D extractor test configuration.
// - Defaults:
//   - Malformed field bounds fail closed.
//

//! Scrooby project-child extraction regressions.

use super::*;
use crate::ChunkKind;

fn chunk(
    ordinal: usize,
    parent_ordinal: Option<usize>,
    kind: ChunkKind,
) -> ChunkRecord {
    ChunkRecord {
        ordinal,
        depth: usize::from(parent_ordinal.is_some()),
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

fn push_pascal(bytes: &mut Vec<u8>, value: &str) -> Result<(), String> {
    let length = u8::try_from(value.len()).map_err(|error| {
        format!("fixture Pascal string is too long: {error}")
    })?;
    bytes.push(length);
    bytes.extend_from_slice(value.as_bytes());
    Ok(())
}

fn record(
    kind: ChunkKind,
    id: u32,
    header_size: usize,
    total_size: usize,
    child_count: usize,
) -> ChunkRecord {
    ChunkRecord {
        ordinal: 2,
        depth: 2,
        parent_ordinal: Some(1),
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

#[test]
fn publishes_only_direct_screen_and_page_children_of_project() {
    let project_children = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::ScroobyProject),
        chunk(2, Some(1), ChunkKind::ScroobyScreen),
        chunk(3, Some(1), ChunkKind::ScroobyPage),
    ];
    assert!(should_publish_component(
        &project_children[2],
        &project_children,
    ));
    assert!(should_publish_component(
        &project_children[3],
        &project_children,
    ));

    let unrelated = [
        chunk(0, None, ChunkKind::Root),
        chunk(1, Some(0), ChunkKind::ScroobyPage),
        chunk(2, Some(1), ChunkKind::ScroobyScreen),
    ];
    assert!(!should_publish_component(&unrelated[2], &unrelated));
}

#[test]
fn recovers_scrooby_screen_page_names_exactly() -> Result<(), String> {
    let mut source = vec![0_u8; 12];
    push_pascal(&mut source, "Main")?;
    source.extend_from_slice(&1_u32.to_le_bytes());
    source.extend_from_slice(&2_u32.to_le_bytes());
    push_pascal(&mut source, "Hud")?;
    push_pascal(&mut source, "Pause")?;
    let total_size = source.len();
    source[0..4].copy_from_slice(&0x0001_8001_u32.to_le_bytes());
    source[4..8].copy_from_slice(
        &u32::try_from(total_size)
            .map_err(|error| format!("screen fixture is too large: {error}"))?
            .to_le_bytes(),
    );
    source[8..12].copy_from_slice(
        &u32::try_from(total_size)
            .map_err(|error| format!("screen fixture is too large: {error}"))?
            .to_le_bytes(),
    );
    let component = record(
        ChunkKind::ScroobyScreen,
        0x0001_8001,
        total_size,
        total_size,
        0,
    );
    let recovered = recover_component(&component, &source, 1)
        .map_err(|error| error.to_string())?;
    let json = String::from_utf8(recovered.bytes)
        .map_err(|error| error.to_string())?;
    if !json.contains(r#""schema":"scrooby_screen""#)
        || !json.contains(r#""page_count":2"#)
        || !json.contains(r#""page_names":["Hud","Pause"]"#)
    {
        return Err("screen recovery lost authored page names".to_owned());
    }
    Ok(())
}

#[test]
fn recovers_scrooby_page_header_and_child_inventory() -> Result<(), String> {
    let mut source = vec![0_u8; 12];
    push_pascal(&mut source, "Hud")?;
    source.extend_from_slice(&1_u32.to_le_bytes());
    source.extend_from_slice(&640_u32.to_le_bytes());
    source.extend_from_slice(&480_u32.to_le_bytes());
    let header_size = source.len();
    source.extend_from_slice(&0x0001_8003_u32.to_le_bytes());
    source.extend_from_slice(&12_u32.to_le_bytes());
    source.extend_from_slice(&12_u32.to_le_bytes());
    let image_start = source.len();
    source.extend_from_slice(&[0_u8; 12]);
    push_pascal(&mut source, "Icon")?;
    source.extend_from_slice(&1_u32.to_le_bytes());
    push_pascal(&mut source, "images\\icon.png")?;
    let image_size = source.len() - image_start;
    source[image_start..image_start + 4]
        .copy_from_slice(&0x0001_8100_u32.to_le_bytes());
    let image_size_u32 = u32::try_from(image_size).map_err(|error| {
        format!("image resource fixture is too large: {error}")
    })?;
    source[image_start + 4..image_start + 8]
        .copy_from_slice(&image_size_u32.to_le_bytes());
    source[image_start + 8..image_start + 12]
        .copy_from_slice(&image_size_u32.to_le_bytes());
    let total_size = source.len();
    source[0..4].copy_from_slice(&0x0001_8002_u32.to_le_bytes());
    source[4..8].copy_from_slice(
        &u32::try_from(header_size)
            .map_err(|error| format!("page header is too large: {error}"))?
            .to_le_bytes(),
    );
    source[8..12].copy_from_slice(
        &u32::try_from(total_size)
            .map_err(|error| format!("page fixture is too large: {error}"))?
            .to_le_bytes(),
    );
    let component = record(
        ChunkKind::ScroobyPage,
        0x0001_8002,
        header_size,
        total_size,
        2,
    );
    let recovered = recover_component(&component, &source, 1)
        .map_err(|error| error.to_string())?;
    let json = String::from_utf8(recovered.bytes)
        .map_err(|error| error.to_string())?;
    if !json.contains(r#""schema":"scrooby_page""#)
        || !json.contains(r#""resolution":[640,480]"#)
        || !json.contains(r#""id_hex":"0x00018003""#)
        || !json.contains(r#""id_hex":"0x00018100""#)
        || !json.contains(r#""name":"Icon""#)
    {
        return Err("page recovery lost header or child inventory".to_owned());
    }
    Ok(())
}
