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
//   - Tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Tests test module.
// - Description:
//   - Implements the declared test module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Tests test module.

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
    assert_eq!(classify_text_key("SEDANA"), "language/text/vehicles");
    assert_eq!(
        classify_text_key("H_DONUT"),
        "language/text/characters/homer"
    );
}
