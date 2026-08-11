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
//   - Mission presentation package-reference unit regressions.
// - Must-Not:
//   - Infer presentation timing or drawable semantics.
// - Allows:
//   - Synthetic P3D catalog references and deterministic binding checks.
// - Split-When:
//   - Semantic-scope extraction gains separate fixture requirements.
// - Merge-When:
//   - Presentation package binding loses independent behavior.
// - Summary:
//   - Mission presentation package-reference tests.
// - Description:
//   - Proves canonical binding metadata and fail-closed missing references.
// - Usage:
//   - Included only by the mission presentation reference domain module.
// - Defaults:
//   - No fallback package identity is invented.
//

use super::*;

fn catalog() -> MissionP3dReferenceCatalog {
    MissionP3dReferenceCatalog::from_entries_for_tests(&[(
        "extracted/art/frontend/dynaload/images/mis01_01",
        "presentation-package",
        "extracted/art/frontend/dynaload/images/mis01_01",
    )])
}

#[test]
fn binds_exact_presentation_path_to_package() -> Result<(), String> {
    let mut bindings = Vec::new();
    push_binding(
        &mut bindings,
        &catalog(),
        17,
        MissionPresentationRole::Objective,
        Some((12, 2)),
        Some(15),
        "art/frontend/dynaload/images/mis01_01.p3d",
    )?;
    let [binding] = bindings.as_slice() else {
        return Err("presentation binding count changed".to_owned());
    };
    assert_eq!(binding.source_ordinal(), 17);
    assert_eq!(binding.role(), MissionPresentationRole::Objective);
    assert_eq!(binding.owner_stage_source_ordinal(), Some(12));
    assert_eq!(binding.owner_stage_sequence_ordinal(), Some(2));
    assert_eq!(binding.owner_objective_source_ordinal(), Some(15));
    assert_eq!(
        binding.source_reference(),
        "art/frontend/dynaload/images/mis01_01.p3d"
    );
    assert_eq!(binding.package_id(), "presentation-package");
    assert_eq!(
        binding.package_root(),
        "extracted/art/frontend/dynaload/images/mis01_01"
    );
    Ok(())
}

#[test]
fn missing_presentation_reference_fails_closed() {
    let mut bindings = Vec::new();
    assert!(
        push_binding(
            &mut bindings,
            &catalog(),
            18,
            MissionPresentationRole::Stage,
            Some((12, 2)),
            None,
            "art/frontend/dynaload/images/missing.p3d",
        )
        .is_err()
    );
    assert!(bindings.is_empty());
}

#[test]
fn rejects_inconsistent_presentation_ownership() {
    let mut bindings = Vec::new();
    assert!(
        push_binding(
            &mut bindings,
            &catalog(),
            17,
            MissionPresentationRole::Objective,
            Some((12, 2)),
            None,
            "art/frontend/dynaload/images/mis01_01.p3d",
        )
        .is_err()
    );
    assert!(bindings.is_empty());
}
