// File:
//   - filesystem_batch_cache_tests.rs
// Path:
//   - src/p3d/src/adapters/driven/filesystem_batch_cache_tests.rs
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
//   - Focused regressions for local P3D batch cache evidence.
// - Must-Not:
//   - Create persistent files, execute extraction, or publish batch reports.
// - Allows:
//   - Exercise deterministic manifests and read-only missing-path checks.
// - Split-When:
//   - Header and component-row contracts need independent fixture families.
// - Merge-When:
//   - Cache validation no longer has behavior distinct from batch export.
// - Summary:
//   - Local P3D batch cache regression coverage.
// - Description:
//   - Verifies package headers, component identities, and artifact evidence.
// - Usage:
//   - Included by filesystem_batch_cache.rs under cfg(test).
// - Defaults:
//   - Tests create no persistent local state.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Regression tests for local P3D batch cache evidence.
//!
//! These cases keep malformed headers, incomplete rows, duplicate identities,
//! and missing component artifacts from suppressing required extraction.

use std::path::Path;

use super::super::filesystem_batch_artifact::{
    cache_component_exists, manifest_component_files_exist,
};
use super::manifest_is_complete;

const PACKAGE_HEADER_ONE: &str = concat!(
    r#"{"schema":"p3d.package.v1","#,
    r#""byte_len":24,"chunk_count":2,"component_count":1}"#,
);
const PACKAGE_HEADER_TWO: &str = concat!(
    r#"{"schema":"p3d.package.v1","#,
    r#""byte_len":36,"chunk_count":3,"component_count":2}"#,
);
const PACKAGE_HEADER_MISSING_SIZE: &str = concat!(
    r#"{"schema":"p3d.package.v1","#,
    r#""chunk_count":1,"component_count":1}"#,
);
const PACKAGE_HEADER_ZERO_CHUNKS: &str = concat!(
    r#"{"schema":"p3d.package.v1","#,
    r#""byte_len":12,"chunk_count":0,"component_count":1}"#,
);
const PACKAGE_HEADER_MISSING_ROOT: &str = concat!(
    r#"{"schema":"p3d.package.v1","#,
    r#""byte_len":12,"chunk_count":1,"component_count":1}"#,
);
const PACKAGE_HEADER_SHORT_BYTES: &str = concat!(
    r#"{"schema":"p3d.package.v1","#,
    r#""byte_len":1,"chunk_count":2,"component_count":1}"#,
);
const PACKAGE_HEADER_UNDERSIZED_CHUNKS: &str = concat!(
    r#"{"schema":"p3d.package.v1","#,
    r#""byte_len":12,"chunk_count":2,"component_count":1}"#,
);

const UNKNOWN_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"unknown","schema_ref":"unknown","#,
    r#""recovery_status":"raw_schema_pending","path":"raw.bin"}"#,
);
const PENDING_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"raw_schema_pending","path":"raw.bin"}"#,
);
const EMPTY_KIND_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","path":"mesh.json"}"#,
);
const EMPTY_SCHEMA_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"","#,
    r#""recovery_status":"decoded_schema_payload","path":"mesh.json"}"#,
);
const MISMATCHED_SCHEMA_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"texture","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/mismatched-schema.json"}"#,
);
const UNSUPPORTED_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"failed","path":"mesh.json"}"#,
);
const COMPLETE_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/mesh.json"}"#,
);
const RECOVERED_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"image/png","#,
    r#""kind":"texture","schema_ref":"texture","#,
    r#""recovery_status":"recovered_embedded_image_payload","#,
    r#""path":"texture/main.png"}"#,
);
const DECODED_IMAGE_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"image/png","kind":"mesh","#,
    r#""schema_ref":"mesh","recovery_status":"decoded_schema_payload","#,
    r#""path":"components/decoded-image.json"}"#,
);
const RECOVERED_SCHEMA_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","kind":"texture","#,
    r#""schema_ref":"texture","#,
    r#""recovery_status":"recovered_embedded_image_payload","#,
    r#""path":"texture/recovered-schema.json"}"#,
);
const MISSING_PAYLOAD_FORMAT_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","kind":"mesh","#,
    r#""schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/missing-format.json"}"#,
);
const EMPTY_PAYLOAD_FORMAT_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"","kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/empty-format.json"}"#,
);
const MISSING_NAME_ROW: &str = concat!(
    r#"{"ordinal":1,"payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/missing-name.json"}"#,
);
const MISSING_ORDINAL_ROW: &str = concat!(
    r#"{"name":"value","payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/missing-ordinal.json"}"#,
);

#[test]
fn cache_manifest_rejects_incomplete_component_rows() {
    let unknown = format!(
        "{PACKAGE_HEADER_ONE}
{UNKNOWN_ROW}"
    );
    let pending = format!(
        "{PACKAGE_HEADER_ONE}
{PENDING_ROW}"
    );
    let empty_kind = format!(
        "{PACKAGE_HEADER_ONE}
{EMPTY_KIND_ROW}"
    );
    let empty_schema = format!(
        "{PACKAGE_HEADER_ONE}
{EMPTY_SCHEMA_ROW}"
    );
    let mismatched_schema = format!(
        "{PACKAGE_HEADER_ONE}
{MISMATCHED_SCHEMA_ROW}"
    );
    let unsupported = format!(
        "{PACKAGE_HEADER_ONE}
{UNSUPPORTED_ROW}"
    );
    let missing_payload_format = format!(
        "{PACKAGE_HEADER_ONE}
{MISSING_PAYLOAD_FORMAT_ROW}"
    );
    let empty_payload_format = format!(
        "{PACKAGE_HEADER_ONE}
{EMPTY_PAYLOAD_FORMAT_ROW}"
    );
    let missing_name = format!(
        "{PACKAGE_HEADER_ONE}
{MISSING_NAME_ROW}"
    );
    let missing_ordinal = format!(
        "{PACKAGE_HEADER_ONE}
{MISSING_ORDINAL_ROW}"
    );
    let unknown_valid = manifest_is_complete(unknown.as_str());
    let pending_valid = manifest_is_complete(pending.as_str());
    let empty_kind_valid = manifest_is_complete(empty_kind.as_str());
    let empty_schema_valid = manifest_is_complete(empty_schema.as_str());
    let mismatched_schema_valid =
        manifest_is_complete(mismatched_schema.as_str());
    let unsupported_valid = manifest_is_complete(unsupported.as_str());
    let missing_payload_format_valid =
        manifest_is_complete(missing_payload_format.as_str());
    let empty_payload_format_valid =
        manifest_is_complete(empty_payload_format.as_str());
    let missing_name_valid = manifest_is_complete(missing_name.as_str());
    let missing_ordinal_valid = manifest_is_complete(missing_ordinal.as_str());
    assert!(!unknown_valid);
    assert!(!pending_valid);
    assert!(!empty_kind_valid);
    assert!(!empty_schema_valid);
    assert!(!mismatched_schema_valid);
    assert!(!unsupported_valid);
    assert!(!missing_payload_format_valid);
    assert!(!empty_payload_format_valid);
    assert!(!missing_name_valid);
    assert!(!missing_ordinal_valid);
}

#[test]
fn cache_manifest_validates_header_count_and_artifacts() {
    let complete = format!(
        "{PACKAGE_HEADER_ONE}
{COMPLETE_ROW}"
    );
    let recovered = format!(
        "{PACKAGE_HEADER_ONE}
{RECOVERED_ROW}"
    );
    let truncated = format!(
        "{PACKAGE_HEADER_TWO}
{COMPLETE_ROW}"
    );
    let missing_size = format!(
        "{PACKAGE_HEADER_MISSING_SIZE}
{COMPLETE_ROW}"
    );
    let zero_chunks = format!(
        "{PACKAGE_HEADER_ZERO_CHUNKS}
{COMPLETE_ROW}"
    );
    let missing_root = format!(
        "{PACKAGE_HEADER_MISSING_ROOT}
{COMPLETE_ROW}"
    );
    let short_bytes = format!(
        "{PACKAGE_HEADER_SHORT_BYTES}
{COMPLETE_ROW}"
    );
    let undersized_chunks = format!(
        "{PACKAGE_HEADER_UNDERSIZED_CHUNKS}
{COMPLETE_ROW}"
    );
    let empty_valid = manifest_is_complete("");
    let malformed_valid = manifest_is_complete("not-json");
    let truncated_valid = manifest_is_complete(truncated.as_str());
    let missing_size_valid = manifest_is_complete(missing_size.as_str());
    let zero_chunks_valid = manifest_is_complete(zero_chunks.as_str());
    let missing_root_valid = manifest_is_complete(missing_root.as_str());
    let short_bytes_valid = manifest_is_complete(short_bytes.as_str());
    let undersized_chunks_valid =
        manifest_is_complete(undersized_chunks.as_str());
    let complete_valid = manifest_is_complete(complete.as_str());
    let recovered_valid = manifest_is_complete(recovered.as_str());
    let missing_component = cache_component_exists(
        Path::new("Cargo.toml"),
        "components/mesh.json",
    );
    let missing_manifest_component = manifest_component_files_exist(
        Path::new("Cargo.toml"),
        complete.as_str(),
    );
    assert!(!empty_valid);
    assert!(!malformed_valid);
    assert!(!truncated_valid);
    assert!(!missing_size_valid);
    assert!(!zero_chunks_valid);
    assert!(!missing_root_valid);
    assert!(!short_bytes_valid);
    assert!(!undersized_chunks_valid);
    assert!(complete_valid);
    assert!(recovered_valid);
    assert!(!missing_component);
    assert!(!missing_manifest_component);
}

#[test]
fn recovery_evidence_requires_matching_encoding() {
    let decoded_image = format!(
        "{PACKAGE_HEADER_ONE}
{DECODED_IMAGE_ROW}"
    );
    let recovered_schema = format!(
        "{PACKAGE_HEADER_ONE}
{RECOVERED_SCHEMA_ROW}"
    );
    assert!(!manifest_is_complete(decoded_image.as_str()));
    assert!(!manifest_is_complete(recovered_schema.as_str()));
}
