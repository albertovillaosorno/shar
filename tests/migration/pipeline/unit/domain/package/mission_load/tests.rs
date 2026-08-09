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
//   - Mission P3D package-load semantic unit regressions.
// - Must-Not:
//   - Read files or assign behavior to optional load groups.
// - Allows:
//   - Synthetic authored P3D paths and package roots.
// - Split-When:
//   - Package path and optional-group policies diverge independently.
// - Merge-When:
//   - Mission load binding loses independent semantic policy.
// - Summary:
//   - Mission package-load tests.
// - Description:
//   - Proves portable P3D path binding and fail-closed unsafe input handling.
// - Usage:
//   - Included only by the mission-load domain module under cfg(test).
// - Defaults:
//   - Optional load groups remain opaque strings.
//

use super::*;

#[test]
fn maps_authored_windows_p3d_path_to_canonical_package_root() -> Result<(), String> {
    assert_eq!(
        normalized_p3d_package_root(r"ART\MISSIONS\LEVEL01\BM1.P3D")?,
        "extracted/art/missions/level01/bm1"
    );
    Ok(())
}

#[test]
fn rejects_non_p3d_and_traversal_paths() {
    for reference in [r"art\missions\m1.json", r"art\..\outside.p3d"] {
        assert!(normalized_p3d_package_root(reference).is_err());
    }
}

#[test]
fn preserves_valid_optional_group_as_opaque_identity() {
    assert!(validate_opaque_group("GMA_LEVEL_OTHER").is_ok());
    assert!(validate_opaque_group(" GMA_LEVEL_OTHER").is_err());
    assert!(validate_opaque_group("GMA_LEVEL_OTHER_2").is_ok());
    assert!(validate_opaque_group("GMA\u{0}LEVEL").is_err());
}

#[test]
fn package_root_normalization_matches_transport_case_and_separator() -> Result<(), String> {
    assert_eq!(
        normalized_package_root(r"EXTRACTED\ART\L01_FX")?,
        "extracted/art/l01_fx"
    );
    Ok(())
}

#[test]
fn package_candidate_filter_ignores_non_extracted_namespaces() -> Result<(), String> {
    assert_eq!(normalized_candidate_package_root("game")?, None);
    assert_eq!(normalized_candidate_package_root("extracted")?, None);
    assert_eq!(
        normalized_candidate_package_root(r"EXTRACTED\ART\L01_FX")?,
        Some("extracted/art/l01_fx".to_owned())
    );
    if normalized_candidate_package_root("extracted/art/../outside").is_ok() {
        return Err("unsafe extracted package root did not fail closed".to_owned());
    }
    Ok(())
}
