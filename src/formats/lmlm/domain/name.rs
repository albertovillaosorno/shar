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
//   - Name domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Name domain module.
// - Description:
//   - Implements the declared domain module responsibility for lmlm.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Name domain module.

#![expect(
    clippy::redundant_pub_crate,
    reason = "sibling parser modules require crate-visible contracts while \
              the private module prevents external API exposure"
)]

use std::collections::BTreeMap;

use super::LmlmError;
use super::binary::{checked_slice, first_nonzero_byte};
use super::layout::BLOCK;

/// Maximum component length under the legacy Windows path contract.
const MAX_PORTABLE_COMPONENT_UTF16_UNITS: usize = 255;
/// Maximum relative path length under the legacy Windows path contract.
const MAX_PORTABLE_PATH_UTF16_UNITS: usize = 259;

/// Decodes the UTF-16LE name stored in the name block at `pos`.
pub(crate) fn read_name(data: &[u8], pos: usize) -> Result<String, LmlmError> {
    let Some(block) = pos.checked_add(2).and_then(|start| {
        BLOCK
            .checked_sub(2)
            .and_then(|length| checked_slice(data, start, length))
    }) else {
        return Err(LmlmError::Truncated);
    };
    let mut units: Vec<u16> = Vec::new();
    let mut index = 0;
    let mut terminated = false;
    while let Some(pair) = checked_slice(block, index, 2) {
        let Ok(bytes) = <[u8; 2]>::try_from(pair) else {
            break;
        };
        let unit = u16::from_le_bytes(bytes);
        if unit == 0 {
            terminated = true;
            break;
        }
        units.push(unit);
        index = index.checked_add(2).ok_or(LmlmError::Truncated)?;
    }
    if !terminated {
        return Err(LmlmError::UnterminatedName { offset: pos });
    }
    let padding_start = index.checked_add(2).ok_or(LmlmError::Truncated)?;
    let padding_len = block
        .len()
        .checked_sub(padding_start)
        .ok_or(LmlmError::Truncated)?;
    let archive_padding_start = pos
        .checked_add(2)
        .and_then(|start| start.checked_add(padding_start))
        .ok_or(LmlmError::Truncated)?;
    if let Some((offset, value)) =
        first_nonzero_byte(data, archive_padding_start, padding_len)?
    {
        return Err(LmlmError::NonZeroNamePadding { offset, value });
    }
    String::from_utf16(&units).map_err(|error| LmlmError::InvalidNameEncoding {
        offset: pos,
        message: error.to_string(),
    })
}

/// Returns whether a component targets a reserved Windows device identity.
fn reserved_device_name(name: &str) -> bool {
    let stem = name
        .split('.')
        .next()
        .unwrap_or(name)
        .trim_end_matches([' ', '.'])
        .to_ascii_uppercase();
    if matches!(
        stem.as_str(),
        "CON" | "PRN" | "AUX" | "NUL" | "CONIN$" | "CONOUT$"
    ) {
        return true;
    }
    for prefix in ["COM", "LPT"] {
        let Some(suffix) = stem.strip_prefix(prefix) else {
            continue;
        };
        if matches!(
            suffix,
            "1" | "2"
                | "3"
                | "4"
                | "5"
                | "6"
                | "7"
                | "8"
                | "9"
                | "¹"
                | "²"
                | "³"
        ) {
            return true;
        }
    }
    false
}

/// Returns whether a Unicode format character can conceal path identity.
const fn unicode_path_modifier(character: char) -> bool {
    matches!(
        character,
        '\u{061c}'
            | '\u{200b}'..='\u{200f}'
            | '\u{202a}'..='\u{202e}'
            | '\u{2060}'..='\u{2064}'
            | '\u{2066}'..='\u{206f}'
            | '\u{feff}'
    )
}

/// Rejects components that cannot be represented consistently by extraction.
fn safe_component(name: &str) -> bool {
    let utf16_units = name.encode_utf16().count();
    utf16_units <= MAX_PORTABLE_COMPONENT_UTF16_UNITS
        && !name.is_empty()
        && name != ".."
        && name != "."
        && !name.contains('/')
        && !name.contains('\\')
        && !name.chars().any(|character| {
            character.is_control()
                || unicode_path_modifier(character)
                || matches!(character, '<' | '>' | ':' | '"' | '|' | '?' | '*')
        })
        && !name.ends_with('.')
        && !name.ends_with(' ')
        && !reserved_device_name(name)
}

/// Returns whether a complete archive path is portable for extraction.
pub(crate) fn portable_path_is_safe(path: &str) -> bool {
    !path.is_empty()
        && path.encode_utf16().count() <= MAX_PORTABLE_PATH_UTF16_UNITS
        && path.split('/').all(safe_component)
}

/// Produces a locale-independent case-insensitive portable path identity.
pub(crate) fn portable_identity(path: &str) -> String {
    path.chars().flat_map(char::to_uppercase).collect()
}

/// Registers one safe archive path and rejects portable collisions.
pub(crate) fn register_path(
    name: String,
    prefix: &str,
    seen_paths: &mut BTreeMap<String, String>,
) -> Result<String, LmlmError> {
    if !safe_component(&name) {
        return Err(LmlmError::UnsafePath(name));
    }
    let full_path = format!("{prefix}{name}");
    if full_path.encode_utf16().count() > MAX_PORTABLE_PATH_UTF16_UNITS {
        return Err(LmlmError::UnsafePath(full_path));
    }
    let portable_identity = portable_identity(&full_path);
    if let Some(first_path) =
        seen_paths.insert(portable_identity, full_path.clone())
    {
        return Err(LmlmError::PathCollision {
            first_path,
            second_path: full_path,
        });
    }
    Ok(full_path)
}
