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
//   - Verified Unreal staging-manifest generation and atomic publication.
// - Must-Not:
//   - Execute Unreal Editor, import packages, or mutate extracted sources.
// - Allows:
//   - Read audited minor-unit and package-index evidence.
//   - Hash normalized source files and replace the generated staging root.
// - Split-When:
//   - Split when staged binary conversion gains an independent lifecycle.
// - Merge-When:
//   - Merge when another adapter owns identical prepare-unreal effects.
// - Summary:
//   - Local prepare-unreal outbound adapter.
// - Description:
//   - Verifies every indexed source and publishes a versioned editor-facing
//     manifest without exposing partial output.
// - Usage:
//   - Invoked after audit and package-index generation by LocalPipeline.
// - Defaults:
//   - Missing, stale, unsafe, colliding, or malformed evidence fails closed.
//

//! Local prepare-unreal outbound adapter.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
// cspell:ignore recv
use std::sync::mpsc;
use std::time::SystemTime;
use std::{fs, thread};

use same_file::Handle;
use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local::{
    path_kind as local_path_kind, read_bytes as local_read_bytes, read_utf8 as local_read_utf8,
};
use serde_json::{Map, Value};
use shar_sha256::{Sha256, digest_hex};
use shar_unreal_conversion::domain::PlanBundle;

use super::mission_camera_catalog::load_mission_camera_catalog;
use super::mission_completion_dialog_context as completion_dialog_context;
use super::mission_dialogue_info_context as dialogue_info_context;
use super::mission_definition_context;
use super::mission_locator_catalog::load_mission_locator_catalog;
use super::mission_locator_context::{
    MissionLocatorScriptSnapshot, build_level_locator_source_contexts,
    build_mission_locator_source_contexts,
};
use super::mission_music_context::preflight_mission_music_states;
use super::mission_order_context::build_mission_order_source_reports;
use super::unreal_fbx_catalog::verified_fbx_catalog_at;
use crate::adapters::driven::check_cancellation;
use crate::adapters::driven::local::progress::StageProgress;
use crate::domain::{
    MISSION_SCRIPT_SCHEMA, MissionCameraCatalog, MissionLocatorCatalog,
    MissionP3dReferenceCatalog, MissionReferenceCatalog, PhaseThreePackageIndex,
    PipelineConfig, PipelineError, PipelineOutcome, StageReport, UNREAL_IMPORT_MANIFEST_SCHEMA,
    UNREAL_IMPORT_SUMMARY_SCHEMA, UnrealImportManifest, UnrealSourceEvidence,
    compile_mission_scope_graphs,
    preflight_mission_authored_stage_topology,
    preflight_mission_camera_references,
    preflight_mission_condition_commands,
    preflight_mission_condition_semantics, preflight_mission_conditions,
    preflight_mission_fmv_references, preflight_mission_gag_totals,
    preflight_mission_initialization,
    preflight_mission_level_locator_references,
    preflight_mission_level_npcs, preflight_mission_locator_references,
    preflight_mission_objective_commands,
    preflight_mission_objective_semantics,
    preflight_mission_objectives, preflight_mission_package_loads_with_catalog,
    preflight_mission_ped_group_selections, preflight_mission_ped_groups,
    preflight_mission_presentation_references,
    preflight_mission_purchase_rewards, preflight_mission_references,
    preflight_mission_reward_offers, preflight_mission_reward_references,
    preflight_mission_script,
    preflight_mission_stage_message_references,
    preflight_mission_stage_semantics,
    preflight_mission_traffic_groups,
    preflight_mission_vehicle_attributes, preflight_mission_vehicle_selects,
};
use crate::manifest_paths::FBX_MANIFEST_PATH;
use crate::workspace::{FBX_WORKSPACE_ROOT, UNREAL_STAGING_WORKSPACE_ROOT};

/// Canonical import-manifest filename.
const MANIFEST_FILE: &str = "manifest.jsonl";
/// Canonical import-summary filename.
const SUMMARY_FILE: &str = "summary.json";
/// Canonical mission definition-core bundle filename.
const MISSION_DEFINITIONS_FILE: &str = "mission-definitions.jsonl";
/// Canonical generated plan directory.
const PLAN_ROOT: &str = "plans";
/// Canonical generated plan-bundle index filename.
const PLAN_INDEX_FILE: &str = "plans/index.json";
/// Complete set of files published by one prepare-unreal transaction.
const PUBLISHED_FILES: [&str; 10] = [
    MANIFEST_FILE,
    SUMMARY_FILE,
    MISSION_DEFINITIONS_FILE,
    PLAN_INDEX_FILE,
    "plans/asset-import-plan.json",
    "plans/asset-construction-plan.json",
    "plans/world-assembly-plan.json",
    "plans/runtime-binding-plan.json",
    "plans/validation-plan.json",
    "plans/package-plan.json",
];
/// Expected successful minor-unit audit schema.
const AUDIT_SCHEMA: &str = "shar-schoenwald.minor-unit-audit.v2";

/// Generate and atomically publish Unreal staging evidence.
pub(super) fn prepare_unreal(config: &PipelineConfig) -> PipelineOutcome<StageReport> {
    let minor_unit_root = config.extracted_root.join("minor-unit");
    let manifest_path = minor_unit_root.join("manifest.jsonl");
    let audit_path = minor_unit_root.join("audit.json");
    let index_path = minor_unit_root.join("index.jsonl");
    let manifest_text = read_utf8(&manifest_path, "read minor-unit manifest")?;
    validate_audit(&audit_path, &manifest_text)?;
    let index = PhaseThreePackageIndex::read_for_unreal(&index_path).map_err(|error| {
        PipelineError::new(format!("Unreal package-index intake failed: {error}"))
    })?;
    let mission_cameras =
        load_mission_camera_catalog(&index, &config.extracted_root)?;
    let mission_locators = load_mission_locator_catalog(&index, &config.extracted_root)?;
    let mission_p3d_references =
        MissionP3dReferenceCatalog::from_package_index(&index).map_err(|error| {
            PipelineError::new(format!(
                "mission P3D reference catalog intake failed: {error}"
            ))
        })?;
    let mission_references =
        MissionReferenceCatalog::from_package_index(&index).map_err(|error| {
            PipelineError::new(format!("mission reference catalog intake failed: {error}"))
        })?;
    let source_report = source_evidence(
        &manifest_text,
        config,
        &mission_references,
        &mission_cameras,
        &mission_locators,
        &mission_p3d_references,
        &index,
    )?;
    let mission_definitions_jsonl = validate_mission_definition_bundle(
        &source_report.mission_definitions,
        &source_report.evidence,
    )?;
    let evidence = retain_importable_evidence(&index, source_report.evidence);
    let unreal_manifest = UnrealImportManifest::build(&index, evidence)
        .map_err(|error| PipelineError::new(format!("Unreal manifest planning failed: {error}")))?;
    let manifest_jsonl = unreal_manifest.to_jsonl();
    let summary_json = unreal_manifest.summary_json();
    validate_rendered_output(&manifest_jsonl, &summary_json)?;
    let manifest_revision = digest_hex(manifest_jsonl.as_bytes());
    let fbx_catalog = verified_fbx_catalog_at(
        Path::new(FBX_WORKSPACE_ROOT),
        Path::new(FBX_MANIFEST_PATH),
    )?;
    let verified_fbx_count = fbx_catalog.as_ref().map_or(0, Vec::len);
    let plan_bundle = fbx_catalog
        .as_deref()
        .map_or_else(
            || unreal_manifest.plan_bundle(&manifest_revision),
            |catalog| {
                unreal_manifest.plan_bundle_with_complete_fbx_catalog(&manifest_revision, catalog)
            },
        )
        .map_err(|error| PipelineError::new(format!("Unreal plan generation failed: {error}")))?;
    publish_staging(
        &manifest_jsonl,
        &summary_json,
        &mission_definitions_jsonl,
        &plan_bundle,
    )?;
    Ok(StageReport {
        name: "prepare-unreal",
        files: PUBLISHED_FILES.len(),
        bytes: published_byte_count(
            &manifest_jsonl,
            &summary_json,
            &mission_definitions_jsonl,
            &plan_bundle,
        ),
        note: format!(
            concat!(
                "verified {} sources across {} semantic packages and {} ",
                "generated FBX artifacts; published {} mission definitions ",
                "to {} with plan bundle {}"
            ),
            unreal_manifest.source_count(),
            unreal_manifest.package_count(),
            verified_fbx_count,
            mission_definitions_jsonl.lines().count(),
            UNREAL_STAGING_WORKSPACE_ROOT,
            plan_bundle.index_revision(),
        ),
    })
}

fn validate_audit(path: &Path, manifest: &str) -> PipelineOutcome<()> {
    let text = read_utf8(path, "read minor-unit audit")?;
    let audit = parse_object(&text, "minor-unit audit")?;
    let schema = required_string(&audit, "schema", "minor-unit audit")?;
    let rows = required_u64(&audit, "rows", "minor-unit audit")?;
    let failures = required_u64(&audit, "failures", "minor-unit audit")?;
    let error_rows = required_u64(&audit, "error_rows", "minor-unit audit")?;
    let audited_sha256 = required_string(&audit, "manifest_sha256", "minor-unit audit")?;
    if schema != AUDIT_SCHEMA {
        return Err(PipelineError::new(
            "minor-unit audit schema is not supported",
        ));
    }
    if failures != 0 || error_rows != 0 {
        return Err(PipelineError::new(format!(
            "minor-unit audit is not clean: failures={failures} \
             error_rows={error_rows}"
        )));
    }
    let expected_rows = u64::try_from(
        manifest
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count(),
    )
    .unwrap_or(u64::MAX);
    if rows != expected_rows {
        return Err(PipelineError::new(format!(
            "minor-unit audit is stale: audit rows={rows} \
             manifest rows={expected_rows}"
        )));
    }
    if audited_sha256 != digest_hex(manifest.as_bytes()) {
        return Err(PipelineError::new(
            "minor-unit audit is stale: manifest SHA-256 changed",
        ));
    }
    Ok(())
}

fn source_evidence(
    manifest: &str,
    config: &PipelineConfig,
    mission_references: &MissionReferenceCatalog,
    mission_cameras: &MissionCameraCatalog,
    mission_locators: &MissionLocatorCatalog,
    mission_p3d_references: &MissionP3dReferenceCatalog,
    index: &PhaseThreePackageIndex,
) -> PipelineOutcome<SourceEvidenceReport> {
    let source_count = manifest
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    let mut inputs = Vec::with_capacity(source_count);
    let mut ids = BTreeSet::new();
    for (line_index, line) in manifest.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        if line_index % 256 == 0 {
            check_cancellation()?;
        }
        let line_number = line_index.saturating_add(1);
        let row = parse_object(line, &format!("minor-unit line {line_number}"))?;
        let id = manifest_string(&row, "id", line_number)?;
        validate_public_identifier(&id, "minor-unit source id")?;
        if !ids.insert(id.clone()) {
            return Err(PipelineError::new(format!(
                "minor-unit manifest has duplicate id {id}"
            )));
        }
        let path = manifest_string(&row, "path", line_number)?;
        let expected_size = manifest_u64(&row, "size_bytes", line_number)?;
        let resolved = resolve_source_path(config, &path)?;
        inputs.push(SourceEvidenceInput {
            id,
            path,
            resolved,
            expected_size,
            file_extension: manifest_string(&row, "file_extension", line_number)?,
            unit_type: manifest_string(&row, "type", line_number)?,
            subtype: manifest_string(&row, "subtype", line_number)?,
            kind: manifest_string(&row, "kind", line_number)?,
            function: manifest_string(&row, "function", line_number)?,
            schema: manifest_string(&row, "schema", line_number)?,
            origin: manifest_string(&row, "origin", line_number)?,
            source_path: manifest_string(&row, "source_path", line_number)?,
            source_chunk_kind: manifest_string(&row, "source_chunk_kind", line_number)?,
            unreal_import_relation: manifest_string(&row, "unreal_import_relation", line_number)?,
            future_normalization: manifest_string(&row, "future_normalization", line_number)?,
        });
    }
    let report = parallel_source_evidence(
        &inputs,
        mission_references,
        mission_p3d_references,
    )?;
    preflight_cross_source_mission_locators(
        &inputs,
        &report.evidence,
        mission_references,
        mission_cameras,
        mission_locators,
        mission_p3d_references,
        index,
        &config.extracted_root,
    )?;
    Ok(report)
}

/// Verified source evidence plus selected mission-definition rows.
struct SourceEvidenceReport {
    evidence: Vec<UnrealSourceEvidence>,
    mission_definitions: Vec<String>,
}

/// One verified physical source and its optional selected mission definition.
struct VerifiedSourceOutput {
    evidence: UnrealSourceEvidence,
    mission_definition: Option<String>,
}

/// One parsed manifest row awaiting physical source verification.
#[derive(Debug)]
struct SourceEvidenceInput {
    id: String,
    path: String,
    resolved: PathBuf,
    expected_size: u64,
    file_extension: String,
    unit_type: String,
    subtype: String,
    kind: String,
    function: String,
    schema: String,
    origin: String,
    source_path: String,
    source_chunk_kind: String,
    unreal_import_relation: String,
    future_normalization: String,
}

/// Verify prepared rows concurrently and restore manifest ordering.
fn parallel_source_evidence(
    inputs: &[SourceEvidenceInput],
    mission_references: &MissionReferenceCatalog,
    mission_p3d_references: &MissionP3dReferenceCatalog,
) -> PipelineOutcome<SourceEvidenceReport> {
    if inputs.is_empty() {
        return Ok(SourceEvidenceReport {
            evidence: Vec::new(),
            mission_definitions: Vec::new(),
        });
    }
    let next = AtomicUsize::new(0);
    let (sender, receiver) = mpsc::channel();
    let workers = source_worker_count(inputs.len());
    let mut progress = StageProgress::begin("Unreal source evidence", inputs.len());
    let mut collected = Vec::with_capacity(inputs.len());
    thread::scope(|scope| {
        for _worker in 0..workers {
            let worker_sender = sender.clone();
            let worker_next = &next;
            let worker_inputs = inputs;
            let worker_mission_references = mission_references;
            let worker_mission_p3d_references = mission_p3d_references;
            let _handle = scope.spawn(move || {
                loop {
                    let position = worker_next.fetch_add(1, Ordering::Relaxed);
                    let Some(input) = worker_inputs.get(position) else {
                        break;
                    };
                    let result = read_source_evidence(
                        input,
                        worker_mission_references,
                        worker_mission_p3d_references,
                    );
                    if worker_sender
                        .send((position, input.id.clone(), result))
                        .is_err()
                    {
                        break;
                    }
                }
            });
        }
        drop(sender);
        while let Ok((position, id, result)) = receiver.recv() {
            progress.advance(&id);
            collected.push((position, result));
        }
    });
    if collected.len() != inputs.len() {
        return Err(PipelineError::new(format!(
            "Unreal source workers returned {} of {} rows",
            collected.len(),
            inputs.len()
        )));
    }
    collected.sort_by_key(|(position, _result)| *position);
    let mut evidence = Vec::with_capacity(collected.len());
    let mut mission_definitions = Vec::new();
    for (_position, result) in collected {
        let output = result?;
        evidence.push(output.evidence);
        if let Some(definition) = output.mission_definition {
            mission_definitions.push(definition);
        }
    }
    progress.finish();
    Ok(SourceEvidenceReport {
        evidence,
        mission_definitions,
    })
}

/// Re-bind typed mission locator references across the exact source snapshot
/// that was already hashed for Unreal import planning.
fn preflight_cross_source_mission_locators(
    inputs: &[SourceEvidenceInput],
    verified: &[UnrealSourceEvidence],
    mission_references: &MissionReferenceCatalog,
    mission_cameras: &MissionCameraCatalog,
    mission_locators: &MissionLocatorCatalog,
    mission_p3d_references: &MissionP3dReferenceCatalog,
    index: &PhaseThreePackageIndex,
    extracted_root: &Path,
) -> PipelineOutcome<()> {
    let mut verified_by_id = BTreeMap::new();
    for source in verified {
        if verified_by_id.insert(source.id.as_str(), source).is_some() {
            return Err(PipelineError::new(
                "mission locator source verification duplicated an id",
            ));
        }
    }

    let mut snapshots = Vec::new();
    for input in inputs {
        if input.kind != "mission-script" {
            continue;
        }
        let source = verified_by_id.get(input.id.as_str()).ok_or_else(|| {
            PipelineError::new("mission locator source is missing from verified evidence")
        })?;
        if source.source_path != input.source_path {
            return Err(PipelineError::new(
                "mission locator source provenance changed after verification",
            ));
        }
        let bytes = read_stable_source_bytes(&input.resolved)?;
        let actual_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if actual_size != input.expected_size || actual_size != source.size_bytes {
            return Err(PipelineError::new(format!(
                "mission locator source size changed after verification for {}",
                input.path
            )));
        }
        if digest_hex(&bytes) != source.sha256 {
            return Err(PipelineError::new(format!(
                "mission locator source digest changed after verification for {}",
                input.path
            )));
        }
        let text = std::str::from_utf8(&bytes).map_err(|_error| {
            PipelineError::new("mission locator source is not valid UTF-8 after verification")
        })?;
        let evidence = preflight_mission_script(text).map_err(|error| {
            PipelineError::new(format!(
                "mission locator source semantic preflight failed: {error}"
            ))
        })?;
        let loads = preflight_mission_package_loads_with_catalog(
            &evidence,
            mission_p3d_references,
        )
        .map_err(|error| {
            PipelineError::new(format!(
                "mission locator source package-load preflight failed: {error}"
            ))
        })?;
        let package_roots = loads
            .bindings()
            .iter()
            .map(|binding| binding.package_root().to_owned())
            .collect();
        snapshots.push(MissionLocatorScriptSnapshot::new(
            input.source_path.clone(),
            evidence,
            package_roots,
        ));
    }

    drop(
        build_mission_order_source_reports(&snapshots).map_err(|error| {
            PipelineError::new(format!(
                "mission authored registration preflight failed: {error}"
            ))
        })?,
    );
    drop(preflight_mission_music_states(
        index,
        extracted_root,
        &snapshots,
    )?);
    drop(completion_dialog_context::preflight_mission_completion_dialogs(
        index,
        mission_references,
        &snapshots,
    )?);
    drop(dialogue_info_context::preflight_mission_dialogue_info(
        index,
        mission_references,
        &snapshots,
    )?);

    let indexed_package_roots = index
        .packages()
        .iter()
        .map(|package| package.package_root.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let level_contexts = build_level_locator_source_contexts(&snapshots)
        .map_err(|error| {
            PipelineError::new(format!(
                "level locator sibling-load context failed: {error}"
            ))
        })?;
    for snapshot in &snapshots {
        let Some(level_context) =
            level_contexts.get(snapshot.source_path())
        else {
            continue;
        };
        let scopes = compile_mission_scope_graphs(snapshot.evidence())
            .map_err(|error| {
                PipelineError::new(format!(
                    "level locator scope preflight failed: {error}"
                ))
            })?;
        let npcs = preflight_mission_level_npcs(mission_references, &scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "level locator NPC preflight failed: {error}"
                ))
            })?;
        let purchases = preflight_mission_purchase_rewards(
            mission_references,
            &scopes,
        )
        .map_err(|error| {
            PipelineError::new(format!(
                "level locator storefront preflight failed: {error}"
            ))
        })?;
        drop(
            preflight_mission_level_locator_references(
                mission_locators,
                level_context.package_roots(),
                &scopes,
                &npcs,
                &purchases,
            )
            .map_err(|error| {
                PipelineError::new(format!(
                    "level locator reference preflight failed via {}: {error}",
                    level_context.load_source_path(),
                ))
            })?,
        );
    }
    let contexts = build_mission_locator_source_contexts(&snapshots, &indexed_package_roots)
        .map_err(|error| {
            PipelineError::new(format!(
                "mission locator active-package context failed: {error}"
            ))
        })?;
    for snapshot in &snapshots {
        let Some(context) = contexts.get(snapshot.source_path()) else {
            continue;
        };
        let scopes = compile_mission_scope_graphs(snapshot.evidence()).map_err(|error| {
            PipelineError::new(format!("mission locator scope preflight failed: {error}"))
        })?;
        let initialization = preflight_mission_initialization(&scopes).map_err(|error| {
            PipelineError::new(format!(
                "mission locator initialization preflight failed: {error}"
            ))
        })?;
        let has_ped_group_selection = initialization
            .missions()
            .iter()
            .flat_map(|mission| mission.directives())
            .any(|directive| matches!(
                directive,
                crate::domain::MissionInitializationDirective::PedGroup { .. }
            ));
        if has_ped_group_selection {
            let setup_path = context.level_setup_source_path().ok_or_else(|| {
                PipelineError::new(
                    "mission pedestrian-group setup source is missing",
                )
            })?;
            let setup = snapshots
                .iter()
                .find(|candidate| candidate.source_path() == setup_path)
                .ok_or_else(|| {
                    PipelineError::new(
                        "mission pedestrian-group setup source disappeared",
                    )
                })?;
            let setup_scopes = compile_mission_scope_graphs(setup.evidence())
                .map_err(|error| {
                    PipelineError::new(format!(
                        "mission pedestrian-group setup scope failed: {error}"
                    ))
                })?;
            let groups = preflight_mission_ped_groups(
                mission_references,
                &setup_scopes,
            )
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission pedestrian-group setup preflight failed: {error}"
                ))
            })?;
            for mission in initialization.missions() {
                drop(
                    preflight_mission_ped_group_selections(mission, &groups)
                        .map_err(|error| {
                            PipelineError::new(format!(
                                concat!(
                                    "mission pedestrian-group ",
                                    "selection failed: {}"
                                ),
                                error
                            ))
                        })?,
                );
            }
        }
        drop(
            preflight_mission_camera_references(
                snapshot.source_path(),
                mission_cameras,
                &initialization,
            )
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission camera reference preflight failed: {error}"
                ))
            })?,
        );
        let stage_semantics =
            preflight_mission_stage_semantics(&scopes).map_err(|error| {
                PipelineError::new(format!(
                    "mission locator stage preflight failed: {error}"
                ))
            })?;
        drop(
            preflight_mission_stage_message_references(index, &stage_semantics)
                .map_err(|error| {
                    PipelineError::new(format!(
                        concat!(
                            "mission stage-message reference ",
                            "preflight failed: {}"
                        ),
                        error
                    ))
                })?,
        );
        let objective_semantics =
            preflight_mission_objective_semantics(&scopes).map_err(|error| {
                PipelineError::new(format!(
                    "mission locator objective preflight failed: {error}"
                ))
            })?;
        drop(
            preflight_mission_fmv_references(index, &objective_semantics)
                .map_err(|error| {
                    PipelineError::new(format!(
                        "mission FMV reference preflight failed: {error}"
                    ))
                })?,
        );
        drop(
            preflight_mission_locator_references(
                mission_locators,
                context.active_packages(),
                &scopes,
                &initialization,
                &stage_semantics,
                &objective_semantics,
            )
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission locator reference preflight failed: {error}"
                ))
            })?,
        );
    }
    Ok(())
}

/// Read, validate, and hash one physical source row.
fn read_source_evidence(
    input: &SourceEvidenceInput,
    mission_references: &MissionReferenceCatalog,
    mission_p3d_references: &MissionP3dReferenceCatalog,
) -> PipelineOutcome<VerifiedSourceOutput> {
    let (actual_size, sha256, mission_definition) =
        if input.kind == "mission-script" {
        let bytes = read_stable_source_bytes(&input.resolved)?;
        let actual_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        let mission_definition = validate_normalized_mission_source(
            &input.id,
            &input.kind,
            &input.schema,
            &input.file_extension,
            &input.origin,
            &bytes,
            mission_references,
            mission_p3d_references,
        )?;
        (actual_size, digest_hex(&bytes), mission_definition)
    } else {
        let (actual_size, sha256) = stream_source_digest(&input.resolved)?;
        (actual_size, sha256, None)
    };
    if actual_size != input.expected_size {
        return Err(PipelineError::new(format!(
            "source size changed for {}: manifest={} actual={actual_size}",
            input.path, input.expected_size
        )));
    }
    Ok(VerifiedSourceOutput {
        evidence: UnrealSourceEvidence {
            id: input.id.clone(),
        path: input.path.clone(),
        file_extension: input.file_extension.clone(),
        unit_type: input.unit_type.clone(),
        subtype: input.subtype.clone(),
        kind: input.kind.clone(),
        function: input.function.clone(),
        schema: input.schema.clone(),
        origin: input.origin.clone(),
        source_path: input.source_path.clone(),
        source_chunk_kind: input.source_chunk_kind.clone(),
        size_bytes: actual_size,
        sha256,
        unreal_import_relation: input.unreal_import_relation.clone(),
            future_normalization: input.future_normalization.clone(),
        },
        mission_definition,
    })
}

/// Physical identity and mutable metadata captured from one open source.
struct StableSourceIdentity {
    handle: Handle,
    len: u64,
    modified: Option<SystemTime>,
}

/// Open one source only after the shared filesystem boundary proves it is a
/// regular non-linked file, then bind the path to the exact open descriptor.
fn open_stable_source(path: &Path) -> PipelineOutcome<(fs::File, StableSourceIdentity)> {
    let kind = local_path_kind(path).map_err(path_error("inspect Unreal source evidence"))?;
    if kind != PathKind::File {
        return Err(PipelineError::new(
            "Unreal source evidence is not a regular non-linked file",
        ));
    }
    let file = fs::File::open(path).map_err(path_error("read Unreal source evidence"))?;
    let descriptor = file
        .try_clone()
        .and_then(Handle::from_file)
        .map_err(path_error("identify open Unreal source evidence"))?;
    let path_handle =
        Handle::from_path(path).map_err(path_error("identify Unreal source evidence path"))?;
    if descriptor != path_handle {
        return Err(PipelineError::new(
            "Unreal source evidence changed while opening",
        ));
    }
    let metadata = file
        .metadata()
        .map_err(path_error("inspect open Unreal source evidence"))?;
    let identity = StableSourceIdentity {
        handle: descriptor,
        len: metadata.len(),
        modified: metadata.modified().ok(),
    };
    Ok((file, identity))
}

/// Require the path, descriptor identity, size, and modification time to remain
/// stable across one complete source read.
fn verify_stable_source(
    path: &Path,
    file: &fs::File,
    initial: &StableSourceIdentity,
) -> PipelineOutcome<()> {
    let kind =
        local_path_kind(path).map_err(path_error("inspect Unreal source evidence after read"))?;
    if kind != PathKind::File {
        return Err(PipelineError::new(
            "Unreal source evidence changed during verification",
        ));
    }
    let final_path = Handle::from_path(path)
        .map_err(path_error("identify Unreal source evidence after read"))?;
    if final_path != initial.handle {
        return Err(PipelineError::new(
            "Unreal source evidence identity changed during verification",
        ));
    }
    let metadata = file
        .metadata()
        .map_err(path_error("inspect open Unreal source evidence after read"))?;
    if metadata.len() != initial.len || metadata.modified().ok() != initial.modified {
        return Err(PipelineError::new(
            "Unreal source evidence metadata changed during verification",
        ));
    }
    Ok(())
}

/// Read one mission source completely while preserving the same stable-source
/// identity checks used by streamed source hashing.
#[expect(
    clippy::verbose_file_reads,
    reason = "fs::read would reopen the path and discard the descriptor identity               that must remain stable across verification."
)]
fn read_stable_source_bytes(path: &Path) -> PipelineOutcome<Vec<u8>> {
    let (mut file, identity) = open_stable_source(path)?;
    let mut bytes = Vec::new();
    let _read = file
        .read_to_end(&mut bytes)
        .map_err(path_error("read Unreal source evidence"))?;
    verify_stable_source(path, &file, &identity)?;
    if u64::try_from(bytes.len()).unwrap_or(u64::MAX) != identity.len {
        return Err(PipelineError::new(
            "Unreal source evidence size changed during verification",
        ));
    }
    Ok(bytes)
}

/// Stream one non-mission source into the shared SHA-256 boundary without
/// retaining the complete payload in memory.
fn stream_source_digest(path: &Path) -> PipelineOutcome<(u64, String)> {
    const BUFFER_BYTES: usize = 1024 * 1024;
    let (mut file, identity) = open_stable_source(path)?;
    let mut digest = Sha256::new();
    let mut total = 0_u64;
    let mut buffer = vec![0_u8; BUFFER_BYTES].into_boxed_slice();
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(path_error("read Unreal source evidence"))?;
        if read == 0 {
            break;
        }
        let chunk = buffer
            .get(..read)
            .ok_or_else(|| PipelineError::new("Unreal source read exceeded its bounded buffer"))?;
        digest.update(chunk);
        total = total
            .checked_add(u64::try_from(read).unwrap_or(u64::MAX))
            .ok_or_else(|| PipelineError::new("Unreal source byte count overflowed"))?;
    }
    verify_stable_source(path, &file, &identity)?;
    if total != identity.len {
        return Err(PipelineError::new(
            "Unreal source evidence size changed during verification",
        ));
    }
    Ok((total, digest.finalize_hex()))
}

/// Bound physical source verification workers for this machine.
fn source_worker_count(source_count: usize) -> usize {
    let available = thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    source_worker_count_for(available, source_count)
}

/// Calculate the bounded worker count from explicit machine capacity.
fn source_worker_count_for(available: usize, source_count: usize) -> usize {
    available
        .saturating_mul(2)
        .checked_div(3)
        .unwrap_or(1)
        .clamp(1, 8)
        .min(source_count.max(1))
}

fn validate_normalized_mission_source(
    source_id: &str,
    kind: &str,
    schema: &str,
    file_extension: &str,
    origin: &str,
    bytes: &[u8],
    mission_references: &MissionReferenceCatalog,
    mission_p3d_references: &MissionP3dReferenceCatalog,
) -> PipelineOutcome<Option<String>> {
    if kind != "mission-script" {
        return Ok(None);
    }
    if schema != MISSION_SCRIPT_SCHEMA {
        return Err(PipelineError::new(
            "normalized mission source schema is stale",
        ));
    }
    if file_extension != "json" || origin != "game-straggler-normalize" {
        return Err(PipelineError::new(
            "normalized mission source routing identity is invalid",
        ));
    }
    let text = std::str::from_utf8(bytes)
        .map_err(|_error| PipelineError::new("normalized mission source is not valid UTF-8"))?;
    let evidence = preflight_mission_script(text).map_err(|error| {
        PipelineError::new(format!("mission semantic preflight failed: {error}"))
    })?;
    drop(
        preflight_mission_vehicle_selects(
            &evidence,
            mission_references,
            mission_p3d_references,
        )
        .map_err(|error| {
            PipelineError::new(format!(
                "mission vehicle-select preflight failed: {error}"
            ))
        })?,
    );
    drop(
        preflight_mission_package_loads_with_catalog(
            &evidence,
            mission_p3d_references,
        )
        .map_err(|error| {
            PipelineError::new(format!("mission package-load preflight failed: {error}"))
        })?,
    );
    drop(preflight_mission_objectives(&evidence).map_err(|error| {
        PipelineError::new(format!("mission objective preflight failed: {error}"))
    })?);
    drop(
        preflight_mission_objective_commands(&evidence).map_err(|error| {
            PipelineError::new(format!(
                "mission objective command preflight failed: {error}"
            ))
        })?,
    );
    drop(preflight_mission_conditions(&evidence).map_err(|error| {
        PipelineError::new(format!("mission condition preflight failed: {error}"))
    })?);
    drop(
        preflight_mission_condition_commands(&evidence).map_err(|error| {
            PipelineError::new(format!(
                "mission condition command preflight failed: {error}"
            ))
        })?,
    );
    let scopes = compile_mission_scope_graphs(&evidence)
        .map_err(|error| PipelineError::new(format!("mission scope preflight failed: {error}")))?;
    let objective_semantics = preflight_mission_objective_semantics(&scopes).map_err(|error| {
        PipelineError::new(format!(
            "mission objective semantic preflight failed: {error}"
        ))
    })?;
    let condition_semantics = preflight_mission_condition_semantics(&scopes).map_err(|error| {
        PipelineError::new(format!(
            "mission condition semantic preflight failed: {error}"
        ))
    })?;
    let initialization = preflight_mission_initialization(&scopes).map_err(|error| {
        PipelineError::new(format!("mission initialization preflight failed: {error}"))
    })?;
    let stage_semantics = preflight_mission_stage_semantics(&scopes).map_err(|error| {
        PipelineError::new(format!("mission stage semantic preflight failed: {error}"))
    })?;
    let topology =
        preflight_mission_authored_stage_topology(&stage_semantics)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission authored stage topology failed: {error}"
                ))
            })?;
    let mission_definition =
        mission_definition_context::preflight_mission_definition_core(
            &scopes,
            &initialization,
            &stage_semantics,
            &objective_semantics,
            &condition_semantics,
            &topology,
        )?
        .map(|definition| {
            mission_definition_context::render_definition_core(
                source_id,
                &definition,
            )
        })
        .transpose()?;
    drop(
        preflight_mission_presentation_references(
            mission_p3d_references,
            &initialization,
            &stage_semantics,
            &objective_semantics,
        )
        .map_err(|error| {
            PipelineError::new(format!(
                "mission presentation reference preflight failed: {error}"
            ))
        })?,
    );
    let reward_references =
        preflight_mission_reward_references(mission_p3d_references, &scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission reward reference preflight failed: {error}"
                ))
            })?;
    drop(
        preflight_mission_reward_offers(&reward_references).map_err(|error| {
            PipelineError::new(format!(
                "mission reward offer preflight failed: {error}"
            ))
        })?,
    );
    drop(
        preflight_mission_vehicle_attributes(mission_references, &scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission vehicle attribute preflight failed: {error}"
                ))
            })?,
    );
    drop(
        preflight_mission_gag_totals(&scopes).map_err(|error| {
            PipelineError::new(format!(
                "mission gag total preflight failed: {error}"
            ))
        })?,
    );
    drop(
        preflight_mission_purchase_rewards(mission_references, &scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission purchase reward preflight failed: {error}"
                ))
            })?,
    );
    drop(
        preflight_mission_level_npcs(mission_references, &scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission level NPC preflight failed: {error}"
                ))
            })?,
    );
    drop(
        preflight_mission_ped_groups(mission_references, &scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission pedestrian group preflight failed: {error}"
                ))
            })?,
    );
    drop(
        preflight_mission_traffic_groups(mission_references, &scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission traffic group preflight failed: {error}"
                ))
            })?,
    );
    drop(
        preflight_mission_references(
            mission_references,
            &scopes,
            &objective_semantics,
            &condition_semantics,
            &initialization,
            &stage_semantics,
        )
        .map_err(|error| {
            PipelineError::new(format!(
                "mission participant reference preflight failed: {error}"
            ))
        })?,
    );
    Ok(mission_definition)
}

fn validate_mission_definition_bundle(
    rows: &[String],
    verified: &[UnrealSourceEvidence],
) -> PipelineOutcome<String> {
    let verified_mission_sources = verified
        .iter()
        .enumerate()
        .filter(|(_index, source)| source.kind == "mission-script")
        .map(|(index, source)| (source.id.as_str(), index))
        .collect::<BTreeMap<_, _>>();
    let mut source_ids = BTreeSet::new();
    let mut previous_source_position = None;
    let mut output = String::new();
    for (index, row) in rows.iter().enumerate() {
        if !row.ends_with(char::from(10)) || row.lines().count() != 1 {
            return Err(PipelineError::new(
                "mission definition row is not one canonical JSONL record",
            ));
        }
        let label = format!("mission definition row {}", index + 1);
        let object = parse_object(
            row.trim_end_matches(char::from(10)),
            &label,
        )?;
        if required_string(&object, "schema", &label)?
            != mission_definition_context::MISSION_DEFINITION_CORE_SCHEMA
        {
            return Err(PipelineError::new(
                "mission definition row has a noncanonical schema",
            ));
        }
        let source_id = required_string(&object, "source_id", &label)?;
        validate_public_identifier(
            &source_id,
            "mission definition source id",
        )?;
        if !source_ids.insert(source_id.clone()) {
            return Err(PipelineError::new(
                "mission definition bundle duplicates a source id",
            ));
        }
        let source_position = verified_mission_sources
            .get(source_id.as_str())
            .copied()
            .ok_or_else(|| {
                PipelineError::new(concat!(
                    "mission definition source is not verified ",
                    "mission evidence"
                ))
            })?;
        if previous_source_position
            .is_some_and(|previous| source_position <= previous)
        {
            return Err(PipelineError::new(
                "mission definition bundle is not in verified source order",
            ));
        }
        previous_source_position = Some(source_position);
        let mission_id = required_string(&object, "mission_id", &label)?;
        validate_mission_id(&mission_id)?;
        let stages = object
            .get("stages")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                PipelineError::new(
                    "mission definition row is missing its stage array",
                )
            })?;
        validate_mission_definition_stages(stages, &label)?;
        let canonical = serde_json::to_string(&Value::Object(object))
            .map_err(|_error| {
                PipelineError::new(
                    "mission definition canonical serialization failed",
                )
            })?;
        if canonical != row.trim_end_matches(char::from(10)) {
            return Err(PipelineError::new(
                "mission definition row is not canonical JSON",
            ));
        }
        output.push_str(row);
    }
    Ok(output)
}

fn validate_mission_definition_stages(
    stages: &[Value],
    label: &str,
) -> PipelineOutcome<()> {
    if stages.is_empty() {
        return Err(PipelineError::new(format!(
            "{label} has no authored stages"
        )));
    }
    let last_index = stages.len() - 1;
    let mut previous_source_ordinal = None;
    for (index, value) in stages.iter().enumerate() {
        let stage_label = format!("{label} stage {}", index + 1);
        let stage = value.as_object().ok_or_else(|| {
            PipelineError::new(format!("{stage_label} is not an object"))
        })?;
        let sequence_ordinal =
            required_u64(stage, "sequence_ordinal", &stage_label)?;
        let expected_sequence = u64::try_from(index).unwrap_or(u64::MAX);
        if sequence_ordinal != expected_sequence {
            return Err(PipelineError::new(format!(
                "{stage_label} sequence ordinal is not dense"
            )));
        }
        let source_ordinal =
            required_u64(stage, "stage_source_ordinal", &stage_label)?;
        if previous_source_ordinal
            .is_some_and(|previous| source_ordinal <= previous)
        {
            return Err(PipelineError::new(format!(
                "{stage_label} source ordinal is not strictly increasing"
            )));
        }
        previous_source_ordinal = Some(source_ordinal);

        let expected_next = (index < last_index)
            .then(|| u64::try_from(index + 1).unwrap_or(u64::MAX));
        let actual_next = match stage.get("next_authored_sequence_ordinal") {
            Some(Value::Null) => None,
            Some(value) => Some(value.as_u64().ok_or_else(|| {
                PipelineError::new(format!(
                    "{stage_label} has invalid authored neighbor"
                ))
            })?),
            None => {
                return Err(PipelineError::new(format!(
                    "{stage_label} is missing authored neighbor"
                )));
            },
        };
        if actual_next != expected_next {
            return Err(PipelineError::new(format!(
                "{stage_label} authored neighbor drifted"
            )));
        }

        let explicit_final = stage
            .get("explicit_final")
            .and_then(Value::as_bool)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "{stage_label} is missing boolean field explicit_final"
                ))
            })?;
        if explicit_final && index != last_index {
            return Err(PipelineError::new(format!(
                "{stage_label} marks a nonterminal authored stage final"
            )));
        }
        validate_mission_definition_stage_kind(
            stage,
            explicit_final,
            &stage_label,
        )?;
        let objective_source_ordinal = validate_mission_definition_objective(
            stage,
            source_ordinal,
            &stage_label,
        )?;
        validate_mission_definition_conditions(
            stage,
            source_ordinal,
            objective_source_ordinal,
            &stage_label,
        )?;
        validate_mission_definition_checkpoint(
            stage,
            source_ordinal,
            &stage_label,
        )?;
        validate_mission_definition_countdown(
            stage,
            source_ordinal,
            sequence_ordinal,
            &stage_label,
        )?;
        validate_mission_definition_objective_bindings(
            stage,
            source_ordinal,
            sequence_ordinal,
            objective_source_ordinal,
            &stage_label,
        )?;
        let terminal = required_string(stage, "terminal", &stage_label)?;
        if !matches!(
            terminal.as_str(),
            "none" | "chapter-transition" | "game-completion"
        ) {
            return Err(PipelineError::new(format!(
                "{stage_label} has unknown terminal classification"
            )));
        }
        if terminal != "none" && index != last_index {
            return Err(PipelineError::new(format!(
                "{stage_label} has terminal outcome before the final stage"
            )));
        }

        for field in [
            "successor_sequence_ordinal",
            "success_transition_id",
            "failure_transition_id",
            "retry_sequence_ordinal",
            "retry_transition_id",
            "rollback_sequence_ordinal",
            "rollback_transition_id",
            "recovery_sequence_ordinal",
            "recovery_transition_id",
        ] {
            if stage.contains_key(field) {
                return Err(PipelineError::new(format!(
                    "{stage_label} invents unresolved runtime field {field}"
                )));
            }
        }
    }
    Ok(())
}

fn validate_mission_definition_stage_kind(
    stage: &Map<String, Value>,
    explicit_final: bool,
    stage_label: &str,
) -> PipelineOutcome<()> {
    let kind = stage
        .get("kind")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{stage_label} is missing its stage kind"
            ))
        })?;
    let token = required_string(kind, "kind", stage_label)?;
    let kind_final = match token.as_str() {
        "standard" => kind
            .get("final_stage")
            .and_then(Value::as_bool)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "{stage_label} standard kind is missing final_stage"
                ))
            })?,
        "locked-vehicle" | "locked-costume" => false,
        _ => {
            return Err(PipelineError::new(format!(
                "{stage_label} has unknown stage kind"
            )));
        },
    };
    if kind_final != explicit_final {
        return Err(PipelineError::new(format!(
            "{stage_label} final marker disagrees with stage kind"
        )));
    }
    Ok(())
}

fn validate_mission_definition_objective(
    stage: &Map<String, Value>,
    stage_source_ordinal: u64,
    stage_label: &str,
) -> PipelineOutcome<u64> {
    let objective = stage
        .get("objective")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{stage_label} is missing its objective"
            ))
        })?;
    let objective_source_ordinal =
        required_u64(objective, "source_ordinal", stage_label)?;
    let source_alias = required_string(objective, "source_alias", stage_label)?;
    if objective_source_ordinal <= stage_source_ordinal || source_alias.is_empty() {
        return Err(PipelineError::new(format!(
            "{stage_label} objective identity is malformed"
        )));
    }
    let canonical_kind = optional_nonempty_string(
        objective,
        "canonical_kind",
        stage_label,
    )?;
    let unavailable_code = optional_nonempty_string(
        objective,
        "unavailable_code",
        stage_label,
    )?;
    if canonical_kind.is_some() == unavailable_code.is_some() {
        return Err(PipelineError::new(format!(
            "{stage_label} objective mapping is not exclusive"
        )));
    }
    Ok(objective_source_ordinal)
}

fn validate_mission_definition_conditions(
    stage: &Map<String, Value>,
    stage_source_ordinal: u64,
    objective_source_ordinal: u64,
    stage_label: &str,
) -> PipelineOutcome<()> {
    let conditions = stage
        .get("conditions")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{stage_label} is missing its condition array"
            ))
        })?;
    let mut previous_source_ordinal = None;
    for (index, value) in conditions.iter().enumerate() {
        let condition_label = format!("{stage_label} condition {}", index + 1);
        let condition = value.as_object().ok_or_else(|| {
            PipelineError::new(format!("{condition_label} is not an object"))
        })?;
        let source_ordinal =
            required_u64(condition, "source_ordinal", &condition_label)?;
        if source_ordinal <= stage_source_ordinal
            || previous_source_ordinal
                .is_some_and(|previous| source_ordinal <= previous)
        {
            return Err(PipelineError::new(format!(
                "{condition_label} source ordinal is malformed"
            )));
        }
        previous_source_ordinal = Some(source_ordinal);
        if required_string(condition, "source_alias", &condition_label)?
            .is_empty()
            || required_string(condition, "schema_id", &condition_label)?
                .is_empty()
        {
            return Err(PipelineError::new(format!(
                "{condition_label} identity is malformed"
            )));
        }
        let owner = condition.get("owner_objective_source_ordinal");
        match required_string(condition, "scope", &condition_label)?.as_str() {
            "stage" => {
                if !matches!(owner, Some(Value::Null)) {
                    return Err(PipelineError::new(format!(
                        "{condition_label} stage scope owns an objective"
                    )));
                }
            },
            "objective" => {
                if owner.and_then(Value::as_u64) != Some(objective_source_ordinal) {
                    return Err(PipelineError::new(format!(
                        "{condition_label} objective owner drifted"
                    )));
                }
            },
            _ => {
                return Err(PipelineError::new(format!(
                    "{condition_label} has unknown condition scope"
                )));
            },
        }
        if required_string(condition, "violation_effect", &condition_label)?
            != "stage-failure"
        {
            return Err(PipelineError::new(format!(
                "{condition_label} has unknown violation effect"
            )));
        }
    }
    Ok(())
}

fn validate_mission_definition_checkpoint(
    stage: &Map<String, Value>,
    stage_source_ordinal: u64,
    stage_label: &str,
) -> PipelineOutcome<()> {
    let checkpoint = stage.get("checkpoint_source_ordinal").ok_or_else(|| {
        PipelineError::new(format!(
            "{stage_label} is missing checkpoint source ordinal"
        ))
    })?;
    if !checkpoint.is_null()
        && checkpoint.as_u64().is_none_or(|value| value <= stage_source_ordinal)
    {
        return Err(PipelineError::new(format!(
            "{stage_label} checkpoint source ordinal is malformed"
        )));
    }
    Ok(())
}

fn validate_mission_definition_countdown(
    stage: &Map<String, Value>,
    stage_source_ordinal: u64,
    stage_sequence_ordinal: u64,
    stage_label: &str,
) -> PipelineOutcome<()> {
    let Some(value) = stage.get("countdown") else {
        return Err(PipelineError::new(format!(
            "{stage_label} is missing countdown evidence"
        )));
    };
    if value.is_null() {
        return Ok(());
    }
    let countdown = value.as_object().ok_or_else(|| {
        PipelineError::new(format!("{stage_label} countdown is not an object"))
    })?;
    if required_u64(countdown, "stage_source_ordinal", stage_label)?
        != stage_source_ordinal
        || required_u64(countdown, "stage_sequence_ordinal", stage_label)?
            != stage_sequence_ordinal
    {
        return Err(PipelineError::new(format!(
            "{stage_label} countdown owner drifted"
        )));
    }
    let start_source_ordinal =
        required_u64(countdown, "start_source_ordinal", stage_label)?;
    if start_source_ordinal <= stage_source_ordinal
        || required_string(countdown, "sequence_id", stage_label)?.is_empty()
    {
        return Err(PipelineError::new(format!(
            "{stage_label} countdown identity is malformed"
        )));
    }
    let entries = countdown
        .get("entries")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{stage_label} countdown is missing entry array"
            ))
        })?;
    let mut previous_source_ordinal = start_source_ordinal;
    for (index, value) in entries.iter().enumerate() {
        let entry_label = format!("{stage_label} countdown entry {}", index + 1);
        let entry = value.as_object().ok_or_else(|| {
            PipelineError::new(format!("{entry_label} is not an object"))
        })?;
        let source_ordinal = required_u64(entry, "source_ordinal", &entry_label)?;
        if source_ordinal <= previous_source_ordinal
            || required_string(entry, "token", &entry_label)?.is_empty()
            || required_u64(entry, "duration_milliseconds", &entry_label)? == 0
        {
            return Err(PipelineError::new(format!(
                "{entry_label} identity or order is malformed"
            )));
        }
        previous_source_ordinal = source_ordinal;
    }
    Ok(())
}

fn validate_mission_definition_objective_bindings(
    stage: &Map<String, Value>,
    stage_source_ordinal: u64,
    stage_sequence_ordinal: u64,
    objective_source_ordinal: u64,
    stage_label: &str,
) -> PipelineOutcome<()> {
    validate_mission_definition_collectible_waypoints(
        stage,
        stage_source_ordinal,
        stage_sequence_ordinal,
        objective_source_ordinal,
        stage_label,
    )?;
    validate_mission_definition_npc_waypoints(
        stage,
        stage_source_ordinal,
        stage_sequence_ordinal,
        objective_source_ordinal,
        stage_label,
    )?;
    validate_mission_definition_pickup_state_props(
        stage,
        stage_source_ordinal,
        stage_sequence_ordinal,
        objective_source_ordinal,
        stage_label,
    )?;
    Ok(())
}

fn validate_mission_definition_binding_owner(
    binding: &Map<String, Value>,
    stage_source_ordinal: u64,
    stage_sequence_ordinal: u64,
    objective_source_ordinal: u64,
    label: &str,
) -> PipelineOutcome<()> {
    if required_u64(binding, "stage_source_ordinal", label)?
        != stage_source_ordinal
        || required_u64(binding, "stage_sequence_ordinal", label)?
            != stage_sequence_ordinal
        || required_u64(binding, "objective_source_ordinal", label)?
            != objective_source_ordinal
    {
        return Err(PipelineError::new(format!("{label} owner drifted")));
    }
    Ok(())
}

fn validate_mission_definition_collectible_waypoints(
    stage: &Map<String, Value>,
    stage_source_ordinal: u64,
    stage_sequence_ordinal: u64,
    objective_source_ordinal: u64,
    stage_label: &str,
) -> PipelineOutcome<()> {
    let bindings = stage
        .get("collectible_waypoints")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{stage_label} is missing collectible waypoint array"
            ))
        })?;
    let mut previous_source_ordinal = None;
    for (index, value) in bindings.iter().enumerate() {
        let label = format!(
            "{stage_label} collectible waypoint {}",
            index + 1
        );
        let binding = value.as_object().ok_or_else(|| {
            PipelineError::new(format!("{label} is not an object"))
        })?;
        validate_mission_definition_binding_owner(
            binding,
            stage_source_ordinal,
            stage_sequence_ordinal,
            objective_source_ordinal,
            &label,
        )?;
        let source_ordinal = required_u64(binding, "source_ordinal", &label)?;
        let collectible_source_ordinal =
            required_u64(binding, "collectible_source_ordinal", &label)?;
        let waypoint_source_ordinal =
            required_u64(binding, "waypoint_source_ordinal", &label)?;
        let _ = required_u64(binding, "collectible_index", &label)?;
        let _ = required_u64(binding, "waypoint_index", &label)?;
        if source_ordinal <= objective_source_ordinal
            || collectible_source_ordinal >= source_ordinal
            || waypoint_source_ordinal >= source_ordinal
            || previous_source_ordinal
                .is_some_and(|previous| source_ordinal <= previous)
            || required_string(binding, "collectible_locator_id", &label)?
                .is_empty()
            || required_string(binding, "waypoint_locator_id", &label)?
                .is_empty()
        {
            return Err(PipelineError::new(format!(
                "{label} relationship is malformed"
            )));
        }
        previous_source_ordinal = Some(source_ordinal);
    }
    Ok(())
}

fn validate_mission_definition_npc_waypoints(
    stage: &Map<String, Value>,
    stage_source_ordinal: u64,
    stage_sequence_ordinal: u64,
    objective_source_ordinal: u64,
    stage_label: &str,
) -> PipelineOutcome<()> {
    let bindings = stage
        .get("objective_npc_waypoints")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{stage_label} is missing NPC waypoint array"
            ))
        })?;
    let mut previous_source_ordinal = None;
    for (index, value) in bindings.iter().enumerate() {
        let label = format!("{stage_label} NPC waypoint {}", index + 1);
        let binding = value.as_object().ok_or_else(|| {
            PipelineError::new(format!("{label} is not an object"))
        })?;
        validate_mission_definition_binding_owner(
            binding,
            stage_source_ordinal,
            stage_sequence_ordinal,
            objective_source_ordinal,
            &label,
        )?;
        let source_ordinal = required_u64(binding, "source_ordinal", &label)?;
        let declaration_source_ordinal =
            required_u64(binding, "declaration_source_ordinal", &label)?;
        if source_ordinal <= objective_source_ordinal
            || declaration_source_ordinal >= source_ordinal
            || previous_source_ordinal
                .is_some_and(|previous| source_ordinal <= previous)
            || required_string(binding, "npc_id", &label)?.is_empty()
            || required_string(binding, "npc_locator_id", &label)?.is_empty()
            || required_string(binding, "waypoint_locator_id", &label)?.is_empty()
        {
            return Err(PipelineError::new(format!(
                "{label} relationship is malformed"
            )));
        }
        previous_source_ordinal = Some(source_ordinal);
    }
    Ok(())
}

fn validate_mission_definition_pickup_state_props(
    stage: &Map<String, Value>,
    stage_source_ordinal: u64,
    stage_sequence_ordinal: u64,
    objective_source_ordinal: u64,
    stage_label: &str,
) -> PipelineOutcome<()> {
    let bindings = stage
        .get("pickup_state_props")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{stage_label} is missing pickup state-prop array"
            ))
        })?;
    let mut previous_target_source_ordinal = None;
    for (index, value) in bindings.iter().enumerate() {
        let label = format!("{stage_label} pickup state prop {}", index + 1);
        let binding = value.as_object().ok_or_else(|| {
            PipelineError::new(format!("{label} is not an object"))
        })?;
        validate_mission_definition_binding_owner(
            binding,
            stage_source_ordinal,
            stage_sequence_ordinal,
            objective_source_ordinal,
            &label,
        )?;
        let target_source_ordinal =
            required_u64(binding, "target_source_ordinal", &label)?;
        let declaration_source_ordinal =
            required_u64(binding, "declaration_source_ordinal", &label)?;
        let _ = required_u64(binding, "source_state", &label)?;
        if target_source_ordinal <= objective_source_ordinal
            || declaration_source_ordinal >= target_source_ordinal
            || previous_target_source_ordinal
                .is_some_and(|previous| target_source_ordinal <= previous)
            || required_string(binding, "target_id", &label)?.is_empty()
            || required_string(binding, "locator_id", &label)?.is_empty()
        {
            return Err(PipelineError::new(format!(
                "{label} relationship is malformed"
            )));
        }
        let scope = binding
            .get("declaration_scope")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "{label} is missing declaration scope"
                ))
            })?;
        match required_string(scope, "kind", &label)?.as_str() {
            "mission" => {},
            "stage" => {
                if required_u64(scope, "source_ordinal", &label)?
                    >= declaration_source_ordinal
                {
                    return Err(PipelineError::new(format!(
                        "{label} declaration scope is malformed"
                    )));
                }
                let _ = required_u64(scope, "sequence_ordinal", &label)?;
            },
            _ => {
                return Err(PipelineError::new(format!(
                    "{label} has unknown declaration scope"
                )));
            },
        }
        previous_target_source_ordinal = Some(target_source_ordinal);
    }
    Ok(())
}

fn optional_nonempty_string<'a>(
    object: &'a Map<String, Value>,
    field: &str,
    label: &str,
) -> PipelineOutcome<Option<&'a str>> {
    match object.get(field) {
        Some(Value::Null) => Ok(None),
        Some(value) => value
            .as_str()
            .filter(|text| !text.is_empty())
            .map(Some)
            .ok_or_else(|| {
                PipelineError::new(format!(
                    "{label} has invalid optional string field {field}"
                ))
            }),
        None => Err(PipelineError::new(format!(
            "{label} is missing optional string field {field}"
        ))),
    }
}

fn validate_mission_id(value: &str) -> PipelineOutcome<()> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'-' | b'_')
        })
    {
        return Err(PipelineError::new(
            "mission definition mission id is not canonical",
        ));
    }
    Ok(())
}

fn retain_importable_evidence(
    index: &PhaseThreePackageIndex,
    evidence: Vec<UnrealSourceEvidence>,
) -> Vec<UnrealSourceEvidence> {
    let importable_ids = index
        .packages()
        .iter()
        .flat_map(crate::domain::PhaseThreePackageRow::members)
        .map(|member| member.id.as_str())
        .collect::<BTreeSet<_>>();
    retain_source_ids(evidence, &importable_ids)
}

fn retain_source_ids(
    mut evidence: Vec<UnrealSourceEvidence>,
    importable_ids: &BTreeSet<&str>,
) -> Vec<UnrealSourceEvidence> {
    evidence.retain(|source| importable_ids.contains(source.id.as_str()));
    evidence
}

fn resolve_source_path(config: &PipelineConfig, manifest_path: &str) -> PipelineOutcome<PathBuf> {
    for root in [&config.game_root, &config.extracted_root] {
        let root_name = root
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| PipelineError::new("pipeline root has no portable basename"))?;
        let prefix = format!("{root_name}/");
        if let Some(relative) = manifest_path.strip_prefix(&prefix) {
            validate_relative_path(relative)?;
            return Ok(root.join(relative));
        }
    }
    Err(PipelineError::new(
        "minor-unit path is outside configured roots",
    ))
}

fn validate_relative_path(path: &str) -> PipelineOutcome<()> {
    if path.is_empty()
        || path.starts_with('/')
        || path.contains(char::from(92))
        || path.contains(':')
        || path.chars().any(char::is_control)
        || path
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(PipelineError::new("unsafe minor-unit relative path"));
    }
    Ok(())
}

fn parse_object(json: &str, label: &str) -> PipelineOutcome<Map<String, Value>> {
    let value = serde_json::from_str::<Value>(json)
        .map_err(|error| PipelineError::new(format!("invalid {label} JSON: {error}")))?;
    value
        .as_object()
        .cloned()
        .ok_or_else(|| PipelineError::new(format!("{label} must be a JSON object")))
}

fn manifest_string(
    row: &Map<String, Value>,
    field: &str,
    line_number: usize,
) -> PipelineOutcome<String> {
    required_string(row, field, &format!("minor-unit line {line_number}"))
}

fn manifest_u64(row: &Map<String, Value>, field: &str, line_number: usize) -> PipelineOutcome<u64> {
    let label = format!("minor-unit line {line_number}");
    let value = required_string(row, field, &label)?;
    value.parse::<u64>().map_err(|error| {
        PipelineError::new(format!(
            "{label} has invalid unsigned integer field {field}: {error}"
        ))
    })
}

fn required_string(
    object: &Map<String, Value>,
    field: &str,
    label: &str,
) -> PipelineOutcome<String> {
    object
        .get(field)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| PipelineError::new(format!("{label} is missing string field {field}")))
}

fn required_u64(object: &Map<String, Value>, field: &str, label: &str) -> PipelineOutcome<u64> {
    object.get(field).and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new(format!("{label} is missing unsigned integer field {field}"))
    })
}

fn required_string_array(
    object: &Map<String, Value>,
    field: &str,
    label: &str,
) -> PipelineOutcome<Vec<String>> {
    let values = object
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{label} is missing string-array field {field}"
            ))
        })?;
    values
        .iter()
        .map(|value| {
            value.as_str().map(ToOwned::to_owned).ok_or_else(|| {
                PipelineError::new(format!(
                    "{label} has non-string value in field {field}"
                ))
            })
        })
        .collect()
}

fn validate_rendered_output(manifest: &str, summary: &str) -> PipelineOutcome<()> {
    let mut lines = manifest.lines();
    let header_line = lines
        .next()
        .ok_or_else(|| PipelineError::new("Unreal import manifest is empty"))?;
    if header_line.trim().is_empty() {
        return Err(PipelineError::new("Unreal import manifest header is blank"));
    }
    let header = parse_object(header_line, "Unreal manifest header")?;
    let summary = parse_object(summary, "Unreal manifest summary")?;
    validate_rendered_schemas(&header, &summary)?;
    let declared = declared_rendered_counts(&header, &summary)?;

    let mut package_ids = BTreeSet::new();
    let mut source_ids = BTreeSet::new();
    let mut expected_sources = BTreeMap::new();
    let mut derived_source_ids = BTreeMap::<String, Vec<String>>::new();
    let mut actual_sources = BTreeMap::<String, u64>::new();
    let mut actual = RenderedCounts::default();
    let mut saw_source = false;
    for (offset, line) in lines.enumerate() {
        let line_number = offset.saturating_add(2);
        if line.trim().is_empty() {
            return Err(PipelineError::new(format!(
                "Unreal import manifest line {line_number} is blank"
            )));
        }
        let label = format!("Unreal manifest line {line_number}");
        let record = parse_object(line, &label)?;
        if required_string(&record, "schema", &label)? != UNREAL_IMPORT_MANIFEST_SCHEMA {
            return Err(PipelineError::new(format!(
                "{label} has a noncanonical schema"
            )));
        }
        match required_string(&record, "record_type", &label)?.as_str() {
            "package" => {
                if saw_source {
                    return Err(PipelineError::new(format!(
                        "{label} appears after source records"
                    )));
                }
                validate_rendered_package(
                    &record,
                    &label,
                    &mut package_ids,
                    &mut expected_sources,
                    &mut derived_source_ids,
                    &mut actual,
                )?;
            }
            "source" => {
                saw_source = true;
                validate_rendered_source(
                    &record,
                    &label,
                    &package_ids,
                    &mut source_ids,
                    &mut actual_sources,
                    &mut actual,
                )?;
            }
            _unsupported => {
                return Err(PipelineError::new(format!(
                    "{label} has an unsupported record type"
                )));
            }
        }
    }
    validate_rendered_source_counts(expected_sources, actual_sources)?;
    validate_rendered_derived_sources(&derived_source_ids, &source_ids)?;
    if actual != declared {
        return Err(PipelineError::new(format!(
            "Unreal manifest counts disagree with its header and summary: \
             declared={declared:?} actual={actual:?}"
        )));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct RenderedCounts {
    packages: u64,
    sources: u64,
    direct_imports: u64,
    requires_fbx: u64,
    requires_editor_factory: u64,
    requires_semantic_conversion: u64,
    metadata_only: u64,
}

fn validate_rendered_schemas(
    header: &Map<String, Value>,
    summary: &Map<String, Value>,
) -> PipelineOutcome<()> {
    if required_string(header, "schema", "Unreal manifest header")? != UNREAL_IMPORT_MANIFEST_SCHEMA
        || required_string(header, "record_type", "Unreal manifest header")? != "header"
    {
        return Err(PipelineError::new(
            "Unreal import manifest header is not canonical",
        ));
    }
    if required_string(summary, "schema", "Unreal manifest summary")?
        != UNREAL_IMPORT_SUMMARY_SCHEMA
    {
        return Err(PipelineError::new(
            "Unreal import summary schema is not canonical",
        ));
    }
    Ok(())
}

fn declared_rendered_counts(
    header: &Map<String, Value>,
    summary: &Map<String, Value>,
) -> PipelineOutcome<RenderedCounts> {
    let fields = [
        ("package_count", "packages"),
        ("source_count", "sources"),
        ("direct_import_count", "direct_imports"),
        ("requires_fbx_count", "requires_fbx"),
        ("requires_editor_factory_count", "requires_editor_factory"),
        (
            "requires_semantic_conversion_count",
            "requires_semantic_conversion",
        ),
        ("metadata_only_count", "metadata_only"),
    ];
    for (header_field, summary_field) in fields {
        let header_value = required_u64(header, header_field, "Unreal manifest header")?;
        let summary_value = required_u64(summary, summary_field, "Unreal manifest summary")?;
        if header_value != summary_value {
            return Err(PipelineError::new(format!(
                "Unreal header field {header_field} disagrees with summary \
                 field {summary_field}"
            )));
        }
    }
    Ok(RenderedCounts {
        packages: required_u64(header, "package_count", "Unreal header")?,
        sources: required_u64(header, "source_count", "Unreal header")?,
        direct_imports: required_u64(header, "direct_import_count", "Unreal header")?,
        requires_fbx: required_u64(header, "requires_fbx_count", "Unreal header")?,
        requires_editor_factory: required_u64(
            header,
            "requires_editor_factory_count",
            "Unreal header",
        )?,
        requires_semantic_conversion: required_u64(
            header,
            "requires_semantic_conversion_count",
            "Unreal header",
        )?,
        metadata_only: required_u64(header, "metadata_only_count", "Unreal header")?,
    })
}

fn validate_rendered_package(
    record: &Map<String, Value>,
    label: &str,
    package_ids: &mut BTreeSet<String>,
    expected_sources: &mut BTreeMap<String, u64>,
    derived_source_ids: &mut BTreeMap<String, Vec<String>>,
    counts: &mut RenderedCounts,
) -> PipelineOutcome<()> {
    let package_id = required_string(record, "package_id", label)?;
    validate_public_identifier(&package_id, "package id")?;
    if !package_ids.insert(package_id.clone()) {
        return Err(PipelineError::new(format!(
            "{label} duplicates package id {package_id}"
        )));
    }
    let source_count = required_u64(record, "source_count", label)?;
    if source_count == 0 {
        let category = required_string(record, "category", label)?;
        let disposition = required_string(record, "disposition", label)?;
        let target_kind = required_string(record, "target_kind", label)?;
        let source_unit_ids =
            required_string_array(record, "source_unit_ids", label)?;
        let text_key_ids =
            required_string_array(record, "text_key_ids", label)?;
        if category != "language"
            || disposition != "requires-editor-factory"
            || target_kind != "StringTable"
            || source_unit_ids.is_empty()
            || text_key_ids.is_empty()
        {
            return Err(PipelineError::new(format!(
                "Unreal package {package_id} declares no source records"
            )));
        }
        let mut seen = BTreeSet::new();
        for source_id in &source_unit_ids {
            validate_public_identifier(source_id, "derived source unit id")?;
            if !seen.insert(source_id.clone()) {
                return Err(PipelineError::new(format!(
                    "Unreal package {package_id} duplicates derived source id"
                )));
            }
        }
        for text_key_id in &text_key_ids {
            validate_public_identifier(text_key_id, "text key id")?;
        }
        drop(derived_source_ids.insert(package_id.clone(), source_unit_ids));
    }
    let _ = expected_sources.insert(package_id, source_count);
    counts.packages = counts.packages.saturating_add(1);
    match required_string(record, "disposition", label)?.as_str() {
        "direct-editor-import" => {}
        "requires-fbx" => {
            counts.requires_fbx = counts.requires_fbx.saturating_add(1);
        }
        "requires-editor-factory" => {
            counts.requires_editor_factory = counts.requires_editor_factory.saturating_add(1);
        }
        "requires-semantic-conversion" => {
            counts.requires_semantic_conversion =
                counts.requires_semantic_conversion.saturating_add(1);
        }
        "metadata-only" => {
            counts.metadata_only = counts.metadata_only.saturating_add(1);
        }
        _unsupported => {
            return Err(PipelineError::new(format!(
                "{label} has an unsupported disposition"
            )));
        }
    }
    Ok(())
}

fn validate_rendered_source(
    record: &Map<String, Value>,
    label: &str,
    package_ids: &BTreeSet<String>,
    source_ids: &mut BTreeSet<String>,
    actual_sources: &mut BTreeMap<String, u64>,
    counts: &mut RenderedCounts,
) -> PipelineOutcome<()> {
    let package_id = required_string(record, "package_id", label)?;
    validate_public_identifier(&package_id, "source package id")?;
    if !package_ids.contains(&package_id) {
        return Err(PipelineError::new(format!(
            "{label} references undeclared package {package_id}"
        )));
    }
    let source_id = required_string(record, "id", label)?;
    validate_public_identifier(&source_id, "source id")?;
    if !source_ids.insert(source_id.clone()) {
        return Err(PipelineError::new(format!(
            "{label} duplicates source id {source_id}"
        )));
    }
    let package_sources = actual_sources.entry(package_id).or_default();
    *package_sources = package_sources.saturating_add(1);
    counts.sources = counts.sources.saturating_add(1);
    match record.get("direct_import") {
        None | Some(Value::Null) => {}
        Some(Value::Object(_direct_import)) => {
            counts.direct_imports = counts.direct_imports.saturating_add(1);
        }
        Some(_invalid) => {
            return Err(PipelineError::new(format!(
                "{label} has a non-object direct_import contract"
            )));
        }
    }
    Ok(())
}

fn validate_public_identifier(value: &str, label: &str) -> PipelineOutcome<()> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || bytes.windows(2).any(|pair| pair == b"--")
        || !bytes
            .iter()
            .copied()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
    {
        return Err(PipelineError::new(format!(
            "rendered Unreal {label} is not canonical"
        )));
    }
    Ok(())
}

fn validate_rendered_source_counts(
    expected_sources: BTreeMap<String, u64>,
    mut actual_sources: BTreeMap<String, u64>,
) -> PipelineOutcome<()> {
    for (package_id, expected) in expected_sources {
        let observed = actual_sources.remove(&package_id).unwrap_or_default();
        if observed != expected {
            return Err(PipelineError::new(format!(
                "Unreal package {package_id} source count mismatch: \
                 declared={expected} actual={observed}"
            )));
        }
    }
    if actual_sources.is_empty() {
        Ok(())
    } else {
        Err(PipelineError::new(
            "Unreal manifest contains sources for undeclared packages",
        ))
    }
}

fn validate_rendered_derived_sources(
    derived_source_ids: &BTreeMap<String, Vec<String>>,
    source_ids: &BTreeSet<String>,
) -> PipelineOutcome<()> {
    for (package_id, referenced_ids) in derived_source_ids {
        if referenced_ids
            .iter()
            .any(|source_id| !source_ids.contains(source_id))
        {
            return Err(PipelineError::new(format!(
                concat!(
                    "Unreal derived package {} has missing source ",
                    "provenance"
                ),
                package_id
            )));
        }
    }
    Ok(())
}

fn published_byte_count(
    manifest: &str,
    summary: &str,
    mission_definitions: &str,
    plans: &PlanBundle,
) -> u64 {
    let plan_bytes = plans
        .artifacts()
        .iter()
        .fold(plans.index_json().len(), |total, artifact| {
            total.saturating_add(artifact.json.len())
        });
    u64::try_from(
        manifest
            .len()
            .saturating_add(summary.len())
            .saturating_add(mission_definitions.len())
            .saturating_add(plan_bytes),
    )
    .unwrap_or(u64::MAX)
}

fn publish_staging(
    manifest: &str,
    summary: &str,
    mission_definitions: &str,
    plans: &PlanBundle,
) -> PipelineOutcome<()> {
    let destination = PathBuf::from(UNREAL_STAGING_WORKSPACE_ROOT);
    let temporary_root = PathBuf::from(".temp");
    let pipeline_root = temporary_root.join("pipeline");
    let transaction_root = pipeline_root.join("unreal-prepare");
    let staging = transaction_root.join(format!("staging-{}", std::process::id()));
    let backup = transaction_root.join(format!("backup-{}", std::process::id()));
    ensure_generated_directory(&temporary_root, "create Unreal temporary root")?;
    ensure_generated_directory(&pipeline_root, "create Unreal pipeline root")?;
    ensure_generated_directory(&transaction_root, "create Unreal transaction root")?;
    validate_generated_chain(&[
        temporary_root.as_path(),
        pipeline_root.as_path(),
        transaction_root.as_path(),
    ])?;
    remove_generated_directory(&staging)?;
    remove_generated_directory(&backup)?;
    fs::create_dir_all(&staging).map_err(path_error("create Unreal staging directory"))?;
    let plan_root = staging.join(PLAN_ROOT);
    fs::create_dir_all(&plan_root).map_err(path_error("create Unreal plan directory"))?;
    let mut published_paths = BTreeSet::new();
    for (relative_path, content) in [
        (MANIFEST_FILE, manifest),
        (SUMMARY_FILE, summary),
        (MISSION_DEFINITIONS_FILE, mission_definitions),
        (PLAN_INDEX_FILE, plans.index_json()),
    ] {
        write_staged_file(&staging, relative_path, content)?;
        let _inserted = published_paths.insert(relative_path.to_owned());
    }
    for artifact in plans.artifacts() {
        let relative_path = format!("{PLAN_ROOT}/{}", artifact.filename);
        if !PUBLISHED_FILES.contains(&relative_path.as_str()) {
            return Err(PipelineError::new(
                "Unreal plan publication path is not declared",
            ));
        }
        if !published_paths.insert(relative_path.clone()) {
            return Err(PipelineError::new(
                "Unreal staging publication path is duplicated",
            ));
        }
        write_staged_file(&staging, &relative_path, &artifact.json)?;
    }
    validate_publication_inventory(&published_paths)?;

    let had_destination = destination.exists();
    if had_destination {
        validate_generated_directory(&destination)?;
        fs::rename(&destination, &backup)
            .map_err(|error| prepare_io_error("back up Unreal staging root", &error))?;
    }
    if let Err(error) = fs::rename(&staging, &destination) {
        let rollback_error = had_destination
            .then(|| fs::rename(&backup, &destination).err())
            .flatten();
        return Err(publication_error(&error, rollback_error.as_ref()));
    }
    if had_destination {
        remove_generated_directory(&backup)?;
    }
    Ok(())
}

fn validate_publication_inventory(published_paths: &BTreeSet<String>) -> PipelineOutcome<()> {
    let expected = PUBLISHED_FILES
        .iter()
        .map(|path| (*path).to_owned())
        .collect::<BTreeSet<_>>();
    if *published_paths != expected {
        return Err(PipelineError::new(
            "Unreal staging publication inventory is not exact",
        ));
    }
    Ok(())
}

fn write_staged_file(root: &Path, relative_path: &str, content: &str) -> PipelineOutcome<()> {
    validate_relative_path(relative_path)?;
    let path = root.join(relative_path);
    fs::write(&path, content).map_err(path_error("write Unreal staged file"))?;
    verify_staged_file(&path, relative_path, content.as_bytes())
}

fn verify_staged_file(path: &Path, public_path: &str, expected: &[u8]) -> PipelineOutcome<()> {
    let actual = local_read_bytes(path).map_err(path_error("read Unreal staged file"))?;
    if actual != expected {
        return Err(PipelineError::new(format!(
            "staged output verification failed for {public_path}"
        )));
    }
    Ok(())
}

fn remove_generated_directory(path: &Path) -> PipelineOutcome<()> {
    if !path.exists() {
        return Ok(());
    }
    validate_generated_directory(path)?;
    fs::remove_dir_all(path).map_err(path_error("remove generated Unreal directory"))
}

fn ensure_generated_directory(path: &Path, create_action: &'static str) -> PipelineOutcome<()> {
    match fs::symlink_metadata(path) {
        Ok(_metadata) => validate_generated_directory(path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir_all(path).map_err(path_error(create_action))?;
            validate_generated_directory(path)
        }
        Err(error) => Err(prepare_io_error(
            "inspect generated Unreal directory",
            &error,
        )),
    }
}

fn validate_generated_chain(paths: &[&Path]) -> PipelineOutcome<()> {
    for path in paths {
        validate_generated_directory(path)?;
    }
    Ok(())
}

fn validate_generated_directory(path: &Path) -> PipelineOutcome<()> {
    let metadata =
        fs::symlink_metadata(path).map_err(path_error("inspect generated Unreal directory"))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(PipelineError::new(
            "generated staging path is not a regular directory",
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt as _;
        const REPARSE_POINT: u32 = 0x400;
        if metadata.file_attributes() & REPARSE_POINT != 0 {
            return Err(PipelineError::new(
                "generated staging path is a reparse boundary",
            ));
        }
    }
    Ok(())
}

fn read_utf8(path: &Path, action: &'static str) -> PipelineOutcome<String> {
    local_read_utf8(path).map_err(path_error(action))
}

fn prepare_io_error(action: &str, error: &std::io::Error) -> PipelineError {
    PipelineError::new(format!("{action} failed ({:?})", error.kind()))
}

fn publication_error(
    publish_error: &std::io::Error,
    rollback_error: Option<&std::io::Error>,
) -> PipelineError {
    let message = rollback_error.map_or_else(
        || {
            format!(
                "publish Unreal staging root failed ({:?})",
                publish_error.kind()
            )
        },
        |error| {
            format!(
                concat!(
                    "publish Unreal staging root failed ({:?}); ",
                    "restore previous Unreal staging root failed ({:?})"
                ),
                publish_error.kind(),
                error.kind()
            )
        },
    );
    PipelineError::new(message)
}

fn path_error(action: &'static str) -> impl FnOnce(std::io::Error) -> PipelineError {
    move |error| prepare_io_error(action, &error)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_prepare/tests.rs"]
mod tests;
