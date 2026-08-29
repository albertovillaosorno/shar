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
mod output_summary;
pub mod package;
mod pipeline;

/// Escapes text for insertion inside one JSON string value.
pub(crate) fn escape_json(value: &str) -> String {
    json::escape(value)
}
pub use output_summary::{DirectorySummary, OutputSummary};
pub use package::{
    ConversionFamily, DynaLoadData, DynaLoadOperation, DynaLoadOperationKind,
    DynaLoadPackageEffect, DynaLoadPackageTransition,
    DynamicZoneTraversalHistory, DynamicZoneTraversalStep,
    DynamicZoneTriggerEffect, DynamicZoneTriggerEvent,
    DynamicZoneTriggerOccupancy,
    FbxModelPlan, MISSION_SCRIPT_SCHEMA, VEHICLE_TUNING_SCHEMA,
    MissionCameraCatalog, MissionCameraCatalogEntry,
    MissionCameraComponentKind, MissionCameraReferenceBinding,
    MissionCameraReferenceReport, MissionCameraReferenceRole,
    MissionCharacterCatalogReference, MissionCommandInvocation,
    MissionConditionBinding, MissionConditionCommandBinding,
    MissionConditionCommandReport, MissionConditionDirective,
    MissionConditionParameterBinding, MissionConditionParameterReport,
    MissionConditionParameters, MissionConditionReport, MissionConditionScope,
    MissionConditionSemanticBinding, MissionConditionSemanticReport,
    MissionFmvReferenceBinding, MissionFmvReferenceReport,
    MissionGagTotalBinding, MissionGagTotalReport,
    MissionBonusDialogueLocatorBinding, MissionContextAdaptation,
    MissionInitializationBinding, MissionLevelLocatorReferenceBinding,
    MissionLevelLocatorReferenceReport, MissionLevelLocatorRole,
    MissionLevelNpcBinding, MissionLevelNpcKind, MissionLevelNpcReport,
    MissionLevelNpcWaypointBinding,
    MissionLocatorActivePackageReport, MissionLocatorActivePackages,
        MissionLocatorCatalog,
        MissionLocatorCatalogEntry,
        MissionLocatorMissionBindings,
        MissionLocatorReferenceBinding,
        MissionLocatorReferenceReport,
        MissionLocatorResolution,
        MissionLocatorRole,
        MissionLocatorTypeConstraint,
        MissionResolvedLocatorReference,
    MissionPackageLoadBinding, MissionPackageLoadReport,
    MissionP3dPackageReference, MissionP3dReferenceCatalog,
    MissionPedGroupBinding, MissionPedGroupMemberBinding, MissionPedGroupReport,
    MissionPedGroupSelectionBinding,
    MissionPresentationPackageReference, MissionPresentationReferenceReport,
    MissionPresentationRole, MissionPurchaseRewardBinding,
    MissionPurchaseRewardReport, MissionPurchaseRewardSeller,
    MissionPurchaseRewardWaypointBinding, MissionRewardOfferBinding,
    MissionRewardOfferKind, MissionRewardOfferReport, MissionRewardOfferVendor,
    MissionRewardPackageReference, MissionRewardReferenceReport,
    MissionInitializationDirective, MissionInitializationReport,
    MissionObjectiveBinding, MissionObjectiveDirective,
    MissionObjectiveNpcReference, MissionObjectiveNpcWaypointBinding,
    MissionObjectiveNpcWaypointReport, MissionObjectiveParameterBinding,
    MissionObjectiveParameterReport, MissionObjectiveParameters,
    MissionObjectiveReport, MissionObjectiveSemanticBinding,
    MissionObjectiveSemanticReport, MissionParticipantReference,
    MissionParticipantRole, MissionReferenceCatalog, MissionReferenceReport,
    MissionResolvedMissionReferences, MissionResolvedParticipantReference,
    MissionRoadArrowBinding, MissionRoadArrowMode, MissionScopeCommand,
    MissionScopeCondition, MissionScopeGraph, MissionScopeObjective,
    MissionScopeReport, MissionScopeStage, MissionScriptEvidence,
    MissionStageDirective, MissionStageKind, MissionStageMessageKind,
    MissionStageMessageReferenceBinding, MissionStageMessageReferenceReport,
    MissionStageSemanticBinding, MissionStageSemanticReport,
    MissionStageTerminalOutcome, MissionStageTransitionMarker,
    MissionStageTransitionMarkerKind, MissionStageTransitionPolicy,
    MissionStageTransitionReport, MissionStageVehicleReference,
    MissionStageVisualTransition, MissionTrafficGroupBinding,
    MissionVehicleAttributeBinding, MissionVehicleAttributeReport,
    MissionTrafficGroupMemberBinding, MissionTrafficGroupReport,
    MissionVehicleCatalogReference,
    MissionVehicleReference, PackageMemberRef, PackageRole,
    PhaseThreePackageIndex, PhaseThreePackageMember, PhaseThreePackagePlan,
    PhaseThreePackagePlanner, PhaseThreePackageRow, PhaseThreePackageSelector,
    PhaseThreeTextKey,
    UNREAL_IMPORT_MANIFEST_SCHEMA, UNREAL_IMPORT_SUMMARY_SCHEMA,
    UnrealFbxArtifactEvidence, UnrealImportManifest, UnrealNativePlan,
    UnrealSourceEvidence, UnrealTargetKind, UnrealUiRasterArtifactEvidence,
    VehicleTuningCommandInvocation, VehicleTuningEvidence,
    compile_mission_scope_graphs,
    compile_dyna_load_package_transition, parse_dyna_load_data,
    preflight_mission_authored_stage_topology,
    preflight_mission_camera_references,
    preflight_mission_collectible_waypoints,
    preflight_mission_countdowns,
    preflight_mission_condition_commands,
    preflight_mission_condition_parameters,
    preflight_mission_condition_semantics, preflight_mission_conditions,
    preflight_mission_fmv_references, preflight_mission_package_loads,
    preflight_mission_package_loads_with_catalog,
    preflight_mission_ped_group_selections, preflight_mission_ped_groups,
    preflight_mission_pickup_state_props,
    preflight_mission_presentation_references,
    preflight_mission_level_locator_references, preflight_mission_level_npcs,
    preflight_mission_purchase_rewards,
    preflight_mission_reward_offers, preflight_mission_reward_references,
    preflight_mission_locator_references,
    preflight_mission_gag_totals, preflight_mission_initialization,
    preflight_mission_objective_commands,
    preflight_mission_objective_npc_waypoints,
    preflight_mission_objective_parameters,
    preflight_mission_objective_semantics, preflight_mission_objectives,
    preflight_mission_references,
    preflight_mission_stage_message_references,
    preflight_mission_stage_semantics, preflight_mission_stage_transitions,
    preflight_mission_traffic_groups,
    preflight_mission_vehicle_attributes, preflight_mission_vehicle_selects,
};
pub use pipeline::{
    PipelineConfig, PipelineError, PipelineOutcome, PipelineReport, StageReport,
};
