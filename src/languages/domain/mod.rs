// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT

//! Canonical non-English localization domain and filesystem composition.

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use schoenwald_filesystem::PathKind;
use schoenwald_filesystem::adapters::driving::local;
use serde::Serialize;

const TEXT_TABLE: &str = "art/frontend/scrooby2/resource/txtbible/srr2.txt";
const UI_ROOT: &str = "art/frontend/dynaload/images";
const SCHEMA: &str = "shar.language-mod-source.v2";

/// One official non-English language carried by the original game source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// French localization.
    French,
    /// German localization.
    German,
    /// Italian localization where a lawful source actually contains it.
    Italian,
    /// Spanish localization.
    Spanish,
}

#[derive(Debug, Clone, Copy)]
struct LanguageSpec {
    name: &'static str,
    code: &'static str,
    column: usize,
    dialogue: &'static str,
    readme: Option<&'static str>,
    ui_directory: &'static str,
    movie_audio_track: Option<usize>,
}

impl Language {
    const fn spec(self) -> LanguageSpec {
        match self {
            Self::French => LanguageSpec {
                name: "french",
                code: "F",
                column: 4,
                dialogue: "dialogf.rcf",
                readme: Some("Lisez-moi.rtf"),
                ui_directory: "french",
                movie_audio_track: Some(2),
            },
            Self::German => LanguageSpec {
                name: "german",
                code: "G",
                column: 5,
                dialogue: "dialogg.rcf",
                readme: Some("Liesmich.rtf"),
                ui_directory: "german",
                movie_audio_track: Some(3),
            },
            Self::Italian => LanguageSpec {
                name: "italian",
                code: "I",
                column: 6,
                dialogue: "dialogi.rcf",
                readme: None,
                ui_directory: "italian",
                movie_audio_track: None,
            },
            Self::Spanish => LanguageSpec {
                name: "spanish",
                code: "S",
                column: 7,
                dialogue: "dialogs.rcf",
                readme: Some("Léeme.rtf"),
                ui_directory: "spanish",
                movie_audio_track: Some(4),
            },
        }
    }
}

/// One preserved source artifact.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SourceEvidence {
    /// Portable source-relative path.
    pub path: String,
    /// Exact byte size.
    pub size: u64,
    /// Exact SHA-256 digest.
    pub sha256: String,
}

/// One normalized cinematic audio artifact selected for the language.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CinematicAudioEvidence {
    /// Movie package name.
    pub movie: String,
    /// Normalized track filename selected from that movie package.
    pub track: String,
    /// Exact byte size.
    pub size: u64,
    /// Exact SHA-256 digest.
    pub sha256: String,
}

/// Deterministic evidence for one canonical language-mod source bundle.
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LanguageManifest {
    /// Stable schema identifier.
    pub schema: &'static str,
    /// Canonical base language this mod overlays.
    pub base_language: &'static str,
    /// Selected non-English language.
    pub language: &'static str,
    /// Original `TextBible` language code.
    pub language_code: &'static str,
    /// Number of translated `TextBible` records.
    pub records: usize,
    /// Number of authored `???` placeholders retained verbatim.
    pub untranslated_placeholders: usize,
    /// Exact preserved original-language sources.
    pub included_sources: Vec<SourceEvidence>,
    /// Normalized localized movie-audio tracks included in the bundle.
    pub cinematic_audio: Vec<CinematicAudioEvidence>,
    /// Optional original sources not present in this lawful installation.
    pub missing_optional_sources: Vec<String>,
    /// Current final-package adaptation state.
    pub status: &'static str,
}

/// Deterministic language composition failure.
#[derive(Debug)]
pub enum ExportError {
    /// Input/output contract failed.
    Contract(String),
    /// Local filesystem operation failed.
    Io(io::Error),
    /// JSON serialization failed.
    Json(serde_json::Error),
}

impl fmt::Display for ExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contract(message) => formatter.write_str(message),
            Self::Io(error) => write!(formatter, "{error}"),
            Self::Json(error) => write!(formatter, "{error}"),
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
        .ok_or_else(|| ExportError::Contract("TextBible declaration is missing".to_owned()))?
        .split('\t')
        .collect::<Vec<_>>();
    if declaration.first() != Some(&"Languages") || declaration.get(1) != Some(&"EFGIS") {
        return Err(ExportError::Contract(
            "TextBible language declaration is not EFGIS".to_owned(),
        ));
    }
    let columns = lines
        .get(2)
        .ok_or_else(|| ExportError::Contract("TextBible code row is missing".to_owned()))?
        .split('\t')
        .collect::<Vec<_>>();
    let names = lines
        .get(3)
        .ok_or_else(|| ExportError::Contract("TextBible name row is missing".to_owned()))?
        .split('\t')
        .collect::<Vec<_>>();
    if columns.get(3..8) != Some(&["E", "F", "G", "I", "S"])
        || names.get(3..8) != Some(&["ENGLISH", "FRENCH", "GERMAN", "ITALIAN", "SPANISH"])
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
            ExportError::Contract("TextBible language value is missing".to_owned())
        })?;
        let screen = fields
            .first()
            .copied()
            .ok_or_else(|| ExportError::Contract("TextBible screen value is missing".to_owned()))?;
        let key = fields
            .get(1)
            .copied()
            .ok_or_else(|| ExportError::Contract("TextBible key value is missing".to_owned()))?;
        let english = fields.get(3).copied().ok_or_else(|| {
            ExportError::Contract("TextBible English value is missing".to_owned())
        })?;
        let notes = fields
            .get(8)
            .copied()
            .ok_or_else(|| ExportError::Contract("TextBible notes value is missing".to_owned()))?;
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
    let size = u64::try_from(bytes.len())
        .map_err(|_error| ExportError::Contract("source size does not fit u64".to_owned()))?;
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
    let destination = schoenwald_filesystem::resolve_under(&staging.join("source"), relative)
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
        PathKind::File => included.push(copy_source(game_root, staging, relative)?),
        PathKind::Missing => missing.push(relative.to_string_lossy().replace('\\', "/")),
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
        let relative_root = PathBuf::from(UI_ROOT).join(family).join(spec.ui_directory);
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
        for source in local::regular_files(&source_root)? {
            let tail = source.strip_prefix(game_root).map_err(|_error| {
                ExportError::Contract("localized UI file escaped game root".to_owned())
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
            "normalized movie root is required for localized cinematic audio".to_owned(),
        ));
    }
    let track_name = format!("audio_track_{track_number:02}.wav");
    let mut cinematic_audio = Vec::new();
    for source in local::regular_files(movies_root)? {
        if source.file_name().and_then(|value| value.to_str()) != Some(track_name.as_str()) {
            continue;
        }
        let movie = source
            .parent()
            .and_then(Path::file_name)
            .and_then(|value| value.to_str())
            .ok_or_else(|| {
                ExportError::Contract("movie package has no portable name".to_owned())
            })?;
        let bytes = local::read_bytes(&source)?;
        let destination = staging.join("cinematics").join(movie).join(&track_name);
        local::write_bytes(&destination, &bytes, true)?;
        let size = u64::try_from(bytes.len()).map_err(|_error| {
            ExportError::Contract("cinematic audio size does not fit u64".to_owned())
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
            "{} localization has no normalized cinematic audio track {track_name}",
            spec.name
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
    local::create_dir_all(parent)?;
    let parent_identity = local::canonicalize(parent)?;
    if parent_identity == input || parent_identity.starts_with(&input) {
        return Err(ExportError::Contract(format!(
            "output must be outside the {label} directory"
        )));
    }
    Ok(())
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
    let (text_jsonl, records, untranslated_placeholders) = parse_text_table(&table_bytes, spec)?;

    let output_name = output
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| ExportError::Contract("output has no portable name".to_owned()))?;
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
        included_sources.push(copy_source(game_root, &staging, table_relative)?);
        let sidecar = PathBuf::from(TEXT_TABLE).with_file_name(format!("srr2.{}", spec.code));
        included_sources.push(copy_source(game_root, &staging, &sidecar)?);
        included_sources.push(copy_source(game_root, &staging, Path::new(spec.dialogue))?);

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

        let cinematic_audio = copy_cinematic_audio(movies_root, &staging, spec)?;
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
            status: "canonical-language-mod-needs-final-package-adaptation",
        };
        let mut json = serde_json::to_string_pretty(&manifest)?;
        json.push('\n');
        local::write_text(&staging.join("manifest.json"), &json, false)?;
        fs::rename(&staging, output)?;
        Ok(manifest)
    })();

    if publication.is_err() {
        remove_staging(&staging);
    }
    publication
}

#[cfg(test)]
mod tests;
