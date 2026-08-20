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
//   - Selector domain module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Selector domain module.
// - Description:
//   - Implements the declared domain module responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Selector domain module.

use super::index::{
    PackageIntakeError, PhaseThreePackageIndex, PhaseThreePackageRow,
};

/// Typed selector for package-index rows.
// The phase-qualified name prevents command and adapter callers from selecting
// against a package-index model owned by a different pipeline phase.
#[expect(
    clippy::module_name_repetitions,
    reason = "Public names preserve distinct phase-three selector boundaries \
              for callers."
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PhaseThreePackageSelector {
    /// Exact generated package id.
    PackageId(String),
    /// Exact generated subcategory.
    Subcategory(String),
    /// Generated subcategory prefix that must resolve to one package.
    SubcategoryPrefix(String),
    /// Prop token such as `wrench`.
    Prop(String),
    /// Vehicle model token such as `homer-v`.
    Vehicle(String),
    /// Character model token such as `homer`.
    Character(String),
}

/// Reject selector values that can alias another canonical identity.
fn validate_selector_value(value: &str) -> Result<(), PackageIntakeError> {
    if value.trim().is_empty()
        || value != value.trim()
        || value.contains(':')
        || value.chars().any(char::is_control)
    {
        return Err(PackageIntakeError::new(
            "selector value must be one non-empty token",
        ));
    }
    Ok(())
}

impl PhaseThreePackageSelector {
    /// Build an exact package-id selector.
    #[must_use]
    pub fn package_id(package_id: impl Into<String>) -> Self {
        Self::PackageId(package_id.into())
    }

    /// Build an exact subcategory selector.
    #[must_use]
    pub fn subcategory(subcategory: impl Into<String>) -> Self {
        Self::Subcategory(subcategory.into())
    }

    /// Build a unique subcategory-prefix selector.
    #[must_use]
    pub fn subcategory_prefix(prefix: impl Into<String>) -> Self {
        Self::SubcategoryPrefix(prefix.into())
    }

    /// Build a prop selector from a stable prop token.
    #[must_use]
    pub fn prop(prop_token: impl Into<String>) -> Self {
        Self::Prop(prop_token.into())
    }

    /// Build a vehicle selector from a stable vehicle model token.
    #[must_use]
    pub fn vehicle(vehicle_token: impl Into<String>) -> Self {
        Self::Vehicle(vehicle_token.into())
    }

    /// Build a character selector from a stable character token.
    #[must_use]
    pub fn character(character_token: impl Into<String>) -> Self {
        Self::Character(character_token.into())
    }

    /// Parse the compact CLI selector syntax.
    ///
    /// # Errors
    ///
    /// Returns an error when the selector does not use a supported prefix.
    pub fn parse(raw: &str) -> Result<Self, PackageIntakeError> {
        let Some((kind, value)) = raw.split_once(':') else {
            return Err(PackageIntakeError::new(
                "selector must use kind:value syntax",
            ));
        };
        validate_selector_value(value)?;
        let selector = match kind {
            "package" => Self::package_id(value),
            "subcategory" => Self::subcategory(value),
            "prefix" => Self::subcategory_prefix(value),
            "prop" => Self::prop(value),
            "vehicle" => Self::vehicle(value),
            "character" => Self::character(value),
            _ => {
                return Err(PackageIntakeError::new(format!(
                    "unsupported selector kind: {kind}"
                )));
            },
        };
        Ok(selector)
    }

    /// Resolve this selector to exactly one package row.
    ///
    /// # Errors
    ///
    /// Returns an error when the selector finds no packages or more than one
    /// package where a unique package is required.
    pub fn resolve<'a>(
        &self,
        index: &'a PhaseThreePackageIndex,
    ) -> Result<&'a PhaseThreePackageRow, PackageIntakeError> {
        let value = match self {
            Self::PackageId(value)
            | Self::Subcategory(value)
            | Self::SubcategoryPrefix(value)
            | Self::Prop(value)
            | Self::Vehicle(value)
            | Self::Character(value) => value,
        };
        validate_selector_value(value)?;
        match self {
            Self::PackageId(package_id) => index.require_package(package_id),
            Self::Subcategory(subcategory) => {
                require_exact_subcategory(index, subcategory)
            },
            Self::SubcategoryPrefix(prefix) => {
                require_unique_prefix(index, prefix)
            },
            Self::Prop(prop_token) => require_exact_subcategory(
                index,
                &format!("props/{}", normalize_selector_token(prop_token)),
            ),
            Self::Vehicle(vehicle_token) => require_category_token(
                index,
                "cars",
                &normalize_selector_token(vehicle_token),
            ),
            Self::Character(character_token) => require_unique_prefix(
                index,
                &format!(
                    "characters/{}/",
                    normalize_selector_token(character_token)
                ),
            ),
        }
    }
}

/// Resolves one category token while rejecting zero or multiple matches.
fn require_category_token<'a>(
    index: &'a PhaseThreePackageIndex,
    category: &str,
    token: &str,
) -> Result<&'a PhaseThreePackageRow, PackageIntakeError> {
    let needle = format!("/{token}");
    let matches: Vec<_> = index
        .packages_by_category(category)
        .into_iter()
        .filter(|package| package.subcategory.ends_with(&needle))
        .collect();
    require_one(&matches, &format!("{category}:{token}"))
}
/// Resolves one exact subcategory without accepting prefix ambiguity.
fn require_exact_subcategory<'a>(
    index: &'a PhaseThreePackageIndex,
    subcategory: &str,
) -> Result<&'a PhaseThreePackageRow, PackageIntakeError> {
    let matches: Vec<_> = index
        .packages()
        .iter()
        .filter(|package| package.subcategory == subcategory)
        .collect();
    require_one(&matches, subcategory)
}

/// Resolves one prefix only when it identifies a single package.
fn require_unique_prefix<'a>(
    index: &'a PhaseThreePackageIndex,
    prefix: &str,
) -> Result<&'a PhaseThreePackageRow, PackageIntakeError> {
    let matches = index.packages_by_subcategory_prefix(prefix);
    require_one(&matches, prefix)
}

/// Converts a candidate slice into one fail-closed selector result.
fn require_one<'a>(
    matches: &[&'a PhaseThreePackageRow],
    label: &str,
) -> Result<&'a PhaseThreePackageRow, PackageIntakeError> {
    match matches {
        [package] => Ok(*package),
        [] => Err(PackageIntakeError::new(format!(
            "selector did not match any package: {label}"
        ))),
        _ => Err(PackageIntakeError::new(format!(
            "selector matched more than one package: {label}"
        ))),
    }
}

/// Normalizes operator tokens to the generated package-id convention.
fn normalize_selector_token(token: &str) -> String {
    token
        .trim()
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect()
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/selector/tests.rs"]
mod tests;
