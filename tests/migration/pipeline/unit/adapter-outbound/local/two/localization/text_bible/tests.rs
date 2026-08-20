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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

use std::fs;

use super::{LANGUAGE, TEXT_BIBLE, parse_language, parse_text_bible};

fn language_bytes(
    id: u8,
    modulo: u32,
    buffer: &[u8],
    entry_offset: u32,
) -> Result<Vec<u8>, String> {
    let mut bytes = vec![0; 12];
    bytes.extend([1, b'S', id]);
    bytes.extend(1_u32.to_le_bytes());
    bytes.extend(modulo.to_le_bytes());
    let buffer_size =
        u32::try_from(buffer.len()).map_err(|error| error.to_string())?;
    bytes.extend(buffer_size.to_le_bytes());
    bytes.extend(7_u32.to_le_bytes());
    bytes.extend(entry_offset.to_le_bytes());
    bytes.extend(buffer);
    Ok(bytes)
}

fn chunk(id: u32, payload: &[u8], children: &[u8]) -> Result<Vec<u8>, String> {
    let header_size = 12usize
        .checked_add(payload.len())
        .ok_or_else(|| "chunk header size overflowed".to_owned())?;
    let total_size = header_size
        .checked_add(children.len())
        .ok_or_else(|| "chunk total size overflowed".to_owned())?;
    let mut bytes = Vec::with_capacity(total_size);
    bytes.extend(id.to_le_bytes());
    bytes.extend(
        u32::try_from(header_size)
            .map_err(|error| error.to_string())?
            .to_le_bytes(),
    );
    bytes.extend(
        u32::try_from(total_size)
            .map_err(|error| error.to_string())?
            .to_le_bytes(),
    );
    bytes.extend(payload);
    bytes.extend(children);
    Ok(bytes)
}

fn text_bible_bytes(declared: u8, actual: u8) -> Result<Vec<u8>, String> {
    let mut language_payload = vec![1, b'S', actual];
    language_payload.extend(1_u32.to_le_bytes());
    language_payload.extend(100_u32.to_le_bytes());
    language_payload.extend(2_u32.to_le_bytes());
    language_payload.extend(7_u32.to_le_bytes());
    language_payload.extend(0_u32.to_le_bytes());
    language_payload.extend([0, 0]);
    let language_chunk = chunk(LANGUAGE, &language_payload, &[])?;
    let mut text_bible_payload = vec![1, b'T'];
    text_bible_payload.extend(1_u32.to_le_bytes());
    text_bible_payload.extend([1, declared]);
    let text_bible_chunk =
        chunk(TEXT_BIBLE, &text_bible_payload, &language_chunk)?;
    chunk(0xff44_3350, &[], &text_bible_chunk)
}

fn wrap_text_bible(payload: &[u8], children: &[u8]) -> Result<Vec<u8>, String> {
    let text_bible = chunk(TEXT_BIBLE, payload, children)?;
    chunk(0xff44_3350, &[], &text_bible)
}

fn valid_language_chunk(actual: u8) -> Result<Vec<u8>, String> {
    let bytes = language_bytes(actual, 100, &[0, 0], 0)?;
    let payload = bytes
        .get(12..)
        .ok_or_else(|| "language fixture has no payload".to_owned())?;
    chunk(LANGUAGE, payload, &[])
}

fn duplicate_language_bytes() -> Result<Vec<u8>, String> {
    let mut children = valid_language_chunk(b'S')?;
    children.extend(valid_language_chunk(b'S')?);
    let mut text_bible = vec![1, b'T'];
    text_bible.extend(2_u32.to_le_bytes());
    text_bible.extend([2, b'S', b'S']);
    wrap_text_bible(&text_bible, &children)
}

fn truncated_root_spill_bytes() -> Result<Vec<u8>, String> {
    let filler_header_size = 0x5301usize;
    let filler_payload = vec![
        0;
        filler_header_size.checked_sub(12).ok_or_else(
            || "filler header is too small".to_owned()
        )?
    ];
    let filler = chunk(1, &filler_payload, &[])?;
    let mut children = filler;
    children.extend(valid_language_chunk(b'S')?);
    wrap_text_bible(&[1, b'T'], &children)
}

fn truncated_language_spill_bytes() -> Result<Vec<u8>, String> {
    let source_language = language_bytes(b'S', 100, &[0, 0], 0)?;
    let payload = source_language
        .get(12..source_language.len().saturating_sub(2))
        .ok_or_else(|| "language spill fixture is truncated".to_owned())?;
    let filler = chunk(0, &[], &[])?;
    let language_chunk = chunk(LANGUAGE, payload, &filler)?;
    let mut text_bible = vec![1, b'T'];
    text_bible.extend(1_u32.to_le_bytes());
    text_bible.extend([1, b'S']);
    wrap_text_bible(&text_bible, &language_chunk)
}

fn parse_temp_text_bible(
    bytes: &[u8],
    label: &str,
) -> Result<super::TextBibleDocument, String> {
    let path = std::env::temp_dir().join(format!(
        "pipeline-text-bible-{label}-{}.p3d",
        std::process::id(),
    ));
    fs::write(&path, bytes).map_err(|error| error.to_string())?;
    let result = parse_text_bible(&path).map_err(|error| error.to_string());
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    result?.ok_or_else(|| "TextBible fixture was not discovered".to_owned())
}

#[test]
fn rejects_duplicate_language_channels() -> Result<(), String> {
    if parse_temp_text_bible(
        &duplicate_language_bytes()?,
        "duplicate-language-channel",
    )
    .is_err()
    {
        Ok(())
    } else {
        Err("duplicate language channel was accepted".to_owned())
    }
}

#[test]
fn rejects_root_fields_spilling_into_children() -> Result<(), String> {
    if parse_temp_text_bible(&truncated_root_spill_bytes()?, "root-field-spill")
        .is_err()
    {
        Ok(())
    } else {
        Err("TextBible root fields consumed child chunk bytes".to_owned())
    }
}

#[test]
fn rejects_language_fields_spilling_into_children() -> Result<(), String> {
    if parse_temp_text_bible(
        &truncated_language_spill_bytes()?,
        "language-field-spill",
    )
    .is_err()
    {
        Ok(())
    } else {
        Err("language fields consumed child chunk bytes".to_owned())
    }
}

#[test]
fn accepts_matching_declared_language_ids() -> Result<(), String> {
    let document = parse_temp_text_bible(
        &text_bible_bytes(b'S', b'S')?,
        "matching-language",
    )?;
    if document.declared_language_ids == "S"
        && document
            .languages
            .first()
            .is_some_and(|language| language.id == 'S')
    {
        Ok(())
    } else {
        Err(format!("unexpected TextBible fixture: {document:?}"))
    }
}

#[test]
fn rejects_declared_language_identity_mismatch() -> Result<(), String> {
    let bytes = text_bible_bytes(b'E', b'S')?;
    let path = std::env::temp_dir().join(format!(
        "pipeline-text-bible-mismatch-{}.p3d",
        std::process::id(),
    ));
    fs::write(&path, bytes).map_err(|error| error.to_string())?;
    let result = parse_text_bible(&path);
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    if result.is_err() {
        Ok(())
    } else {
        Err("declared language identity mismatch was accepted".to_owned())
    }
}

#[test]
fn rejects_unsupported_language_channel() -> Result<(), String> {
    let bytes = language_bytes(b'X', 100, &[0, 0], 0)?;
    if parse_language(&bytes, 0, bytes.len()).is_err() {
        Ok(())
    } else {
        Err("unsupported language channel was accepted".to_owned())
    }
}

#[test]
fn rejects_zero_modulus() -> Result<(), String> {
    let bytes = language_bytes(b'S', 0, &[0, 0], 0)?;
    if parse_language(&bytes, 0, bytes.len()).is_err() {
        Ok(())
    } else {
        Err("zero language modulus was accepted".to_owned())
    }
}

#[test]
fn rejects_odd_unused_utf16_buffer() -> Result<(), String> {
    let mut bytes = vec![0; 12];
    bytes.extend([1, b'S', b'S']);
    bytes.extend(0_u32.to_le_bytes());
    bytes.extend(100_u32.to_le_bytes());
    bytes.extend(1_u32.to_le_bytes());
    bytes.push(0);
    if parse_language(&bytes, 0, bytes.len()).is_err() {
        Ok(())
    } else {
        Err("odd unused UTF-16 buffer was accepted".to_owned())
    }
}

#[test]
fn rejects_hash_outside_modulus() -> Result<(), String> {
    let mut bytes = language_bytes(b'S', 100, &[0, 0], 0)?;
    let hash = bytes
        .get_mut(28..32)
        .ok_or_else(|| "language hash fixture is truncated".to_owned())?;
    hash.copy_from_slice(&100_u32.to_le_bytes());
    if parse_language(&bytes, 0, bytes.len()).is_err() {
        Ok(())
    } else {
        Err("hash equal to language modulus was accepted".to_owned())
    }
}

#[test]
fn rejects_out_of_range_offset() -> Result<(), String> {
    let bytes = language_bytes(b'S', 100, &[0, 0], 4)?;
    if parse_language(&bytes, 0, bytes.len()).is_err() {
        Ok(())
    } else {
        Err("out-of-range language offset was accepted".to_owned())
    }
}
