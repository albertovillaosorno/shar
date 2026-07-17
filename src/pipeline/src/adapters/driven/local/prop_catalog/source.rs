// File:
//   - source.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/source.rs
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
//   - Mission prop source extraction and inventory facade.
// - Must-Not:
//   - Serialize FBX, deduplicate prepared assets, or publish final artifacts.
// - Allows:
//   - Delegation to mission inventory.
// - Split-When:
//   - The facade gains policy beyond ordered delegation.
// - Merge-When:
//   - One caller can depend directly on category modules without drift.
// - Summary:
//   - Produces one deterministic candidate list from mission data.
// - Description:
//   - Keeps extraction and category discovery behind one application boundary.
// - Usage:
//   - Called once by the mission prop batch.
// - Defaults:
//   - Candidate order is canonicalized after mission discovery.
//
// ADRs:
// - docs/adr/fbx/extraction/source-discovery-boundary.md
// - docs/adr/fbx/export/fbx-output-contract-boundary.md
//
// Large file:
//   - false
//

//! Mission prop source extraction and inventory facade.

use std::path::Path;

use super::extraction::extract_selected_packages;
use super::mission_inventory::discover_mission_candidates;
use super::model::PropCandidate;
use crate::domain::PipelineError;
use crate::domain::package::PhaseThreePackageIndex;

/// Extract mission packages and discover all model-bearing occurrences.
///
/// # Errors
///
/// Returns an error when extraction or mission inventory fails.
pub(super) fn extract_and_discover(
    index: &PhaseThreePackageIndex,
    game_root: &Path,
    normalized_root: &Path,
) -> Result<
    (
        usize,
        Vec<PropCandidate>,
    ),
    PipelineError,
> {
    let source_packages = extract_selected_packages(
        index,
        game_root,
        normalized_root,
    )?;
    let mut candidates = discover_mission_candidates(
        index,
        normalized_root,
    )?;
    candidates.sort();
    Ok(
        (
            source_packages,
            candidates,
        ),
    )
}
