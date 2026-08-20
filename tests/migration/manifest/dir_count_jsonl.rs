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
//   - Dir count jsonl test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Dir count jsonl test module.
// - Description:
//   - Implements the declared test module responsibility for manifest.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Dir count jsonl test module.

use game_manifest::DirCount;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

#[test]
fn observation_jsonl_uses_count_not_minimum() {
    let record = DirCount {
        dir: "at/cs".to_owned(),
        extension: "p3d".to_owned(),
        min_count: 2,
        kind: "p3d_container".to_owned(),
    };

    assert_eq!(
        record.to_observation_jsonl(),
        concat!(
            "{\"dir\":\"at/cs\",\"ext\":\"p3d\",",
            "\"count\":2,\"kind\":\"p3d_container\"}"
        )
    );
    assert!(!record.to_observation_jsonl().contains("\"min\""));
}

#[test]
fn dir_count_jsonl_escapes_quotes_in_directory() {
    let record = DirCount {
        dir: "a\"b".to_owned(),
        extension: "p3d".to_owned(),
        min_count: 1,
        kind: "p3d_container".to_owned(),
    };

    assert_eq!(
        record.to_jsonl(),
        "{\"dir\":\"a\\\"b\",\"ext\":\"p3d\",\"min\":1,\"kind\":\"\
         p3d_container\"}"
    );
}

#[test]
fn dir_count_jsonl_escapes_quotes_in_extension() {
    let record = DirCount {
        dir: "at".to_owned(),
        extension: "p\"3d".to_owned(),
        min_count: 1,
        kind: "p3d_container".to_owned(),
    };

    assert_eq!(
        record.to_jsonl(),
        "{\"dir\":\"at\",\"ext\":\"p\\\"3d\",\"min\":1,\"kind\":\"\
         p3d_container\"}"
    );
}

#[test]
fn dir_count_jsonl_escapes_quotes_in_kind() {
    let record = DirCount {
        dir: "at".to_owned(),
        extension: "p3d".to_owned(),
        min_count: 1,
        kind: "p3d_\"container".to_owned(),
    };

    assert_eq!(
        record.to_jsonl(),
        "{\"dir\":\"at\",\"ext\":\"p3d\",\"min\":1,\"kind\":\"p3d_\\\"\
         container\"}"
    );
}
