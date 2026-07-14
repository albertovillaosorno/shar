// File:
//   - dir_count_parse.rs
// Path:
//   - src/game-manifest/tests/dir_count_parse.rs
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
//   - Canonical JSONL parsing regressions for manifest directory counts.
// - Must-Not:
//   - Depend on private local outputs or non-deterministic repository state.
// - Allows:
//   - Focused canonical rows and malformed-line assertions.
// - Split-When:
//   - Split when parsing needs filesystem-backed fixture support.
// - Merge-When:
//   - Another game-manifest test owns the same JSONL parsing boundary.
// - Summary:
//   - Protects canonical manifest record parsing.
// - Description:
//   - Verifies caller-visible rows preserve fields and reject malformed input.
// - Usage:
//   - Executed through cargo test for the game-manifest crate.
// - Defaults:
//   - Fixtures remain deterministic and repository-local.
//
// ADRs:
// - docs/adr/pipeline/game-manifest-ledger.md
//
// Large file:
//   - false
//

//! Canonical JSONL parsing regression coverage.
//!
//! These tests protect the shared parser used by validation and extraction from
//! silently changing or accepting manifest data outside the public contract.

use game_manifest::DirCount;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn dir_count_parse_preserves_kind() {
    let parsed = DirCount::parse(
        "{\"dir\":\"at\",\"ext\":\"p3d\",\"min\":1,\"kind\":\"p3d_container\"}",
    );

    assert_eq!(
        parsed
            .as_ref()
            .map(
                |record| record
                    .kind
                    .as_str()
            ),
        Some("p3d_container")
    );
}

#[test]
fn dir_count_parse_rejects_missing_kind() {
    assert!(
        DirCount::parse("{\"dir\":\"at\",\"ext\":\"p3d\",\"min\":1}").is_none()
    );
}

#[test]
fn dir_count_parse_rejects_trailing_fields() {
    assert!(
        DirCount::parse(
            "{\"dir\":\"at\",\"ext\":\"p3d\",\"min\":1,\"kind\":\"\
             p3d_container\",\"extra\":true}"
        )
        .is_none()
    );
}

#[test]
fn dir_count_parse_rejects_non_integer_minimums() {
    for minimum in [
        "1.5", "1e3",
    ] {
        let line = format!(
            "{{\"dir\":\"at\",\"ext\":\"p3d\",\"min\":{minimum},\"kind\":\"\
             p3d_container\"}}"
        );
        assert!(DirCount::parse(&line).is_none());
    }
}

#[test]
fn dir_count_parse_rejects_leading_zero_minimums() {
    assert!(
        DirCount::parse(
            "{\"dir\":\"at\",\"ext\":\"p3d\",\"min\":01,\"kind\":\"\
             p3d_container\"}"
        )
        .is_none()
    );
}

#[test]
fn dir_count_parse_decodes_json_strings() {
    let original = DirCount {
        dir: "a\"b\\c\n".to_owned(),
        extension: "p\t3d".to_owned(),
        min_count: 1,
        kind: "p3d_container".to_owned(),
    };
    let parsed = DirCount::parse(&original.to_jsonl());

    assert_eq!(
        parsed
            .as_ref()
            .map(
                |record| record
                    .dir
                    .as_str()
            ),
        Some(
            original
                .dir
                .as_str()
        )
    );
    assert_eq!(
        parsed
            .as_ref()
            .map(
                |record| record
                    .extension
                    .as_str()
            ),
        Some(
            original
                .extension
                .as_str()
        )
    );
    assert_eq!(
        parsed
            .as_ref()
            .map(
                |record| record
                    .kind
                    .as_str()
            ),
        Some(
            original
                .kind
                .as_str()
        )
    );

    let unicode = DirCount::parse(
        concat!(
            r#"{"dir":"\u0061\ud83d\ude00","ext":"p3d","#,
            r#""min":1,"kind":"p3d_container"}"#
        ),
    );
    assert_eq!(
        unicode
            .as_ref()
            .map(
                |record| record
                    .dir
                    .as_str()
            ),
        Some("a😀")
    );

    for invalid in [
        concat!(
            r#"{"dir":"\q","ext":"p3d","#,
            r#""min":1,"kind":"p3d_container"}"#
        ),
        r#"{"dir":"\ud800","ext":"p3d","min":1,"kind":"p3d_container"}"#,
    ] {
        assert!(DirCount::parse(invalid).is_none());
    }

    let raw_control = concat!(
        "{\"dir\":\"raw\ncontrol\",\"ext\":\"p3d\",",
        "\"min\":1,\"kind\":\"p3d_container\"}"
    );
    assert!(DirCount::parse(raw_control).is_none());
}

#[test]
fn dir_count_parse_rejects_surrounding_whitespace() {
    let canonical =
        "{\"dir\":\"at\",\"ext\":\"p3d\",\"min\":1,\"kind\":\"p3d_container\"}";
    assert!(DirCount::parse(&format!(" {canonical}")).is_none());
    assert!(DirCount::parse(&format!("{canonical} ")).is_none());
}

#[test]
fn dir_count_parse_rejects_unknown_kind() {
    assert!(
        DirCount::parse(
            "{\"dir\":\"at\",\"ext\":\"p3d\",\"min\":1,\"kind\":\"\
             invented_kind\"}"
        )
        .is_none()
    );
}
