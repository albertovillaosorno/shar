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
//   - Planning application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Planning application service.
// - Description:
//   - Implements the declared application service responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Planning application service.

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
