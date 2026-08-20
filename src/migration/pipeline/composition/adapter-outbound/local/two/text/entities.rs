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
//   - Entities outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Entities outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Entities outbound adapter.

use super::matching::has_any;

/// Supports the `is_vehicle_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_vehicle_key(upper: &str) -> bool {
    const EXACT_KEYS: &[&str] = &[
        "BURNSARM",
        "CBLBART",
        "CCELLA",
        "CCELLB",
        "CCELLC",
        "CCELLD",
        "CCOLA",
        "CCUBE",
        "CCURATOR",
        "CMILK",
        "CNERD",
        "COFFIN",
        "COMPACTA",
        "CPOLICE",
        "CVAN",
        "ENGINE",
        "SPORTSA",
        "SPORTSB",
        "SUVA",
        "SPEED",
        "TOUGHNESS",
        "VEHICLES",
    ];
    const EXTENDED_KEYS: &[&str] = &[
        "ACCELERATION",
        "AMBUL",
        "ATV_V",
        "BIKE",
        "BUMPER",
        "BUS",
        "CDONUT",
        "CDUFF",
        "CHEARS",
        "CNONUP",
        "FISHTRUC",
        "GARBAGE",
        "HALLO",
        "HBIKE_V",
        "HUSKA",
        "ICECREAM",
        "MINIVANA",
        "PICKUPA",
        "PIZZA",
        "SCHOOLBU",
        "SHIP",
        "TAXIA",
        "WAGONA",
        "WITCHCAR",
    ];

    EXACT_KEYS.contains(&upper)
        || upper.starts_with("VEHICLE_")
        || upper.ends_with("_V")
        || has_any(upper, &["CAR", "TRUCK", "TRUC", "SEDAN", "LIMO"])
        || EXTENDED_KEYS.contains(&upper)
}

/// Supports the `character_category` operation within this deterministic
/// classification boundary.
pub(super) fn character_category(upper: &str) -> Option<&'static str> {
    if upper == "CHARACTER_CLOTHING" {
        return Some("clothing");
    }
    let cases = [
        ("A_", "apu"),
        ("APU", "apu"),
        ("B_", "bart"),
        ("BART", "bart"),
        ("BRN_", "barney"),
        ("BARNEY", "barney"),
        ("H_", "homer"),
        ("HOMER", "homer"),
        ("L_", "lisa"),
        ("LISA", "lisa"),
        ("M_", "marge"),
        ("MARGE", "marge"),
        ("MILHOUSE", "milhouse"),
        ("MOE", "moe"),
        ("WIGGUM", "wiggum"),
    ];
    cases.iter().find_map(|(prefix, character)| {
        (upper == *prefix || upper.starts_with(prefix)).then_some(*character)
    })
}
