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
//   - Canonical package binding for source `BindReward` P3D references.
// - Must-Not:
//   - Infer unlock, purchase, progression, seller, or reward runtime behavior.
// - Allows:
//   - Validate observed source shape and bind exact authored P3D paths.
// - Split-When:
//   - Reward catalog identity and progression policy gain independent schemas.
// - Merge-When:
//   - Final reward compilation owns this exact package-reference boundary.
// - Summary:
//   - BindReward package-reference preflight.
// - Description:
//   - Preserves observed BindReward source tokens and resolves only its P3D
//     path through the shared phase-three package catalog.
// - Usage:
//   - Runs after lossless mission scope projection and P3D catalog creation.
// - Defaults:
//   - Unsupported arity, malformed scalar tokens, or missing packages fail.
//

//! Canonical package binding for source `BindReward` references.

use super::{MissionP3dReferenceCatalog, MissionScopeReport};

#[cfg(test)]
type MissionRewardReferenceTestEntry<'a> = (
    usize,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    Option<&'a str>,
    Option<&'a str>,
    &'a str,
);

/// One source `BindReward` command bound to one canonical P3D package.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionRewardPackageReference {
    source_ordinal: usize,
    reward_id: String,
    source_reference: String,
    reward_type_token: String,
    source_mode_token: String,
    source_level: String,
    source_cost: Option<String>,
    source_vendor: Option<String>,
    package_id: String,
    package_root: String,
}

impl MissionRewardPackageReference {
    /// Return the source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact source reward identity token.
    #[must_use]
    pub fn reward_id(&self) -> &str {
        &self.reward_id
    }

    /// Return the exact authored P3D path.
    #[must_use]
    pub fn source_reference(&self) -> &str {
        &self.source_reference
    }

    /// Return the exact source reward-type token without runtime meaning.
    #[must_use]
    pub fn reward_type_token(&self) -> &str {
        &self.reward_type_token
    }

    /// Return the exact source mode token without unlock-policy meaning.
    #[must_use]
    pub fn source_mode_token(&self) -> &str {
        &self.source_mode_token
    }

    /// Return the exact source level scalar.
    #[must_use]
    pub fn source_level(&self) -> &str {
        &self.source_level
    }

    /// Return the optional exact source cost scalar.
    #[must_use]
    pub fn source_cost(&self) -> Option<&str> {
        self.source_cost.as_deref()
    }

    /// Return the optional exact source vendor token.
    #[must_use]
    pub fn source_vendor(&self) -> Option<&str> {
        self.source_vendor.as_deref()
    }

    /// Return the canonical phase-three package id.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return the canonical phase-three package root.
    #[must_use]
    pub fn package_root(&self) -> &str {
        &self.package_root
    }
}

/// Canonical `BindReward` package references for one normalized source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionRewardReferenceReport {
    bindings: Vec<MissionRewardPackageReference>,
}

impl MissionRewardReferenceReport {
    /// Return bindings in source order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionRewardPackageReference] {
        &self.bindings
    }

    #[cfg(test)]
    pub(crate) fn from_entries_for_tests(
        entries: &[MissionRewardReferenceTestEntry<'_>],
    ) -> Self {
        Self {
            bindings: entries
                .iter()
                .map(
                    |(
                        source_ordinal,
                        reward_id,
                        reward_type,
                        source_mode,
                        source_level,
                        source_cost,
                        source_vendor,
                        package_id,
                    )| MissionRewardPackageReference {
                        source_ordinal: *source_ordinal,
                        reward_id: (*reward_id).to_owned(),
                        source_reference: format!("{package_id}.p3d"),
                        reward_type_token: (*reward_type).to_owned(),
                        source_mode_token: (*source_mode).to_owned(),
                        source_level: (*source_level).to_owned(),
                        source_cost: source_cost.map(str::to_owned),
                        source_vendor: source_vendor.map(str::to_owned),
                        package_id: (*package_id).to_owned(),
                        package_root: format!("{package_id}-root"),
                    },
                )
                .collect(),
        }
    }
}

/// Bind every unscoped source `BindReward` P3D path to a canonical package.
///
/// The remaining source tokens are preserved exactly and validated only for
/// the scalar shape observed in normalized base-game evidence. This function
/// does not assign unlock, purchase, progression, or seller behavior.
///
/// # Errors
///
/// Returns an error for unsupported arity, malformed source scalars, semantic
/// role drift, unsafe/non-P3D paths, or paths absent from the shared catalog.
pub fn preflight_mission_reward_references(
    catalog: &MissionP3dReferenceCatalog,
    scopes: &MissionScopeReport,
) -> Result<MissionRewardReferenceReport, String> {
    let mut bindings = Vec::new();
    for command in scopes
        .unscoped_commands()
        .iter()
        .filter(|command| command.name() == "bindreward")
    {
        if command.semantic_role() != "mission-reward" {
            return Err("BindReward semantic role changed".to_owned());
        }
        push_binding(
            &mut bindings,
            catalog,
            command.source_ordinal(),
            command.arguments(),
        )?;
    }
    Ok(MissionRewardReferenceReport { bindings })
}

fn push_binding(
    bindings: &mut Vec<MissionRewardPackageReference>,
    catalog: &MissionP3dReferenceCatalog,
    source_ordinal: usize,
    arguments: &[String],
) -> Result<(), String> {
    if !matches!(arguments.len(), 5 | 7) {
        return Err("BindReward must have five or seven arguments".to_owned());
    }
    let reward_id = required_token(arguments, 0, "reward identity")?;
    let source_reference = arguments
        .get(1)
        .ok_or_else(|| "BindReward P3D path is missing".to_owned())?;
    let reward_type_token = required_token(arguments, 2, "reward type")?;
    let source_mode_token = required_token(arguments, 3, "source mode")?;
    let source_level = required_level(arguments, 4)?;
    let (source_cost, source_vendor) = if arguments.len() == 7 {
        (
            Some(required_unsigned(arguments, 5, "source cost")?),
            Some(required_token(arguments, 6, "source vendor")?),
        )
    } else {
        (None, None)
    };
    let reference = catalog.resolve(source_reference)?;
    bindings.push(MissionRewardPackageReference {
        source_ordinal,
        reward_id,
        source_reference: reference.source_reference().to_owned(),
        reward_type_token,
        source_mode_token,
        source_level,
        source_cost,
        source_vendor,
        package_id: reference.package_id().to_owned(),
        package_root: reference.package_root().to_owned(),
    });
    Ok(())
}

fn required_token(
    arguments: &[String],
    index: usize,
    role: &str,
) -> Result<String, String> {
    let value = arguments
        .get(index)
        .ok_or_else(|| format!("BindReward {role} is missing"))?;
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
    {
        return Err(format!("BindReward {role} is malformed"));
    }
    Ok(value.clone())
}

fn required_level(arguments: &[String], index: usize) -> Result<String, String> {
    let value = required_unsigned(arguments, index, "source level")?;
    let level = value
        .parse::<u32>()
        .map_err(|_error| "BindReward source level is not an unsigned integer".to_owned())?;
    if !(1..=7).contains(&level) {
        return Err("BindReward source level is outside observed base levels".to_owned());
    }
    Ok(value)
}

fn required_unsigned(
    arguments: &[String],
    index: usize,
    role: &str,
) -> Result<String, String> {
    let value = required_token(arguments, index, role)?;
    if value.parse::<u32>().is_err() {
        return Err(format!("BindReward {role} is not an unsigned integer"));
    }
    Ok(value)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_reward_reference/tests.rs"]
mod tests;
