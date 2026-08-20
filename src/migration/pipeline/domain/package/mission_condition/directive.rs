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
//   - Typed values carried by reviewed condition-scoped source commands.
// - Must-Not:
//   - Invent units or runtime meaning beyond the command's reviewed role.
// - Allows:
//   - Preserve source values and references in deterministic typed evidence.
// - Split-When:
//   - One condition directive gains an independent schema lifecycle.
// - Merge-When:
//   - Mission definition compilation owns this exact semantic boundary.
// - Summary:
//   - Mission condition directive compiler.
// - Description:
//   - Types every command in the closed condition-scope registry.
// - Usage:
//   - Runs after mission scope projection and condition preflight.
// - Defaults:
//   - Malformed references and values fail closed.
//

//! Typed compilation of reviewed condition-scoped commands.

use super::super::{
    MissionConditionScope, MissionScopeCondition, MissionScopeReport,
};
use super::MissionConditionParameters;

const LEGACY_HIT_AND_RUN_NO_OP: &str = "legacy-set-hit-n-run-dummy-command-v1";

#[cfg(test)]
type MissionConditionBindingTestParts<'a> = (
    usize,
    usize,
    Option<usize>,
    usize,
    &'a str,
    MissionConditionScope,
    &'static str,
);

#[cfg(test)]
type MissionConditionTestEntry = (
    usize,
    usize,
    Option<usize>,
    usize,
    String,
    MissionConditionScope,
    &'static str,
);

#[cfg(test)]
type MissionConditionParameterizedTestEntry = (
    usize,
    usize,
    Option<usize>,
    usize,
    String,
    MissionConditionScope,
    &'static str,
    MissionConditionParameters,
);

/// One typed command owned by a reviewed mission condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MissionConditionDirective {
    /// Preserve the finite nonnegative source minimum-health value.
    MinimumHealth {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact numeric source lexeme; no unit is assigned here.
        source_value: String,
    },
    /// Select the exact mission vehicle observed by the condition.
    TargetVehicle {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact vehicle identity, including the special `current` token.
        vehicle_id: String,
    },
    /// Preserve the exact boss target label used by the condition.
    TargetBoss {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact nonempty source label.
        source_label: String,
    },
    /// Preserve the two nonnegative source follow-distance values.
    FollowDistances {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact first source value.
        minimum: u32,
        /// Exact second source value.
        maximum: u32,
    },
    /// Preserve one positive condition time value without assigning units.
    TimeValue {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact positive source integer.
        source_value: u32,
    },
    /// Preserve one positive source position index.
    PositionIndex {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Exact positive source index.
        source_index: u32,
    },
    /// Preserve an observed release-game dummy command explicitly as a no-op.
    LegacyHitAndRunNoOp {
        /// Source statement ordinal.
        source_ordinal: usize,
        /// Versioned evidence identity for the reviewed no-op command.
        code: &'static str,
    },
}

/// Typed condition directives for one projected condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionConditionSemanticBinding {
    owner_stage_source_ordinal: usize,
    owner_stage_sequence_ordinal: usize,
    owner_objective_source_ordinal: Option<usize>,
    source_ordinal: usize,
    source_alias: String,
    scope: MissionConditionScope,
    schema_id: &'static str,
    parameters: MissionConditionParameters,
    directives: Vec<MissionConditionDirective>,
}

impl MissionConditionSemanticBinding {
    #[cfg(test)]
    pub(crate) fn from_parts_for_tests(
        parts: MissionConditionBindingTestParts<'_>,
        directives: Vec<MissionConditionDirective>,
    ) -> Self {
        let (
            owner_stage_source_ordinal,
            owner_stage_sequence_ordinal,
            owner_objective_source_ordinal,
            source_ordinal,
            source_alias,
            scope,
            schema_id,
        ) = parts;
        Self {
            owner_stage_source_ordinal,
            owner_stage_sequence_ordinal,
            owner_objective_source_ordinal,
            source_ordinal,
            source_alias: source_alias.to_owned(),
            scope,
            schema_id,
            parameters: MissionConditionParameters::None,
            directives,
        }
    }

    /// Return the source `AddStage` ordinal owning this condition.
    #[must_use]
    pub const fn owner_stage_source_ordinal(&self) -> usize {
        self.owner_stage_source_ordinal
    }

    /// Return the dense authored stage ordinal owning this condition.
    #[must_use]
    pub const fn owner_stage_sequence_ordinal(&self) -> usize {
        self.owner_stage_sequence_ordinal
    }

    /// Return the root `AddObjective` ordinal for objective-scoped conditions.
    #[must_use]
    pub const fn owner_objective_source_ordinal(&self) -> Option<usize> {
        self.owner_objective_source_ordinal
    }

    /// Return the source `AddCondition` ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact source condition alias.
    #[must_use]
    pub fn source_alias(&self) -> &str {
        &self.source_alias
    }

    /// Return whether the condition was authored on the stage or objective.
    #[must_use]
    pub const fn scope(&self) -> MissionConditionScope {
        self.scope
    }

    /// Return the versioned legacy condition schema identity.
    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    /// Return typed parameters carried directly by `AddCondition`.
    #[must_use]
    pub const fn parameters(&self) -> &MissionConditionParameters {
        &self.parameters
    }

    /// Return typed condition directives in source order.
    #[must_use]
    pub fn directives(&self) -> &[MissionConditionDirective] {
        &self.directives
    }
}

/// Typed command semantics for all projected mission conditions.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionConditionSemanticReport {
    conditions: Vec<MissionConditionSemanticBinding>,
}

impl MissionConditionSemanticReport {
    /// Return typed condition bindings in mission/source order.
    #[must_use]
    pub fn conditions(&self) -> &[MissionConditionSemanticBinding] {
        &self.conditions
    }

    #[cfg(test)]
    pub(crate) fn from_owned_entries_for_tests(
        entries: Vec<MissionConditionTestEntry>,
    ) -> Self {
        Self::from_owned_entries_with_parameters_for_tests(
            entries
                .into_iter()
                .map(|(
                    stage_source,
                    stage_sequence,
                    objective_source,
                    source_ordinal,
                    source_alias,
                    scope,
                    schema_id,
                )| {
                    (
                        stage_source,
                        stage_sequence,
                        objective_source,
                        source_ordinal,
                        source_alias,
                        scope,
                        schema_id,
                        MissionConditionParameters::None,
                    )
                })
                .collect(),
        )
    }

    #[cfg(test)]
    pub(crate) fn from_owned_entries_with_parameters_for_tests(
        entries: Vec<MissionConditionParameterizedTestEntry>,
    ) -> Self {
        Self {
            conditions: entries
                .into_iter()
                .map(|(
                    owner_stage_source_ordinal,
                    owner_stage_sequence_ordinal,
                    owner_objective_source_ordinal,
                    source_ordinal,
                    source_alias,
                    scope,
                    schema_id,
                    parameters,
                )| MissionConditionSemanticBinding {
                    owner_stage_source_ordinal,
                    owner_stage_sequence_ordinal,
                    owner_objective_source_ordinal,
                    source_ordinal,
                    source_alias,
                    scope,
                    schema_id,
                    parameters,
                    directives: Vec::new(),
                })
                .collect(),
        }
    }
}

/// Compile every command in the closed condition-scope registry.
///
/// # Errors
/// Returns an error when a reviewed source value or reference is malformed.
pub fn preflight_mission_condition_semantics(
    scopes: &MissionScopeReport,
) -> Result<MissionConditionSemanticReport, String> {
    let mut conditions = Vec::new();
    for mission in scopes.missions() {
        for stage in mission.stages() {
            for condition in stage.conditions() {
                conditions.push(compile_condition(
                    stage.source_ordinal(),
                    stage.sequence_ordinal(),
                    condition,
                )?);
            }
        }
    }
    Ok(MissionConditionSemanticReport { conditions })
}

fn compile_condition(
    owner_stage_source_ordinal: usize,
    owner_stage_sequence_ordinal: usize,
    condition: &MissionScopeCondition,
) -> Result<MissionConditionSemanticBinding, String> {
    let source_alias = condition.binding().source_alias();
    let mut directives = Vec::with_capacity(condition.commands().len());
    for command in condition.commands() {
        directives.push(compile_directive(
            source_alias,
            command.ordinal(),
            command.command(),
            command.arguments(),
        )?);
    }
    Ok(MissionConditionSemanticBinding {
        owner_stage_source_ordinal,
        owner_stage_sequence_ordinal,
        owner_objective_source_ordinal:
            condition.owner_objective_source_ordinal(),
        source_ordinal: condition.binding().ordinal(),
        source_alias: source_alias.to_owned(),
        scope: condition.scope(),
        schema_id: condition.binding().schema_id(),
        parameters: condition.parameters().parameters().clone(),
        directives,
    })
}

fn compile_directive(
    source_alias: &str,
    source_ordinal: usize,
    command: &str,
    arguments: &[String],
) -> Result<MissionConditionDirective, String> {
    if super::modifier::condition_modifier_schema(source_alias, command)
        .is_none()
    {
        return Err(
            "mission typed condition directive crossed alias scope".to_owned()
        );
    }
    match command {
        "setcondminhealth" => Ok(MissionConditionDirective::MinimumHealth {
            source_ordinal,
            source_value: required_nonnegative_decimal(
                arguments,
                "mission condition minimum health",
            )?,
        }),
        "setcondtargetvehicle" => {
            Ok(MissionConditionDirective::TargetVehicle {
                source_ordinal,
                vehicle_id: required_identity(
                    arguments,
                    "mission condition target vehicle",
                )?,
            })
        },
        "setobjtargetboss" => Ok(MissionConditionDirective::TargetBoss {
            source_ordinal,
            source_label: required_legacy_text(
                arguments,
                "mission condition boss target",
            )?,
        }),
        "setfollowdistances" => {
            let [minimum, maximum] = arguments else {
                return Err("mission follow-distance shape drifted".to_owned());
            };
            let minimum = parse_u32(minimum, "mission follow minimum")?;
            let maximum = parse_u32(maximum, "mission follow maximum")?;
            if maximum < minimum {
                return Err(
                    "mission follow-distance range is inverted".to_owned()
                );
            }
            Ok(MissionConditionDirective::FollowDistances {
                source_ordinal,
                minimum,
                maximum,
            })
        },
        "setcondtime" => Ok(MissionConditionDirective::TimeValue {
            source_ordinal,
            source_value: required_positive_u32(
                arguments,
                "mission condition time value",
            )?,
        }),
        "setconditionposition" => {
            Ok(MissionConditionDirective::PositionIndex {
                source_ordinal,
                source_index: required_positive_u32(
                    arguments,
                    "mission condition position index",
                )?,
            })
        },
        "sethitnrun" => {
            if !arguments.is_empty() {
                return Err(
                    "mission SetHitNRun dummy acquired arguments".to_owned()
                );
            }
            Ok(MissionConditionDirective::LegacyHitAndRunNoOp {
                source_ordinal,
                code: LEGACY_HIT_AND_RUN_NO_OP,
            })
        },
        _ => Err("mission condition directive is not typed".to_owned()),
    }
}

fn required_identity(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    let valid = !value.is_empty()
        && value.bytes().all(|byte| {
            byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-')
        });
    if valid {
        Ok(value.clone())
    } else {
        Err(format!("{label} identity is malformed"))
    }
}

fn required_legacy_text(
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

fn is_plain_decimal(value: &str, allow_negative: bool) -> bool {
    let body = match value.strip_prefix('-') {
        Some(rest) if allow_negative => rest,
        Some(_rest) => return false,
        None => value,
    };
    let mut parts = body.split('.');
    let Some(integer) = parts.next() else {
        return false;
    };
    if integer.is_empty() || !integer.chars().all(|ch| ch.is_ascii_digit()) {
        return false;
    }
    parts.next().is_none_or(|fraction| {
        !fraction.is_empty()
            && fraction.chars().all(|ch| ch.is_ascii_digit())
            && parts.next().is_none()
    })
}

fn required_nonnegative_decimal(
    arguments: &[String],
    label: &str,
) -> Result<String, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    if !is_plain_decimal(value, false) {
        return Err(format!("{label} source decimal is malformed"));
    }
    Ok(value.clone())
}

fn required_positive_u32(
    arguments: &[String],
    label: &str,
) -> Result<u32, String> {
    let [value] = arguments else {
        return Err(format!("{label} shape drifted"));
    };
    let parsed = parse_u32(value, label)?;
    if parsed == 0 {
        Err(format!("{label} must be positive"))
    } else {
        Ok(parsed)
    }
}

fn parse_u32(value: &str, label: &str) -> Result<u32, String> {
    if value.is_empty() || !value.chars().all(|ch| ch.is_ascii_digit()) {
        return Err(format!("{label} is malformed"));
    }
    value
        .parse::<u32>()
        .map_err(|_error| format!("{label} is malformed"))
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/domain/package/mission_condition/directive_tests.rs"]
mod tests;
