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
//   - Dataset outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Dataset outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Dataset outbound adapter.

use super::model::FbxRepairAlgorithm;

/// Return every verified per-FBX repair in deterministic registration order.
///
/// Future modules live below `algorithms/dataset/` and use a normalized
/// relative path slug such as `level_01_zones_l1z1.rs`. Each module contributes
/// exactly one algorithm after its original-versus-edited comparison passes.
#[must_use]
pub(super) const fn registered_algorithms() -> &'static [FbxRepairAlgorithm] {
    &[]
}
