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
//   - Cross-binding of collectible indices to authored stage waypoints.
// - Must-Not:
//   - Infer navigation paths, checkpoint traversal, or collectible movement.
// - Allows:
//   - Resolve authored collectible/waypoint indices to exact locator
//     identities.
//   - Reject out-of-range or forward references before mission emission.
// - Split-When:
//   - Route navigation gains an independently authoritative graph model.
// - Merge-When:
//   - Final mission graph compilation owns this exact cross-reference.
// - Summary:
//   - Collectible-to-stage-waypoint semantic preflight.
// - Description:
//   - Closes authored index references without inventing route topology.
// - Usage:
//   - Runs after stage and objective semantic compilation.
// - Defaults:
//   - Missing, out-of-range, or forward references fail closed.
//

//! Source-backed collectible-to-stage-waypoint bindings.

use super::{
    MissionObjectiveDirective, MissionObjectiveSemanticReport,
    MissionStageDirective, MissionStageSemanticReport,
};

/// One resolved `BindCollectibleTo` source relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionCollectibleWaypointBinding {
    stage_sequence_ordinal: usize,
    source_ordinal: usize,
    collectible_index: u32,
    collectible_source_ordinal: usize,
    collectible_locator_id: String,
    waypoint_index: u32,
    waypoint_source_ordinal: usize,
    waypoint_locator_id: String,
}

impl MissionCollectibleWaypointBinding {
    /// Return the dense owning stage ordinal.
    #[must_use]
    pub const fn stage_sequence_ordinal(&self) -> usize {
        self.stage_sequence_ordinal
    }

    /// Return the `BindCollectibleTo` source ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact authored collectible index.
    #[must_use]
    pub const fn collectible_index(&self) -> u32 {
        self.collectible_index
    }

    /// Return the matched `AddCollectible` source ordinal.
    #[must_use]
    pub const fn collectible_source_ordinal(&self) -> usize {
        self.collectible_source_ordinal
    }

    /// Return the matched collectible locator identity.
    #[must_use]
    pub fn collectible_locator_id(&self) -> &str {
        &self.collectible_locator_id
    }

    /// Return the exact authored stage-waypoint index.
    #[must_use]
    pub const fn waypoint_index(&self) -> u32 {
        self.waypoint_index
    }

    /// Return the matched `AddStageWaypoint` source ordinal.
    #[must_use]
    pub const fn waypoint_source_ordinal(&self) -> usize {
        self.waypoint_source_ordinal
    }

    /// Return the matched stage-waypoint locator identity.
    #[must_use]
    pub fn waypoint_locator_id(&self) -> &str {
        &self.waypoint_locator_id
    }
}

/// All resolved collectible-to-waypoint relationships for one source.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionCollectibleWaypointReport {
    bindings: Vec<MissionCollectibleWaypointBinding>,
}

impl MissionCollectibleWaypointReport {
    /// Return bindings in source stage/directive order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionCollectibleWaypointBinding] {
        &self.bindings
    }
}

/// Resolve every collectible/waypoint index relationship within its stage.
///
/// # Errors
///
/// Fails when stage/objective projection counts drift, an index is out of
/// range, or the indexed collectible/waypoint is authored after its binding.
pub fn preflight_mission_collectible_waypoints(
    stages: &MissionStageSemanticReport,
    objectives: &MissionObjectiveSemanticReport,
) -> Result<MissionCollectibleWaypointReport, String> {
    if stages.stages().len() != objectives.objectives().len() {
        return Err("mission stage/objective route count drifted".to_owned());
    }

    let mut bindings = Vec::new();
    for (stage, objective) in stages
        .stages()
        .iter()
        .zip(objectives.objectives())
    {
        let collectibles = objective
            .directives()
            .iter()
            .filter_map(|directive| match directive {
                MissionObjectiveDirective::Collectible {
                    source_ordinal,
                    locator_id,
                    ..
                } => Some((*source_ordinal, locator_id.as_str())),
                _ => None,
            })
            .collect::<Vec<_>>();
        let waypoints = stage
            .directives()
            .iter()
            .filter_map(|directive| match directive {
                MissionStageDirective::Waypoint {
                    source_ordinal,
                    locator_id,
                } => Some((*source_ordinal, locator_id.as_str())),
                _ => None,
            })
            .collect::<Vec<_>>();

        for directive in objective.directives() {
            let MissionObjectiveDirective::BindCollectibleToWaypoint {
                source_ordinal,
                collectible_index,
                waypoint_index,
            } = directive
            else {
                continue;
            };
            let collectible_position = usize::try_from(*collectible_index)
                .map_err(|_| "mission collectible index exceeds host range")?;
            let waypoint_position = usize::try_from(*waypoint_index)
                .map_err(|_| "mission waypoint index exceeds host range")?;
            let (collectible_source_ordinal, collectible_locator_id) =
                collectibles
                    .get(collectible_position)
                .copied()
                    .ok_or_else(|| {
                        "mission collectible binding index is out of range"
                            .to_owned()
                    })?;
            let (waypoint_source_ordinal, waypoint_locator_id) = waypoints
                .get(waypoint_position)
                .copied()
                .ok_or_else(|| {
                    "mission waypoint binding index is out of range".to_owned()
                })?;
            if collectible_source_ordinal >= *source_ordinal
                || waypoint_source_ordinal >= *source_ordinal
            {
                return Err(
                    "mission collectible binding references a later declaration"
                        .to_owned(),
                );
            }
            bindings.push(MissionCollectibleWaypointBinding {
                stage_sequence_ordinal: stage.sequence_ordinal(),
                source_ordinal: *source_ordinal,
                collectible_index: *collectible_index,
                collectible_source_ordinal,
                collectible_locator_id: collectible_locator_id.to_owned(),
                waypoint_index: *waypoint_index,
                waypoint_source_ordinal,
                waypoint_locator_id: waypoint_locator_id.to_owned(),
            });
        }
    }
    Ok(MissionCollectibleWaypointReport { bindings })
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_collectible_route/tests.rs"]
mod tests;
