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
//   - Unit evidence for explicit DynamicZone traversal-history projection.
// - Must-Not:
//   - Infer traversal events, geometry semantics, or locator precedence.
// - Allows:
//   - Supply exact ordered traversal steps and verify package residency
//     effects.
// - Split-When:
//   - Runtime-observer integration requires physical fixtures.
// - Merge-When:
//   - Traversal history becomes part of a larger mission-session state model.
// - Summary:
//   - DynamicZone traversal-history unit tests.
// - Description:
//   - Proves caller order is preserved and unresolved per-zone ordering fails.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Empty histories preserve the initial package set.
//

use super::*;
use crate::domain::{compile_dyna_load_package_transition, parse_dyna_load_data};

fn step(
    name: &str,
    source_package_root: &str,
    dyna: &str,
) -> Result<DynamicZoneTraversalStep, String> {
    let parsed = parse_dyna_load_data(dyna)?;
    let transition = compile_dyna_load_package_transition(&parsed)?;
    DynamicZoneTraversalStep::new(
        name.to_owned(),
        source_package_root.to_owned(),
        1,
        transition,
    )
}

#[test]
fn empty_history_preserves_initial_packages() -> Result<(), String> {
    let initial = vec!["extracted/art/l1z1".to_owned()];
    let history = DynamicZoneTraversalHistory::new(Vec::new());

    assert_eq!(history.apply_package_roots(&initial)?, initial);
    Ok(())
}

#[test]
fn empty_history_still_rejects_malformed_initial_package_roots() {
    let initial = vec!["../outside".to_owned()];
    let history = DynamicZoneTraversalHistory::new(Vec::new());

    assert!(history.apply_package_roots(&initial).is_err());
}

#[test]
fn caller_supplied_history_order_controls_cross_zone_residency()
-> Result<(), String> {
    let source = "extracted/art/L1_TERRA";
    let load = step("loader_a", source, "l1z3.p3d;")?;
    let unload = step("loader_b", source, "l1z3.p3d:")?;

    let load_then_unload = DynamicZoneTraversalHistory::new(vec![
        load.clone(),
        unload.clone(),
    ]);
    let unload_then_load = DynamicZoneTraversalHistory::new(vec![unload, load]);

    assert!(load_then_unload.apply_package_roots(&[])?.is_empty());
    assert_eq!(
        unload_then_load.apply_package_roots(&[])?,
        vec!["extracted/art/l1z3".to_owned()]
    );
    Ok(())
}

#[test]
fn history_preserves_exact_zone_identity() -> Result<(), String> {
    let step = step("loader11", "extracted/art/L1_TERRA", "l1z1.p3d;")?;
    let history = DynamicZoneTraversalHistory::new(vec![step]);

    let [observed] = history.steps() else {
        return Err("expected exactly one observed DynamicZone".to_owned());
    };
    assert_eq!(observed.locator_name(), "loader11");
    assert_eq!(observed.source_package_root(), "extracted/art/L1_TERRA");
    assert_eq!(observed.transition().effects().len(), 1);
    Ok(())
}

#[test]
fn step_rejects_unsafe_source_identity() -> Result<(), String> {
    let parsed = parse_dyna_load_data("l1z1.p3d;")?;
    let transition = compile_dyna_load_package_transition(&parsed)?;
    let Err(error) = DynamicZoneTraversalStep::new(
        "loader11".to_owned(),
        "../outside".to_owned(),
        1,
        transition,
    ) else {
        return Err("unsafe source package was accepted".to_owned());
    };

    assert!(error.contains("source package root is unsafe"));
    Ok(())
}

#[test]
fn unresolved_transition_order_propagates_with_zone_identity()
-> Result<(), String> {
    let step = step(
        "loader_conflict",
        "extracted/art/L1_TERRA",
        "l1z1.p3d;l1z1.p3d:",
    )?;
    let history = DynamicZoneTraversalHistory::new(vec![step]);
    let Err(error) = history.apply_package_roots(&[]) else {
        return Err("conflicting transition was accepted".to_owned());
    };

    assert!(error.contains("loader_conflict"));
    assert!(error.contains("conflicting load/unload effects"));
    Ok(())
}

#[test]
fn first_child_entry_fires_one_zone_transition() -> Result<(), String> {
    let mut occupancy = DynamicZoneTriggerOccupancy::new(3)?;
    assert_eq!(
        occupancy.observe(1, DynamicZoneTriggerEvent::Enter)?,
        DynamicZoneTriggerEffect::ApplyTransition
    );
    assert_eq!(occupancy.active_trigger_count(), 1);
    Ok(())
}

#[test]
fn overlapping_child_volumes_do_not_retrigger_entry() -> Result<(), String> {
    let mut occupancy = DynamicZoneTriggerOccupancy::new(3)?;
    let _effect = occupancy.observe(0, DynamicZoneTriggerEvent::Enter)?;
    assert_eq!(
        occupancy.observe(2, DynamicZoneTriggerEvent::Enter)?,
        DynamicZoneTriggerEffect::NoTransition
    );
    assert_eq!(
        occupancy.observe(0, DynamicZoneTriggerEvent::Exit)?,
        DynamicZoneTriggerEffect::NoTransition
    );
    assert_eq!(occupancy.active_trigger_count(), 1);
    Ok(())
}

#[test]
fn final_exit_rearms_a_later_zone_entry() -> Result<(), String> {
    let mut occupancy = DynamicZoneTriggerOccupancy::new(2)?;
    let _effect = occupancy.observe(0, DynamicZoneTriggerEvent::Enter)?;
    let _effect = occupancy.observe(1, DynamicZoneTriggerEvent::Enter)?;
    let _effect = occupancy.observe(0, DynamicZoneTriggerEvent::Exit)?;
    let _effect = occupancy.observe(1, DynamicZoneTriggerEvent::Exit)?;
    assert_eq!(occupancy.active_trigger_count(), 0);
    assert_eq!(
        occupancy.observe(0, DynamicZoneTriggerEvent::Enter)?,
        DynamicZoneTriggerEffect::ApplyTransition
    );
    Ok(())
}

#[test]
fn traversal_step_builds_occupancy_from_decoded_count() -> Result<(), String> {
    let parsed = parse_dyna_load_data("l1z1.p3d;")?;
    let transition = compile_dyna_load_package_transition(&parsed)?;
    let step = DynamicZoneTraversalStep::new(
        "loader11".to_owned(),
        "extracted/art/L1_TERRA".to_owned(),
        3,
        transition,
    )?;
    assert_eq!(step.trigger_volume_count(), 3);
    assert_eq!(step.occupancy()?.active_trigger_count(), 0);
    Ok(())
}

#[test]
fn impossible_trigger_observations_fail_closed() -> Result<(), String> {
    let Err(error) = DynamicZoneTriggerOccupancy::new(0) else {
        return Err("trigger-less occupancy was accepted".to_owned());
    };
    assert!(error.contains("at least one volume"));

    let mut occupancy = DynamicZoneTriggerOccupancy::new(2)?;
    let Err(error) = occupancy.observe(2, DynamicZoneTriggerEvent::Enter) else {
        return Err("out-of-range trigger ordinal was accepted".to_owned());
    };
    assert!(error.contains("out of range"));

    let _effect = occupancy.observe(0, DynamicZoneTriggerEvent::Enter)?;
    let Err(error) = occupancy.observe(0, DynamicZoneTriggerEvent::Enter) else {
        return Err("duplicate trigger entry was accepted".to_owned());
    };
    assert!(error.contains("repeats an active entry"));

    let _effect = occupancy.observe(0, DynamicZoneTriggerEvent::Exit)?;
    let Err(error) = occupancy.observe(0, DynamicZoneTriggerEvent::Exit) else {
        return Err("inactive trigger exit was accepted".to_owned());
    };
    assert!(error.contains("exits an inactive volume"));
    Ok(())
}
