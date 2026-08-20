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

fn publication_chunk(parent_ordinal: Option<usize>) -> ChunkRecord {
    ChunkRecord {
        ordinal: 1,
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
    let first = recovered_publication("mesh/shared.json", b"same");
    let nested = recovered_publication("mesh/shared.json", b"same");
    let first_publish = register_recovered_path(
        &mut paths,
        &publication_chunk(Some(0)),
        &first,
    )
    .map_err(|error| error.to_string())?;
    let nested_publish = register_recovered_path(
        &mut paths,
        &publication_chunk(Some(1)),
        &nested,
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
fn publication_registry_rejects_nested_payload_conflict() -> Result<(), String>
{
    let mut paths = BTreeMap::new();
    let first = recovered_publication("mesh/shared.json", b"first");
    let nested = recovered_publication("mesh/shared.json", b"second");
    let first_publish = register_recovered_path(
        &mut paths,
        &publication_chunk(Some(0)),
        &first,
    )
    .map_err(|error| error.to_string())?;
    if !first_publish {
        return Err("first component path claim was skipped".to_owned());
    }
    if register_recovered_path(&mut paths, &publication_chunk(Some(1)), &nested)
        .is_ok()
    {
        return Err(
            "different nested payload reused one component path".to_owned()
        );
    }
    Ok(())
}

#[test]
fn publication_registry_rejects_direct_root_duplicate() -> Result<(), String> {
    let mut paths = BTreeMap::new();
    let first = recovered_publication("mesh/shared.json", b"same");
    let second = recovered_publication("mesh/shared.json", b"same");
    let first_publish = register_recovered_path(
        &mut paths,
        &publication_chunk(Some(0)),
        &first,
    )
    .map_err(|error| error.to_string())?;
    if !first_publish {
        return Err("first component path claim was skipped".to_owned());
    }
    if register_recovered_path(&mut paths, &publication_chunk(Some(0)), &second)
        .is_ok()
    {
        return Err(
            "direct root duplicate component path was accepted".to_owned()
        );
    }
    Ok(())
}

#[test]
fn publication_registry_rejects_case_equivalent_path() -> Result<(), String> {
    let mut paths = BTreeMap::new();
    let first = recovered_publication("mesh/Shared.json", b"same");
    let nested = recovered_publication("MESH/shared.json", b"same");
    let first_publish = register_recovered_path(
        &mut paths,
        &publication_chunk(Some(0)),
        &first,
    )
    .map_err(|error| error.to_string())?;
    if !first_publish {
        return Err("first component path claim was skipped".to_owned());
    }
    if register_recovered_path(&mut paths, &publication_chunk(Some(1)), &nested)
        .is_ok()
    {
        return Err("case-equivalent component path was accepted".to_owned());
    }
    Ok(())
}
