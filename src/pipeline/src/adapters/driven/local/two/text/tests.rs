// File:
//   - tests.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/text/tests.rs
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
//   - The tests contract for pipeline phase two minor units language text.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute tests.
// - Split-When:
//   - Split when tests contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Tests for pipeline phase two minor units language text.
// - Description:
//   - Defines tests data and behavior for pipeline phase two minor units
//   - language text.
// - Usage:
//   - Used by pipeline phase two minor units language text code that needs
//   - tests.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - false
//

//! Tests for pipeline phase two minor units language text.
//!
//! This boundary keeps tests for pipeline phase two minor units language text
//! explicit and returns deterministic results to pipeline callers.
use super::classification::classify_text_key;

#[test]
fn classifies_level_mission_text_without_file_routes() {
    assert_eq!(
        classify_text_key("MISSION_TITLE_L4_M7"),
        "language/text/missions/level-04/title"
    );
    assert_eq!(
        classify_text_key("MISSION_INFO_L2_M10"),
        "language/text/missions/level-02/info"
    );
}

#[test]
fn classifies_global_objectives_without_guessing_levels() {
    assert_eq!(
        classify_text_key("MISSION_OBJECTIVE_42"),
        "language/text/missions/objective-lines"
    );
}

#[test]
fn classifies_vehicle_and_costume_keys() {
    assert_eq!(
        // cspell:disable-next-line -- SEDANA
        classify_text_key("SEDANA"),
        "language/text/vehicles"
    );
    assert_eq!(
        classify_text_key("H_DONUT"),
        "language/text/characters/homer"
    );
}
