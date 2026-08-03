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
//   - Package evidence test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Package evidence test module.
// - Description:
//   - Implements the declared test module responsibility for fbx.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Package evidence test module.

use fbx::ports::package_index::{
    ModelPackageEvidence, PackageIndexError, PackageModelFamily,
};
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

#[test]
fn canonicalizes_package_evidence_member_order() {
    let first = ModelPackageEvidence::new(
        "package",
        PackageModelFamily::Prop,
        vec!["model-b".to_owned(), "model-a".to_owned()],
        vec!["material-b".to_owned(), "material-a".to_owned()],
        vec!["texture-b".to_owned(), "texture-a".to_owned()],
        vec!["animation-b".to_owned(), "animation-a".to_owned()],
    );
    let second = ModelPackageEvidence::new(
        "package",
        PackageModelFamily::Prop,
        vec!["model-a".to_owned(), "model-b".to_owned()],
        vec!["material-a".to_owned(), "material-b".to_owned()],
        vec!["texture-a".to_owned(), "texture-b".to_owned()],
        vec!["animation-a".to_owned(), "animation-b".to_owned()],
    );

    assert_eq!(first, second);
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

    assert_eq!(result, Err(PackageIndexError::DuplicateMemberId));
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
            .all(|result| result
                == &Err(PackageIndexError::NonCanonicalIdentity))
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
            .all(|result| result
                == &Err(PackageIndexError::NonCanonicalIdentity))
    );
}
