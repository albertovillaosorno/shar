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
//   - Ordered projection of explicitly observed DynamicZone traversal events.
// - Must-Not:
//   - Infer trigger entry, traversal order, retrigger policy, or locator lookup
//     precedence from geometry or package names.
// - Allows:
//   - Apply exact caller-supplied DynamicZone transitions to active packages.
// - Split-When:
//   - Trigger observation transport or geometry acquires independent behavior.
// - Merge-When:
//   - Runtime mission state directly owns the same ordered traversal evidence.
// - Summary:
//   - Pure DynamicZone traversal-history package model.
// - Description:
//   - Preserves exact zone identity and caller-supplied traversal order while
//     delegating each Dyna package effect to the conservative transition model.
// - Usage:
//   - Future runtime evidence can refine package residency after mission start.
// - Defaults:
//   - An empty history leaves the explicit initial package set unchanged.
//

//! Pure package-residency projection for observed `DynamicZone` history.

use super::dyna_load_package::validate_active_package_roots;
use super::DynaLoadPackageTransition;

/// One exact `DynamicZone` traversal supplied by an external runtime observer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynamicZoneTraversalStep {
    locator_name: String,
    source_package_root: String,
    transition: DynaLoadPackageTransition,
}

impl DynamicZoneTraversalStep {
    /// Build one identified traversal step from a validated package transition.
    ///
    /// # Errors
    ///
    /// Returns an error when the locator identity or source package root is
    /// malformed.
    pub fn new(
        locator_name: String,
        source_package_root: String,
        transition: DynaLoadPackageTransition,
    ) -> Result<Self, String> {
        validate_identity(&locator_name)?;
        validate_source_package_root(&source_package_root)?;
        Ok(Self {
            locator_name,
            source_package_root,
            transition,
        })
    }

    /// Return the exact decoded `DynamicZone` locator name.
    #[must_use]
    pub fn locator_name(&self) -> &str {
        &self.locator_name
    }

    /// Return the exact package root that owns the decoded locator.
    #[must_use]
    pub fn source_package_root(&self) -> &str {
        &self.source_package_root
    }

    /// Return the typed Dyna package transition executed by this zone.
    #[must_use]
    pub const fn transition(&self) -> &DynaLoadPackageTransition {
        &self.transition
    }
}

/// Ordered `DynamicZone` traversals observed for one runtime path.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct DynamicZoneTraversalHistory {
    steps: Vec<DynamicZoneTraversalStep>,
}

impl DynamicZoneTraversalHistory {
    /// Preserve one caller-supplied traversal sequence exactly.
    #[must_use]
    pub const fn new(steps: Vec<DynamicZoneTraversalStep>) -> Self {
        Self { steps }
    }

    /// Return traversals in caller-supplied runtime order.
    #[must_use]
    pub fn steps(&self) -> &[DynamicZoneTraversalStep] {
        &self.steps
    }

    /// Apply the observed traversal sequence to one explicit initial package
    /// set.
    ///
    /// # Errors
    ///
    /// Returns an error for malformed initial package roots or when one
    /// observed zone contains load/unload effects whose runtime order remains
    /// unresolved.
    pub fn apply_package_roots(
        &self,
        initial_package_roots: &[String],
    ) -> Result<Vec<String>, String> {
        validate_active_package_roots(initial_package_roots)?;
        let mut active = initial_package_roots.to_vec();
        for step in &self.steps {
            active = step
                .transition
                .apply_order_independent_package_roots(&active)
                .map_err(|error| {
                    format!(
                        concat!(
                            "DynamicZone `{}` from `{}` cannot refine package ",
                            "history: {error}"
                        ),
                        step.locator_name,
                        step.source_package_root,
                        error = error
                    )
                })?;
        }
        Ok(active)
    }
}

fn validate_identity(value: &str) -> Result<(), String> {
    if value.is_empty()
        || value != value.trim()
        || value.chars().any(char::is_control)
    {
        return Err(
            "DynamicZone traversal locator identity is malformed".to_owned()
        );
    }
    Ok(())
}

fn validate_source_package_root(root: &str) -> Result<(), String> {
    if root.is_empty()
        || root != root.trim()
        || root.starts_with('/')
        || root.ends_with('/')
        || root.contains(char::from(92))
        || root.contains(':')
        || root.chars().any(char::is_control)
        || root
            .split('/')
            .any(|segment| {
                segment.is_empty() || segment == "." || segment == ".."
            })
        || !root.to_ascii_lowercase().starts_with("extracted/")
    {
        return Err(
            "DynamicZone traversal source package root is unsafe".to_owned()
        );
    }
    Ok(())
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/dynamic_zone_history/tests.rs"]
mod tests;
