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
//   - Canonical Unreal import-manifest JSON serialization.
// - Must-Not:
//   - Select import policy, inspect files, or publish outputs.
// - Allows:
//   - Validated manifest records and deterministic JSON text.
// - Split-When:
//   - Split when another serialization format gains a lifecycle.
// - Merge-When:
//   - Merge when another module owns identical manifest rendering.
// - Summary:
//   - Unreal import-manifest renderer.
// - Description:
//   - Emits one header followed by canonical package and source records.
// - Usage:
//   - Called only through UnrealImportManifest serialization methods.
// - Defaults:
//   - Strings are escaped and arrays preserve validated order.
//

//! Unreal import-manifest renderer.

use super::{
    DirectImportRecord, UNREAL_IMPORT_MANIFEST_SCHEMA,
    UNREAL_IMPORT_SUMMARY_SCHEMA, UnrealImportManifest, UnrealPackageRecord,
    UnrealSourceRecord,
};
use crate::domain::escape_json;

pub(super) fn manifest_jsonl(manifest: &UnrealImportManifest) -> String {
    let mut body = String::new();
    for package in &manifest.packages {
        body.push_str(&package_json(package));
        body.push('\n');
    }
    for source in &manifest.sources {
        body.push_str(&source_json(source));
        body.push('\n');
    }
    let summary = &manifest.summary;
    format!(
        concat!(
            "{{\"schema\":\"{}\",\"record_type\":\"header\",",
            "\"package_count\":{},\"source_count\":{},",
            "\"direct_import_count\":{},\"requires_fbx_count\":{},",
            "\"requires_editor_factory_count\":{},",
            "\"requires_semantic_conversion_count\":{},",
            "\"metadata_only_count\":{}}}",
            "\n{}"
        ),
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        summary.packages,
        summary.sources,
        summary.direct_imports,
        summary.requires_fbx,
        summary.requires_editor_factory,
        summary.requires_semantic_conversion,
        summary.metadata_only,
        body,
    )
}

pub(super) fn summary_json(manifest: &UnrealImportManifest) -> String {
    let summary = &manifest.summary;
    format!(
        concat!(
            "{{\"schema\":\"{}\",\"packages\":{},",
            "\"sources\":{},\"direct_imports\":{},",
            "\"requires_fbx\":{},\"requires_editor_factory\":{},",
            "\"requires_semantic_conversion\":{},",
            "\"metadata_only\":{}}}\n"
        ),
        UNREAL_IMPORT_SUMMARY_SCHEMA,
        summary.packages,
        summary.sources,
        summary.direct_imports,
        summary.requires_fbx,
        summary.requires_editor_factory,
        summary.requires_semantic_conversion,
        summary.metadata_only,
    )
}

fn package_json(record: &UnrealPackageRecord) -> String {
    format!(
        concat!(
            "{{\"schema\":\"{}\",\"record_type\":\"package\",",
            "\"package_id\":\"{}\",\"package_root\":\"{}\",",
            "\"category\":\"{}\",\"subcategory\":\"{}\",",
            "\"conversion_family\":\"{}\",",
            "\"disposition\":\"{}\",\"target_kind\":\"{}\",",
            "\"importer\":\"{}\",\"import_profile\":\"{}\",",
            "\"package_path\":\"{}\",\"asset_name\":\"{}\",",
            "\"expected_staged_files\":{},",
            "\"expected_unreal_objects\":{},\"source_count\":{},",
            "\"source_unit_ids\":{},\"text_key_ids\":{},",
            "\"reason\":{}}}"
        ),
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        escape_json(&record.package_id),
        escape_json(&record.package_root),
        escape_json(&record.category),
        escape_json(&record.subcategory),
        record.conversion_family,
        record.disposition,
        record.target_kind,
        record.importer,
        record.import_profile,
        escape_json(&record.package_path),
        escape_json(&record.asset_name),
        string_array_json(&record.expected_staged_files),
        string_array_json(&record.expected_unreal_objects),
        record.source_count,
        string_array_json(&record.source_unit_ids),
        string_array_json(&record.text_key_ids),
        nullable_string_json(record.reason),
    )
}

fn source_json(record: &UnrealSourceRecord) -> String {
    let source = &record.evidence;
    format!(
        concat!(
            "{{\"schema\":\"{}\",\"record_type\":\"source\",",
            "\"package_id\":\"{}\",\"id\":\"{}\",",
            "\"role\":\"{}\",\"path\":\"{}\",",
            "\"file_extension\":\"{}\",\"type\":\"{}\",",
            "\"subtype\":\"{}\",\"kind\":\"{}\",",
            "\"function\":\"{}\",\"source_schema\":\"{}\",",
            "\"origin\":\"{}\",\"source_path\":\"{}\",",
            "\"source_chunk_kind\":\"{}\",\"size_bytes\":{},",
            "\"sha256\":\"{}\",",
            "\"unreal_import_relation\":\"{}\",",
            "\"future_normalization\":\"{}\",",
            "\"direct_import\":{}}}"
        ),
        UNREAL_IMPORT_MANIFEST_SCHEMA,
        escape_json(&record.package_id),
        escape_json(&record.id),
        record.role.as_str(),
        escape_json(&source.path),
        escape_json(&source.file_extension),
        escape_json(&source.unit_type),
        escape_json(&source.subtype),
        escape_json(&source.kind),
        escape_json(&source.function),
        escape_json(&source.schema),
        escape_json(&source.origin),
        escape_json(&source.source_path),
        escape_json(&source.source_chunk_kind),
        source.size_bytes,
        source.sha256,
        escape_json(&source.unreal_import_relation),
        escape_json(&source.future_normalization),
        direct_import_json(record.direct_import.as_ref()),
    )
}

fn direct_import_json(record: Option<&DirectImportRecord>) -> String {
    let Some(record) = record else {
        return "null".to_owned();
    };
    format!(
        concat!(
            "{{\"importer\":\"{}\",\"import_profile\":\"{}\",",
            "\"target_class\":\"{}\",\"package_path\":\"{}\",",
            "\"asset_name\":\"{}\",\"object_path\":\"{}\"}}"
        ),
        record.importer,
        record.import_profile,
        record.target_class,
        escape_json(&record.package_path),
        escape_json(&record.asset_name),
        escape_json(&record.object_path),
    )
}

fn string_array_json(values: &[String]) -> String {
    let mut output = String::from("[");
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push(',');
        }
        output.push('"');
        output.push_str(&escape_json(value));
        output.push('"');
    }
    output.push(']');
    output
}

fn nullable_string_json(value: Option<&str>) -> String {
    value.map_or_else(
        || "null".to_owned(),
        |text| format!("\"{}\"", escape_json(text)),
    )
}
