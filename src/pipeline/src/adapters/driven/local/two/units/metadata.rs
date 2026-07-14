// File:
//   - metadata.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/metadata.rs
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
//   - The metadata contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute metadata.
// - Split-When:
//   - Split when metadata contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - and never used to locate the file on disk.
// - Description:
//   - Defines metadata data and behavior for pipeline phase two minor units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs metadata.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: and never used to locate the file on disk keeps tightly
//   - coupled validation, ordering, and deterministic transformation
//   - invariants together; split when a stable independently testable sub-
//   - boundary is identified.
//

//! and never used to locate the file on disk.
//!
//! This boundary keeps and never used to locate the file on disk explicit and
//! returns deterministic results to pipeline callers.
use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use super::taxonomy::UNKNOWN;
use crate::domain::PipelineError;

/// Result.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Complete derived metadata for one normalized minor unit.
#[derive(Debug, Clone)]
pub(super) struct MinorUnitMetadata {
    /// Opaque, deterministic identity: `<type>-<uuid-shaped hash>`. Name-free
    /// and never used to locate the file on disk.
    pub id: String,
    /// Name-free route carried over from the manifest stage.
    pub obfuscated_route: String,
    /// Source chunk kind from package provenance, or the not-applicable
    /// sentinel for loose files.
    pub source_chunk_kind: String,
    /// Source chunk ordinal from package provenance, or the not-applicable
    /// sentinel.
    pub source_chunk_ordinal: String,
    /// Recovery outcome from the lossless extraction vocabulary.
    pub recovery_status: String,
    /// Type .
    pub type_: String,
    /// Subtype.
    pub subtype: String,
    /// Kind.
    pub kind: String,
    /// Function.
    pub function: String,
    /// Schema.
    pub schema: String,
    /// Origin.
    pub origin: String,
    /// Source path.
    pub source_path: String,
    /// Source extension.
    pub source_extension: String,
    /// Source container.
    pub source_container: String,
    /// Derived from.
    pub derived_from: String,
    /// Size bytes.
    pub size_bytes: String,
    /// Unreal import relation.
    pub unreal_import_relation: String,
    /// Future normalization.
    pub future_normalization: String,
    /// Component links.
    pub component_links: String,
    /// Classification notes.
    pub classification_notes: String,
}

impl MinorUnitMetadata {
    /// Error.
    fn error() -> Self {
        Self {
            id: UNKNOWN.to_owned(),
            obfuscated_route: UNKNOWN.to_owned(),
            source_chunk_kind: UNKNOWN.to_owned(),
            source_chunk_ordinal: UNKNOWN.to_owned(),
            recovery_status: UNKNOWN.to_owned(),
            type_: UNKNOWN.to_owned(),
            subtype: UNKNOWN.to_owned(),
            kind: UNKNOWN.to_owned(),
            function: UNKNOWN.to_owned(),
            schema: UNKNOWN.to_owned(),
            origin: UNKNOWN.to_owned(),
            source_path: UNKNOWN.to_owned(),
            source_extension: UNKNOWN.to_owned(),
            source_container: UNKNOWN.to_owned(),
            derived_from: UNKNOWN.to_owned(),
            size_bytes: UNKNOWN.to_owned(),
            unreal_import_relation: UNKNOWN.to_owned(),
            future_normalization: UNKNOWN.to_owned(),
            component_links: UNKNOWN.to_owned(),
            classification_notes: UNKNOWN.to_owned(),
        }
    }
}

/// Classify minor unit.
///
/// # Errors
///
/// Returns an error when source evidence cannot be inspected or normalized.
// One decision tree must assign complete metadata before finalization.
#[expect(
    clippy::too_many_lines,
    reason = "One decision tree evaluates source families and assigns every \
              metadata field before finalization."
)]
pub(super) fn classify_minor_unit(
    extracted_root: &Path,
    manifest_path: &str,
    file_extension: &str,
) -> PipelineOutcome<MinorUnitMetadata> {
    let mut meta = MinorUnitMetadata::error();
    let route = RouteSignature::from_manifest_path(manifest_path);
    let ext = file_extension.to_ascii_lowercase();

    let header = manifest_path
        .strip_prefix("extracted/")
        .map(|relative| extracted_root.join(relative))
        .filter(|unit_path| unit_path.is_file())
        .map_or_else(
            || Ok(Vec::new()),
            |unit_path| {
                read_prefix(
                    &unit_path, 4096,
                )
            },
        )?;
    let header_text = String::from_utf8_lossy(&header).to_ascii_lowercase();

    if route.has(RouteFeature::MovieVideo) && ext == "mov" {
        apply_classification(
            &mut meta,
            MetadataClassification::new(
                "movie-video",
                "hap-mov",
                "runtime-asset",
                "HAP cinematic video",
                "rmv-decode",
                "import-as-media-source",
                "hap-movie-to-media-source",
            ),
        );
    } else if route.has(RouteFeature::MovieAudio) && ext == "wav" {
        apply_classification(
            &mut meta,
            MetadataClassification::new(
                "audio",
                "wav-pcm",
                "movie-audio",
                "movie audio track",
                route.audio_origin(),
                "import-after-conversion",
                "wav-to-soundwave",
            ),
        );
    } else if ext == "wav" {
        apply_classification(
            &mut meta,
            MetadataClassification::new(
                "audio",
                "wav-pcm",
                route.audio_kind(),
                "decoded audio",
                route.audio_origin(),
                "import-after-conversion",
                "wav-to-soundwave",
            ),
        );
    } else if matches!(
        ext.as_str(),
        "png" | "bmp" | "dds" | "tga"
    ) {
        classify_image(
            &mut meta, &route, &ext,
        );
    } else if ext == "json" || ext == "jsonl" {
        classify_json_like(
            &mut meta,
            &route,
            &ext,
            &header_text,
        );
    } else if ext == "tsv" {
        apply_classification(
            &mut meta,
            MetadataClassification::new(
                "table",
                "tsv",
                if route.has(RouteFeature::Timing) {
                    "timing-table"
                } else {
                    "runtime-asset"
                },
                "tabular data",
                "rmv-decode",
                "import-as-data-asset",
                "tsv-to-data-table",
            ),
        );
    } else if ext == "ini" {
        apply_classification(
            &mut meta,
            MetadataClassification::new(
                "text",
                "ini",
                if route.has(RouteFeature::Lmlm) {
                    "localization-override"
                } else {
                    "runtime-asset"
                },
                "configuration or localization text",
                route.text_origin(),
                "import-as-data-asset",
                "json-to-data-asset",
            ),
        );
    } else if ext == "md" {
        apply_classification(
            &mut meta,
            MetadataClassification::new(
                "text",
                "markdown",
                "editor-only-metadata",
                "human documentation",
                "readme-rtf-decode",
                "editor-only-metadata",
                "keep",
            ),
        );
    } else if ext == "bin" && route.has(RouteFeature::P3dComponent) {
        apply_classification(
            &mut meta,
            MetadataClassification::new(
                "package-component",
                "p3d-binary-chunk",
                "derived-component",
                "raw package component",
                "p3d-package",
                "compose-into-asset",
                "p3d-component-to-mesh-material-texture",
            ),
        );
    }

    if meta.type_ == UNKNOWN {
        classify_legacy_extension(
            &mut meta, &route, &ext,
        );
    }
    finalize_metadata(
        &mut meta,
        extracted_root,
        manifest_path,
        &ext,
        &route,
    )?;
    Ok(meta)
}

/// Stable route features derived from one normalized manifest path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RouteFeature {
    /// Route belongs to an LMLM override package.
    Lmlm,
    /// Route belongs to a movie package.
    MoviePackage,
    /// Route names the movie video output.
    MovieVideo,
    /// Route names one movie audio output.
    MovieAudio,
    /// Route belongs to a decoded P3D component tree.
    P3dComponent,
    /// Route names a timing manifest.
    Timing,
}

/// Parsed route evidence used by metadata classification.
#[derive(Debug, Clone)]
struct RouteSignature {
    /// Typed route features detected from normalized segments.
    features: BTreeSet<RouteFeature>,
    /// Lowercase final path segment.
    leaf: String,
}

impl RouteSignature {
    /// From manifest path.
    fn from_manifest_path(path: &str) -> Self {
        let lower = path.to_ascii_lowercase();
        let segments = lower
            .split('/')
            .collect::<Vec<_>>();
        let leaf = segments
            .last()
            .copied()
            .unwrap_or_default()
            .to_owned();
        let mut features = BTreeSet::new();
        if segments.contains(&"lmlm") {
            let _ = features.insert(RouteFeature::Lmlm);
        }
        let movie_package = segments.contains(&"movies");
        if movie_package {
            let _ = features.insert(RouteFeature::MoviePackage);
        }
        if movie_package && leaf == "movie.mov" {
            let _ = features.insert(RouteFeature::MovieVideo);
        }
        if movie_package && leaf.starts_with("audio_track_") {
            let _ = features.insert(RouteFeature::MovieAudio);
        }
        if segments.contains(&"components") {
            let _ = features.insert(RouteFeature::P3dComponent);
        }
        if leaf == "timing.tsv" {
            let _ = features.insert(RouteFeature::Timing);
        }
        Self {
            features,
            leaf,
        }
    }

    /// Returns whether the route carries one typed feature.
    fn has(
        &self,
        feature: RouteFeature,
    ) -> bool {
        self.features
            .contains(&feature)
    }

    /// Audio origin.
    fn audio_origin(&self) -> &'static str {
        if self.has(RouteFeature::Lmlm) {
            "lmlm-override"
        } else if self.has(RouteFeature::MovieAudio) {
            "rmv-decode"
        } else {
            "rsd-decode"
        }
    }

    /// Audio kind.
    fn audio_kind(&self) -> &'static str {
        if self.has(RouteFeature::Lmlm) {
            "audio-override"
        } else {
            "runtime-asset"
        }
    }

    /// Text origin.
    fn text_origin(&self) -> &'static str {
        if self.has(RouteFeature::Lmlm) {
            "lmlm-override"
        } else {
            "game-root"
        }
    }
}

/// Classify image.
fn classify_image(
    meta: &mut MinorUnitMetadata,
    route: &RouteSignature,
    ext: &str,
) {
    let subtype = format!("{ext}-texture");
    let future = format!("{ext}-to-texture2d");
    apply_classification(
        meta,
        MetadataClassification::new(
            "image",
            &subtype,
            if route.has(RouteFeature::P3dComponent) {
                "derived-component"
            } else {
                "runtime-asset"
            },
            "image asset",
            if route.has(RouteFeature::P3dComponent) {
                "p3d-package"
            } else {
                "rcf-expansion"
            },
            "import-after-conversion",
            &future,
        ),
    );
}

/// Classify json like.
fn classify_json_like(
    meta: &mut MinorUnitMetadata,
    route: &RouteSignature,
    ext: &str,
    text: &str,
) {
    if classify_straggler_json(
        meta, text,
    ) {
        return;
    }
    if text.contains("p3d.package.v1") || route.leaf == "components.jsonl" {
        apply_classification(
            meta,
            MetadataClassification::new(
                "metadata",
                ext,
                "package-manifest",
                "package component manifest",
                "p3d-package",
                "editor-only-metadata",
                "keep",
            ),
        );
        if text.contains("p3d.package.v1") {
            "p3d.package.v1".clone_into(&mut meta.schema);
        } else {
            "p3d.components-jsonl".clone_into(&mut meta.schema);
        }
    } else if text.contains("shar-schoenwald.radmusic-compiled.v3") {
        apply_classification(
            meta,
            MetadataClassification::new(
                "script",
                "radmusic-json",
                "runtime-asset",
                "compiled music script metadata",
                "rms-decode",
                "import-as-data-asset",
                "json-to-data-asset",
            ),
        );
        "shar-schoenwald.radmusic-compiled.v3".clone_into(&mut meta.schema);
    } else if classify_movie_json(
        meta, route,
    ) {
    } else if route.leaf == "manifest.json" && route.has(RouteFeature::Lmlm) {
        apply_classification(
            meta,
            MetadataClassification::new(
                "metadata",
                "lmlm-manifest-json",
                "package-manifest",
                "override manifest",
                "lmlm-override",
                "editor-only-metadata",
                "keep",
            ),
        );
    } else if route.has(RouteFeature::P3dComponent)
        || text.contains("schema_ref")
        || text.contains("payload_format")
    {
        apply_classification(
            meta,
            MetadataClassification::new(
                "metadata",
                ext,
                "derived-component",
                "package component metadata",
                "p3d-package",
                "compose-into-asset",
                "p3d-component-to-mesh-material-texture",
            ),
        );
    } else {
        apply_classification(
            meta,
            MetadataClassification::new(
                "metadata",
                ext,
                "runtime-asset",
                "structured data",
                "rcf-expansion",
                "import-as-data-asset",
                "json-to-data-asset",
            ),
        );
    }
}

/// Immutable metadata classification selected from source evidence.
#[derive(Debug, Clone, Copy)]
struct MetadataClassification<'value> {
    /// Stable top-level type.
    type_: &'value str,
    /// Stable subtype within the top-level type.
    subtype: &'value str,
    /// Runtime or editor behavior kind.
    kind: &'value str,
    /// Human-readable functional role.
    function: &'value str,
    /// Provenance family that produced the unit.
    origin: &'value str,
    /// Expected Unreal import relationship.
    unreal_import_relation: &'value str,
    /// Required future normalization operation.
    future_normalization: &'value str,
}

impl<'value> MetadataClassification<'value> {
    /// Creates one complete classification assignment.
    const fn new(
        type_: &'value str,
        subtype: &'value str,
        kind: &'value str,
        function: &'value str,
        origin: &'value str,
        unreal_import_relation: &'value str,
        future_normalization: &'value str,
    ) -> Self {
        Self {
            type_,
            subtype,
            kind,
            function,
            origin,
            unreal_import_relation,
            future_normalization,
        }
    }
}

/// Applies one complete classification assignment to mutable metadata.
fn apply_classification(
    metadata: &mut MinorUnitMetadata,
    classification: MetadataClassification<'_>,
) {
    classification
        .type_
        .clone_into(&mut metadata.type_);
    classification
        .subtype
        .clone_into(&mut metadata.subtype);
    classification
        .kind
        .clone_into(&mut metadata.kind);
    classification
        .function
        .clone_into(&mut metadata.function);
    classification
        .origin
        .clone_into(&mut metadata.origin);
    classification
        .unreal_import_relation
        .clone_into(&mut metadata.unreal_import_relation);
    classification
        .future_normalization
        .clone_into(&mut metadata.future_normalization);
}

/// Read prefix.
fn read_prefix(
    path: &Path,
    limit: usize,
) -> PipelineOutcome<Vec<u8>> {
    let mut file = File::open(path).map_err(
        |error| {
            PipelineError::new(
                format!(
                    "failed to read metadata header {}: {error}",
                    path.display()
                ),
            )
        },
    )?;
    let mut buffer = vec![0; limit];
    let count = file
        .read(&mut buffer)
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "failed to read metadata header {}: {error}",
                        path.display()
                    ),
                )
            },
        )?;
    buffer.truncate(count);
    Ok(buffer)
}

/// Classify legacy extension.
fn classify_legacy_extension(
    meta: &mut MinorUnitMetadata,
    route: &RouteSignature,
    ext: &str,
) {
    match ext {
        "mfk" => apply_classification(
            meta,
            MetadataClassification::new(
                "script",
                "mfk-script",
                "mission-script",
                "mission gameplay source script",
                "game-root",
                "import-as-data-asset",
                "mission-json-to-statetree",
            ),
        ),
        "con" => apply_classification(
            meta,
            MetadataClassification::new(
                "config",
                "con-script",
                "vehicle-tuning",
                "vehicle gameplay tuning source",
                "game-root",
                "import-as-data-asset",
                "vehicle-json-to-data-asset",
            ),
        ),
        "pag" | "scr" | "prj" => apply_classification(
            meta,
            MetadataClassification::new(
                "ui",
                ext,
                "ui-layout",
                "Scrooby UI source",
                "game-root",
                "import-as-data-asset",
                "ui-json-to-umg",
            ),
        ),
        "cho" => apply_classification(
            meta,
            MetadataClassification::new(
                "animation",
                "cho",
                "choreography-bank",
                "character choreography source",
                "game-root",
                "import-as-data-asset",
                "choreography-json-to-animation-data",
            ),
        ),
        "txt" | "e" | "f" | "g" | "i" | "s" | "x" => apply_classification(
            meta,
            MetadataClassification::new(
                "localization",
                ext,
                "localization-table",
                "TextBible source",
                "game-root",
                "import-as-data-asset",
                "localization-json-to-string-table",
            ),
        ),
        "typ" => apply_classification(
            meta,
            MetadataClassification::new(
                "metadata",
                "typ",
                "sound-metadata",
                "sound type metadata source",
                "game-root",
                "import-as-data-asset",
                "sound-metadata-json-to-data-asset",
            ),
        ),
        "err" => apply_classification(
            meta,
            MetadataClassification::new(
                "metadata",
                "err",
                "junk-artifact",
                "build/export error log",
                "game-root",
                "do-not-import",
                "junk-to-ignore",
            ),
        ),
        _ => apply_classification(
            meta,
            MetadataClassification::new(
                "metadata",
                ext,
                "runtime-asset",
                "fallback minor unit",
                route.origin_for_non_straggler(),
                "import-as-data-asset",
                "error",
            ),
        ),
    }
}

/// Finalize metadata.
fn finalize_metadata(
    meta: &mut MinorUnitMetadata,
    extracted_root: &Path,
    manifest_path: &str,
    ext: &str,
    route: &RouteSignature,
) -> PipelineOutcome<()> {
    if meta.schema == UNKNOWN {
        schema_hint(
            ext, route,
        )
        .clone_into(&mut meta.schema);
    }
    manifest_path.clone_into(&mut meta.source_path);
    meta.source_extension = if ext == UNKNOWN || ext.is_empty() {
        "none".to_owned()
    } else {
        ext.to_owned()
    };
    route
        .container()
        .clone_into(&mut meta.source_container);
    derived_from(manifest_path).clone_into(&mut meta.derived_from);
    meta.size_bytes = size_value(
        extracted_root,
        manifest_path,
    )?;
    meta.component_links = if route.has(RouteFeature::P3dComponent) {
        "component-sibling-set".to_owned()
    } else {
        "none".to_owned()
    };
    route
        .note()
        .clone_into(&mut meta.classification_notes);
    replace_remaining_errors(meta);
    Ok(())
}

/// Replace remaining errors.
fn replace_remaining_errors(meta: &mut MinorUnitMetadata) {
    for value in [
        &mut meta.subtype,
        &mut meta.kind,
        &mut meta.function,
        &mut meta.schema,
        &mut meta.origin,
        &mut meta.source_path,
        &mut meta.source_extension,
        &mut meta.source_container,
        &mut meta.derived_from,
        &mut meta.size_bytes,
        &mut meta.unreal_import_relation,
        &mut meta.future_normalization,
        &mut meta.component_links,
        &mut meta.classification_notes,
    ] {
        if value == UNKNOWN || value.is_empty() {
            "none".clone_into(value);
        }
    }
}

/// Schema hint.
fn schema_hint(
    ext: &str,
    route: &RouteSignature,
) -> &'static str {
    if route.has(RouteFeature::P3dComponent) || route.leaf == "components.jsonl"
    {
        "p3d.components-jsonl"
    } else if ext == "json" {
        "json"
    } else if ext == "jsonl" {
        "jsonl"
    } else if ext == "wav" {
        "wav-pcm"
    } else {
        "none"
    }
}

/// Derived from.
fn derived_from(path: &str) -> &str {
    if path.starts_with("extracted/game/") {
        "game-straggler"
    } else if path.starts_with("extracted/lmlm/") {
        "lmlm"
    } else if path.starts_with("extracted/movies/") {
        "rmv"
    } else if path.starts_with(
        &format!(
            "{}/",
            "extracted"
        ),
    ) {
        "extracted"
    } else if path.starts_with("game/") {
        "game"
    } else {
        "none"
    }
}

/// File len with retry.
fn file_len_with_retry(path: &Path) -> PipelineOutcome<u64> {
    for attempt in 0_u8..20_u8 {
        match fs::metadata(path) {
            Ok(metadata) => return Ok(metadata.len()),
            Err(error)
                if error.kind() == std::io::ErrorKind::NotFound
                    && attempt < 19 =>
            {
                // Windows can report a just-written component through directory
                // iteration before metadata lookup can reopen it; retrying
                // keeps the manifest strict without accepting
                // missing payloads.
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            Err(error) => {
                return Err(
                    PipelineError::new(
                        format!(
                            "{}: {error}",
                            path.display()
                        ),
                    ),
                );
            }
        }
    }
    Err(
        PipelineError::new(
            format!(
                "{}: metadata retry exhausted",
                path.display()
            ),
        ),
    )
}

/// Size value.
fn size_value(
    extracted_root: &Path,
    manifest_path: &str,
) -> PipelineOutcome<String> {
    let source_disk_path = manifest_path
        .strip_prefix("extracted/")
        .map(|relative| extracted_root.join(relative))
        .or_else(
            || {
                manifest_path
                    .strip_prefix("game/")
                    .map(
                        |relative| {
                            std::path::PathBuf::from("game").join(relative)
                        },
                    )
            },
        );
    if let Some(existing_path) =
        source_disk_path.filter(|candidate_path| candidate_path.is_file())
    {
        return Ok(file_len_with_retry(&existing_path)?.to_string());
    }
    Ok("not-materialized".to_owned())
}

impl RouteSignature {
    /// Origin for non straggler.
    fn origin_for_non_straggler(&self) -> &'static str {
        if self.has(RouteFeature::Lmlm) {
            "lmlm-override"
        } else if self.has(RouteFeature::P3dComponent)
            || self.leaf == "components.jsonl"
        {
            "p3d-package"
        } else {
            "rcf-expansion"
        }
    }

    /// Container.
    fn container(&self) -> &'static str {
        if self.has(RouteFeature::Lmlm) {
            "lmlm-overlay"
        } else if self.has(RouteFeature::MovieAudio)
            || self.has(RouteFeature::Timing)
            || self.has(RouteFeature::MovieVideo)
        {
            "movie-package"
        } else if self.has(RouteFeature::P3dComponent)
            || self.leaf == "components.jsonl"
        {
            "p3d-package"
        } else {
            "extracted-tree"
        }
    }

    /// Note.
    fn note(&self) -> &'static str {
        if self.has(RouteFeature::Lmlm) {
            "classified-from-lmlm-overlay-route"
        } else if self.has(RouteFeature::MovieAudio)
            || self.has(RouteFeature::MovieVideo)
        {
            "classified-from-movie-package-route"
        } else if self.has(RouteFeature::P3dComponent) {
            "classified-from-p3d-component-route"
        } else {
            "classified-from-extension-route-and-header"
        }
    }
}

/// Classify movie json.
fn classify_movie_json(
    meta: &mut MinorUnitMetadata,
    route: &RouteSignature,
) -> bool {
    if route.leaf == "decode-report.json"
        || route.leaf == "source-video.ffprobe.json"
    {
        apply_classification(
            meta,
            MetadataClassification::new(
                "metadata",
                "decode-report-json",
                "decode-report",
                "media decode report",
                "rmv-decode",
                "editor-only-metadata",
                "keep",
            ),
        );
        true
    } else if route.leaf == "manifest.json"
        && route.has(RouteFeature::MoviePackage)
    {
        apply_classification(
            meta,
            MetadataClassification::new(
                "metadata",
                "rmv-manifest-json",
                "package-manifest",
                "movie package manifest",
                "rmv-decode",
                "editor-only-metadata",
                "keep",
            ),
        );
        true
    } else {
        false
    }
}

/// Classify straggler json.
fn classify_straggler_json(
    meta: &mut MinorUnitMetadata,
    text: &str,
) -> bool {
    let cases = [
        (
            "straggler.mission-script",
            "script",
            "mission-script-json",
            "mission-script",
            "mission gameplay sequence",
            "mission-json-to-statetree",
        ),
        (
            "straggler.config-script",
            "config",
            "vehicle-config-json",
            "vehicle-tuning",
            "vehicle gameplay tuning",
            "vehicle-json-to-data-asset",
        ),
        (
            "straggler.scrooby-page",
            "ui",
            "scrooby-page-json",
            "ui-layout",
            "frontend page layout",
            "ui-json-to-umg",
        ),
        (
            "straggler.scrooby-screen",
            "ui",
            "scrooby-screen-json",
            "ui-layout",
            "frontend screen flow",
            "ui-json-to-umg",
        ),
        (
            "straggler.scrooby-project",
            "ui",
            "scrooby-project-json",
            "editor-only-metadata",
            "frontend project metadata",
            "keep",
        ),
        (
            "straggler.choreography",
            "animation",
            "choreography-json",
            "choreography-bank",
            "character choreography metadata",
            "choreography-json-to-animation-data",
        ),
        (
            "straggler.text-bible",
            "localization",
            "text-bible-json",
            "localization-table",
            "localized text table",
            "localization-json-to-string-table",
        ),
        (
            "straggler.sound-type",
            "metadata",
            "sound-type-json",
            "sound-metadata",
            "sound resource type metadata",
            "sound-metadata-json-to-data-asset",
        ),
        (
            "straggler.error-log",
            "metadata",
            "error-log-json",
            "junk-artifact",
            "build or export error log",
            "junk-to-ignore",
        ),
    ];
    for (needle, type_, subtype, kind, function, future) in cases {
        if text.contains(needle) {
            let relation = if future == "junk-to-ignore" {
                "do-not-import"
            } else if kind == "editor-only-metadata" {
                "editor-only-metadata"
            } else {
                "import-as-data-asset"
            };
            apply_classification(
                meta,
                MetadataClassification::new(
                    type_,
                    subtype,
                    kind,
                    function,
                    "game-straggler-normalize",
                    relation,
                    future,
                ),
            );
            meta.schema = format!("shar-schoenwald.{needle}.v1");
            return true;
        }
    }
    false
}
