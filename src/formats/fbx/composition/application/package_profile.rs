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
//   - Package profile application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Package profile application service.
// - Description:
//   - Implements the declared application service responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Package profile application service.

pub mod character;
pub mod prop;
pub mod terrain;
pub mod vehicle;

/// Package family selected by the phase-three package-index adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelPackageFamily {
    /// Static or animated prop package.
    Prop,
    /// Vehicle model package.
    Vehicle,
    /// Character or costume package.
    Character,
    /// Terrain or world-piece package represented as mesh geometry.
    Terrain,
}
