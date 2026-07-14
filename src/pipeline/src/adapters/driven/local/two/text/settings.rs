// File:
//   - settings.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/text/settings.rs
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
//   - The settings contract for pipeline phase two minor units language
//   - text.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute settings.
// - Split-When:
//   - Split when settings contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Settings for pipeline phase two minor units language text.
// - Description:
//   - Defines settings data and behavior for pipeline phase two minor units
//   - language text.
// - Usage:
//   - Used by pipeline phase two minor units language text code that needs
//   - settings.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Settings for pipeline phase two minor units language text.
//!
//! This boundary keeps settings for pipeline phase two minor units language
//! text explicit and returns deterministic results to pipeline callers.
use super::matching::has_any;

/// Supports the `is_audio_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_audio_key(upper: &str) -> bool {
    upper == "MUSIC"
        || upper.starts_with("MU_")
        || upper.starts_with("MUSIC_")
        || has_any(
            upper,
            &[
                "EFFECTS",
                "RUMBLE",
                "SOUND",
                "VIBRATION",
                "VOICE",
                "VOLUME",
            ],
        )
}

/// Supports the `is_platform_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_platform_key(upper: &str) -> bool {
    upper.starts_with("XBOX_")
        || upper.starts_with("PS2_")
        || upper.starts_with("GC_")
        || upper.contains("(XBOX)")
}

/// Supports the `is_collectible_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_collectible_key(upper: &str) -> bool {
    has_any(
        upper,
        &[
            "CARD", "GAG", "WASP",
        ],
    )
}

/// Supports the `is_setting_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_setting_key(upper: &str) -> bool {
    matches!(
        upper,
        "32_BIT"
            | "COLOUR DEPTH"
            | "CONTROLLER"
            | "DISABLE_TUTORIAL"
            | "SPANISH"
            | "WINDOW"
    ) || has_any(
        upper,
        &[
            "16_BIT",
            "32_BIT",
            "ALTERNATE",
            "CONFIG",
            "CONTROLLER",
            "COLOUR DEPTH",
            "DEMO",
            "ENGLISH",
            "FRENCH",
            "GAMMA",
            "GERMAN",
            "INTERLACED",
            "INVERT",
            "ITALIAN",
            "KMH",
            "MPH",
            "MULTIPLAYER",
            "SPANISH",
            "WINDOW",
            "PROGRESSIVE",
            "READY",
            "RESOLUTION",
            "SETTINGS",
            "STABILITY",
            "SURROUND",
            "WIDESCREEN",
        ],
    ) || matches!(
        upper,
        "NO" | "OFF" | "OK" | "ON" | "START" | "YES"
    )
}

/// Supports the `is_generic_token` operation within this deterministic
/// classification boundary.
pub(super) fn is_generic_token(upper: &str) -> bool {
    matches!(
        upper,
        "AUTO" | "BASE" | "BLOCKS" | "COINS" | "COLON" | "SPACE"
    ) || upper.starts_with("UNUSED_STRING_")
        || upper
            .chars()
            .all(|character| character.is_ascii_digit())
        || upper.starts_with("0X")
        || upper.len() == 1
            && upper
                .chars()
                .all(|character| character.is_ascii_alphabetic())
        || matches!(
            upper,
            // cspell:disable-next-line -- HAHA
            "!!!" | "???" | "FUNCTION_BUTTON" | "HAHA" | "PSYCHE" | "SLASH"
        )
}
