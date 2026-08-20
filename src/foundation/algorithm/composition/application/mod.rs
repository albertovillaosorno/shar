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
//   - Application service composition for authoring and replay.
// - Must-Not:
//   - Own adapters or domain serialization.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - One responsibility gains an independent lifecycle.
// - Merge-When:
//   - Another module owns the identical responsibility.
// - Summary:
//   - Algorithm application module.
// - Description:
//   - Application service composition for authoring and replay.
// - Usage:
//   - Used through the owning algorithm function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Algorithm application services.

mod engine;

pub use engine::{
    create_algorithm, create_algorithm_with_source_projections,
    replay_algorithm,
};
