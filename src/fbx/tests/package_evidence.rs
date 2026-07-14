// File:
//   - package_evidence.rs
// Path:
//   - src/fbx/tests/package_evidence.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Regression coverage for FBX package evidence value invariants.
// - Must-Not:
//   - Read files, parse indexes, or depend on concrete adapters.
// - Allows:
//   - Synthetic package identities and public port construction.
// - Split-When:
//   - One evidence family requires independent fixtures or adapters.
// - Merge-When:
//   - Package value regressions move into shared port conformance tests.
// - Summary:
//   - Protects canonical package evidence before application planning.
// - Description:
//   - Exercises package evidence construction with synthetic identities.
// - Usage:
//   - Run through the fbx crate test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/fbx/hexagonal-scene-export.md
//
// Large file:
//   - false
//

//! Regression coverage for FBX package evidence value invariants.

use fbx::ports::package_index::{
    ModelPackageEvidence, PackageIndexError, PackageModelFamily,
};
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;

#[test]
fn canonicalizes_package_evidence_member_order() {
    let first = ModelPackageEvidence::new(
        "package",
        PackageModelFamily::Prop,
        vec![
            "model-b".to_owned(),
            "model-a".to_owned(),
        ],
        vec![
            "material-b".to_owned(),
            "material-a".to_owned(),
        ],
        vec![
            "texture-b".to_owned(),
            "texture-a".to_owned(),
        ],
        vec![
            "animation-b".to_owned(),
            "animation-a".to_owned(),
        ],
    );
    let second = ModelPackageEvidence::new(
        "package",
        PackageModelFamily::Prop,
        vec![
            "model-a".to_owned(),
            "model-b".to_owned(),
        ],
        vec![
            "material-a".to_owned(),
            "material-b".to_owned(),
        ],
        vec![
            "texture-a".to_owned(),
            "texture-b".to_owned(),
        ],
        vec![
            "animation-a".to_owned(),
            "animation-b".to_owned(),
        ],
    );

    assert_eq!(
        first,
        second
    );
}

#[test]
fn rejects_case_insensitive_member_aliases() {
    let result = ModelPackageEvidence::new(
        "package",
        PackageModelFamily::Prop,
        vec!["Mesh".to_owned()],
        vec!["mesh".to_owned()],
        Vec::new(),
        Vec::new(),
    );

    assert_eq!(
        result,
        Err(PackageIndexError::DuplicateMemberId)
    );
}

#[test]
fn rejects_noncanonical_package_evidence_identities() {
    let cases = [
        ModelPackageEvidence::new(
            " package",
            PackageModelFamily::Prop,
            vec!["mesh".to_owned()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ModelPackageEvidence::new(
            "package\nalias",
            PackageModelFamily::Prop,
            vec!["mesh".to_owned()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec!["mesh ".to_owned()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
    ];

    assert!(
        cases
            .iter()
            .all(
                |result| result
                    == &Err(PackageIndexError::NonCanonicalIdentity)
            )
    );
}

#[test]
fn rejects_nonportable_package_member_ids() {
    let cases = [
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec!["../mesh".to_owned()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec!["mesh.".to_owned()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
    ];

    assert!(
        cases
            .iter()
            .all(
                |result| result
                    == &Err(PackageIndexError::NonCanonicalIdentity)
            )
    );
}
