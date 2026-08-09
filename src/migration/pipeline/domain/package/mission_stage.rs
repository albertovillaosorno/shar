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
//   - Reviewed stage header, timing, checkpoint, message, vehicle, and waypoint
//     evidence.
// - Must-Not:
//   - Interpret unrelated stage commands or resolve world objects by guesswork.
// - Allows:
//   - Compile documented stage forms into deterministic typed evidence.
// - Split-When:
//   - One stage directive family gains an independent runtime lifecycle.
// - Merge-When:
//   - Mission definition compilation owns this exact stage evidence boundary.
// - Summary:
//   - Mission stage semantic compiler.
// - Description:
//   - Types reviewed AddStage arguments and selected direct stage commands.
// - Usage:
//   - Runs after lossless mission scope projection succeeds.
// - Defaults:
//   - Unknown stage forms and malformed reviewed directive values fail closed.
//

//! Typed compilation of reviewed mission stage evidence.

use super::{MissionScopeReport, MissionScopeStage};

/// Reviewed kind carried by one `AddStage` declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionStageKind {
    /// Standard stage with preserved opaque legacy flags and final marker.
    Standard {
        /// Exact reviewed numeric flags from the legacy `AddStage` call.
        legacy_flags: Option<u8>,
        /// Whether the source explicitly marks this as the final stage.
        final_stage: bool,
    },
    /// Locked stage requiring one exact vehicle identity.
    LockedVehicle {
        /// Exact required source vehicle identity.
        vehicle_id: String,
    },
    /// Locked stage requiring one exact costume identity.
    LockedCostume {
        /// Exact required source costume identity.
        costume_id: String,
    },
}

/// Text namespace selected by the stage message command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionStageMessageKind {
    /// `MISSION_OBJECTIVE_000..299` namespace.
    Objective,
    /// `INGAME_MESSAGE_00..19` namespace for locked stages.
    Locked,
}

/// One reviewed vehicle declaration attached directly to a stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageVehicleReference {
    source_ordinal: usize,
    vehicle_id: String,
    locator_id: String,
    behaviour: String,
    con_file: String,
    driver_id: Option<String>,
}

impl MissionStageVehicleReference {
    /// Return source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return exact vehicle identity.
    #[must_use]
    pub fn vehicle_id(&self) -> &str {
        &self.vehicle_id
    }

    /// Return exact `CarStart` locator identity.
    #[must_use]
    pub fn locator_id(&self) -> &str {
        &self.locator_id
    }

    /// Return exact source vehicle behaviour token.
    #[must_use]
    pub fn behaviour(&self) -> &str {
        &self.behaviour
    }

    /// Return exact source CON path.
    #[must_use]
    pub fn con_file(&self) -> &str {
        &self.con_file
    }

    /// Return optional exact driver identity.
    #[must_use]
    pub fn driver_id(&self) -> Option<&str> {
        self.driver_id.as_deref()
    }
}

/// Selected typed direct stage directives.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionStageDirective {
    /// Replace the stage timer with this many seconds.
    SetTimeSeconds {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact positive timer value in seconds.
        seconds: u32,
    },
    /// Add this many seconds to the current stage timer.
    AddTimeSeconds {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source value in seconds.
        source_seconds: u32,
        /// Effective legacy addition; source zero adds one second.
        effective_seconds: u32,
    },
    /// Restart/select checkpoint begins at this stage.
    ResetCheckpoint {
        /// Source statement ordinal.
        source_ordinal: usize,
    },
    /// Stage message index in its stage-dependent namespace.
    MessageIndex {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Stage-dependent message namespace.
        kind: MissionStageMessageKind,
        /// Exact source message index.
        index: u16,
        /// Optional exact argument documented by the legacy API as unused.
        unused_argument: Option<String>,
    },
    /// Vehicle declaration with documented field roles.
    Vehicle(MissionStageVehicleReference),
    /// Activate an existing mission vehicle with reviewed field roles.
    ActivateVehicle {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact mission vehicle identity.
        vehicle_id: String,
        /// Exact `CarStart` locator identity, including `NULL` when authored.
        locator_id: String,
        /// Exact reviewed vehicle behaviour token.
        behaviour: String,
    },
    /// Select the exact HUD icon sprite for the stage.
    HudIcon {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source sprite identity.
        sprite_id: String,
    },
    /// Preserve the required but documented-unused fade-out argument.
    FadeOutLegacyArgument {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source argument retained as compatibility evidence.
        source_value: String,
    },
    /// Preserve the required but documented-unused iris-wipe argument.
    IrisWipeLegacyArgument {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source argument retained as compatibility evidence.
        source_value: String,
    },
    /// Set the maximum number of traffic cars active for the stage.
    MaxTrafficCars {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact reviewed car limit; the current source corpus uses 1 through
        /// 5.
        cars: u8,
    },
    /// Type-0 event locator used as a stage waypoint.
    Waypoint {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact source locator identity.
        locator_id: String,
    },
}

/// Typed stage semantics for one projected source stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageSemanticBinding {
    source_ordinal: usize,
    sequence_ordinal: usize,
    kind: MissionStageKind,
    directives: Vec<MissionStageDirective>,
}

impl MissionStageSemanticBinding {
    /// Return source `AddStage` ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return dense source stage order.
    #[must_use]
    pub const fn sequence_ordinal(&self) -> usize {
        self.sequence_ordinal
    }

    /// Return reviewed stage header semantics.
    #[must_use]
    pub const fn kind(&self) -> &MissionStageKind {
        &self.kind
    }

    /// Return selected typed stage directives in source order.
    #[must_use]
    pub fn directives(&self) -> &[MissionStageDirective] {
        &self.directives
    }
}

/// Typed stage semantics for all projected missions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageSemanticReport {
    stages: Vec<MissionStageSemanticBinding>,
}

impl MissionStageSemanticReport {
    /// Return typed stages in mission/source order.
    #[must_use]
    pub fn stages(&self) -> &[MissionStageSemanticBinding] {
        &self.stages
    }
}

/// Compile reviewed stage header and direct-command semantics.
///
/// # Errors
///
/// Returns an error when a stage header, timer, message, vehicle, or waypoint
/// value is outside the reviewed contract.
pub fn preflight_mission_stage_semantics(
    scopes: &MissionScopeReport,
) -> Result<MissionStageSemanticReport, String> {
    let mut stages = Vec::new();
    for mission in scopes.missions() {
        for stage in mission.stages() {
            stages.push(compile_stage(stage)?);
        }
    }
    Ok(MissionStageSemanticReport { stages })
}

fn compile_stage(
    stage: &MissionScopeStage,
) -> Result<MissionStageSemanticBinding, String> {
    let kind = compile_stage_kind(stage.legacy_parameters())?;
    let mut directives = Vec::<(usize, MissionStageDirective)>::new();
    for command in stage.commands() {
        if let Some(directive) = compile_stage_directive(
            &kind,
            command.source_ordinal(),
            command.name(),
            command.arguments(),
        )? {
            directives.push((command.source_ordinal(), directive));
        }
    }
    for command in stage.objective().commands() {
        if let Some(directive) = compile_stage_directive(
            &kind,
            command.ordinal(),
            command.command(),
            command.arguments(),
        )? {
            directives.push((command.ordinal(), directive));
        }
    }
    directives.sort_by_key(|(ordinal, _directive)| *ordinal);
    Ok(MissionStageSemanticBinding {
        source_ordinal: stage.source_ordinal(),
        sequence_ordinal: stage.sequence_ordinal(),
        kind,
        directives: directives
            .into_iter()
            .map(|(_ordinal, directive)| directive)
            .collect(),
    })
}

fn compile_stage_directive(
    kind: &MissionStageKind,
    source_ordinal: usize,
    name: &str,
    arguments: &[String],
) -> Result<Option<MissionStageDirective>, String> {
    let directive = match name {
        "setstagetime" => Some(MissionStageDirective::SetTimeSeconds {
            source_ordinal,
            seconds: parse_positive_seconds(arguments, "stage timer")?,
        }),
        "addstagetime" => {
            let source_seconds =
                parse_nonnegative_seconds(arguments, "stage added time")?;
            Some(MissionStageDirective::AddTimeSeconds {
                source_ordinal,
                source_seconds,
                effective_seconds: source_seconds.max(1),
            })
        },
        "reset_to_here" => {
            Some(MissionStageDirective::ResetCheckpoint { source_ordinal })
        },
        "setstagemessageindex" => {
            Some(compile_message(kind, source_ordinal, arguments)?)
        },
        "addstagevehicle" => Some(MissionStageDirective::Vehicle(
            compile_vehicle(source_ordinal, arguments)?,
        )),
        "activatevehicle" => {
            let [vehicle, locator, behaviour] = arguments else {
                return Err("stage vehicle activation shape drifted".to_owned());
            };
            validate_identity(vehicle, "activated stage vehicle")?;
            validate_identity(locator, "stage vehicle activation locator")?;
            validate_token(behaviour, "stage vehicle activation behaviour")?;
            Some(MissionStageDirective::ActivateVehicle {
                source_ordinal,
                vehicle_id: vehicle.clone(),
                locator_id: locator.clone(),
                behaviour: behaviour.clone(),
            })
        },
        "sethudicon" => Some(MissionStageDirective::HudIcon {
            source_ordinal,
            sprite_id: required_identity(arguments, "stage HUD icon sprite")?,
        }),
        "setfadeout" => Some(MissionStageDirective::FadeOutLegacyArgument {
            source_ordinal,
            source_value: required_legacy_argument(
                arguments,
                "stage fade-out unused argument",
            )?,
        }),
        "setiriswipe" => Some(MissionStageDirective::IrisWipeLegacyArgument {
            source_ordinal,
            source_value: required_legacy_argument(
                arguments,
                "stage iris-wipe unused argument",
            )?,
        }),
        "setmaxtraffic" => Some(MissionStageDirective::MaxTrafficCars {
            source_ordinal,
            cars: parse_max_traffic(arguments)?,
        }),
        "addstagewaypoint" => Some(MissionStageDirective::Waypoint {
            source_ordinal,
            locator_id: required_identity(arguments, "stage waypoint locator")?,
        }),
        _ => None,
    };
    Ok(directive)
}

fn compile_stage_kind(
    parameters: &[String],
) -> Result<MissionStageKind, String> {
    match parameters {
        [] => Ok(MissionStageKind::Standard {
            legacy_flags: None,
            final_stage: false,
        }),
        [final_token] if final_token == "final" => {
            Ok(MissionStageKind::Standard {
                legacy_flags: None,
                final_stage: true,
            })
        },
        [flags] => Ok(MissionStageKind::Standard {
            legacy_flags: Some(parse_legacy_stage_flags(flags)?),
            final_stage: false,
        }),
        [flags, final_token] if final_token == "final" => {
            Ok(MissionStageKind::Standard {
                legacy_flags: Some(parse_legacy_stage_flags(flags)?),
                final_stage: true,
            })
        },
        [locked, kind, identity] if locked == "locked" && kind == "car" => {
            validate_identity(identity, "locked-stage vehicle")?;
            Ok(MissionStageKind::LockedVehicle {
                vehicle_id: identity.clone(),
            })
        },
        [locked, kind, identity] if locked == "locked" && kind == "skin" => {
            validate_identity(identity, "locked-stage costume")?;
            Ok(MissionStageKind::LockedCostume {
                costume_id: identity.clone(),
            })
        },
        _ => Err("mission stage header parameters are not reviewed".to_owned()),
    }
}

fn parse_legacy_stage_flags(value: &str) -> Result<u8, String> {
    let flags = value.parse::<u8>().map_err(|_error| {
        "mission legacy stage flags are not numeric".to_owned()
    })?;
    if matches!(
        flags,
        0 | 1
            | 2
            | 3
            | 4
            | 5
            | 7
            | 8
            | 9
            | 10
            | 15
            | 16
            | 18
            | 22
            | 25
            | 36
            | 37
    ) {
        Ok(flags)
    } else {
        Err("mission legacy stage flags are not reviewed".to_owned())
    }
}

fn compile_message(
    kind: &MissionStageKind,
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageDirective, String> {
    let (raw, unused_argument) = match arguments {
        [raw] => (raw, None),
        [raw, unused] => (raw, Some(unused.clone())),
        _ => return Err("stage message index shape is not reviewed".to_owned()),
    };
    let index = raw
        .parse::<u16>()
        .map_err(|_error| "stage message index is not numeric".to_owned())?;
    let message_kind = match kind {
        MissionStageKind::LockedVehicle { .. }
        | MissionStageKind::LockedCostume { .. } => {
            if index > 19 {
                return Err(
                    "locked-stage message index is out of range".to_owned()
                );
            }
            MissionStageMessageKind::Locked
        },
        MissionStageKind::Standard { .. } => {
            if index > 299 {
                return Err(
                    "stage objective message index is out of range".to_owned()
                );
            }
            MissionStageMessageKind::Objective
        },
    };
    Ok(MissionStageDirective::MessageIndex {
        source_ordinal,
        kind: message_kind,
        index,
        unused_argument,
    })
}

fn compile_vehicle(
    source_ordinal: usize,
    arguments: &[String],
) -> Result<MissionStageVehicleReference, String> {
    let (vehicle_id, locator_id, behaviour, con_file, driver_id) =
        match arguments {
            [vehicle, locator, behaviour, con] => {
                (vehicle, locator, behaviour, con, None)
            },
            [vehicle, locator, behaviour, con, driver] => {
                (vehicle, locator, behaviour, con, Some(driver.clone()))
            },
            _ => {
                return Err(
                    "stage vehicle parameter shape is not reviewed".to_owned()
                );
            },
        };
    validate_identity(vehicle_id, "stage vehicle")?;
    validate_identity(locator_id, "stage vehicle locator")?;
    validate_token(behaviour, "stage vehicle behaviour")?;
    validate_con_path(con_file)?;
    if let Some(driver) = &driver_id {
        validate_identity(driver, "stage vehicle driver")?;
    }
    Ok(MissionStageVehicleReference {
        source_ordinal,
        vehicle_id: vehicle_id.clone(),
        locator_id: locator_id.clone(),
        behaviour: behaviour.clone(),
        con_file: con_file.clone(),
        driver_id,
    })
}

fn parse_max_traffic(arguments: &[String]) -> Result<u8, String> {
    let [raw] = arguments else {
        return Err("stage max-traffic shape is not reviewed".to_owned());
    };
    if raw.is_empty() || !raw.chars().all(|ch| ch.is_ascii_digit()) {
        return Err("stage max-traffic value is not numeric".to_owned());
    }
    let cars = raw.parse::<u8>().map_err(|_error| {
        "stage max-traffic value is not numeric".to_owned()
    })?;
    if matches!(cars, 1..=5) {
        Ok(cars)
    } else {
        Err("stage max-traffic value is not reviewed".to_owned())
    }
}

fn required_legacy_argument(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    if value.is_empty() || value.chars().any(char::is_control) {
        Err(format!("{label} is malformed"))
    } else {
        Ok(value.clone())
    }
}

fn parse_nonnegative_seconds(
    arguments: &[String],
    label: &str,
) -> Result<u32, String> {
    let [raw] = arguments else {
        return Err(format!("{label} shape is not reviewed"));
    };
    match raw.parse::<u32>() {
        Ok(seconds) => Ok(seconds),
        Err(_error) => {
            let mut error = label.to_owned();
            error.push_str(" is not numeric");
            Err(error)
        },
    }
}

fn parse_positive_seconds(
    arguments: &[String],
    label: &str,
) -> Result<u32, String> {
    let [raw] = arguments else {
        return Err(format!("{label} shape is not reviewed"));
    };
    let seconds = match raw.parse::<u32>() {
        Ok(seconds) => seconds,
        Err(_error) => {
            let mut error = label.to_owned();
            error.push_str(" is not numeric");
            return Err(error);
        },
    };
    if seconds == 0 {
        Err(format!("{label} must be positive"))
    } else {
        Ok(seconds)
    }
}

fn required_identity(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape is not reviewed"));
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
        Err(format!("{label} identity is malformed"))
    }
}

fn validate_token(value: &str, label: &str) -> Result<(), String> {
    if value.is_empty() || value.chars().any(char::is_control) {
        Err(format!("{label} is malformed"))
    } else {
        Ok(())
    }
}

fn validate_con_path(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value.chars().any(char::is_control)
        || !value.to_ascii_lowercase().ends_with(".con")
        || value.contains("..")
        || value.starts_with('/')
        || value.starts_with(char::from(92))
    {
        Err("stage vehicle CON path is malformed".to_owned())
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_stage/tests.rs"]
mod tests;
