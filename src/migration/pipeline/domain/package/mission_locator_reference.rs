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
//   - Mission-scoped binding of typed locator references to active packages.
// - Must-Not:
//   - Read files, infer package load precedence, or invent locator types.
// - Allows:
//   - Preserve resolved, missing, and ambiguous package-context outcomes.
// - Summary:
//   - Typed mission locator-reference binding.

//! Mission-scoped typed locator-reference binding.

use std::collections::BTreeMap;

use super::{
    MissionInitializationDirective, MissionInitializationReport, MissionLocatorCatalog,
    MissionLocatorResolution, MissionLocatorTypeConstraint, MissionObjectiveDirective,
    MissionObjectiveSemanticReport, MissionScopeReport, MissionStageDirective,
    MissionStageSemanticReport,
};

const EVENT_LOCATOR_TYPE: u32 = 0;
const CAR_START_LOCATOR_TYPE: u32 = 3;
const NULL_LOCATOR_SENTINEL: &str = "NULL";

/// Semantic role of one exact locator reference in mission source.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MissionLocatorRole {
    /// Vehicle restart position from `SetMissionResetPlayerInCar`.
    InitializationResetVehicle,
    /// Player restart position from `SetMissionResetPlayerOutCar`.
    InitializationResetPlayer,
    /// Vehicle restart position from `SetMissionResetPlayerOutCar`.
    InitializationResetOutCarVehicle,
    /// Automatic initial-walk target.
    InitializationWalk,
    /// Mission-scope collectible state-prop locator.
    InitializationCollectibleStateProp,
    /// Mission-scope player-car placement locator.
    InitializationPlacePlayerCar,
    /// Initial player-vehicle `CarStart` locator.
    InitializationPlayerVehicle,
    /// Stage vehicle declaration `CarStart` locator.
    StageVehicle,
    /// Stage vehicle activation `CarStart` locator.
    StageActivateVehicle,
    /// Stage safe-zone locator.
    StageSafeZone,
    /// Stage collectible state-prop locator.
    StageCollectibleStateProp,
    /// Optional on-foot stage-character locator.
    StageCharacter,
    /// Stage-character vehicle placement locator.
    StageCharacterVehicle,
    /// Stage player-car placement locator.
    StagePlacePlayerCar,
    /// Default-car swap locator.
    StageSwapDefaultCar,
    /// Forced-car swap locator.
    StageSwapForcedCar,
    /// Player swap locator.
    StageSwapPlayer,
    /// Type-0 stage waypoint locator.
    StageWaypoint,
    /// Objective NPC placement locator.
    ObjectiveNpc,
    /// Objective NPC walking-waypoint locator.
    ObjectiveNpcWaypoint,
    /// Dialogue camera best-side locator.
    ObjectiveCameraBestSide,
    /// First authored dialogue-position locator.
    ObjectiveDialoguePositionFirst,
    /// Second authored dialogue-position locator.
    ObjectiveDialoguePositionSecond,
    /// Third authored dialogue-position locator.
    ObjectiveDialoguePositionThird,
    /// Objective destination locator.
    ObjectiveDestination,
    /// Objective collectible placement locator.
    ObjectiveCollectible,
}

/// Active package roots for one exact selected mission id.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLocatorActivePackages {
    mission_id: String,
    script_package_roots: Vec<String>,
    package_roots: Vec<String>,
}

impl MissionLocatorActivePackages {
    /// Build a context whose supplied roots are already script-visible.
    ///
    /// # Errors
    /// Returns an error when the mission id or a package root is malformed.
    pub fn new(
        mission_id: String,
        package_roots: Vec<String>,
    ) -> Result<Self, String> {
        Self::new_with_initial_dynamic(mission_id, package_roots, Vec::new())
    }

    /// Build script visibility plus initial post-script Dyna visibility.
    ///
    /// Dyna roots extend runtime visibility but are not treated as visible to
    /// locator lookups executed while the mission init script is being parsed.
    /// First-occurrence source order is preserved case-insensitively.
    ///
    /// # Errors
    /// Returns an error when the mission id or a package root is malformed.
    pub fn new_with_initial_dynamic(
        mission_id: String,
        mut script_package_roots: Vec<String>,
        initial_dynamic_package_roots: Vec<String>,
    ) -> Result<Self, String> {
        validate_source_identity(&mission_id, "mission locator context id")?;
        for root in script_package_roots
            .iter()
            .chain(&initial_dynamic_package_roots)
        {
            validate_package_root(root)?;
        }
        let mut seen = std::collections::BTreeSet::new();
        script_package_roots
            .retain(|root| seen.insert(root.to_ascii_lowercase()));
        let mut package_roots = script_package_roots.clone();
        package_roots.extend(
            initial_dynamic_package_roots
                .into_iter()
                .filter(|root| seen.insert(root.to_ascii_lowercase())),
        );
        Ok(Self {
            mission_id,
            script_package_roots,
            package_roots,
        })
    }

    /// Return the exact selected mission id.
    #[must_use]
    pub fn mission_id(&self) -> &str {
        &self.mission_id
    }

    /// Return roots visible while the mission init script is being parsed.
    #[must_use]
    pub fn script_package_roots(&self) -> &[String] {
        &self.script_package_roots
    }

    /// Return roots visible after the reviewed initial Dyna load completes.
    #[must_use]
    pub fn package_roots(&self) -> &[String] {
        &self.package_roots
    }
}

/// Active package context indexed by exact selected mission id.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLocatorActivePackageReport {
    by_mission_id: BTreeMap<String, MissionLocatorActivePackages>,
}

impl MissionLocatorActivePackageReport {
    /// Build deterministic mission package contexts.
    ///
    /// # Errors
    /// Returns an error when a selected mission id occurs more than once.
    pub fn from_missions(missions: Vec<MissionLocatorActivePackages>) -> Result<Self, String> {
        let mut by_mission_id = BTreeMap::new();
        for mission in missions {
            if by_mission_id
                .insert(mission.mission_id.clone(), mission)
                .is_some()
            {
                return Err(
                    "mission locator active-package context duplicated a mission".to_owned(),
                );
            }
        }
        Ok(Self { by_mission_id })
    }

    /// Return package context for one exact selected mission id.
    #[must_use]
    pub fn mission(&self, mission_id: &str) -> Option<&MissionLocatorActivePackages> {
        self.by_mission_id.get(mission_id)
    }
}

/// One typed source locator reference and its package-context resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLocatorReferenceBinding {
    owner_stage_source_ordinal: Option<usize>,
    owner_stage_sequence_ordinal: Option<usize>,
    owner_objective_source_ordinal: Option<usize>,
    source_ordinal: usize,
    role: MissionLocatorRole,
    source_name: String,
    type_constraint: MissionLocatorTypeConstraint,
    resolution: MissionLocatorResolution,
}

impl MissionLocatorReferenceBinding {
    /// Return source `AddStage` ordinal for stage/objective locator references.
    #[must_use]
    pub const fn owner_stage_source_ordinal(&self) -> Option<usize> {
        self.owner_stage_source_ordinal
    }

    /// Return dense stage ordinal for stage/objective locator references.
    #[must_use]
    pub const fn owner_stage_sequence_ordinal(&self) -> Option<usize> {
        self.owner_stage_sequence_ordinal
    }

    /// Return source `AddObjective` ordinal for objective locator references.
    #[must_use]
    pub const fn owner_objective_source_ordinal(&self) -> Option<usize> {
        self.owner_objective_source_ordinal
    }

    /// Return source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }
    /// Return semantic locator-reference role.
    #[must_use]
    pub const fn role(&self) -> MissionLocatorRole {
        self.role
    }
    /// Return the exact authored locator identity.
    #[must_use]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }
    /// Return the reviewed locator-type constraint.
    #[must_use]
    pub const fn type_constraint(&self) -> MissionLocatorTypeConstraint {
        self.type_constraint
    }
    /// Return package-context resolution evidence.
    #[must_use]
    pub const fn resolution(&self) -> &MissionLocatorResolution {
        &self.resolution
    }
}

/// Resolved locator-reference evidence for one selected mission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLocatorMissionBindings {
    mission_id: String,
    references: Vec<MissionLocatorReferenceBinding>,
}

impl MissionLocatorMissionBindings {
    /// Return the exact selected mission id.
    #[must_use]
    pub fn mission_id(&self) -> &str {
        &self.mission_id
    }
    /// Return locator bindings in source/role order.
    #[must_use]
    pub fn references(&self) -> &[MissionLocatorReferenceBinding] {
        &self.references
    }
}

/// Complete typed mission locator-reference resolution evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionLocatorReferenceReport {
    missions: Vec<MissionLocatorMissionBindings>,
}

impl MissionLocatorReferenceReport {
    /// Return mission bindings in source mission order.
    #[must_use]
    pub fn missions(&self) -> &[MissionLocatorMissionBindings] {
        &self.missions
    }

    /// Return whether every typed locator reference resolved uniquely.
    #[must_use]
    pub fn has_only_resolved_references(&self) -> bool {
        self.missions.iter().all(|mission| {
            mission
                .references
                .iter()
                .all(|binding| matches!(binding.resolution, MissionLocatorResolution::Resolved(_)))
        })
    }

    /// Count missing or ambiguous typed locator references.
    #[must_use]
    pub fn unresolved_reference_count(&self) -> usize {
        self.missions
            .iter()
            .map(|mission| {
                mission
                    .references
                    .iter()
                    .filter(|binding| {
                        !matches!(binding.resolution, MissionLocatorResolution::Resolved(_))
                    })
                    .count()
            })
            .sum()
    }
}

/// Resolve every typed locator field inside explicit active mission packages.
///
/// Missing and ambiguous identities remain typed outcomes because runtime
/// package precedence is intentionally not guessed.
///
/// # Errors
/// Returns an error when semantic reports drift, a mission context is absent,
/// or package-root evidence is malformed.
pub fn preflight_mission_locator_references(
    catalog: &MissionLocatorCatalog,
    active_packages: &MissionLocatorActivePackageReport,
    scopes: &MissionScopeReport,
    initialization: &MissionInitializationReport,
    stage_semantics: &MissionStageSemanticReport,
    objective_semantics: &MissionObjectiveSemanticReport,
) -> Result<MissionLocatorReferenceReport, String> {
    let mut initializations = initialization.missions().iter();
    let mut stages = stage_semantics.stages().iter();
    let mut objectives = objective_semantics.objectives().iter();
    let mut missions = Vec::with_capacity(scopes.missions().len());
    for mission in scopes.missions() {
        let mission_id = mission.source_mission_id();
        let active = active_packages.mission(mission_id).ok_or_else(|| {
            format!("mission locator active-package context is missing for {mission_id}")
        })?;
        let initialization = initializations
            .next()
            .ok_or_else(|| "mission locator initialization report is incomplete".to_owned())?;
        if initialization.mission_id() != mission_id {
            return Err("mission locator initialization report drifted".to_owned());
        }
        let mut references = Vec::new();
        resolve_initialization(
            catalog,
            active,
            initialization,
            &mut references,
        )?;
        for stage in mission.stages() {
            let stage_semantics = stages
                .next()
                .ok_or_else(|| "mission locator stage report is incomplete".to_owned())?;
            if stage_semantics.source_ordinal() != stage.source_ordinal() {
                return Err("mission locator stage report drifted".to_owned());
            }
            resolve_stage(
                catalog,
                active,
                stage_semantics,
                &mut references,
            )?;
            let objective_semantics = objectives
                .next()
                .ok_or_else(|| "mission locator objective report is incomplete".to_owned())?;
            if objective_semantics.source_ordinal()
                != stage.objective().binding().ordinal()
                || objective_semantics.owner_stage_source_ordinal()
                    != stage_semantics.source_ordinal()
                || objective_semantics.owner_stage_sequence_ordinal()
                    != stage_semantics.sequence_ordinal()
            {
                return Err("mission locator objective report drifted".to_owned());
            }
            resolve_objective(
                catalog,
                active,
                objective_semantics,
                &mut references,
            )?;
        }
        references.sort_by_key(|binding| (binding.source_ordinal, binding.role));
        missions.push(MissionLocatorMissionBindings {
            mission_id: mission_id.to_owned(),
            references,
        });
    }
    if initializations.next().is_some() || stages.next().is_some() || objectives.next().is_some() {
        return Err("mission locator semantic reports contain unowned bindings".to_owned());
    }
    Ok(MissionLocatorReferenceReport { missions })
}

fn push_locator(
    catalog: &MissionLocatorCatalog,
    active: &MissionLocatorActivePackages,
    references: &mut Vec<MissionLocatorReferenceBinding>,
    source_ordinal: usize,
    role: MissionLocatorRole,
    source_name: &str,
    type_constraint: MissionLocatorTypeConstraint,
) -> Result<(), String> {
    let package_roots = package_roots_for_role(active, role);
    let ordered_lookup = role == MissionLocatorRole::ObjectiveCameraBestSide
        || (matches!(type_constraint, MissionLocatorTypeConstraint::Exact(_))
            && role_uses_script_visibility(role));
    let resolution = if ordered_lookup {
        catalog.resolve_in_package_order(
            source_name,
            package_roots,
            type_constraint,
        )?
    } else {
        catalog.resolve(source_name, package_roots, type_constraint)?
    };
    references.push(MissionLocatorReferenceBinding {
        owner_stage_source_ordinal: None,
        owner_stage_sequence_ordinal: None,
        owner_objective_source_ordinal: None,
        source_ordinal,
        role,
        source_name: source_name.to_owned(),
        resolution,
        type_constraint,
    });
    Ok(())
}

fn resolve_initialization(
    catalog: &MissionLocatorCatalog,
    active: &MissionLocatorActivePackages,
    binding: &super::MissionInitializationBinding,
    out: &mut Vec<MissionLocatorReferenceBinding>,
) -> Result<(), String> {
    for directive in binding.directives() {
        match directive {
            MissionInitializationDirective::ResetPlayerInCar {
                source_ordinal,
                vehicle_locator_id,
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::InitializationResetVehicle,
                vehicle_locator_id,
                MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
            )?,
            MissionInitializationDirective::ResetPlayerOutCar {
                source_ordinal,
                player_locator_id,
                vehicle_locator_id,
            } => {
                push_locator(
                    catalog,
                    active,
                    out,
                    *source_ordinal,
                    MissionLocatorRole::InitializationResetPlayer,
                    player_locator_id,
                    MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
                )?;
                push_locator(
                    catalog,
                    active,
                    out,
                    *source_ordinal,
                    MissionLocatorRole::InitializationResetOutCarVehicle,
                    vehicle_locator_id,
                    MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
                )?;
            }
            MissionInitializationDirective::InitialWalk {
                source_ordinal,
                locator_id,
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::InitializationWalk,
                locator_id,
                MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
            )?,
            MissionInitializationDirective::CollectibleStateProp {
                source_ordinal,
                locator_id,
                ..
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::InitializationCollectibleStateProp,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionInitializationDirective::PlacePlayerCar {
                source_ordinal,
                locator_id,
                ..
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::InitializationPlacePlayerCar,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionInitializationDirective::InitialPlayerVehicle {
                source_ordinal,
                locator_id,
                ..
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::InitializationPlayerVehicle,
                locator_id,
                MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
            )?,
            _ => {}
        }
    }
    Ok(())
}

fn resolve_stage(
    catalog: &MissionLocatorCatalog,
    active: &MissionLocatorActivePackages,
    stage: &super::MissionStageSemanticBinding,
    out: &mut Vec<MissionLocatorReferenceBinding>,
) -> Result<(), String> {
    let first_reference = out.len();
    for directive in stage.directives() {
        match directive {
            MissionStageDirective::Vehicle(vehicle) => push_locator(
                catalog,
                active,
                out,
                vehicle.source_ordinal(),
                MissionLocatorRole::StageVehicle,
                vehicle.locator_id(),
                MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
            )?,
            MissionStageDirective::ActivateVehicle {
                source_ordinal,
                locator_id,
                ..
            } if locator_id != NULL_LOCATOR_SENTINEL => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::StageActivateVehicle,
                locator_id,
                MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
            )?,
            MissionStageDirective::SafeZone {
                source_ordinal,
                locator_id,
                ..
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::StageSafeZone,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionStageDirective::CollectibleStateProp {
                source_ordinal,
                locator_id,
                ..
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::StageCollectibleStateProp,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionStageDirective::StageCharacter {
                source_ordinal,
                character_locator_id,
                vehicle_locator_id,
                ..
            } => {
                if let Some(locator_id) = character_locator_id {
                    push_locator(
                        catalog,
                        active,
                        out,
                        *source_ordinal,
                        MissionLocatorRole::StageCharacter,
                        locator_id,
                        MissionLocatorTypeConstraint::Any,
                    )?;
                }
                push_locator(
                    catalog,
                    active,
                    out,
                    *source_ordinal,
                    MissionLocatorRole::StageCharacterVehicle,
                    vehicle_locator_id,
                    MissionLocatorTypeConstraint::Any,
                )?;
            }
            MissionStageDirective::PlacePlayerCar {
                source_ordinal,
                locator_id,
                ..
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::StagePlacePlayerCar,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionStageDirective::SwapDefaultCarLocator {
                source_ordinal,
                locator_id,
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::StageSwapDefaultCar,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionStageDirective::SwapForcedCarLocator {
                source_ordinal,
                locator_id,
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::StageSwapForcedCar,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionStageDirective::SwapPlayerLocator {
                source_ordinal,
                locator_id,
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::StageSwapPlayer,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionStageDirective::Waypoint {
                source_ordinal,
                locator_id,
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::StageWaypoint,
                locator_id,
                MissionLocatorTypeConstraint::Exact(EVENT_LOCATOR_TYPE),
            )?,
            _ => {}
        }
    }
    for binding in &mut out[first_reference..] {
        binding.owner_stage_source_ordinal = Some(stage.source_ordinal());
        binding.owner_stage_sequence_ordinal = Some(stage.sequence_ordinal());
    }
    Ok(())
}

fn resolve_objective(
    catalog: &MissionLocatorCatalog,
    active: &MissionLocatorActivePackages,
    objective: &super::MissionObjectiveSemanticBinding,
    out: &mut Vec<MissionLocatorReferenceBinding>,
) -> Result<(), String> {
    let first_reference = out.len();
    for directive in objective.directives() {
        match directive {
            MissionObjectiveDirective::Npc(npc) => push_locator(
                catalog,
                active,
                out,
                npc.source_ordinal(),
                MissionLocatorRole::ObjectiveNpc,
                npc.locator_id(),
                MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
            )?,
            MissionObjectiveDirective::NpcWaypoint {
                source_ordinal,
                locator_id,
                ..
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::ObjectiveNpcWaypoint,
                locator_id,
                MissionLocatorTypeConstraint::Exact(CAR_START_LOCATOR_TYPE),
            )?,
            MissionObjectiveDirective::CameraBestSide {
                source_ordinal,
                locator_id,
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::ObjectiveCameraBestSide,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionObjectiveDirective::DialoguePositions {
                source_ordinal,
                locator_ids,
                ..
            } => {
                for (role, locator_id) in [
                    MissionLocatorRole::ObjectiveDialoguePositionFirst,
                    MissionLocatorRole::ObjectiveDialoguePositionSecond,
                    MissionLocatorRole::ObjectiveDialoguePositionThird,
                ]
                .into_iter()
                .zip(locator_ids)
                {
                    push_locator(
                        catalog,
                        active,
                        out,
                        *source_ordinal,
                        role,
                        locator_id,
                        MissionLocatorTypeConstraint::Any,
                    )?;
                }
            }
            MissionObjectiveDirective::Destination {
                source_ordinal,
                destination_id,
                ..
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::ObjectiveDestination,
                destination_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            MissionObjectiveDirective::Collectible {
                source_ordinal,
                locator_id,
                ..
            } => push_locator(
                catalog,
                active,
                out,
                *source_ordinal,
                MissionLocatorRole::ObjectiveCollectible,
                locator_id,
                MissionLocatorTypeConstraint::Any,
            )?,
            _ => {}
        }
    }
    for binding in &mut out[first_reference..] {
        binding.owner_stage_source_ordinal =
            Some(objective.owner_stage_source_ordinal());
        binding.owner_stage_sequence_ordinal =
            Some(objective.owner_stage_sequence_ordinal());
        binding.owner_objective_source_ordinal =
            Some(objective.source_ordinal());
    }
    Ok(())
}

fn role_uses_script_visibility(role: MissionLocatorRole) -> bool {
    matches!(
        role,
        MissionLocatorRole::InitializationResetVehicle
            | MissionLocatorRole::InitializationResetPlayer
            | MissionLocatorRole::InitializationResetOutCarVehicle
            | MissionLocatorRole::InitializationCollectibleStateProp
            | MissionLocatorRole::InitializationPlacePlayerCar
            | MissionLocatorRole::InitializationPlayerVehicle
            | MissionLocatorRole::StageVehicle
            | MissionLocatorRole::StageActivateVehicle
            | MissionLocatorRole::StageSafeZone
            | MissionLocatorRole::StageCollectibleStateProp
            | MissionLocatorRole::StageCharacter
            | MissionLocatorRole::StageCharacterVehicle
            | MissionLocatorRole::StagePlacePlayerCar
            | MissionLocatorRole::StageWaypoint
            | MissionLocatorRole::ObjectiveDialoguePositionFirst
            | MissionLocatorRole::ObjectiveDialoguePositionSecond
            | MissionLocatorRole::ObjectiveDialoguePositionThird
    )
}

fn package_roots_for_role(
    active: &MissionLocatorActivePackages,
    role: MissionLocatorRole,
) -> &[String] {
    if role_uses_script_visibility(role) {
        active.script_package_roots()
    } else {
        active.package_roots()
    }
}

fn validate_source_identity(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty() || value != value.trim() || value.chars().any(char::is_control) {
        return Err(format!("{label} is malformed"));
    }
    Ok(())
}

fn validate_package_root(value: &str) -> Result<(), String> {
    validate_source_identity(value, "mission locator active package root")?;
    if value.starts_with('/')
        || value.ends_with('/')
        || value.contains(':')
        || value.contains('\\')
        || value
            .split('/')
            .any(|segment| segment.is_empty() || segment == "." || segment == "..")
        || !value.to_ascii_lowercase().starts_with("extracted/")
    {
        return Err("mission locator active package root is unsafe".to_owned());
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_locator_reference/tests.rs"]
mod tests;
