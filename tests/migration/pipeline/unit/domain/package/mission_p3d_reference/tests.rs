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
//   - Shared mission P3D package-reference catalog unit regressions.
// - Must-Not:
//   - Read repository files or assign runtime meaning to P3D references.
// - Allows:
//   - Synthetic package entries and authored path normalization checks.
// - Split-When:
//   - Catalog construction and path normalization diverge independently.
// - Merge-When:
//   - The shared P3D reference catalog loses independent behavior.
// - Summary:
//   - Mission P3D package-reference tests.
// - Description:
//   - Proves portable path normalization and exact canonical package binding.
// - Usage:
//   - Included only by the mission P3D reference domain module.
// - Defaults:
//   - Missing, unsafe, and non-P3D references fail closed.
//

//! Mission P3D package-reference tests.

use super::*;

fn catalog() -> MissionP3dReferenceCatalog {
    MissionP3dReferenceCatalog {
        by_root: BTreeMap::from([(
            "extracted/art/frontend/dynaload/images/mis01_01".to_owned(),
            MissionP3dCatalogEntry {
                package_id: "presentation-package".to_owned(),
                package_root: concat!(
                    "extracted/art/frontend/dynaload/images/",
                    "mis01_01"
                )
                .to_owned(),
            },
        )]),
    }
}

#[test]
fn resolves_windows_path_case_insensitively() -> Result<(), String> {
    let reference = catalog().resolve(
        r"ART\FRONTEND\DYNALOAD\IMAGES\MIS01_01.P3D",
    )?;
    assert_eq!(
        reference.source_reference(),
        r"ART\FRONTEND\DYNALOAD\IMAGES\MIS01_01.P3D"
    );
    assert_eq!(reference.package_id(), "presentation-package");
    assert_eq!(
        reference.package_root(),
        "extracted/art/frontend/dynaload/images/mis01_01"
    );
    Ok(())
}

#[test]
fn rejects_missing_and_non_p3d_references() {
    assert!(catalog().resolve("art/missing.p3d").is_err());
    assert!(catalog().resolve("art/missing.json").is_err());
}

#[test]
fn rejects_traversal_reference() {
    assert!(catalog().resolve(r"art\..\outside.p3d").is_err());
}

#[test]
fn package_candidate_filter_preserves_existing_contract()
-> Result<(), String> {
    assert_eq!(normalized_candidate_package_root("game")?, None);
    assert_eq!(normalized_candidate_package_root("extracted")?, None);
    assert_eq!(
        normalized_candidate_package_root(r"EXTRACTED\ART\L01_FX")?,
        Some("extracted/art/l01_fx".to_owned())
    );
    Ok(())
}
