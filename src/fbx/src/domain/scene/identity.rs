// File:
//   - identity.rs
// Path:
//   - src/fbx/src/domain/scene/identity.rs
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
//   - Pure fbx domain rules for domain scene identity.
// - Must-Not:
//   - Read files, parse generated indexes, invoke CLI code, or call writer
//   - adapters.
// - Allows:
//   - Value objects, invariant checks, and pure evidence-to-domain translation.
// - Split-When:
//   - Split when identity contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same domain boundary with no distinct
//   - invariant.
// - Summary:
//   - Stable scene identity selected by application planning.
// - Description:
//   - Defines identity data and behavior for fbx domain scene.
// - Usage:
//   - Imported through crate domain facades or sibling domain modules.
// - Defaults:
//   - No filesystem paths, no external process calls, and no implicit IO
//   - defaults.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Stable scene identity selected by application planning.
//!
//! This boundary keeps stable scene identity selected by application planning
//! explicit and returns deterministic results to fbx callers.
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
        && value
            .chars()
            .all(
                |character| {
                    !character.is_control()
                        && !r#"<>:"/\|?*"#.contains(character)
                },
            )
        && !is_windows_reserved_name(value)
        && matches!(
            components.next(),
            Some(Component::Normal(_))
        )
        && components
            .next()
            .is_none()
}

/// Return whether one portable segment maps to a Windows device name.
#[must_use]
pub fn is_windows_reserved_name(value: &str) -> bool {
    let stem = value
        .split('.')
        .next()
        .unwrap_or(value);
    if [
        "con", "prn", "aux", "nul", "clock$", "conin$", "conout$",
    ]
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
    let numbered_suffix = matches!(
        suffix,
        "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
    );
    let superscript_suffix = matches!(
        suffix,
        "¹" | "²" | "³"
    );
    (numbered_suffix || superscript_suffix)
        && (prefix.eq_ignore_ascii_case("com")
            || prefix.eq_ignore_ascii_case("lpt"))
}

#[cfg(test)]
#[test]
fn recognizes_superscript_device_suffixes() {
    for value in [
        "COM¹.png",
        "com².json",
        "LPT³.fbx",
    ] {
        assert!(
            is_windows_reserved_name(value),
            "reserved Windows alias was accepted: {value}"
        );
    }
}
