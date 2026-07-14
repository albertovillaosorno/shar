// File:
//   - planning.rs
// Path:
//   - src/fbx/src/application/planning.rs
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
//   - fbx use-case orchestration for application planning.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when planning contains two independently testable contracts.
// - Merge-When:
//   - Another fbx module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - FBX export plan for one model-like package.
// - Description:
//   - Defines planning data and behavior for fbx application.
// - Usage:
//   - Called by entrypoints after ports and adapters are selected by the
//   - caller.
// - Defaults:
//   - No concrete adapter is selected unless the caller supplies one through a
//   - port.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
// - docs/adr/pipeline/unreal/unreal-manifest-and-package-taxonomy.md
//
// Large file:
//   - false
//

//! FBX export plan for one model-like package.
//!
//! This boundary keeps fbx export plan for one model-like package explicit and
//! returns deterministic results to fbx callers.
use crate::application::package_profile::ModelPackageFamily;
use crate::domain::capability::CapabilityReport;
use crate::domain::coordinate::CoordinateSystem;

/// FBX export plan for one model-like package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelExportPlan {
    /// Stable package id from phase-three index evidence.
    pub package_id: String,
    /// Package family used only to select capability requirements.
    pub family: ModelPackageFamily,
    /// Model member ids selected by the package-index adapter.
    pub model_member_ids: Vec<String>,
    /// Material member ids selected by the package-index adapter.
    pub material_member_ids: Vec<String>,
    /// Texture member ids selected by the package-index adapter.
    pub texture_member_ids: Vec<String>,
    /// Animation member ids selected by the package-index adapter.
    pub animation_member_ids: Vec<String>,
    /// Coordinate policy selected by application rules.
    pub coordinate_system: CoordinateSystem,
    /// Explicit capability decisions.
    pub capability_report: CapabilityReport,
}
