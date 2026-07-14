// File:
//   - package.rs
// Path:
//   - src/p3d/tests/package.rs
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
//   - Regression coverage for public Pure3D package serialization invariants.
// - Must-Not:
//   - Access private assets or duplicate package implementation logic.
// - Allows:
//   - Synthetic component metadata and public serializer assertions.
// - Split-When:
//   - Another package artifact requires independent fixtures.
// - Merge-When:
//   - Package serialization regressions no longer need a distinct boundary.
// - Summary:
//   - Protects lossless Pure3D package metadata.
// - Description:
//   - Exercises public package JSON serialization with synthetic metadata.
// - Usage:
//   - Run through the p3d crate test target.
// - Defaults:
//   - No local files, generated assets, or external processes are required.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Regression coverage for public `Pure3D` package serialization invariants.
//!
//! Synthetic metadata proves JSON output remains lossless and deterministic.

use p3d::adapters::driven::package::{
    ComponentOutput, component_line, kind_schema,
};
use p3d::{ChunkKind, ChunkRecord};
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;

fn component(name: &str) -> ComponentOutput {
    ComponentOutput {
        chunk: ChunkRecord {
            ordinal: 0,
            depth: 0,
            parent_ordinal: None,
            id: 1,
            kind: ChunkKind::Unknown,
            offset: 0,
            header_size: 12,
            total_size: 12,
            payload_offset: 12,
            payload_size: 0,
            child_count: 0,
        },
        name: String::from(name),
        path: String::from("components/value.json"),
        payload_format: String::from("json"),
        schema_ref: String::from("schema"),
        recovery_status: String::from("decoded"),
    }
}

#[test]
fn component_json_preserves_escaped_name_characters() {
    let quote = char::from(34);
    let slash = char::from(92);
    let newline = char::from(10);
    let tab = char::from(9);
    let low = char::from(1);
    let name =
        format!("quote{quote} slash{slash} line{newline} tab{tab} low{low}");
    let json = component_line(&component(&name));
    let mut expected = format!("{quote}name{quote}:{quote}quote");
    expected.push(slash);
    expected.push(quote);
    expected.push_str(" slash");
    expected.push(slash);
    expected.push(slash);
    expected.push_str(" line");
    expected.push(slash);
    expected.push('n');
    expected.push_str(" tab");
    expected.push(slash);
    expected.push('t');
    expected.push_str(" low");
    expected.push(slash);
    expected.push_str("u0001");
    expected.push(quote);

    assert!(json.contains(&expected));
}

#[test]
fn unknown_kind_uses_unknown_schema_identity() {
    assert_eq!(
        kind_schema("unregistered_kind"),
        "unknown"
    );
}
