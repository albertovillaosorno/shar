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
//   - Canonical package resolution for reviewed mission character and vehicle
//     source identities.
// - Must-Not:
//   - Guess ambiguous locators, collapse authored character variants, or invent
//     runtime participant policy.
// - Allows:
//   - Resolve exact source identities against the validated package index and
//     retain symbolic runtime vehicle references explicitly.
// - Split-When:
//   - Locator, reward, or presentation catalogs gain independent namespaces.
// - Merge-When:
//   - Final mission-definition compilation owns this exact reference boundary.
// - Summary:
//   - Mission participant package-reference resolver.
// - Description:
//   - Binds reviewed mission character and vehicle identities to canonical
//     phase-three packages before Unreal mission asset emission.
// - Usage:
//   - Built once from the package index and shared by mission semantic
//     preflight.
// - Defaults:
//   - Missing or ambiguous referenced identities fail closed.
//

//! Canonical package resolution for mission participant references.

use std::collections::BTreeMap;
use std::path::Path;

use super::{
    MissionConditionDirective, MissionConditionSemanticReport,
    MissionInitializationDirective, MissionInitializationReport,
    MissionObjectiveDirective, MissionObjectiveParameters,
    MissionObjectiveSemanticReport, MissionScopeReport, MissionStageDirective,
    MissionStageKind, MissionStageSemanticReport, PackageRole,
    PhaseThreePackageIndex, PhaseThreePackageRow,
};

/// One exact character-package variant resolved from a source identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionCharacterCatalogReference {
    source_id: String,
    participant_id: String,
    package_id: String,
    package_subcategory: String,
}

impl MissionCharacterCatalogReference {
    /// Return the exact authored source identity.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Return the canonical participant identity from the package taxonomy.
    #[must_use]
    pub fn participant_id(&self) -> &str {
        &self.participant_id
    }

    /// Return the exact canonical phase-three package identity.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the exact package subcategory, preserving model/costume variant.
    #[must_use]
    pub fn package_subcategory(&self) -> &str {
        &self.package_subcategory
    }
}

/// One exact physical vehicle package resolved from a source identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionVehicleCatalogReference {
    source_id: String,
    package_id: String,
    package_subcategory: String,
}

impl MissionVehicleCatalogReference {
    /// Return the exact authored source identity.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Return the exact canonical phase-three package identity.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the exact vehicle package subcategory.
    #[must_use]
    pub fn package_subcategory(&self) -> &str {
        &self.package_subcategory
    }
}

/// Resolved mission vehicle identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionVehicleReference {
    /// Resolve to one physical vehicle package.
    Catalog(MissionVehicleCatalogReference),
    /// Preserve the exact runtime `current` vehicle token symbolically.
    Current,
}

/// Semantic field that owns one resolved participant reference.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MissionParticipantRole {
    /// Vehicle required directly by a `buycar` objective.
    ObjectiveBuyVehicle,
    /// Costume required directly by a `buyskin` objective.
    ObjectiveBuyCostume,
    /// Vehicle required directly by a `getin` objective.
    ObjectiveEnterVehicle,
    /// NPC introduced by an objective.
    ObjectiveNpc,
    /// NPC receiving an objective walking waypoint.
    ObjectiveNpcWaypoint,
    /// NPC assigned as an objective vehicle driver.
    ObjectiveDriverCharacter,
    /// Vehicle receiving an objective driver.
    ObjectiveDriverVehicle,
    /// NPC removed from a driver assignment.
    ObjectiveRemoveDriver,
    /// NPC removed from an objective.
    ObjectiveRemoveNpc,
    /// Vehicle targeted by an objective.
    ObjectiveTargetVehicle,
    /// NPC targeted by a talk objective.
    ObjectiveTalkTarget,
    /// Playable participant in dialogue source evidence.
    ObjectiveDialoguePlayer,
    /// Other participant in dialogue source evidence.
    ObjectiveDialogueNpc,
    /// Vehicle observed by a mission condition.
    ConditionTargetVehicle,
    /// Initial mission player vehicle.
    MissionInitialPlayerVehicle,
    /// Mission-scope player-car placement vehicle.
    MissionPlacePlayerCar,
    /// Vehicle required by a locked stage header.
    StageLockedVehicle,
    /// Costume required by a locked stage header.
    StageLockedCostume,
    /// Vehicle declared for a stage.
    StageVehicle,
    /// Optional driver declared with a stage vehicle.
    StageVehicleDriver,
    /// Vehicle activated by a stage.
    StageActivateVehicle,
    /// Vehicle receiving stage AI tuning.
    StageVehicleAi,
    /// Vehicle receiving target catch-up tuning.
    StageTargetCatchupVehicle,
    /// Character placed for a stage.
    StageCharacter,
    /// Vehicle associated with a stage character.
    StageCharacterVehicle,
    /// Vehicle used by stage player-car placement.
    StagePlacePlayerCar,
    /// Character hidden by a stage.
    StageHiddenCharacter,
    /// Optional completion-dialog character.
    StageCompletionDialogCharacter,
    /// Vehicle receiving race catch-up tuning.
    StageRaceCatchupVehicle,
    /// Optional countdown character.
    StageCountdownCharacter,
}

/// Canonical resolved value for one participant reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionParticipantReference {
    /// Exact character model/costume package variant.
    Character(MissionCharacterCatalogReference),
    /// Physical or symbolic vehicle reference.
    Vehicle(MissionVehicleReference),
}

/// One participant reference bound to its source field and statement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionResolvedParticipantReference {
    source_ordinal: usize,
    role: MissionParticipantRole,
    reference: MissionParticipantReference,
}

impl MissionResolvedParticipantReference {
    /// Return the source statement ordinal that authored this reference.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the semantic field that owns this reference.
    #[must_use]
    pub const fn role(&self) -> MissionParticipantRole {
        self.role
    }

    /// Return the canonical participant reference.
    #[must_use]
    pub const fn reference(&self) -> &MissionParticipantReference {
        &self.reference
    }
}

/// Canonical participant references for one selected mission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionResolvedMissionReferences {
    mission_id: String,
    participants: Vec<MissionResolvedParticipantReference>,
}

impl MissionResolvedMissionReferences {
    /// Return the exact selected mission identity.
    #[must_use]
    pub fn mission_id(&self) -> &str {
        &self.mission_id
    }

    /// Return participant references sorted by source ordinal and role.
    #[must_use]
    pub fn participants(&self) -> &[MissionResolvedParticipantReference] {
        &self.participants
    }
}

/// Complete canonical participant-resolution evidence for one mission source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionReferenceReport {
    missions: Vec<MissionResolvedMissionReferences>,
}

impl MissionReferenceReport {
    /// Return resolved mission references in source mission order.
    #[must_use]
    pub fn missions(&self) -> &[MissionResolvedMissionReferences] {
        &self.missions
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CharacterCatalogEntry {
    participant_id: String,
    package_id: String,
    package_subcategory: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct VehicleCatalogEntry {
    package_id: String,
    package_subcategory: String,
}

/// Package-backed participant lookup built from validated phase-three evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionReferenceCatalog {
    characters: BTreeMap<String, Vec<CharacterCatalogEntry>>,
    vehicles: BTreeMap<String, Vec<VehicleCatalogEntry>>,
}

impl MissionReferenceCatalog {
    #[cfg(test)]
    pub(crate) const fn empty_for_tests() -> Self {
        Self {
            characters: BTreeMap::new(),
            vehicles: BTreeMap::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_character_entries_for_tests(
        entries: &[(&str, &str, &str, &str)],
    ) -> Self {
        let mut characters = BTreeMap::new();
        for (source_id, participant_id, package_id, package_subcategory) in entries {
            characters
                .entry(source_id.to_ascii_lowercase())
                .or_insert_with(Vec::new)
                .push(CharacterCatalogEntry {
                    participant_id: (*participant_id).to_owned(),
                    package_id: (*package_id).to_owned(),
                    package_subcategory: (*package_subcategory).to_owned(),
                });
        }
        Self {
            characters,
            vehicles: BTreeMap::new(),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_vehicle_entries_for_tests(
        entries: &[(&str, &str, &str)],
    ) -> Self {
        let mut vehicles = BTreeMap::new();
        for (source_id, package_id, package_subcategory) in entries {
            vehicles
                .entry(source_id.to_ascii_lowercase())
                .or_insert_with(Vec::new)
                .push(VehicleCatalogEntry {
                    package_id: (*package_id).to_owned(),
                    package_subcategory: (*package_subcategory).to_owned(),
                });
        }
        Self {
            characters: BTreeMap::new(),
            vehicles,
        }
    }

    /// Build the deterministic mission participant catalog from package index
    /// evidence.
    ///
    /// # Errors
    ///
    /// Returns an error when a character package taxonomy or source skeleton
    /// identity cannot be represented safely.
    pub fn from_package_index(
        index: &PhaseThreePackageIndex,
    ) -> Result<Self, String> {
        let mut characters =
            BTreeMap::<String, Vec<CharacterCatalogEntry>>::new();
        let mut vehicles = BTreeMap::<String, Vec<VehicleCatalogEntry>>::new();
        for package in index.packages() {
            if package.category() == "characters" {
                index_character_package(package, &mut characters)?;
            } else if package.category() == "cars"
                && !package.ids_for_role(PackageRole::Model).is_empty()
                && package.subcategory() != "cars/runtime-base/common"
            {
                index_vehicle_package(package, &mut vehicles)?;
            }
        }
        sort_catalogs(&mut characters, &mut vehicles);
        Ok(Self { characters, vehicles })
    }

    pub(crate) fn resolve_character(
        &self,
        source_id: &str,
    ) -> Result<MissionCharacterCatalogReference, String> {
        let key = source_id.to_ascii_lowercase();
        let Some(entries) = self.characters.get(&key) else {
            return Err(
                "mission character source identity has no package".to_owned()
            );
        };
        let [entry] = entries.as_slice() else {
            return Err(
                "mission character source identity is ambiguous".to_owned()
            );
        };
        Ok(MissionCharacterCatalogReference {
            source_id: source_id.to_owned(),
            participant_id: entry.participant_id.clone(),
            package_id: entry.package_id.clone(),
            package_subcategory: entry.package_subcategory.clone(),
        })
    }

    pub(crate) fn resolve_vehicle(
        &self,
        source_id: &str,
    ) -> Result<MissionVehicleReference, String> {
        if source_id == "current" {
            return Ok(MissionVehicleReference::Current);
        }
        let key = source_id.to_ascii_lowercase();
        let Some(entries) = self.vehicles.get(&key) else {
            return Err(
                "mission vehicle source identity has no package".to_owned()
            );
        };
        let [entry] = entries.as_slice() else {
            return Err(
                "mission vehicle source identity is ambiguous".to_owned()
            );
        };
        Ok(MissionVehicleReference::Catalog(
            MissionVehicleCatalogReference {
                source_id: source_id.to_owned(),
                package_id: entry.package_id.clone(),
                package_subcategory: entry.package_subcategory.clone(),
            },
        ))
    }
}

fn index_character_package(
    package: &PhaseThreePackageRow,
    catalog: &mut BTreeMap<String, Vec<CharacterCatalogEntry>>,
) -> Result<(), String> {
    let participant_id = character_participant_id(package.subcategory())?;
    for member in package.members() {
        if member.role != PackageRole::Animation
            || member.kind != "p3d-skeleton"
        {
            continue;
        }
        let source_id = Path::new(&member.path)
            .file_stem()
            .and_then(|value| value.to_str())
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                "character skeleton has no portable source identity".to_owned()
            })?;
        catalog
            .entry(source_id.to_ascii_lowercase())
            .or_default()
            .push(CharacterCatalogEntry {
                participant_id: participant_id.clone(),
                package_id: package.package_id.clone(),
                package_subcategory: package.subcategory().to_owned(),
            });
    }
    Ok(())
}

fn character_participant_id(subcategory: &str) -> Result<String, String> {
    let mut segments = subcategory.split('/');
    if segments.next() != Some("characters") {
        return Err("character package subcategory escaped character taxonomy"
            .to_owned());
    }
    let participant = segments
        .next()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            "character package has no participant identity".to_owned()
        })?;
    Ok(participant.to_owned())
}

fn index_vehicle_package(
    package: &PhaseThreePackageRow,
    catalog: &mut BTreeMap<String, Vec<VehicleCatalogEntry>>,
) -> Result<(), String> {
    let source_id = Path::new(&package.package_root)
        .file_name()
        .and_then(|value| value.to_str())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            "vehicle package has no portable source identity".to_owned()
        })?;
    catalog
        .entry(source_id.to_ascii_lowercase())
        .or_default()
        .push(VehicleCatalogEntry {
            package_id: package.package_id.clone(),
            package_subcategory: package.subcategory().to_owned(),
        });
    Ok(())
}

fn sort_catalogs(
    characters: &mut BTreeMap<String, Vec<CharacterCatalogEntry>>,
    vehicles: &mut BTreeMap<String, Vec<VehicleCatalogEntry>>,
) {
    for entries in characters.values_mut() {
        entries.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    }
    for entries in vehicles.values_mut() {
        entries.sort_by(|left, right| left.package_id.cmp(&right.package_id));
    }
}

/// Resolve every reviewed character and vehicle source identity to canonical
/// package evidence.
///
/// # Errors
///
/// Returns an error when semantic reports no longer align with the scope graph
/// or when a referenced participant is missing or ambiguous in the catalog.
pub fn preflight_mission_references(
    catalog: &MissionReferenceCatalog,
    scopes: &MissionScopeReport,
    objective_semantics: &MissionObjectiveSemanticReport,
    condition_semantics: &MissionConditionSemanticReport,
    initialization: &MissionInitializationReport,
    stage_semantics: &MissionStageSemanticReport,
) -> Result<MissionReferenceReport, String> {
    let mut objective_bindings = objective_semantics.objectives().iter();
    let mut condition_bindings = condition_semantics.conditions().iter();
    let mut initialization_bindings = initialization.missions().iter();
    let mut stage_bindings = stage_semantics.stages().iter();
    let mut missions = Vec::with_capacity(scopes.missions().len());

    for mission in scopes.missions() {
        let initialization =
            initialization_bindings.next().ok_or_else(|| {
                "mission reference initialization report is incomplete"
                    .to_owned()
            })?;
        if initialization.mission_id() != mission.source_mission_id() {
            return Err(
                "mission reference initialization report drifted".to_owned()
            );
        }
        let mut participants = Vec::new();
        resolve_initialization(catalog, initialization, &mut participants)?;

        for stage in mission.stages() {
            let stage_semantics = stage_bindings.next().ok_or_else(|| {
                "mission reference stage report is incomplete".to_owned()
            })?;
            if stage_semantics.source_ordinal() != stage.source_ordinal() {
                return Err("mission reference stage report drifted".to_owned());
            }
            resolve_stage(catalog, stage_semantics, &mut participants)?;

            let objective_semantics =
                objective_bindings.next().ok_or_else(|| {
                    "mission reference objective report is incomplete"
                        .to_owned()
                })?;
            if objective_semantics.source_ordinal()
                != stage.objective().binding().ordinal()
            {
                return Err(
                    "mission reference objective report drifted".to_owned()
                );
            }
            resolve_objective_parameters(
                catalog,
                stage.objective().parameters(),
                &mut participants,
            )?;
            resolve_objective(catalog, objective_semantics, &mut participants)?;

            for condition in stage.conditions() {
                let condition_semantics =
                    condition_bindings.next().ok_or_else(|| {
                        "mission reference condition report is incomplete"
                            .to_owned()
                    })?;
                if condition_semantics.source_ordinal()
                    != condition.binding().ordinal()
                {
                    return Err(
                        "mission reference condition report drifted".to_owned()
                    );
                }
                resolve_condition(
                    catalog,
                    condition_semantics,
                    &mut participants,
                )?;
            }
        }
        participants.sort_by_key(|reference| {
            (reference.source_ordinal, reference.role)
        });
        missions.push(MissionResolvedMissionReferences {
            mission_id: mission.source_mission_id().to_owned(),
            participants,
        });
    }

    if objective_bindings.next().is_some()
        || condition_bindings.next().is_some()
        || initialization_bindings.next().is_some()
        || stage_bindings.next().is_some()
    {
        return Err(
            "mission reference semantic reports contain unowned bindings"
                .to_owned(),
        );
    }
    Ok(MissionReferenceReport { missions })
}

fn push_character(
    catalog: &MissionReferenceCatalog,
    participants: &mut Vec<MissionResolvedParticipantReference>,
    source_ordinal: usize,
    role: MissionParticipantRole,
    source_id: &str,
) -> Result<(), String> {
    participants.push(MissionResolvedParticipantReference {
        source_ordinal,
        role,
        reference: MissionParticipantReference::Character(
            catalog.resolve_character(source_id)?,
        ),
    });
    Ok(())
}

fn push_vehicle(
    catalog: &MissionReferenceCatalog,
    participants: &mut Vec<MissionResolvedParticipantReference>,
    source_ordinal: usize,
    role: MissionParticipantRole,
    source_id: &str,
) -> Result<(), String> {
    participants.push(MissionResolvedParticipantReference {
        source_ordinal,
        role,
        reference: MissionParticipantReference::Vehicle(
            catalog.resolve_vehicle(source_id)?,
        ),
    });
    Ok(())
}

fn resolve_objective_parameters(
    catalog: &MissionReferenceCatalog,
    binding: &super::MissionObjectiveParameterBinding,
    participants: &mut Vec<MissionResolvedParticipantReference>,
) -> Result<(), String> {
    match binding.parameters() {
        MissionObjectiveParameters::BuyVehicle { vehicle_id } => push_vehicle(
            catalog,
            participants,
            binding.ordinal(),
            MissionParticipantRole::ObjectiveBuyVehicle,
            vehicle_id,
        ),
        MissionObjectiveParameters::BuyCostume { costume_id } => {
            push_character(
                catalog,
                participants,
                binding.ordinal(),
                MissionParticipantRole::ObjectiveBuyCostume,
                costume_id,
            )
        },
        MissionObjectiveParameters::EnterVehicle { vehicle_id } => {
            push_vehicle(
                catalog,
                participants,
                binding.ordinal(),
                MissionParticipantRole::ObjectiveEnterVehicle,
                vehicle_id,
            )
        },
        MissionObjectiveParameters::None
        | MissionObjectiveParameters::RoadArrows(_)
        | MissionObjectiveParameters::Race { .. } => Ok(()),
    }
}

fn resolve_objective(
    catalog: &MissionReferenceCatalog,
    binding: &super::MissionObjectiveSemanticBinding,
    participants: &mut Vec<MissionResolvedParticipantReference>,
) -> Result<(), String> {
    for directive in binding.directives() {
        match directive {
            MissionObjectiveDirective::Npc(reference) => push_character(
                catalog,
                participants,
                reference.source_ordinal(),
                MissionParticipantRole::ObjectiveNpc,
                reference.npc_id(),
            )?,
            MissionObjectiveDirective::NpcWaypoint {
                source_ordinal,
                npc_id,
                ..
            } => push_character(
                catalog,
                participants,
                *source_ordinal,
                MissionParticipantRole::ObjectiveNpcWaypoint,
                npc_id,
            )?,
            MissionObjectiveDirective::Driver {
                source_ordinal,
                npc_id,
                vehicle_id,
            } => {
                push_character(
                    catalog,
                    participants,
                    *source_ordinal,
                    MissionParticipantRole::ObjectiveDriverCharacter,
                    npc_id,
                )?;
                push_vehicle(
                    catalog,
                    participants,
                    *source_ordinal,
                    MissionParticipantRole::ObjectiveDriverVehicle,
                    vehicle_id,
                )?;
            },
            MissionObjectiveDirective::RemoveDriver {
                source_ordinal,
                npc_id,
            } => push_character(
                catalog,
                participants,
                *source_ordinal,
                MissionParticipantRole::ObjectiveRemoveDriver,
                npc_id,
            )?,
            MissionObjectiveDirective::RemoveNpc { source_ordinal, npc_id } => {
                push_character(
                    catalog,
                    participants,
                    *source_ordinal,
                    MissionParticipantRole::ObjectiveRemoveNpc,
                    npc_id,
                )?
            },
            MissionObjectiveDirective::TargetVehicle {
                source_ordinal,
                vehicle_id,
            } => push_vehicle(
                catalog,
                participants,
                *source_ordinal,
                MissionParticipantRole::ObjectiveTargetVehicle,
                vehicle_id,
            )?,
            MissionObjectiveDirective::TalkTarget {
                source_ordinal,
                npc_id,
                ..
            } => push_character(
                catalog,
                participants,
                *source_ordinal,
                MissionParticipantRole::ObjectiveTalkTarget,
                npc_id,
            )?,
            MissionObjectiveDirective::DialogueInfo {
                source_ordinal,
                player_character_id,
                npc_character_id,
                ..
            } => {
                push_character(
                    catalog,
                    participants,
                    *source_ordinal,
                    MissionParticipantRole::ObjectiveDialoguePlayer,
                    player_character_id,
                )?;
                push_character(
                    catalog,
                    participants,
                    *source_ordinal,
                    MissionParticipantRole::ObjectiveDialogueNpc,
                    npc_character_id,
                )?;
            },
            _ => {},
        }
    }
    Ok(())
}

fn resolve_condition(
    catalog: &MissionReferenceCatalog,
    binding: &super::MissionConditionSemanticBinding,
    participants: &mut Vec<MissionResolvedParticipantReference>,
) -> Result<(), String> {
    for directive in binding.directives() {
        if let MissionConditionDirective::TargetVehicle {
            source_ordinal,
            vehicle_id,
        } = directive
        {
            push_vehicle(
                catalog,
                participants,
                *source_ordinal,
                MissionParticipantRole::ConditionTargetVehicle,
                vehicle_id,
            )?;
        }
    }
    Ok(())
}

fn resolve_initialization(
    catalog: &MissionReferenceCatalog,
    binding: &super::MissionInitializationBinding,
    participants: &mut Vec<MissionResolvedParticipantReference>,
) -> Result<(), String> {
    for directive in binding.directives() {
        match directive {
            MissionInitializationDirective::InitialPlayerVehicle {
                source_ordinal,
                vehicle_id,
                ..
            } => push_vehicle(
                catalog,
                participants,
                *source_ordinal,
                MissionParticipantRole::MissionInitialPlayerVehicle,
                vehicle_id,
            )?,
            MissionInitializationDirective::PlacePlayerCar {
                source_ordinal,
                vehicle_id,
                ..
            } => push_vehicle(
                catalog,
                participants,
                *source_ordinal,
                MissionParticipantRole::MissionPlacePlayerCar,
                vehicle_id,
            )?,
            _ => {},
        }
    }
    Ok(())
}

fn resolve_stage(
    catalog: &MissionReferenceCatalog,
    binding: &super::MissionStageSemanticBinding,
    participants: &mut Vec<MissionResolvedParticipantReference>,
) -> Result<(), String> {
    match binding.kind() {
        MissionStageKind::LockedVehicle { vehicle_id } => push_vehicle(
            catalog,
            participants,
            binding.source_ordinal(),
            MissionParticipantRole::StageLockedVehicle,
            vehicle_id,
        )?,
        MissionStageKind::LockedCostume { costume_id } => push_character(
            catalog,
            participants,
            binding.source_ordinal(),
            MissionParticipantRole::StageLockedCostume,
            costume_id,
        )?,
        MissionStageKind::Standard { .. } => {},
    }
    for directive in binding.directives() {
        resolve_stage_directive(catalog, directive, participants)?;
    }
    Ok(())
}

fn resolve_stage_directive(
    catalog: &MissionReferenceCatalog,
    directive: &MissionStageDirective,
    participants: &mut Vec<MissionResolvedParticipantReference>,
) -> Result<(), String> {
    match directive {
        MissionStageDirective::Vehicle(vehicle) => {
            push_vehicle(
                catalog,
                participants,
                vehicle.source_ordinal(),
                MissionParticipantRole::StageVehicle,
                vehicle.vehicle_id(),
            )?;
            if let Some(driver) = vehicle.driver_id()
                && driver != "none"
            {
                push_character(
                    catalog,
                    participants,
                    vehicle.source_ordinal(),
                    MissionParticipantRole::StageVehicleDriver,
                    driver,
                )?;
            }
        },
        MissionStageDirective::ActivateVehicle {
            source_ordinal,
            vehicle_id,
            ..
        } => push_vehicle(
            catalog,
            participants,
            *source_ordinal,
            MissionParticipantRole::StageActivateVehicle,
            vehicle_id,
        )?,
        MissionStageDirective::VehicleAiTuning {
            source_ordinal,
            vehicle_id,
            ..
        } => push_vehicle(
            catalog,
            participants,
            *source_ordinal,
            MissionParticipantRole::StageVehicleAi,
            vehicle_id,
        )?,
        MissionStageDirective::TargetCatchupTuning {
            source_ordinal,
            vehicle_id,
            ..
        } => push_vehicle(
            catalog,
            participants,
            *source_ordinal,
            MissionParticipantRole::StageTargetCatchupVehicle,
            vehicle_id,
        )?,
        MissionStageDirective::StageCharacter {
            source_ordinal,
            character_id,
            vehicle_id,
            ..
        } => {
            push_character(
                catalog,
                participants,
                *source_ordinal,
                MissionParticipantRole::StageCharacter,
                character_id,
            )?;
            push_vehicle(
                catalog,
                participants,
                *source_ordinal,
                MissionParticipantRole::StageCharacterVehicle,
                vehicle_id,
            )?;
        },
        MissionStageDirective::PlacePlayerCar {
            source_ordinal,
            vehicle_id,
            ..
        } => push_vehicle(
            catalog,
            participants,
            *source_ordinal,
            MissionParticipantRole::StagePlacePlayerCar,
            vehicle_id,
        )?,
        MissionStageDirective::CharacterToHide {
            source_ordinal,
            character_id,
        } => push_character(
            catalog,
            participants,
            *source_ordinal,
            MissionParticipantRole::StageHiddenCharacter,
            character_id,
        )?,
        MissionStageDirective::CompletionDialog {
            source_ordinal,
            character_id: Some(character_id),
            ..
        } => push_character(
            catalog,
            participants,
            *source_ordinal,
            MissionParticipantRole::StageCompletionDialogCharacter,
            character_id,
        )?,
        MissionStageDirective::RaceCatchupTuning {
            source_ordinal,
            vehicle_id,
            ..
        } => push_vehicle(
            catalog,
            participants,
            *source_ordinal,
            MissionParticipantRole::StageRaceCatchupVehicle,
            vehicle_id,
        )?,
        MissionStageDirective::StartCountdown {
            source_ordinal,
            character_id: Some(character_id),
            ..
        } => push_character(
            catalog,
            participants,
            *source_ordinal,
            MissionParticipantRole::StageCountdownCharacter,
            character_id,
        )?,
        _ => {},
    }
    Ok(())
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_reference/tests.rs"]
mod tests;
