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
//   - Unit evidence for package-backed vehicle attribute tuples.
// - Must-Not:
//   - Assign semantic names to the four positional scalar values.
// - Allows:
//   - Verify exact lexeme preservation and physical vehicle binding.
// - Split-When:
//   - Vehicle-stat semantic tests gain separate authority.
// - Merge-When:
//   - Vehicle tuning catalog tests own these tuple invariants.
// - Summary:
//   - SetCarAttributes tuple unit tests.
// - Description:
//   - Proves opaque numeric source tuples bind to canonical vehicle packages.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Duplicate, symbolic, missing, or out-of-range evidence fails closed.
//

//! Unit evidence for opaque vehicle attribute tuples.

use std::collections::BTreeSet;

use super::*;

fn catalog() -> MissionReferenceCatalog {
    MissionReferenceCatalog::from_vehicle_entries_for_tests(&[
        ("famil_v", "vehicle-famil", "cars/character-rigs/famil-v"),
        ("gramR_v", "vehicle-gramr", "cars/character-rigs/gramr-v"),
    ])
}

#[test]
fn binds_exact_opaque_tuple_to_physical_vehicle() -> Result<(), String> {
    let mut bindings = Vec::new();
    let mut ids = BTreeSet::new();
    push_binding(
        &mut bindings,
        &mut ids,
        &catalog(),
        78,
        &[
            "famil_v".to_owned(),
            "1".to_owned(),
            "1.5".to_owned(),
            "2.5".to_owned(),
            "4".to_owned(),
        ],
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("vehicle attribute binding count changed".to_owned());
    };
    assert_eq!(binding.source_ordinal(), 78);
    assert_eq!(binding.vehicle_id(), "famil_v");
    assert_eq!(binding.vehicle().package_id(), "vehicle-famil");
    assert_eq!(binding.source_values(), &["1", "1.5", "2.5", "4"]);
    Ok(())
}

#[test]
fn binds_tuning_only_vehicle_without_reward_assumption() -> Result<(), String> {
    let mut bindings = Vec::new();
    let mut ids = BTreeSet::new();
    push_binding(
        &mut bindings,
        &mut ids,
        &catalog(),
        120,
        &[
            "gramR_v".to_owned(),
            "5".to_owned(),
            "5".to_owned(),
            "3.5".to_owned(),
            "3".to_owned(),
        ],
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("vehicle attribute binding count changed".to_owned());
    };
    assert_eq!(binding.vehicle().package_id(), "vehicle-gramr");
    Ok(())
}

#[test]
fn rejects_symbolic_duplicate_and_out_of_range_values() -> Result<(), String> {
    let mut bindings = Vec::new();
    let mut ids = BTreeSet::new();
    let current = [
        "current".to_owned(),
        "1".to_owned(),
        "1".to_owned(),
        "1".to_owned(),
        "1".to_owned(),
    ];
    assert!(
        push_binding(
            &mut bindings,
            &mut ids,
            &MissionReferenceCatalog::empty_for_tests(),
            1,
            &current,
        )
        .is_err()
    );

    let mut ids = BTreeSet::new();
    let valid = [
        "famil_v".to_owned(),
        "1".to_owned(),
        "1".to_owned(),
        "1".to_owned(),
        "1".to_owned(),
    ];
    push_binding(&mut bindings, &mut ids, &catalog(), 2, &valid)?;
    assert!(
        push_binding(&mut bindings, &mut ids, &catalog(), 3, &valid).is_err()
    );

    let mut ids = BTreeSet::new();
    let bad = [
        "famil_v".to_owned(),
        "0".to_owned(),
        "1".to_owned(),
        "1".to_owned(),
        "1".to_owned(),
    ];
    assert!(
        push_binding(&mut Vec::new(), &mut ids, &catalog(), 4, &bad).is_err()
    );
    Ok(())
}
