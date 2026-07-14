// File:
//   - selector.rs
// Path:
//   - src/pipeline/src/domain/package/selector.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - The selector contract for pipeline phase three package.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute selector.
// - Split-When:
//   - Split when selector contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Typed package selectors for phase-three intake.
// - Description:
//   - Defines selector data and behavior for pipeline phase three package.
// - Usage:
//   - Used by pipeline phase three package code that needs selector.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Typed package selectors for phase-three intake keeps tightly
//   - coupled validation, ordering, and deterministic transformation
//   - invariants together; split when a stable independently testable sub-
//   - boundary is identified.
//

//! Typed package selectors for phase-three intake.
//! Typed package selectors for phase-three intake.

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
    if value
        .trim()
        .is_empty()
        || value != value.trim()
        || value.contains(':')
        || value
            .chars()
            .any(char::is_control)
    {
        return Err(
            PackageIntakeError::new(
                "selector value must be one non-empty token",
            ),
        );
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
            return Err(
                PackageIntakeError::new("selector must use kind:value syntax"),
            );
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
                return Err(
                    PackageIntakeError::new(
                        format!("unsupported selector kind: {kind}"),
                    ),
                );
            }
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
            Self::Subcategory(subcategory) => require_exact_subcategory(
                index,
                subcategory,
            ),
            Self::SubcategoryPrefix(prefix) => require_unique_prefix(
                index, prefix,
            ),
            Self::Prop(prop_token) => require_exact_subcategory(
                index,
                &format!(
                    "props/{}",
                    normalize_selector_token(prop_token)
                ),
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
        .filter(
            |package| {
                package
                    .subcategory
                    .ends_with(&needle)
            },
        )
        .collect();
    require_one(
        &matches,
        &format!("{category}:{token}"),
    )
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
    require_one(
        &matches,
        subcategory,
    )
}

/// Resolves one prefix only when it identifies a single package.
fn require_unique_prefix<'a>(
    index: &'a PhaseThreePackageIndex,
    prefix: &str,
) -> Result<&'a PhaseThreePackageRow, PackageIntakeError> {
    let matches = index.packages_by_subcategory_prefix(prefix);
    require_one(
        &matches, prefix,
    )
}

/// Converts a candidate slice into one fail-closed selector result.
fn require_one<'a>(
    matches: &[&'a PhaseThreePackageRow],
    label: &str,
) -> Result<&'a PhaseThreePackageRow, PackageIntakeError> {
    match matches {
        [package] => Ok(*package),
        [] => Err(
            PackageIntakeError::new(
                format!("selector did not match any package: {label}"),
            ),
        ),
        _ => Err(
            PackageIntakeError::new(
                format!("selector matched more than one package: {label}"),
            ),
        ),
    }
}

/// Normalizes operator tokens to the generated package-id convention.
fn normalize_selector_token(token: &str) -> String {
    token
        .trim()
        .chars()
        .map(
            |character| {
                if character.is_ascii_alphanumeric() {
                    character.to_ascii_lowercase()
                } else {
                    '-'
                }
            },
        )
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{
        PhaseThreePackageIndex, PhaseThreePackageSelector,
        normalize_selector_token,
    };

    #[test]
    fn rejects_whitespace_padded_compact_selectors() -> Result<(), String> {
        for raw in [
            "package: ",
            "subcategory:\t",
            "prefix:  ",
            "prop:\r",
            "vehicle:\n",
            "character:\t ",
            "package: pkg-car",
            "subcategory:cars/example ",
            "prefix:\tcars/",
            "prop:wrench ",
            "vehicle: homer-v",
            "character:homer\t",
        ] {
            if PhaseThreePackageSelector::parse(raw).is_ok() {
                return Err(
                    format!(
                        "whitespace-padded selector must be rejected: {raw:?}",
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_empty_or_multi_colon_selectors() -> Result<(), String> {
        for raw in [
            "package:",
            "subcategory:",
            "prefix:",
            "prop:",
            "vehicle:",
            "character:",
            "prop:wrench:extra",
        ] {
            if PhaseThreePackageSelector::parse(raw).is_ok() {
                return Err(
                    format!("malformed selector must be rejected: {raw}"),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn normalizes_tokens_like_generated_package_ids() -> Result<(), String> {
        for (input, expected) in [
            (
                " Homer V.2 ",
                "homer-v-2",
            ),
            (
                "SNAKE_CASE",
                "snake-case",
            ),
            (
                // cspell:disable-next-line -- caf
                "café", "caf-",
            ),
        ] {
            let actual = normalize_selector_token(input);
            if actual != expected {
                return Err(
                    format!(
                        "selector token {input:?} normalized to {actual:?}"
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_empty_programmatic_prefixes() -> Result<(), String> {
        let row = concat!(
            "{\"package_id\":\"pkg-car\",",
            "\"package_root\":\"pkg-car\",",
            "\"package_category\":\"cars\",",
            "\"package_subcategory\":",
            "\"cars/character-rigs/homer-v\",",
            "\"unit_count\":1,\"text_key_count\":0,",
            "\"unit_ids\":[\"model-a\"],",
            "\"world_ids\":[],\"texture_ids\":[],",
            "\"material_ids\":[],",
            "\"model_ids\":[\"model-a\"],",
            "\"physics_ids\":[],\"animation_ids\":[],",
            "\"scene_ids\":[],\"locator_ids\":[],",
            "\"camera_ids\":[],\"light_ids\":[],",
            "\"particle_ids\":[],\"controller_ids\":[],",
            "\"audio_ids\":[],\"movie_ids\":[],",
            "\"script_ids\":[],\"text_ids\":[],",
            "\"ui_ids\":[],\"metadata_ids\":[],",
            "\"error_ids\":[],\"source_unit_ids\":[],",
            "\"text_key_ids\":[],",
            "\"members\":[{",
            "\"id\":\"model-a\",",
            "\"role\":\"model\",",
            "\"path\":\"extracted/model.p3d\",",
            "\"type\":\"model\",",
            "\"kind\":\"mesh\",",
            "\"source_chunk_kind\":\"mesh\"}],",
            "\"text_keys\":[]}"
        );
        let index = PhaseThreePackageIndex::from_jsonl(row)
            .map_err(|error| error.to_string())?;
        if PhaseThreePackageSelector::subcategory_prefix("")
            .resolve(&index)
            .is_ok()
        {
            return Err(
                "empty programmatic prefix must not select a package"
                    .to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn vehicle_selector_rejects_intermediate_subcategory_segments()
    -> Result<(), String> {
        let row = concat!(
            "{\"package_id\":\"pkg-car\",",
            "\"package_root\":\"pkg-car\",",
            "\"package_category\":\"cars\",",
            "\"package_subcategory\":",
            "\"cars/character-rigs/homer-v\",",
            "\"unit_count\":1,\"text_key_count\":0,",
            "\"unit_ids\":[\"model-a\"],",
            "\"world_ids\":[],\"texture_ids\":[],",
            "\"material_ids\":[],",
            "\"model_ids\":[\"model-a\"],",
            "\"physics_ids\":[],\"animation_ids\":[],",
            "\"scene_ids\":[],\"locator_ids\":[],",
            "\"camera_ids\":[],\"light_ids\":[],",
            "\"particle_ids\":[],\"controller_ids\":[],",
            "\"audio_ids\":[],\"movie_ids\":[],",
            "\"script_ids\":[],\"text_ids\":[],",
            "\"ui_ids\":[],\"metadata_ids\":[],",
            "\"error_ids\":[],\"source_unit_ids\":[],",
            "\"text_key_ids\":[],",
            "\"members\":[{",
            "\"id\":\"model-a\",",
            "\"role\":\"model\",",
            "\"path\":\"extracted/model.p3d\",",
            "\"type\":\"model\",",
            "\"kind\":\"mesh\",",
            "\"source_chunk_kind\":\"mesh\"}],",
            "\"text_keys\":[]}"
        );
        let index = PhaseThreePackageIndex::from_jsonl(row)
            .map_err(|error| error.to_string())?;
        if PhaseThreePackageSelector::vehicle("character-rigs")
            .resolve(&index)
            .is_ok()
        {
            return Err(
                "vehicle selectors must match the terminal model token"
                    .to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_invalid_programmatic_selector_values() -> Result<(), String> {
        let row = concat!(
            "{\"package_id\":\"pkg-car\",",
            "\"package_root\":\"pkg-car\",",
            "\"package_category\":\"cars\",",
            "\"package_subcategory\":",
            "\"cars/character-rigs/homer-v\",",
            "\"unit_count\":1,\"text_key_count\":0,",
            "\"unit_ids\":[\"model-a\"],",
            "\"world_ids\":[],\"texture_ids\":[],",
            "\"material_ids\":[],",
            "\"model_ids\":[\"model-a\"],",
            "\"physics_ids\":[],\"animation_ids\":[],",
            "\"scene_ids\":[],\"locator_ids\":[],",
            "\"camera_ids\":[],\"light_ids\":[],",
            "\"particle_ids\":[],\"controller_ids\":[],",
            "\"audio_ids\":[],\"movie_ids\":[],",
            "\"script_ids\":[],\"text_ids\":[],",
            "\"ui_ids\":[],\"metadata_ids\":[],",
            "\"error_ids\":[],\"source_unit_ids\":[],",
            "\"text_key_ids\":[],",
            "\"members\":[{",
            "\"id\":\"model-a\",",
            "\"role\":\"model\",",
            "\"path\":\"extracted/model.p3d\",",
            "\"type\":\"model\",",
            "\"kind\":\"mesh\",",
            "\"source_chunk_kind\":\"mesh\"}],",
            "\"text_keys\":[]}"
        );
        let index = PhaseThreePackageIndex::from_jsonl(row)
            .map_err(|error| error.to_string())?;
        for invalid in [
            " homer-v ",
            "homer\u{0}v",
            "homer:v",
        ] {
            if PhaseThreePackageSelector::vehicle(invalid)
                .resolve(&index)
                .is_ok()
            {
                return Err(
                    format!(
                        "invalid programmatic selector must fail: {invalid:?}"
                    ),
                );
            }
        }
        Ok(())
    }

    #[test]
    fn parses_compact_selectors() -> Result<(), String> {
        if PhaseThreePackageSelector::parse("prop:wrench")
            .map_err(|error| error.to_string())?
            != PhaseThreePackageSelector::prop("wrench")
        {
            return Err("prop selector should parse".to_owned());
        }
        if PhaseThreePackageSelector::parse("package:pkg")
            .map_err(|error| error.to_string())?
            != PhaseThreePackageSelector::package_id("pkg")
        {
            return Err("package selector should parse".to_owned());
        }
        Ok(())
    }
}
