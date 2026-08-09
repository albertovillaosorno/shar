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
//   - Local decoded mission locator intake unit regressions.
// - Must-Not:
//   - Depend on the installed game or infer locator load precedence.
// - Allows:
//   - Synthetic decoded locator JSON and extracted-root paths.
// - Split-When:
//   - Physical path and JSON decoding policies diverge independently.
// - Merge-When:
//   - Locator intake no longer has adapter-specific behavior.
// - Summary:
//   - Mission locator catalog adapter tests.
// - Description:
//   - Proves trailing NUL normalization and fail-closed path/schema handling.
// - Usage:
//   - Included only by the local mission-locator catalog adapter in tests.
// - Defaults:
//   - Interior control data and unsafe relative paths fail closed.
//

use super::*;

#[test]
fn decoded_locator_trims_only_trailing_nul_padding() -> Result<(), String> {
    let decoded = parse_decoded_locator(
        r#"{"schema":"locator","name":"check1\u0000\u0000","locator_type":0,"locator_type_name":"event"}"#,
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(decoded.name, "check1");
    assert_eq!(decoded.locator_type, 0);
    assert_eq!(decoded.locator_type_name, "event");
    Ok(())
}

#[test]
fn decoded_locator_rejects_interior_nul() -> Result<(), String> {
    let Err(error) = parse_decoded_locator(
        r#"{"schema":"locator","name":"bad\u0000name","locator_type":3,"locator_type_name":"car_start"}"#,
    ) else {
        return Err("interior control data did not fail closed".to_owned());
    };
    assert!(error.to_string().contains("interior control"));
    Ok(())
}

#[test]
fn decoded_locator_rejects_schema_drift() -> Result<(), String> {
    let Err(error) = parse_decoded_locator(
        r#"{"schema":"other","name":"check1","locator_type":0,"locator_type_name":"event"}"#,
    ) else {
        return Err("schema drift did not fail closed".to_owned());
    };
    assert!(error.to_string().contains("schema is not supported"));
    Ok(())
}

#[test]
fn locator_path_stays_below_configured_extracted_root() -> Result<(), String> {
    let root = Path::new("C:/work/extracted");
    let path = resolve_locator_path(
        root,
        "extracted/art/missions/level01/bm1/components/srr_locator/check1.json",
    )
    .map_err(|error| error.to_string())?;
    assert_eq!(
        path,
        root.join("art/missions/level01/bm1/components/srr_locator/check1.json")
    );
    Ok(())
}

#[test]
fn locator_path_rejects_traversal() -> Result<(), String> {
    let Err(error) = resolve_locator_path(
        Path::new("C:/work/extracted"),
        "extracted/art/../outside.json",
    ) else {
        return Err("locator path traversal did not fail closed".to_owned());
    };
    assert!(
        error
            .to_string()
            .contains("unsafe locator member relative path")
    );
    Ok(())
}

#[test]
fn accepts_only_observed_locator_member_classifications() {
    assert!(validate_locator_member("locator", "p3d-locator", "srr_locator").is_ok());
    assert!(validate_locator_member("locator", "p3d-locator", "locator").is_ok());
    assert!(validate_locator_member("other", "p3d-locator", "srr_locator").is_err());
    assert!(validate_locator_member("locator", "p3d-locator", "unknown").is_err());
}
