// File:
//   - taxonomy.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/taxonomy.rs
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
//   - The taxonomy contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute taxonomy.
// - Split-When:
//   - Split when taxonomy contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Error sentinel used when a value would otherwise hide decoder debt.
// - Description:
//   - Defines taxonomy data and behavior for pipeline phase two minor units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs taxonomy.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Error sentinel used when a value would otherwise hide decoder
//   - debt keeps tightly coupled validation, ordering, and deterministic
//   - transformation invariants together; split when a stable independently
//   - testable sub-boundary is identified.
//

//! Error sentinel used when a value would otherwise hide decoder debt.
use std::path::{Path, PathBuf};

/// Error sentinel used when a value would otherwise hide decoder debt.
pub(super) const UNKNOWN: &str = "error";
/// Output dir name.
pub(super) const OUTPUT_DIR_NAME: &str = "minor-unit";
/// Manifest file name.
pub(super) const MANIFEST_FILE_NAME: &str = "manifest.jsonl";
/// Audit file name.
pub(super) const AUDIT_FILE_NAME: &str = "audit.json";
/// Taxonomy json.
pub(super) const TAXONOMY_JSON: &str = include_str!("minor_unit_taxonomy.json");

/// Classification fields.
///
/// These carry decoded meaning and must never remain the [`UNKNOWN`] sentinel
/// after metadata fill; the audit treats a leftover `error` here as a
/// failure. `id`, `obfuscated_route`, and `source_chunk_ordinal` are excluded
/// because they are computed identity/coordinate values that are never
/// classified — they are enforced only for presence.
pub(super) const CLASSIFICATION_FIELDS: &[&str] = &[
    "type",
    "subtype",
    "kind",
    "function",
    "schema",
    "origin",
    "source_path",
    "source_extension",
    "source_container",
    "source_chunk_kind",
    "recovery_status",
    "derived_from",
    "size_bytes",
    "unreal_import_relation",
    "future_normalization",
    "component_links",
    "classification_notes",
];

/// Required fields.
///
/// `id` and `obfuscated_route` lead the provenance columns so a reader can key
/// on a name-free identity without ever depending on the exact `path`, which is
/// retained only for local file resolution.
pub(super) const REQUIRED_FIELDS: &[&str] = &[
    "path",
    "id",
    "obfuscated_route",
    "file_extension",
    "type",
    "subtype",
    "kind",
    "function",
    "schema",
    "origin",
    "source_path",
    "source_extension",
    "source_container",
    "source_chunk_kind",
    "source_chunk_ordinal",
    "recovery_status",
    "derived_from",
    "size_bytes",
    "unreal_import_relation",
    "future_normalization",
    "component_links",
    "classification_notes",
];

/// Type values.
const TYPE_VALUES: &[&str] = &[
    "audio",
    "image",
    "model",
    "material",
    "animation",
    "script",
    "text",
    "metadata",
    "table",
    "movie-video",
    "movie-audio",
    "package-component",
    "config",
    "ui",
    "localization",
    "world",
    "physics",
    "camera",
    "light",
    "particle",
    "controller",
    "scene",
    "locator",
    UNKNOWN,
];

/// Kind values.
const KIND_VALUES: &[&str] = &[
    "runtime-asset",
    "derived-component",
    "package-manifest",
    "decode-report",
    "timing-table",
    "localization-override",
    "audio-override",
    "movie-audio",
    "gameplay-rule",
    "ui-asset",
    "vehicle-asset",
    "world-asset",
    "editor-only-metadata",
    "mission-script",
    "vehicle-tuning",
    "ui-layout",
    "localization-table",
    "sound-metadata",
    "junk-artifact",
    "choreography-bank",
    "p3d-shader",
    "p3d-attribute",
    "p3d-mesh",
    "p3d-skin",
    "p3d-composite-drawable",
    "p3d-sprite",
    "p3d-animation",
    "p3d-skeleton",
    "p3d-controller",
    "p3d-camera",
    "p3d-light",
    "p3d-particle",
    "p3d-scenegraph",
    "p3d-locator",
    "p3d-road-network",
    "p3d-ped-path",
    "p3d-collision",
    "p3d-physics",
    "p3d-world-dsg",
    "p3d-texture",
    "p3d-texture-font",
    "p3d-text-bible",
    "p3d-export-info",
    "p3d-scrooby-project",
    "p3d-vertex-animation",
    "p3d-animated-prop",
    "p3d-light-group",
    UNKNOWN,
];

/// Origin values.
const ORIGIN_VALUES: &[&str] = &[
    "game-root",
    "rcf-expansion",
    "p3d-package",
    "rmv-decode",
    "rsd-decode",
    "lmlm-override",
    "spt-decode",
    "rms-decode",
    "readme-rtf-decode",
    "metadata-fill",
    "game-straggler-normalize",
    UNKNOWN,
];

/// Unreal import relation values.
const UNREAL_IMPORT_RELATION_VALUES: &[&str] = &[
    "import-direct",
    "import-after-conversion",
    "import-as-data-asset",
    "import-as-media-source",
    "compose-into-asset",
    "editor-only-metadata",
    "do-not-import",
    UNKNOWN,
];

/// Future normalization values.
const FUTURE_NORMALIZATION_VALUES: &[&str] = &[
    NOT_APPLICABLE,
    "keep",
    "wav-to-soundwave",
    "hap-movie-to-media-source",
    "png-to-texture2d",
    "bmp-to-texture2d",
    "dds-to-texture2d",
    "tga-to-texture2d",
    "json-to-data-asset",
    "tsv-to-data-table",
    "p3d-component-to-mesh-material-texture",
    "script-to-unreal-logic",
    "mission-json-to-statetree",
    "vehicle-json-to-data-asset",
    "ui-json-to-umg",
    "localization-json-to-string-table",
    "sound-metadata-json-to-data-asset",
    "choreography-json-to-animation-data",
    "junk-to-ignore",
    "p3d-shader-to-material",
    "p3d-attribute-to-physical-material",
    "p3d-mesh-to-static-mesh",
    "p3d-skin-to-skeletal-mesh",
    "p3d-composite-drawable-to-blueprint",
    "p3d-sprite-to-texture2d",
    "p3d-animation-to-animation-asset",
    "p3d-skeleton-to-skeleton-asset",
    "p3d-controller-to-animation",
    "p3d-camera-to-camera-actor",
    "p3d-light-to-light-component",
    "p3d-particle-to-niagara",
    "p3d-scenegraph-to-level-hierarchy",
    "p3d-locator-to-scene-component",
    "p3d-road-to-spline-network",
    "p3d-ped-path-to-spline",
    "p3d-physics-to-collision",
    "p3d-world-dsg-to-actor-data",
    "p3d-texture-to-texture2d",
    "p3d-texture-font-to-font-asset",
    "p3d-text-bible-to-string-table",
    "p3d-export-info-to-editor-metadata",
    "p3d-scrooby-project-to-ui-project",
    "p3d-vertex-animation-to-morph-targets",
    "p3d-animated-prop-to-actor-blueprint",
    "p3d-light-group-to-light-rig",
    UNKNOWN,
];

/// Recovery status values.
///
/// Only fully decoded/materialized units are valid extraction outputs. Anything
/// else must fail before the manifest is accepted, because alternate success
/// states hide decoder debt and payload loss.
const RECOVERY_STATUS_VALUES: &[&str] = &[
    "fully-decoded",
    UNKNOWN,
];

/// Sentinel used for provenance columns of units that are not decoded from a
/// source chunk, so they are truthful without borrowing the `error` failure
/// sentinel that the audit rejects.
pub(super) const NOT_APPLICABLE: &str = "none";

/// Successful recovery status for every accepted unit, including files that
/// were materialized from another container instead of decoded from a P3D
/// chunk.
pub(super) const FULLY_DECODED: &str = "fully-decoded";

#[must_use]
/// Map a raw `components.jsonl` recovery status onto the controlled taxonomy
/// value, so extractor-internal spellings never leak into the ledger.
pub(super) fn map_recovery_status(raw: &str) -> &'static str {
    match raw {
        "decoded_schema_payload" | "recovered_embedded_image_payload" => {
            FULLY_DECODED
        }
        // Any other extractor status means decoder debt or intentional
        // omission; the audit rejects it instead of normalizing it into
        // another success mode.
        _ => UNKNOWN,
    }
}

#[must_use]
/// Output dir.
pub(super) fn output_dir(extracted_root: &Path) -> PathBuf {
    extracted_root.join(OUTPUT_DIR_NAME)
}

#[must_use]
/// Manifest path.
pub(super) fn manifest_path(extracted_root: &Path) -> PathBuf {
    output_dir(extracted_root).join(MANIFEST_FILE_NAME)
}

#[must_use]
/// Audit path.
pub(super) fn audit_path(extracted_root: &Path) -> PathBuf {
    output_dir(extracted_root).join(AUDIT_FILE_NAME)
}

#[must_use]
/// Controlled values.
pub(super) fn controlled_values(
    field: &str
) -> Option<&'static [&'static str]> {
    match field {
        "type" => Some(TYPE_VALUES),
        "kind" => Some(KIND_VALUES),
        "origin" => Some(ORIGIN_VALUES),
        "unreal_import_relation" => Some(UNREAL_IMPORT_RELATION_VALUES),
        "future_normalization" => Some(FUTURE_NORMALIZATION_VALUES),
        "recovery_status" => Some(RECOVERY_STATUS_VALUES),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::map_recovery_status;

    #[test]
    fn maps_decoded_and_image_payloads_to_fully_decoded() {
        assert_eq!(
            map_recovery_status("decoded_schema_payload"),
            "fully-decoded"
        );
        assert_eq!(
            map_recovery_status("recovered_embedded_image_payload"),
            "fully-decoded"
        );
    }

    #[test]
    fn maps_unrecognized_status_to_error_sentinel() {
        assert_eq!(
            map_recovery_status("something_new"),
            super::UNKNOWN
        );
    }
}
