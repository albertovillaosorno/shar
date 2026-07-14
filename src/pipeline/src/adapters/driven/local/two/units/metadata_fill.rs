// File:
//   - metadata_fill.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/metadata_fill.rs
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
//   - The metadata fill contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute metadata fill.
// - Split-When:
//   - Split when metadata fill contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Fill minor unit metadata.
// - Description:
//   - Defines metadata fill data and behavior for pipeline phase two minor
//   - units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs metadata fill.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Fill minor unit metadata keeps tightly coupled validation,
//   - ordering, and deterministic transformation invariants together; split
//   - when a stable independently testable sub-boundary is identified.
//

//! Fill minor unit metadata.
//!
//! This boundary keeps fill minor unit metadata explicit and returns
//! deterministic results to pipeline callers.
use std::path::Path;

use schoenwald_filesystem::adapters::driving::local::{
    read_utf8 as local_read_utf8, write_text as local_write_text,
};

use super::editor::edit_minor_unit_metadata;
use super::manifest_minor_unit::render_row;
use super::metadata::{MinorUnitMetadata, classify_minor_unit};
use super::taxonomy;
use crate::domain::{PipelineError, StageReport};

/// Result.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Fill minor unit metadata.
///
/// # Errors
///
/// Returns an error when validation, filesystem access, or output writing
/// fails.
pub(in crate::adapters::driven::local) fn fill_minor_unit_metadata(
    extracted_root: &Path
) -> PipelineOutcome<StageReport> {
    let manifest_path = taxonomy::manifest_path(extracted_root);
    let input =
        local_read_utf8(&manifest_path).map_err(io_error(&manifest_path))?;
    let mut output = String::new();
    let mut rows = 0usize;
    let mut changed = 0usize;

    for (line_index, line) in input
        .lines()
        .enumerate()
    {
        if line
            .trim()
            .is_empty()
        {
            continue;
        }
        let path = read_string_field(
            line, "path",
        )
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "{}:{} missing path field",
                        manifest_path.display(),
                        line_index.saturating_add(1)
                    ),
                )
            },
        )?;
        let file_extension = read_string_field(
            line,
            "file_extension",
        )
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "{}:{} missing file_extension field",
                        manifest_path.display(),
                        line_index.saturating_add(1)
                    ),
                )
            },
        )?;
        let mut metadata = classify_minor_unit(
            extracted_root,
            &path,
            &file_extension,
        )?;
        // The manifest stage already stamped real provenance and the name-free
        // route; carry them through classification untouched and derive the
        // opaque identity from the final type.
        carry_manifest_provenance(
            &mut metadata,
            line,
        );
        apply_manifest_evidence_classification(
            &mut metadata,
            line,
            &path,
        );
        metadata.id = compute_id(
            &metadata.obfuscated_route,
            &metadata.type_,
            &metadata.recovery_status,
        );
        if metadata.type_ != taxonomy::UNKNOWN {
            changed = changed.saturating_add(1);
        }
        output.push_str(
            &render_filled_row(
                &path,
                &file_extension,
                &metadata,
            ),
        );
        output.push('\n');
        rows = rows.saturating_add(1);
    }

    local_write_text(
        &manifest_path,
        &output,
        true,
    )
    .map_err(io_error(&manifest_path))?;
    let editor_report = edit_minor_unit_metadata(extracted_root)?;
    Ok(
        StageReport {
            name: "minor-unit-metadata-fill",
            files: rows,
            bytes: u64::try_from(changed).unwrap_or(u64::MAX),
            note: format!(
                "metadata_fill classified {changed}/{rows} minor units in \
                 place; {}",
                editor_report.note
            ),
        },
    )
}

/// Render filled row.
fn render_filled_row(
    path: &str,
    file_extension: &str,
    metadata: &MinorUnitMetadata,
) -> String {
    render_row(
        &[
            (
                "path", path,
            ),
            (
                "id",
                &metadata.id,
            ),
            (
                "obfuscated_route",
                &metadata.obfuscated_route,
            ),
            (
                "file_extension",
                file_extension,
            ),
            (
                "type",
                &metadata.type_,
            ),
            (
                "subtype",
                &metadata.subtype,
            ),
            (
                "kind",
                &metadata.kind,
            ),
            (
                "function",
                &metadata.function,
            ),
            (
                "schema",
                &metadata.schema,
            ),
            (
                "origin",
                &metadata.origin,
            ),
            (
                "source_path",
                &metadata.source_path,
            ),
            (
                "source_extension",
                &metadata.source_extension,
            ),
            (
                "source_container",
                &metadata.source_container,
            ),
            (
                "source_chunk_kind",
                &metadata.source_chunk_kind,
            ),
            (
                "source_chunk_ordinal",
                &metadata.source_chunk_ordinal,
            ),
            (
                "recovery_status",
                &metadata.recovery_status,
            ),
            (
                "derived_from",
                &metadata.derived_from,
            ),
            (
                "size_bytes",
                &metadata.size_bytes,
            ),
            (
                "unreal_import_relation",
                &metadata.unreal_import_relation,
            ),
            (
                "future_normalization",
                &metadata.future_normalization,
            ),
            (
                "component_links",
                &metadata.component_links,
            ),
            (
                "classification_notes",
                &metadata.classification_notes,
            ),
        ],
    )
}

/// Copy the provenance columns the manifest stage already resolved onto the
/// classified metadata, defaulting to truthful sentinels when a column is
/// absent so a re-run never regresses a row to `error`.
fn carry_manifest_provenance(
    metadata: &mut MinorUnitMetadata,
    line: &str,
) {
    metadata.obfuscated_route = read_string_field(
        line,
        "obfuscated_route",
    )
    .unwrap_or_else(|| taxonomy::UNKNOWN.to_owned());
    metadata.source_chunk_kind = read_string_field(
        line,
        "source_chunk_kind",
    )
    .filter(|value| value != taxonomy::UNKNOWN)
    .unwrap_or_else(|| taxonomy::NOT_APPLICABLE.to_owned());
    metadata.source_chunk_ordinal = read_string_field(
        line,
        "source_chunk_ordinal",
    )
    .filter(|value| value != taxonomy::UNKNOWN)
    .unwrap_or_else(|| taxonomy::NOT_APPLICABLE.to_owned());
    metadata.recovery_status = read_string_field(
        line,
        "recovery_status",
    )
    .filter(|value| value != taxonomy::UNKNOWN)
    .unwrap_or_else(|| taxonomy::FULLY_DECODED.to_owned());
}

/// Refine classifications with manifest facts that are already stable enough
/// for export. This lives after generic metadata fill because the base pass
/// establishes safe import behavior first, while this pass only narrows
/// mission and dialog roles using deterministic row evidence.
fn apply_manifest_evidence_classification(
    metadata: &mut MinorUnitMetadata,
    line: &str,
    path: &str,
) {
    if metadata.kind == "mission-script" {
        refine_mission_script(
            metadata, path,
        );
    } else if metadata.kind == "vehicle-tuning" {
        refine_vehicle_tuning(
            metadata, path,
        );
    } else if metadata.type_ == "audio" && is_dialog_audio(path) {
        refine_dialog_audio(
            metadata, line, path,
        );
    }
}

/// Mission script files use generated normalized rows, so deriving a level and
/// role here keeps the public category stable without relying on a hard-coded
/// absolute asset route.
fn refine_mission_script(
    metadata: &mut MinorUnitMetadata,
    path: &str,
) {
    let level = mission_level(path);
    let role = mission_asset_role(path);
    let focus = mission_focus(path);
    metadata.subtype = format!("mission-{level}-{focus}-{role}-script-json");
    metadata.function =
        format!("mission {level} {focus} {role} gameplay sequence");
    metadata.classification_notes = format!(
        "classified-from-generated-mission-index:{level}:{focus}:{role}"
    );
}

/// Vehicle tuning rows share the script stream with missions, so this narrows
/// them to gameplay tuning buckets instead of letting them look like generic
/// configuration data in downstream imports.
fn refine_vehicle_tuning(
    metadata: &mut MinorUnitMetadata,
    path: &str,
) {
    let level = mission_level(path);
    let role = vehicle_tuning_role(path);
    metadata.subtype = format!("vehicle-tuning-{level}-{role}-config-json");
    metadata.function = format!("vehicle tuning {level} {role} configuration");
    metadata.classification_notes = format!(
        "classified-from-generated-vehicle-tuning-index:{level}:{role}"
    );
}

/// Dialog rows are grouped by generated manifest position before filename
/// context so a stable speaker/conversation bucket wins over a fragile leaf
/// pattern whenever both are present.
fn refine_dialog_audio(
    metadata: &mut MinorUnitMetadata,
    line: &str,
    path: &str,
) {
    let group = dialog_group(path);
    let context = dialog_context(path);
    let route_group = read_string_field(
        line,
        "obfuscated_route",
    )
    .map_or_else(
        || "none".to_owned(),
        |route| route_parent_key(&route),
    );
    metadata.subtype = format!("dialog-{group}-{context}-wav-pcm");
    metadata.function = format!("dialog {group} {context} voice line");
    metadata.classification_notes =
        format!("classified-from-dialog-index:{group}:{context}:{route_group}");
}

/// Detect dialog audio using container ids already present in generated rows,
/// keeping speaker/context classification independent from local directories.
fn is_dialog_audio(path: &str) -> bool {
    path.split('/')
        .any(is_dialog_container_segment)
}

/// Keep the accepted dialog container ids in one place so language variants
/// share a category rule instead of duplicating branch logic.
fn is_dialog_container_segment(segment: &str) -> bool {
    matches!(
        segment,
        "dialog" | "dialogf" | "dialogg" | "dialogs"
    )
}

/// Prefer the generated dialog group over the filename because conversation
/// and speaker buckets remain stable when individual line names change.
fn dialog_group(path: &str) -> &'static str {
    let segments = path
        .split('/')
        .collect::<Vec<_>>();
    for (index, segment) in segments
        .iter()
        .enumerate()
    {
        if is_dialog_container_segment(segment)
            && let Some(next) = segments.get(index.saturating_add(1))
        {
            if *next == "conversations" {
                return "conversation";
            }
            return "speaker";
        }
    }
    "context"
}

/// Derive dialog context from compact filename tokens only after the group is
/// known, which prevents conversation rows from collapsing into ad-lib audio.
fn dialog_context(path: &str) -> &'static str {
    let stem = leaf_stem(path);
    if stem.contains("tutorial") || stem.contains("tut") {
        "tutorial"
    } else if has_level_mission_token(stem) {
        "mission"
    } else if dialog_group(path) == "conversation" || stem.starts_with("c_") {
        "conversation"
    } else if stem.starts_with("d_") {
        "ad-lib"
    } else {
        "context"
    }
}

/// Map normalized mission level ids to exported buckets so downstream tools
/// can select a level without reading source filenames or local routes.
fn mission_level(path: &str) -> &'static str {
    for segment in path.split('/') {
        match segment {
            "level01" => return "level-01",
            "level02" => return "level-02",
            "level03" => return "level-03",
            "level04" => return "level-04",
            "level05" => return "level-05",
            "level06" => return "level-06",
            "level07" => return "level-07",
            _ => {}
        }
    }
    "generic"
}

/// Split mission focus from role because tutorial and head-to-head missions
/// need stable subcategories even when they share the same file extension.
fn mission_focus(path: &str) -> &'static str {
    let stem = leaf_stem(path);
    if stem.starts_with("m0") || stem.contains("tutorial") {
        "tutorial"
    } else if stem.contains("head") || stem.contains("h2h") {
        "head-to-head"
    // cspell:disable-next-line -- rwrds
    } else if stem.contains("reward") || stem.contains("rwrds") {
        "generic"
    } else {
        "scripts"
    }
}

/// Convert compact mission suffixes into import roles so init, logic, and
/// stage data stay queryable without exposing raw script names as rules.
fn mission_asset_role(path: &str) -> &'static str {
    let stem = leaf_stem(path);
    // cspell:disable-next-line -- sdi
    if stem.ends_with("sdi") || stem.ends_with("sdl") {
        "stage-data"
    } else if stem.ends_with('i') {
        "init"
    } else if stem.ends_with('l') {
        "logic"
    } else if stem.contains("level") {
        "level-setup"
    } else if stem.contains("demo") {
        "demo"
    } else {
        "script"
    }
}

/// Keep vehicle tuning roles separate from mission scripts because gameplay
/// configuration imports need a narrower destination than generic script data.
fn vehicle_tuning_role(path: &str) -> &'static str {
    let lower = path.to_ascii_lowercase();
    if lower.contains("race") {
        "race"
    } else if lower.contains("chase") || lower.contains("pursuit") {
        "pursuit"
    } else if lower.contains("bonus") {
        "bonus"
    } else if lower.contains("mission") {
        "mission"
    } else if lower.contains("car") {
        "vehicle"
    } else {
        "generic"
    }
}

/// Recognize compact level-mission tokens using byte patterns so dialog
/// context detection stays allocation-free and cannot panic on short stems.
fn has_level_mission_token(stem: &str) -> bool {
    stem.as_bytes()
        .windows(4)
        .any(
            |window| {
                matches!(window, [b'l', level, b'm', mission]
            if level.is_ascii_digit() && mission.is_ascii_digit())
            },
        )
}

/// Use the leaf stem as the only filename-derived evidence because earlier
/// path segments are reserved for generated grouping and package identity.
fn leaf_stem(path: &str) -> &str {
    path.rsplit('/')
        .next()
        .and_then(
            |leaf| {
                leaf.split('.')
                    .next()
            },
        )
        .unwrap_or_default()
}

/// Store only the obfuscated parent coordinate in notes because samples need
/// grouping evidence without reintroducing any local asset names.
fn route_parent_key(route: &str) -> String {
    route
        .rsplit_once('/')
        .map_or_else(
            || "none".to_owned(),
            |(parent, _leaf)| parent.to_owned(),
        )
}

/// FNV-1a 128-bit offset basis. A fully specified hash keeps the identity
/// scheme reproducible across Rust releases, unlike the standard-library
/// hashers whose output is deliberately unspecified.
const FNV_OFFSET_128: u128 = 0x6c62_272e_07bb_0142_62b8_2175_6295_c58d;
/// FNV-1a 128-bit prime.
const FNV_PRIME_128: u128 = 0x0000_0000_0100_0000_0000_0000_0000_013b;

/// Compute the opaque, deterministic unit identity.
///
/// The hash consumes only the name-free route plus the classification, so no
/// real path or file name ever enters the identity. The exact type leads the
/// id, letting a reader recover the taxonomy without a lookup.
pub(in crate::adapters::driven::local::two) fn compute_id(
    obfuscated_route: &str,
    type_: &str,
    recovery_status: &str,
) -> String {
    let seed = format!("{obfuscated_route}|{type_}|{recovery_status}");
    format!(
        "{type_}-{}",
        uuid_shaped(fnv1a_128(&seed))
    )
}

/// Hash a byte string with the 128-bit FNV-1a variant.
fn fnv1a_128(input: &str) -> u128 {
    let mut hash = FNV_OFFSET_128;
    for byte in input.bytes() {
        hash ^= u128::from(byte);
        hash = hash.wrapping_mul(FNV_PRIME_128);
    }
    hash
}

/// Render a 128-bit value in the canonical 8-4-4-4-12 uuid layout so downstream
/// readers can treat the identity as a uuid without implying it is random.
fn uuid_shaped(value: u128) -> String {
    let hex = format!("{value:032x}");
    let mut out = String::with_capacity(36);
    for (index, character) in hex
        .chars()
        .enumerate()
    {
        if matches!(
            index,
            8 | 12 | 16 | 20
        ) {
            out.push('-');
        }
        out.push(character);
    }
    out
}

/// Read string field.
pub(super) fn read_string_field(
    line: &str,
    field: &str,
) -> Option<String> {
    let needle = format!("\"{field}\":\"");
    let start = line
        .find(&needle)?
        .saturating_add(needle.len());
    let rest = line.get(start..)?;
    let end = rest.find('"')?;
    Some(
        rest.get(..end)?
            .to_owned(),
    )
}

/// Io error.
fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> PipelineError + '_ {
    move |error| {
        PipelineError::new(
            format!(
                "{}: {error}",
                path.display()
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::compute_id;

    #[test]
    fn id_is_deterministic_and_type_prefixed() {
        let first = compute_id(
            "ed/at/#1.json",
            "model",
            "fully-decoded",
        );
        let second = compute_id(
            "ed/at/#1.json",
            "model",
            "fully-decoded",
        );
        assert_eq!(
            first,
            second
        );
        assert!(first.starts_with("model-"));
    }

    #[test]
    fn id_changes_with_route_and_recovery() {
        let base = compute_id(
            "ed/at/#1.json",
            "model",
            "fully-decoded",
        );
        let other_route = compute_id(
            "ed/at/#2.json",
            "model",
            "fully-decoded",
        );
        let other_type = compute_id(
            "ed/at/#1.json",
            "image",
            "fully-decoded",
        );
        assert_ne!(
            base,
            other_route
        );
        assert_ne!(
            base,
            other_type
        );
    }

    #[test]
    fn id_values_are_pinned_across_machines_and_releases() {
        for (route, unit_type, status, expected) in [
            (
                "ed/#1.md",
                "text",
                "fully-decoded",
                "text-1bb89d0a-0b6b-26bc-9356-9ba8d43d1c54",
            ),
            (
                "ed/ae/sd/ae/#1.wav",
                "audio",
                "fully-decoded",
                "audio-d43c57f7-259e-1172-0559-dd6548e4bf9e",
            ),
            (
                "ed/at/b7/cs/mh/#1.json",
                "model",
                "fully-decoded",
                "model-6558a19c-512f-46ce-1b3f-0b904343b3d6",
            ),
            (
                "ed/at/b7/cs/sg/#1.json",
                "world",
                "fully-decoded",
                "world-c9e85a29-c47c-569e-8555-f7981aeab284",
            ),
        ] {
            assert_eq!(
                compute_id(
                    route, unit_type, status,
                ),
                expected
            );
        }
    }

    #[test]
    fn id_suffix_is_uuid_shaped() {
        let id = compute_id(
            "ed/at/#1.json",
            "image",
            "fully-decoded",
        );
        let suffix = id.strip_prefix("image-");
        assert_eq!(
            suffix.map(str::len),
            Some(36)
        );
        assert_eq!(
            suffix.map(
                |value| value
                    .matches('-')
                    .count()
            ),
            Some(4)
        );
    }
}
