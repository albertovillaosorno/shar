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

// cspell:ignore selectmission closemission addstage closestage addobjective
// cspell:ignore closeobjective addcondition closecondition
//! Prepare-Unreal adapter unit tests.

use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use serde_json::json;
use shar_sha256::digest_hex;

use super::{
    MANIFEST_FILE, PLAN_INDEX_FILE, PUBLISHED_FILES, SUMMARY_FILE,
    SourceEvidenceInput, ensure_generated_directory, open_stable_source,
    parallel_source_evidence, prepare_io_error, publication_error, read_utf8,
    retain_source_ids, source_worker_count_for, stream_source_digest,
    validate_audit, verify_stable_source,
    validate_generated_chain,
    validate_normalized_mission_source, validate_public_identifier,
    validate_publication_inventory, validate_relative_path,
    validate_rendered_output,
};
use crate::domain::{
    MISSION_SCRIPT_SCHEMA, UNREAL_IMPORT_MANIFEST_SCHEMA,
    UNREAL_IMPORT_SUMMARY_SCHEMA, UnrealSourceEvidence,
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
fn parallel_source_verification_reports_first_manifest_error() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".temp")
        .join(format!("unreal-source-errors-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let first = root.join("first.bin");
    fs::write(&first, b"first").map_err(|error| error.to_string())?;
    let input = |id: &str, path: PathBuf, size: u64| SourceEvidenceInput {
        id: id.to_owned(),
        path: format!("extracted/{id}.bin"),
        resolved: path,
        expected_size: size,
        file_extension: "bin".to_owned(),
        unit_type: "metadata".to_owned(),
        subtype: "none".to_owned(),
        kind: "test".to_owned(),
        function: "test".to_owned(),
        schema: "none".to_owned(),
        origin: "test".to_owned(),
        source_path: "none".to_owned(),
        source_chunk_kind: "none".to_owned(),
        unreal_import_relation: "none".to_owned(),
        future_normalization: "none".to_owned(),
    };
    let inputs = vec![
        input("first", first, 999),
        input("second", root.join("missing.bin"), 1),
    ];
    let result = parallel_source_evidence(&inputs);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("parallel verification unexpectedly accepted invalid rows".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.starts_with("source size changed for extracted/first.bin:") {
        return Err(format!("parallel error priority changed: {rendered}"));
    }
    Ok(())
}

#[test]
fn stable_source_verification_rejects_path_replacement() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".temp")
        .join(format!("unreal-source-replacement-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let path = root.join("source.bin");
    let moved = root.join("opened.bin");
    fs::write(&path, b"original-source").map_err(|error| error.to_string())?;
    let (file, identity) = open_stable_source(&path)
        .map_err(|error| error.to_string())?;
    fs::rename(&path, &moved).map_err(|error| error.to_string())?;
    fs::write(&path, b"replacement-src").map_err(|error| error.to_string())?;
    let result = verify_stable_source(&path, &file, &identity);
    drop(file);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("path replacement preserved a stale source identity".to_owned());
    };
    if !error.to_string().contains("identity changed") {
        return Err(format!("unexpected replacement error: {error}"));
    }
    Ok(())
}

#[test]
fn streamed_source_digest_matches_one_shot_across_io_blocks() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".temp")
        .join(format!("unreal-stream-hash-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let path = root.join("payload.bin");
    let payload = vec![0x5a_u8; 1_048_593];
    fs::write(&path, &payload).map_err(|error| error.to_string())?;
    let result = stream_source_digest(&path).map_err(|error| error.to_string());
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let (size, digest) = result?;
    if size != 1_048_593 || digest != digest_hex(&payload) {
        return Err("streamed source digest changed across I/O blocks".to_owned());
    }
    Ok(())
}

#[test]
fn source_worker_count_is_bounded_and_never_zero() {
    assert_eq!(source_worker_count_for(1, 100), 1);
    assert_eq!(source_worker_count_for(3, 100), 2);
    assert_eq!(source_worker_count_for(24, 100), 8);
    assert_eq!(source_worker_count_for(24, 3), 3);
    assert_eq!(source_worker_count_for(24, 0), 1);
}

#[test]
fn parallel_source_verification_preserves_manifest_order() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".temp")
        .join(format!("unreal-source-order-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let first = root.join("first.bin");
    let second = root.join("second.bin");
    fs::write(&first, b"first").map_err(|error| error.to_string())?;
    fs::write(&second, b"second-source").map_err(|error| error.to_string())?;
    let input = |id: &str, path: PathBuf, size: u64| SourceEvidenceInput {
        id: id.to_owned(),
        path: format!("extracted/{id}.bin"),
        resolved: path,
        expected_size: size,
        file_extension: "bin".to_owned(),
        unit_type: "metadata".to_owned(),
        subtype: "none".to_owned(),
        kind: "test".to_owned(),
        function: "test".to_owned(),
        schema: "none".to_owned(),
        origin: "test".to_owned(),
        source_path: "none".to_owned(),
        source_chunk_kind: "none".to_owned(),
        unreal_import_relation: "none".to_owned(),
        future_normalization: "none".to_owned(),
    };
    let inputs = vec![input("first", first, 5), input("second", second, 13)];
    let result = parallel_source_evidence(&inputs).map_err(|error| error.to_string());
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let evidence = result?;
    let ids = evidence
        .iter()
        .map(|source| source.id.as_str())
        .collect::<Vec<_>>();
    if ids != ["first", "second"] {
        return Err(format!("parallel source order changed: {ids:?}"));
    }
    if evidence.first().is_none_or(|source| source.sha256 != digest_hex(b"first"))
        || evidence
            .get(1)
            .is_none_or(|source| source.sha256 != digest_hex(b"second-source"))
    {
        return Err("parallel source hashing changed physical evidence".to_owned());
    }
    Ok(())
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

fn clean_mission_json(with_finding: bool) -> Result<Vec<u8>, String> {
    let findings = if with_finding {
        json!([{
            "ordinal": 3,
            "command": "closecondition",
            "code": "condition-close-without-open-condition"
        }])
    } else {
        json!([])
    };
    let finding_count = usize::from(with_finding);
    serde_json::to_vec(&json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension": "mfk",
        "route_class": "mission",
        "source_bytes": 64,
        "context_command_count": 2,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": finding_count,
        "context_findings": findings,
        "statement_count": 2,
        "unique_command_count": 2,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 2,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {"closemission": 1, "selectmission": 1},
        "source_statements": [
            "SelectMission(\"m1\");",
            "CloseMission();"
        ],
        "p3d_references": [],
        "command_invocations": [
            {
                "ordinal": 1,
                "name": "selectmission",
                "args_raw": "\"m1\"",
                "semantic_role": "mission-script",
                "arguments": ["m1"]
            },
            {
                "ordinal": 2,
                "name": "closemission",
                "args_raw": "",
                "semantic_role": "mission-script",
                "arguments": []
            }
        ]
    }))
    .map_err(|error| error.to_string())
}

#[test]
fn mission_semantic_gate_accepts_clean_v3_and_bypasses_other_kinds()
-> Result<(), String> {
    let clean = clean_mission_json(false)?;
    validate_normalized_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &clean,
    )
    .map_err(|error| error.to_string())?;
    validate_normalized_mission_source(
        "texture",
        "unrelated-schema",
        "bin",
        "test",
        b"not-json",
    )
    .map_err(|error| error.to_string())
}

#[test]
fn mission_semantic_gate_rejects_stale_schema_and_context_findings()
-> Result<(), String> {
    let clean = clean_mission_json(false)?;
    let stale = validate_normalized_mission_source(
        "mission-script",
        "shar-schoenwald.straggler.mission-script.v2",
        "json",
        "game-straggler-normalize",
        &clean,
    );
    let Err(stale) = stale else {
        return Err("stale normalized mission schema was accepted".to_owned());
    };
    if !stale.to_string().contains("schema is stale") {
        return Err(format!("unexpected stale schema failure: {stale}"));
    }

    let finding = clean_mission_json(true)?;
    let finding = validate_normalized_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &finding,
    );
    let Err(finding) = finding else {
        return Err(
            "mission context finding reached Unreal planning".to_owned()
        );
    };
    if !finding.to_string().contains("must be resolved") {
        return Err(format!("unexpected finding failure: {finding}"));
    }
    Ok(())
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
    for unsafe_path in [
        "",
        "../escape",
        "a/../escape",
        r"a\escape",
        "artifact.json:stream",
        "artifact\n.json",
        "artifact\0.json",
    ] {
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
            "\"requires_semantic_conversion_count\":0,",
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
            "\"requires_semantic_conversion\":0,",
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

#[test]
fn prepare_io_diagnostics_hide_paths_and_raw_error_text() -> Result<(), String>
{
    let private_fragment = "private-workstation-unreal-prepare";
    let error = std::io::Error::other(private_fragment);
    let rendered =
        prepare_io_error("read Unreal source evidence", &error).to_string();
    if rendered.contains(private_fragment)
        || rendered != "read Unreal source evidence failed (Other)"
    {
        return Err(format!("prepare diagnostic leaked: {rendered}"));
    }

    let missing = std::env::temp_dir()
        .join(private_fragment)
        .join("missing-manifest.jsonl");
    let Err(read_error) = read_utf8(&missing, "read minor-unit manifest")
    else {
        return Err("missing manifest was accepted".to_owned());
    };
    let read_error = read_error.to_string();
    if read_error.contains(private_fragment)
        || read_error != "read minor-unit manifest failed (NotFound)"
    {
        return Err(format!("manifest diagnostic leaked: {read_error}"));
    }
    Ok(())
}

#[test]
fn relative_path_diagnostics_do_not_echo_rejected_values() -> Result<(), String>
{
    let private_path = "private-workstation/../escape";
    let Err(error) = validate_relative_path(private_path) else {
        return Err("escaping path was accepted".to_owned());
    };
    let error = error.to_string();
    if error.contains(private_path)
        || error != "unsafe minor-unit relative path"
    {
        return Err(format!("relative-path diagnostic leaked: {error}"));
    }
    Ok(())
}

#[test]
fn publication_failure_reports_failed_rollback_without_raw_text()
-> Result<(), String> {
    let publish = std::io::Error::new(
        std::io::ErrorKind::PermissionDenied,
        "private-publish-path",
    );
    let rollback = std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        "private-backup-path",
    );
    let rendered = publication_error(&publish, Some(&rollback)).to_string();
    let expected = concat!(
        "publish Unreal staging root failed (PermissionDenied); ",
        "restore previous Unreal staging root failed (AlreadyExists)"
    );
    if rendered != expected
        || rendered.contains("private-publish-path")
        || rendered.contains("private-backup-path")
    {
        return Err(format!("rollback diagnostic is unsafe: {rendered}"));
    }
    Ok(())
}

#[test]
fn generated_transaction_chain_rejects_non_directory_ancestors()
-> Result<(), String> {
    let private_fragment =
        format!("private-unreal-chain-{}", std::process::id());
    let root = std::env::temp_dir().join(&private_fragment);
    let directory = root.join("directory");
    let file = root.join("not-a-directory");
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    fs::write(&file, b"not a directory").map_err(|error| error.to_string())?;
    let result = validate_generated_chain(&[
        root.as_path(),
        directory.as_path(),
        file.as_path(),
    ]);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err(
            "non-directory transaction ancestor was accepted".to_owned()
        );
    };
    let rendered = error.to_string();
    if rendered.contains(&private_fragment)
        || rendered != "generated staging path is not a regular directory"
    {
        return Err(format!("transaction-chain diagnostic leaked: {rendered}"));
    }
    Ok(())
}

#[test]
fn generated_directory_creation_stops_at_unsafe_parent() -> Result<(), String> {
    let private_fragment =
        format!("private-unreal-create-{}", std::process::id());
    let root = std::env::temp_dir().join(&private_fragment);
    let blocked = root.join("blocked");
    let child = blocked.join("child");
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    fs::write(&blocked, b"not a directory")
        .map_err(|error| error.to_string())?;
    let result = ensure_generated_directory(&blocked, "create generated child");
    let child_exists = child.exists();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("unsafe parent was accepted".to_owned());
    };
    let rendered = error.to_string();
    if child_exists
        || rendered.contains(&private_fragment)
        || rendered != "generated staging path is not a regular directory"
    {
        return Err(format!("unsafe parent handling failed: {rendered}"));
    }
    Ok(())
}

#[test]
fn rejects_rendered_package_with_unknown_disposition() -> Result<(), String> {
    let private_disposition = "C:/private/import-policy";
    let (manifest, summary) = rendered_fixture(1, "pkg");
    let manifest =
        manifest.replace("direct-editor-import", private_disposition);
    let Err(error) = validate_rendered_output(&manifest, &summary) else {
        return Err("unknown package disposition was accepted".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains("unsupported disposition")
        || rendered.contains(private_disposition)
    {
        return Err(format!("unexpected disposition failure: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_rendered_source_with_non_object_direct_import() -> Result<(), String>
{
    let (manifest, summary) = rendered_fixture(1, "pkg");
    let manifest = manifest
        .replace("\"direct_import\":{}", "\"direct_import\":\"invalid\"");
    let Err(error) = validate_rendered_output(&manifest, &summary) else {
        return Err("non-object direct-import contract was accepted".to_owned());
    };
    if !error.to_string().contains("non-object direct_import") {
        return Err(format!("unexpected direct-import failure: {error}"));
    }
    Ok(())
}

#[test]
fn rejects_noncanonical_rendered_ids_without_echo() -> Result<(), String> {
    let private_id = "C:/private/package";
    let (manifest, summary) = rendered_fixture(1, "pkg");
    let manifest = manifest.replace(
        "\"package_id\":\"pkg\"",
        &format!("\"package_id\":\"{private_id}\""),
    );
    let Err(error) = validate_rendered_output(&manifest, &summary) else {
        return Err("path-shaped rendered package id was accepted".to_owned());
    };
    let rendered = error.to_string();
    if rendered.contains(private_id)
        || !rendered.contains("package id is not canonical")
    {
        return Err(format!("rendered-id diagnostic leaked: {rendered}"));
    }
    Ok(())
}

#[test]
fn rejects_rendered_packages_without_sources() -> Result<(), String> {
    let (manifest, summary) = rendered_fixture(1, "pkg");
    let mut rewritten = manifest
        .lines()
        .map(|line| {
            if line.contains("\"record_type\":\"package\"") {
                line.replace("\"source_count\":1", "\"source_count\":0")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    rewritten.push('\n');
    let manifest = rewritten;
    let Err(error) = validate_rendered_output(&manifest, &summary) else {
        return Err("source-less rendered package was accepted".to_owned());
    };
    if !error.to_string().contains("declares no source") {
        return Err(format!("unexpected empty-package failure: {error}"));
    }
    Ok(())
}

#[test]
fn publication_inventory_requires_every_declared_file() -> Result<(), String> {
    let exact = PUBLISHED_FILES
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    validate_publication_inventory(&exact)
        .map_err(|error| error.to_string())?;

    let mut missing = exact;
    let _removed = missing.remove("plans/package-plan.json");
    if validate_publication_inventory(&missing).is_ok() {
        return Err("incomplete publication inventory was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn publication_inventory_rejects_undeclared_files() -> Result<(), String> {
    let mut paths = PUBLISHED_FILES
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    let _inserted = paths.insert("plans/undeclared.json".to_owned());
    let Err(error) = validate_publication_inventory(&paths) else {
        return Err("extra publication file was accepted".to_owned());
    };
    if error.to_string() != "Unreal staging publication inventory is not exact"
    {
        return Err(format!("unexpected inventory failure: {error}"));
    }
    Ok(())
}

#[test]
fn public_identifier_guard_rejects_path_shaped_values() -> Result<(), String> {
    let private_id = "C:/private/minor-unit";
    let Err(error) =
        validate_public_identifier(private_id, "minor-unit source id")
    else {
        return Err("path-shaped minor-unit id was accepted".to_owned());
    };
    let rendered = error.to_string();
    if rendered.contains(private_id)
        || rendered != "rendered Unreal minor-unit source id is not canonical"
    {
        return Err(format!("minor-unit id diagnostic leaked: {rendered}"));
    }
    Ok(())
}

#[test]
fn audit_schema_failure_does_not_echo_rejected_value() -> Result<(), String> {
    let private_schema = "C:/private/audit-schema";
    let manifest = "one\n";
    let path = std::env::temp_dir().join(format!(
        "shar-unreal-schema-audit-{}.json",
        std::process::id()
    ));
    let audit = clean_audit(manifest, 1)
        .replace("shar-schoenwald.minor-unit-audit.v2", private_schema);
    fs::write(&path, audit).map_err(|error| error.to_string())?;
    let result = validate_audit(&path, manifest);
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("unsupported audit schema was accepted".to_owned());
    };
    let rendered = error.to_string();
    if rendered.contains(private_schema)
        || rendered != "minor-unit audit schema is not supported"
    {
        return Err(format!("audit-schema diagnostic leaked: {rendered}"));
    }
    Ok(())
}

#[test]
fn rendered_record_type_failure_does_not_echo_rejected_value()
-> Result<(), String> {
    let private_type = "C:/private/record-type";
    let (manifest, summary) = rendered_fixture(1, "pkg");
    let manifest = manifest.replacen(
        "\"record_type\":\"package\"",
        &format!("\"record_type\":\"{private_type}\""),
        1,
    );
    let Err(error) = validate_rendered_output(&manifest, &summary) else {
        return Err("unsupported record type was accepted".to_owned());
    };
    let rendered = error.to_string();
    if rendered.contains(private_type)
        || !rendered.contains("unsupported record type")
    {
        return Err(format!("record-type diagnostic leaked: {rendered}"));
    }
    Ok(())
}
