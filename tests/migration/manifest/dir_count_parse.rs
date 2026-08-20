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
//   - Dir count parse test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Dir count parse test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Dir count parse test module.

use game_manifest::DirCount;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn dir_count_parse_preserves_kind() {
    let parsed = DirCount::parse(
        "{\"dir\":\"at\",\"ext\":\"p3d\",\"min\":1,\"kind\":\"p3d_container\"}",
    );

    assert_eq!(
        parsed.as_ref().map(|record| record.kind.as_str()),
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
    for minimum in ["1.5", "1e3"] {
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
        parsed.as_ref().map(|record| record.dir.as_str()),
        Some(original.dir.as_str())
    );
    assert_eq!(
        parsed.as_ref().map(|record| record.extension.as_str()),
        Some(original.extension.as_str())
    );
    assert_eq!(
        parsed.as_ref().map(|record| record.kind.as_str()),
        Some(original.kind.as_str())
    );

    let unicode = DirCount::parse(concat!(
        r#"{"dir":"\u0061\ud83d\ude00","ext":"p3d","#,
        r#""min":1,"kind":"p3d_container"}"#
    ));
    assert_eq!(
        unicode.as_ref().map(|record| record.dir.as_str()),
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
