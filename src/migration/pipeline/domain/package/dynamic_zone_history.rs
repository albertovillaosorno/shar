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
//   - Infer traversal order or locator lookup precedence from geometry or
//     package names.
// - Allows:
//   - Apply exact caller-supplied DynamicZone transitions to active packages.
//   - Reduce observed child-volume events to conservative entry episodes.
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

/// One observed child trigger-volume boundary crossing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicZoneTriggerEvent {
    /// The tracked player entered one child trigger volume.
    Enter,
    /// The tracked player exited one child trigger volume.
    Exit,
}

/// Package-transition effect of one observed trigger-volume boundary crossing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynamicZoneTriggerEffect {
    /// Aggregate occupancy changed from zero to one active child volume.
    ApplyTransition,
    /// Occupancy changed without executing Dyna Load Data.
    NoTransition,
}

/// Exact child-volume occupancy for one decoded `DynamicZone` locator.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynamicZoneTriggerOccupancy {
    trigger_volume_count: u32,
    active_trigger_ordinals: BTreeSet<u32>,
}

impl DynamicZoneTriggerOccupancy {
    /// Build empty occupancy for one non-empty decoded trigger-volume set.
    ///
    /// # Errors
    ///
    /// Returns an error when the locator has no child trigger volumes.
    pub fn new(trigger_volume_count: u32) -> Result<Self, String> {
        if trigger_volume_count == 0 {
            return Err(
                "DynamicZone trigger occupancy requires at least one volume"
                    .to_owned(),
            );
        }
        Ok(Self {
            trigger_volume_count,
            active_trigger_ordinals: BTreeSet::new(),
        })
    }

    /// Return the number of child trigger volumes currently occupied.
    #[must_use]
    pub fn active_trigger_count(&self) -> usize {
        self.active_trigger_ordinals.len()
    }

    /// Apply one exact observed child-volume boundary crossing.
    ///
    /// The first entry in an occupancy episode executes Dyna Load Data.
    /// Overlapping entries and all exits do not. The final exit rearms the
    /// locator so a later entry can start another episode.
    ///
    /// # Errors
    ///
    /// Returns an error for an out-of-range ordinal, duplicate entry, or exit
    /// of a child volume that is not currently active.
    pub fn observe(
        &mut self,
        trigger_ordinal: u32,
        event: DynamicZoneTriggerEvent,
    ) -> Result<DynamicZoneTriggerEffect, String> {
        if trigger_ordinal >= self.trigger_volume_count {
            return Err(
                "DynamicZone trigger observation ordinal is out of range"
                    .to_owned(),
            );
        }
        match event {
            DynamicZoneTriggerEvent::Enter => {
                if !self.active_trigger_ordinals.insert(trigger_ordinal) {
                    return Err(concat!(
                        "DynamicZone trigger observation repeats an active ",
                        "entry"
                    )
                    .to_owned());
                }
                if self.active_trigger_ordinals.len() == 1 {
                    Ok(DynamicZoneTriggerEffect::ApplyTransition)
                } else {
                    Ok(DynamicZoneTriggerEffect::NoTransition)
                }
            }
            DynamicZoneTriggerEvent::Exit => {
                if !self.active_trigger_ordinals.remove(&trigger_ordinal) {
                    return Err(concat!(
                        "DynamicZone trigger observation exits an inactive ",
                        "volume"
                    )
                    .to_owned());
                }
                Ok(DynamicZoneTriggerEffect::NoTransition)
            }
        }
    }
}

/// One exact `DynamicZone` traversal supplied by an external runtime observer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DynamicZoneTraversalStep {
    locator_name: String,
    source_package_root: String,
    trigger_volume_count: u32,
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
        trigger_volume_count: u32,
        transition: DynaLoadPackageTransition,
    ) -> Result<Self, String> {
        validate_identity(&locator_name)?;
        validate_source_package_root(&source_package_root)?;
        drop(DynamicZoneTriggerOccupancy::new(trigger_volume_count)?);
        Ok(Self {
            locator_name,
            source_package_root,
            trigger_volume_count,
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

    /// Return the exact decoded child trigger-volume count.
    #[must_use]
    pub const fn trigger_volume_count(&self) -> u32 {
        self.trigger_volume_count
    }

    /// Build empty runtime occupancy for this exact decoded trigger set.
    ///
    /// # Errors
    ///
    /// Returns an error only if the stored count violates construction
    /// invariants.
    pub fn occupancy(&self) -> Result<DynamicZoneTriggerOccupancy, String> {
        DynamicZoneTriggerOccupancy::new(self.trigger_volume_count)
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
