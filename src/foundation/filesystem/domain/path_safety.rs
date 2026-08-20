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
//   - Path safety domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Path safety domain module.
// - Description:
//   - Implements the declared domain module responsibility for filesystem.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Path safety domain module.

use std::path::{Component, Path};

use super::RootedPathError;

/// Maximum UTF-16 code units in one portable component.
const MAX_PORTABLE_COMPONENT_UTF16_UNITS: usize = 255;

/// Reserved host stems that alias non-file destinations.
const RESERVED_HOST_STEMS: [&str; 7] =
    ["AUX", "CLOCK$", "CON", "CONIN$", "CONOUT$", "NUL", "PRN"];
/// Reserved numbered host aliases recognized by Windows.
const RESERVED_HOST_SUFFIXES: [&str; 12] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "\u{b9}", "\u{b2}", "\u{b3}",
];

/// Reports whether one portable component targets a reserved host alias.
fn is_reserved_host_alias(name: &str) -> bool {
    let stem = name
        .split('.')
        .next()
        .unwrap_or(name)
        .trim_end_matches([' ', '.'])
        .to_ascii_uppercase();
    if RESERVED_HOST_STEMS.contains(&stem.as_str()) {
        return true;
    }
    let Some(suffix) = stem
        .strip_prefix("COM")
        .or_else(|| stem.strip_prefix("LPT"))
    else {
        return false;
    };
    RESERVED_HOST_SUFFIXES.contains(&suffix)
}

/// Reports whether one component ends with a discarded trailing dot.
fn has_trailing_dot(name: &str) -> bool {
    name.ends_with('.')
}

/// Reports whether one component ends with a discarded trailing space.
fn has_trailing_space(name: &str) -> bool {
    name.ends_with(' ')
}

/// Reports whether one component selects an alternate data stream.
fn has_stream_separator(name: &str) -> bool {
    name.contains(':')
}

/// Reports whether one component contains reserved host punctuation.
fn has_forbidden_host_character(name: &str) -> bool {
    if name.contains(char::from(92)) {
        return true;
    }
    name.chars()
        .any(|character| matches!(character, '<' | '>' | '"' | '|' | '?' | '*'))
}

/// Reports whether one component contains a control character.
fn has_control_character(name: &str) -> bool {
    name.chars().any(char::is_control)
}

/// Reports whether one Unicode character can conceal path identity.
const fn is_unicode_path_modifier(character: char) -> bool {
    matches!(
        character,
        '\u{061c}'
            | '\u{200b}'..='\u{200f}'
            | '\u{2028}'..='\u{202e}'
            | '\u{2060}'..='\u{2064}'
            | '\u{2066}'..='\u{206f}'
            | '\u{fe00}'..='\u{fe0f}'
            | '\u{feff}'
    )
}

/// Reports whether one component contains an invisible path modifier.
fn has_unicode_path_modifier(name: &str) -> bool {
    name.chars().any(is_unicode_path_modifier)
}

/// Reports whether one component exceeds the portable unit limit.
fn is_component_too_long(name: &str) -> bool {
    name.encode_utf16().count() > MAX_PORTABLE_COMPONENT_UTF16_UNITS
}

/// Validates portable component identities without interpreting path policy.
///
/// # Errors
///
/// Returns [`RootedPathError`] when one component targets a reserved host
/// alias.
pub fn validate_portable_path(path: &Path) -> Result<(), RootedPathError> {
    for component in path.components() {
        let Component::Normal(value) = component else {
            continue;
        };
        let Some(name) = value.to_str() else {
            return Err(RootedPathError::NonUnicodeComponent);
        };
        if has_stream_separator(name) {
            return Err(RootedPathError::AlternateDataStream);
        }
        if has_forbidden_host_character(name) {
            return Err(RootedPathError::ForbiddenCharacter);
        }
        if has_control_character(name) {
            return Err(RootedPathError::ControlCharacter);
        }
        if has_unicode_path_modifier(name) {
            return Err(RootedPathError::UnicodePathModifier);
        }
        if is_component_too_long(name) {
            return Err(RootedPathError::ComponentTooLong);
        }
        if has_trailing_dot(name) {
            return Err(RootedPathError::TrailingDot);
        }
        if has_trailing_space(name) {
            return Err(RootedPathError::TrailingSpace);
        }
        if is_reserved_host_alias(name) {
            return Err(RootedPathError::ReservedHostAlias);
        }
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../tests/foundation/filesystem/unit/domain/path_safety/tests.rs"]
mod tests;
