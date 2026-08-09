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
//   - Reviewed mission-scope restart, load, and initial-vehicle references.
// - Must-Not:
//   - Invent meaning for undocumented dyna-load compatibility arguments.
// - Allows:
//   - Type documented locator and vehicle fields and exact P3D load references.
// - Split-When:
//   - Dyna-load or restart policy gains an independent runtime lifecycle.
// - Merge-When:
//   - Mission definition compilation owns this identical evidence boundary.
// - Summary:
//   - Mission initialization semantic compiler.
// - Description:
//   - Types reviewed mission-scope load and restart directives fail closed.
// - Usage:
//   - Runs after lossless mission scope projection succeeds.
// - Defaults:
//   - Malformed identities and P3D paths fail closed.
//

//! Typed mission-scope initialization and restart evidence.

use super::{MissionScopeCommand, MissionScopeReport};

/// One reviewed mission-scope initialization directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionInitializationDirective {
    /// Restart the player inside the current vehicle at a `CarStart` locator.
    ResetPlayerInCar {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source vehicle locator identity.
        vehicle_locator_id: String,
    },
    /// Restart the player on foot with the current vehicle at another locator.
    ResetPlayerOutCar {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source player locator identity.
        player_locator_id: String,
        /// Exact source vehicle locator identity.
        vehicle_locator_id: String,
    },
    /// Walk the player toward this locator after mission selection/restart.
    InitialWalk {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source locator identity.
        locator_id: String,
    },
    /// Exact dynamic-load evidence for one mission restart.
    DynamicLoad {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact first legacy source argument.
        source_data: String,
        /// Exact `.p3d` references parsed from the source data string.
        p3d_files: Vec<String>,
        /// Optional second legacy argument whose semantics remain unassigned.
        legacy_argument: Option<String>,
    },
    /// Load street-race props when the selected mission begins.
    StreetRacePropsLoad {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact authored Dyna Load Data string, including its terminal `;`.
        source_data: String,
        /// Exact `.p3d` references extracted from the source data.
        p3d_files: Vec<String>,
    },
    /// Unload street-race props when the selected mission ends.
    StreetRacePropsUnload {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact authored Dyna Load Data string, including its terminal `:`.
        source_data: String,
        /// Exact `.p3d` references extracted from the source data.
        p3d_files: Vec<String>,
    },
    /// Mission explicitly requires forced-car behavior.
    ForcedCar {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Initialize the mission player vehicle at an exact locator.
    InitialPlayerVehicle {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source vehicle identity.
        vehicle_id: String,
        /// Exact source `CarStart` locator identity.
        locator_id: String,
        /// Exact source initialization mode token.
        mode_token: String,
    },
}

/// Typed initialization evidence for one selected mission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionInitializationBinding {
    mission_id: String,
    directives: Vec<MissionInitializationDirective>,
}

impl MissionInitializationBinding {
    /// Return the exact selected mission identity.
    #[must_use]
    pub fn mission_id(&self) -> &str {
        &self.mission_id
    }

    /// Return typed mission-scope directives in source order.
    #[must_use]
    pub fn directives(&self) -> &[MissionInitializationDirective] {
        &self.directives
    }
}

/// Complete typed mission-scope initialization evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionInitializationReport {
    missions: Vec<MissionInitializationBinding>,
}

impl MissionInitializationReport {
    /// Return mission bindings in source order.
    #[must_use]
    pub fn missions(&self) -> &[MissionInitializationBinding] {
        &self.missions
    }
}

/// Compile reviewed mission-scope initialization and restart evidence.
///
/// # Errors
///
/// Returns an error for malformed locator, vehicle, mode, or P3D evidence.
pub fn preflight_mission_initialization(
    scopes: &MissionScopeReport,
) -> Result<MissionInitializationReport, String> {
    let mut missions = Vec::with_capacity(scopes.missions().len());
    for mission in scopes.missions() {
        let mut directives = Vec::new();
        for command in mission.commands() {
            if let Some(directive) = compile_directive(command)? {
                directives.push(directive);
            }
        }
        missions.push(MissionInitializationBinding {
            mission_id: mission.source_mission_id().to_owned(),
            directives,
        });
    }
    Ok(MissionInitializationReport { missions })
}

fn compile_directive(
    command: &MissionScopeCommand,
) -> Result<Option<MissionInitializationDirective>, String> {
    let directive = match command.name() {
        "setmissionresetplayerincar" => {
            Some(MissionInitializationDirective::ResetPlayerInCar {
                source_ordinal: command.source_ordinal(),
                vehicle_locator_id: required_identity(
                    command.arguments(),
                    "mission reset vehicle locator",
                )?,
            })
        },
        "setmissionresetplayeroutcar" => {
            let [player, vehicle] = command.arguments() else {
                return Err(
                    "mission reset-out-car parameter shape is not reviewed"
                        .to_owned(),
                );
            };
            validate_identity(player, "mission reset player locator")?;
            validate_identity(vehicle, "mission reset vehicle locator")?;
            Some(MissionInitializationDirective::ResetPlayerOutCar {
                source_ordinal: command.source_ordinal(),
                player_locator_id: player.clone(),
                vehicle_locator_id: vehicle.clone(),
            })
        },
        "setinitialwalk" => Some(MissionInitializationDirective::InitialWalk {
            source_ordinal: command.source_ordinal(),
            locator_id: required_identity(
                command.arguments(),
                "mission initial-walk locator",
            )?,
        }),
        "setdynaloaddata" => Some(compile_dynamic_load(command)?),
        "streetracepropsload" => {
            let (source_data, p3d_files) =
                compile_street_race_props(command.arguments(), ';', "load")?;
            Some(MissionInitializationDirective::StreetRacePropsLoad {
                source_ordinal: command.source_ordinal(),
                source_data,
                p3d_files,
            })
        },
        "streetracepropsunload" => {
            let (source_data, p3d_files) =
                compile_street_race_props(command.arguments(), ':', "unload")?;
            Some(MissionInitializationDirective::StreetRacePropsUnload {
                source_ordinal: command.source_ordinal(),
                source_data,
                p3d_files,
            })
        },
        "setforcedcar" => {
            if !command.arguments().is_empty() {
                return Err(
                    "mission forced-car marker has arguments".to_owned()
                );
            }
            Some(MissionInitializationDirective::ForcedCar {
                source_ordinal: command.source_ordinal(),
            })
        },
        "initlevelplayervehicle" => Some(compile_initial_vehicle(command)?),
        _ => None,
    };
    Ok(directive)
}

fn compile_dynamic_load(
    command: &MissionScopeCommand,
) -> Result<MissionInitializationDirective, String> {
    let (source_data, legacy_argument) = match command.arguments() {
        [data] => (data, None),
        [data, legacy] => (data, Some(legacy.clone())),
        _ => {
            return Err(
                "mission dyna-load parameter shape is not reviewed".to_owned()
            );
        },
    };
    if source_data.is_empty() || source_data.chars().any(char::is_control) {
        return Err("mission dyna-load source data is malformed".to_owned());
    }
    if let Some(argument) = &legacy_argument {
        validate_identity(argument, "mission dyna-load legacy argument")?;
    }
    let mut p3d_files = Vec::new();
    for raw in source_data.split(';').filter(|value| !value.is_empty()) {
        let value = raw.strip_suffix('@').unwrap_or(raw);
        validate_p3d_path(value)?;
        p3d_files.push(value.to_owned());
    }
    if p3d_files.is_empty() {
        return Err("mission dyna-load contains no P3D references".to_owned());
    }
    Ok(MissionInitializationDirective::DynamicLoad {
        source_ordinal: command.source_ordinal(),
        source_data: source_data.clone(),
        p3d_files,
        legacy_argument,
    })
}

fn compile_street_race_props(
    arguments: &[String],
    terminator: char,
    label: &str,
) -> Result<(String, Vec<String>), String> {
    let [source_data] = arguments else {
        return Err(format!("street-race props {label} shape is not reviewed"));
    };
    let Some(body) = source_data.strip_suffix(terminator) else {
        return Err(format!(
            "street-race props {label} Dyna Load Data terminator drifted"
        ));
    };
    if body.is_empty() || body.chars().any(char::is_control) {
        return Err(format!(
            "street-race props {label} Dyna Load Data is malformed"
        ));
    }
    let mut p3d_files = Vec::new();
    for raw in body.split(';') {
        if raw.is_empty() {
            return Err(format!(
                "street-race props {label} contains an empty P3D reference"
            ));
        }
        validate_p3d_path(raw)?;
        p3d_files.push(raw.to_owned());
    }
    Ok((source_data.clone(), p3d_files))
}

fn compile_initial_vehicle(
    command: &MissionScopeCommand,
) -> Result<MissionInitializationDirective, String> {
    let [vehicle, locator, mode] = command.arguments() else {
        return Err(
            "mission initial-player-vehicle parameter shape is not reviewed"
                .to_owned(),
        );
    };
    validate_identity(vehicle, "mission initial vehicle")?;
    validate_identity(locator, "mission initial vehicle locator")?;
    if !matches!(mode.as_str(), "DEFAULT" | "OTHER") {
        return Err("mission initial vehicle mode is not reviewed".to_owned());
    }
    Ok(MissionInitializationDirective::InitialPlayerVehicle {
        source_ordinal: command.source_ordinal(),
        vehicle_id: vehicle.clone(),
        locator_id: locator.clone(),
        mode_token: mode.clone(),
    })
}

fn required_identity(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        let mut error = label.to_owned();
        error.push_str(" shape is not reviewed");
        return Err(error);
    };
    validate_identity(value, label)?;
    Ok(value.clone())
}

fn validate_identity(value: &str, label: &str) -> Result<(), String> {
    let valid = !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-')
        });
    if valid {
        Ok(())
    } else {
        let mut error = label.to_owned();
        error.push_str(" identity is malformed");
        Err(error)
    }
}

fn validate_p3d_path(value: &str) -> Result<(), String> {
    let valid = !value.is_empty()
        && value.to_ascii_lowercase().ends_with(".p3d")
        && !value.contains("..")
        && !value.starts_with('/')
        && !value.starts_with(char::from(92))
        && !value.chars().any(char::is_control);
    if valid {
        Ok(())
    } else {
        Err("mission dyna-load P3D reference is malformed".to_owned())
    }
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_initialization/tests.rs"]
mod tests;
