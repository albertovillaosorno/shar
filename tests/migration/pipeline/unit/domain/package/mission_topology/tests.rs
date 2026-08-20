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
//   - Unit evidence for authored mission-stage topology invariants.
// - Must-Not:
//   - Treat authored adjacency as runtime transition behavior.
// - Allows:
//   - Verify explicit final/terminal placement and authored neighbors.
// - Split-When:
//   - Runtime successor graph tests gain source-backed semantics.
// - Merge-When:
//   - Authored topology tests move into complete mission graph tests.
// - Summary:
//   - Authored mission-stage topology unit tests.
// - Description:
//   - Locks conservative stage ordering without inventing runtime flow.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Missing final markers are accepted and contradictions fail closed.
//

//! Unit evidence for authored mission-stage topology.

use super::*;
use crate::domain::MissionStageDirective;

fn semantics(
    entries: Vec<(usize, usize, bool, Vec<MissionStageDirective>)>,
) -> MissionStageSemanticReport {
    MissionStageSemanticReport::from_topology_entries_for_tests(entries)
}

fn rejection<T>(
    result: Result<T, String>,
    context: &str,
) -> Result<String, String> {
    match result {
        Ok(_value) => Err(format!("{context} unexpectedly passed")),
        Err(error) => Ok(error),
    }
}

#[test]
fn exposes_authored_neighbors_and_last_final_marker() -> Result<(), String> {
    let report = preflight_mission_authored_stage_topology(&semantics(vec![
        (2, 0, false, vec![]),
        (
            5,
            1,
            false,
            vec![MissionStageDirective::ResetCheckpoint { source_ordinal: 6 }],
        ),
        (8, 2, true, vec![]),
    ]))?;
    let [first, second, third] = report.stages() else {
        return Err("authored topology stage count changed".to_owned());
    };
    assert_eq!(first.source_ordinal(), 2);
    assert_eq!(first.sequence_ordinal(), 0);
    assert_eq!(first.next_authored_sequence_ordinal(), Some(1));
    assert_eq!(second.next_authored_sequence_ordinal(), Some(2));
    assert_eq!(second.checkpoint_source_ordinal(), Some(6));
    assert_eq!(third.next_authored_sequence_ordinal(), None);
    assert_eq!(third.checkpoint_source_ordinal(), None);
    assert!(!first.explicit_final());
    assert!(third.explicit_final());
    Ok(())
}

#[test]
fn accepts_sequence_without_explicit_final_marker() -> Result<(), String> {
    let report = preflight_mission_authored_stage_topology(&semantics(vec![
        (2, 0, false, vec![]),
        (5, 1, false, vec![]),
    ]))?;
    assert!(report.stages().iter().all(|stage| !stage.explicit_final()));
    Ok(())
}

#[test]
fn rejects_duplicate_checkpoint_markers_inside_one_stage(
) -> Result<(), String> {
    let result = preflight_mission_authored_stage_topology(&semantics(vec![(
        2,
        0,
        false,
        vec![
            MissionStageDirective::ResetCheckpoint { source_ordinal: 3 },
            MissionStageDirective::ResetCheckpoint { source_ordinal: 4 },
        ],
    )]));
    let error = rejection(result, "duplicate checkpoint marker fixture")?;
    assert!(error.contains("more than one checkpoint marker"));
    Ok(())
}

#[test]
fn rejects_non_last_explicit_final_marker() -> Result<(), String> {
    let result = preflight_mission_authored_stage_topology(&semantics(vec![
        (2, 0, true, vec![]),
        (5, 1, false, vec![]),
    ]));
    let error = rejection(result, "non-last final marker fixture")?;
    assert!(error.contains("final stage is not authored last"));
    Ok(())
}

#[test]
fn rejects_non_last_explicit_terminal_marker() -> Result<(), String> {
    let result = preflight_mission_authored_stage_topology(&semantics(vec![
        (
            2,
            0,
            false,
            vec![MissionStageDirective::LevelOver { source_ordinal: 3 }],
        ),
        (5, 1, false, vec![]),
    ]));
    let error = rejection(result, "non-last terminal marker fixture")?;
    assert!(error.contains("terminal stage is not authored last"));
    Ok(())
}

#[test]
fn rejects_non_dense_or_non_increasing_authored_order() -> Result<(), String> {
    let sparse = preflight_mission_authored_stage_topology(&semantics(vec![
        (2, 0, false, vec![]),
        (5, 2, false, vec![]),
    ]));
    assert!(rejection(sparse, "sparse order fixture")?.contains("dense"));

    let reversed = preflight_mission_authored_stage_topology(&semantics(vec![
        (5, 0, false, vec![]),
        (2, 1, false, vec![]),
    ]));
    assert!(
        rejection(reversed, "reversed source order fixture")?
            .contains("not increasing")
    );
    Ok(())
}
