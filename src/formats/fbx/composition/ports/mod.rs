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
//   - Port outbound outbound port.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Port outbound outbound port.
// - Description:
//   - Implements the declared outbound port responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Port outbound outbound port.

/// Source boundary that supplies decoded world evidence to FBX export.
pub trait WorldSourcePort {
    /// Returns the stable ids of decoded world candidates available to plan.
    fn world_candidate_ids(&self) -> Vec<String>;
}

/// Writes deterministic world assembly plans.
pub trait WorldPlanSinkPort {
    /// Persist a deterministic world plan document.
    ///
    /// # Errors
    ///
    /// Returns an error when the target cannot store the complete plan.
    fn write_world_plan(&mut self, plan_json: &str) -> Result<(), String>;
}

/// Scene serialization outbound port.
pub mod scene_writer;

/// Component-source inbound port.
pub mod component_source;

/// Package-index catalog inbound port.
pub mod artifact_sink;
pub mod package_index;
