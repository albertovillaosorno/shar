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

use std::collections::BTreeSet;

use super::dyna_load_package::validate_active_package_roots;
use super::DynaLoadPackageTransition;

/// One exact `DynamicZone` traversal supplied by an external runtime observer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynamicZoneTraversalStep {
    locator_name: String,
    source_package_root: String,
    trigger_count: u32,
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
        trigger_count: u32,
        transition: DynaLoadPackageTransition,
    ) -> Result<Self, String> {
        validate_identity(&locator_name)?;
        validate_source_package_root(&source_package_root)?;
        if trigger_count == 0 {
            return Err(
                "DynamicZone traversal trigger count must be positive".to_owned()
            );
        }
        Ok(Self {
            locator_name,
            source_package_root,
            trigger_count,
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

    /// Return the number of trigger volumes owned by this locator.
    #[must_use]
    pub const fn trigger_count(&self) -> u32 {
        self.trigger_count
    }

    /// Return the typed Dyna package transition executed by this zone.
    #[must_use]
    pub const fn transition(&self) -> &DynaLoadPackageTransition {
        &self.transition
    }
}

/// Per-locator trigger-volume state for one observed runtime session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynamicZoneTriggerState {
    step: DynamicZoneTraversalStep,
    active_trigger_indices: BTreeSet<u32>,
}

impl DynamicZoneTriggerState {
    /// Start observing one zone with no active child volumes.
    #[must_use]
    pub const fn new(step: DynamicZoneTraversalStep) -> Self {
        Self {
            step,
            active_trigger_indices: BTreeSet::new(),
        }
    }

    /// Observe one child trigger-volume boundary transition.
    ///
    /// The first child entry of an episode emits the traversal. Overlapping
    /// child entries and every exit do not. After the final exit, a later
    /// entry begins a new episode and emits again.
    ///
    /// # Errors
    ///
    /// Returns an error for an out-of-range child index, duplicate entry, or
    /// an exit for a child that is not active.
    pub fn observe_volume(
        &mut self,
        trigger_index: u32,
        entered: bool,
    ) -> Result<Option<DynamicZoneTraversalStep>, String> {
        if trigger_index >= self.step.trigger_count {
            return Err(
                "DynamicZone trigger observation index is out of range".to_owned()
            );
        }
        if entered {
            let was_empty = self.active_trigger_indices.is_empty();
            if !self.active_trigger_indices.insert(trigger_index) {
                return Err(
                    "DynamicZone trigger observation duplicated an entry".to_owned()
                );
            }
            return Ok(was_empty.then(|| self.step.clone()));
        }
        if !self.active_trigger_indices.remove(&trigger_index) {
            return Err(
                "DynamicZone trigger observation exited an inactive volume"
                    .to_owned()
            );
        }
        Ok(None)
    }

    /// Return active child trigger indices in deterministic order.
    #[must_use]
    pub fn active_trigger_indices(&self) -> &BTreeSet<u32> {
        &self.active_trigger_indices
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
