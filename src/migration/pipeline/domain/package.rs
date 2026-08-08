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
//   - Package domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Package domain module.
// - Description:
//   - Implements the declared domain module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Package domain module.

pub mod index;
/// Reviewed mission condition alias preflight.
pub mod mission_condition;
/// Reviewed mission objective alias preflight.
pub mod mission_objective;
/// Lossless reviewed mission scope graph projection.
pub mod mission_scope;
/// Normalized mission-script semantic preflight.
pub mod mission_script;
/// Package conversion planner.
pub mod plan;
/// Typed package selectors.
pub mod selector;
/// Unreal import-manifest planning.
pub mod unreal_manifest;

// Re-exporting the domain-qualified names keeps downstream imports explicit
// while preserving one public package boundary instead of exposing file layout.
#[expect(
    clippy::module_name_repetitions,
    reason = "Re-exports preserve explicit package-domain names for \
              downstream callers."
)]
pub use index::{
    PackageMemberRef, PackageRole, PhaseThreePackageIndex,
    PhaseThreePackageMember, PhaseThreePackageRow,
};
pub use mission_condition::{
    MissionConditionBinding, MissionConditionCommandBinding,
    MissionConditionCommandReport, MissionConditionReport,
    preflight_mission_condition_commands, preflight_mission_conditions,
};
pub use mission_objective::{
    MissionObjectiveBinding, MissionObjectiveCommandBinding,
    MissionObjectiveCommandReport, MissionObjectiveReport,
    preflight_mission_objective_commands, preflight_mission_objectives,
};
pub use mission_scope::{
    MissionConditionScope, MissionScopeCommand, MissionScopeCondition,
    MissionScopeGraph, MissionScopeObjective, MissionScopeReport,
    MissionScopeStage, compile_mission_scope_graphs,
};
pub use mission_script::{
    MISSION_SCRIPT_SCHEMA, MissionCommandInvocation, MissionContextAdaptation,
    MissionScriptEvidence, preflight_mission_script,
};
pub use plan::{
    ConversionFamily, FbxModelPlan, FbxTargetKind, PhaseThreePackagePlan,
    PhaseThreePackagePlanner, UnrealNativePlan, UnrealTargetKind,
};
pub use selector::PhaseThreePackageSelector;
pub use unreal_manifest::{
    UNREAL_IMPORT_MANIFEST_SCHEMA, UNREAL_IMPORT_SUMMARY_SCHEMA,
    UnrealFbxArtifactEvidence, UnrealImportManifest, UnrealSourceEvidence,
};
