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
//   - Filesystem batch identity tests test module.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Filesystem batch identity tests test module.
// - Description:
//   - Implements the declared test module responsibility for p3d.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Filesystem batch identity tests test module.

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
    r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"container_ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/mesh.json"}"#,
);
const DECODED_PATH_MISMATCH_ROW: &str = concat!(
    r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"container_ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/mesh.png"}"#,
);
const RECOVERED_PATH_MISMATCH_ROW: &str = concat!(
    r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"container_ordinal":1,"name":"value","#,
    r#""payload_format":"image/png","#,
    r#""kind":"texture","schema_ref":"texture","#,
    r#""recovery_status":"recovered_embedded_image_payload","#,
    r#""path":"texture/main.jpg"}"#,
);
const OUT_OF_RANGE_ORDINAL_ROW: &str = concat!(
    r#"{"ordinal":2,"depth":1,"parent_ordinal":0,"container_ordinal":2,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/out-of-range.json"}"#,
);
const DUPLICATE_PATH_ROW: &str = concat!(
    r#"{"ordinal":2,"depth":1,"parent_ordinal":0,"container_ordinal":2,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"texture","schema_ref":"texture","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/mesh.json"}"#,
);
const CASE_EQUIVALENT_PATH_ROW: &str = concat!(
    r#"{"ordinal":2,"depth":1,"parent_ordinal":0,"container_ordinal":2,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"texture","schema_ref":"texture","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"COMPONENTS/MESH.json"}"#,
);
const MISSING_ANCESTRY_ROW: &str = concat!(
    r#"{"ordinal":1,"name":"value","#,
    r#""payload_format":"schema_json","#,
    r#""kind":"mesh","schema_ref":"mesh","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/missing-ancestry.json"}"#,
);
const PARENT_DEPTH_MISMATCH_ROW: &str = concat!(
    r#"{"ordinal":2,"depth":3,"parent_ordinal":1,"container_ordinal":1,"#,
    r#""name":"value","payload_format":"schema_json","#,
    r#""kind":"texture","schema_ref":"texture","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/depth.json"}"#,
);
const PARENT_CONTAINER_MISMATCH_ROW: &str = concat!(
    r#"{"ordinal":2,"depth":2,"parent_ordinal":1,"container_ordinal":2,"#,
    r#""name":"value","payload_format":"schema_json","#,
    r#""kind":"texture","schema_ref":"texture","#,
    r#""recovery_status":"decoded_schema_payload","#,
    r#""path":"components/container.json"}"#,
);
const DUPLICATE_ORDINAL_ROW: &str = concat!(
    r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"container_ordinal":1,"name":"value","#,
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

#[test]
fn rejects_case_equivalent_component_paths() {
    let duplicate = format!(
        "{PACKAGE_HEADER_TWO}\n{COMPLETE_ROW}\n{CASE_EQUIVALENT_PATH_ROW}"
    );
    assert!(!manifest_is_complete(duplicate.as_str()));
}

#[test]
fn rejects_missing_component_ancestry() {
    let missing = format!("{PACKAGE_HEADER_ONE}
{MISSING_ANCESTRY_ROW}");
    assert!(!manifest_is_complete(missing.as_str()));
}

#[test]
fn rejects_published_parent_relationship_mismatches() {
    let depth = format!(
        "{PACKAGE_HEADER_TWO}
{COMPLETE_ROW}
{PARENT_DEPTH_MISMATCH_ROW}"
    );
    let container = format!(
        "{PACKAGE_HEADER_TWO}
{COMPLETE_ROW}
{PARENT_CONTAINER_MISMATCH_ROW}"
    );
    assert!(!manifest_is_complete(depth.as_str()));
    assert!(!manifest_is_complete(container.as_str()));
}
