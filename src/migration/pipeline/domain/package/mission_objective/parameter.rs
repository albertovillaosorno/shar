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
//   - Typed parameters carried directly by reviewed AddObjective calls.
// - Must-Not:
//   - Infer objective-scoped command semantics or repair legacy source tokens.
// - Allows:
//   - Type reviewed route, wager, vehicle, and costume parameters fail closed.
// - Split-When:
//   - One objective parameter family gains an independent runtime lifecycle.
// - Merge-When:
//   - Mission definition compilation owns the identical parameter boundary.
// - Summary:
//   - Mission objective parameter compiler.
// - Description:
//   - Replaces raw AddObjective positional values with reviewed typed evidence.
// - Usage:
//   - Runs after objective alias and invocation preflight succeeds.
// - Defaults:
//   - Unknown route tokens and malformed source references fail closed.
//

//! Typed compilation of parameters carried directly by `AddObjective`.

use super::{MissionObjectiveBinding, preflight_mission_objectives};
use crate::domain::package::MissionScriptEvidence;

const LEGACY_NIETHER_ROUTE_TOKEN: &str = "niether";
const LEGACY_NIETHER_ROUTE_CODE: &str =
    "legacy-road-arrow-token-niether-unrecognized-v1";

/// Effective road-arrow policy recognized by the legacy objective parser.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionRoadArrowMode {
    /// Draw arrows on the nearest road and at intersections.
    Both,
    /// Suppress road arrows.
    Neither,
    /// Draw arrows at intersections.
    Intersection,
    /// Draw arrows on the nearest road.
    NearestRoad,
}

/// Reviewed result of parsing one optional road-arrow source token.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionRoadArrowBinding {
    /// The source token maps to an effective legacy road-arrow mode.
    Effective(MissionRoadArrowMode),
    /// The exact reviewed source token is not recognized by the legacy parser.
    LegacyUnrecognized {
        /// Exact source token retained for provenance.
        source_token: String,
        /// Versioned review identity explaining why the token is retained.
        code: &'static str,
    },
}

impl MissionRoadArrowBinding {
    /// Return the effective road-arrow mode when the source token is
    /// recognized.
    #[must_use]
    pub const fn effective_mode(&self) -> Option<MissionRoadArrowMode> {
        match self {
            Self::Effective(mode) => Some(*mode),
            Self::LegacyUnrecognized { .. } => None,
        }
    }

    /// Return the exact legacy token when it is deliberately retained ignored.
    #[must_use]
    pub fn legacy_unrecognized_token(&self) -> Option<&str> {
        match self {
            Self::Effective(_) => None,
            Self::LegacyUnrecognized { source_token, .. } => Some(source_token),
        }
    }

    /// Return the review identity for an unrecognized legacy token.
    #[must_use]
    pub const fn legacy_unrecognized_code(&self) -> Option<&'static str> {
        match self {
            Self::Effective(_) => None,
            Self::LegacyUnrecognized { code, .. } => Some(*code),
        }
    }
}

/// Typed parameters carried directly by one reviewed objective invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionObjectiveParameters {
    /// The objective carries no direct positional parameters.
    None,
    /// The only direct parameter is a road-arrow token.
    RoadArrows(MissionRoadArrowBinding),
    /// A buy-car objective names the required vehicle.
    BuyVehicle {
        /// Exact reviewed source vehicle identity.
        vehicle_id: String,
    },
    /// A buy-skin objective names the required costume.
    BuyCostume {
        /// Exact reviewed source costume identity.
        costume_id: String,
    },
    /// A get-in objective names the exact required vehicle.
    EnterVehicle {
        /// Exact reviewed source vehicle identity.
        vehicle_id: String,
    },
    /// A race objective may be a wager race and may set road arrows.
    Race {
        /// Whether the exact `gamble` marker was present.
        gamble: bool,
        /// Optional reviewed road-arrow token.
        road_arrows: Option<MissionRoadArrowBinding>,
    },
}

/// Typed direct-parameter evidence for one objective invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveParameterBinding {
    ordinal: usize,
    source_alias: String,
    parameters: MissionObjectiveParameters,
}

impl MissionObjectiveParameterBinding {
    /// Return the source statement ordinal of `AddObjective`.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// Return the exact reviewed source objective alias.
    #[must_use]
    pub fn source_alias(&self) -> &str {
        &self.source_alias
    }

    /// Return the typed direct parameters.
    #[must_use]
    pub const fn parameters(&self) -> &MissionObjectiveParameters {
        &self.parameters
    }
}

/// Complete typed direct-parameter coverage for one mission source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionObjectiveParameterReport {
    objectives: Vec<MissionObjectiveParameterBinding>,
}

impl MissionObjectiveParameterReport {
    /// Return typed objective parameters in source order.
    #[must_use]
    pub fn objectives(&self) -> &[MissionObjectiveParameterBinding] {
        &self.objectives
    }
}

/// Compile every reviewed direct `AddObjective` parameter into typed evidence.
///
/// # Errors
///
/// Returns an error for an unreviewed shape, unknown route token, or malformed
/// required source identity.
pub fn preflight_mission_objective_parameters(
    evidence: &MissionScriptEvidence,
) -> Result<MissionObjectiveParameterReport, String> {
    let aliases = preflight_mission_objectives(evidence)?;
    let mut objectives = Vec::with_capacity(aliases.objectives().len());
    for objective in aliases.objectives() {
        objectives.push(MissionObjectiveParameterBinding {
            ordinal: objective.ordinal(),
            source_alias: objective.source_alias().to_owned(),
            parameters: compile_objective_parameters(objective)?,
        });
    }
    Ok(MissionObjectiveParameterReport { objectives })
}

fn compile_objective_parameters(
    objective: &MissionObjectiveBinding,
) -> Result<MissionObjectiveParameters, String> {
    let alias = objective.source_alias();
    let parameters = objective.legacy_parameters();
    match alias {
        "buycar" => Ok(MissionObjectiveParameters::BuyVehicle {
            vehicle_id: required_vehicle(parameters)?,
        }),
        "buyskin" => Ok(MissionObjectiveParameters::BuyCostume {
            costume_id: required_costume(parameters)?,
        }),
        "getin" => compile_get_in(parameters),
        "race" => compile_race(parameters),
        "delivery" | "destroy" | "dump" | "follow" | "goto" | "interior"
        | "losetail" | "talkto" => compile_optional_road_arrows(parameters),
        "coins" | "destroyboss" | "dialogue" | "dummy" | "fmv"
        | "gooutside" | "pickupitem" | "timer" => {
            require_no_parameters(parameters)?;
            Ok(MissionObjectiveParameters::None)
        },
        _ => {
            Err("mission objective parameter alias is not reviewed".to_owned())
        },
    }
}

fn compile_optional_road_arrows(
    parameters: &[String],
) -> Result<MissionObjectiveParameters, String> {
    match parameters {
        [] => Ok(MissionObjectiveParameters::None),
        [token] => Ok(MissionObjectiveParameters::RoadArrows(
            parse_road_arrow_token(token)?,
        )),
        _ => Err("mission objective route parameter shape is not reviewed"
            .to_owned()),
    }
}

fn compile_get_in(
    parameters: &[String],
) -> Result<MissionObjectiveParameters, String> {
    match parameters {
        [] => Ok(MissionObjectiveParameters::None),
        [token] if effective_road_arrow_mode(token).is_some() => {
            Ok(MissionObjectiveParameters::RoadArrows(
                parse_road_arrow_token(token)?,
            ))
        },
        [vehicle_id] if vehicle_id.ends_with("_v") => {
            validate_source_identity(vehicle_id, "mission get-in vehicle")?;
            Ok(MissionObjectiveParameters::EnterVehicle {
                vehicle_id: vehicle_id.clone(),
            })
        },
        [_] => Err(
            "mission get-in parameter is not a reviewed route or vehicle"
                .to_owned(),
        ),
        _ => Err("mission get-in parameter shape is not reviewed".to_owned()),
    }
}

fn compile_race(
    parameters: &[String],
) -> Result<MissionObjectiveParameters, String> {
    match parameters {
        [] => Ok(MissionObjectiveParameters::Race {
            gamble: false,
            road_arrows: None,
        }),
        [marker] if marker == "gamble" => {
            Ok(MissionObjectiveParameters::Race {
                gamble: true,
                road_arrows: None,
            })
        },
        [route] => Ok(MissionObjectiveParameters::Race {
            gamble: false,
            road_arrows: Some(parse_effective_road_arrow_token(route)?),
        }),
        [marker, route] if marker == "gamble" => {
            Ok(MissionObjectiveParameters::Race {
                gamble: true,
                road_arrows: Some(parse_effective_road_arrow_token(route)?),
            })
        },
        _ => Err("mission race parameter shape is not reviewed".to_owned()),
    }
}

fn required_vehicle(parameters: &[String]) -> Result<String, String> {
    let [vehicle_id] = parameters else {
        return Err("mission buy-car objective requires one vehicle".to_owned());
    };
    validate_source_identity(vehicle_id, "mission buy-car vehicle")?;
    if !vehicle_id.ends_with("_v") {
        return Err(
            "mission buy-car vehicle identity is not reviewed".to_owned()
        );
    }
    Ok(vehicle_id.clone())
}

fn required_costume(parameters: &[String]) -> Result<String, String> {
    let [costume_id] = parameters else {
        return Err(
            "mission buy-skin objective requires one costume".to_owned()
        );
    };
    validate_source_identity(costume_id, "mission buy-skin costume")?;
    Ok(costume_id.clone())
}

fn require_no_parameters(parameters: &[String]) -> Result<(), String> {
    if parameters.is_empty() {
        Ok(())
    } else {
        Err("mission objective unexpectedly carries direct parameters"
            .to_owned())
    }
}

fn parse_road_arrow_token(
    token: &str,
) -> Result<MissionRoadArrowBinding, String> {
    if let Some(mode) = effective_road_arrow_mode(token) {
        return Ok(MissionRoadArrowBinding::Effective(mode));
    }
    if token == LEGACY_NIETHER_ROUTE_TOKEN {
        return Ok(MissionRoadArrowBinding::LegacyUnrecognized {
            source_token: token.to_owned(),
            code: LEGACY_NIETHER_ROUTE_CODE,
        });
    }
    Err("mission road-arrow token is not reviewed".to_owned())
}

fn parse_effective_road_arrow_token(
    token: &str,
) -> Result<MissionRoadArrowBinding, String> {
    effective_road_arrow_mode(token)
        .map(MissionRoadArrowBinding::Effective)
        .ok_or_else(|| {
            "mission race road-arrow token is not reviewed".to_owned()
        })
}

fn effective_road_arrow_mode(token: &str) -> Option<MissionRoadArrowMode> {
    match token {
        "both" => Some(MissionRoadArrowMode::Both),
        "neither" => Some(MissionRoadArrowMode::Neither),
        "intersection" => Some(MissionRoadArrowMode::Intersection),
        "nearest road" => Some(MissionRoadArrowMode::NearestRoad),
        _ => None,
    }
}

fn validate_source_identity(value: &str, label: &str) -> Result<(), String> {
    let valid = !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || byte == b'_' || byte == b'-'
        });
    if valid {
        Ok(())
    } else {
        Err(format!("{label} identity is malformed"))
    }
}

#[cfg(test)]
#[path = "../../../../../../tests/migration/pipeline/unit/domain/package/mission_objective/parameter_tests.rs"]
mod tests;
