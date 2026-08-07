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
//   - Domain domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Domain domain module.
// - Description:
//   - Implements the declared domain module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Domain domain module.

mod json;
#[rustfmt::skip]
mod optional_mod_preview;
mod output_summary;
pub mod package;
mod pipeline;

/// Escapes text for insertion inside one JSON string value.
pub(crate) fn escape_json(value: &str) -> String {
    json::escape(value)
}
pub use optional_mod_preview::{
    OPTIONAL_MOD_PREVIEW_SCHEMA, OptionalModPreview,
};
pub use output_summary::{DirectorySummary, OutputSummary};
pub use package::{
    ConversionFamily, FbxModelPlan, MISSION_SCRIPT_SCHEMA,
    MissionCommandInvocation, MissionScriptEvidence, PackageMemberRef,
    PackageRole, PhaseThreePackageIndex, PhaseThreePackageMember,
    PhaseThreePackagePlan, PhaseThreePackagePlanner, PhaseThreePackageRow,
    PhaseThreePackageSelector, UNREAL_IMPORT_MANIFEST_SCHEMA,
    UNREAL_IMPORT_SUMMARY_SCHEMA, UnrealFbxArtifactEvidence,
    UnrealImportManifest, UnrealNativePlan, UnrealSourceEvidence,
    UnrealTargetKind, preflight_mission_script,
};
pub use pipeline::{
    PipelineConfig, PipelineError, PipelineOutcome, PipelineReport, StageReport,
};
