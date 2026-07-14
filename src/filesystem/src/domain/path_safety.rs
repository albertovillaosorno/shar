// File:
//   - path_safety.rs
// Path:
//   - src/filesystem/src/domain/path_safety.rs
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
//   - Portable component identity validation for shared filesystem paths.
// - Must-Not:
//   - Perform IO, resolve containment roots, or select storage adapters.
// - Allows:
//   - Reject host aliases that cannot represent ordinary file identities.
// - Split-When:
//   - Split when another identity family has independent versioning policy.
// - Merge-When:
//   - Another domain module owns the same portable component rules.
// - Summary:
//   - Portable filesystem path safety.
// - Description:
//   - Rejects component identities reserved by Windows.
// - Usage:
//   - Called by explicit local operations and rooted path resolution.
// - Defaults:
//   - Ordinary Unicode component names remain accepted.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Portable component identity validation.
//!
//! This module rejects host aliases without performing filesystem access.
use std::path::{Component, Path};

use super::RootedPathError;

/// Maximum UTF-16 code units in one portable component.
const MAX_PORTABLE_COMPONENT_UTF16_UNITS: usize = 255;

/// Reserved host stems that alias non-file destinations.
const RESERVED_HOST_STEMS: [&str; 7] = [
    "AUX", "CLOCK$", "CON", "CONIN$", "CONOUT$", "NUL", "PRN",
];
/// Reserved numbered host aliases recognized by Windows.
const RESERVED_HOST_SUFFIXES: [&str; 12] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "¹", "²", "³",
];

/// Reports whether one portable component targets a reserved host alias.
fn is_reserved_host_alias(name: &str) -> bool {
    let stem = name
        .split('.')
        .next()
        .unwrap_or(name)
        .trim_end_matches(
            [
                ' ', '.',
            ],
        )
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
        .any(
            |character| {
                matches!(
                    character,
                    '<' | '>' | '"' | '|' | '?' | '*'
                )
            },
        )
}

/// Reports whether one component contains a control character.
fn has_control_character(name: &str) -> bool {
    name.chars()
        .any(char::is_control)
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
    name.chars()
        .any(is_unicode_path_modifier)
}

/// Reports whether one component exceeds the portable unit limit.
fn is_component_too_long(name: &str) -> bool {
    name.encode_utf16()
        .count()
        > MAX_PORTABLE_COMPONENT_UTF16_UNITS
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
mod tests {
    use super::has_forbidden_host_character;

    #[test]
    fn backslash_is_reserved_host_punctuation() {
        assert!(has_forbidden_host_character(r"folder\file.bin"));
    }
}
