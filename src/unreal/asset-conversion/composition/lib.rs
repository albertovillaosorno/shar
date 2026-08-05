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
//   - Asset conversion lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Asset conversion lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for asset conversion.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Asset conversion lib.rs.

mod conversion_plan;

#[path = "../domain/mod.rs"]
pub mod domain;
/// Conversion artifact-storage ports.
#[path = "../port-outbound/mod.rs"]
pub mod ports;
