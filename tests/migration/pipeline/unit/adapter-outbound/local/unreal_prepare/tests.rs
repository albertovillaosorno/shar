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
use std::path::PathBuf;

use serde_json::json;
use shar_sha256::digest_hex;

use super::{
    MISSION_DEFINITIONS_FILE, PLAN_INDEX_FILE, PUBLISHED_FILE_COUNT,
    PUBLISHED_FILES, SUMMARY_FILE, SourceEvidenceInput,
        ensure_generated_directory,
        open_stable_source,
        parallel_source_evidence,
        prepare_io_error,
    publication_error, read_utf8, restore_previous_publication,
    retain_source_ids, source_worker_count_for, stream_source_digest,
    validate_audit, validate_generated_chain,
    validate_mission_definition_bundle, validate_public_identifier,
    validate_publication_inventory, validate_relative_path,
    validate_rendered_output,
    verify_stable_source,
};
use crate::domain::{
    MISSION_SCRIPT_SCHEMA, MissionP3dReferenceCatalog, MissionReferenceCatalog,
    PipelineOutcome,
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        UNREAL_IMPORT_SUMMARY_SCHEMA,
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

trait RejectionResult<T, E> {
    fn rejection(self, message: &str) -> Result<E, String>;
}

impl<T, E> RejectionResult<T, E> for Result<T, E> {
    fn rejection(self, message: &str) -> Result<E, String> {
        match self {
            Err(error) => Ok(error),
            Ok(_) => Err(message.to_owned()),
        }
    }
}

fn mission_source(id: &str) -> UnrealSourceEvidence {
    let mut value = source(id);
    value.kind = "mission-script".to_owned();
    value
}

fn mission_definition_row(source_id: &str, mission_id: &str) -> String {
    mission_definition_row_with_stages(
        source_id,
        mission_id,
        vec![json!({
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )
}

fn first_array_object_mut<'a>(
    value: &'a mut serde_json::Value,
    field: &str,
) -> Result<&'a mut serde_json::Map<String, serde_json::Value>, String> {
    value
        .get_mut(field)
        .and_then(serde_json::Value::as_array_mut)
        .and_then(|entries| entries.first_mut())
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| format!("mission fixture field is malformed: {field}"))
}

fn mission_definition_relationship_stage() -> serde_json::Value {
    json!({
        "collectible_waypoints": [{
            "collectible_index": 0,
            "collectible_locator_id": "collectible",
            "collectible_source_ordinal": 3,
            "objective_source_ordinal": 2,
            "source_ordinal": 5,
            "stage_sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "waypoint_index": 0,
            "waypoint_locator_id": "route",
            "waypoint_source_ordinal": 4,
        }],
        "explicit_final": false,
        "kind": {
            "final_stage": false,
            "kind": "standard",
            "legacy_flags": 0,
        },
        "next_authored_sequence_ordinal": null,
        "objective": {
            "canonical_kind": "travel",
            "source_alias": "goto",
            "source_ordinal": 2,
            "unavailable_code": null,
        },
        "objective_npc_waypoints": [{
            "declaration_source_ordinal": 6,
            "npc_id": "homer",
            "npc_locator_id": "npc",
            "objective_source_ordinal": 2,
            "source_ordinal": 7,
            "stage_sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "waypoint_locator_id": "walk",
        }],
        "pickup_state_props": [{
            "declaration_scope": {
                "kind": "stage",
                "sequence_ordinal": 0,
                "source_ordinal": 1,
            },
            "declaration_source_ordinal": 8,
            "locator_id": "pickup",
            "objective_source_ordinal": 2,
            "source_state": 2,
            "stage_sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "target_id": "prop",
            "target_source_ordinal": 9,
        }],
        "sequence_ordinal": 0,
        "stage_source_ordinal": 1,
        "terminal": "none",
    })
}

fn mission_definition_row_with_stages(
    source_id: &str,
    mission_id: &str,
    mut stages: Vec<serde_json::Value>,
) -> String {
    for stage in &mut stages {
        if let Some(stage) = stage.as_object_mut() {
            let _ = stage
                .entry("conditions".to_owned())
                .or_insert_with(|| json!([]));
            let _ = stage
                .entry("checkpoint_source_ordinal".to_owned())
                .or_insert_with(|| json!(null));
            let _ = stage
                .entry("countdown".to_owned())
                .or_insert_with(|| json!(null));
            for field in [
                "collectible_waypoints",
                "objective_npc_waypoints",
                "pickup_state_props",
            ] {
                let _ = stage
                    .entry(field.to_owned())
                    .or_insert_with(|| json!([]));
            }
        }
    }
    let mut value = json!({
        "mission_id": mission_id,
        "schema": "shar-schoenwald.mission-definition-core.v3",
        "source_id": source_id,
        "stages": stages,
    })
    .to_string();
    value.push(char::from(10));
    value
}

#[test]
// jig-ignore-next-line: long identifier
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
    let result = parallel_source_evidence(
        &inputs,
        &MissionReferenceCatalog::empty_for_tests(),
        &MissionP3dReferenceCatalog::empty_for_tests(),
    );
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        // jig-ignore-next-line: literal
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
        // jig-ignore-next-line: expression
        let (file, identity) = open_stable_source(&path).map_err(|error| error.to_string())?;
    fs::rename(&path, &moved).map_err(|error| error.to_string())?;
    fs::write(&path, b"replacement-src").map_err(|error| error.to_string())?;
    let result = verify_stable_source(&path, &file, &identity);
    drop(file);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("path replacement preserved a stale source identity".to_owned());
    };
    if !error.to_string().contains("identity changed") {
        return Err(format!("unexpected replacement error: {error}"));
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
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
        // jig-ignore-next-line: literal
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
// jig-ignore-next-line: long identifier
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
    let result = parallel_source_evidence(
        &inputs,
        &MissionReferenceCatalog::empty_for_tests(),
        &MissionP3dReferenceCatalog::empty_for_tests(),
    )
    .map_err(|error| error.to_string());
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let report = result?;
    assert!(report.mission_definitions.is_empty());
    let ids = report
        .evidence
        .iter()
        .map(|source| source.id.as_str())
        .collect::<Vec<_>>();
    if ids != ["first", "second"] {
        return Err(format!("parallel source order changed: {ids:?}"));
    }
    if report
        .evidence
        .first()
        .is_none_or(|source| source.sha256 != digest_hex(b"first"))
        || report
            .evidence
            .get(1)
            .is_none_or(|source| source.sha256 != digest_hex(b"second-source"))
    {
        // jig-ignore-next-line: literal
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

fn validate_test_mission_source(
    kind: &str,
    schema: &str,
    file_extension: &str,
    origin: &str,
    bytes: &[u8],
) -> PipelineOutcome<()> {
    super::validate_normalized_mission_source(
        "script-test-source",
        kind,
        schema,
        file_extension,
        origin,
        bytes,
        &MissionReferenceCatalog::empty_for_tests(),
        &MissionP3dReferenceCatalog::empty_for_tests(),
    )
    .map(drop)
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
        "source_bytes": 96,
        "context_command_count": 6,
        "context_adaptation_count": 0,
        "context_adaptations": [],
        "context_finding_count": finding_count,
        "context_findings": findings,
        "statement_count": 6,
        "unique_command_count": 6,
        "load_p3d_reference_count": 0,
        "mission_flow_command_count": 6,
        "vehicle_physics_command_count": 0,
        "semantic_family": "mission-script",
        "command_counts": {
            "addobjective": 1,
            "addstage": 1,
            "closemission": 1,
            "closeobjective": 1,
            "closestage": 1,
            "selectmission": 1
        },
        "source_statements": [
            "SelectMission(\"m1\");",
            "AddStage(0);",
            "AddObjective(\"goto\");",
            "CloseObjective();",
            "CloseStage();",
            "CloseMission();"
        ],
        "p3d_references": [],
        "command_invocations": [
                        {
                            "ordinal":1,
                            "name":"selectmission",
                            "args_raw":"\"m1\"",
                            "semantic_role":"mission-script",
                            "arguments":["m1"]
                        },
                        {
                            "ordinal":2,
                            "name":"addstage",
                            "args_raw":"0",
                            "semantic_role":"mission-stage",
                            "arguments":["0"]
                        },
                        {
                            "ordinal":3,
                            "name":"addobjective",
                            "args_raw":"\"goto\"",
                            "semantic_role":"mission-objective",
                            "arguments":["goto"]
                        },
                        {
                            "ordinal":4,
                            "name":"closeobjective",
                            "args_raw":"",
                            "semantic_role":"mission-objective",
                            "arguments":[]
                        },
                        {
                            "ordinal":5,
                            "name":"closestage",
                            "args_raw":"",
                            "semantic_role":"mission-stage",
                            "arguments":[]
                        },
                        {
                            "ordinal":6,
                            "name":"closemission",
                            "args_raw":"",
                            "semantic_role":"mission-script",
                            "arguments":[]
                        }
        ]
    }))
    .map_err(|error| error.to_string())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_accepts_clean_v3_and_bypasses_other_kinds() -> Result<(), String> {
    let clean = clean_mission_json(false)?;
    validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &clean,
    )
    .map_err(|error| error.to_string())?;
    // jig-ignore-next-line: literal
    validate_test_mission_source("texture", "unrelated-schema", "bin", "test", b"not-json")
        .map_err(|error| error.to_string())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_rejects_stale_schema_and_context_findings() -> Result<(), String> {
    let clean = clean_mission_json(false)?;
    let stale = validate_test_mission_source(
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
    let finding = validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &finding,
    );
    let Err(finding) = finding else {
        // jig-ignore-next-line: literal
        return Err("mission context finding reached Unreal planning".to_owned());
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
            SUMMARY_FILE,
            MISSION_DEFINITIONS_FILE,
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
    if PUBLISHED_FILES.len() != 9 || PUBLISHED_FILE_COUNT != 10 {
        return Err(
            "prepare-unreal must publish nine cache files plus one manifest"
                .to_owned(),
        );
    }
    Ok(())
}

#[test]
fn accepts_clean_current_audit() -> Result<(), String> {
    // jig-ignore-next-line: literal
    let path = std::env::temp_dir().join(format!("shar-unreal-audit-{}.json", std::process::id()));
    let manifest = "one\ntwo\nthree\n";
        // jig-ignore-next-line: expression
        fs::write(&path, clean_audit(manifest, 3)).map_err(|error| error.to_string())?;
        // jig-ignore-next-line: expression
        let result = validate_audit(&path, manifest).map_err(|error| error.to_string());
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
        // jig-ignore-next-line: expression
        fs::write(&path, clean_audit(audited_manifest, 2)).map_err(|error| error.to_string())?;
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
// jig-ignore-next-line: long identifier
fn rejects_same_row_count_with_changed_manifest_content() -> Result<(), String> {
    let path = std::env::temp_dir().join(format!(
        "shar-unreal-hash-audit-{}.json",
        std::process::id()
    ));
    let audited_manifest = "one\ntwo\n";
        // jig-ignore-next-line: expression
        fs::write(&path, clean_audit(audited_manifest, 2)).map_err(|error| error.to_string())?;
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

// jig-ignore-next-line: long identifier
fn rendered_fixture(package_count: u64, source_package: &str) -> (String, String) {
    let manifest = format!(
        concat!(
            "{{\"schema\":\"{}\",\"record_type\":\"header\",",
            "\"package_count\":{},\"source_count\":1,",
            "\"direct_import_count\":1,\"requires_fbx_count\":0,",
            "\"requires_editor_factory_count\":0,",
            "\"requires_semantic_conversion_count\":0,",
            "\"metadata_only_count\":0}}\n",
            "{{\"schema\":\"{}\",\"record_type\":\"package\",",
            "\"package_id\":\"pkg\",\"category\":\"ui-images\",",
            "\"disposition\":\"direct-editor-import\",",
            "\"target_kind\":\"Texture2D\",",
            "\"source_count\":1,\"source_unit_ids\":[],",
            "\"text_key_ids\":[]}}\n",
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
        // jig-ignore-next-line: expression
        validate_rendered_output(&manifest, &summary).map_err(|error| error.to_string())
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
// jig-ignore-next-line: long identifier
fn prepare_io_diagnostics_hide_paths_and_raw_error_text() -> Result<(), String> {
    let private_fragment = "private-workstation-unreal-prepare";
    let error = std::io::Error::other(private_fragment);
    // jig-ignore-next-line: literal
    let rendered = prepare_io_error("read Unreal source evidence", &error).to_string();
    if rendered.contains(private_fragment)
        || rendered != "read Unreal source evidence failed (Other)"
    {
        return Err(format!("prepare diagnostic leaked: {rendered}"));
    }

    let missing = std::env::temp_dir()
        .join(private_fragment)
        .join("missing-manifest.jsonl");
    let Err(read_error) = read_utf8(&missing, "read minor-unit manifest") else {
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
// jig-ignore-next-line: long identifier
fn relative_path_diagnostics_do_not_echo_rejected_values() -> Result<(), String> {
    let private_path = "private-workstation/../escape";
    let Err(error) = validate_relative_path(private_path) else {
        return Err("escaping path was accepted".to_owned());
    };
    let error = error.to_string();
    // jig-ignore-next-line: literal
    if error.contains(private_path) || error != "unsafe minor-unit relative path" {
        return Err(format!("relative-path diagnostic leaked: {error}"));
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn publication_failure_reports_failed_rollback_without_raw_text() -> Result<(), String> {
    // jig-ignore-next-line: literal
    let publish = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "private-publish-path");
    // jig-ignore-next-line: literal
    let rollback = std::io::Error::new(std::io::ErrorKind::AlreadyExists, "private-backup-path");
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
fn publication_rollback_restores_cache_and_manifest() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "shar-unreal-publication-rollback-{}",
        std::process::id(),
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let destination = root.join("accepted");
    let backup = root.join("backup");
    let manifest = root.join("unreal.jsonl");
    let manifest_backup = root.join("manifest-backup.jsonl");
    fs::create_dir_all(&destination).map_err(|error| error.to_string())?;
    fs::write(destination.join("new.txt"), b"new")
        .map_err(|error| error.to_string())?;
    fs::create_dir_all(&backup).map_err(|error| error.to_string())?;
    fs::write(backup.join("old.txt"), b"old")
        .map_err(|error| error.to_string())?;
    fs::write(&manifest_backup, b"old-manifest")
        .map_err(|error| error.to_string())?;

    let result = restore_previous_publication(
        &destination,
        &backup,
        true,
        &manifest,
        &manifest_backup,
        true,
        true,
    )
    .map_err(|error| error.to_string());
    let old_cache = fs::read(destination.join("old.txt"))
        .map_err(|error| error.to_string())?;
    let old_manifest = fs::read(&manifest).map_err(|error| error.to_string())?;
    let new_exists = destination.join("new.txt").exists();
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    result?;
    if old_cache != b"old" || old_manifest != b"old-manifest" || new_exists {
        return Err("Unreal publication rollback was not atomic".to_owned());
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn generated_transaction_chain_rejects_non_directory_ancestors() -> Result<(), String> {
    // jig-ignore-next-line: literal
    let private_fragment = format!("private-unreal-chain-{}", std::process::id());
    let root = std::env::temp_dir().join(&private_fragment);
    let directory = root.join("directory");
    let file = root.join("not-a-directory");
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&directory).map_err(|error| error.to_string())?;
    fs::write(&file, b"not a directory").map_err(|error| error.to_string())?;
        // jig-ignore-next-line: expression
        let result = validate_generated_chain(&[root.as_path(), directory.as_path(), file.as_path()]);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("non-directory transaction ancestor was accepted".to_owned());
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
    // jig-ignore-next-line: literal
    let private_fragment = format!("private-unreal-create-{}", std::process::id());
    let root = std::env::temp_dir().join(&private_fragment);
    let blocked = root.join("blocked");
    let child = blocked.join("child");
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    fs::write(&blocked, b"not a directory").map_err(|error| error.to_string())?;
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
    // jig-ignore-next-line: literal
    let manifest = manifest.replace("direct-editor-import", private_disposition);
    let Err(error) = validate_rendered_output(&manifest, &summary) else {
        return Err("unknown package disposition was accepted".to_owned());
    };
    let rendered = error.to_string();
    // jig-ignore-next-line: literal
    if !rendered.contains("unsupported disposition") || rendered.contains(private_disposition) {
        return Err(format!("unexpected disposition failure: {error}"));
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rejects_rendered_source_with_non_object_direct_import() -> Result<(), String> {
    let (manifest, summary) = rendered_fixture(1, "pkg");
    // jig-ignore-next-line: literal
    let manifest = manifest.replace("\"direct_import\":{}", "\"direct_import\":\"invalid\"");
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
    // jig-ignore-next-line: literal
    if rendered.contains(private_id) || !rendered.contains("package id is not canonical") {
        return Err(format!("rendered-id diagnostic leaked: {rendered}"));
    }
    Ok(())
}

fn rendered_derived_fixture(source_unit_id: &str) -> (String, String) {
    let manifest = format!(
        concat!(
            "{{\"schema\":\"{}\",\"record_type\":\"header\",",
            "\"package_count\":2,\"source_count\":1,",
            "\"direct_import_count\":1,\"requires_fbx_count\":0,",
            "\"requires_editor_factory_count\":1,",
            "\"requires_semantic_conversion_count\":0,",
            "\"metadata_only_count\":0}}\n",
            "{{\"schema\":\"{}\",\"record_type\":\"package\",",
            "\"package_id\":\"derived-text\",",
            "\"category\":\"language\",",
            "\"disposition\":\"requires-editor-factory\",",
            "\"target_kind\":\"StringTable\",\"source_count\":0,",
            "\"source_unit_ids\":[\"{}\"],",
            "\"text_key_ids\":[\"text-key-a\"]}}\n",
            "{{\"schema\":\"{}\",\"record_type\":\"package\",",
            "\"package_id\":\"physical\",",
            "\"disposition\":\"direct-editor-import\",",
            "\"target_kind\":\"Texture2D\",\"source_count\":1,",
            "\"source_unit_ids\":[],\"text_key_ids\":[]}}\n",
            "{{\"schema\":\"{}\",\"record_type\":\"source\",",
            "\"package_id\":\"physical\",\"id\":\"src\",",
            "\"direct_import\":{{}}}}\n"
        ),
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        source_unit_id,
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        UNREAL_IMPORT_MANIFEST_SCHEMA,
    );
    let summary = format!(
        concat!(
            "{{\"schema\":\"{}\",\"packages\":2,",
            "\"sources\":1,\"direct_imports\":1,",
            "\"requires_fbx\":0,\"requires_editor_factory\":1,",
            "\"requires_semantic_conversion\":0,",
            "\"metadata_only\":0}}\n"
        ),
        UNREAL_IMPORT_SUMMARY_SCHEMA,
    );
    (manifest, summary)
}

#[test]
fn accepts_source_backed_derived_string_table_package() -> Result<(), String> {
    let (manifest, summary) = rendered_derived_fixture("src");
    validate_rendered_output(&manifest, &summary)
        .map_err(|error| error.to_string())
}

#[test]
fn rejects_derived_package_with_missing_source_provenance(
) -> Result<(), String> {
    let (manifest, summary) = rendered_derived_fixture("missing");
    let Err(error) = validate_rendered_output(&manifest, &summary) else {
        return Err("missing derived source provenance was accepted".to_owned());
    };
    if !error.to_string().contains("missing source provenance") {
        return Err(format!("unexpected derived provenance failure: {error}"));
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
    validate_publication_inventory(&exact).map_err(|error| error.to_string())?;

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
    // jig-ignore-next-line: literal
    if error.to_string() != "Unreal staging publication inventory is not exact" {
        return Err(format!("unexpected inventory failure: {error}"));
    }
    Ok(())
}

#[test]
fn public_identifier_guard_rejects_path_shaped_values() -> Result<(), String> {
    let private_id = "C:/private/minor-unit";
    // jig-ignore-next-line: literal
    let Err(error) = validate_public_identifier(private_id, "minor-unit source id") else {
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
    let audit =
        // jig-ignore-next-line: literal
        clean_audit(manifest, 1).replace("shar-schoenwald.minor-unit-audit.v2", private_schema);
    fs::write(&path, audit).map_err(|error| error.to_string())?;
    let result = validate_audit(&path, manifest);
    fs::remove_file(&path).map_err(|error| error.to_string())?;
    let Err(error) = result else {
        return Err("unsupported audit schema was accepted".to_owned());
    };
    let rendered = error.to_string();
    // jig-ignore-next-line: literal
    if rendered.contains(private_schema) || rendered != "minor-unit audit schema is not supported" {
        return Err(format!("audit-schema diagnostic leaked: {rendered}"));
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rendered_record_type_failure_does_not_echo_rejected_value() -> Result<(), String> {
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
    // jig-ignore-next-line: literal
    if rendered.contains(private_type) || !rendered.contains("unsupported record type") {
        return Err(format!("record-type diagnostic leaked: {rendered}"));
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_rejects_stage_without_root_objective() -> Result<(), String> {
    let bytes = serde_json::to_vec(&json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension":"mfk","route_class":"mission","source_bytes":64,
        "context_command_count":4,"context_adaptation_count":0,
        "context_adaptations":[],
        "context_finding_count":0,
        "context_findings":[],
        "statement_count":4,
        "unique_command_count":4,
        "load_p3d_reference_count":0,
        "mission_flow_command_count":4,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script",
                // jig-ignore-next-line: literal
                "command_counts":{"selectmission":1,"addstage":1,"closestage":1,"closemission":1},
        "source_statements":[
                        // jig-ignore-next-line: literal
                        "SelectMission(\"m1\");","AddStage(0);","CloseStage();","CloseMission();"
        ],"p3d_references":[],
        "command_invocations":[
                        {
                            "ordinal":1,
                            "name":"selectmission",
                            "args_raw":"\"m1\"",
                            "semantic_role":"mission-script",
                            "arguments":["m1"]
                        },
                        {
                            "ordinal":2,
                            "name":"addstage",
                            "args_raw":"0",
                            "semantic_role":"mission-stage",
                            "arguments":["0"]
                        },
                        {
                            "ordinal":3,
                            "name":"closestage",
                            "args_raw":"",
                            "semantic_role":"mission-stage",
                            "arguments":[]
                        },
                        {
                            "ordinal":4,
                            "name":"closemission",
                            "args_raw":"",
                            "semantic_role":"mission-script",
                            "arguments":[]
                        }
        ]
    })).map_err(|error| error.to_string())?;
    let result = validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &bytes,
    );
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("mission stage without root objective was accepted".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains("mission scope preflight failed")
        || !rendered.contains("exactly one root objective")
    {
        return Err(format!("unexpected mission scope failure: {rendered}"));
    }
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_rejects_unreviewed_stage_flags() -> Result<(), String> {
    let bytes = clean_mission_json(false)?;
    let mut value =
                // jig-ignore-next-line: expression
                serde_json::from_slice::<serde_json::Value>(&bytes).map_err(|error| error.to_string())?;
    *value
        .pointer_mut("/source_statements/1")
        // jig-ignore-next-line: literal
        .ok_or_else(|| "mission fixture statement disappeared".to_owned())? = json!("AddStage(6);");
    *value
        .pointer_mut("/command_invocations/1/args_raw")
        // jig-ignore-next-line: literal
        .ok_or_else(|| "mission fixture raw arguments disappeared".to_owned())? = json!("6");
    *value
        .pointer_mut("/command_invocations/1/arguments")
        // jig-ignore-next-line: literal
        .ok_or_else(|| "mission fixture arguments disappeared".to_owned())? = json!(["6"]);
    let bytes = serde_json::to_vec(&value).map_err(|error| error.to_string())?;
    let result = validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &bytes,
    );
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("unreviewed legacy stage flags reached Unreal planning".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains("mission stage semantic preflight failed")
        || !rendered.contains("stage flags are not reviewed")
    {
        return Err(format!("unexpected stage semantic failure: {rendered}"));
    }
    Ok(())
}

fn timer_mission_json(duration: &str) -> Result<Vec<u8>, String> {
    let duration_raw = duration.to_owned();
    serde_json::to_vec(&json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension":"mfk","route_class":"mission","source_bytes":112,
        "context_command_count":6,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,
        "context_findings":[],"statement_count":7,"unique_command_count":7,
        "load_p3d_reference_count":0,"mission_flow_command_count":6,
        "vehicle_physics_command_count":0,"semantic_family":"mission-script",
        "command_counts":{
            "selectmission":1,"addstage":1,"addobjective":1,
            "setdurationtime":1,"closeobjective":1,"closestage":1,
            "closemission":1
        },
        "source_statements":[
            "SelectMission(\"m1\");","AddStage(0);",
            "AddObjective(\"timer\");",
            format!("SetDurationTime({duration_raw});"),
            "CloseObjective();","CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
                        {
                            "ordinal":1,
                            "name":"selectmission",
                            "args_raw":"\"m1\"",
                            "semantic_role":"mission-script",
                            "arguments":["m1"]
                        },
                        {
                            "ordinal":2,
                            "name":"addstage",
                            "args_raw":"0",
                            "semantic_role":"mission-stage",
                            "arguments":["0"]
                        },
                        {
                            "ordinal":3,
                            "name":"addobjective",
                            "args_raw":"\"timer\"",
                            "semantic_role":"mission-objective",
                            "arguments":["timer"]
                        },
                        {
                            "ordinal":4,
                            "name":"setdurationtime",
                            "args_raw":duration_raw,
                            "semantic_role":"mission-script",
                            "arguments":[duration]
                        },
                        {
                            "ordinal":5,
                            "name":"closeobjective",
                            "args_raw":"",
                            "semantic_role":"mission-objective",
                            "arguments":[]
                        },
                        {
                            "ordinal":6,
                            "name":"closestage",
                            "args_raw":"",
                            "semantic_role":"mission-stage",
                            "arguments":[]
                        },
                        {
                            "ordinal":7,
                            "name":"closemission",
                            "args_raw":"",
                            "semantic_role":"mission-script",
                            "arguments":[]
                        }
        ]
    }))
    .map_err(|error| error.to_string())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_rejects_invalid_objective_duration() -> Result<(), String> {
    let bytes = timer_mission_json("0")?;
    let result = validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &bytes,
    );
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("invalid objective duration reached Unreal planning".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains("mission objective semantic preflight failed")
        || !rendered.contains("must be positive")
    {
        // jig-ignore-next-line: literal
        return Err(format!("unexpected objective semantic failure: {rendered}"));
    }
    Ok(())
}

fn condition_time_mission_json(value: &str) -> Result<Vec<u8>, String> {
    let value_raw = value.to_owned();
    serde_json::to_vec(&json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension":"mfk","route_class":"mission","source_bytes":144,
        "context_command_count":8,"context_adaptation_count":0,
        "context_adaptations":[],"context_finding_count":0,
        "context_findings":[],"statement_count":9,"unique_command_count":9,
        "load_p3d_reference_count":0,"mission_flow_command_count":6,
        "vehicle_physics_command_count":0,"semantic_family":"mission-script",
        "command_counts":{
            "selectmission":1,"addstage":1,"addobjective":1,
            "closeobjective":1,"addcondition":1,"setcondtime":1,
            "closecondition":1,"closestage":1,"closemission":1
        },
        "source_statements":[
            "SelectMission(\"m1\");","AddStage(0);",
            "AddObjective(\"dummy\");","CloseObjective();",
            "AddCondition(\"outofvehicle\");",
            format!("SetCondTime({value_raw});"),"CloseCondition();",
            "CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
                        {
                            "ordinal":1,
                            "name":"selectmission",
                            "args_raw":"\"m1\"",
                            "semantic_role":"mission-script",
                            "arguments":["m1"]
                        },
                        {
                            "ordinal":2,
                            "name":"addstage",
                            "args_raw":"0",
                            "semantic_role":"mission-stage",
                            "arguments":["0"]
                        },
                        {
                            "ordinal":3,
                            "name":"addobjective",
                            "args_raw":"\"dummy\"",
                            "semantic_role":"mission-objective",
                            "arguments":["dummy"]
                        },
                        {
                            "ordinal":4,
                            "name":"closeobjective",
                            "args_raw":"",
                            "semantic_role":"mission-objective",
                            "arguments":[]
                        },
                        {
                            "ordinal":5,
                            "name":"addcondition",
                            "args_raw":"\"outofvehicle\"",
                            "semantic_role":"mission-script",
                            "arguments":["outofvehicle"]
                        },
                        {
                            "ordinal":6,
                            "name":"setcondtime",
                            "args_raw":value_raw,
                            "semantic_role":"mission-script",
                            "arguments":[value]
                        },
                        {
                            "ordinal":7,
                            "name":"closecondition",
                            "args_raw":"",
                            "semantic_role":"mission-script",
                            "arguments":[]
                        },
                        {
                            "ordinal":8,
                            "name":"closestage",
                            "args_raw":"",
                            "semantic_role":"mission-stage",
                            "arguments":[]
                        },
                        {
                            "ordinal":9,
                            "name":"closemission",
                            "args_raw":"",
                            "semantic_role":"mission-script",
                            "arguments":[]
                        }
        ]
    }))
    .map_err(|error| error.to_string())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_rejects_invalid_condition_time() -> Result<(), String> {
    let bytes = condition_time_mission_json("0")?;
    let result = validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &bytes,
    );
    let Err(error) = result else {
        return Err("invalid condition time reached Unreal planning".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains("mission condition semantic preflight failed")
        || !rendered.contains("must be positive")
    {
        // jig-ignore-next-line: literal
        return Err(format!("unexpected condition semantic failure: {rendered}"));
    }
    Ok(())
}

fn mission_root_ped_group_json(value: &str) -> Result<Vec<u8>, String> {
    let value_raw = value.to_owned();
    serde_json::to_vec(&json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension":"mfk","route_class":"mission","source_bytes":128,
        "context_command_count":6,"context_adaptation_count":0,
        "context_adaptations":[],
        "context_finding_count":0,
        "context_findings":[],
        "statement_count":7,
        "unique_command_count":7,
        "load_p3d_reference_count":0,
        "mission_flow_command_count":6,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script",
        "command_counts":{
            "selectmission":1,"usepedgroup":1,"addstage":1,"addobjective":1,
            "closeobjective":1,"closestage":1,"closemission":1
        },
        "source_statements":[
            "SelectMission(\"m1\");",format!("UsePedGroup({value_raw});"),
            "AddStage(0);","AddObjective(\"goto\");","CloseObjective();",
            "CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
                        {
                            "ordinal":1,
                            "name":"selectmission",
                            "args_raw":"\"m1\"",
                            "semantic_role":"mission-script",
                            "arguments":["m1"]
                        },
                        {
                            "ordinal":2,
                            "name":"usepedgroup",
                            "args_raw":value_raw,
                            "semantic_role":"mission-script",
                            "arguments":[value]
                        },
                        {
                            "ordinal":3,
                            "name":"addstage",
                            "args_raw":"0",
                            "semantic_role":"mission-stage",
                            "arguments":["0"]
                        },
                        {
                            "ordinal":4,
                            "name":"addobjective",
                            "args_raw":"\"goto\"",
                            "semantic_role":"mission-objective",
                            "arguments":["goto"]
                        },
                        {
                            "ordinal":5,
                            "name":"closeobjective",
                            "args_raw":"",
                            "semantic_role":"mission-objective",
                            "arguments":[]
                        },
                        {
                            "ordinal":6,
                            "name":"closestage",
                            "args_raw":"",
                            "semantic_role":"mission-stage",
                            "arguments":[]
                        },
                        {
                            "ordinal":7,
                            "name":"closemission",
                            "args_raw":"",
                            "semantic_role":"mission-script",
                            "arguments":[]
                        }
        ]
    }))
    .map_err(|error| error.to_string())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_rejects_invalid_root_ped_group() -> Result<(), String> {
    let bytes = mission_root_ped_group_json("8")?;
    let result = validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &bytes,
    );
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("invalid mission pedestrian group reached Unreal planning".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains("mission initialization preflight failed")
        || !rendered.contains("ped-group index is not reviewed")
    {
        return Err(format!(
            "unexpected mission-root semantic failure: {rendered}"
        ));
    }
    Ok(())
}

fn conversation_camera_mission_json(slot: &str) -> Result<Vec<u8>, String> {
    let slot_raw = slot.to_owned();
    serde_json::to_vec(&json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension":"mfk","route_class":"mission","source_bytes":128,
        "context_command_count":6,"context_adaptation_count":0,
        "context_adaptations":[],
        "context_finding_count":0,
        "context_findings":[],
        "statement_count":7,
        "unique_command_count":7,
        "load_p3d_reference_count":0,
        "mission_flow_command_count":6,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script",
        "command_counts":{
            "selectmission":1,"addstage":1,"addobjective":1,
            "setconversationcam":1,"closeobjective":1,"closestage":1,
            "closemission":1
        },
        "source_statements":[
            "SelectMission(\"m1\");","AddStage(0);",
            "AddObjective(\"dialogue\");",
            format!("SetConversationCam({slot_raw},\"pc_far\");"),
            "CloseObjective();","CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
                        {
                            "ordinal":1,
                            "name":"selectmission",
                            "args_raw":"\"m1\"",
                            "semantic_role":"mission-script",
                            "arguments":["m1"]
                        },
                        {
                            "ordinal":2,
                            "name":"addstage",
                            "args_raw":"0",
                            "semantic_role":"mission-stage",
                            "arguments":["0"]
                        },
                        {
                            "ordinal":3,
                            "name":"addobjective",
                            "args_raw":"\"dialogue\"",
                            "semantic_role":"mission-objective",
                            "arguments":["dialogue"]
                        },
                        {
                            "ordinal":4,
                            "name":"setconversationcam",
                            "args_raw":format!("{slot_raw},\"pc_far\""),
                            "semantic_role":"mission-script",
                            "arguments":[slot,"pc_far"]
                        },
                        {
                            "ordinal":5,
                            "name":"closeobjective",
                            "args_raw":"",
                            "semantic_role":"mission-objective",
                            "arguments":[]
                        },
                        {
                            "ordinal":6,
                            "name":"closestage",
                            "args_raw":"",
                            "semantic_role":"mission-stage",
                            "arguments":[]
                        },
                        {
                            "ordinal":7,
                            "name":"closemission",
                            "args_raw":"",
                            "semantic_role":"mission-script",
                            "arguments":[]
                        }
        ]
    }))
    .map_err(|error| error.to_string())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_rejects_invalid_conversation_camera() -> Result<(), String> {
    let bytes = conversation_camera_mission_json("7")?;
    let result = validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &bytes,
    );
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("invalid conversation-camera slot reached Unreal planning".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains("mission objective semantic preflight failed")
        || !rendered.contains("conversation-camera slot is not reviewed")
    {
        return Err(format!(
            "unexpected conversation-camera failure: {rendered}"
        ));
    }
    Ok(())
}

fn stage_race_catchup_mission_json(factor: &str) -> Result<Vec<u8>, String> {
    let factor_raw = factor.to_owned();
    serde_json::to_vec(&json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension":"mfk","route_class":"mission","source_bytes":160,
        "context_command_count":6,"context_adaptation_count":0,
        "context_adaptations":[],
        "context_finding_count":0,
        "context_findings":[],
        "statement_count":7,
        "unique_command_count":7,
        "load_p3d_reference_count":0,
        "mission_flow_command_count":7,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script",
        "command_counts":{
            "selectmission":1,"addstage":1,"setstageairacecatchupparams":1,
            "addobjective":1,"closeobjective":1,"closestage":1,"closemission":1
        },
        "source_statements":[
            "SelectMission(\"m1\");","AddStage(0);",
                        // jig-ignore-next-line: literal
            format!("SetStageAIRaceCatchupParams(\"car\",80,{factor_raw},1.0,1.0);"),
            "AddObjective(\"goto\");","CloseObjective();","CloseStage();",
            "CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
                        {
                            "ordinal":1,
                            "name":"selectmission",
                            "args_raw":"\"m1\"",
                            "semantic_role":"mission-script",
                            "arguments":["m1"]
                        },
                        {
                            "ordinal":2,
                            "name":"addstage",
                            "args_raw":"0",
                            "semantic_role":"mission-stage",
                            "arguments":["0"]
                        },
                        {
                            "ordinal":3,
                            "name":"setstageairacecatchupparams",
                                                // jig-ignore-next-line: literal
                                                        "args_raw":format!("\"car\",80,{factor_raw},1.0,1.0"),
                            "semantic_role":"mission-stage",
                            "arguments":["car","80",factor,"1.0","1.0"]
                        },
                        {
                            "ordinal":4,
                            "name":"addobjective",
                            "args_raw":"\"goto\"",
                            "semantic_role":"mission-objective",
                            "arguments":["goto"]
                        },
                        {
                            "ordinal":5,
                            "name":"closeobjective",
                            "args_raw":"",
                            "semantic_role":"mission-objective",
                            "arguments":[]
                        },
                        {
                            "ordinal":6,
                            "name":"closestage",
                            "args_raw":"",
                            "semantic_role":"mission-stage",
                            "arguments":[]
                        },
                        {
                            "ordinal":7,
                            "name":"closemission",
                            "args_raw":"",
                            "semantic_role":"mission-script",
                            "arguments":[]
                        }
        ]
    }))
    .map_err(|error| error.to_string())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_rejects_invalid_stage_race_catchup() -> Result<(), String> {
    let bytes = stage_race_catchup_mission_json("1e0")?;
    let result = validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &bytes,
    );
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("invalid stage race catch-up reached Unreal planning".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains("mission stage semantic preflight failed")
        || !rendered.contains("race catch-up decimal is malformed")
    {
        return Err(format!("unexpected stage catch-up failure: {rendered}"));
    }
    Ok(())
}

fn participant_mission_json() -> Result<Vec<u8>, String> {
    serde_json::to_vec(&json!({
        "schema": MISSION_SCRIPT_SCHEMA,
        "source_extension":"mfk","route_class":"mission","source_bytes":128,
        "context_command_count":6,"context_adaptation_count":0,
        "context_adaptations":[],
        "context_finding_count":0,
        "context_findings":[],
        "statement_count":7,
        "unique_command_count":7,
        "load_p3d_reference_count":0,
        "mission_flow_command_count":6,"vehicle_physics_command_count":0,
        "semantic_family":"mission-script",
        "command_counts":{
            "selectmission":1,"addstage":1,"addobjective":1,"addnpc":1,
            "closeobjective":1,"closestage":1,"closemission":1
        },
        "source_statements":[
            "SelectMission(\"m1\");","AddStage(0);",
            "AddObjective(\"talkto\");","AddNPC(\"bart\",\"npc_loc\");",
            "CloseObjective();","CloseStage();","CloseMission();"
        ],
        "p3d_references":[],
        "command_invocations":[
                        {
                            "ordinal":1,
                            "name":"selectmission",
                            "args_raw":"\"m1\"",
                            "semantic_role":"mission-script",
                            "arguments":["m1"]
                        },
                        {
                            "ordinal":2,
                            "name":"addstage",
                            "args_raw":"0",
                            "semantic_role":"mission-stage",
                            "arguments":["0"]
                        },
                        {
                            "ordinal":3,
                            "name":"addobjective",
                            "args_raw":"\"talkto\"",
                            "semantic_role":"mission-objective",
                            "arguments":["talkto"]
                        },
                        {
                            "ordinal":4,
                            "name":"addnpc",
                            "args_raw":"\"bart\",\"npc_loc\"",
                            "semantic_role":"mission-script",
                            "arguments":["bart","npc_loc"]
                        },
                        {
                            "ordinal":5,
                            "name":"closeobjective",
                            "args_raw":"",
                            "semantic_role":"mission-objective",
                            "arguments":[]
                        },
                        {
                            "ordinal":6,
                            "name":"closestage",
                            "args_raw":"",
                            "semantic_role":"mission-stage",
                            "arguments":[]
                        },
                        {
                            "ordinal":7,
                            "name":"closemission",
                            "args_raw":"",
                            "semantic_role":"mission-script",
                            "arguments":[]
                        }
        ]
    }))
    .map_err(|error| error.to_string())
}

#[test]
// jig-ignore-next-line: long identifier
fn mission_semantic_gate_rejects_missing_participant_package() -> Result<(), String> {
    let bytes = participant_mission_json()?;
    let result = validate_test_mission_source(
        "mission-script",
        MISSION_SCRIPT_SCHEMA,
        "json",
        "game-straggler-normalize",
        &bytes,
    );
    let Err(error) = result else {
        // jig-ignore-next-line: literal
        return Err("missing participant package reached Unreal planning".to_owned());
    };
    let rendered = error.to_string();
    if !rendered.contains("mission participant reference preflight failed")
        || !rendered.contains("character source identity has no package")
    {
        return Err(format!(
            "unexpected participant reference failure: {rendered}"
        ));
    }
    Ok(())
}

#[test]
fn accepts_source_distinct_mission_definition_rows() -> Result<(), String> {
    let rows = vec![
        mission_definition_row("script-one", "m1"),
        mission_definition_row("script-two", "m1"),
    ];
    let verified = vec![
        mission_source("script-one"),
        mission_source("script-two"),
    ];
    let rendered = validate_mission_definition_bundle(&rows, &verified)
        .map_err(|error| error.to_string())?;
    assert_eq!(rendered, rows.concat());
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rejects_mission_definitions_out_of_verified_source_order() -> Result<(), String> {
    let rows = vec![
        mission_definition_row("script-two", "m1"),
        mission_definition_row("script-one", "m1"),
    ];
    let verified = vec![
        mission_source("script-one"),
        mission_source("script-two"),
    ];
    let error = validate_mission_definition_bundle(&rows, &verified)
        .rejection("out-of-order mission definition sources must fail")?;
    assert!(error.to_string().contains("verified source order"));
    Ok(())
}

#[test]
fn rejects_duplicate_mission_definition_source() -> Result<(), String> {
    let row = mission_definition_row("script-one", "m1");
    let rows = vec![row.clone(), row];
    let verified = vec![mission_source("script-one")];
    let error = validate_mission_definition_bundle(&rows, &verified)
        .rejection("duplicate mission definition source must fail")?;
    assert!(error.to_string().contains("duplicates a source id"));
    Ok(())
}

#[test]
fn rejects_unverified_mission_definition_source() -> Result<(), String> {
    let rows = vec![mission_definition_row("script-one", "m1")];
    let verified = vec![source("script-one")];
    let error = validate_mission_definition_bundle(&rows, &verified)
        .rejection("non-mission verified source must fail")?;
    assert!(error.to_string().contains("not verified mission evidence"));
    Ok(())
}

#[test]
fn rejects_mission_definition_with_sparse_stage_order() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 99,
                "unavailable_code": null,
            },
            "sequence_ordinal": 1,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("sparse staged mission topology must fail")?;
    assert!(error.to_string().contains("sequence ordinal is not dense"));
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rejects_mission_definition_with_authored_neighbor_drift() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![
            json!({
                "explicit_final": false,
                "kind": {
                    "final_stage": false,
                    "kind": "standard",
                    "legacy_flags": 0,
                },
                "next_authored_sequence_ordinal": null,
                "objective": {
                    "canonical_kind": "travel",
                    "source_alias": "goto",
                    "source_ordinal": 99,
                    "unavailable_code": null,
                },
                "sequence_ordinal": 0,
                "stage_source_ordinal": 1,
                "terminal": "none",
            }),
            json!({
                "explicit_final": false,
                "kind": {
                    "final_stage": false,
                    "kind": "standard",
                    "legacy_flags": 0,
                },
                "next_authored_sequence_ordinal": null,
                "objective": {
                    "canonical_kind": "travel",
                    "source_alias": "goto",
                    "source_ordinal": 99,
                    "unavailable_code": null,
                },
                "sequence_ordinal": 1,
                "stage_source_ordinal": 2,
                "terminal": "none",
            }),
        ],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("authored neighbor drift must fail")?;
    assert!(error.to_string().contains("authored neighbor drifted"));
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rejects_mission_definition_with_early_terminal_outcome() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![
            json!({
                "explicit_final": false,
                "kind": {
                    "final_stage": false,
                    "kind": "standard",
                    "legacy_flags": 0,
                },
                "next_authored_sequence_ordinal": 1,
                "objective": {
                    "canonical_kind": "travel",
                    "source_alias": "goto",
                    "source_ordinal": 99,
                    "unavailable_code": null,
                },
                "sequence_ordinal": 0,
                "stage_source_ordinal": 1,
                "terminal": "chapter-transition",
            }),
            json!({
                "explicit_final": false,
                "kind": {
                    "final_stage": false,
                    "kind": "standard",
                    "legacy_flags": 0,
                },
                "next_authored_sequence_ordinal": null,
                "objective": {
                    "canonical_kind": "travel",
                    "source_alias": "goto",
                    "source_ordinal": 99,
                    "unavailable_code": null,
                },
                "sequence_ordinal": 1,
                "stage_source_ordinal": 2,
                "terminal": "none",
            }),
        ],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("early terminal outcome must fail")?;
    assert!(error.to_string().contains("before the final stage"));
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rejects_mission_definition_with_invented_runtime_edge() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 99,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "successor_sequence_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("invented runtime edge must fail")?;
    assert!(error.to_string().contains("invents unresolved runtime field"));
    Ok(())
}

#[test]
fn rejects_mission_definition_with_kind_final_drift() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "explicit_final": true,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("stage kind final drift must fail")?;
    assert!(error.to_string().contains("final marker disagrees"));
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rejects_mission_definition_with_nonexclusive_objective_mapping() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": "legacy-objective-unavailable",
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("nonexclusive objective mapping must fail")?;
    assert!(error.to_string().contains("objective mapping is not exclusive"));
    Ok(())
}

#[test]
fn accepts_owned_mission_definition_conditions() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "conditions": [
                {
                    "owner_objective_source_ordinal": null,
                    "schema_id": "legacy-mission-condition.timeout.v1",
                    "scope": "stage",
                    "source_alias": "timeout",
                    "source_ordinal": 3,
                    "violation_effect": "stage-failure",
                },
                {
                    "owner_objective_source_ordinal": 2,
                    "schema_id": "legacy-mission-condition.damage.v1",
                    "scope": "objective",
                    "source_alias": "damage",
                    "source_ordinal": 4,
                    "violation_effect": "stage-failure",
                },
            ],
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    drop(
        validate_mission_definition_bundle(
            &rows,
            &[mission_source("script-one")],
        )
        .map_err(|error| error.to_string())?,
    );
    Ok(())
}

#[test]
fn rejects_mission_definition_condition_owner_drift() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "conditions": [{
                "owner_objective_source_ordinal": 9,
                "schema_id": "legacy-mission-condition.timeout.v1",
                "scope": "objective",
                "source_alias": "timeout",
                "source_ordinal": 3,
                "violation_effect": "stage-failure",
            }],
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("condition owner drift must fail")?;
    assert!(error.to_string().contains("objective owner drifted"));
    Ok(())
}

#[test]
fn rejects_mission_definition_condition_order_drift() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "conditions": [
                {
                    "owner_objective_source_ordinal": null,
                    "schema_id": "legacy-mission-condition.timeout.v1",
                    "scope": "stage",
                    "source_alias": "timeout",
                    "source_ordinal": 4,
                    "violation_effect": "stage-failure",
                },
                {
                    "owner_objective_source_ordinal": null,
                    "schema_id": "legacy-mission-condition.damage.v1",
                    "scope": "stage",
                    "source_alias": "damage",
                    "source_ordinal": 3,
                    "violation_effect": "stage-failure",
                },
            ],
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("condition source order drift must fail")?;
    assert!(error.to_string().contains("source ordinal is malformed"));
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rejects_mission_definition_condition_violation_drift() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "conditions": [{
                "owner_objective_source_ordinal": null,
                "schema_id": "legacy-mission-condition.timeout.v1",
                "scope": "stage",
                "source_alias": "timeout",
                "source_ordinal": 3,
                "violation_effect": "retry-stage",
            }],
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("condition violation drift must fail")?;
    assert!(error.to_string().contains("unknown violation effect"));
    Ok(())
}

#[test]
fn accepts_owned_mission_definition_countdown() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "checkpoint_source_ordinal": 3,
            "countdown": {
                "character_id": "homer",
                "entries": [
                    {
                        "duration_milliseconds": 1000,
                        "source_ordinal": 5,
                        "token": "three",
                    },
                    {
                        "duration_milliseconds": 500,
                        "source_ordinal": 6,
                        "token": "two",
                    },
                ],
                "sequence_id": "countdown",
                "stage_sequence_ordinal": 0,
                "stage_source_ordinal": 1,
                "start_source_ordinal": 4,
            },
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    drop(
        validate_mission_definition_bundle(
            &rows,
            &[mission_source("script-one")],
        )
        .map_err(|error| error.to_string())?,
    );
    Ok(())
}

#[test]
fn rejects_mission_definition_checkpoint_before_stage() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "checkpoint_source_ordinal": 1,
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("checkpoint at or before its stage must fail")?;
    // jig-ignore-next-line: literal
    assert!(error.to_string().contains("checkpoint source ordinal is malformed"));
    Ok(())
}

#[test]
fn rejects_mission_definition_countdown_owner_drift() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "countdown": {
                "character_id": null,
                "entries": [],
                "sequence_id": "countdown",
                "stage_sequence_ordinal": 1,
                "stage_source_ordinal": 1,
                "start_source_ordinal": 3,
            },
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("countdown owner drift must fail")?;
    assert!(error.to_string().contains("countdown owner drifted"));
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rejects_mission_definition_countdown_entry_order_drift() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![json!({
            "countdown": {
                "character_id": null,
                "entries": [
                    {
                        "duration_milliseconds": 1000,
                        "source_ordinal": 6,
                        "token": "three",
                    },
                    {
                        "duration_milliseconds": 500,
                        "source_ordinal": 5,
                        "token": "two",
                    },
                ],
                "sequence_id": "countdown",
                "stage_sequence_ordinal": 0,
                "stage_source_ordinal": 1,
                "start_source_ordinal": 4,
            },
            "explicit_final": false,
            "kind": {
                "final_stage": false,
                "kind": "standard",
                "legacy_flags": 0,
            },
            "next_authored_sequence_ordinal": null,
            "objective": {
                "canonical_kind": "travel",
                "source_alias": "goto",
                "source_ordinal": 2,
                "unavailable_code": null,
            },
            "sequence_ordinal": 0,
            "stage_source_ordinal": 1,
            "terminal": "none",
        })],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("countdown entry order drift must fail")?;
    assert!(error.to_string().contains("identity or order is malformed"));
    Ok(())
}

#[test]
fn accepts_owned_mission_definition_objective_bindings() -> Result<(), String> {
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![mission_definition_relationship_stage()],
    )];
    drop(
        validate_mission_definition_bundle(
            &rows,
            &[mission_source("script-one")],
        )
        .map_err(|error| error.to_string())?,
    );
    Ok(())
}

#[test]
fn rejects_mission_definition_collectible_owner_drift() -> Result<(), String> {
    let mut stage = mission_definition_relationship_stage();
    drop(first_array_object_mut(&mut stage, "collectible_waypoints")?.insert(
        "stage_sequence_ordinal".to_owned(),
        json!(1),
    ));
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![stage],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("collectible owner drift must fail")?;
    assert!(error.to_string().contains("collectible waypoint 1 owner drifted"));
    Ok(())
}

#[test]
// jig-ignore-next-line: long identifier
fn rejects_mission_definition_npc_declaration_order_drift() -> Result<(), String> {
    let mut stage = mission_definition_relationship_stage();
    drop(first_array_object_mut(&mut stage, "objective_npc_waypoints")?.insert(
        "declaration_source_ordinal".to_owned(),
        json!(8),
    ));
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![stage],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("NPC declaration order drift must fail")?;
    // jig-ignore-next-line: literal
    assert!(error.to_string().contains("NPC waypoint 1 relationship is malformed"));
    Ok(())
}

#[test]
fn rejects_mission_definition_pickup_scope_drift() -> Result<(), String> {
    let mut stage = mission_definition_relationship_stage();
    let pickup = first_array_object_mut(&mut stage, "pickup_state_props")?;
    let scope = pickup
        .get_mut("declaration_scope")
        .and_then(serde_json::Value::as_object_mut)
        // jig-ignore-next-line: literal
        .ok_or_else(|| "pickup declaration scope fixture is malformed".to_owned())?;
    drop(scope.insert("source_ordinal".to_owned(), json!(8)));
    let rows = vec![mission_definition_row_with_stages(
        "script-one",
        "m1",
        vec![stage],
    )];
    let error = validate_mission_definition_bundle(
        &rows,
        &[mission_source("script-one")],
    )
    .rejection("pickup declaration scope drift must fail")?;
    assert!(error.to_string().contains("declaration scope is malformed"));
    Ok(())
}
