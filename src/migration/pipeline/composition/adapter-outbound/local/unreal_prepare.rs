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
use std::fs;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::adapters::driving::local::{
    read_bytes as local_read_bytes, read_utf8 as local_read_utf8,
};
use serde_json::{Map, Value};
use shar_sha256::digest_hex;
use shar_unreal_conversion::domain::PlanBundle;

use super::unreal_fbx_catalog::{FBX_CATALOG_ROOT, verified_fbx_catalog};
use crate::adapters::driven::check_cancellation;
use crate::adapters::driven::local::progress::StageProgress;
use crate::domain::{
    PhaseThreePackageIndex, PipelineConfig, PipelineError, PipelineOutcome,
    StageReport, UNREAL_IMPORT_MANIFEST_SCHEMA, UNREAL_IMPORT_SUMMARY_SCHEMA,
    UnrealImportManifest, UnrealSourceEvidence,
};

/// Canonical generated Unreal staging root.
pub(super) const UNREAL_STAGING_ROOT: &str = "unreal-staging";
/// Canonical import-manifest filename.
const MANIFEST_FILE: &str = "manifest.jsonl";
/// Canonical import-summary filename.
const SUMMARY_FILE: &str = "summary.json";
/// Canonical generated plan directory.
const PLAN_ROOT: &str = "plans";
/// Canonical generated plan-bundle index filename.
const PLAN_INDEX_FILE: &str = "plans/index.json";
/// Complete set of files published by one prepare-unreal transaction.
const PUBLISHED_FILES: [&str; 9] = [
    MANIFEST_FILE,
    SUMMARY_FILE,
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
pub(super) fn prepare_unreal(
    config: &PipelineConfig,
) -> PipelineOutcome<StageReport> {
    let minor_unit_root = config.extracted_root.join("minor-unit");
    let manifest_path = minor_unit_root.join("manifest.jsonl");
    let audit_path = minor_unit_root.join("audit.json");
    let index_path = minor_unit_root.join("index.jsonl");
    let manifest_text = read_utf8(&manifest_path, "read minor-unit manifest")?;
    validate_audit(&audit_path, &manifest_text)?;
    let index = PhaseThreePackageIndex::read_for_unreal(&index_path).map_err(
        |error| {
            PipelineError::new(format!(
                "Unreal package-index intake failed: {error}"
            ))
        },
    )?;
    let evidence = source_evidence(&manifest_text, config)?;
    let evidence = retain_importable_evidence(&index, evidence);
    let unreal_manifest = UnrealImportManifest::build(&index, evidence)
        .map_err(|error| {
            PipelineError::new(format!(
                "Unreal manifest planning failed: {error}"
            ))
        })?;
    let manifest_jsonl = unreal_manifest.to_jsonl();
    let summary_json = unreal_manifest.summary_json();
    validate_rendered_output(&manifest_jsonl, &summary_json)?;
    let manifest_revision = digest_hex(manifest_jsonl.as_bytes());
    let fbx_catalog = verified_fbx_catalog(Path::new(FBX_CATALOG_ROOT))?;
    let verified_fbx_count = fbx_catalog.as_ref().map_or(0, Vec::len);
    let plan_bundle = fbx_catalog
        .as_deref()
        .map_or_else(
            || unreal_manifest.plan_bundle(&manifest_revision),
            |catalog| {
                unreal_manifest.plan_bundle_with_complete_fbx_catalog(
                    &manifest_revision,
                    catalog,
                )
            },
        )
        .map_err(|error| {
            PipelineError::new(format!(
                "Unreal plan generation failed: {error}"
            ))
        })?;
    publish_staging(&manifest_jsonl, &summary_json, &plan_bundle)?;
    Ok(StageReport {
        name: "prepare-unreal",
        files: PUBLISHED_FILES.len(),
        bytes: published_byte_count(
            &manifest_jsonl,
            &summary_json,
            &plan_bundle,
        ),
        note: format!(
            concat!(
                "verified {} sources across {} semantic packages and {} ",
                "generated FBX artifacts; published {} with plan bundle {}"
            ),
            unreal_manifest.source_count(),
            unreal_manifest.package_count(),
            verified_fbx_count,
            UNREAL_STAGING_ROOT,
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
    let audited_sha256 =
        required_string(&audit, "manifest_sha256", "minor-unit audit")?;
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
) -> PipelineOutcome<Vec<UnrealSourceEvidence>> {
    let source_count = manifest
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count();
    let mut progress =
        StageProgress::begin("Unreal source evidence", source_count);
    let mut evidence = Vec::with_capacity(source_count);
    let mut ids = BTreeSet::new();
    for (line_index, line) in manifest.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        if line_index % 256 == 0 {
            check_cancellation()?;
        }
        let line_number = line_index.saturating_add(1);
        let row =
            parse_object(line, &format!("minor-unit line {line_number}"))?;
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
        progress.advance(&id);
        let bytes = local_read_bytes(&resolved)
            .map_err(path_error("read Unreal source evidence"))?;
        let actual_size = u64::try_from(bytes.len()).unwrap_or(u64::MAX);
        if actual_size != expected_size {
            return Err(PipelineError::new(format!(
                "source size changed for {path}: manifest={expected_size} \
                 actual={actual_size}"
            )));
        }
        evidence.push(UnrealSourceEvidence {
            id,
            path,
            file_extension: manifest_string(
                &row,
                "file_extension",
                line_number,
            )?,
            unit_type: manifest_string(&row, "type", line_number)?,
            subtype: manifest_string(&row, "subtype", line_number)?,
            kind: manifest_string(&row, "kind", line_number)?,
            function: manifest_string(&row, "function", line_number)?,
            schema: manifest_string(&row, "schema", line_number)?,
            origin: manifest_string(&row, "origin", line_number)?,
            source_path: manifest_string(&row, "source_path", line_number)?,
            source_chunk_kind: manifest_string(
                &row,
                "source_chunk_kind",
                line_number,
            )?,
            size_bytes: actual_size,
            sha256: digest_hex(&bytes),
            unreal_import_relation: manifest_string(
                &row,
                "unreal_import_relation",
                line_number,
            )?,
            future_normalization: manifest_string(
                &row,
                "future_normalization",
                line_number,
            )?,
        });
    }
    progress.finish();
    Ok(evidence)
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

fn resolve_source_path(
    config: &PipelineConfig,
    manifest_path: &str,
) -> PipelineOutcome<PathBuf> {
    for root in [&config.game_root, &config.extracted_root] {
        let root_name = root
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                PipelineError::new("pipeline root has no portable basename")
            })?;
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

fn parse_object(
    json: &str,
    label: &str,
) -> PipelineOutcome<Map<String, Value>> {
    let value = serde_json::from_str::<Value>(json).map_err(|error| {
        PipelineError::new(format!("invalid {label} JSON: {error}"))
    })?;
    value.as_object().cloned().ok_or_else(|| {
        PipelineError::new(format!("{label} must be a JSON object"))
    })
}

fn manifest_string(
    row: &Map<String, Value>,
    field: &str,
    line_number: usize,
) -> PipelineOutcome<String> {
    required_string(row, field, &format!("minor-unit line {line_number}"))
}

fn manifest_u64(
    row: &Map<String, Value>,
    field: &str,
    line_number: usize,
) -> PipelineOutcome<u64> {
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
        .ok_or_else(|| {
            PipelineError::new(format!(
                "{label} is missing string field {field}"
            ))
        })
}

fn required_u64(
    object: &Map<String, Value>,
    field: &str,
    label: &str,
) -> PipelineOutcome<u64> {
    object.get(field).and_then(Value::as_u64).ok_or_else(|| {
        PipelineError::new(format!(
            "{label} is missing unsigned integer field {field}"
        ))
    })
}

fn validate_rendered_output(
    manifest: &str,
    summary: &str,
) -> PipelineOutcome<()> {
    let mut lines = manifest.lines();
    let header_line = lines
        .next()
        .ok_or_else(|| PipelineError::new("Unreal import manifest is empty"))?;
    if header_line.trim().is_empty() {
        return Err(PipelineError::new(
            "Unreal import manifest header is blank",
        ));
    }
    let header = parse_object(header_line, "Unreal manifest header")?;
    let summary = parse_object(summary, "Unreal manifest summary")?;
    validate_rendered_schemas(&header, &summary)?;
    let declared = declared_rendered_counts(&header, &summary)?;

    let mut package_ids = BTreeSet::new();
    let mut source_ids = BTreeSet::new();
    let mut expected_sources = BTreeMap::new();
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
        if required_string(&record, "schema", &label)?
            != UNREAL_IMPORT_MANIFEST_SCHEMA
        {
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
                    &mut actual,
                )?;
            },
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
            },
            _unsupported => {
                return Err(PipelineError::new(format!(
                    "{label} has an unsupported record type"
                )));
            },
        }
    }
    validate_rendered_source_counts(expected_sources, actual_sources)?;
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
    if required_string(header, "schema", "Unreal manifest header")?
        != UNREAL_IMPORT_MANIFEST_SCHEMA
        || required_string(header, "record_type", "Unreal manifest header")?
            != "header"
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
        let header_value =
            required_u64(header, header_field, "Unreal manifest header")?;
        let summary_value =
            required_u64(summary, summary_field, "Unreal manifest summary")?;
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
        direct_imports: required_u64(
            header,
            "direct_import_count",
            "Unreal header",
        )?,
        requires_fbx: required_u64(
            header,
            "requires_fbx_count",
            "Unreal header",
        )?,
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
        metadata_only: required_u64(
            header,
            "metadata_only_count",
            "Unreal header",
        )?,
    })
}

fn validate_rendered_package(
    record: &Map<String, Value>,
    label: &str,
    package_ids: &mut BTreeSet<String>,
    expected_sources: &mut BTreeMap<String, u64>,
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
        return Err(PipelineError::new(
            "Unreal package declares no source records",
        ));
    }
    let _ = expected_sources.insert(package_id, source_count);
    counts.packages = counts.packages.saturating_add(1);
    match required_string(record, "disposition", label)?.as_str() {
        "direct-editor-import" => {},
        "requires-fbx" => {
            counts.requires_fbx = counts.requires_fbx.saturating_add(1);
        },
        "requires-editor-factory" => {
            counts.requires_editor_factory =
                counts.requires_editor_factory.saturating_add(1);
        },
        "requires-semantic-conversion" => {
            counts.requires_semantic_conversion =
                counts.requires_semantic_conversion.saturating_add(1);
        },
        "metadata-only" => {
            counts.metadata_only = counts.metadata_only.saturating_add(1);
        },
        _unsupported => {
            return Err(PipelineError::new(format!(
                "{label} has an unsupported disposition"
            )));
        },
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
        None | Some(Value::Null) => {},
        Some(Value::Object(_direct_import)) => {
            counts.direct_imports = counts.direct_imports.saturating_add(1);
        },
        Some(_invalid) => {
            return Err(PipelineError::new(format!(
                "{label} has a non-object direct_import contract"
            )));
        },
    }
    Ok(())
}

fn validate_public_identifier(value: &str, label: &str) -> PipelineOutcome<()> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || !bytes.first().is_some_and(u8::is_ascii_alphanumeric)
        || !bytes.last().is_some_and(u8::is_ascii_alphanumeric)
        || bytes.windows(2).any(|pair| pair == b"--")
        || !bytes.iter().copied().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
        })
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

fn published_byte_count(
    manifest: &str,
    summary: &str,
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
            .saturating_add(plan_bytes),
    )
    .unwrap_or(u64::MAX)
}

fn publish_staging(
    manifest: &str,
    summary: &str,
    plans: &PlanBundle,
) -> PipelineOutcome<()> {
    let destination = PathBuf::from(UNREAL_STAGING_ROOT);
    let temporary_root = PathBuf::from(".temp");
    let pipeline_root = temporary_root.join("pipeline");
    let transaction_root = pipeline_root.join("unreal-prepare");
    let staging =
        transaction_root.join(format!("staging-{}", std::process::id()));
    let backup =
        transaction_root.join(format!("backup-{}", std::process::id()));
    ensure_generated_directory(
        &temporary_root,
        "create Unreal temporary root",
    )?;
    ensure_generated_directory(&pipeline_root, "create Unreal pipeline root")?;
    ensure_generated_directory(
        &transaction_root,
        "create Unreal transaction root",
    )?;
    validate_generated_chain(&[
        temporary_root.as_path(),
        pipeline_root.as_path(),
        transaction_root.as_path(),
    ])?;
    remove_generated_directory(&staging)?;
    remove_generated_directory(&backup)?;
    fs::create_dir_all(&staging)
        .map_err(path_error("create Unreal staging directory"))?;
    let plan_root = staging.join(PLAN_ROOT);
    fs::create_dir_all(&plan_root)
        .map_err(path_error("create Unreal plan directory"))?;
    let mut published_paths = BTreeSet::new();
    for (relative_path, content) in [
        (MANIFEST_FILE, manifest),
        (SUMMARY_FILE, summary),
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
        fs::rename(&destination, &backup).map_err(|error| {
            prepare_io_error("back up Unreal staging root", &error)
        })?;
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

fn validate_publication_inventory(
    published_paths: &BTreeSet<String>,
) -> PipelineOutcome<()> {
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

fn write_staged_file(
    root: &Path,
    relative_path: &str,
    content: &str,
) -> PipelineOutcome<()> {
    validate_relative_path(relative_path)?;
    let path = root.join(relative_path);
    fs::write(&path, content)
        .map_err(path_error("write Unreal staged file"))?;
    verify_staged_file(&path, relative_path, content.as_bytes())
}

fn verify_staged_file(
    path: &Path,
    public_path: &str,
    expected: &[u8],
) -> PipelineOutcome<()> {
    let actual = local_read_bytes(path)
        .map_err(path_error("read Unreal staged file"))?;
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
    fs::remove_dir_all(path)
        .map_err(path_error("remove generated Unreal directory"))
}

fn ensure_generated_directory(
    path: &Path,
    create_action: &'static str,
) -> PipelineOutcome<()> {
    match fs::symlink_metadata(path) {
        Ok(_metadata) => validate_generated_directory(path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir_all(path).map_err(path_error(create_action))?;
            validate_generated_directory(path)
        },
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
    let metadata = fs::symlink_metadata(path)
        .map_err(path_error("inspect generated Unreal directory"))?;
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

fn path_error(
    action: &'static str,
) -> impl FnOnce(std::io::Error) -> PipelineError {
    move |error| prepare_io_error(action, &error)
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/unreal_prepare/tests.rs"]
mod tests;
