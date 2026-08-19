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
//   - Source-backed parsing, copying, hashing, and package publication.
// - Must-Not:
//   - Invent missing localization or modify the lawful source installation.
// - Allows:
//   - Read source evidence and publish deterministic bundles outside it.
// - Split-When:
//   - Parsing, copying, or publication gain independent lifecycles.
// - Merge-When:
//   - Another composition module owns the identical language export workflow.
// - Summary:
//   - Official-language export composition.
// - Description:
//   - Owns source-backed localization export outside the pure domain model.
// - Usage:
//   - Called through the public export_language facade.
// - Defaults:
//   - Missing translated source evidence fails closed before publication.
//

//! Source-backed official-language export composition.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;
use serde::Serialize;
use shar_mod_package::{
    CONTRACT_VERSION as MOD_CONTRACT_VERSION, Member, PackageKind,
    PackageManifest, Provenance, TrustLevel, content_revision,
    member_from_bytes,
};

use crate::domain::{
    CinematicAudioEvidence, Language, LanguageManifest, LanguageSpec,
    SourceEvidence,
};

const TEXT_TABLE: &str = "art/frontend/scrooby2/resource/txtbible/srr2.txt";
const UI_ROOT: &str = "art/frontend/dynaload/images";
const SCHEMA: &str = "shar.language-mod-source.v3";
const LANGUAGE_MOD_PRIORITY: i32 = 100;

/// Deterministic language composition failure.
#[derive(Debug)]
pub enum ExportError {
    /// Input/output contract failed.
    Contract(String),
    /// Local filesystem operation failed.
    Io(io::Error),
    /// JSON serialization failed.
    Json(serde_json::Error),
    /// Normalized SHAR mod-package validation failed.
    Package(shar_mod_package::PackageError),
}

impl std::fmt::Display for ExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Contract(message) => formatter.write_str(message),
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
            Self::Package(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for ExportError {}

impl From<io::Error> for ExportError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for ExportError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

impl From<shar_mod_package::PackageError> for ExportError {
    fn from(error: shar_mod_package::PackageError) -> Self {
        Self::Package(error)
    }
}

#[derive(Debug, Serialize)]
struct SourceEvidenceDocument<'a> {
    path: &'a str,
    size: u64,
    sha256: &'a str,
}

impl<'a> From<&'a SourceEvidence> for SourceEvidenceDocument<'a> {
    fn from(value: &'a SourceEvidence) -> Self {
        Self {
            path: &value.path,
            size: value.size,
            sha256: &value.sha256,
        }
    }
}

#[derive(Debug, Serialize)]
struct CinematicAudioDocument<'a> {
    movie: &'a str,
    track: &'a str,
    size: u64,
    sha256: &'a str,
}

impl<'a> From<&'a CinematicAudioEvidence> for CinematicAudioDocument<'a> {
    fn from(value: &'a CinematicAudioEvidence) -> Self {
        Self {
            movie: &value.movie,
            track: &value.track,
            size: value.size,
            sha256: &value.sha256,
        }
    }
}

#[derive(Debug, Serialize)]
struct LanguageManifestDocument<'a> {
    schema: &'a str,
    base_language: &'a str,
    language: &'a str,
    language_code: &'a str,
    records: usize,
    untranslated_placeholders: usize,
    included_sources: Vec<SourceEvidenceDocument<'a>>,
    cinematic_audio: Vec<CinematicAudioDocument<'a>>,
    missing_optional_sources: &'a [String],
    package_id: &'a str,
    status: &'a str,
}

impl<'a> From<&'a LanguageManifest> for LanguageManifestDocument<'a> {
    fn from(value: &'a LanguageManifest) -> Self {
        Self {
            schema: value.schema,
            base_language: value.base_language,
            language: value.language,
            language_code: value.language_code,
            records: value.records,
            untranslated_placeholders: value.untranslated_placeholders,
            included_sources: value
                .included_sources
                .iter()
                .map(Into::into)
                .collect(),
            cinematic_audio: value
                .cinematic_audio
                .iter()
                .map(Into::into)
                .collect(),
            missing_optional_sources: &value.missing_optional_sources,
            package_id: &value.package_id,
            status: value.status,
        }
    }
}

fn decode_windows_1252(bytes: &[u8]) -> String {
    bytes.iter().copied().map(decode_cp1252).collect()
}

fn decode_cp1252(byte: u8) -> char {
    if !(0x80..0xa0).contains(&byte) {
        return char::from(byte);
    }
    let code = match byte {
        0x80 => 0x20ac,
        0x82 => 0x201a,
        0x83 => 0x0192,
        0x84 => 0x201e,
        0x85 => 0x2026,
        0x86 => 0x2020,
        0x87 => 0x2021,
        0x88 => 0x02c6,
        0x89 => 0x2030,
        0x8a => 0x0160,
        0x8b => 0x2039,
        0x8c => 0x0152,
        0x8e => 0x017d,
        0x91 => 0x2018,
        0x92 => 0x2019,
        0x93 => 0x201c,
        0x94 => 0x201d,
        0x95 => 0x2022,
        0x96 => 0x2013,
        0x97 => 0x2014,
        0x98 => 0x02dc,
        0x99 => 0x2122,
        0x9a => 0x0161,
        0x9b => 0x203a,
        0x9c => 0x0153,
        0x9e => 0x017e,
        0x9f => 0x0178,
        _ => 0xfffd,
    };
    char::from_u32(code).unwrap_or('�')
}

#[derive(Debug, Clone, Serialize)]
struct TextRecord<'a> {
    screen: &'a str,
    key: &'a str,
    english: &'a str,
    value: &'a str,
    notes: &'a str,
}

fn parse_text_table(
    bytes: &[u8],
    spec: LanguageSpec,
) -> Result<(String, usize, usize), ExportError> {
    let decoded = decode_windows_1252(bytes);
    let lines = decoded.lines().collect::<Vec<_>>();
    if lines.len() < 5 {
        return Err(ExportError::Contract(
            "TextBible table is truncated".to_owned(),
        ));
    }
    let declaration = lines
        .first()
        .ok_or_else(|| {
            ExportError::Contract("TextBible declaration is missing".to_owned())
        })?
        .split('\t')
        .collect::<Vec<_>>();
    if declaration.first() != Some(&"Languages")
        || declaration.get(1) != Some(&"EFGIS")
    {
        return Err(ExportError::Contract(
            "TextBible language declaration is not EFGIS".to_owned(),
        ));
    }
    let columns = lines
        .get(2)
        .ok_or_else(|| {
            ExportError::Contract("TextBible code row is missing".to_owned())
        })?
        .split('\t')
        .collect::<Vec<_>>();
    let names = lines
        .get(3)
        .ok_or_else(|| {
            ExportError::Contract("TextBible name row is missing".to_owned())
        })?
        .split('\t')
        .collect::<Vec<_>>();
    let expected_codes = ["E", "F", "G", "I", "S"];
    let expected_names = [
        "ENGLISH", "FRENCH", "GERMAN", "ITALIAN", "SPANISH",
    ];
    if columns.get(3..8) != Some(&expected_codes)
        || names.get(3..8) != Some(&expected_names)
    {
        return Err(ExportError::Contract(
            "TextBible language columns are not canonical E/F/G/I/S".to_owned(),
        ));
    }

    let mut jsonl = String::new();
    let mut records = 0usize;
    let mut placeholders = 0usize;
    let mut translated = 0usize;
    for (offset, line) in lines.iter().skip(5).enumerate() {
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 9 {
            return Err(ExportError::Contract(format!(
                "TextBible row {} does not have 9 fields",
                offset.saturating_add(6)
            )));
        }
        let value = fields.get(spec.column).copied().ok_or_else(|| {
            let message = "TextBible language value is missing";
            ExportError::Contract(message.to_owned())
        })?;
        let screen = fields
            .first()
            .copied()
            .ok_or_else(|| {
                ExportError::Contract(
                    "TextBible screen value is missing".to_owned(),
                )
            })?;
        let key = fields
            .get(1)
            .copied()
            .ok_or_else(|| {
                let message = "TextBible key value is missing";
                ExportError::Contract(message.to_owned())
            })?;
        let english = fields.get(3).copied().ok_or_else(|| {
            let message = "TextBible English value is missing";
            ExportError::Contract(message.to_owned())
        })?;
        let notes = fields
            .get(8)
            .copied()
            .ok_or_else(|| {
                ExportError::Contract(
                    "TextBible notes value is missing".to_owned(),
                )
            })?;
        if value == "???" {
            placeholders = placeholders.saturating_add(1);
        } else {
            translated = translated.saturating_add(1);
        }
        let record = TextRecord {
            screen,
            key,
            english,
            value,
            notes,
        };
        jsonl.push_str(&serde_json::to_string(&record)?);
        jsonl.push('\n');
        records = records.saturating_add(1);
    }
    if records > 0 && translated == 0 {
        return Err(ExportError::Contract(format!(
            "{} column contains no translated text in this source",
            spec.name
        )));
    }
    Ok((jsonl, records, placeholders))
}

fn evidence(path: &str, bytes: &[u8]) -> Result<SourceEvidence, ExportError> {
    let size = u64::try_from(bytes.len()).map_err(|_error| {
        ExportError::Contract("source size does not fit u64".to_owned())
    })?;
    Ok(SourceEvidence {
        path: path.to_owned(),
        size,
        sha256: shar_sha256::digest_hex(bytes),
    })
}

fn copy_source(
    game_root: &Path,
    staging: &Path,
    relative: &Path,
) -> Result<SourceEvidence, ExportError> {
    let source = game_root.join(relative);
    if local::path_kind(&source)? != PathKind::File {
        return Err(ExportError::Contract(format!(
            "required localization source is missing: {}",
            relative.to_string_lossy()
        )));
    }
    let bytes = local::read_bytes(&source)?;
    let destination = schoenwald_filesystem::resolve_under(
        &staging.join("source"),
        relative,
    )
    .map_err(|error| ExportError::Contract(error.to_string()))?;
    local::write_bytes(&destination, &bytes, true)?;
    evidence(&relative.to_string_lossy().replace('\\', "/"), &bytes)
}

fn copy_optional_source(
    game_root: &Path,
    staging: &Path,
    relative: &Path,
    included: &mut Vec<SourceEvidence>,
    missing: &mut Vec<String>,
) -> Result<(), ExportError> {
    let source = game_root.join(relative);
    match local::path_kind(&source)? {
        PathKind::File => {
            included.push(copy_source(game_root, staging, relative)?);
        }
        PathKind::Missing => {
            missing.push(relative.to_string_lossy().replace('\\', "/"));
        }
        PathKind::Directory | PathKind::Other => {
            return Err(ExportError::Contract(format!(
                "localization source is not a regular file: {}",
                relative.to_string_lossy()
            )));
        }
    }
    Ok(())
}

fn copy_localized_ui(
    game_root: &Path,
    staging: &Path,
    spec: LanguageSpec,
    included: &mut Vec<SourceEvidence>,
) -> Result<(), ExportError> {
    let mut copied = 0usize;
    for family in ["loading", "license"] {
        let relative_root = PathBuf::from(UI_ROOT)
            .join(family)
            .join(spec.ui_directory);
        let source_root = game_root.join(&relative_root);
        if local::path_kind(&source_root)? == PathKind::Missing {
            continue;
        }
        if local::path_kind(&source_root)? != PathKind::Directory {
            return Err(ExportError::Contract(format!(
                "localized UI root is not a directory: {}",
                relative_root.to_string_lossy()
            )));
        }
        for source in local::strict_regular_files(&source_root)? {
            let tail = source.strip_prefix(game_root).map_err(|_error| {
                let message = "localized UI file escaped game root";
                ExportError::Contract(message.to_owned())
            })?;
            included.push(copy_source(game_root, staging, tail)?);
            copied = copied.saturating_add(1);
        }
    }
    if spec.movie_audio_track.is_some() && copied == 0 {
        return Err(ExportError::Contract(format!(
            "{} localization has no localized loading/license UI assets",
            spec.name
        )));
    }
    Ok(())
}

fn copy_cinematic_audio(
    movies_root: &Path,
    staging: &Path,
    spec: LanguageSpec,
) -> Result<Vec<CinematicAudioEvidence>, ExportError> {
    let Some(track_number) = spec.movie_audio_track else {
        return Ok(Vec::new());
    };
    if local::path_kind(movies_root)? != PathKind::Directory {
        return Err(ExportError::Contract(
            concat!(
                "normalized movie root is required for localized ",
                "cinematic audio",
            )
            .to_owned(),
        ));
    }
    let track_name = format!("audio_track_{track_number:02}.wav");
    let mut cinematic_audio = Vec::new();
    for source in local::strict_regular_files(movies_root)? {
        if source.file_name().and_then(|value| value.to_str())
            != Some(track_name.as_str())
        {
            continue;
        }
        let movie = source
            .parent()
            .and_then(Path::file_name)
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                let message = "movie package has no portable name";
                ExportError::Contract(message.to_owned())
            })?;
        let bytes = local::read_bytes(&source)?;
        let destination = staging
            .join("cinematics")
            .join(movie)
            .join(&track_name);
        local::write_bytes(&destination, &bytes, true)?;
        let size = u64::try_from(bytes.len()).map_err(|_error| {
            let message = "cinematic audio size does not fit u64";
            ExportError::Contract(message.to_owned())
        })?;
        cinematic_audio.push(CinematicAudioEvidence {
            movie: movie.to_owned(),
            track: track_name.clone(),
            size,
            sha256: shar_sha256::digest_hex(&bytes),
        });
    }
    cinematic_audio.sort_by(|left, right| left.movie.cmp(&right.movie));
    if cinematic_audio.is_empty() {
        return Err(ExportError::Contract(format!(
            concat!(
                "{} localization has no normalized cinematic audio ",
                "track {}",
            ),
            spec.name,
            track_name
        )));
    }
    Ok(cinematic_audio)
}

fn remove_staging(path: &Path) {
    if let Ok(metadata) = fs::symlink_metadata(path)
        && metadata.is_dir()
        && !metadata.file_type().is_symlink()
    {
        let _cleanup = fs::remove_dir_all(path);
    }
}

fn nearest_existing_directory(path: &Path) -> Result<PathBuf, ExportError> {
    for candidate in path.ancestors() {
        if candidate.as_os_str().is_empty() {
            continue;
        }
        match local::path_kind(candidate)? {
            PathKind::Directory => return Ok(candidate.to_path_buf()),
            PathKind::Missing => {}
            PathKind::File | PathKind::Other => {
                return Err(ExportError::Contract(
                    concat!(
                        "output parent chain contains a non-directory ",
                        "path",
                    )
                    .to_owned(),
                ));
            }
        }
    }
    Err(ExportError::Contract(
        "output has no existing directory ancestor".to_owned(),
    ))
}

fn reject_output_inside_input(
    output: &Path,
    input_root: &Path,
    label: &str,
) -> Result<(), ExportError> {
    let input = local::canonicalize(input_root)?;
    let parent = output
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let existing_parent = nearest_existing_directory(parent)?;
    let parent_identity = local::canonicalize(&existing_parent)?;
    if parent_identity == input || parent_identity.starts_with(&input) {
        return Err(ExportError::Contract(format!(
            "output must be outside the {label} directory"
        )));
    }
    Ok(())
}

fn language_package_id(spec: LanguageSpec) -> String {
    format!("shar.localization.{}", spec.name)
}

fn language_conflicts(spec: LanguageSpec) -> Vec<String> {
    let mut conflicts = ["french", "german", "italian", "spanish"]
        .into_iter()
        .filter(|language| *language != spec.name)
        .map(|language| format!("shar.localization.{language}"))
        .collect::<Vec<_>>();
    conflicts.sort();
    conflicts
}

fn package_member_metadata(path: &str) -> (&'static str, &'static str) {
    if path == "text.jsonl" {
        ("application/jsonl", "localization/text")
    } else if path == "manifest.json" {
        ("application/json", "localization/evidence")
    } else if path.starts_with("cinematics/") {
        ("audio/wav", "localization/cinematic-audio")
    } else if Path::new(path)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("rtf"))
    {
        ("application/rtf", "localization/readme")
    } else if Path::new(path)
        .extension()
        .is_some_and(|extension| extension.eq_ignore_ascii_case("rcf"))
    {
        ("application/octet-stream", "localization/dialogue")
    } else if path.contains("/dynaload/images/") {
        ("application/octet-stream", "localization/ui")
    } else {
        ("application/octet-stream", "localization/source")
    }
}

fn package_members(staging: &Path) -> Result<Vec<Member>, ExportError> {
    let mut members = Vec::new();
    for path in local::strict_regular_files(staging)? {
        let relative = path.strip_prefix(staging).map_err(|_error| {
            let message = "language package member escaped staging";
            ExportError::Contract(message.to_owned())
        })?;
        let portable = relative.to_string_lossy().replace('\\', "/");
        if portable == "mod.json" {
            continue;
        }
        let bytes = local::read_bytes(&path)?;
        let (media_type, role) = package_member_metadata(&portable);
        members.push(member_from_bytes(&portable, media_type, role, &bytes)?);
    }
    members.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(members)
}

fn package_manifest(
    staging: &Path,
    spec: LanguageSpec,
) -> Result<PackageManifest, ExportError> {
    let members = package_members(staging)?;
    let package_revision = content_revision(&members)?;
    let manifest = PackageManifest {
        contract_version: MOD_CONTRACT_VERSION.to_owned(),
        canonical_id: language_package_id(spec),
        package_revision,
        package_kind: PackageKind::Content,
        priority: LANGUAGE_MOD_PRIORITY,
        dependencies: Vec::new(),
        conflicts: language_conflicts(spec),
        supersedes: Vec::new(),
        required_capabilities: vec!["localization.overlay.v1".to_owned()],
        supported_targets: Vec::new(),
        members,
        provenance: Provenance {
            authors: vec!["original-rightsholders".to_owned()],
            source: String::from(
                "generated-from-user-supplied-lawful-original-game",
            ),
            license: "NOASSERTION".to_owned(),
        },
        trust_level: TrustLevel::ContentOnly,
    };
    manifest.validate()?;
    Ok(manifest)
}

/// Publishes one deterministic canonical language-mod source bundle.
///
/// `movies_root` is the normalized movie package root produced by the faithful
/// movie extraction stage. French, German, and Spanish select tracks 02, 03,
/// and 04 respectively because the original runtime indexes localized FMV audio
/// after English track 0.
///
/// # Errors
///
/// Returns a deterministic failure when required localized text, dialogue,
/// interface art, cinematic audio, filesystem safety, or publication fails.
pub fn export_language(
    game_root: &Path,
    movies_root: &Path,
    output: &Path,
    language: Language,
) -> Result<LanguageManifest, ExportError> {
    if local::path_kind(output)? != PathKind::Missing {
        return Err(ExportError::Contract("output already exists".to_owned()));
    }
    if local::path_kind(game_root)? != PathKind::Directory {
        return Err(ExportError::Contract(
            "source game root must be a real directory".to_owned(),
        ));
    }
    reject_output_inside_input(output, game_root, "source game")?;
    if local::path_kind(movies_root)? == PathKind::Directory {
        reject_output_inside_input(output, movies_root, "normalized movie")?;
    }

    let spec = language.spec();
    let table_relative = Path::new(TEXT_TABLE);
    let table_bytes = local::read_bytes(&game_root.join(table_relative))?;
    let (text_jsonl, records, untranslated_placeholders) =
        parse_text_table(&table_bytes, spec)?;

    let output_name = output
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            ExportError::Contract("output has no portable name".to_owned())
        })?;
    let staging = output.with_file_name(format!(
        ".{output_name}.language-{}.tmp",
        std::process::id()
    ));
    if local::path_kind(&staging)? != PathKind::Missing {
        return Err(ExportError::Contract(
            "language staging directory already exists".to_owned(),
        ));
    }
    local::create_dir_all(&staging)?;

    let publication = (|| -> Result<LanguageManifest, ExportError> {
        let mut included_sources = Vec::new();
        included_sources.push(copy_source(
            game_root,
            &staging,
            table_relative,
        )?);
        let sidecar = PathBuf::from(TEXT_TABLE)
            .with_file_name(format!("srr2.{}", spec.code));
        included_sources.push(copy_source(game_root, &staging, &sidecar)?);
        included_sources.push(copy_source(
            game_root,
            &staging,
            Path::new(spec.dialogue),
        )?);

        let mut missing_optional_sources = Vec::new();
        if let Some(readme) = spec.readme {
            copy_optional_source(
                game_root,
                &staging,
                Path::new(readme),
                &mut included_sources,
                &mut missing_optional_sources,
            )?;
        }
        copy_localized_ui(game_root, &staging, spec, &mut included_sources)?;
        included_sources.sort_by(|left, right| left.path.cmp(&right.path));
        missing_optional_sources.sort();

        let cinematic_audio =
            copy_cinematic_audio(movies_root, &staging, spec)?;
        local::write_text(&staging.join("text.jsonl"), &text_jsonl, false)?;

        let manifest = LanguageManifest {
            schema: SCHEMA,
            base_language: "english",
            language: spec.name,
            language_code: spec.code,
            records,
            untranslated_placeholders,
            included_sources,
            cinematic_audio,
            missing_optional_sources,
            package_id: language_package_id(spec),
            status: "canonical-language-mod-v1",
        };
        let document = LanguageManifestDocument::from(&manifest);
        let mut json = serde_json::to_string_pretty(&document)?;
        json.push('\n');
        local::write_text(&staging.join("manifest.json"), &json, false)?;

        let package = package_manifest(&staging, spec)?;
        local::write_text(
            &staging.join("mod.json"),
            &package.to_pretty_json()?,
            false,
        )?;
        fs::rename(&staging, output)?;
        Ok(manifest)
    })();

    if publication.is_err() {
        remove_staging(&staging);
    }
    publication
}
