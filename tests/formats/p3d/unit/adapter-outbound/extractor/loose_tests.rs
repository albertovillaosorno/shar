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
