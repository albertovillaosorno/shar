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
//   - Fbx lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Fbx lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Fbx lib.rs.

#[path = "adapters.rs"]
pub mod adapters;
/// Use cases that assemble domain scenes and invoke ports.
#[path = "application/mod.rs"]
pub mod application;
/// Package-independent scene, mesh, material, and texture model.
#[path = "../domain/mod.rs"]
pub mod domain;
/// Hexagonal ports used by future readers, writers, and validators.
#[path = "ports/mod.rs"]
pub mod ports;
