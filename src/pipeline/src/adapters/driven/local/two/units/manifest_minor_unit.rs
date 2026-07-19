// File:
//   - manifest_minor_unit.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/units/manifest_minor_unit.rs
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
//   - The manifest minor unit contract for pipeline phase two minor units.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute manifest minor unit.
// - Split-When:
//   - Split when manifest minor unit contains two independently testable
//   - contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Manifestminorunitconfig.
// - Description:
//   - Defines manifest minor unit data and behavior for pipeline phase two
//   - minor units.
// - Usage:
//   - Used by pipeline phase two minor units code that needs manifest minor
//   - unit.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Manifestminorunitconfig keeps tightly coupled validation,
//   - ordering, and deterministic transformation invariants together; split
//   - when a stable independently testable sub-boundary is identified.
//

//! Manifestminorunitconfig.
//!
//! This boundary keeps manifestminorunitconfig explicit and returns
//! deterministic results to pipeline callers.
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

use game_manifest::obfuscate_component;
use schoenwald_filesystem::adapters::driving::local::{
    read_utf8 as local_read_utf8, write_bytes as local_write_bytes,
};

use super::metadata_fill::read_string_field;
use super::taxonomy::{self, UNKNOWN};
use crate::adapters::driven::check_cancellation;
use crate::adapters::driven::local::progress::StageProgress;
use crate::adapters::driven::local::two::stragglers;
use crate::domain::{PipelineError, StageReport, escape_json as json_escape};

/// Result.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Manifestunits.
type ManifestUnits = BTreeMap<String, String>;

/// Explicit roots used to build the minor-unit manifest.
#[derive(Debug, Clone)]
pub(super) struct ManifestMinorUnitConfig {
    /// Game root.
    pub game_root: PathBuf,
    /// Extracted root.
    pub extracted_root: PathBuf,
}

/// Write manifest minor units.
///
/// # Errors
///
/// Returns an error when validation, filesystem access, or output writing
/// fails.
pub(in crate::adapters::driven::local) fn write_manifest_minor_units(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    let config = ManifestMinorUnitConfig {
        game_root: game_root.to_path_buf(),
        extracted_root: extracted_root.to_path_buf(),
    };
    if !config
        .game_root
        .is_dir()
    {
        return Err(
            PipelineError::new(
                format!(
                    "game root does not exist: {}",
                    config
                        .game_root
                        .display()
                ),
            ),
        );
    }
    if !config
        .extracted_root
        .is_dir()
    {
        return Err(
            PipelineError::new(
                format!(
                    "extracted root must exist before minor-unit manifest \
                     generation: {}",
                    config
                        .extracted_root
                        .display()
                ),
            ),
        );
    }

    let staged = stragglers::normalize_game_stragglers(
        &config.game_root,
        &config.extracted_root,
    )?;
    let mut units = ManifestUnits::new();
    collect_game_first_units(
        &config, &mut units,
    )?;
    collect_extracted_overlay_units(
        &config.extracted_root,
        &mut units,
    )?;

    let manifest = render_manifest_jsonl(
        &units,
        &config.extracted_root,
    )?;
    let manifest_path = taxonomy::manifest_path(&config.extracted_root);
    write_bytes(
        &manifest_path,
        manifest.as_bytes(),
    )?;

    Ok(
        StageReport {
            name: "minor-unit-manifest",
            files: units.len(),
            bytes: u64::try_from(manifest.len()).unwrap_or(u64::MAX),
            note: format!(
                "phase two manifest wrote {}/{} from game-first plus \
                 extracted overlay; staged {} loose files; container files \
                 are replaced by normalized structures and cinematics are \
                 represented as HAP media units",
                taxonomy::OUTPUT_DIR_NAME,
                taxonomy::MANIFEST_FILE_NAME,
                staged.files
            ),
        },
    )
}

/// Collects game-first units before applying extracted overlay evidence.
// One ordered traversal owns filtering, provenance, and identity insertion.
#[expect(
    clippy::too_many_lines,
    reason = "One traversal preserves filtering, provenance, identity, and \
              deterministic insertion before extracted overlays."
)]
fn collect_game_first_units(
    config: &ManifestMinorUnitConfig,
    units: &mut ManifestUnits,
) -> PipelineOutcome<()> {
    let game_files = collect_files(&config.game_root)?;
    let mut progress = StageProgress::begin(
        "minor-unit game scan",
        game_files.len(),
    );
    for file in game_files {
        let relative = relative_path(
            &config.game_root,
            &file,
        )?;
        check_cancellation()?;
        progress.advance(&relative.to_string_lossy());
        let extension = file_extension(&file);
        if should_skip_local_game_file(
            &relative, &extension,
        ) {
            continue;
        }

        match extension.as_str() {
            "rcf" => {
                let extracted_dir = config
                    .extracted_root
                    .join(relative.with_extension(""));
                collect_required_extracted_tree(
                    &config.extracted_root,
                    &extracted_dir,
                    units,
                    "RCF",
                )?;
            }
            "p3d" => {
                let extracted_dir = config
                    .extracted_root
                    .join(relative.with_extension(""));
                collect_required_extracted_tree(
                    &config.extracted_root,
                    &extracted_dir,
                    units,
                    "P3D",
                )?;
            }
            "rmv" => {
                collect_required_movie_package(
                    &config.extracted_root,
                    &file,
                    units,
                )?;
            }
            "lmlm" => {
                let extracted_dir = config
                    .extracted_root
                    .join("lmlm");
                collect_required_extracted_tree(
                    &config.extracted_root,
                    &extracted_dir,
                    units,
                    "LMLM",
                )?;
            }
            "rsd" => {
                let normalized = stragglers::normalized_wav_path(
                    &config.extracted_root,
                    &relative,
                );
                if normalized.is_file() {
                    add_extracted_file(
                        &config.extracted_root,
                        &normalized,
                        units,
                    )?;
                }
            }
            "rtf" => {
                let normalized = config
                    .extracted_root
                    .join(relative.with_extension("md"));
                if normalized.is_file() {
                    add_extracted_file(
                        &config.extracted_root,
                        &normalized,
                        units,
                    )?;
                } else {
                    add_game_file(
                        &config.game_root,
                        &file,
                        units,
                    )?;
                }
            }
            candidate_extension
                if stragglers::is_json_straggler_extension(
                    candidate_extension,
                ) =>
            {
                let normalized = stragglers::normalized_json_path(
                    &config.extracted_root,
                    &relative,
                );
                if normalized.is_file() {
                    add_extracted_file(
                        &config.extracted_root,
                        &normalized,
                        units,
                    )?;
                } else {
                    add_game_file(
                        &config.game_root,
                        &file,
                        units,
                    )?;
                }
            }
            _ => add_game_file(
                &config.game_root,
                &file,
                units,
            )?,
        }
    }
    progress.finish();
    Ok(())
}

/// Collect extracted overlay units.
fn collect_extracted_overlay_units(
    extracted_root: &Path,
    units: &mut ManifestUnits,
) -> PipelineOutcome<()> {
    let extracted_files = collect_files(extracted_root)?;
    let mut progress = StageProgress::begin(
        "minor-unit extracted scan",
        extracted_files.len(),
    );
    for file in extracted_files {
        let relative = relative_path(
            extracted_root,
            &file,
        )?;
        check_cancellation()?;
        progress.advance(&relative.to_string_lossy());
        if is_reserved_minor_unit_output(
            extracted_root,
            &file,
        ) {
            continue;
        }
        if is_pipeline_run_report(
            extracted_root,
            &file,
        ) {
            continue;
        }
        if is_forbidden_extracted_source_file(&file) {
            continue;
        }
        add_extracted_file(
            extracted_root,
            &file,
            units,
        )?;
    }
    progress.finish();
    Ok(())
}

/// Return whether one path is the pipeline run report at the extracted root.
///
/// `extract-game` writes `pipeline-report.jsonl` after its own manifest stage.
/// Including that telemetry would make manifest coverage depend on command
/// order and would inject a volatile report byte size into asset identity.
fn is_pipeline_run_report(
    extracted_root: &Path,
    path: &Path,
) -> bool {
    path.strip_prefix(extracted_root)
        .is_ok_and(|relative| relative == Path::new("pipeline-report.jsonl"))
}

/// Collect required extracted tree.
fn collect_required_extracted_tree(
    extracted_root: &Path,
    extracted_dir: &Path,
    units: &mut ManifestUnits,
    label: &str,
) -> PipelineOutcome<()> {
    if !extracted_dir.is_dir() {
        return Err(
            PipelineError::new(
                format!(
                    "expected {label} replacement directory to exist: {}",
                    extracted_dir.display()
                ),
            ),
        );
    }
    for file in collect_files(extracted_dir)? {
        if is_forbidden_extracted_source_file(&file) {
            continue;
        }
        add_extracted_file(
            extracted_root,
            &file,
            units,
        )?;
    }
    Ok(())
}

/// Is forbidden extracted source file.
fn is_forbidden_extracted_source_file(file: &Path) -> bool {
    file.file_name()
        .and_then(|value| value.to_str())
        .is_some_and(|value| value.eq_ignore_ascii_case("source.p3d"))
}

/// Collect required movie package.
fn collect_required_movie_package(
    extracted_root: &Path,
    movie_file: &Path,
    units: &mut ManifestUnits,
) -> PipelineOutcome<()> {
    let stem = movie_file
        .file_stem()
        .and_then(|value| value.to_str())
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "movie file has no UTF-8 stem: {}",
                        movie_file.display()
                    ),
                )
            },
        )?;
    let extracted_dir = extracted_root
        .join("movies")
        .join(stem);
    collect_required_extracted_tree(
        extracted_root,
        &extracted_dir,
        units,
        "RMV",
    )
}

/// Collect files.
fn collect_files(root: &Path) -> PipelineOutcome<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(current) = stack.pop() {
        let mut entries = fs::read_dir(&current)
            .map_err(io_error(&current))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to read {}: {error}",
                            current.display()
                        ),
                    )
                },
            )?;
        entries.sort_by_key(fs::DirEntry::path);
        for entry in entries {
            let path = entry.path();
            let ty = entry
                .file_type()
                .map_err(
                    |error| {
                        PipelineError::new(
                            format!(
                                "failed to stat {}: {error}",
                                path.display()
                            ),
                        )
                    },
                )?;
            if ty.is_dir() {
                stack.push(path);
            } else if ty.is_file() {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

/// Should skip local game file.
fn should_skip_local_game_file(
    relative: &Path,
    extension: &str,
) -> bool {
    relative == Path::new("manifest.jsonl")
        || extension == "png"
        || extension == "schoenwald-original"
}

/// Add game file.
fn add_game_file(
    game_root: &Path,
    file: &Path,
    units: &mut ManifestUnits,
) -> PipelineOutcome<()> {
    let path = prefixed_relative_path(
        "game", game_root, file,
    )?;
    add_unit(
        units,
        path,
        file_extension(file),
    );
    Ok(())
}

/// Add extracted file.
fn add_extracted_file(
    extracted_root: &Path,
    file: &Path,
    units: &mut ManifestUnits,
) -> PipelineOutcome<()> {
    let path = prefixed_relative_path(
        "extracted",
        extracted_root,
        file,
    )?;
    add_unit(
        units,
        path,
        file_extension(file),
    );
    Ok(())
}

/// Add unit.
fn add_unit(
    units: &mut ManifestUnits,
    path: String,
    extension: String,
) {
    let _ = units
        .entry(path)
        .or_insert(extension);
}

/// Is reserved minor unit output.
fn is_reserved_minor_unit_output(
    extracted_root: &Path,
    path: &Path,
) -> bool {
    let Ok(relative) = path.strip_prefix(extracted_root) else {
        return false;
    };
    matches!(
        relative.components().next(),
        Some(Component::Normal(value)) if value == taxonomy::OUTPUT_DIR_NAME
    )
}

/// One enriched minor-unit row: the exact local path plus the name-free
/// coordinates and real extraction provenance that let the manifest act as a
/// durable, diffable ledger without ever publishing the game's names.
struct EnrichedUnit {
    /// Portable exact path, retained only for local file resolution.
    path: String,
    /// Lowercase file extension.
    extension: String,
    /// Name-free route: each folder reduced to its first and last character
    /// and the file replaced by a per-folder count token.
    obfuscated_route: String,
    /// Grouping key that keeps every component of one source object adjacent.
    source_object: String,
    /// Source chunk kind, or the not-applicable sentinel for loose files.
    source_chunk_kind: String,
    /// Source chunk ordinal, or the not-applicable sentinel.
    source_chunk_ordinal: String,
    /// Recovery outcome drawn from the lossless extraction vocabulary.
    recovery_status: String,
}

/// Render manifest jsonl.
///
/// # Errors
///
/// Returns an error when a package provenance ledger cannot be read.
pub(super) fn render_manifest_jsonl(
    units: &ManifestUnits,
    extracted_root: &Path,
) -> PipelineOutcome<String> {
    let enriched = enrich_units(
        units,
        extracted_root,
    )?;
    let mut progress = StageProgress::begin(
        "minor-unit render",
        enriched.len(),
    );
    let mut output = String::new();
    for unit in &enriched {
        check_cancellation()?;
        progress.advance(&unit.obfuscated_route);
        output.push_str(&render_manifest_row(unit));
        output.push('\n');
    }
    progress.finish();
    Ok(output)
}

/// Enrich collected units with real provenance, obfuscated routes, and a
/// deterministic order that keeps one source object's components adjacent.
fn enrich_units(
    units: &ManifestUnits,
    extracted_root: &Path,
) -> PipelineOutcome<Vec<EnrichedUnit>> {
    let mut cache = ProvenanceCache::new();
    let mut folder_index = BTreeMap::<
        (
            String,
            String,
        ),
        usize,
    >::new();
    let mut enriched = Vec::with_capacity(units.len());
    let mut progress = StageProgress::begin(
        "minor-unit provenance",
        units.len(),
    );
    // `units` is a BTreeMap, so iteration is sorted by path and the per-folder
    // count token is assigned deterministically regardless of enumeration
    // order.
    for (path, extension) in units {
        check_cancellation()?;
        progress.advance(path);
        let route = obfuscated_route(
            path,
            extension,
            &mut folder_index,
        );
        let provenance = cache.lookup(
            extracted_root,
            path,
        )?;
        enriched.push(
            EnrichedUnit {
                source_object: source_object_key(path).to_owned(),
                source_chunk_kind: provenance.chunk_kind,
                source_chunk_ordinal: provenance.chunk_ordinal,
                recovery_status: provenance.recovery_status,
                obfuscated_route: route,
                path: path.clone(),
                extension: extension.clone(),
            },
        );
    }
    progress.finish();
    // Group by source object, then order by source chunk kind and path so the
    // ledger is stable and a package's components stay together across kinds.
    enriched.sort_by(
        |left, right| {
            left.source_object
                .cmp(&right.source_object)
                .then_with(
                    || {
                        left.source_chunk_kind
                            .cmp(&right.source_chunk_kind)
                    },
                )
                .then_with(
                    || {
                        left.path
                            .cmp(&right.path)
                    },
                )
        },
    );
    Ok(enriched)
}

/// Render one manifest row with real provenance columns and `error`
/// placeholders for the classification metadata fill resolves later.
fn render_manifest_row(unit: &EnrichedUnit) -> String {
    render_row(
        &[
            (
                "path",
                unit.path
                    .as_str(),
            ),
            (
                "id", UNKNOWN,
            ),
            (
                "obfuscated_route",
                unit.obfuscated_route
                    .as_str(),
            ),
            (
                "file_extension",
                unit.extension
                    .as_str(),
            ),
            (
                "type", UNKNOWN,
            ),
            (
                "subtype", UNKNOWN,
            ),
            (
                "kind", UNKNOWN,
            ),
            (
                "function", UNKNOWN,
            ),
            (
                "schema", UNKNOWN,
            ),
            (
                "origin", UNKNOWN,
            ),
            (
                "source_path",
                UNKNOWN,
            ),
            (
                "source_extension",
                UNKNOWN,
            ),
            (
                "source_container",
                UNKNOWN,
            ),
            (
                "source_chunk_kind",
                unit.source_chunk_kind
                    .as_str(),
            ),
            (
                "source_chunk_ordinal",
                unit.source_chunk_ordinal
                    .as_str(),
            ),
            (
                "recovery_status",
                unit.recovery_status
                    .as_str(),
            ),
            (
                "derived_from",
                UNKNOWN,
            ),
            (
                "size_bytes",
                UNKNOWN,
            ),
            (
                "unreal_import_relation",
                UNKNOWN,
            ),
            (
                "future_normalization",
                UNKNOWN,
            ),
            (
                "component_links",
                UNKNOWN,
            ),
            (
                "classification_notes",
                UNKNOWN,
            ),
        ],
    )
}

/// Render an ordered field list as one canonical JSON object line. Shared by
/// the manifest and metadata-fill stages so the column order stays a single
/// source of truth.
pub(super) fn render_row(
    fields: &[(
        &str,
        &str,
    )]
) -> String {
    let mut row = String::from("{");
    for (index, (field, value)) in fields
        .iter()
        .enumerate()
    {
        if index > 0 {
            row.push(',');
        }
        row.push('"');
        row.push_str(field);
        row.push_str("\":\"");
        row.push_str(&json_escape(value));
        row.push('"');
    }
    row.push('}');
    row
}

/// Key that groups every component of one source object. Package components
/// live under `<package>/components/...`; the substring before that marker is
/// the object. Loose files are their own object.
fn source_object_key(path: &str) -> &str {
    path.split_once("/components/")
        .map_or(
            path,
            |(head, _tail)| head,
        )
}

/// Build the name-free route: obfuscate each folder segment to its first and
/// last character (the game-manifest rule) and replace the file name with a
/// count token. The count is assigned per obfuscated folder shape, not per
/// exact folder, so distinct real folders that collapse to the same shape
/// receive distinct indices; that keeps the route a globally unique, name-free
/// coordinate and therefore a unique identity seed.
fn obfuscated_route(
    path: &str,
    extension: &str,
    folder_index: &mut BTreeMap<
        (
            String,
            String,
        ),
        usize,
    >,
) -> String {
    let (parent, _leaf) = path
        .rsplit_once('/')
        .unwrap_or(
            (
                "", path,
            ),
        );
    let obfuscated_parent = parent
        .split('/')
        .filter(|segment| !segment.is_empty())
        .map(obfuscate_component)
        .collect::<Vec<_>>()
        .join("/");
    let index = folder_index
        .entry(
            (
                obfuscated_parent.clone(),
                extension.to_owned(),
            ),
        )
        .or_insert(0);
    *index = index.saturating_add(1);
    let count_token = format!("#{index}.{extension}");
    if obfuscated_parent.is_empty() {
        count_token
    } else {
        format!("{obfuscated_parent}/{count_token}")
    }
}

#[derive(Clone)]
/// Resolved provenance for one unit.
struct UnitProvenance {
    /// Source chunk kind, or the not-applicable sentinel.
    chunk_kind: String,
    /// Source chunk ordinal, or the not-applicable sentinel.
    chunk_ordinal: String,
    /// Recovery outcome taxonomy value.
    recovery_status: String,
}

impl UnitProvenance {
    /// Provenance for a file that is materialized on disk in full rather than
    /// decoded from a source chunk.
    fn fully_decoded() -> Self {
        Self {
            chunk_kind: taxonomy::NOT_APPLICABLE.to_owned(),
            chunk_ordinal: taxonomy::NOT_APPLICABLE.to_owned(),
            recovery_status: taxonomy::FULLY_DECODED.to_owned(),
        }
    }
}

/// One package ledger: component-relative path to its recorded provenance.
type PackageLedger = BTreeMap<String, UnitProvenance>;

/// Caches parsed `components.jsonl` ledgers so each package is read at most
/// once.
struct ProvenanceCache {
    /// Keyed by package portable path; `None` marks a directory with no
    /// ledger.
    packages: BTreeMap<String, Option<PackageLedger>>,
}

impl ProvenanceCache {
    /// New.
    const fn new() -> Self {
        Self {
            packages: BTreeMap::new(),
        }
    }

    /// Resolve provenance for a unit path, reading and caching the owning
    /// package ledger. Units outside a `components/` tree, or whose component
    /// is absent from the ledger, report fully decoded provenance so the
    /// audit sees a truthful value rather than an error backlog
    /// sentinel.
    fn lookup(
        &mut self,
        extracted_root: &Path,
        path: &str,
    ) -> PipelineOutcome<UnitProvenance> {
        let Some((package, component_rel)) = path.split_once("/components/")
        else {
            return Ok(UnitProvenance::fully_decoded());
        };
        if !self
            .packages
            .contains_key(package)
        {
            let ledger = read_package_ledger(
                extracted_root,
                package,
            )?;
            let _ = self
                .packages
                .insert(
                    package.to_owned(),
                    ledger,
                );
        }
        let resolved = self
            .packages
            .get(package)
            .and_then(Option::as_ref)
            .and_then(|ledger| ledger.get(component_rel))
            .cloned()
            .unwrap_or_else(UnitProvenance::fully_decoded);
        Ok(resolved)
    }
}

/// Read and parse one package `components.jsonl` into a provenance ledger.
/// Returns `Ok(None)` when the directory has no ledger (an RCF tree or a loose
/// folder), so callers fall back to fully decoded provenance.
fn read_package_ledger(
    extracted_root: &Path,
    package_portable: &str,
) -> PipelineOutcome<Option<PackageLedger>> {
    let Some(relative) = package_portable.strip_prefix("extracted/") else {
        return Ok(None);
    };
    let ledger_path = extracted_root
        .join(relative)
        .join("components.jsonl");
    if !ledger_path.is_file() {
        return Ok(None);
    }
    let text = local_read_utf8(&ledger_path).map_err(io_error(&ledger_path))?;
    let mut ledger = PackageLedger::new();
    for line in text.lines() {
        if let Some((component_rel, provenance)) = parse_component_line(line) {
            let _ = ledger.insert(
                component_rel,
                provenance,
            );
        }
    }
    Ok(Some(ledger))
}

/// Parse one `components.jsonl` component line into a ledger entry. The header
/// line has no `path` field and is skipped.
fn parse_component_line(
    line: &str
) -> Option<(
    String,
    UnitProvenance,
)> {
    let component_rel = read_string_field(
        line, "path",
    )?;
    let chunk_kind = read_string_field(
        line, "kind",
    )
    .unwrap_or_else(|| taxonomy::NOT_APPLICABLE.to_owned());
    let chunk_ordinal = read_number_field(
        line, "ordinal",
    )
    .unwrap_or_else(|| taxonomy::NOT_APPLICABLE.to_owned());
    let raw_status = read_string_field(
        line,
        "recovery_status",
    )
    .unwrap_or_default();
    let mapped = taxonomy::map_recovery_status(&raw_status);
    // An unrecognized extractor status must not poison the audit as an
    // error backlog row; treat it as a fully decoded artifact instead.
    let recovery_status = if mapped == UNKNOWN {
        taxonomy::FULLY_DECODED
    } else {
        mapped
    };
    Some(
        (
            component_rel,
            UnitProvenance {
                chunk_kind,
                chunk_ordinal,
                recovery_status: recovery_status.to_owned(),
            },
        ),
    )
}

/// Read a bare (unquoted) non-negative integer field from a canonical JSONL
/// line, returned as its digit string.
fn read_number_field(
    line: &str,
    field: &str,
) -> Option<String> {
    let needle = format!("\"{field}\":");
    let start = line
        .find(&needle)?
        .checked_add(needle.len())?;
    let digits = line
        .get(start..)?
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>();
    if digits.is_empty() {
        None
    } else {
        Some(digits)
    }
}

/// Prefixed relative path.
pub(super) fn prefixed_relative_path(
    prefix: &str,
    root: &Path,
    path: &Path,
) -> PipelineOutcome<String> {
    let source_relative = relative_path(
        root, path,
    )?;
    let mut portable = String::from(prefix);
    let relative = source_relative
        .to_string_lossy()
        .replace(
            char::from(92),
            "/",
        );
    if !relative.is_empty() {
        portable.push('/');
        portable.push_str(&relative);
    }
    Ok(portable)
}

/// Relative path.
fn relative_path(
    root: &Path,
    path: &Path,
) -> PipelineOutcome<PathBuf> {
    path.strip_prefix(root)
        .map(Path::to_path_buf)
        .map_err(
            |_error| {
                PipelineError::new(
                    format!(
                        "failed to relativize {} against {}",
                        path.display(),
                        root.display()
                    ),
                )
            },
        )
}

/// File extension.
fn file_extension(path: &Path) -> String {
    path.extension()
        .and_then(|value| value.to_str())
        .map_or_else(
            || UNKNOWN.to_owned(),
            str::to_ascii_lowercase,
        )
}

/// Write bytes.
fn write_bytes(
    path: &Path,
    bytes: &[u8],
) -> PipelineOutcome<()> {
    local_write_bytes(
        path, bytes, true,
    )
    .map_err(io_error(path))
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
    use std::collections::BTreeMap;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    use super::{
        is_pipeline_run_report, obfuscated_route, parse_component_line,
        read_number_field, source_object_key, write_manifest_minor_units,
    };

    /// Distinguishes concurrent synthetic manifest cases within one process.
    static CASE_COUNTER: AtomicU64 = AtomicU64::new(0);

    /// Create one collision-resistant synthetic case root.
    fn case_root(label: &str) -> PathBuf {
        std::env::temp_dir().join(
            format!(
                "pipeline-manifest-{label}-{}-{}",
                std::process::id(),
                CASE_COUNTER.fetch_add(
                    1,
                    Ordering::Relaxed,
                ),
            ),
        )
    }

    /// Write one synthetic extracted file and its parent directories.
    fn write_sample(
        root: &Path,
        relative: &str,
        contents: &[u8],
    ) -> Result<(), String> {
        let path = root.join(relative);
        let parent = path
            .parent()
            .ok_or_else(
                || String::from("synthetic sample must have a parent"),
            )?;
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        fs::write(
            path, contents,
        )
        .map_err(|error| error.to_string())
    }

    #[test]
    fn pipeline_report_exclusion_matches_only_the_extracted_root() {
        let root = Path::new("extracted");
        assert!(
            is_pipeline_run_report(
                root,
                Path::new("extracted/pipeline-report.jsonl"),
            )
        );
        assert!(
            !is_pipeline_run_report(
                root,
                Path::new("extracted/art/pipeline-report.jsonl"),
            )
        );
        assert!(
            !is_pipeline_run_report(
                root,
                Path::new("extracted/pipeline-report.json"),
            )
        );
    }

    #[test]
    fn manifest_is_independent_of_run_report_presence() {
        assert_eq!(
            run_report_presence_case(),
            Ok(())
        );
    }

    /// Run the synthetic report-presence determinism case.
    fn run_report_presence_case() -> Result<(), String> {
        let case = case_root("report");
        let game_root = case.join("game");
        let extracted_root = case.join("extracted");
        fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
        write_sample(
            &extracted_root,
            "art/sample.json",
            b"{}",
        )?;

        let first_report = write_manifest_minor_units(
            &game_root,
            &extracted_root,
        )
        .map_err(|error| error.to_string())?;
        let manifest_path = extracted_root
            .join("minor-unit")
            .join("manifest.jsonl");
        let before = fs::read_to_string(&manifest_path)
            .map_err(|error| error.to_string())?;

        write_sample(
            &extracted_root,
            "pipeline-report.jsonl",
            br#"{"stage":"manifest","files":1}
"#,
        )?;
        let second_report = write_manifest_minor_units(
            &game_root,
            &extracted_root,
        )
        .map_err(|error| error.to_string())?;
        let after = fs::read_to_string(&manifest_path)
            .map_err(|error| error.to_string())?;
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;

        if before.contains("pipeline-report") {
            return Err(
                String::from("manifest covered pipeline run telemetry"),
            );
        }
        if before != after {
            return Err(String::from("manifest changed after report creation"));
        }
        if first_report.files != second_report.files {
            return Err(
                String::from(
                    "manifest unit count changed after report creation",
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn manifest_is_independent_of_file_creation_order() {
        assert_eq!(
            run_creation_order_case(),
            Ok(())
        );
    }

    /// Run the synthetic creation-order determinism case.
    fn run_creation_order_case() -> Result<(), String> {
        let case = case_root("order");
        let game_root = case.join("game");
        let first_root = case.join("first");
        let second_root = case.join("second");
        fs::create_dir_all(&game_root).map_err(|error| error.to_string())?;
        write_sample(
            &first_root,
            "art/z.json",
            br#"{"value":2}"#,
        )?;
        write_sample(
            &first_root,
            "art/a.json",
            br#"{"value":1}"#,
        )?;
        write_sample(
            &second_root,
            "art/a.json",
            br#"{"value":1}"#,
        )?;
        write_sample(
            &second_root,
            "art/z.json",
            br#"{"value":2}"#,
        )?;

        let _first_report = write_manifest_minor_units(
            &game_root,
            &first_root,
        )
        .map_err(|error| error.to_string())?;
        let _second_report = write_manifest_minor_units(
            &game_root,
            &second_root,
        )
        .map_err(|error| error.to_string())?;
        let first = fs::read_to_string(
            first_root
                .join("minor-unit")
                .join("manifest.jsonl"),
        )
        .map_err(|error| error.to_string())?;
        let second = fs::read_to_string(
            second_root
                .join("minor-unit")
                .join("manifest.jsonl"),
        )
        .map_err(|error| error.to_string())?;
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;

        if first != second {
            return Err(
                String::from("manifest changed with file creation order"),
            );
        }
        Ok(())
    }

    #[test]
    fn source_object_key_groups_package_components() {
        assert_eq!(
            source_object_key("packages/sample/components/mesh/a.json"),
            "packages/sample"
        );
        assert_eq!(
            source_object_key("packages/scripts/level.mfk"),
            "packages/scripts/level.mfk"
        );
    }

    #[test]
    fn obfuscated_route_hides_names_and_counts_per_folder() {
        let mut index = BTreeMap::new();
        let first = obfuscated_route(
            "packages/sample/components/texture/circle.png",
            "png",
            &mut index,
        );
        let second = obfuscated_route(
            "packages/sample/components/texture/nostril.png",
            "png",
            &mut index,
        );
        assert_eq!(
            first,
            "ps/se/cs/te/#1.png"
        );
        assert_eq!(
            second,
            "ps/se/cs/te/#2.png"
        );
        assert!(!first.contains("wiggum"));
        assert!(!first.contains("circle"));
    }

    #[test]
    fn obfuscated_route_is_unique_across_folders_sharing_a_shape() {
        let mut index = BTreeMap::new();
        // cspell:disable-next-line -- aro
        // "aro" and "ado" both obfuscate to "ao", so a per-exact-folder counter
        // would give both "#1"; the per-shape counter must keep them distinct
        // so the route stays a unique identity seed.
        let first = obfuscated_route(
            // cspell:disable-next-line -- aro
            "game/aro/x.json",
            "json",
            &mut index,
        );
        let second = obfuscated_route(
            "game/ado/y.json",
            "json",
            &mut index,
        );
        assert_ne!(
            first,
            second
        );
    }

    #[test]
    fn read_number_field_reads_bare_integers_only() {
        assert_eq!(
            read_number_field(
                "{\"ordinal\":42,\"x\":1}",
                "ordinal"
            ),
            Some("42".to_owned())
        );
        assert_eq!(
            read_number_field(
                "{\"a\":1}",
                "ordinal"
            ),
            None
        );
    }

    #[test]
    fn parse_component_line_maps_provenance() {
        let line = [
            "{\"ordinal\":7",
            "\"name\":\"x\"",
            "\"path\":\"texture/c.png\"",
            "\"kind\":\"texture\"",
            "\"payload_format\":\"image/png\"",
            "\"schema_ref\":\"texture\"",
            "\"recovery_status\":\"recovered_embedded_image_payload\"}",
        ]
        .join(",");
        let parsed = parse_component_line(&line);
        assert!(parsed.is_some());
        if let Some((component_rel, provenance)) = parsed {
            assert_eq!(
                component_rel,
                "texture/c.png"
            );
            assert_eq!(
                provenance.chunk_kind,
                "texture"
            );
            assert_eq!(
                provenance.chunk_ordinal,
                "7"
            );
            assert_eq!(
                provenance.recovery_status,
                "fully-decoded"
            );
        }
    }

    #[test]
    fn parse_component_line_skips_header() {
        assert!(
            parse_component_line(
                "{\"schema\":\"p3d.package.v1\",\"chunk_count\":3}"
            )
            .is_none()
        );
    }
}
