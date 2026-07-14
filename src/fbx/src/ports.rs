// File:
//   - ports.rs
// Path:
//   - src/fbx/src/ports.rs
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
//   - fbx module behavior for ports.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute ports.
// - Split-When:
//   - Split when ports contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Reads already-decoded minor-unit world candidates.
// - Description:
//   - Defines ports data and behavior for fbx root.
// - Usage:
//   - Used by fbx root code that needs ports.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! Reads already-decoded minor-unit world candidates.
//!
//! This boundary keeps reads already-decoded minor-unit world candidates
//! explicit and returns deterministic results to fbx callers.
/// Reads stable decoded world candidates for planning.
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
    fn write_world_plan(
        &mut self,
        plan_json: &str,
    ) -> Result<(), String>;
}

/// Scene serialization outbound port.
pub mod scene_writer;

/// Component-source inbound port.
pub mod component_source;

/// Package-index catalog inbound port.
pub mod artifact_sink;
pub mod package_index;
