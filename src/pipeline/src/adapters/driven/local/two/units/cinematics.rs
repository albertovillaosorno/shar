// File:
//   - cinematics.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/cinematics.rs
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
//   - The cinematics contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute cinematics.
// - Split-When:
//   - Split when cinematics contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Cinematic package classifier.
// - Description:
//   - Defines cinematics data and behavior for pipeline phase two minor
//   - units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs cinematics.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Cinematic package classifier.
//!
//! This boundary keeps cinematic package classifier explicit and returns
//! deterministic results to pipeline callers.

use super::index::{
    MinorUnitPackage, PackageCategory, category_from_root, package_id_tokens,
};

/// Supports the `classification_from_package` operation within this
/// deterministic classification boundary.
pub(super) fn classification_from_package(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root) != PackageCategory::Cinematics
        && !tokens.contains(&"nis")
    {
        return None;
    }
    let subcategory = if tokens.contains(&"sound") {
        audio_subcategory(&tokens)
    } else if tokens.contains(&"gags") {
        gag_subcategory(&tokens)?
    } else {
        return Some(
            (
                PackageCategory::Error,
                "error/missing-cinematic-scope".to_owned(),
            ),
        );
    };
    Some(
        (
            PackageCategory::Cinematics,
            subcategory,
        ),
    )
}

/// Supports the `audio_subcategory` operation within this deterministic
/// classification boundary.
fn audio_subcategory(tokens: &[&str]) -> String {
    format!(
        "cinematics/nis-audio/{}",
        localized_scope(tokens),
    )
}

/// Supports the `gag_subcategory` operation within this deterministic
/// classification boundary.
fn gag_subcategory(tokens: &[&str]) -> Option<String> {
    let gag_index = tokens
        .iter()
        .position(|token| *token == "gags")?;
    let tail = tokens.get(gag_index.saturating_add(1)..)?;
    let first = tail
        .first()
        .copied()?;
    if let Some(level) = explicit_level_scope(first) {
        return Some(
            explicit_level_gag_subcategory(
                &level, tail,
            ),
        );
    }
    if let Some(series) = numbered_gag_series(first) {
        return Some(
            format!("cinematics/gags/series-{series}/numbered/{first}"),
        );
    }
    named_gag_scene_subcategory(tail)
}

/// Supports the `explicit_level_gag_subcategory` operation within this
/// deterministic classification boundary.
fn explicit_level_gag_subcategory(
    level: &str,
    tail: &[&str],
) -> String {
    if tail.contains(&"dump") {
        format!("cinematics/gags/{level}/dump-scenes")
    } else if let Some(scene) = tail.get(1) {
        format!("cinematics/gags/{level}/named/{scene}")
    } else {
        format!("cinematics/gags/{level}/named/source-metadata")
    }
}

/// Supports the `named_gag_scene_subcategory` operation within this
/// deterministic classification boundary.
fn named_gag_scene_subcategory(tail: &[&str]) -> Option<String> {
    let scene_tokens = tail
        .strip_prefix(&["gag"])
        .unwrap_or(tail);
    (!scene_tokens.is_empty()).then(
        || {
            format!(
                "cinematics/gags/named/{}",
                scene_tokens.join("-")
            )
        },
    )
}

/// Supports the `explicit_level_scope` operation within this deterministic
/// classification boundary.
fn explicit_level_scope(token: &str) -> Option<String> {
    token
        .strip_prefix('l')
        .and_then(
            |raw| {
                raw.parse::<u8>()
                    .ok()
            },
        )
        .filter(|level| (1..=7).contains(level))
        .map(|level| format!("level-{level:02}"))
}

/// Supports the `numbered_gag_series` operation within this deterministic
/// classification boundary.
fn numbered_gag_series(token: &str) -> Option<String> {
    let raw = token.strip_prefix("gag")?;
    let prefix = raw.get(0..2)?;
    prefix
        .chars()
        .all(|character| character.is_ascii_digit())
        .then(|| prefix.to_owned())
}

/// Supports the `localized_scope` operation within this deterministic
/// classification boundary.
fn localized_scope(tokens: &[&str]) -> &'static str {
    if tokens.contains(&"french") {
        "french"
    } else if tokens.contains(&"german") {
        "german"
    } else if tokens.contains(&"spanish") {
        "spanish"
    } else {
        "default"
    }
}

#[cfg(test)]
mod tests {
    use super::{
        gag_subcategory, named_gag_scene_subcategory, numbered_gag_series,
    };

    #[test]
    fn classifies_explicit_level_gags() {
        assert_eq!(
            gag_subcategory(
                &[
                    "extracted",
                    "art",
                    "nis",
                    "gags",
                    "l04",
                    // cspell:disable-next-line -- azte
                    "azte"
                ]
            ),
            // cspell:disable-next-line -- azte
            Some("cinematics/gags/level-04/named/azte".to_owned())
        );
        assert_eq!(
            gag_subcategory(
                &[
                    "extracted",
                    "art",
                    "nis",
                    "gags",
                    "l2",
                    "dump"
                ]
            ),
            Some("cinematics/gags/level-02/dump-scenes".to_owned())
        );
    }

    #[test]
    fn classifies_named_gag_scene_codes() {
        assert_eq!(
            named_gag_scene_subcategory(
                &[
                    // cspell:disable-next-line -- alm
                    "gag", "alm2"
                ]
            ),
            // cspell:disable-next-line -- alm
            Some("cinematics/gags/named/alm2".to_owned())
        );
        assert_eq!(
            named_gag_scene_subcategory(
                &[
                    "gag", "k", "h1"
                ]
            ),
            Some("cinematics/gags/named/k-h1".to_owned())
        );
        assert_eq!(
            gag_subcategory(
                &[
                    "extracted",
                    "art",
                    "nis",
                    "gags",
                    "gag",
                    "bbq"
                ]
            ),
            Some("cinematics/gags/named/bbq".to_owned())
        );
    }

    #[test]
    fn classifies_numbered_gag_series() {
        assert_eq!(
            numbered_gag_series("gag0207"),
            Some("02".to_owned())
        );
        assert_eq!(
            gag_subcategory(
                &[
                    "extracted",
                    "art",
                    "nis",
                    "gags",
                    "gag0207"
                ]
            ),
            Some("cinematics/gags/series-02/numbered/gag0207".to_owned())
        );
    }
}
