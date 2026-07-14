// File:
//   - missions.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/text/missions.rs
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
//   - The missions contract for pipeline phase two minor units language
//   - text.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute missions.
// - Split-When:
//   - Split when missions contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Missions for pipeline phase two minor units language text.
// - Description:
//   - Defines missions data and behavior for pipeline phase two minor units
//   - language text.
// - Usage:
//   - Used by pipeline phase two minor units language text code that needs
//   - missions.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Missions for pipeline phase two minor units language text.
//!
//! This boundary keeps missions for pipeline phase two minor units language
//! text explicit and returns deterministic results to pipeline callers.
/// Supports the `mission_category` operation within this deterministic
/// classification boundary.
pub(super) fn mission_category(upper: &str) -> Option<String> {
    if let Some(level) = mission_level(upper) {
        if upper.starts_with("MISSION_TITLE_") {
            return Some(format!("language/text/missions/{level}/title"));
        }
        if upper.starts_with("MISSION_INFO_") {
            return Some(format!("language/text/missions/{level}/info"));
        }
        if upper.starts_with("MISSION_BRIEFING_")
            || upper.starts_with("MISSION_SUCCESS_")
        {
            return Some(format!("language/text/missions/{level}/flow"));
        }
    }
    if upper.starts_with("MISSION_OBJECTIVE_") {
        return Some("language/text/missions/objective-lines".to_owned());
    }
    if upper.starts_with("MISSION_FAILED") {
        return Some("language/text/missions/failure".to_owned());
    }
    if upper == "MISSION_COMPLETE" {
        return Some("language/text/missions/flow".to_owned());
    }
    if upper == "MISSION_SELECT" {
        return Some("language/text/ui/menu/mission-select".to_owned());
    }
    if let Some(number) = upper.strip_prefix("MISSION_")
        && number
            .chars()
            .all(|character| character.is_ascii_digit())
        && let Ok(parsed) = number.parse::<u8>()
    {
        return Some(mission_label_category(parsed));
    }
    if upper.starts_with("MISSION_INFO_WAGER") {
        return Some("language/text/missions/wager/info".to_owned());
    }
    if upper.starts_with("MISSION_INFO_") {
        return Some("language/text/missions/info-lines".to_owned());
    }
    upper
        .starts_with("MISSION_")
        .then(|| "language/text/missions/fallback-lines".to_owned())
}

/// Supports the `mission_level` operation within this deterministic
/// classification boundary.
fn mission_level(upper: &str) -> Option<String> {
    for window in upper
        .as_bytes()
        .windows(4)
    {
        if let [
            b'_',
            b'L',
            level,
            b'_',
        ] = window
            && (b'1'..=b'7').contains(level)
        {
            return Some(
                format!(
                    "level-0{}",
                    char::from(*level)
                ),
            );
        }
    }
    None
}

/// Supports the `mission_label_category` operation within this deterministic
/// classification boundary.
fn mission_label_category(value: u8) -> String {
    match value {
        0 => "language/text/missions/tutorial/label".to_owned(),
        1..=7 => format!("language/text/missions/level-{value:02}/label"),
        8..=10 => "language/text/races/street-race/label".to_owned(),
        11 => "language/text/races/wager/label".to_owned(),
        12 => "language/text/missions/bonus/label".to_owned(),
        _ => "language/text/missions/label-lines".to_owned(),
    }
}
