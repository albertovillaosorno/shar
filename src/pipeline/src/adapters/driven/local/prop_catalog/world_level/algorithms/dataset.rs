// File:
//   - dataset.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/prop_catalog/world_level/
//     algorithms/dataset.rs
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
//   - The compile-time registry of verified per-FBX repair modules.
// - Must-Not:
//   - Read local Blender exports or register unverified transformations.
// - Allows:
//   - One source-dependent Rust module per manually edited FBX.
// - Summary:
//   - Registers verified world-FBX repair algorithms.
//
// Large file:
//   - false
//

//! Compile-time registry of verified world-FBX repair algorithms.

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
