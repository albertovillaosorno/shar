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
/// Typed Dyna Load Data postfix syntax.
pub mod dyna_load_data;
/// Pure Dyna Load Data package transitions.
pub mod dyna_load_package;
/// Explicit DynamicZone traversal-history package projection.
pub mod dynamic_zone_history;
/// Reviewed mission condition alias preflight.
pub mod mission_condition;
/// Authored stage countdown block relationships.
pub mod mission_countdown;
/// Collectible-to-stage-waypoint cross-reference binding.
pub mod mission_collectible_route;
/// Canonical objective FMV package-reference resolution.
mod mission_fmv_reference;
/// Source-backed per-level gag totals.
mod mission_gag_total;
/// Typed mission-scope initialization and restart semantics.
pub mod mission_initialization;
/// Reviewed mission objective alias preflight.
pub mod mission_objective;
/// Level-scoped mission camera component reference resolution.
pub mod mission_camera_reference;
/// Canonical explicit mission package-load resolution.
pub mod mission_load;
/// Shared authored mission P3D package-reference catalog.
pub mod mission_p3d_reference;
/// Canonical mission presentation package-reference resolution.
pub mod mission_presentation_reference;
/// Canonical BindReward package-reference resolution.
pub mod mission_reward_reference;
/// Typed source merchandise and price evidence.
mod mission_reward_offer;
/// Source-backed ambient and bonus-mission NPC setup.
pub mod mission_level_npc;
pub mod mission_level_locator_reference;
/// Source-backed pedestrian group declarations.
pub mod mission_ped_group;
/// Pickup-target state-prop cross-scope binding.
pub mod mission_pickup_state_prop;
/// Source-backed traffic model-group declarations.
pub mod mission_traffic_group;
/// Source-backed purchase-car storefront setup.
pub mod mission_purchase_reward;
/// Canonical mission locator package-reference resolution.
pub mod mission_locator;
/// Typed mission locator-reference binding.
pub mod mission_locator_reference;
/// Canonical mission participant package-reference resolution.
pub mod mission_reference;
/// Lossless reviewed mission scope graph projection.
pub mod mission_scope;
/// Normalized mission-script semantic preflight.
pub mod mission_script;
/// Typed reviewed mission stage semantics.
pub mod mission_stage;
/// Canonical localization references for mission stage messages.
mod mission_stage_message_reference;
/// Authored mission-stage order and final/terminal invariants.
pub mod mission_topology;
/// Effective reviewed stage transition and presentation policy.
pub mod mission_transition;
/// Package-backed opaque vehicle attribute tuple evidence.
mod mission_vehicle_attributes;
/// Canonical vehicle-select registration binding.
pub mod mission_vehicle_select;
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
    PhaseThreePackageMember, PhaseThreePackageRow, PhaseThreeTextKey,
};
pub use dyna_load_data::{
    DynaLoadData, DynaLoadOperation, DynaLoadOperationKind, parse_dyna_load_data,
};
pub use dyna_load_package::{
    DynaLoadPackageEffect, DynaLoadPackageTransition,
    compile_dyna_load_package_transition,
};
pub use dynamic_zone_history::{
    DynamicZoneTraversalHistory, DynamicZoneTraversalStep,
    DynamicZoneTriggerEffect, DynamicZoneTriggerEvent,
    DynamicZoneTriggerOccupancy,
};
pub use mission_countdown::{
    MissionCountdownBinding, MissionCountdownEntryBinding,
    MissionCountdownReport, preflight_mission_countdowns,
};
pub use mission_collectible_route::{
    MissionCollectibleWaypointBinding, MissionCollectibleWaypointReport,
    preflight_mission_collectible_waypoints,
};
pub use mission_condition::{
    MissionConditionBinding, MissionConditionCommandBinding,
    MissionConditionCommandReport, MissionConditionDirective,
    MissionConditionParameterBinding, MissionConditionParameterReport,
    MissionConditionParameters, MissionConditionReport,
    MissionConditionSemanticBinding, MissionConditionSemanticReport,
    MissionConditionViolationBinding, MissionConditionViolationEffect,
    MissionConditionViolationReport, preflight_mission_condition_commands,
    preflight_mission_condition_parameters,
    preflight_mission_condition_semantics,
    preflight_mission_condition_violations, preflight_mission_conditions,
};
pub use mission_initialization::{
    MissionInitializationBinding, MissionInitializationDirective,
    MissionInitializationReport, preflight_mission_initialization,
};
pub use mission_camera_reference::{
    MissionCameraCatalog, MissionCameraCatalogEntry,
    MissionCameraComponentKind, MissionCameraReferenceBinding,
    MissionCameraReferenceReport, MissionCameraReferenceRole,
    preflight_mission_camera_references,
};
pub use mission_load::{
    MissionPackageLoadBinding, MissionPackageLoadReport,
    preflight_mission_package_loads,
    preflight_mission_package_loads_with_catalog,
};
pub use mission_p3d_reference::{
    MissionP3dPackageReference, MissionP3dReferenceCatalog,
};
pub use mission_presentation_reference::{
    MissionPresentationPackageReference, MissionPresentationReferenceReport,
    MissionPresentationRole, preflight_mission_presentation_references,
};
pub use self::{
    mission_fmv_reference::{
        MissionFmvReferenceBinding, MissionFmvReferenceReport,
        preflight_mission_fmv_references,
    },
    mission_gag_total::{
        MissionGagTotalBinding, MissionGagTotalReport,
        preflight_mission_gag_totals,
    },
    mission_reward_offer::{
        MissionRewardOfferBinding, MissionRewardOfferKind,
        MissionRewardOfferReport, MissionRewardOfferVendor,
        preflight_mission_reward_offers,
    },
    mission_reward_reference::{
        MissionRewardPackageReference, MissionRewardReferenceReport,
        preflight_mission_reward_references,
    },
    mission_stage_message_reference::{
        MissionStageMessageReferenceBinding,
        MissionStageMessageReferenceReport,
        preflight_mission_stage_message_references,
    },
    mission_vehicle_attributes::{
        MissionVehicleAttributeBinding, MissionVehicleAttributeReport,
        preflight_mission_vehicle_attributes,
    },
};
pub use mission_locator::{
    MissionLocatorCatalog, MissionLocatorCatalogEntry, MissionLocatorResolution,
    MissionLocatorTypeConstraint, MissionResolvedLocatorReference,
};
pub use mission_locator_reference::{
    MissionLocatorActivePackageReport, MissionLocatorActivePackages,
    MissionLocatorMissionBindings, MissionLocatorReferenceBinding,
    MissionLocatorReferenceReport, MissionLocatorRole,
    preflight_mission_locator_references,
};
pub use mission_objective::{
    MissionObjectiveBinding, MissionObjectiveCommandBinding,
    MissionObjectiveCommandReport, MissionObjectiveDirective,
    MissionObjectiveNpcReference, MissionObjectiveNpcWaypointBinding,
    MissionObjectiveNpcWaypointReport, MissionObjectiveParameterBinding,
    MissionObjectiveParameterReport, MissionObjectiveParameters,
    MissionObjectiveReport, MissionObjectiveSemanticBinding,
    MissionObjectiveSemanticReport, MissionRoadArrowBinding,
    MissionRoadArrowMode, preflight_mission_objective_commands,
    preflight_mission_objective_npc_waypoints,
    preflight_mission_objective_parameters,
    preflight_mission_objective_semantics, preflight_mission_objectives,
};
pub use mission_level_npc::{
    MissionBonusDialogueLocatorBinding, MissionLevelNpcBinding,
    MissionLevelNpcKind, MissionLevelNpcReport, MissionLevelNpcWaypointBinding,
    preflight_mission_level_npcs,
};
pub use mission_level_locator_reference::{
    MissionLevelLocatorReferenceBinding, MissionLevelLocatorReferenceReport,
    MissionLevelLocatorRole, preflight_mission_level_locator_references,
};
pub use mission_pickup_state_prop::{
    MissionPickupStatePropBinding, MissionPickupStatePropReport,
    MissionPickupStatePropScope, preflight_mission_pickup_state_props,
};
pub use mission_ped_group::{
    MissionPedGroupBinding, MissionPedGroupMemberBinding, MissionPedGroupReport,
    MissionPedGroupSelectionBinding, preflight_mission_ped_group_selections,
    preflight_mission_ped_groups,
};
pub use mission_purchase_reward::{
    MissionPurchaseRewardBinding, MissionPurchaseRewardReport,
    MissionPurchaseRewardSeller, MissionPurchaseRewardWaypointBinding,
    preflight_mission_purchase_rewards,
};
pub use mission_reference::{
    MissionCharacterCatalogReference, MissionParticipantReference,
    MissionParticipantRole, MissionReferenceCatalog, MissionReferenceReport,
    MissionResolvedMissionReferences, MissionResolvedParticipantReference,
    MissionVehicleCatalogReference, MissionVehicleReference,
    preflight_mission_references,
};
pub use mission_scope::{
    MissionConditionScope, MissionScopeCommand, MissionScopeCondition,
    MissionScopeGraph, MissionScopeObjective, MissionScopeReport,
    MissionScopeStage, compile_mission_scope_graphs,
};
pub use mission_traffic_group::{
    MissionTrafficGroupBinding, MissionTrafficGroupMemberBinding,
    MissionTrafficGroupReport, preflight_mission_traffic_groups,
};
pub use mission_script::{
    MISSION_SCRIPT_SCHEMA, MissionCommandInvocation, MissionContextAdaptation,
    MissionScriptEvidence, preflight_mission_script,
};
pub use mission_stage::{
    MissionStageDirective, MissionStageKind, MissionStageMessageKind,
    MissionStageSemanticBinding, MissionStageSemanticReport,
    MissionStageVehicleReference, preflight_mission_stage_semantics,
};
pub use mission_topology::{
    MissionAuthoredStageTopologyBinding, MissionAuthoredStageTopologyReport,
    preflight_mission_authored_stage_topology,
};
pub use mission_vehicle_select::{
    MissionVehicleSelectBinding, MissionVehicleSelectReport,
    preflight_mission_vehicle_selects,
};
pub use mission_transition::{
    MissionStageTerminalOutcome, MissionStageTransitionMarker,
    MissionStageTransitionMarkerKind, MissionStageTransitionPolicy,
    MissionStageTransitionReport, MissionStageVisualTransition,
    preflight_mission_stage_transitions,
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
