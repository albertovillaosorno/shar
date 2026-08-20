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
//   - Settings outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Settings outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Settings outbound adapter.

use super::matching::has_any;

/// Supports the `is_audio_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_audio_key(upper: &str) -> bool {
    upper == "MUSIC"
        || upper.starts_with("MU_")
        || upper.starts_with("MUSIC_")
        || has_any(upper, &[
            "EFFECTS",
            "RUMBLE",
            "SOUND",
            "VIBRATION",
            "VOICE",
            "VOLUME",
        ])
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
    has_any(upper, &["CARD", "GAG", "WASP"])
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
    ) || has_any(upper, &[
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
    ]) || matches!(upper, "NO" | "OFF" | "OK" | "ON" | "START" | "YES")
}

/// Supports the `is_generic_token` operation within this deterministic
/// classification boundary.
pub(super) fn is_generic_token(upper: &str) -> bool {
    matches!(
        upper,
        "AUTO" | "BASE" | "BLOCKS" | "COINS" | "COLON" | "SPACE"
    ) || upper.starts_with("UNUSED_STRING_")
        || upper.chars().all(|character| character.is_ascii_digit())
        || upper.starts_with("0X")
        || upper.len() == 1
            && upper
                .chars()
                .all(|character| character.is_ascii_alphabetic())
        || matches!(
            upper,
            "!!!" | "???" | "FUNCTION_BUTTON" | "HAHA" | "PSYCHE" | "SLASH"
        )
}
