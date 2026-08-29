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
//   - Exact source-backed vehicle-tuning usage bindings from mission commands.
// - Must-Not:
//   - Infer profile ownership, repair authored CON paths, map tuning setters,
//     convert units, or emit Unreal assets.
// - Allows:
//   - Resolve reviewed mission vehicle/config pairs to exact package evidence.
// - Split-When:
//   - Native tuning application gains independent runtime policy.
// - Merge-When:
//   - Mission compilation owns this exact contextual tuning provenance.
// - Summary:
//   - Vehicle-tuning usage binder.
// - Description:
//   - Preserves contextual applications separately from physical tuning cores.
// - Usage:
//   - Runs after mission scope projection and package-index validation.
// - Defaults:
//   - Missing vehicles and ambiguous tuning identities fail closed; an absent
//     tuning source remains explicit unresolved provenance.
//

//! Exact source-backed vehicle-tuning usage bindings.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use super::index::{PackageRole, PhaseThreePackageIndex};
use super::mission_reference::{
    MissionReferenceCatalog, MissionVehicleCatalogReference,
};
use super::mission_scope::{MissionScopeCommand, MissionScopeReport};

const NORMALIZED_TUNING_ROOT: &str = "extracted/game/scripts/cars/";
const CREATE_CHASE_MANAGER_COMMAND: &str =
    concat!("create", "chase", "manager");

/// Context in which one reviewed tuning application was authored.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VehicleTuningUsageScope {
    /// Command was authored outside a selected mission.
    Unscoped,
    /// Command was authored directly in a selected mission.
    Mission,
    /// Command was authored directly in one mission stage.
    Stage,
    /// Command was authored inside the stage's root objective.
    Objective,
}

impl VehicleTuningUsageScope {
    /// Return the stable serialization token.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unscoped => "unscoped",
            Self::Mission => "mission",
            Self::Stage => "stage",
            Self::Objective => "objective",
        }
    }
}

/// One exact normalized tuning source selected by an authored CON path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VehicleTuningSourceReference {
    source_id: String,
    package_id: String,
    package_subcategory: String,
}

impl VehicleTuningSourceReference {
    /// Return the exact minor-unit source identity.
    #[must_use]
    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    /// Return the exact owning package identity.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the exact package subcategory.
    #[must_use]
    pub fn package_subcategory(&self) -> &str {
        &self.package_subcategory
    }
}

/// Deterministic exact lookup from authored CON paths to tuning sources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VehicleTuningSourceCatalog {
    by_path: BTreeMap<String, VehicleTuningSourceReference>,
}

impl VehicleTuningSourceCatalog {
    /// Build a tuning-source catalog from validated phase-three packages.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed tuning package membership or portable
    /// path ambiguity.
    pub fn from_package_index(
        index: &PhaseThreePackageIndex,
    ) -> Result<Self, String> {
        let mut by_path = BTreeMap::new();
        for package in index.packages() {
            if package.category() != "vehicle-tuning" {
                continue;
            }
            if !package
                .subcategory()
                .split('/')
                .any(|segment| segment == "vehicle-tuning")
            {
                return Err(
                    "vehicle tuning package subcategory drifted".to_owned(),
                );
            }
            for member in package.members() {
                if member.kind != "vehicle-tuning"
                    || member.unit_type != "config"
                    || member.role != PackageRole::Metadata
                    || member.source_chunk_kind != "none"
                {
                    return Err(
                        "vehicle tuning package contains a non-tuning member"
                            .to_owned(),
                    );
                }
                let key = normalized_member_key(&member.path)?;
                let reference = VehicleTuningSourceReference {
                    source_id: member.id.clone(),
                    package_id: package.package_id.clone(),
                    package_subcategory: package.subcategory().to_owned(),
                };
                if by_path.insert(key, reference).is_some() {
                    return Err(
                        "vehicle tuning source path is ambiguous".to_owned(),
                    );
                }
            }
        }
        Ok(Self { by_path })
    }

    #[cfg(test)]
    pub(crate) fn from_entries_for_tests(
        entries: &[(&str, &str, &str, &str)],
    ) -> Result<Self, String> {
        let mut by_path = BTreeMap::new();
        for (con_file, source_id, package_id, package_subcategory) in entries {
            let key = normalize_authored_con_path(con_file)?;
            let reference = VehicleTuningSourceReference {
                source_id: (*source_id).to_owned(),
                package_id: (*package_id).to_owned(),
                package_subcategory: (*package_subcategory).to_owned(),
            };
            if by_path.insert(key, reference).is_some() {
                return Err(
                    "vehicle tuning source path is ambiguous".to_owned(),
                );
            }
        }
        Ok(Self { by_path })
    }

    fn resolve_optional(
        &self,
        con_file: &str,
    ) -> Result<Option<VehicleTuningSourceReference>, String> {
        let key = normalize_authored_con_path(con_file)?;
        Ok(self.by_path.get(&key).cloned())
    }
}

/// One exact contextual tuning application retained from a mission source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VehicleTuningUsageBinding {
    mission_source_id: String,
    source_ordinal: usize,
    command: String,
    scope: VehicleTuningUsageScope,
    owner_mission_id: Option<String>,
    owner_stage_sequence_ordinal: Option<usize>,
    owner_objective_source_ordinal: Option<usize>,
    con_file: String,
    vehicle: MissionVehicleCatalogReference,
    tuning_source: Option<VehicleTuningSourceReference>,
}

impl VehicleTuningUsageBinding {
    /// Return the normalized mission-script source identity.
    #[must_use]
    pub fn mission_source_id(&self) -> &str {
        &self.mission_source_id
    }

    /// Return the exact source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the normalized command identity.
    #[must_use]
    pub fn command(&self) -> &str {
        &self.command
    }

    /// Return the authored source scope.
    #[must_use]
    pub const fn scope(&self) -> VehicleTuningUsageScope {
        self.scope
    }

    /// Return the owning mission identity when selected.
    #[must_use]
    pub fn owner_mission_id(&self) -> Option<&str> {
        self.owner_mission_id.as_deref()
    }

    /// Return dense owning stage order when stage-scoped.
    #[must_use]
    pub const fn owner_stage_sequence_ordinal(&self) -> Option<usize> {
        self.owner_stage_sequence_ordinal
    }

    /// Return the owning objective source ordinal when objective-scoped.
    #[must_use]
    pub const fn owner_objective_source_ordinal(&self) -> Option<usize> {
        self.owner_objective_source_ordinal
    }

    /// Return the exact authored CON path.
    #[must_use]
    pub fn con_file(&self) -> &str {
        &self.con_file
    }

    /// Return exact physical vehicle package provenance.
    #[must_use]
    pub const fn vehicle(&self) -> &MissionVehicleCatalogReference {
        &self.vehicle
    }

    /// Return exact tuning source provenance when indexed.
    #[must_use]
    pub const fn tuning_source(&self) -> Option<&VehicleTuningSourceReference> {
        self.tuning_source.as_ref()
    }
}

/// Ordered contextual tuning applications for one normalized mission source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VehicleTuningUsageReport {
    bindings: Vec<VehicleTuningUsageBinding>,
}

impl VehicleTuningUsageReport {
    /// Return bindings in authored statement order.
    #[must_use]
    pub fn bindings(&self) -> &[VehicleTuningUsageBinding] {
        &self.bindings
    }
}

/// Bind reviewed tuning applications from one mission scope projection.
///
/// # Errors
///
/// Returns an error for malformed reviewed command shapes, missing/ambiguous
/// physical vehicles, unsafe CON paths, or duplicate occurrence identities.
pub fn preflight_vehicle_tuning_usages(
    mission_source_id: &str,
    scopes: &MissionScopeReport,
    vehicles: &MissionReferenceCatalog,
    tuning_sources: &VehicleTuningSourceCatalog,
) -> Result<VehicleTuningUsageReport, String> {
    validate_source_id(mission_source_id)?;
    let mut bindings = Vec::new();
    for command in scopes.unscoped_commands() {
        bind_usage_command(
            &mut bindings,
            mission_source_id,
            command,
            VehicleTuningUsageScope::Unscoped,
            None,
            None,
            None,
            vehicles,
            tuning_sources,
        )?;
    }
    for mission in scopes.missions() {
        for command in mission.commands() {
            bind_usage_command(
                &mut bindings,
                mission_source_id,
                command,
                VehicleTuningUsageScope::Mission,
                Some(mission.source_mission_id()),
                None,
                None,
                vehicles,
                tuning_sources,
            )?;
        }
        for stage in mission.stages() {
            for command in stage.commands() {
                bind_usage_command(
                    &mut bindings,
                    mission_source_id,
                    command,
                    VehicleTuningUsageScope::Stage,
                    Some(mission.source_mission_id()),
                    Some(stage.sequence_ordinal()),
                    None,
                    vehicles,
                    tuning_sources,
                )?;
            }
            for command in stage.objective().commands() {
                bind_usage_parts(
                    &mut bindings,
                    mission_source_id,
                    command.ordinal(),
                    command.command(),
                    command.arguments(),
                    VehicleTuningUsageScope::Objective,
                    Some(mission.source_mission_id()),
                    Some(stage.sequence_ordinal()),
                    Some(stage.objective().binding().ordinal()),
                    vehicles,
                    tuning_sources,
                )?;
            }
        }
    }
    bindings.sort_by_key(VehicleTuningUsageBinding::source_ordinal);
    let mut ordinals = BTreeSet::new();
    if bindings
        .iter()
        .any(|binding| !ordinals.insert(binding.source_ordinal()))
    {
        return Err("vehicle tuning usage occurrence is duplicated".to_owned());
    }
    Ok(VehicleTuningUsageReport { bindings })
}

#[expect(
    clippy::too_many_arguments,
    reason = "Explicit occurrence provenance keeps tuning usage auditable."
)]
fn bind_usage_command(
    bindings: &mut Vec<VehicleTuningUsageBinding>,
    mission_source_id: &str,
    command: &MissionScopeCommand,
    scope: VehicleTuningUsageScope,
    owner_mission_id: Option<&str>,
    owner_stage_sequence_ordinal: Option<usize>,
    owner_objective_source_ordinal: Option<usize>,
    vehicles: &MissionReferenceCatalog,
    tuning_sources: &VehicleTuningSourceCatalog,
) -> Result<(), String> {
    bind_usage_parts(
        bindings,
        mission_source_id,
        command.source_ordinal(),
        command.name(),
        command.arguments(),
        scope,
        owner_mission_id,
        owner_stage_sequence_ordinal,
        owner_objective_source_ordinal,
        vehicles,
        tuning_sources,
    )
}

#[expect(
    clippy::too_many_arguments,
    reason = "Explicit occurrence provenance keeps tuning usage auditable."
)]
fn bind_usage_parts(
    bindings: &mut Vec<VehicleTuningUsageBinding>,
    mission_source_id: &str,
    source_ordinal: usize,
    command: &str,
    arguments: &[String],
    scope: VehicleTuningUsageScope,
    owner_mission_id: Option<&str>,
    owner_stage_sequence_ordinal: Option<usize>,
    owner_objective_source_ordinal: Option<usize>,
    vehicles: &MissionReferenceCatalog,
    tuning_sources: &VehicleTuningSourceCatalog,
) -> Result<(), String> {
    let (vehicle_id, con_file) = match command {
        "addstagevehicle" => match arguments {
            [vehicle, _, _, con] | [vehicle, _, _, con, _] => {
                (vehicle, con)
            },
            _ => {
                return Err(
                    "vehicle tuning stage usage shape is not reviewed"
                        .to_owned(),
                );
            },
        },
        "initlevelplayervehicle" => match arguments {
            [_, _, _] => return Ok(()),
            [vehicle, _, _, con] => (vehicle, con),
            _ => {
                return Err(
                    "vehicle tuning player usage shape is not reviewed"
                        .to_owned(),
                );
            },
        },
        CREATE_CHASE_MANAGER_COMMAND => match arguments {
            [vehicle, con, _] => (vehicle, con),
            _ => {
                return Err(
                    "vehicle tuning chase usage shape is not reviewed"
                        .to_owned(),
                );
            },
        },
        _ => return Ok(()),
    };
    let scope_is_reviewed = match command {
        "addstagevehicle" => matches!(
            scope,
            VehicleTuningUsageScope::Stage | VehicleTuningUsageScope::Objective
        ),
        "initlevelplayervehicle" | CREATE_CHASE_MANAGER_COMMAND => {
            scope == VehicleTuningUsageScope::Unscoped
        },
        _ => false,
    };
    if source_ordinal == 0 || !scope_is_reviewed {
        return Err("vehicle tuning usage scope drifted".to_owned());
    }
    validate_scope_ownership(
        scope,
        owner_mission_id,
        owner_stage_sequence_ordinal,
        owner_objective_source_ordinal,
    )?;
    let vehicle = vehicles
        .resolve_optional_vehicle(vehicle_id)?
        .ok_or_else(|| {
            "vehicle tuning usage vehicle has no physical package".to_owned()
        })?;
    let tuning_source = tuning_sources.resolve_optional(con_file)?;
    bindings.push(VehicleTuningUsageBinding {
        mission_source_id: mission_source_id.to_owned(),
        source_ordinal,
        command: command.to_owned(),
        scope,
        owner_mission_id: owner_mission_id.map(str::to_owned),
        owner_stage_sequence_ordinal,
        owner_objective_source_ordinal,
        con_file: con_file.clone(),
        vehicle,
        tuning_source,
    });
    Ok(())
}

fn validate_scope_ownership(
    scope: VehicleTuningUsageScope,
    owner_mission_id: Option<&str>,
    owner_stage_sequence_ordinal: Option<usize>,
    owner_objective_source_ordinal: Option<usize>,
) -> Result<(), String> {
    let valid = match scope {
        VehicleTuningUsageScope::Unscoped => {
            owner_mission_id.is_none()
                && owner_stage_sequence_ordinal.is_none()
                && owner_objective_source_ordinal.is_none()
        },
        VehicleTuningUsageScope::Mission => {
            owner_mission_id.is_some()
                && owner_stage_sequence_ordinal.is_none()
                && owner_objective_source_ordinal.is_none()
        },
        VehicleTuningUsageScope::Stage => {
            owner_mission_id.is_some()
                && owner_stage_sequence_ordinal.is_some()
                && owner_objective_source_ordinal.is_none()
        },
        VehicleTuningUsageScope::Objective => {
            owner_mission_id.is_some()
                && owner_stage_sequence_ordinal.is_some()
                && owner_objective_source_ordinal.is_some()
        },
    };
    if !valid {
        return Err("vehicle tuning usage owner scope drifted".to_owned());
    }
    Ok(())
}

fn normalized_member_key(path: &str) -> Result<String, String> {
    let lower = path.to_ascii_lowercase();
    let relative = lower
        .strip_prefix(NORMALIZED_TUNING_ROOT)
        .ok_or_else(|| {
            "vehicle tuning member path is outside tuning root".to_owned()
        })?;
    let con_file = relative
        .strip_suffix(".json")
        .ok_or_else(|| {
            "vehicle tuning member path is not normalized JSON".to_owned()
        })?;
    normalize_authored_con_path(con_file)
}

fn normalize_authored_con_path(value: &str) -> Result<String, String> {
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
    {
        return Err("vehicle tuning CON path is malformed".to_owned());
    }
    let normalized = value.replace('\\', "/").to_ascii_lowercase();
    if normalized.starts_with('/')
        || normalized.get(1..2) == Some(":")
        || normalized.split('/').any(|part| {
            part.is_empty() || part == "." || part == ".."
        })
    {
        return Err(
            "vehicle tuning CON path is not portable relative".to_owned(),
        );
    }
    let relative = normalized
        .strip_prefix("scripts/cars/")
        .or_else(|| normalized.strip_prefix("cars/"))
        .unwrap_or(&normalized);
    if !Path::new(relative)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("con"))
        || relative.len() <= ".con".len()
    {
        return Err("vehicle tuning CON path is not a CON source".to_owned());
    }
    Ok(relative.to_owned())
}

fn validate_source_id(value: &str) -> Result<(), String> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || bytes.windows(2).any(|pair| pair == b"--")
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
    {
        return Err(
            "vehicle tuning usage source identity is not canonical".to_owned(),
        );
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: literal
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/vehicle_tuning_usage/tests.rs"]
mod tests;
