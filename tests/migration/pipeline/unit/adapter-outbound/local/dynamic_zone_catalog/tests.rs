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
//   - Unit evidence for DynamicZone decoded-data and load-binding preflight.
// - Must-Not:
//   - Infer trigger traversal order or runtime package precedence.
// - Allows:
//   - Synthetic locator JSON and explicit indexed-root sets.
// - Split-When:
//   - Filesystem/package-index integration requires independent fixtures.
// - Merge-When:
//   - DynamicZone intake no longer has adapter-specific behavior.
// - Summary:
//   - DynamicZone package-transition adapter tests.
// - Description:
//   - Locks type-five decoding and load-only package-index integrity rules.
// - Usage:
//   - Included only by the local DynamicZone preflight adapter in tests.
// - Defaults:
//   - Missing unload targets remain valid remove-if-present effects.
//

//! `DynamicZone` package-transition adapter tests.

use super::*;

fn dynamic_zone_json(zone: &str) -> String {
    format!(
        concat!(
            r#"{{"schema":"locator","name":"loader11","locator_type":5,"#,
            r#""locator_type_name":"dynamic_zone","data_interpretation":{{"#,
            r#""kind":"dynamic_zone","zone":"{zone}"}},"num_triggers":1,"#,
            r#""trigger_volumes":[{{}}]}}"#
        ),
        zone = zone
    )
}

#[test]
fn parses_type_five_dynamic_zone_without_runtime_order_claim()
-> Result<(), String> {
    let decoded = parse_dynamic_zone(&dynamic_zone_json("l1z1.p3d:l1z2.p3d;"))
        .map_err(|error| error.to_string())?
        .ok_or_else(|| "dynamic zone was ignored".to_owned())?;
    assert_eq!(decoded.name, "loader11");
    assert_eq!(decoded.data, "l1z1.p3d:l1z2.p3d;");
    Ok(())
}

#[test]
fn dynamic_zone_rejects_surrounding_name_whitespace() -> Result<(), String> {
    let json = dynamic_zone_json("l1z1.p3d;").replace(
        r#""name":"loader11""#,
        r#""name":" loader11 ""#,
    );
    if parse_dynamic_zone(&json).is_ok() {
        return Err(
            "surrounding DynamicZone whitespace was accepted".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn ignores_ordinary_srr_locator_types() -> Result<(), String> {
    let json = concat!(
        r#"{"schema":"locator","name":"carstart","locator_type":3,"#,
        r#""locator_type_name":"car_start"}"#
    );
    assert_eq!(
        parse_dynamic_zone(json).map_err(|error| error.to_string())?,
        None
    );
    Ok(())
}

#[test]
fn rejects_dynamic_zone_type_name_drift() -> Result<(), String> {
    let json = concat!(
        r#"{"schema":"locator","name":"loader11","locator_type":5,"#,
        r#""locator_type_name":"car_start"}"#
    );
    let Err(error) = parse_dynamic_zone(json) else {
        return Err("type-five classification drift was accepted".to_owned());
    };
    assert!(error.to_string().contains("classification drifted"));
    Ok(())
}

#[test]
fn rejects_dynamic_zone_interpretation_kind_drift() -> Result<(), String> {
    let json = concat!(
        r#"{"schema":"locator","name":"loader11","locator_type":5,"#,
        r#""locator_type_name":"dynamic_zone","data_interpretation":{"#,
        r#""kind":"other","zone":"l1z1.p3d;"},"num_triggers":1}"#
    );
    let Err(error) = parse_dynamic_zone(json) else {
        return Err("interpretation kind drift was accepted".to_owned());
    };
    assert!(error.to_string().contains("interpretation kind drifted"));
    Ok(())
}

#[test]
fn rejects_dynamic_zone_trigger_count_drift() -> Result<(), String> {
    let json = concat!(
        r#"{"schema":"locator","name":"loader11","locator_type":5,"#,
        r#""locator_type_name":"dynamic_zone","data_interpretation":{"#,
        r#""kind":"dynamic_zone","zone":"l1z1.p3d;"},"num_triggers":2,"#,
        r#""trigger_volumes":[{}]}"#
    );
    let Err(error) = parse_dynamic_zone(json) else {
        return Err("trigger-count drift was accepted".to_owned());
    };
    assert!(error.to_string().contains("trigger count does not match"));
    Ok(())
}

#[test]
fn conflicting_package_effects_fail_dynamic_zone_preflight()
-> Result<(), String> {
    let roots = BTreeSet::from(["extracted/art/l1z1".to_owned()]);
    let Err(error) = preflight_dynamic_zone_json(
        &dynamic_zone_json("l1z1.p3d;l1z1.p3d:"),
        "extracted/art/L1_TERRA",
        &roots,
    ) else {
        return Err("conflicting package effects were accepted".to_owned());
    };
    assert!(error.to_string().contains("package ordering is unresolved"));
    assert!(error.to_string().contains("conflicting load/unload effects"));
    Ok(())
}

#[test]
fn load_target_must_exist_in_package_index() -> Result<(), String> {
    let parsed = parse_dyna_load_data("l1z1.p3d;")?;
    let transition = compile_dyna_load_package_transition(&parsed)?;
    let Err(error) =
        validate_load_targets("loader11", &transition, &BTreeSet::new())
    else {
        return Err("missing DynamicZone load target was accepted".to_owned());
    };
    assert!(error.to_string().contains("absent from the package index"));
    Ok(())
}

#[test]
fn missing_unload_target_remains_valid() -> Result<(), String> {
    let parsed = parse_dyna_load_data("l7z4.p3d:")?;
    let transition = compile_dyna_load_package_transition(&parsed)?;
    validate_load_targets("loader19", &transition, &BTreeSet::new())
        .map_err(|error| error.to_string())
}

#[test]
fn top_level_index_roots_remain_valid_lookup_entries() -> Result<(), String> {
    assert_eq!(
        normalize_index_package_root("game")
            .map_err(|error| error.to_string())?,
        "game"
    );
    assert_eq!(
        normalize_index_package_root("extracted")
            .map_err(|error| error.to_string())?,
        "extracted"
    );
    Ok(())
}

#[test]
fn complete_dynamic_zone_preflight_binds_loaded_package() -> Result<(), String>
{
    let roots = BTreeSet::from(["extracted/art/l1z1".to_owned()]);
    assert!(
        preflight_dynamic_zone_json(
            &dynamic_zone_json("l1z2.p3d:l1z1.p3d;"),
            "extracted/art/L1_TERRA",
            &roots,
        )
        .map_err(|error| error.to_string())?
        .is_some()
    );
    Ok(())
}

#[test]
fn indexed_load_target_is_admitted_after_case_normalization()
-> Result<(), String> {
    let parsed = parse_dyna_load_data("L1Z1.P3D;")?;
    let transition = compile_dyna_load_package_transition(&parsed)?;
    let roots =
        BTreeSet::from([normalize_index_package_root("extracted/art/L1Z1")
            .map_err(|error| error.to_string())?]);
    validate_load_targets("loader11", &transition, &roots)
        .map_err(|error| error.to_string())
}
