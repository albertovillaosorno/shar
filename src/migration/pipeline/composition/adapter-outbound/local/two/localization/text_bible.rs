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
//   - Text bible outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Text bible outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Text bible outbound adapter.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local;

use super::binary::{ByteCursor, read_utf16z};
use super::{
    Error, LanguageDocument, LanguageEntry, Outcome, TextBibleDocument,
};

/// Stable chunk identifier for a `TextBible` package root.
const TEXT_BIBLE: u32 = 0x0001_800d;
/// Stable chunk identifier for one language payload.
const LANGUAGE: u32 = 0x0001_800e;
/// Byte width of the validated `P3D` chunk header.
const CHUNK_HEADER_SIZE: usize = 12;

/// Parse the first `TextBible` package found in one `P3D` source.
///
/// # Errors
///
/// Returns an error for IO, malformed P3D structure, invalid text fields, or a
/// mismatch between declared and contained language counts.
pub(super) fn parse_text_bible(
    path: &Path,
) -> Outcome<Option<TextBibleDocument>> {
    let bytes = local::read_bytes(path)
        .map_err(|source| Error::io(path.to_path_buf(), source))?;
    parse_text_bible_bytes(path.to_path_buf(), &bytes)
}

/// Normalize an already-loaded source without duplicating filesystem IO.
fn parse_text_bible_bytes(
    source_path: PathBuf,
    bytes: &[u8],
) -> Outcome<Option<TextBibleDocument>> {
    let document = p3d::analyze_p3d(bytes)
        .map_err(|error| Error::invalid(error.to_string()))?;
    let Some(root) =
        document.chunks.iter().find(|chunk| chunk.id == TEXT_BIBLE)
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
    let mut cursor = ByteCursor::new(bytes, start, end)?;
    let name = cursor.read_pstring()?;
    let language_count =
        usize::try_from(cursor.read_u32()?).map_err(|error| {
            Error::invalid(format!(
                "TextBible language count is invalid: {error}"
            ))
        })?;
    let declared_language_ids = cursor.read_pstring()?;
    let mut languages = Vec::new();
    for child in document.chunks.iter().filter(|chunk| {
        chunk.parent_ordinal == Some(root.ordinal) && chunk.id == LANGUAGE
    }) {
        languages.push(parse_language(bytes, child.offset, child.header_size)?);
    }
    if language_count != languages.len() {
        return Err(Error::invalid(format!(
            "{} declares {language_count} languages but contains {}",
            source_path.display(),
            languages.len()
        )));
    }
    let mut seen_language_ids = BTreeSet::new();
    if let Some(duplicate) = languages
        .iter()
        .map(|language| language.id)
        .find(|id| !seen_language_ids.insert(*id))
    {
        return Err(Error::invalid(format!(
            "{} contains duplicate language channel {duplicate}",
            source_path.display()
        )));
    }
    let actual_language_ids: String =
        languages.iter().map(|language| language.id).collect();
    if declared_language_ids != actual_language_ids {
        return Err(Error::invalid(format!(
            "{} declares language ids {declared_language_ids:?} but \
                     contains {actual_language_ids:?}",
            source_path.display()
        )));
    }
    Ok(Some(TextBibleDocument {
        source_path,
        name,
        declared_language_ids,
        languages,
    }))
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
    let mut cursor = ByteCursor::new(bytes, start, end)?;
    let source_name = cursor.read_pstring()?;
    let id = char::from(cursor.read_u8()?);
    let count = usize::try_from(cursor.read_u32()?).map_err(|error| {
        Error::invalid(format!("language entry count is invalid: {error}"))
    })?;
    let modulo = cursor.read_u32()?;
    if modulo == 0 {
        return Err(Error::invalid(
            "language hash modulus must be greater than zero",
        ));
    }
    let buffer_size = usize::try_from(cursor.read_u32()?).map_err(|error| {
        Error::invalid(format!("language buffer size is invalid: {error}"))
    })?;
    if buffer_size % 2 != 0 {
        return Err(Error::invalid(
            "language UTF-16 buffer has an odd byte length",
        ));
    }
    let mut hashes = Vec::with_capacity(count);
    for _ in 0..count {
        hashes.push(cursor.read_u32()?);
    }
    if let Some(hash) = hashes.iter().copied().find(|hash| *hash >= modulo) {
        return Err(Error::invalid(format!(
            "language hash {hash} is outside modulus {modulo}"
        )));
    }
    let mut offsets = Vec::with_capacity(count);
    for _ in 0..count {
        offsets.push(cursor.read_u32()?);
    }
    let buffer = cursor.read_bytes(buffer_size)?;
    let mut entries = Vec::with_capacity(count);
    for (hash, entry_offset) in hashes.into_iter().zip(offsets) {
        let value_offset = usize::try_from(entry_offset).map_err(|error| {
            Error::invalid(format!("language entry offset is invalid: {error}"))
        })?;
        entries.push(LanguageEntry {
            hash,
            offset: entry_offset,
            value: read_utf16z(buffer, value_offset)?,
        });
    }
    Ok(LanguageDocument {
        id,
        language: language_label(id)?,
        source_name,
        modulo,
        entries,
    })
}

/// Map a supported source channel to its stable pipeline language label.
fn language_label(id: char) -> Outcome<&'static str> {
    match id {
        'E' => Ok("english"),
        'F' => Ok("french"),
        'G' => Ok("german"),
        'I' => Ok("italian"),
        'S' => Ok("spanish_spain"),
        _ => Err(Error::invalid(format!(
            "unsupported language channel '{id}'"
        ))),
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/localization/text_bible/tests.rs"]
mod tests;
