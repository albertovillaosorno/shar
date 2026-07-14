// File:
//   - entities.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/text/entities.rs
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
//   - The entities contract for pipeline phase two minor units language
//   - text.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute entities.
// - Split-When:
//   - Split when entities contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Entities for pipeline phase two minor units language text.
// - Description:
//   - Defines entities data and behavior for pipeline phase two minor units
//   - language text.
// - Usage:
//   - Used by pipeline phase two minor units language text code that needs
//   - entities.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Entities for pipeline phase two minor units language text.
//!
//! This boundary keeps entities for pipeline phase two minor units language
//! text explicit and returns deterministic results to pipeline callers.
use super::matching::has_any;

/// Supports the `is_vehicle_key` operation within this deterministic
/// classification boundary.
pub(super) fn is_vehicle_key(upper: &str) -> bool {
    const EXACT_KEYS: &[&str] = &[
        // cspell:disable-next-line -- BURNSARM
        "BURNSARM",
        // cspell:disable-next-line -- CBLBART
        "CBLBART",
        // cspell:disable-next-line -- CCELLA
        "CCELLA",
        // cspell:disable-next-line -- CCELLB
        "CCELLB",
        // cspell:disable-next-line -- CCELLC
        "CCELLC",
        // cspell:disable-next-line -- CCELLD
        "CCELLD",
        // cspell:disable-next-line -- CCOLA
        "CCOLA",
        // cspell:disable-next-line -- CCUBE
        "CCUBE",
        // cspell:disable-next-line -- CCURATOR
        "CCURATOR",
        // cspell:disable-next-line -- CMILK
        "CMILK",
        // cspell:disable-next-line -- CNERD
        "CNERD",
        "COFFIN",
        // cspell:disable-next-line -- COMPACTA
        "COMPACTA",
        // cspell:disable-next-line -- CPOLICE
        "CPOLICE",
        // cspell:disable-next-line -- CVAN
        "CVAN",
        "ENGINE",
        // cspell:disable-next-line -- SPORTSA
        "SPORTSA",
        // cspell:disable-next-line -- SPORTSB
        "SPORTSB",
        "SUVA",
        "SPEED",
        "TOUGHNESS",
        "VEHICLES",
    ];
    const EXTENDED_KEYS: &[&str] = &[
        "ACCELERATION",
        // cspell:disable-next-line -- AMBUL
        "AMBUL",
        "ATV_V",
        "BIKE",
        "BUMPER",
        "BUS",
        // cspell:disable-next-line -- CDONUT
        "CDONUT",
        // cspell:disable-next-line -- CDUFF
        "CDUFF",
        // cspell:disable-next-line -- CHEARS
        "CHEARS",
        // cspell:disable-next-line -- CNONUP
        "CNONUP",
        // cspell:disable-next-line -- FISHTRUC
        "FISHTRUC",
        "GARBAGE",
        "HALLO",
        // cspell:disable-next-line -- HBIKE
        "HBIKE_V",
        // cspell:disable-next-line -- HUSKA
        "HUSKA",
        // cspell:disable-next-line -- ICECREAM
        "ICECREAM",
        // cspell:disable-next-line -- MINIVANA
        "MINIVANA",
        // cspell:disable-next-line -- PICKUPA
        "PICKUPA",
        "PIZZA",
        // cspell:disable-next-line -- SCHOOLBU
        "SCHOOLBU",
        "SHIP",
        // cspell:disable-next-line -- TAXIA
        "TAXIA",
        // cspell:disable-next-line -- WAGONA
        "WAGONA",
        // cspell:disable-next-line -- WITCHCAR
        "WITCHCAR",
    ];

    EXACT_KEYS.contains(&upper)
        || upper.starts_with("VEHICLE_")
        || upper.ends_with("_V")
        || has_any(
            upper,
            &[
                // cspell:disable-next-line -- TRUC
                "CAR", "TRUCK", "TRUC", "SEDAN", "LIMO",
            ],
        )
        || EXTENDED_KEYS.contains(&upper)
}

/// Supports the `character_category` operation within this deterministic
/// classification boundary.
pub(super) fn character_category(upper: &str) -> Option<&'static str> {
    if upper == "CHARACTER_CLOTHING" {
        return Some("clothing");
    }
    let cases = [
        (
            "A_", "apu",
        ),
        (
            "APU", "apu",
        ),
        (
            "B_", "bart",
        ),
        (
            "BART", "bart",
        ),
        (
            // cspell:disable-next-line -- BRN
            "BRN_", "barney",
        ),
        (
            "BARNEY", "barney",
        ),
        (
            "H_", "homer",
        ),
        (
            "HOMER", "homer",
        ),
        (
            "L_", "lisa",
        ),
        (
            "LISA", "lisa",
        ),
        (
            "M_", "marge",
        ),
        (
            "MARGE", "marge",
        ),
        (
            "MILHOUSE", "milhouse",
        ),
        (
            "MOE", "moe",
        ),
        (
            "WIGGUM", "wiggum",
        ),
    ];
    cases
        .iter()
        .find_map(
            |(prefix, character)| {
                (upper == *prefix || upper.starts_with(prefix))
                    .then_some(*character)
            },
        )
}
