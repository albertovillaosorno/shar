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
//   - Unit evidence for authored countdown block relationships.
// - Must-Not:
//   - Interpret countdown display tokens or runtime playback behavior.
// - Allows:
//   - Verify one start and ordered entries per stage.
// - Split-When:
//   - Runtime countdown behavior requires independent fixtures.
// - Merge-When:
//   - Complete stage graph tests own countdown relationships.
// - Summary:
//   - Authored countdown block unit tests.
// - Description:
//   - Locks source ordering without inventing countdown-token meaning.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Orphan entries and duplicate starts fail closed.
//

//! Unit evidence for authored countdown blocks.

use super::*;

fn stages(
    directives: Vec<MissionStageDirective>,
) -> MissionStageSemanticReport {
    MissionStageSemanticReport::from_topology_entries_for_tests(vec![(
        2, 0, false, directives,
    )])
}

#[test]
fn binds_ordered_entries_to_prior_start() -> Result<(), String> {
    let report = preflight_mission_countdowns(&stages(vec![
        MissionStageDirective::StartCountdown {
            source_ordinal: 4,
            sequence_id: "count".to_owned(),
            character_id: Some("lisa".to_owned()),
        },
        MissionStageDirective::CountdownSequenceEntry {
            source_ordinal: 5,
            token: "3".to_owned(),
            duration_milliseconds: 1_000,
        },
        MissionStageDirective::CountdownSequenceEntry {
            source_ordinal: 6,
            token: "GO".to_owned(),
            duration_milliseconds: 400,
        },
    ]))?;
    let [countdown] = report.countdowns() else {
        return Err("countdown block count drifted".to_owned());
    };
    assert_eq!(countdown.stage_source_ordinal(), 2);
    assert_eq!(countdown.stage_sequence_ordinal(), 0);
    assert_eq!(countdown.start_source_ordinal(), 4);
    assert_eq!(countdown.sequence_id(), "count");
    assert_eq!(countdown.character_id(), Some("lisa"));
    let [three, go] = countdown.entries() else {
        return Err("countdown entry count drifted".to_owned());
    };
    assert_eq!(three.source_ordinal(), 5);
    assert_eq!(three.token(), "3");
    assert_eq!(three.duration_milliseconds(), 1_000);
    assert_eq!(go.token(), "GO");
    Ok(())
}

#[test]
fn rejects_orphan_countdown_entry() -> Result<(), String> {
    let result = preflight_mission_countdowns(&stages(vec![
        MissionStageDirective::CountdownSequenceEntry {
            source_ordinal: 4,
            token: "3".to_owned(),
            duration_milliseconds: 1_000,
        },
    ]));
    let Err(error) = result else {
        return Err("orphan countdown entry unexpectedly passed".to_owned());
    };
    assert!(error.contains("precedes countdown start"));
    Ok(())
}

#[test]
fn rejects_duplicate_countdown_start() -> Result<(), String> {
    let first = MissionStageDirective::StartCountdown {
        source_ordinal: 4,
        sequence_id: "count".to_owned(),
        character_id: None,
    };
    let second = MissionStageDirective::StartCountdown {
        source_ordinal: 5,
        sequence_id: "count".to_owned(),
        character_id: None,
    };
    let result = preflight_mission_countdowns(&stages(vec![first, second]));
    let Err(error) = result else {
        return Err("duplicate countdown start unexpectedly passed".to_owned());
    };
    assert!(error.contains("more than one countdown"));
    Ok(())
}
