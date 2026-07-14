// File:
//   - interface.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/text/interface.rs
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
//   - The interface contract for pipeline phase two minor units language
//   - text.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute interface.
// - Split-When:
//   - Split when interface contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Interface for pipeline phase two minor units language text.
// - Description:
//   - Defines interface data and behavior for pipeline phase two minor units
//   - language text.
// - Usage:
//   - Used by pipeline phase two minor units language text code that needs
//   - interface.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Interface for pipeline phase two minor units language text.
//!
//! This boundary keeps interface for pipeline phase two minor units language
//! text explicit and returns deterministic results to pipeline callers.
use super::matching::has_any;

/// Supports the `is_race_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_race_key(upper: &str) -> bool {
    const RACE_KEYS: &[&str] = &[
        "ENTRY_FEE",
        "GAMBLING_RACE_FAILURE",
        "GAMBLING_RACE_SUCCESS",
        // cspell:disable-next-line -- NBT
        "GAMBLING_RACE_SUCCESS_NBT",
        "NUMBER_OF_FLAGS",
    ];
    const RESULT_KEYS: &[&str] = &[
        "1ST",
        "2ND",
        "3RD",
        "4TH",
        "DNF",
        "GO",
        "LAPS",
        "PAYOUT",
        "POINTS",
        "STREET_RACES",
        "TIME_TO_BEAT",
        "TOTAL_TIME",
        "WINS",
    ];

    RACE_KEYS.contains(&upper)
        || upper.starts_with("RACE_")
        || upper.ends_with("_LAPS")
        || RESULT_KEYS.contains(&upper)
}

/// Supports the `is_control_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_control_key(upper: &str) -> bool {
    upper == "KEYBOARD"
        || matches!(
            upper,
            "CHANGE_TRACK_DIRECTION"
                | "CHARACTER_CONTROLS"
                | "CHARACTER_CONTROLS_CAPS"
                | "CLICK_ENTER_FIRST_PERSON"
                | "ENTER_FIRST_PERSON"
                | "INTERSECT_NAV_SYSTEM"
                | "MOVE_CHARACTER"
                | "ON_FOOT"
                | "OPEN_BOOK"
                | "REPAIR_VEHICLE"
                | "WHEEL_SENSITIVITY"
                | "ZOOM_FOV"
        )
        || upper.starts_with("VINPUT_")
        || upper.starts_with("INPUT_")
        || upper.starts_with("GAMEPAD_")
        || upper.starts_with("MOUSE_")
        || upper.starts_with("KEYBOARD_")
        || upper.starts_with("STEERING_WHEEL_")
        || has_any(
            upper,
            &[
                "ACCELERATE",
                "ACTION",
                "ATTACK",
                "BRAKE",
                "BUTTON",
                "CAM",
                "DASH",
                "HORN",
                "JUMP",
                "LOOK",
                "MOUSE",
                "REVERSE",
                "ROTATE",
                "TURN",
                "WALK",
            ],
        )
}

/// Supports the `is_menu_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_menu_key(upper: &str) -> bool {
    matches!(
        upper,
        "APPLY_CHANGES"
            | "CHANGE"
            | "EXIT_GAME"
            | "PRESS_START"
            | "PRESS_START_GC"
            | "PRESS_START_PS2"
            | "PRESS_START_XBOX"
            | "RESTART_MISSION"
    ) || has_any(
        upper,
        &[
            "ABORT",
            "ACCEPT",
            "BACK",
            "BUY",
            "CANCEL",
            "CLOTHES",
            "CONTINUE",
            "DELETE",
            "LOAD",
            "MENU",
            "NEW_GAME",
            "OPTIONS",
            "PAUSE",
            "PURCHASE",
            "QUIT",
            "RESTORE",
            "RESUME",
            "RETRY",
            "SAVE",
            "SCRAP_BOOK",
            "SELECT",
            "SKIP",
        ],
    )
}

/// Supports the `is_system_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_system_key(upper: &str) -> bool {
    matches!(
        upper,
        "NO_MEMORY_DEVICE_(GC)" | "NO_MEMORY_DEVICE_(PS2)" | "INSERT_CD"
    ) || upper.starts_with("ERROR_")
        || upper.starts_with("MSG_")
        || upper.starts_with("MEMCARD_")
        || upper.starts_with("FORMAT")
        || upper.starts_with("CHECKING_")
        || has_any(
            upper,
            &[
                "CORRUPT",
                "EMPTY_SLOT",
                "FREE_SPACE",
                "MEDIA_ERROR",
                "MEM_CARD",
                "NO_MEMORY_DEVICE",
                "INSERT_CD",
                "NOT_AVAILABLE",
                "NOT_USED",
            ],
        )
}

/// Supports the `is_runtime_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_runtime_key(upper: &str) -> bool {
    matches!(
        upper,
        "BONUS_MISSIONS"
            | "BONUS_MISSION_COMPLETE"
            | "BUSTED"
            | "DRIVE"
            | "HIT_AND_RUN"
            | "IS_MOVIE_UNLOCKED"
            | "ITEM_DROPPED"
            | "RADAR"
            | "STORY_MISSIONS"
            | "TUTORIAL"
            | "WAITING_FOR_OTHER_PLAYERS"
    ) || upper.starts_with("LOADING_")
        || upper.starts_with("MOVIE_")
        || upper.starts_with("CREDITS_")
        || upper.starts_with("INGAME_")
        || upper.starts_with("HUD_")
        || upper.starts_with("MINI_")
        || upper.starts_with("ORDINAL_")
        || has_any(
            upper,
            &[
                "BEST",
                "DISPLAY",
                "FULL",
                "GAME_COMPLETE",
                "GAME_COMPLETION",
                "GAME_STATS",
                "BONUS_MISSION",
                "BONUS_MISSIONS",
                "BUSTED",
                "HIT_AND_RUN",
                "INTRO_MOVIE",
                "MOVIES",
                "PRIMARY",
                "RUNNING_TOTALS",
                "STAGE_COMPLETE",
                "STORY_MISSIONS",
                "ITEM_DROPPED",
                "RADAR",
                "WAITING_FOR_OTHER_PLAYERS",
                "STATUS",
                "VIEW",
            ],
        )
}
