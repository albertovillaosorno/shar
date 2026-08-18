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

use std::fs;
use std::sync::atomic::{AtomicU64, Ordering};

use super::{
    is_cache_current, manifest_is_complete,
    manifest_normalized_source_matches_bytes, manifest_source_matches_bytes,
};
use super::super::filesystem_batch_artifact::manifest_component_files_exist;

static NEXT_CACHE_FIXTURE: AtomicU64 = AtomicU64::new(0);

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

#[test]
fn cache_source_digest_must_match_current_input_bytes() {
    let source = b"current-source";
    let digest = shar_sha256::digest_hex(source);
    let header = format!(
        r#"{{"schema":"p3d.package.v1","source_sha256":"{digest}","byte_len":24,"chunk_count":2,"component_count":1}}"#
    );
    assert!(manifest_source_matches_bytes(&header, source));
    assert!(!manifest_source_matches_bytes(&header, b"changed-source"));
    assert!(!manifest_source_matches_bytes(PACKAGE_HEADER_ONE, source));
}

#[test]
fn cache_normalized_digest_must_match_published_source_bytes() {
    let normalized = b"normalized-source";
    let digest = shar_sha256::digest_hex(normalized);
    let header = format!(
        r#"{{"schema":"p3d.package.v1","normalized_sha256":"{digest}","byte_len":24,"chunk_count":2,"component_count":1}}"#
    );
    assert!(manifest_normalized_source_matches_bytes(&header, normalized));
    assert!(!manifest_normalized_source_matches_bytes(
        &header,
        b"corrupted-source"
    ));
    assert!(!manifest_normalized_source_matches_bytes(
        PACKAGE_HEADER_ONE,
        normalized
    ));
}

#[test]
fn current_cache_requires_exact_normalized_source_artifact() -> Result<(), String> {
    let sequence = NEXT_CACHE_FIXTURE.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir().join(format!(
        "shar-p3d-cache-current-{}-{sequence}",
        std::process::id()
    ));
    let input = root.join("input.p3d");
    let output = root.join("output");
    let component = output.join("components/mesh/mesh.json");
    fs::create_dir_all(component.parent().ok_or("component has no parent")?)
        .map_err(|error| error.to_string())?;
    let raw = b"raw-source";
    let normalized = b"normalized-source";
    fs::write(&input, raw).map_err(|error| error.to_string())?;
    fs::write(output.join("source.p3d"), normalized)
        .map_err(|error| error.to_string())?;
    fs::write(&component, br#"{"name":"mesh"}"#)
        .map_err(|error| error.to_string())?;
    let header = format!(
        r#"{{"schema":"p3d.package.v1","source_sha256":"{}","normalized_sha256":"{}","byte_len":24,"chunk_count":2,"component_count":1}}"#,
        shar_sha256::digest_hex(raw),
        shar_sha256::digest_hex(normalized),
    );
    let row = r#"{"ordinal":1,"depth":1,"parent_ordinal":0,"container_ordinal":1,"name":"mesh","payload_format":"schema_json","kind":"mesh","schema_ref":"mesh","recovery_status":"decoded_schema_payload","path":"mesh/mesh.json"}"#;
    let manifest = format!("{header}\n{row}\n");
    fs::write(output.join("components.jsonl"), manifest)
        .map_err(|error| error.to_string())?;

    let manifest_text = fs::read_to_string(output.join("components.jsonl"))
        .map_err(|error| error.to_string())?;
    let structural = manifest_is_complete(&manifest_text);
    let raw_matches = manifest_source_matches_bytes(&manifest_text, raw);
    let normalized_matches =
        manifest_normalized_source_matches_bytes(&manifest_text, normalized);
    let components_exist = manifest_component_files_exist(&output, &manifest_text);
    if !is_cache_current(&output, &input) {
        drop(fs::remove_dir_all(&root));
        return Err(format!(
            "complete source-bound cache was rejected: structural={structural} raw={raw_matches} normalized={normalized_matches} components={components_exist}"
        ));
    }
    fs::write(output.join("source.p3d"), b"corrupted-source")
        .map_err(|error| error.to_string())?;
    if is_cache_current(&output, &input) {
        drop(fs::remove_dir_all(&root));
        return Err("corrupted normalized source artifact was accepted".to_owned());
    }
    fs::write(output.join("source.p3d"), normalized)
        .map_err(|error| error.to_string())?;
    fs::remove_file(output.join("source.p3d")).map_err(|error| error.to_string())?;
    let accepted_missing = is_cache_current(&output, &input);
    drop(fs::remove_dir_all(&root));
    if accepted_missing {
        return Err("missing normalized source artifact was accepted".to_owned());
    }
    Ok(())
}
