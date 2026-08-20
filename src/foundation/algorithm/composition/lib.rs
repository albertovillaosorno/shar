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
//   - Public facade for the generic source-bound algorithm function.
// - Must-Not:
//   - Expose private reconstruction inputs or product policy.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Algorithm crate facade.
// - Description:
//   - Public facade for the generic source-bound algorithm function.
// - Usage:
//   - Used through the owning algorithm function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Generic source-bound reconstruction algorithm foundation.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "application/mod.rs"]
mod application;
pub mod document;
#[path = "../domain/mod.rs"]
pub mod domain;

pub use application::{create_algorithm, replay_algorithm};
pub use domain::{AlgorithmError, Settings};
