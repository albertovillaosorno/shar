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
//   - Encoding outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Encoding outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Encoding outbound adapter.

use super::{Error, Outcome};

/// Decode UTF-8, supported UTF-16, or era Windows-1252 source bytes without
/// lossy replacement.
pub(super) fn decode_text_source(
    bytes: &[u8],
    source_label: &str,
) -> Outcome<String> {
    if bytes.starts_with(&[0xfe, 0xff]) {
        decode_utf16(bytes, source_label, u16::from_be_bytes)
    } else if bytes.starts_with(&[0xff, 0xfe]) || looks_like_utf16_le(bytes) {
        decode_utf16(bytes, source_label, u16::from_le_bytes)
    } else if let Ok(text) = String::from_utf8(bytes.to_vec()) {
        Ok(text)
    } else {
        decode_windows_1252(bytes, source_label)
    }
}

/// Windows-1252 mappings for the 0x80..=0x9f range; `None` marks the five
/// code points Windows leaves undefined so corrupted bytes stay rejected.
const WINDOWS_1252_C1: [Option<char>; 32] = [
    Some('\u{20ac}'),
    None,
    Some('\u{201a}'),
    Some('\u{192}'),
    Some('\u{201e}'),
    Some('\u{2026}'),
    Some('\u{2020}'),
    Some('\u{2021}'),
    Some('\u{2c6}'),
    Some('\u{2030}'),
    Some('\u{160}'),
    Some('\u{2039}'),
    Some('\u{152}'),
    None,
    Some('\u{17d}'),
    None,
    None,
    Some('\u{2018}'),
    Some('\u{2019}'),
    Some('\u{201c}'),
    Some('\u{201d}'),
    Some('\u{2022}'),
    Some('\u{2013}'),
    Some('\u{2014}'),
    Some('\u{2dc}'),
    Some('\u{2122}'),
    Some('\u{161}'),
    Some('\u{203a}'),
    Some('\u{153}'),
    None,
    Some('\u{17e}'),
    Some('\u{178}'),
];

/// Decode strict Windows-1252, reporting the first undefined byte.
///
/// Original-era text sources store accented FIGS letters and symbol glyphs
/// as single Windows-1252 bytes, so the era-correct code page is the
/// deterministic non-UTF-8 contract shared by every text-source reader in
/// phase two. Every defined byte maps to exactly one scalar value and no
/// replacement character can appear, which preserves strict-key guarantees.
/// The five undefined code points fail closed instead of decoding silently.
///
/// # Errors
///
/// Returns the first byte that Windows-1252 leaves undefined.
pub(in crate::adapters::driven::local::two) fn windows_1252_to_string(
    bytes: &[u8],
) -> Result<String, u8> {
    let mut text = String::with_capacity(bytes.len());
    for byte in bytes {
        let decoded = match byte {
            0x00..=0x7f | 0xa0..=0xff => Some(char::from(*byte)),
            0x80..=0x9f => (*byte)
                .checked_sub(0x80)
                .and_then(|offset| WINDOWS_1252_C1.get(usize::from(offset)))
                .copied()
                .flatten(),
        };
        let Some(character) = decoded else {
            return Err(*byte);
        };
        text.push(character);
    }
    Ok(text)
}

/// Decode strict Windows-1252 after UTF-8 validation fails.
///
/// # Errors
///
/// Returns an error naming the first byte Windows-1252 leaves undefined.
fn decode_windows_1252(bytes: &[u8], source_label: &str) -> Outcome<String> {
    windows_1252_to_string(bytes).map_err(|byte| {
        Error::invalid(format!(
            "{source_label}: byte 0x{byte:02x} is not defined in \
                     Windows-1252"
        ))
    })
}

/// Treat repeated zero high bytes as bounded UTF-16LE evidence.
fn looks_like_utf16_le(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .skip(1)
        .step_by(2)
        .take(16)
        .filter(|byte| **byte == 0)
        .count()
        >= 8
}

/// Decode complete code units and reject malformed surrogate sequences.
fn decode_utf16(
    bytes: &[u8],
    source_label: &str,
    decode: fn([u8; 2]) -> u16,
) -> Outcome<String> {
    let (pairs, remainder) = bytes.as_chunks::<2>();
    if !remainder.is_empty() {
        return Err(Error::invalid(format!(
            "{source_label}: UTF-16 source has an odd byte length"
        )));
    }
    let mut units = Vec::with_capacity(pairs.len());
    for pair in pairs {
        units.push(decode(*pair));
    }
    if units.first().copied() == Some(0xfeff) {
        let _removed_bom = units.remove(0);
    }
    String::from_utf16(&units).map_err(|error| {
        Error::invalid(format!(
            "{source_label}: source is not valid UTF-16: {error}"
        ))
    })
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/two/localization/encoding/tests.rs"]
mod tests;
