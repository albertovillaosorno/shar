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
//   - Identity domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Identity domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Identity domain module.

use std::path::{Component, Path};

/// Stable scene identity for deterministic export.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SceneId {
    /// Deterministic scene id.
    pub value: String,
}

/// Return whether one identity is one portable filesystem segment.
#[must_use]
pub fn is_portable_path_segment(value: &str) -> bool {
    let mut components = Path::new(value).components();
    !value.is_empty()
        && value == value.trim()
        && !value.ends_with('.')
        && value.chars().all(|character| {
            !character.is_control() && !r#"<>:"/\|?*"#.contains(character)
        })
        && !is_windows_reserved_name(value)
        && matches!(components.next(), Some(Component::Normal(_)))
        && components.next().is_none()
}

/// Return whether one portable segment maps to a Windows device name.
#[must_use]
pub fn is_windows_reserved_name(value: &str) -> bool {
    let stem = value.split('.').next().unwrap_or(value);
    if ["con", "prn", "aux", "nul", "clock$", "conin$", "conout$"]
        .iter()
        .any(|reserved| stem.eq_ignore_ascii_case(reserved))
    {
        return true;
    }
    let Some(prefix) = stem.get(..3) else {
        return false;
    };
    let Some(suffix) = stem.get(3..) else {
        return false;
    };
    let numbered_suffix =
        matches!(suffix, "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9");
    let superscript_suffix = matches!(suffix, "¹" | "²" | "³");
    (numbered_suffix || superscript_suffix)
        && (prefix.eq_ignore_ascii_case("com")
            || prefix.eq_ignore_ascii_case("lpt"))
}
#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/formats/fbx/unit/domain/scene/identity/loose_tests.rs"]
mod loose_tests;
