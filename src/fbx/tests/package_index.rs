// File:
//   - package_index.rs
// Path:
//   - src/fbx/tests/package_index.rs
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
//   - Regression coverage for FBX package-index evidence invariants.
// - Must-Not:
//   - Access private assets, rediscover packages, or parse `Pure3D` sources.
// - Allows:
//   - Synthetic generated-index evidence and public port assertions.
// - Split-When:
//   - Adapter JSONL behavior requires a distinct integration boundary.
// - Merge-When:
//   - Package evidence no longer requires independent port-level coverage.
// - Summary:
//   - Protects stable package evidence consumed by FBX planning.
// - Description:
//   - Exercises package-index port models with synthetic member identities.
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

//! Regression coverage for FBX package-index evidence invariants.
//!
//! Synthetic evidence protects stable identity without filesystem discovery.

use fbx::adapters::driven::generated_package_index::{
    GeneratedPackageCatalog, PackageIndexAdapterError,
};
use fbx::ports::package_index::{
    ModelPackageEvidence, PackageIndexError, PackageIndexReader,
    PackageModelFamily,
};
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;

#[test]
fn rejects_unknown_jsonl_fields() {
    let cases = [
        GeneratedPackageCatalog::from_jsonl(
            concat!(
                r#"{"package_id":"package","package_category":"props","#,
                r#""extra":1,"members":["#,
                r#"{"id":"mesh","role":"model"}]}"#,
            ),
        ),
        GeneratedPackageCatalog::from_jsonl(
            concat!(
                r#"{"package_id":"package","package_category":"props","#,
                r#""members":["#,
                r#"{"id":"mesh","role":"model","extra":1}]}"#,
            ),
        ),
    ];

    assert!(
        cases
            .iter()
            .all(
                |result| matches!(
                    result,
                    Err(
                        PackageIndexAdapterError::Parse {
                            line: 1,
                            ..
                        }
                    )
                )
            )
    );
}

#[test]
fn rejects_duplicate_member_ids_while_reading_jsonl() {
    let jsonl = concat!(
        r#"{"package_id":"package","package_category":"props","members":["#,
        r#"{"id":"shared","role":"model"},"#,
        r#"{"id":"shared","role":"material"}]}"#,
    );

    assert_eq!(
        GeneratedPackageCatalog::from_jsonl(jsonl),
        Err(
            PackageIndexAdapterError::DuplicateMemberId {
                line: 1,
                id: "shared".to_owned(),
            }
        )
    );
}

#[test]
fn rejects_unknown_member_roles_while_reading_jsonl() {
    let jsonl = concat!(
        r#"{"package_id":"package","package_category":"props","members":["#,
        r#"{"id":"mesh","role":"model-metadata"}]}"#,
    );

    assert_eq!(
        GeneratedPackageCatalog::from_jsonl(jsonl),
        Err(
            PackageIndexAdapterError::UnknownMemberRole {
                line: 1,
                role: "model-metadata".to_owned(),
            }
        )
    );
}

#[test]
fn rejects_blank_member_roles_while_reading_jsonl() {
    let jsonl = concat!(
        r#"{"package_id":"package","package_category":"props","members":["#,
        r#"{"id":"mesh","role":"   "}]}"#,
    );

    assert_eq!(
        GeneratedPackageCatalog::from_jsonl(jsonl),
        Err(
            PackageIndexAdapterError::BlankMemberRole {
                line: 1
            }
        )
    );
}

#[test]
fn rejects_blank_member_ids_while_reading_jsonl() {
    let jsonl = concat!(
        r#"{"package_id":"package","package_category":"props","members":["#,
        r#"{"id":"   ","role":"model"}]}"#,
    );

    assert_eq!(
        GeneratedPackageCatalog::from_jsonl(jsonl),
        Err(
            PackageIndexAdapterError::BlankMemberId {
                line: 1
            }
        )
    );
}

#[test]
fn rejects_blank_package_categories_while_reading_jsonl() {
    let jsonl = concat!(
        r#"{"package_id":"package","package_category":"   ","members":["#,
        r#"{"id":"mesh","role":"model"}]}"#,
    );

    assert_eq!(
        GeneratedPackageCatalog::from_jsonl(jsonl),
        Err(
            PackageIndexAdapterError::BlankPackageCategory {
                line: 1
            }
        )
    );
}

#[test]
fn rejects_blank_package_ids_while_reading_jsonl() {
    let jsonl = concat!(
        r#"{"package_id":"   ","package_category":"props","members":["#,
        r#"{"id":"mesh","role":"model"}]}"#,
    );

    assert_eq!(
        GeneratedPackageCatalog::from_jsonl(jsonl),
        Err(
            PackageIndexAdapterError::BlankPackageId {
                line: 1
            }
        )
    );
}

#[test]
fn accepts_utf8_bom_before_first_jsonl_row() {
    let jsonl = concat!(
        "\u{feff}",
        r#"{"package_id":"package","package_category":"props","members":["#,
        r#"{"id":"mesh","role":"model"}]}"#,
    );

    assert!(GeneratedPackageCatalog::from_jsonl(jsonl).is_ok());
}

#[test]
fn classification_uses_canonical_roles_only() {
    let jsonl = concat!(
        r#"{"package_id":"package","package_category":"props","members":["#,
        r#"{"id":"mesh","role":"model"},"#,
        r#"{"id":"metadata","role":"metadata","kind":"mesh-metadata","#,
        r#""source_chunk_kind":"mesh"}]}"#,
    );
    let result = GeneratedPackageCatalog::from_jsonl(jsonl)
        .and_then(|catalog| catalog.require_model_package("package"))
        .map(|evidence| evidence.model_member_ids);

    assert_eq!(
        result,
        Ok(vec!["mesh".to_owned()])
    );
}

#[test]
fn rejects_duplicate_member_identities() {
    let cases = [
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec![
                "mesh".to_owned(),
                "mesh".to_owned(),
            ],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec!["mesh".to_owned()],
            vec![
                "material".to_owned(),
                "material".to_owned(),
            ],
            Vec::new(),
            Vec::new(),
        ),
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec!["shared".to_owned()],
            vec!["shared".to_owned()],
            Vec::new(),
            Vec::new(),
        ),
    ];

    assert!(
        cases
            .iter()
            .all(|result| result == &Err(PackageIndexError::DuplicateMemberId))
    );
}

#[test]
fn rejects_blank_member_identities() {
    let cases = [
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec!["   ".to_owned()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec!["mesh".to_owned()],
            vec!["   ".to_owned()],
            Vec::new(),
            Vec::new(),
        ),
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec!["mesh".to_owned()],
            Vec::new(),
            vec!["   ".to_owned()],
            Vec::new(),
        ),
        ModelPackageEvidence::new(
            "package",
            PackageModelFamily::Prop,
            vec!["mesh".to_owned()],
            Vec::new(),
            Vec::new(),
            vec!["   ".to_owned()],
        ),
    ];

    assert!(
        cases
            .iter()
            .all(|result| result == &Err(PackageIndexError::BlankMemberId))
    );
}

#[test]
fn rejects_blank_package_identity() {
    let result = ModelPackageEvidence::new(
        "   ",
        PackageModelFamily::Prop,
        vec!["mesh-1".to_owned()],
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );

    assert_eq!(
        result,
        Err(PackageIndexError::MissingPackageId)
    );
}
