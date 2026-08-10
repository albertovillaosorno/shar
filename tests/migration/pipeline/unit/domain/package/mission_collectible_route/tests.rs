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
//   - Unit evidence for collectible-to-waypoint index binding.
// - Must-Not:
//   - Infer route navigation or checkpoint traversal behavior.
// - Allows:
//   - Verify exact index resolution, bounds, and declaration order.
// - Split-When:
//   - Navigation topology requires independent fixtures.
// - Merge-When:
//   - Complete mission route tests own this exact cross-reference.
// - Summary:
//   - Collectible waypoint cross-reference unit tests.
// - Description:
//   - Proves authored indices resolve to exact source locator identities.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Out-of-range and forward references fail closed.
//

//! Unit evidence for collectible-to-waypoint bindings.

use super::*;

fn reports(
    collectible_ordinal: usize,
    waypoint_ordinal: usize,
    binding_ordinal: usize,
    collectible_index: u32,
    waypoint_index: u32,
) -> (MissionStageSemanticReport, MissionObjectiveSemanticReport) {
    let stages =
        MissionStageSemanticReport::from_topology_entries_for_tests(vec![(
        2,
        0,
        false,
        vec![MissionStageDirective::Waypoint {
            source_ordinal: waypoint_ordinal,
            locator_id: "route_a".to_owned(),
        }],
    )]);
    let objectives =
        MissionObjectiveSemanticReport::from_route_entries_for_tests(
        vec![(
            3,
            "dump".to_owned(),
            vec![
                MissionObjectiveDirective::Collectible {
                    source_ordinal: collectible_ordinal,
                    locator_id: "cargo_a".to_owned(),
                    drawable_id: None,
                    legacy_arguments: Vec::new(),
                },
                MissionObjectiveDirective::BindCollectibleToWaypoint {
                    source_ordinal: binding_ordinal,
                    collectible_index,
                    waypoint_index,
                },
            ],
        )],
    );
    (stages, objectives)
}

#[test]
fn resolves_collectible_and_waypoint_indices() -> Result<(), String> {
    let (stages, objectives) = reports(5, 4, 6, 0, 0);
    let report = preflight_mission_collectible_waypoints(&stages, &objectives)?;
    let [binding] = report.bindings() else {
        return Err("collectible waypoint binding count drifted".to_owned());
    };
    assert_eq!(binding.stage_sequence_ordinal(), 0);
    assert_eq!(binding.source_ordinal(), 6);
    assert_eq!(binding.collectible_index(), 0);
    assert_eq!(binding.collectible_source_ordinal(), 5);
    assert_eq!(binding.collectible_locator_id(), "cargo_a");
    assert_eq!(binding.waypoint_index(), 0);
    assert_eq!(binding.waypoint_source_ordinal(), 4);
    assert_eq!(binding.waypoint_locator_id(), "route_a");
    Ok(())
}

#[test]
fn rejects_out_of_range_indices() {
    let (stages, objectives) = reports(5, 4, 6, 1, 0);
    let error = preflight_mission_collectible_waypoints(&stages, &objectives)
        .expect_err("out-of-range collectible index must fail");
    assert!(error.contains("collectible binding index is out of range"));

    let (stages, objectives) = reports(5, 4, 6, 0, 1);
    let error = preflight_mission_collectible_waypoints(&stages, &objectives)
        .expect_err("out-of-range waypoint index must fail");
    assert!(error.contains("waypoint binding index is out of range"));
}

#[test]
fn rejects_forward_index_targets() {
    let (stages, objectives) = reports(7, 4, 6, 0, 0);
    let error = preflight_mission_collectible_waypoints(&stages, &objectives)
        .expect_err("forward collectible reference must fail");
    assert!(error.contains("later declaration"));

    let (stages, objectives) = reports(5, 7, 6, 0, 0);
    let error = preflight_mission_collectible_waypoints(&stages, &objectives)
        .expect_err("forward waypoint reference must fail");
    assert!(error.contains("later declaration"));
}
