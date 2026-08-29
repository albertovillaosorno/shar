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
//   - Pipeline lib.rs.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Pipeline lib.rs.
// - Description:
//   - Implements the declared lib.rs responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Pipeline lib.rs.

#[path = "adapters.rs"]
pub mod adapters;
#[path = "../application/mod.rs"]
pub mod application;
#[rustfmt::skip]
#[path = "../domain/mod.rs"]
pub mod domain;
mod manifest_paths;
mod mission_script;
mod package_index;
mod vehicle_tuning;
mod workspace;
#[rustfmt::skip]
#[path = "../port-outbound/mod.rs"]
pub mod ports;

pub use application::{PipelineService, SummarizeOutput};
pub use domain::{
    ConversionFamily, DirectorySummary, FbxModelPlan, OutputSummary,
    PackageMemberRef, PackageRole, PhaseThreePackageIndex,
    PhaseThreePackagePlan, PhaseThreePackagePlanner, PhaseThreePackageRow,
    PhaseThreePackageSelector, PipelineConfig, PipelineError, PipelineOutcome,
    PipelineReport, StageReport, UNREAL_IMPORT_MANIFEST_SCHEMA,
    UNREAL_IMPORT_SUMMARY_SCHEMA, UnrealImportManifest, UnrealNativePlan,
    UnrealSourceEvidence, UnrealTargetKind,
};
pub use mission_script::preflight_mission_script;
pub use ports::{OutputInventory, PipelineOperations};
pub use vehicle_tuning::preflight_vehicle_tuning;
