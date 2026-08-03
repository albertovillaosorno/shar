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
//   - Vehicle application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Vehicle application service.
// - Description:
//   - Implements the declared application service responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Vehicle application service.

#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Vehicle package export profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VehicleProfile {
    /// Vehicles require visible model geometry.
    pub requires_mesh: bool,
    /// Vehicle physics is preserved outside FBX.
    pub preserves_physics_sidecar: bool,
}
