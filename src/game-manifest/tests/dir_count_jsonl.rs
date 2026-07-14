// File:
//   - dir_count_jsonl.rs
// Path:
//   - src/game-manifest/tests/dir_count_jsonl.rs
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
//   - Canonical JSONL serialization regressions for manifest directory counts.
// - Must-Not:
//   - Depend on private local outputs or non-deterministic repository state.
// - Allows:
//   - Focused values and deterministic assertions for JSONL output.
// - Split-When:
//   - Split when serialization and parsing need independent fixture support.
// - Merge-When:
//   - Another game-manifest test owns the same JSONL serialization boundary.
// - Summary:
//   - Protects canonical manifest record serialization.
// - Description:
//   - Verifies caller-visible JSONL remains valid for adversarial field values.
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

//! Canonical JSONL serialization regression coverage.
//!
//! These tests protect the public manifest row contract from malformed
//! JSON when caller-provided field values contain reserved characters.

use game_manifest::DirCount;
use schoenwald_cli as _;
use schoenwald_filesystem as _;

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
