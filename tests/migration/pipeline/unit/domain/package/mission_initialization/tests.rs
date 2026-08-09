// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT

//! Mission initialization helper regressions.

use super::{validate_identity, validate_p3d_path};

#[test]
fn accepts_reviewed_locator_and_p3d_shapes() -> Result<(), String> {
    validate_identity("m4_carstart", "fixture locator")?;
    validate_p3d_path("l7i02.p3d")?;
    Ok(())
}

#[test]
fn rejects_escaping_or_non_p3d_load_references() {
    assert!(validate_p3d_path("../l1z1.p3d").is_err());
    assert!(validate_p3d_path("/l1z1.p3d").is_err());
    assert!(validate_p3d_path("l1z1.txt").is_err());
}

#[test]
fn rejects_path_shaped_locator_identity() {
    assert!(validate_identity("missions/m1", "fixture locator").is_err());
}

#[test]
fn preserves_street_race_load_and_unload_terminators() -> Result<(), String> {
    let load = super::compile_street_race_props(
        &["l1_sr1p.p3d;".to_owned()],
        ';',
        "load",
    )?;
    if load != ("l1_sr1p.p3d;".to_owned(), vec!["l1_sr1p.p3d".to_owned()]) {
        return Err("street-race load evidence changed".to_owned());
    }
    let unload = super::compile_street_race_props(
        &["l1_sr1p.p3d:".to_owned()],
        ':',
        "unload",
    )?;
    if unload != ("l1_sr1p.p3d:".to_owned(), vec!["l1_sr1p.p3d".to_owned()]) {
        return Err("street-race unload evidence changed".to_owned());
    }
    assert!(
        super::compile_street_race_props(
            &["l1_sr1p.p3d:".to_owned()],
            ';',
            "load",
        )
        .is_err()
    );
    assert!(
        super::compile_street_race_props(
            &["../escape.p3d;".to_owned()],
            ';',
            "load",
        )
        .is_err()
    );
    Ok(())
}
