// File:
//   - audio_video.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/audio_video.rs
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
//   - The audio video contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute audio video.
// - Split-When:
//   - Split when audio video contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Audio and movie package classifiers.
// - Description:
//   - Defines audio video data and behavior for pipeline phase two minor
//   - units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs audio video.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Audio and movie package classifiers keeps tightly coupled
//   - validation, ordering, and deterministic transformation invariants
//   - together; split when a stable independently testable sub-boundary is
//   - identified.
//

//! Audio and movie package classifiers.
//! Audio and movie package classifiers.

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
    music_classification(package)
        .or_else(|| sound_effect_classification(package))
        .or_else(|| movie_classification(package))
}

/// Supports the `music_classification` operation within this deterministic
/// classification boundary.
fn music_classification(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root) != PackageCategory::Music
        && !tokens_identify_music_archive(&tokens)
    {
        return None;
    }
    let bank = music_bank(&tokens);
    let role = music_role(&tokens);
    Some(
        (
            PackageCategory::Music,
            format!("music/{bank}/{role}"),
        ),
    )
}

/// Supports the `tokens_identify_music_archive` operation within this
/// deterministic classification boundary.
fn tokens_identify_music_archive(tokens: &[&str]) -> bool {
    tokens
        .iter()
        .any(|token| token.starts_with("music"))
        && tokens.contains(&"sound")
}

/// Supports the `music_bank` operation within this deterministic classification
/// boundary.
fn music_bank(tokens: &[&str]) -> &'static str {
    if tokens.contains(&"music00") {
        "bank-00"
    } else if tokens.contains(&"music01") {
        "bank-01"
    } else if tokens.contains(&"music02") {
        "bank-02"
    } else if tokens.contains(&"music03") {
        "bank-03"
    } else {
        "library"
    }
}

/// Supports the `music_role` operation within this deterministic classification
/// boundary.
fn music_role(tokens: &[&str]) -> &'static str {
    if tokens.contains(&"apu") {
        "character-apu"
    } else if tokens.contains(&"bart") {
        "character-bart"
    } else if tokens.contains(&"homer") {
        "character-homer"
    } else if tokens.contains(&"lisa") {
        "character-lisa"
    } else if tokens.contains(&"marge") {
        "character-marge"
    } else if tokens.contains(&"halloween") {
        "holiday-halloween"
    } else if tokens.contains(&"minigame") {
        "mini-game"
    } else if tokens.contains(&"generic") {
        "runtime-base"
    } else {
        "score-library"
    }
}

/// Supports the `sound_effect_classification` operation within this
/// deterministic classification boundary.
fn sound_effect_classification(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root)
        != PackageCategory::SoundEffects
        && !tokens_identify_sound_effects(&tokens)
    {
        return None;
    }
    let scope = sound_effect_scope(&tokens);
    Some(
        (
            PackageCategory::SoundEffects,
            format!("sound-effects/{scope}"),
        ),
    )
}

/// Supports the `tokens_identify_sound_effects` operation within this
/// deterministic classification boundary.
fn tokens_identify_sound_effects(tokens: &[&str]) -> bool {
    tokens
        .iter()
        .any(
            |token| {
                matches!(
                    *token,
                    "ambience" | "carsound" | "soundfx" | "typ"
                )
            },
        )
}

/// Supports the `sound_effect_scope` operation within this deterministic
/// classification boundary.
fn sound_effect_scope(tokens: &[&str]) -> String {
    if tokens.contains(&"ambience") {
        "ambience".to_owned()
    } else if tokens.contains(&"carsound") {
        format!(
            "vehicle-audio/{}",
            vehicle_sound_scope(tokens),
        )
    } else if tokens.contains(&"typ") {
        "type-metadata".to_owned()
    } else if tokens.contains(&"soundfx") {
        format!(
            "effects/{}",
            soundfx_scope(tokens),
        )
    } else {
        "runtime-base".to_owned()
    }
}

/// Supports the `vehicle_sound_scope` operation within this deterministic
/// classification boundary.
fn vehicle_sound_scope(tokens: &[&str]) -> &'static str {
    if tokens.contains(&"common") {
        "runtime-base"
    } else if tokens.contains(&"homer") {
        "character-homer"
    } else {
        "vehicle-library"
    }
}

/// Supports the `soundfx_scope` operation within this deterministic
/// classification boundary.
fn soundfx_scope(tokens: &[&str]) -> String {
    if tokens.contains(&"interactive") && tokens.contains(&"props") {
        format!(
            "interactive-props/{}",
            localized_scope(tokens),
        )
    } else if tokens.contains(&"collect") {
        "collectibles".to_owned()
    } else if tokens.contains(&"collisions") {
        "collisions".to_owned()
    } else if tokens.contains(&"feet") {
        "footsteps".to_owned()
    } else if tokens.contains(&"frontend") {
        "frontend".to_owned()
    } else if tokens.contains(&"gameplay") {
        "gameplay".to_owned()
    } else if tokens.contains(&"minigame") {
        "mini-game".to_owned()
    } else if tokens.contains(&"optionsmenu") {
        "options-menu".to_owned()
    } else if tokens.contains(&"positional") {
        "positional".to_owned()
    } else if tokens.contains(&"wasp") {
        "wasps".to_owned()
    } else if tokens.contains(&"world") && tokens.contains(&"obj") {
        "world-objects".to_owned()
    } else {
        "runtime-base".to_owned()
    }
}

/// Supports the `movie_classification` operation within this deterministic
/// classification boundary.
fn movie_classification(
    package: &MinorUnitPackage
) -> Option<(
    PackageCategory,
    String,
)> {
    let tokens = package_id_tokens(package);
    if category_from_root(&package.package_root) != PackageCategory::Movies
        && !tokens.contains(&"movies")
    {
        return None;
    }
    let scope = movie_scope(&tokens)?;
    Some(
        (
            PackageCategory::Movies,
            format!("movies/{scope}"),
        ),
    )
}

/// Supports the `movie_scope` operation within this deterministic
/// classification boundary.
fn movie_scope(tokens: &[&str]) -> Option<String> {
    let movie_index = tokens
        .iter()
        .position(|token| *token == "movies")?;
    let name = tokens
        .get(movie_index.saturating_add(1))
        .copied()?;
    if tokens.contains(&"lmlm") {
        return Some(format!("mod-audio/{name}"));
    }
    if name == "credits" {
        Some("credits".to_owned())
    } else if movie_logo_token(name) {
        Some(format!("logos/{name}"))
    } else if name == "intro" {
        Some("intro".to_owned())
    } else if name.starts_with("fmv") {
        Some(format!("story/{name}"))
    } else {
        Some(format!("extras/{name}"))
    }
}

/// Supports the `movie_logo_token` operation within this deterministic
/// classification boundary.
fn movie_logo_token(name: &str) -> bool {
    name.ends_with("logo")
        || matches!(
            name,
            "foxlogo" | "gracie"
        )
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
    use super::{movie_scope, music_bank, music_role, sound_effect_scope};

    #[test]
    fn classifies_music_banks_and_roles() {
        assert_eq!(
            music_bank(
                &[
                    "extracted",
                    "music02"
                ]
            ),
            "bank-02"
        );
        assert_eq!(
            music_role(
                &[
                    "music", "generic"
                ]
            ),
            "runtime-base"
        );
        assert_eq!(
            music_role(
                &[
                    "music",
                    "halloween"
                ]
            ),
            "holiday-halloween"
        );
    }

    #[test]
    fn classifies_sound_effect_scopes() {
        assert_eq!(
            sound_effect_scope(
                &[
                    "extracted",
                    "soundfx",
                    "interactive",
                    "props",
                    "spanish"
                ]
            ),
            "effects/interactive-props/spanish".to_owned()
        );
        assert_eq!(
            sound_effect_scope(
                &[
                    "extracted",
                    "carsound",
                    "common"
                ]
            ),
            "vehicle-audio/runtime-base".to_owned()
        );
    }

    #[test]
    fn classifies_movie_scopes() {
        assert_eq!(
            movie_scope(
                &[
                    "extracted",
                    "movies",
                    "fmv4"
                ]
            ),
            Some("story/fmv4".to_owned())
        );
        assert_eq!(
            movie_scope(
                &[
                    "extracted",
                    "movies",
                    "radlogo"
                ]
            ),
            Some("logos/radlogo".to_owned())
        );
        assert_eq!(
            movie_scope(
                &[
                    "extracted",
                    "lmlm",
                    "movies",
                    "intro"
                ]
            ),
            Some("mod-audio/intro".to_owned())
        );
    }
}
