// File:
//   - filesystem_batch_identity_tests.rs
// Path:
//   - src/p3d/src/adapters/driven/filesystem_batch_identity_tests.rs
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
//   - Regression coverage for cached P3D component identity uniqueness.
// - Must-Not:
//   - Inspect local storage or duplicate package-header parsing behavior.
// - Allows:
//   - Construct deterministic component rows with conflicting identities.
// - Split-When:
//   - Another independent identity dimension requires a dedicated fixture.
// - Merge-When:
//   - Component identity no longer has behavior distinct from cache evidence.
// - Summary:
//   - Cached P3D component identity regressions.
// - Description:
//   - Verifies ordinal range, ordinal uniqueness, and artifact path uniqueness.
// - Usage:
//   - Included by filesystem_batch_cache.rs under cfg(test).
// - Defaults:
//   - Tests are deterministic and filesystem-free.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! Regression tests for cached P3D component identities.
//!
//! Component ordinals must belong to the package chunk table, and both ordinal
//! and artifact path identities must remain unique across manifest rows.

use super::manifest_is_complete;

const PACKAGE_HEADER_ONE: &str = concat!(
    r#"{"schema":"p3d.package.v1","#,
    r#""byte_len":24,"chunk_count":2,"component_count":1}"#,
);
const PACKAGE_HEADER_TWO: &str = concat!(
    r#"{"schema":"p3d.package.v1","#,
    r#""byte_len":36,"chunk_count":3,"component_count":2}"#,
);
const COMPLETE_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/mesh.json"}"#,
);
const DECODED_PATH_MISMATCH_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/mesh.png"}"#,
);
const RECOVERED_PATH_MISMATCH_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"image/png","#,
    r#""kind":"texture","schema_ref":"texture","#,
    r#""recovery_status":"recovered_embedded_image_payload","#,
    r#""path":"texture/main.jpg"}"#,
);
const OUT_OF_RANGE_ORDINAL_ROW: &str = concat!(
    r#"{"ordinal":2,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/out-of-range.json"}"#,
);
const DUPLICATE_PATH_ROW: &str = concat!(
    r#"{"ordinal":2,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"texture","schema_ref":"texture","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/mesh.json"}"#,
);
const DUPLICATE_ORDINAL_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"texture","schema_ref":"texture","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/texture.json"}"#,
);

#[test]
fn artifact_paths_match_declared_encodings() {
    let decoded_mismatch = format!(
        "{PACKAGE_HEADER_ONE}
{DECODED_PATH_MISMATCH_ROW}"
    );
    let recovered_mismatch = format!(
        "{PACKAGE_HEADER_ONE}
{RECOVERED_PATH_MISMATCH_ROW}"
    );
    assert!(!manifest_is_complete(decoded_mismatch.as_str()));
    assert!(!manifest_is_complete(recovered_mismatch.as_str()));
}

#[test]
fn rejects_out_of_range_ordinals() {
    let out_of_range = format!(
        "{PACKAGE_HEADER_ONE}
{OUT_OF_RANGE_ORDINAL_ROW}"
    );
    assert!(!manifest_is_complete(out_of_range.as_str()));
}

#[test]
fn rejects_duplicate_component_paths() {
    let duplicate = format!(
        "{PACKAGE_HEADER_TWO}
{COMPLETE_ROW}
{DUPLICATE_PATH_ROW}"
    );
    assert!(!manifest_is_complete(duplicate.as_str()));
}

#[test]
fn rejects_duplicate_component_ordinals() {
    let duplicate = format!(
        "{PACKAGE_HEADER_TWO}
{COMPLETE_ROW}
{DUPLICATE_ORDINAL_ROW}"
    );
    assert!(!manifest_is_complete(duplicate.as_str()));
}
