// File:
//   - generate_expanded.rs
// Path:
//   - src/game-manifest/src/application/generate_expanded.rs
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
//   - Expanded file-level ledger generation and publication policy.
// - Must-Not:
//   - Traverse filesystems, print diagnostics, or select concrete adapters.
// - Allows:
//   - Validate roots, classify supplied files, and publish one ledger.
// - Split-When:
//   - Split when classification and ledger publication become independent.
// - Merge-When:
//   - Another use case owns the same expanded-manifest contract.
// - Summary:
//   - Application command for expanded manifest generation.
// - Description:
//   - Produces a deterministic ledger from game and extracted RCF evidence.
// - Usage:
//   - Invoked by driving adapters with tree and text-store ports.
// - Defaults:
//   - RCF archives require a separate extracted evidence root.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - true
//   - Reason: Classification, tags, metadata, and JSON rendering remain
//   - one cohesive deterministic expanded-ledger contract.
//

//! Application command for deterministic expanded manifest generation.
//!
//! The command validates evidence roots before publishing a complete ledger.
use std::path::{Component, Path, PathBuf};

use schoenwald_filesystem::resolve_under;

use super::ManifestError;
use super::path_evidence::deduplicate_paths;
use super::rcf_evidence::load_extracted_rcf_files;
use crate::domain::{
    BACKUP_EXTENSION, EXPANDED_MANIFEST_FILE_NAME, MANIFEST_FILE_NAME,
    classify_manifest_bucket, extension_of, kind_taxonomy_jsonl,
};
use crate::ports::{GameTree, PathKind, TextArtifactStore};

/// Canonical second line identifying expanded-manifest output.
pub const EXPANDED_SCHEMA_LINE: &str = concat!(
    "{\"schema\":\"shar-schoenwald.expanded-manifest.v1\",",
    "\"metadata\":\"file-level ledger; RCF archives are represented ",
    "by extracted contents; P3D records may contain mixed content and require ",
    "later chunk-level classification\"}"
);

/// Evidence returned after one expanded ledger publication.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenerateExpandedReport {
    /// Number of file-level records written.
    pub record_count: usize,
    /// Explicit output path.
    pub output_path: PathBuf,
}

/// Stateless expanded-manifest generation use case.
#[derive(Debug, Clone, Copy)]
pub struct GenerateExpandedManifest;

impl GenerateExpandedManifest {
    /// Generates and publishes one expanded file-level ledger.
    ///
    /// # Errors
    ///
    /// Returns a typed root, classification, read, or publication failure.
    pub fn execute(
        tree: &impl GameTree,
        store: &impl TextArtifactStore,
        game_dir: &Path,
        extracted_rcf_dir: &Path,
        output_path: &Path,
    ) -> Result<GenerateExpandedReport, ManifestError> {
        validate_paths(
            game_dir,
            extracted_rcf_dir,
            output_path,
        )?;
        let game_files = deduplicate_paths(
            required_files(
                tree, game_dir, "game",
            )?,
        );
        if game_files.is_empty() {
            return Err(
                ManifestError::Invalid(
                    "game directory contains no files".to_owned(),
                ),
            );
        }
        let game_has_rcf = game_files
            .iter()
            .any(|path| extension_of(path) == "rcf");
        let extracted_files = load_extracted_rcf_files(
            tree,
            extracted_rcf_dir,
            game_has_rcf,
        )?;
        let mut records = Vec::new();
        collect_game_files(
            game_dir,
            output_path,
            &game_files,
            &mut records,
        )?;
        collect_rcf_files(
            extracted_rcf_dir,
            output_path,
            &extracted_files,
            &mut records,
        )?;
        if records.is_empty() {
            return Err(
                ManifestError::Invalid(
                    "expanded manifest contains no source records".to_owned(),
                ),
            );
        }
        validate_existing_output(
            store,
            output_path,
        )?;
        records.sort();
        let mut text = kind_taxonomy_jsonl();
        text.push('\n');
        text.push_str(EXPANDED_SCHEMA_LINE);
        text.push('\n');
        for record in &records {
            text.push_str(record);
            text.push('\n');
        }
        store
            .write(
                output_path,
                &text,
            )
            .map_err(
                |error| {
                    ManifestError::io(
                        "write",
                        output_path.to_path_buf(),
                        error,
                    )
                },
            )?;
        Ok(
            GenerateExpandedReport {
                record_count: records.len(),
                output_path: output_path.to_path_buf(),
            },
        )
    }
}

/// Validates output ownership and nonoverlapping evidence roots.
fn validate_paths(
    game_dir: &Path,
    extracted_rcf_dir: &Path,
    output_path: &Path,
) -> Result<(), ManifestError> {
    if extension_of(output_path) != "jsonl" {
        return Err(
            ManifestError::Invalid(
                "expanded output path must use the jsonl extension".to_owned(),
            ),
        );
    }
    validate_output_destination(
        game_dir,
        output_path,
    )?;
    let comparable_game = comparable_path(
        "game", game_dir,
    )?;
    let comparable_extracted = comparable_path(
        "extracted RCF",
        extracted_rcf_dir,
    )?;
    let comparable_output = comparable_path(
        "expanded output",
        output_path,
    )?;
    if comparable_output.starts_with(&comparable_extracted) {
        return Err(
            ManifestError::Invalid(
                "expanded output must not be inside extracted RCF evidence"
                    .to_owned(),
            ),
        );
    }
    if comparable_game == comparable_extracted {
        return Err(
            ManifestError::Invalid(
                "game and extracted RCF roots must differ".to_owned(),
            ),
        );
    }
    if comparable_game.starts_with(&comparable_extracted)
        || comparable_extracted.starts_with(&comparable_game)
    {
        return Err(
            ManifestError::Invalid(
                "game and extracted RCF roots must not overlap".to_owned(),
            ),
        );
    }
    Ok(())
}

/// Validates ownership of one existing expanded output artifact.
fn validate_existing_output(
    store: &impl TextArtifactStore,
    output_path: &Path,
) -> Result<(), ManifestError> {
    let existing = store
        .read_optional(output_path)
        .map_err(
            |error| {
                ManifestError::io(
                    "read",
                    output_path.to_path_buf(),
                    error,
                )
            },
        )?;
    let taxonomy_header = kind_taxonomy_jsonl();
    if existing
        .as_deref()
        .is_some_and(
            |text| {
                if text.contains('\r') || !text.ends_with('\n') {
                    return true;
                }
                let mut lines = text.lines();
                lines.next() != Some(taxonomy_header.as_str())
                    || lines.next() != Some(EXPANDED_SCHEMA_LINE)
            },
        )
    {
        return Err(
            ManifestError::Invalid(
                "expanded output path contains unrelated data".to_owned(),
            ),
        );
    }
    Ok(())
}

/// Produces one traversal-free path coordinate for identity comparisons.
fn comparable_path(
    _label: &str,
    path: &Path,
) -> Result<PathBuf, ManifestError> {
    let mut comparable = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                return Err(
                    ManifestError::Invalid(
                        "expanded path contains parent traversal".to_owned(),
                    ),
                );
            }
            _ => {
                comparable.push(component.as_os_str());
            }
        }
    }
    if comparable
        .as_os_str()
        .is_empty()
    {
        return Err(
            ManifestError::Invalid("expanded path is empty".to_owned()),
        );
    }
    Ok(comparable)
}

/// Rejects output aliases that escape or resolve to protected game artifacts.
fn validate_output_destination(
    game_dir: &Path,
    output_path: &Path,
) -> Result<(), ManifestError> {
    let minimum_manifest = game_dir.join(MANIFEST_FILE_NAME);
    if output_path == minimum_manifest {
        return Err(
            ManifestError::Invalid(
                "expanded output must not replace the minimum manifest"
                    .to_owned(),
            ),
        );
    }
    let Ok(relative) = output_path.strip_prefix(game_dir) else {
        return Ok(());
    };
    let resolved = resolve_under(
        game_dir, relative,
    )
    .map_err(
        |error| {
            ManifestError::Invalid(
                format!("invalid expanded output path: {error}"),
            )
        },
    )?;
    if resolved == minimum_manifest {
        return Err(
            ManifestError::Invalid(
                "expanded output must not replace the minimum manifest"
                    .to_owned(),
            ),
        );
    }
    Ok(())
}

/// Loads one required directory snapshot through the tree port.
fn required_files(
    tree: &impl GameTree,
    root: &Path,
    label: &str,
) -> Result<Vec<PathBuf>, ManifestError> {
    let kind = tree
        .kind(root)
        .map_err(
            |error| {
                ManifestError::io(
                    "inspect",
                    root.to_path_buf(),
                    error,
                )
            },
        )?;
    if kind != PathKind::Directory {
        return Err(
            ManifestError::Invalid(
                format!(
                    "{label} directory not found: {}",
                    super::diagnostic_path::escaped_path(root)
                ),
            ),
        );
    }
    tree.files(root)
        .map_err(
            |error| {
                ManifestError::io(
                    "scan",
                    root.to_path_buf(),
                    error,
                )
            },
        )
}

/// Converts game-tree paths into expanded ledger rows.
fn collect_game_files(
    root: &Path,
    output_path: &Path,
    files: &[PathBuf],
    records: &mut Vec<String>,
) -> Result<(), ManifestError> {
    for path in files {
        if path == output_path {
            continue;
        }
        let extension = extension_of(path);
        let file_name = path
            .file_name()
            .and_then(|value| value.to_str())
            .ok_or_else(
                || {
                    ManifestError::Invalid(
                        "expanded file name is not valid UTF-8".to_owned(),
                    )
                },
            )?;
        let is_root_output_name = path.parent() == Some(root)
            && (file_name == MANIFEST_FILE_NAME
                || file_name == EXPANDED_MANIFEST_FILE_NAME);
        if is_root_output_name
            || extension == "rcf"
            || extension == BACKUP_EXTENSION
        {
            continue;
        }
        let relative = portable_relative(
            root, path,
        )?;
        records.push(
            record_json(
                "game", &relative, &extension,
            )
            .map_err(ManifestError::Invalid)?,
        );
    }
    Ok(())
}

/// Converts extracted RCF paths into expanded ledger rows.
fn collect_rcf_files(
    root: &Path,
    output_path: &Path,
    files: &[PathBuf],
    records: &mut Vec<String>,
) -> Result<(), ManifestError> {
    for path in files {
        if path == output_path {
            continue;
        }
        let extension = extension_of(path);
        if extension == BACKUP_EXTENSION {
            continue;
        }
        let relative = portable_relative(
            root, path,
        )?;
        records.push(
            record_json(
                "rcf_extracted",
                &relative,
                &extension,
            )
            .map_err(ManifestError::Invalid)?,
        );
    }
    Ok(())
}

/// Produces a portable slash-separated path relative to one root.
fn portable_relative(
    root: &Path,
    path: &Path,
) -> Result<String, ManifestError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|error| ManifestError::Invalid(error.to_string()))?;
    let resolved = resolve_under(
        root, relative,
    )
    .map_err(|error| ManifestError::Invalid(error.to_string()))?;
    let validated_relative = resolved
        .strip_prefix(root)
        .map_err(|error| ManifestError::Invalid(error.to_string()))?;
    let portable = validated_relative
        .to_str()
        .ok_or_else(
            || {
                ManifestError::Invalid(
                    "expanded path is not valid UTF-8".to_owned(),
                )
            },
        )?;
    Ok(
        portable.replace(
            char::from(92),
            "/",
        ),
    )
}

/// Record json.
/// Renders one fully classified expanded ledger row.
fn record_json(
    source: &str,
    path: &str,
    extension: &str,
) -> Result<String, String> {
    let kind = classify_kind(
        path, extension,
    );
    if kind == "error" {
        return Err(format!("unclassified expanded file: {source}:{path}"));
    }
    let tags = classify_tags(
        path, extension,
    );
    let metadata = classify_metadata(
        path, extension,
    );
    Ok(
        format!(
            concat!(
                "{{\"source\":\"{}\",\"path\":\"{}\",",
                "\"ext\":\"{}\",\"kind\":\"{}\",",
                "\"tags\":[{}],\"metadata\":{{{}}}}}"
            ),
            escape(source),
            escape(path),
            escape(extension),
            escape(&kind),
            tags.iter()
                .map(
                    |tag| format!(
                        "\"{}\"",
                        escape(tag)
                    )
                )
                .collect::<Vec<_>>()
                .join(","),
            metadata
                .iter()
                .map(
                    |(key, value)| format!(
                        "\"{}\":\"{}\"",
                        escape(key),
                        escape(value)
                    )
                )
                .collect::<Vec<_>>()
                .join(",")
        ),
    )
}

/// Classify kind.
/// Classifies one expanded file into the controlled kind taxonomy.
fn classify_kind(
    _path: &str,
    extension: &str,
) -> String {
    classify_manifest_bucket(
        "", extension,
    )
}

/// Classify tags.
/// Derives deterministic semantic tags for one expanded file.
fn classify_tags(
    path: &str,
    extension: &str,
) -> Vec<&'static str> {
    let lower = path.to_ascii_lowercase();
    let mut tags = Vec::new();
    if is_dialog_path(&lower) {
        tags.push("dialog");
        tags.push(dialog_country_tag(&lower));
        return tags;
    }
    if extension == "p3d" {
        tags.push("p3d");
        tags.push("mixed_content_possible");
    }
    if is_ui_path(&lower) {
        tags.push("ui");
    }
    if is_mission_path(&lower) || extension == "mfk" {
        tags.push("mission");
    }
    if is_car_path(&lower) || is_vehicle_path(&lower) {
        tags.push("vehicle");
    }
    if is_world_path(&lower) {
        tags.push("world");
    }
    if extension == "rmv" {
        tags.push("cinematic");
    }
    if tags.is_empty() {
        tags.push("unclassified");
    }
    tags
}

/// Reports whether a portable path contains explicit UI evidence.
fn is_ui_path(lower: &str) -> bool {
    for token in lower.split(|value: char| !value.is_ascii_alphanumeric()) {
        if is_ui_token(token) {
            return true;
        }
    }
    false
}

/// Reports whether one normalized path token identifies a UI family.
fn is_ui_token(token: &str) -> bool {
    if is_frontend_token(token) {
        return true;
    }
    is_scrooby_token(token)
}

/// Reports whether one normalized path token identifies a frontend family.
fn is_frontend_token(token: &str) -> bool {
    token == "frontend"
        || token == "frontends"
        || has_numeric_suffix(
            token, "frontend",
        )
}

/// Reports whether one normalized path token identifies a Scrooby family.
fn is_scrooby_token(token: &str) -> bool {
    if matches!(
        token,
        "scrooby" | "scroobys" | "scroobies"
    ) {
        return true;
    }
    has_numeric_suffix(
        token, "scrooby",
    )
}

/// Reports whether a portable path contains explicit world evidence.
fn is_world_path(lower: &str) -> bool {
    for token in lower.split(|value: char| !value.is_ascii_alphanumeric()) {
        if is_world_token(token) {
            return true;
        }
    }
    false
}

/// Reports whether one normalized path token identifies a world family.
fn is_world_token(token: &str) -> bool {
    if matches!(
        token,
        "world"
            | "worlds"
            | "level"
            | "levels"
            | "terra"
            | "terras"
            | "terrain"
            | "terrains"
    ) {
        return true;
    }
    for prefix in [
        "world", "level", "terra", "terrain",
    ] {
        if has_numeric_suffix(
            token, prefix,
        ) {
            return true;
        }
    }
    false
}

/// Reports whether a portable path contains explicit mission evidence.
fn is_mission_path(lower: &str) -> bool {
    for token in lower.split(|value: char| !value.is_ascii_alphanumeric()) {
        if is_mission_token(token) {
            return true;
        }
    }
    false
}

/// Reports whether one normalized path token identifies a mission family.
fn is_mission_token(token: &str) -> bool {
    token == "mission"
        || token == "missions"
        || has_numeric_suffix(
            token, "mission",
        )
}

/// Reports whether a portable path contains explicit vehicle evidence.
fn is_vehicle_path(lower: &str) -> bool {
    for token in lower.split(|value: char| !value.is_ascii_alphanumeric()) {
        if is_vehicle_token(token) {
            return true;
        }
    }
    false
}

/// Reports whether one normalized path token identifies a vehicle family.
fn is_vehicle_token(token: &str) -> bool {
    token == "vehicle"
        || token == "vehicles"
        || has_numeric_suffix(
            token, "vehicle",
        )
}

/// Reports whether a portable path contains explicit car evidence.
fn is_car_path(lower: &str) -> bool {
    for token in lower.split(|value: char| !value.is_ascii_alphanumeric()) {
        if is_car_token(token) {
            return true;
        }
    }
    false
}

/// Reports whether one normalized path token identifies a car family.
fn is_car_token(token: &str) -> bool {
    token == "car"
        || token == "cars"
        || has_numeric_suffix(
            token, "car",
        )
}

/// Reports whether one token has a nonempty decimal suffix after a prefix.
fn has_numeric_suffix(
    token: &str,
    prefix: &str,
) -> bool {
    let Some(suffix) = token.strip_prefix(prefix) else {
        return false;
    };
    if suffix.is_empty() {
        return false;
    }
    for character in suffix.chars() {
        if !character.is_ascii_digit() {
            return false;
        }
    }
    true
}

/// Classify metadata.
/// Derives deterministic metadata pairs for one expanded file.
fn classify_metadata(
    path: &str,
    extension: &str,
) -> Vec<(
    &'static str,
    &'static str,
)> {
    let lower = path.to_ascii_lowercase();
    let mut metadata = Vec::new();
    metadata.push(
        (
            "classification_basis",
            "path_and_extension_heuristic",
        ),
    );
    if extension == "p3d" {
        metadata.push(
            (
                "p3d_scope",
                "container_level_only",
            ),
        );
        metadata.push(
            (
                "chunk_level_required",
                "true",
            ),
        );
    }
    if is_dialog_path(&lower) {
        metadata.push(
            (
                "audio_scope",
                "dialog",
            ),
        );
        metadata.push(
            (
                "country",
                dialog_country_metadata(&lower),
            ),
        );
    }
    if extension == "rmv" {
        metadata.push(
            (
                "normalized_by",
                "rmv",
            ),
        );
    }
    metadata
}

/// Is dialog path.
/// Reports whether a portable path belongs to a dialog tree.
fn is_dialog_path(lower: &str) -> bool {
    lower.starts_with("dialog/")
        || lower.starts_with("dialogf/")
        || lower.starts_with("dialogg/")
        || lower.starts_with("dialogs/")
}

/// Dialog country tag.
/// Returns the stable country tag for one dialog path.
fn dialog_country_tag(lower: &str) -> &'static str {
    if lower.starts_with("dialogs/") {
        "spanish_spain"
    } else if lower.starts_with("dialogf/") {
        "french"
    } else if lower.starts_with("dialogg/") {
        "german"
    } else {
        "english"
    }
}

/// Dialog country metadata.
/// Returns the stable country metadata for one dialog path.
fn dialog_country_metadata(lower: &str) -> &'static str {
    dialog_country_tag(lower)
}

/// Escape.
/// Escapes one JSON string value without emitting control characters.
fn escape(value: &str) -> String {
    let mut output = String::new();
    for character in value.chars() {
        match character {
            '\u{0008}' => push_short_escape(
                &mut output,
                'b',
            ),
            '\u{000c}' => push_short_escape(
                &mut output,
                'f',
            ),
            '\n' => push_short_escape(
                &mut output,
                'n',
            ),
            '\r' => push_short_escape(
                &mut output,
                'r',
            ),
            '\t' => push_short_escape(
                &mut output,
                't',
            ),
            current if current == char::from(34) => {
                output.push(char::from(92));
                output.push(char::from(34));
            }
            current if current == char::from(92) => {
                output.push(char::from(92));
                output.push(char::from(92));
            }
            current if current.is_control() => {
                push_unicode_escape(
                    &mut output,
                    u32::from(current),
                );
            }
            current => output.push(current),
        }
    }
    output
}

/// Appends one two-character JSON escape.
fn push_short_escape(
    output: &mut String,
    character: char,
) {
    output.push(char::from(92));
    output.push(character);
}

/// Appends one four-digit lowercase JSON Unicode escape.
fn push_unicode_escape(
    output: &mut String,
    value: u32,
) {
    output.push(char::from(92));
    output.push('u');
    for shift in [
        12_u32, 8_u32, 4_u32, 0_u32,
    ] {
        output.push(hex_digit((value >> shift) & 0x0f));
    }
}

/// Maps one hexadecimal nibble to its lowercase character.
const fn hex_digit(nibble: u32) -> char {
    match nibble {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => '?',
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::{
        collect_game_files, escape, portable_relative, validate_paths,
    };

    #[test]
    fn overlapping_roots_fail_before_output_read() {
        let result = validate_paths(
            Path::new("game"),
            Path::new("game"),
            Path::new("output/result.jsonl"),
        );

        assert!(result.is_err());
    }

    #[test]
    fn escape_preserves_control_character_identity() {
        assert_eq!(
            escape("line\nfield\t\u{0001}"),
            "line\\nfield\\t\\u0001"
        );
    }

    #[cfg(unix)]
    #[test]
    fn portable_relative_rejects_invalid_utf8() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt as _;

        let component = PathBuf::from(
            OsString::from_vec(
                vec![
                    b'b', 0xff_u8, b'x',
                ],
            ),
        );
        let mut path = PathBuf::from("game");
        path.push(component);
        path.push("asset.p3d");

        assert!(
            portable_relative(
                Path::new("game"),
                &path,
            )
            .is_err()
        );
    }

    #[cfg(unix)]
    #[test]
    fn game_collection_rejects_non_unicode_file_name() {
        use std::ffi::OsString;
        use std::os::unix::ffi::OsStringExt as _;

        let file_name = OsString::from_vec(
            vec![
                b'b', 0xff_u8, b'x', b'.', b'p', b'3', b'd',
            ],
        );
        let path = PathBuf::from("game").join(file_name);
        let mut records = Vec::new();
        let result = collect_game_files(
            Path::new("game"),
            Path::new("output/expanded.jsonl"),
            &[path],
            &mut records,
        );

        assert!(result.is_err());
        assert!(records.is_empty());
    }

    #[cfg(windows)]
    #[test]
    fn game_collection_rejects_non_unicode_file_name() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt as _;

        let file_name = OsString::from_wide(
            &[
                u16::from(b'b'),
                0xd800_u16,
                u16::from(b'x'),
                u16::from(b'.'),
                u16::from(b'p'),
                u16::from(b'3'),
                u16::from(b'd'),
            ],
        );
        let path = PathBuf::from("game").join(file_name);
        let mut records = Vec::new();
        let result = collect_game_files(
            Path::new("game"),
            Path::new("output/expanded.jsonl"),
            &[path],
            &mut records,
        );

        assert!(result.is_err());
        assert!(records.is_empty());
    }

    #[cfg(windows)]
    #[test]
    fn portable_relative_rejects_unpaired_utf16() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt as _;

        let component = PathBuf::from(
            OsString::from_wide(
                &[
                    u16::from(b'b'),
                    0xd800_u16,
                    u16::from(b'x'),
                ],
            ),
        );
        let mut path = PathBuf::from("game");
        path.push(component);
        path.push("asset.p3d");

        assert!(
            portable_relative(
                Path::new("game"),
                &path,
            )
            .is_err()
        );
    }
}
