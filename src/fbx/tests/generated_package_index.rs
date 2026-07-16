// File:
//   - generated_package_index.rs
// Path:
//   - src/fbx/tests/generated_package_index.rs
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
//   - Regression coverage for generated FBX package-index adapter behavior.
// - Must-Not:
//   - Read private assets, rediscover packages, or parse source containers.
// - Allows:
//   - Synthetic generated-index JSONL and public adapter assertions.
// - Split-When:
//   - Filesystem loading requires an independent integration boundary.
// - Merge-When:
//   - Generated-index regressions move into shared adapter conformance tests.
// - Summary:
//   - Protects canonical generated evidence before FBX planning.
// - Description:
//   - Exercises generated package-index parsing with synthetic rows.
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

//! Regression coverage for generated FBX package-index adapter behavior.
//!
//! Synthetic JSONL protects canonical evidence without package rediscovery.

use fbx::adapters::driven::generated_package_index::{
    GeneratedPackageCatalog, PackageIndexAdapterError,
};
use fbx::ports::package_index::PackageIndexReader;
use png as _;
use schoenwald_filesystem as _;
use serde as _;
use serde_json as _;
use shar_sha256 as _;

#[test]
fn rejects_invalid_generated_package_selectors() {
    let jsonl = concat!(
        r#"{"package_id":"package","package_category":"props","#,
        r#""members":[{"id":"mesh","role":"model"}]}"#,
    );
    for selector in [
        "",
        " package",
        "package ",
        "package\nalias",
    ] {
        let result = GeneratedPackageCatalog::from_jsonl(jsonl)
            .and_then(|catalog| catalog.require_model_package(selector));
        assert_eq!(
            result,
            Err(
                PackageIndexAdapterError::InvalidPackageSelector(
                    selector.to_owned()
                )
            )
        );
    }
}

#[test]
fn rejects_generated_member_ids_unsafe_for_component_lookup() {
    for member_id in [
        "../mesh",
        "mesh:stream",
        "mesh.",
        "CON",
    ] {
        let jsonl = format!(
            concat!(
                r#"{{"package_id":"package","package_category":"props","#,
                r#""members":[{{"id":"{}","role":"model"}}]}}"#,
            ),
            member_id,
        );
        assert_eq!(
            GeneratedPackageCatalog::from_jsonl(&jsonl),
            Err(
                PackageIndexAdapterError::InvalidMemberId {
                    line: 1,
                    id: member_id.to_owned(),
                }
            )
        );
    }
}

#[test]
fn rejects_generated_indexes_without_package_rows() {
    assert_eq!(
        GeneratedPackageCatalog::from_jsonl(
            concat!(
                "
", "  ", "
", "	"
            )
        ),
        Err(PackageIndexAdapterError::EmptyCatalog)
    );
}

#[test]
fn member_order_does_not_change_generated_package_evidence() {
    let first = concat!(
        r#"{"package_id":"package","package_category":"props","#,
        r#""members":[{"id":"model-b","role":"model"},{"#,
        r#""id":"texture-b","role":"texture"},{"#,
        r#""id":"model-a","role":"model"},{"#,
        r#""id":"texture-a","role":"texture"}]}"#,
    );
    let second = concat!(
        r#"{"package_id":"package","package_category":"props","#,
        r#""members":[{"id":"texture-a","role":"texture"},{"#,
        r#""id":"model-a","role":"model"},{"#,
        r#""id":"texture-b","role":"texture"},{"#,
        r#""id":"model-b","role":"model"}]}"#,
    );
    let first_evidence = GeneratedPackageCatalog::from_jsonl(first)
        .and_then(|catalog| catalog.require_model_package("package"));
    let second_evidence = GeneratedPackageCatalog::from_jsonl(second)
        .and_then(|catalog| catalog.require_model_package("package"));

    assert_eq!(
        first_evidence,
        second_evidence
    );
}

#[test]
fn rejects_noncanonical_generated_index_fields() {
    let cases = [
        (
            concat!(
                r#"{"package_id":" package","package_category":"props","#,
                r#""members":[{"id":"mesh","role":"model"}]}"#,
            ),
            "package_id",
        ),
        (
            concat!(
                r#"{"package_id":"package","package_category":"props ","#,
                r#""members":[{"id":"mesh","role":"model"}]}"#,
            ),
            "package_category",
        ),
        (
            concat!(
                r#"{"package_id":"package","package_category":"props","#,
                r#""members":[{"id":" mesh","role":"model"}]}"#,
            ),
            "member.id",
        ),
        (
            concat!(
                r#"{"package_id":"package","package_category":"props","#,
                r#""members":[{"id":"mesh","role":"model "}]}"#,
            ),
            "member.role",
        ),
        (
            concat!(
                "{\"package_id\":\"package\\nalias\",",
                r#""package_category":"props","#,
                r#""members":[{"id":"mesh","role":"model"}]}"#,
            ),
            "package_id",
        ),
        (
            concat!(
                r#"{"package_id":"package","#,
                "\"package_category\":\"props\\nalias\",",
                r#""members":[{"id":"mesh","role":"model"}]}"#,
            ),
            "package_category",
        ),
        (
            concat!(
                r#"{"package_id":"package","package_category":"props","#,
                "\"members\":[{\"id\":\"mesh\\nalias\",",
                r#""role":"model"}]}"#,
            ),
            "member.id",
        ),
        (
            concat!(
                r#"{"package_id":"package","package_category":"props","#,
                r#""members":[{"id":"mesh","#,
                "\"role\":\"model\\nalias\"}]}\n",
            ),
            "member.role",
        ),
    ];

    for (jsonl, field) in cases {
        assert_eq!(
            GeneratedPackageCatalog::from_jsonl(jsonl),
            Err(
                PackageIndexAdapterError::NonCanonicalWhitespace {
                    line: 1,
                    field: field.to_owned(),
                }
            )
        );
    }
}
