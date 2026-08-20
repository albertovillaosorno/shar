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
//   - Source mesh grouping outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Source mesh grouping outbound adapter.
// - Description:
//   - Preserves source mesh ownership without spatial regrouping.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Source mesh grouping outbound adapter.

use fbx::domain::mesh::MeshAsset;

/// Preserve one source mesh as one exported mesh.
///
/// Spatial distance between disconnected source components is not
/// object-identity
/// evidence. Source owner and placement records govern downstream object roles;
/// this boundary therefore performs no proximity-based regrouping.
///
#[must_use]
pub(super) fn preserve_source_mesh(mesh: MeshAsset) -> Vec<MeshAsset> {
    vec![mesh]
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/prop_catalog/world_level/islands/tests.rs"]
mod tests;
