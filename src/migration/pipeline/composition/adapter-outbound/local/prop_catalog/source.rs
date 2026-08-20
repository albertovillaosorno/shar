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
//   - Source outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Source outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source outbound adapter.

use std::path::Path;

use super::extraction::extract_selected_packages;
use super::model::PropCandidate;
use super::non_world_inventory::discover_non_world_candidates;
use crate::domain::PipelineError;
use crate::domain::package::PhaseThreePackageIndex;

/// Extract card and mission packages and discover model-bearing occurrences.
///
/// # Errors
///
/// Returns an error when extraction or non-world inventory fails.
pub(super) fn extract_and_discover(
    index: &PhaseThreePackageIndex,
    game_root: &Path,
    normalized_root: &Path,
) -> Result<(usize, Vec<PropCandidate>), PipelineError> {
    let source_packages =
        extract_selected_packages(index, game_root, normalized_root)?;
    let mut candidates = discover_non_world_candidates(index, normalized_root)?;
    candidates.sort();
    Ok((source_packages, candidates))
}
