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
//   - Outcome domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Outcome domain module.
// - Description:
//   - Implements the declared domain module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Outcome domain module.

#![expect(
    clippy::module_name_repetitions,
    reason = "Tests verify these intentional explicit file-local contracts \
              remain safe."
)]

/// Outcome of evaluating one export capability.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityOutcome {
    /// Concept is converted into the FBX artifact.
    Converted,
    /// Concept is retained in a companion report because FBX cannot represent
    /// it.
    PreservedAsMetadata,
    /// Concept is known but intentionally deferred for a later capability pass.
    Deferred,
    /// Concept is required and must fail the package export.
    UnsupportedFailClosed,
}
