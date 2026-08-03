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
//   - Classification outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Classification outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Classification outbound adapter.

use super::entities::{character_category, is_vehicle_key};
use super::interface::{
    is_control_key, is_menu_key, is_race_key, is_runtime_key, is_system_key,
};
use super::missions::mission_category;
use super::settings::{
    is_audio_key, is_collectible_key, is_generic_token, is_platform_key,
    is_setting_key,
};

/// Supports the `classify_text_key` operation within this deterministic
/// classification boundary.
pub(super) fn classify_text_key(key: &str) -> String {
    let upper = key.to_ascii_uppercase();
    if let Some(category) = mission_category(&upper) {
        return category;
    }
    if upper.starts_with("CARD_") || upper == "CARDS" || upper.contains("CARD")
    {
        return "language/text/cards".to_owned();
    }
    if is_vehicle_key(&upper) {
        return "language/text/vehicles".to_owned();
    }
    if let Some(character) = character_category(&upper) {
        return format!("language/text/characters/{character}");
    }
    if upper.starts_with("LEVEL_")
        || matches!(upper.as_str(), "LEVEL" | "LEVELS")
    {
        return "language/text/levels".to_owned();
    }
    if is_race_key(&upper) {
        return "language/text/races".to_owned();
    }
    if upper.starts_with("TUTORIAL_") {
        return "language/text/tutorial".to_owned();
    }
    if is_control_key(&upper) {
        return "language/text/controls".to_owned();
    }
    if is_system_key(&upper) {
        return "language/text/system/messages".to_owned();
    }
    if is_audio_key(&upper) {
        return "language/text/audio/settings".to_owned();
    }
    if is_platform_key(&upper) {
        return "language/text/platform".to_owned();
    }
    if is_collectible_key(&upper) {
        return "language/text/collectibles".to_owned();
    }
    if is_menu_key(&upper) {
        return "language/text/ui/menu".to_owned();
    }
    if is_setting_key(&upper) {
        return "language/text/settings".to_owned();
    }
    if is_runtime_key(&upper) {
        return "language/text/ui/runtime".to_owned();
    }
    if is_generic_token(&upper) {
        return "language/text/engine/tokens".to_owned();
    }
    "language/text/runtime/fallback-tokens".to_owned()
}
