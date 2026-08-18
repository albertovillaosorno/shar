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
//   - Package test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Package test module.
// - Description:
//   - Implements the declared test module responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Package test module.

use p3d::adapters::driven::package::{
    ComponentOutput, component_line, kind_schema, package_header,
};
use p3d::domain::chunk::Endian;
use p3d::{ChunkKind, ChunkRecord, P3dDocument};
use schoenwald_cli as _;
use schoenwald_filesystem as _;
use serde_json as _;
use shar_json_text as _;
use shar_sha256 as _;

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
        container_ordinal: 0,
        name: String::from(name),
        path: String::from("components/value.json"),
        payload_format: String::from("json"),
        schema_ref: String::from("schema"),
        recovery_status: String::from("decoded"),
        sha256: "0".repeat(64),
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
fn component_json_preserves_chunk_ancestry() {
    let mut value = component("value");
    value.chunk.ordinal = 7;
    value.chunk.depth = 3;
    value.chunk.parent_ordinal = Some(4);
    value.container_ordinal = 1;

    let json = component_line(&value);

    assert!(json.contains(r#""ordinal":7"#));
    assert!(json.contains(r#""depth":3"#));
    assert!(json.contains(r#""parent_ordinal":4"#));
    assert!(json.contains(r#""container_ordinal":1"#));
}

#[test]
fn root_component_json_uses_null_parent_ordinal() {
    let json = component_line(&component("value"));

    assert!(json.contains(r#""parent_ordinal":null"#));
}

#[test]
fn unknown_kind_uses_unknown_schema_identity() {
    assert_eq!(kind_schema("unregistered_kind"), "unknown");
}

#[test]
fn package_header_binds_source_and_normalized_digests() {
    let document = P3dDocument {
        endian: Endian::Little,
        compression: "none",
        byte_len: 24,
        chunks: vec![component("root").chunk, component("child").chunk],
    };
    let digest = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let normalized_digest =
        "abcdef0123456789abcdef0123456789abcdef0123456789abcdef0123456789";
    let header = package_header(&document, 1, digest, normalized_digest);

    assert!(header.contains(&format!(r#""source_sha256":"{digest}""#)));
    assert!(header.contains(&format!(
        r#""normalized_sha256":"{normalized_digest}""#
    )));
}

#[test]
fn component_json_publishes_primary_artifact_digest() {
    let value = component("value");
    let json = component_line(&value);

    assert!(json.contains(&format!(r#""sha256":"{}""#, value.sha256)));
}
