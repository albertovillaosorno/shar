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
//   - Typed shapes carried directly by reviewed AddCondition calls.
// - Must-Not:
//   - Assign undocumented gameplay meaning to legacy positional values.
// - Allows:
//   - Preserve closed numeric/token forms as versioned typed evidence.
// - Split-When:
//   - A condition parameter receives independently reviewed semantics.
// - Merge-When:
//   - Mission condition compilation owns this identical evidence boundary.
// - Summary:
//   - Mission condition direct-parameter compiler.
// - Description:
//   - Replaces raw condition positional values with closed typed shapes.
// - Usage:
//   - Runs after condition alias preflight succeeds.
// - Defaults:
//   - Unobserved values and shapes fail closed.
//

//! Typed shapes for direct `AddCondition` parameters.

use super::{MissionConditionBinding, preflight_mission_conditions};
use crate::domain::package::MissionScriptEvidence;

const DAMAGE_NEITHER_CODE: &str =
    "legacy-damage-condition-neither-parameter-v1";

/// Typed direct parameters for one reviewed condition invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionConditionParameters {
    /// The condition carries no direct positional parameters.
    None,
    /// `keepbarrel` carries one observed integer whose meaning remains unknown.
    KeepBarrelLegacyValue {
        /// Exact observed integer, closed to the current corpus range 1
        /// through 4.
        value: u8,
    },
    /// `damage` carries one exact observed token with undocumented semantics.
    DamageLegacyToken {
        /// Exact source token retained without reinterpretation.
        source_token: String,
        /// Versioned review identity for this unexplained source form.
        code: &'static str,
    },
}

/// Typed direct-parameter evidence for one condition invocation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionConditionParameterBinding {
    ordinal: usize,
    source_alias: String,
    parameters: MissionConditionParameters,
}

impl MissionConditionParameterBinding {
    /// Return the source statement ordinal of `AddCondition`.
    #[must_use]
    pub const fn ordinal(&self) -> usize {
        self.ordinal
    }

    /// Return the exact reviewed source condition alias.
    #[must_use]
    pub fn source_alias(&self) -> &str {
        &self.source_alias
    }

    /// Return the typed direct parameter shape.
    #[must_use]
    pub const fn parameters(&self) -> &MissionConditionParameters {
        &self.parameters
    }
}

/// Complete typed direct-condition parameter coverage for one source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionConditionParameterReport {
    conditions: Vec<MissionConditionParameterBinding>,
}

impl MissionConditionParameterReport {
    /// Return typed condition parameters in source order.
    #[must_use]
    pub fn conditions(&self) -> &[MissionConditionParameterBinding] {
        &self.conditions
    }
}

/// Compile every reviewed direct `AddCondition` parameter into a closed shape.
///
/// # Errors
///
/// Returns an error for an unobserved value or positional shape.
pub fn preflight_mission_condition_parameters(
    evidence: &MissionScriptEvidence,
) -> Result<MissionConditionParameterReport, String> {
    let aliases = preflight_mission_conditions(evidence)?;
    let mut conditions = Vec::with_capacity(aliases.conditions().len());
    for condition in aliases.conditions() {
        conditions.push(MissionConditionParameterBinding {
            ordinal: condition.ordinal(),
            source_alias: condition.source_alias().to_owned(),
            parameters: compile_condition_parameters(condition)?,
        });
    }
    Ok(MissionConditionParameterReport { conditions })
}

fn compile_condition_parameters(
    condition: &MissionConditionBinding,
) -> Result<MissionConditionParameters, String> {
    let parameters = condition.legacy_parameters();
    match condition.source_alias() {
        "keepbarrel" => {
            let [raw] = parameters else {
                return Err(
                    "keepbarrel condition requires one legacy value".to_owned()
                );
            };
            let value = raw.parse::<u8>().map_err(|_error| {
                "keepbarrel legacy value is not numeric".to_owned()
            })?;
            if !(1..=4).contains(&value) {
                return Err(
                    "keepbarrel legacy value is not reviewed".to_owned()
                );
            }
            Ok(MissionConditionParameters::KeepBarrelLegacyValue { value })
        },
        "damage" => {
            match parameters {
                [] => Ok(MissionConditionParameters::None),
                [token] if token == "neither" => {
                    Ok(MissionConditionParameters::DamageLegacyToken {
                        source_token: token.clone(),
                        code: DAMAGE_NEITHER_CODE,
                    })
                },
                _ => Err("damage condition parameter shape is not reviewed"
                    .to_owned()),
            }
        },
        "followdistance" | "outofvehicle" | "position" | "race" | "timeout" => {
            if parameters.is_empty() {
                Ok(MissionConditionParameters::None)
            } else {
                Err("mission condition unexpectedly carries direct parameters"
                    .to_owned())
            }
        },
        _ => {
            Err("mission condition parameter alias is not reviewed".to_owned())
        },
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/domain/package/mission_condition/parameter_tests.rs"]
mod tests;
