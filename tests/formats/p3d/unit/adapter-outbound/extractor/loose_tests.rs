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
