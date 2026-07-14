// File:
//   - text_bible.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/localization/text_bible.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - TextBible package discovery and language-chunk normalization.
// - Must-Not:
//   - Rediscover package membership from paths or emit partial language
//   - records.
// - Allows:
//   - Validated P3D chunk intake and deterministic language ordering.
// - Split-When:
//   - Another localization container format needs an independent adapter.
// - Merge-When:
//   - Another phase-two adapter owns the same TextBible source contract.
// - Summary:
//   - Strict TextBible source normalization.
// - Description:
//   - Uses P3D structural evidence while keeping localization semantics
//   - local.
// - Usage:
//   - Called by phase-two source normalization and future StringTable
//   - planning.
// - Defaults:
//   - Malformed source data fails closed without implicit output.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - true
//   - Reason: Strict TextBible source normalization keeps tightly coupled
//   - validation, ordering, and deterministic transformation invariants
//   - together; split when a stable independently testable sub-boundary is
//   - identified.
//

//! `TextBible` normalization consumes validated `P3D` chunk structure without
//! path inference.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local;

use super::binary::{ByteCursor, read_utf16z};
use super::{
    Error, LanguageDocument, LanguageEntry, Outcome, TextBibleDocument,
};

/// Stable chunk identifier for a `TextBible` package root.
const TEXT_BIBLE: u32 = 0x0001_800D;
/// Stable chunk identifier for one language payload.
const LANGUAGE: u32 = 0x0001_800E;
/// Byte width of the validated `P3D` chunk header.
const CHUNK_HEADER_SIZE: usize = 12;

/// Parse the first `TextBible` package found in one `P3D` source.
///
/// # Errors
///
/// Returns an error for IO, malformed P3D structure, invalid text fields, or a
/// mismatch between declared and contained language counts.
pub(super) fn parse_text_bible(
    path: &Path
) -> Outcome<Option<TextBibleDocument>> {
    let bytes = local::read_bytes(path).map_err(
        |source| {
            Error::io(
                path.to_path_buf(),
                source,
            )
        },
    )?;
    parse_text_bible_bytes(
        path.to_path_buf(),
        &bytes,
    )
}

/// Normalize an already-loaded source without duplicating filesystem IO.
fn parse_text_bible_bytes(
    source_path: PathBuf,
    bytes: &[u8],
) -> Outcome<Option<TextBibleDocument>> {
    let document = p3d::analyze_p3d(bytes)
        .map_err(|error| Error::invalid(error.to_string()))?;
    let Some(root) = document
        .chunks
        .iter()
        .find(|chunk| chunk.id == TEXT_BIBLE)
    else {
        return Ok(None);
    };
    let start = root
        .offset
        .checked_add(CHUNK_HEADER_SIZE)
        .ok_or_else(|| Error::invalid("TextBible header offset overflowed"))?;
    let end = root
        .offset
        .checked_add(root.header_size)
        .ok_or_else(|| Error::invalid("TextBible header end overflowed"))?;
    let mut cursor = ByteCursor::new(
        bytes, start, end,
    )?;
    let name = cursor.read_pstring()?;
    let language_count = usize::try_from(cursor.read_u32()?).map_err(
        |error| {
            Error::invalid(
                format!("TextBible language count is invalid: {error}"),
            )
        },
    )?;
    let declared_language_ids = cursor.read_pstring()?;
    let mut languages = Vec::new();
    for child in document
        .chunks
        .iter()
        .filter(
            |chunk| {
                chunk.parent_ordinal == Some(root.ordinal)
                    && chunk.id == LANGUAGE
            },
        )
    {
        languages.push(
            parse_language(
                bytes,
                child.offset,
                child.header_size,
            )?,
        );
    }
    if language_count != languages.len() {
        return Err(
            Error::invalid(
                format!(
                    "{} declares {language_count} languages but contains {}",
                    source_path.display(),
                    languages.len()
                ),
            ),
        );
    }
    let mut seen_language_ids = BTreeSet::new();
    if let Some(duplicate) = languages
        .iter()
        .map(|language| language.id)
        .find(|id| !seen_language_ids.insert(*id))
    {
        return Err(
            Error::invalid(
                format!(
                    "{} contains duplicate language channel {duplicate}",
                    source_path.display()
                ),
            ),
        );
    }
    let actual_language_ids: String = languages
        .iter()
        .map(|language| language.id)
        .collect();
    if declared_language_ids != actual_language_ids {
        return Err(
            Error::invalid(
                format!(
                    "{} declares language ids {declared_language_ids:?} but \
                     contains {actual_language_ids:?}",
                    source_path.display()
                ),
            ),
        );
    }
    Ok(
        Some(
            TextBibleDocument {
                source_path,
                name,
                declared_language_ids,
                languages,
            },
        ),
    )
}

/// Decode one language child while preserving its source order and ids.
fn parse_language(
    bytes: &[u8],
    offset: usize,
    header_size: usize,
) -> Outcome<LanguageDocument> {
    let start = offset
        .checked_add(CHUNK_HEADER_SIZE)
        .ok_or_else(|| Error::invalid("language header offset overflowed"))?;
    let end = offset
        .checked_add(header_size)
        .ok_or_else(|| Error::invalid("language header end overflowed"))?;
    let mut cursor = ByteCursor::new(
        bytes, start, end,
    )?;
    let source_name = cursor.read_pstring()?;
    let id = char::from(cursor.read_u8()?);
    let count = usize::try_from(cursor.read_u32()?).map_err(
        |error| {
            Error::invalid(format!("language entry count is invalid: {error}"))
        },
    )?;
    let modulo = cursor.read_u32()?;
    if modulo == 0 {
        return Err(
            Error::invalid("language hash modulus must be greater than zero"),
        );
    }
    let buffer_size = usize::try_from(cursor.read_u32()?).map_err(
        |error| {
            Error::invalid(format!("language buffer size is invalid: {error}"))
        },
    )?;
    if buffer_size % 2 != 0 {
        return Err(
            Error::invalid("language UTF-16 buffer has an odd byte length"),
        );
    }
    let mut hashes = Vec::with_capacity(count);
    for _ in 0..count {
        hashes.push(cursor.read_u32()?);
    }
    if let Some(hash) = hashes
        .iter()
        .copied()
        .find(|hash| *hash >= modulo)
    {
        return Err(
            Error::invalid(
                format!("language hash {hash} is outside modulus {modulo}"),
            ),
        );
    }
    let mut offsets = Vec::with_capacity(count);
    for _ in 0..count {
        offsets.push(cursor.read_u32()?);
    }
    let buffer = cursor.read_bytes(buffer_size)?;
    let mut entries = Vec::with_capacity(count);
    for (hash, entry_offset) in hashes
        .into_iter()
        .zip(offsets)
    {
        let value_offset = usize::try_from(entry_offset).map_err(
            |error| {
                Error::invalid(
                    format!("language entry offset is invalid: {error}"),
                )
            },
        )?;
        entries.push(
            LanguageEntry {
                hash,
                offset: entry_offset,
                value: read_utf16z(
                    buffer,
                    value_offset,
                )?,
            },
        );
    }
    Ok(
        LanguageDocument {
            id,
            language: language_label(id)?,
            source_name,
            modulo,
            entries,
        },
    )
}

/// Map a supported source channel to its stable pipeline language label.
fn language_label(id: char) -> Outcome<&'static str> {
    match id {
        'E' => Ok("english"),
        'F' => Ok("french"),
        'G' => Ok("german"),
        'I' => Ok("italian"),
        'S' => Ok("spanish_spain"),
        _ => {
            Err(Error::invalid(format!("unsupported language channel '{id}'")))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{LANGUAGE, TEXT_BIBLE, parse_language, parse_text_bible};

    fn language_bytes(
        id: u8,
        modulo: u32,
        buffer: &[u8],
        entry_offset: u32,
    ) -> Result<Vec<u8>, String> {
        let mut bytes = vec![0; 12];
        bytes.extend(
            [
                1, b'S', id,
            ],
        );
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

    fn chunk(
        id: u32,
        payload: &[u8],
        children: &[u8],
    ) -> Result<Vec<u8>, String> {
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

    fn text_bible_bytes(
        declared: u8,
        actual: u8,
    ) -> Result<Vec<u8>, String> {
        let mut language_payload = vec![
            1, b'S', actual,
        ];
        language_payload.extend(1_u32.to_le_bytes());
        language_payload.extend(100_u32.to_le_bytes());
        language_payload.extend(2_u32.to_le_bytes());
        language_payload.extend(7_u32.to_le_bytes());
        language_payload.extend(0_u32.to_le_bytes());
        language_payload.extend(
            [
                0, 0,
            ],
        );
        let language_chunk = chunk(
            LANGUAGE,
            &language_payload,
            &[],
        )?;
        let mut text_bible_payload = vec![
            1, b'T',
        ];
        text_bible_payload.extend(1_u32.to_le_bytes());
        text_bible_payload.extend(
            [
                1, declared,
            ],
        );
        let text_bible_chunk = chunk(
            TEXT_BIBLE,
            &text_bible_payload,
            &language_chunk,
        )?;
        chunk(
            0xFF44_3350,
            &[],
            &text_bible_chunk,
        )
    }

    fn wrap_text_bible(
        payload: &[u8],
        children: &[u8],
    ) -> Result<Vec<u8>, String> {
        let text_bible = chunk(
            TEXT_BIBLE, payload, children,
        )?;
        chunk(
            0xFF44_3350,
            &[],
            &text_bible,
        )
    }

    fn valid_language_chunk(actual: u8) -> Result<Vec<u8>, String> {
        let bytes = language_bytes(
            actual,
            100,
            &[
                0, 0,
            ],
            0,
        )?;
        let payload = bytes
            .get(12..)
            .ok_or_else(|| "language fixture has no payload".to_owned())?;
        chunk(
            LANGUAGE,
            payload,
            &[],
        )
    }

    fn duplicate_language_bytes() -> Result<Vec<u8>, String> {
        let mut children = valid_language_chunk(b'S')?;
        children.extend(valid_language_chunk(b'S')?);
        let mut text_bible = vec![
            1, b'T',
        ];
        text_bible.extend(2_u32.to_le_bytes());
        text_bible.extend(
            [
                2, b'S', b'S',
            ],
        );
        wrap_text_bible(
            &text_bible,
            &children,
        )
    }

    fn truncated_root_spill_bytes() -> Result<Vec<u8>, String> {
        let filler_header_size = 0x5301usize;
        let filler_payload =
            vec![
                0;
                filler_header_size
                    .checked_sub(12)
                    .ok_or_else(|| "filler header is too small".to_owned())?
            ];
        let filler = chunk(
            1,
            &filler_payload,
            &[],
        )?;
        let mut children = filler;
        children.extend(valid_language_chunk(b'S')?);
        wrap_text_bible(
            &[
                1, b'T',
            ],
            &children,
        )
    }

    fn truncated_language_spill_bytes() -> Result<Vec<u8>, String> {
        let source_language = language_bytes(
            b'S',
            100,
            &[
                0, 0,
            ],
            0,
        )?;
        let payload = source_language
            .get(
                12..source_language
                    .len()
                    .saturating_sub(2),
            )
            .ok_or_else(|| "language spill fixture is truncated".to_owned())?;
        let filler = chunk(
            0,
            &[],
            &[],
        )?;
        let language_chunk = chunk(
            LANGUAGE, payload, &filler,
        )?;
        let mut text_bible = vec![
            1, b'T',
        ];
        text_bible.extend(1_u32.to_le_bytes());
        text_bible.extend(
            [
                1, b'S',
            ],
        );
        wrap_text_bible(
            &text_bible,
            &language_chunk,
        )
    }

    fn parse_temp_text_bible(
        bytes: &[u8],
        label: &str,
    ) -> Result<super::TextBibleDocument, String> {
        let path = std::env::temp_dir().join(
            format!(
                "pipeline-text-bible-{label}-{}.p3d",
                std::process::id(),
            ),
        );
        fs::write(
            &path, bytes,
        )
        .map_err(|error| error.to_string())?;
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
        if parse_temp_text_bible(
            &truncated_root_spill_bytes()?,
            "root-field-spill",
        )
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
            &text_bible_bytes(
                b'S', b'S',
            )?,
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
        let bytes = text_bible_bytes(
            b'E', b'S',
        )?;
        let path = std::env::temp_dir().join(
            format!(
                "pipeline-text-bible-mismatch-{}.p3d",
                std::process::id(),
            ),
        );
        fs::write(
            &path, bytes,
        )
        .map_err(|error| error.to_string())?;
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
        let bytes = language_bytes(
            b'X',
            100,
            &[
                0, 0,
            ],
            0,
        )?;
        if parse_language(
            &bytes,
            0,
            bytes.len(),
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("unsupported language channel was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_zero_modulus() -> Result<(), String> {
        let bytes = language_bytes(
            b'S',
            0,
            &[
                0, 0,
            ],
            0,
        )?;
        if parse_language(
            &bytes,
            0,
            bytes.len(),
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("zero language modulus was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_odd_unused_utf16_buffer() -> Result<(), String> {
        let mut bytes = vec![0; 12];
        bytes.extend(
            [
                1, b'S', b'S',
            ],
        );
        bytes.extend(0_u32.to_le_bytes());
        bytes.extend(100_u32.to_le_bytes());
        bytes.extend(1_u32.to_le_bytes());
        bytes.push(0);
        if parse_language(
            &bytes,
            0,
            bytes.len(),
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("odd unused UTF-16 buffer was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_hash_outside_modulus() -> Result<(), String> {
        let mut bytes = language_bytes(
            b'S',
            100,
            &[
                0, 0,
            ],
            0,
        )?;
        let hash = bytes
            .get_mut(28..32)
            .ok_or_else(|| "language hash fixture is truncated".to_owned())?;
        hash.copy_from_slice(&100_u32.to_le_bytes());
        if parse_language(
            &bytes,
            0,
            bytes.len(),
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("hash equal to language modulus was accepted".to_owned())
        }
    }

    #[test]
    fn rejects_out_of_range_offset() -> Result<(), String> {
        let bytes = language_bytes(
            b'S',
            100,
            &[
                0, 0,
            ],
            4,
        )?;
        if parse_language(
            &bytes,
            0,
            bytes.len(),
        )
        .is_err()
        {
            Ok(())
        } else {
            Err("out-of-range language offset was accepted".to_owned())
        }
    }
}
