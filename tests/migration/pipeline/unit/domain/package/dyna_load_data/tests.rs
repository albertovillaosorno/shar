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
//   - Dyna Load Data syntax regression tests.
// - Must-Not:
//   - Production behavior or private source evidence.
// - Allows:
//   - Synthetic Dyna Load Data strings and exact parsed-operation assertions.
// - Split-When:
//   - Another Dyna Load grammar gains independent test fixtures.
// - Merge-When:
//   - The parser no longer has independently testable syntax behavior.
// - Summary:
//   - Dyna Load Data syntax regressions.
// - Description:
//   - Preserves focused unit evidence for the owning package-domain behavior.
// - Usage:
//   - Included only by the owning package-domain module under cfg(test).
// - Defaults:
//   - Invalid or ambiguous synthetic evidence fails closed.
//

//! Dyna Load Data syntax regressions.

use super::{DynaLoadOperationKind, parse_dyna_load_data};

#[test]
fn parses_all_documented_postfix_operations_in_order() -> Result<(), String> {
    let source = concat!(
        "l1z1.p3d;",
        "l1z2.p3d:",
        "l1i00.p3d@",
        "l1i01.p3d$",
        "visibility_a*",
        "visibility_b&"
    );
    let parsed = parse_dyna_load_data(source)?;
    if parsed.source() != source {
        return Err("Dyna Load Data source evidence changed".to_owned());
    }
    let expected = [
        ("l1z1.p3d", DynaLoadOperationKind::RegionLoad),
        ("l1z2.p3d", DynaLoadOperationKind::RegionUnload),
        ("l1i00.p3d", DynaLoadOperationKind::InteriorLoad),
        ("l1i01.p3d", DynaLoadOperationKind::InteriorUnload),
        ("visibility_a", DynaLoadOperationKind::WorldSphereEnable),
        ("visibility_b", DynaLoadOperationKind::WorldSphereDisable),
    ];
    let actual = parsed
        .operations()
        .iter()
        .map(|operation| (operation.target(), operation.kind()))
        .collect::<Vec<_>>();
    if actual != expected {
        // jig-ignore-next-line: literal
        return Err(format!("Dyna Load Data operation order drifted: {actual:?}"));
    }
    Ok(())
}

#[test]
fn classifies_p3d_load_and_unload_operations() {
    use DynaLoadOperationKind::{
        InteriorLoad, InteriorUnload, RegionLoad, RegionUnload,
        WorldSphereDisable, WorldSphereEnable,
    };

    assert!(RegionLoad.is_p3d_load());
    assert!(InteriorLoad.is_p3d_load());
    assert!(RegionUnload.is_p3d_unload());
    assert!(InteriorUnload.is_p3d_unload());
    assert!(!WorldSphereEnable.is_p3d_load());
    assert!(!WorldSphereDisable.is_p3d_unload());
}

#[test]
fn rejects_blank_unterminated_empty_or_unsafe_operations() {
    for source in [
        "",
        " l1z1.p3d;",
        "l1z1.p3d",
        ";",
        "l1z1.p3d;;",
        "../l1z1.p3d;",
        "/l1z1.p3d;",
        "C:/l1z1.p3d;",
        "l1z1.txt;",
        "visibility_a*tail",
    ] {
        assert!(
            parse_dyna_load_data(source).is_err(),
            "malformed Dyna Load Data was accepted: {source:?}"
        );
    }
}
