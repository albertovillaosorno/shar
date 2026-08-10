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
//   - Unit evidence for pure Dyna Load Data package transitions.
// - Must-Not:
//   - Infer trigger order or package precedence from transition syntax.
// - Allows:
//   - Verify target normalization, operation order, and explicit set effects.
// - Split-When:
//   - DynamicZone filesystem binding gains independent fixtures.
// - Merge-When:
//   - Package-transition evidence moves to an integration boundary.
// - Summary:
//   - Dyna Load Data package-transition unit tests.
// - Description:
//   - Locks conservative P3D load/unload behavior without runtime-order claims.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - World Sphere operations never mutate P3D package roots.
//

use super::super::{DynaLoadOperationKind, parse_dyna_load_data};
use super::compile_dyna_load_package_transition;

#[test]
fn transition_normalizes_shorthand_and_explicit_art_targets()
-> Result<(), String> {
    let data = parse_dyna_load_data(r"L1Z1.P3D;art\l1i01.p3d@")?;
    let transition = compile_dyna_load_package_transition(&data)?;
    let effects = transition.effects();

    assert_eq!(transition.source(), data.source());
    assert_eq!(effects.len(), 2);
    assert_eq!(effects[0].kind(), DynaLoadOperationKind::RegionLoad);
    assert_eq!(effects[0].source_target(), "L1Z1.P3D");
    assert_eq!(effects[0].package_root(), Some("extracted/art/l1z1"));
    assert_eq!(effects[1].kind(), DynaLoadOperationKind::InteriorLoad);
    assert_eq!(effects[1].package_root(), Some("extracted/art/l1i01"));
    Ok(())
}

#[test]
fn transition_preserves_authored_effect_order() -> Result<(), String> {
    let data = parse_dyna_load_data("l1z1.p3d;l1z2.p3d:l1i01.p3d@l1i02.p3d$")?;
    let transition = compile_dyna_load_package_transition(&data)?;
    let kinds = transition
        .effects()
        .iter()
        .map(|effect| effect.kind())
        .collect::<Vec<_>>();

    assert_eq!(kinds, vec![
        DynaLoadOperationKind::RegionLoad,
        DynaLoadOperationKind::RegionUnload,
        DynaLoadOperationKind::InteriorLoad,
        DynaLoadOperationKind::InteriorUnload,
    ]);
    Ok(())
}

#[test]
fn world_sphere_effects_preserve_target_without_package_identity()
-> Result<(), String> {
    let data = parse_dyna_load_data("sphere_a*sphere_b&")?;
    let transition = compile_dyna_load_package_transition(&data)?;

    assert_eq!(transition.effects()[0].source_target(), "sphere_a");
    assert_eq!(transition.effects()[0].package_root(), None);
    assert_eq!(transition.effects()[1].source_target(), "sphere_b");
    assert_eq!(transition.effects()[1].package_root(), None);
    Ok(())
}

#[test]
fn applies_order_independent_loads_and_unloads() -> Result<(), String> {
    let data = parse_dyna_load_data("l1z1.p3d:l1z3.p3d;")?;
    let transition = compile_dyna_load_package_transition(&data)?;
    let active = vec![
        "extracted/art/L1Z1".to_owned(),
        "extracted/art/l1z2".to_owned(),
    ];

    assert_eq!(
        transition.apply_order_independent_package_roots(&active)?,
        vec![
            "extracted/art/l1z2".to_owned(),
            "extracted/art/l1z3".to_owned(),
        ]
    );
    Ok(())
}

#[test]
fn unload_of_absent_package_is_a_deterministic_noop() -> Result<(), String> {
    let data = parse_dyna_load_data("l7z4.p3d:")?;
    let transition = compile_dyna_load_package_transition(&data)?;
    let active = vec!["extracted/art/l7z1".to_owned()];

    assert_eq!(
        transition.apply_order_independent_package_roots(&active)?,
        active
    );
    Ok(())
}

#[test]
fn conflicting_load_and_unload_requires_runtime_order_evidence()
-> Result<(), String> {
    let data = parse_dyna_load_data("l1z1.p3d;l1z1.p3d:")?;
    let transition = compile_dyna_load_package_transition(&data)?;
    let error = transition
        .apply_order_independent_package_roots(&[])
        .expect_err("conflicting package effects must remain unresolved");
    assert!(error.contains("conflicting load/unload effects"));
    Ok(())
}

#[test]
fn transition_rejects_malformed_active_package_roots() -> Result<(), String> {
    let data = parse_dyna_load_data("l1z1.p3d;")?;
    let transition = compile_dyna_load_package_transition(&data)?;

    assert!(
        transition
            .apply_order_independent_package_roots(&["../outside".to_owned()])
            .is_err()
    );
    Ok(())
}
