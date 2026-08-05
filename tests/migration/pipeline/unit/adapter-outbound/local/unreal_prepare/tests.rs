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
//   - Prepare-Unreal adapter unit tests.
// - Must-Not:
//   - Publish repository staging output or execute Unreal Editor.
// - Allows:
//   - Pure validation helpers and isolated temporary audit files.
// - Split-When:
//   - Split when publication fixtures gain independent ownership.
// - Merge-When:
//   - Merge when another module owns identical adapter evidence.
// - Summary:
//   - Prepare-Unreal adapter unit tests.
// - Description:
//   - Proves audit freshness, path safety, and rendered schema validation.
// - Usage:
//   - Included only by the owning adapter module under cfg(test).
// - Defaults:
//   - Stale or malformed evidence fails closed.
//

//! Prepare-Unreal adapter unit tests.

use std::collections::BTreeSet;
use std::fs;

use shar_sha256::digest_hex;

use super::{
    MANIFEST_FILE, PLAN_INDEX_FILE, PUBLISHED_FILES, SUMMARY_FILE,
    retain_source_ids, validate_audit, validate_relative_path,
    validate_rendered_output,
};
use crate::domain::{
    UNREAL_IMPORT_MANIFEST_SCHEMA, UNREAL_IMPORT_SUMMARY_SCHEMA,
    UnrealSourceEvidence,
};

fn source(id: &str) -> UnrealSourceEvidence {
    UnrealSourceEvidence {
        id: id.to_owned(),
        path: format!("extracted/{id}.bin"),
        file_extension: "bin".to_owned(),
        unit_type: "metadata".to_owned(),
        subtype: "none".to_owned(),
        kind: "test".to_owned(),
        function: "test".to_owned(),
        schema: "none".to_owned(),
        origin: "test".to_owned(),
        source_path: "none".to_owned(),
        source_chunk_kind: "none".to_owned(),
        size_bytes: 0,
        sha256: "0".repeat(64),
        unreal_import_relation: "none".to_owned(),
        future_normalization: "none".to_owned(),
    }
}

#[test]
fn excludes_fail_closed_source_evidence_from_import_planning() {
    let evidence = vec![source("keep"), source("error")];
    let importable = BTreeSet::from(["keep"]);
    let retained = retain_source_ids(evidence, &importable);
    assert_eq!(retained.len(), 1);
    assert_eq!(
        retained.first().map(|source| source.id.as_str()),
        Some("keep")
    );
}

fn clean_audit(manifest: &str, rows: usize) -> String {
    format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.minor-unit-audit.v2\",",
            "\"rows\":{},\"failures\":0,\"error_rows\":0,",
            "\"manifest_sha256\":\"{}\"}}"
        ),
        rows,
        digest_hex(manifest.as_bytes()),
    )
}

#[test]
fn stage_report_counts_exact_published_files() -> Result<(), String> {
    if PUBLISHED_FILES
        != [
            MANIFEST_FILE,
            SUMMARY_FILE,
            PLAN_INDEX_FILE,
            "plans/asset-import-plan.json",
            "plans/asset-construction-plan.json",
            "plans/world-assembly-plan.json",
            "plans/runtime-binding-plan.json",
            "plans/validation-plan.json",
            "plans/package-plan.json",
        ]
    {
        return Err("prepare-unreal publication inventory drifted".to_owned());
    }
    if PUBLISHED_FILES.len() != 9 {
        return Err("prepare-unreal must report exactly nine files".to_owned());
    }
    Ok(())
}

#[test]
fn accepts_clean_current_audit() -> Result<(), String> {
    let path = std::env::temp_dir()
        .join(format!("shar-unreal-audit-{}.json", std::process::id()));
    let manifest = "one\ntwo\nthree\n";
    fs::write(&path, clean_audit(manifest, 3))
        .map_err(|error| error.to_string())?;
    let result =
        validate_audit(&path, manifest).map_err(|error| error.to_string());
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    result
}

#[test]
fn rejects_stale_audit_and_escaping_paths() -> Result<(), String> {
    let path = std::env::temp_dir().join(format!(
        "shar-unreal-stale-audit-{}.json",
        std::process::id()
    ));
    let audited_manifest = "one\ntwo\n";
    fs::write(&path, clean_audit(audited_manifest, 2))
        .map_err(|error| error.to_string())?;
    let stale = validate_audit(&path, "one\ntwo\nthree\n").is_err();
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    if !stale {
        return Err("stale audit must fail".to_owned());
    }
    for unsafe_path in ["", "../escape", "a/../escape", r"a\escape"] {
        if validate_relative_path(unsafe_path).is_ok() {
            return Err(format!("unsafe path was accepted: {unsafe_path}"));
        }
    }
    Ok(())
}

#[test]
fn rejects_same_row_count_with_changed_manifest_content() -> Result<(), String>
{
    let path = std::env::temp_dir().join(format!(
        "shar-unreal-hash-audit-{}.json",
        std::process::id()
    ));
    let audited_manifest = "one\ntwo\n";
    fs::write(&path, clean_audit(audited_manifest, 2))
        .map_err(|error| error.to_string())?;
    let result = validate_audit(&path, "one\nchanged\n");
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("same-row manifest mutation must fail".to_owned());
    };
    if !error.to_string().contains("SHA-256 changed") {
        return Err(format!("unexpected stale-audit failure: {error}"));
    }
    Ok(())
}

fn rendered_fixture(
    package_count: u64,
    source_package: &str,
) -> (String, String) {
    let manifest = format!(
        concat!(
            "{{\"schema\":\"{}\",\"record_type\":\"header\",",
            "\"package_count\":{},\"source_count\":1,",
            "\"direct_import_count\":1,\"requires_fbx_count\":0,",
            "\"requires_editor_factory_count\":0,",
            "\"metadata_only_count\":0}}\n",
            "{{\"schema\":\"{}\",\"record_type\":\"package\",",
            "\"package_id\":\"pkg\",",
            "\"disposition\":\"direct-editor-import\",",
            "\"source_count\":1}}\n",
            "{{\"schema\":\"{}\",\"record_type\":\"source\",",
            "\"package_id\":\"{}\",\"id\":\"src\",",
            "\"direct_import\":{{}}}}\n"
        ),
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        package_count,
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        source_package,
    );
    let summary = format!(
        concat!(
            "{{\"schema\":\"{}\",\"packages\":{},",
            "\"sources\":1,\"direct_imports\":1,",
            "\"requires_fbx\":0,\"requires_editor_factory\":0,",
            "\"metadata_only\":0}}\n"
        ),
        UNREAL_IMPORT_SUMMARY_SCHEMA, package_count,
    );
    (manifest, summary)
}

#[test]
fn accepts_canonical_rendered_schemas_and_counts() -> Result<(), String> {
    let (manifest, summary) = rendered_fixture(1, "pkg");
    validate_rendered_output(&manifest, &summary)
        .map_err(|error| error.to_string())
}

#[test]
fn rejects_rendered_count_mismatch() -> Result<(), String> {
    let (manifest, summary) = rendered_fixture(2, "pkg");
    let result = validate_rendered_output(&manifest, &summary);
    let Err(error) = result else {
        return Err("rendered package-count mismatch must fail".to_owned());
    };
    if !error.to_string().contains("counts disagree") {
        return Err(format!("unexpected count failure: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_rendered_source_for_unknown_package() -> Result<(), String> {
    let (manifest, summary) = rendered_fixture(1, "missing");
    let result = validate_rendered_output(&manifest, &summary);
    let Err(error) = result else {
        return Err("unknown source package must fail".to_owned());
    };
    if !error.to_string().contains("undeclared package") {
        return Err(format!("unexpected package failure: {error}"));
    }
    Ok(())
}
