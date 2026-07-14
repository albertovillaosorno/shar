// File:
//   - stragglers.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/two/stragglers.rs
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
//   - The stragglers contract for pipeline phase two.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute stragglers.
// - Split-When:
//   - Split when stragglers contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Normalize game stragglers.
// - Description:
//   - Defines stragglers data and behavior for pipeline phase two.
// - Usage:
//   - Used by pipeline phase two code that needs stragglers.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Normalize game stragglers keeps tightly coupled validation,
//   - ordering, and deterministic transformation invariants together; split
//   - when a stable independently testable sub-boundary is identified.
//

//! Normalize game stragglers.
//!
//! This boundary keeps normalize game stragglers explicit and returns
//! deterministic results to pipeline callers.
/// Choreography module.
pub(super) mod choreography;
/// Commands.
pub(super) mod commands;
/// Error log.
pub(super) mod error_log;
/// Json.
pub(super) mod json;
/// Scrooby.
pub(super) mod scrooby;
/// Sound type.
pub(super) mod sound_type;
/// Textbible.
pub(super) mod textbible;

use std::borrow::Cow;
use std::fs;
use std::path::{Path, PathBuf};

use json::JsonObject;
use rmv::Sha256;
use rsd::RsdAudio;
use schoenwald_filesystem::adapters::driving::local::{
    create_dir_all as local_create_dir_all, file_len as local_file_len,
    read_bytes as local_read_bytes, write_bytes as local_write_bytes,
};

use super::localization::encoding;
use crate::adapters::driven::local::filesystem::collect_files;
use crate::domain::{PipelineError, StageReport};

/// Result.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Json extensions.
const JSON_EXTENSIONS: &[&str] = &[
    "mfk", "con", "pag", "scr", "prj", "err", "cho", "txt", "e", "f", "g", "i",
    "s", "x", "typ",
];

/// Converts one in-memory artifact length into report bytes.
fn artifact_length(
    stage: &'static str,
    length: usize,
) -> PipelineOutcome<u64> {
    let message = format!("{stage} artifact length overflowed");
    match u64::try_from(length) {
        Ok(converted_length) => Ok(converted_length),
        Err(_error) => Err(PipelineError::new(message)),
    }
}

/// Return one source path relative to the declared game root.
fn relative_source_path<'source>(
    source: &'source Path,
    game_root: &Path,
) -> PipelineOutcome<&'source Path> {
    source
        .strip_prefix(game_root)
        .map_err(
            |_error| {
                PipelineError::new(
                    format!(
                        "failed to relativize {}",
                        source.display()
                    ),
                )
            },
        )
}

/// Normalize game stragglers.
///
/// # Errors
///
/// Returns an error when validation, filesystem access, or output writing
pub(super) fn normalize_game_stragglers(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    let output_root = extracted_root.join("game");
    if output_root.exists() {
        fs::remove_dir_all(&output_root).map_err(io_error(&output_root))?;
    }
    local_create_dir_all(&output_root).map_err(io_error(&output_root))?;

    let mut files = 0usize;
    let mut bytes = 0u64;
    for source in collect_files(game_root)? {
        let ext = extension_of(&source);
        if JSON_EXTENSIONS.contains(&ext.as_str()) {
            let relative = relative_source_path(
                &source, game_root,
            )?;
            let output = normalized_json_path(
                extracted_root,
                relative,
            );
            let json = semantic_json_for(
                &source, relative, &ext,
            )?;
            write_bytes(
                &output,
                json.as_bytes(),
            )?;
            files = StageReport::checked_file_total(
                "phase-two-stragglers",
                files,
                1,
            )?;
            bytes = StageReport::checked_byte_total(
                "phase-two-stragglers",
                bytes,
                artifact_length(
                    "phase-two-stragglers",
                    json.len(),
                )?,
            )?;
        } else if ext == "rsd" {
            let relative = relative_source_path(
                &source, game_root,
            )?;
            let output = normalized_wav_path(
                extracted_root,
                relative,
            );
            let source_bytes =
                local_read_bytes(&source).map_err(io_error(&source))?;
            let wav = rsd_to_wav(
                &source_bytes,
                relative,
            )?;
            if extracted_duplicate_wav_exists(
                extracted_root,
                &output,
                &wav,
            )? {
                continue;
            }
            write_bytes(
                &output, &wav,
            )?;
            files = StageReport::checked_file_total(
                "phase-two-stragglers",
                files,
                1,
            )?;
            bytes = StageReport::checked_byte_total(
                "phase-two-stragglers",
                bytes,
                artifact_length(
                    "phase-two-stragglers",
                    wav.len(),
                )?,
            )?;
        }
    }

    Ok(
        StageReport {
            name: "phase-two-stragglers",
            files,
            bytes,
            note: "loose game stragglers normalized under extracted/game as \
                   semantic JSON or WAV"
                .to_owned(),
        },
    )
}

#[must_use]
/// Normalized json path.
pub(super) fn normalized_json_path(
    extracted_root: &Path,
    relative: &Path,
) -> PathBuf {
    let mut output = extracted_root
        .join("game")
        .join(relative);
    let file_name = relative
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("straggler");
    output.set_file_name(format!("{file_name}.json"));
    output
}

#[must_use]
/// Normalized wav path.
pub(super) fn normalized_wav_path(
    extracted_root: &Path,
    relative: &Path,
) -> PathBuf {
    let mut output = extracted_root
        .join("game")
        .join(relative);
    let file_name = relative
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("audio.rsd");
    output.set_file_name(format!("{file_name}.wav"));
    output
}

#[must_use]
/// Is json straggler extension.
pub(super) fn is_json_straggler_extension(extension: &str) -> bool {
    JSON_EXTENSIONS.contains(&extension)
}

/// Semantic json for.
fn semantic_json_for(
    source: &Path,
    relative: &Path,
    ext: &str,
) -> PipelineOutcome<String> {
    let bytes = local_read_bytes(source).map_err(io_error(source))?;
    let text = decode_straggler_text(
        &bytes, relative, ext,
    )?;
    let text = text.as_ref();
    let mut json = JsonObject::new();
    json.field(
        "schema",
        schema_for(ext),
    );
    json.field(
        "source_extension",
        ext,
    );
    json.field(
        "route_class",
        route_class(
            relative, ext,
        ),
    );
    json.number(
        "source_bytes",
        u64::try_from(bytes.len()).unwrap_or(u64::MAX),
    );

    match ext {
        "mfk" | "con" => commands::append_summary(
            &mut json, text, ext,
        ),
        "pag" | "scr" | "prj" => scrooby::append_summary(
            &mut json, text,
        ),
        "cho" => choreography::append_summary(
            &mut json, text,
        ),
        "txt" | "e" | "f" | "g" | "i" | "s" | "x" => {
            textbible::append_summary(
                &mut json, text, ext,
            );
        }
        "typ" => sound_type::append_summary(
            &mut json, &bytes,
        ),
        "err" => error_log::append_summary(
            &mut json, text,
        ),
        _ => json.number(
            "line_count",
            text.lines()
                .filter(
                    |line| {
                        !line
                            .trim()
                            .is_empty()
                    },
                )
                .count()
                .try_into()
                .unwrap_or(u64::MAX),
        ),
    }

    Ok(json.finish())
}

/// Decode one text straggler without replacing malformed source bytes.
///
/// UTF-8 stays the primary contract; bytes that fail it decode through the
/// shared strict Windows-1252 mapping because original-era text sources store
/// accented FIGS letters and symbol glyphs as single code-page bytes. The
/// five undefined Windows-1252 code points still fail closed.
fn decode_straggler_text<'bytes>(
    bytes: &'bytes [u8],
    relative: &Path,
    ext: &str,
) -> PipelineOutcome<Cow<'bytes, str>> {
    if ext == "typ" {
        return Ok(Cow::Borrowed(""));
    }
    match std::str::from_utf8(bytes) {
        Ok(text) => Ok(Cow::Borrowed(text)),
        Err(_utf8_error) => encoding::windows_1252_to_string(bytes)
            .map(Cow::Owned)
            .map_err(
                |byte| {
                    PipelineError::new(
                        format!(
                            "{}: byte 0x{byte:02x} is not defined in \
                             Windows-1252",
                            relative.display()
                        ),
                    )
                },
            ),
    }
}

/// Schema for.
fn schema_for(ext: &str) -> &'static str {
    match ext {
        "mfk" | "con" => commands::schema_for(ext),
        "pag" | "scr" | "prj" => scrooby::schema_for(ext),
        "cho" => "shar-schoenwald.straggler.choreography.v1",
        "typ" => "shar-schoenwald.straggler.sound-type.v1",
        "err" => "shar-schoenwald.straggler.error-log.v1",
        "txt" | "e" | "f" | "g" | "i" | "s" | "x" => {
            "shar-schoenwald.straggler.text-bible.v1"
        }
        _ => "shar-schoenwald.straggler.text.v1",
    }
}

/// Route class.
fn route_class(
    relative: &Path,
    ext: &str,
) -> &'static str {
    let route = relative
        .to_string_lossy()
        .to_ascii_lowercase();
    if ext == "mfk" || route.contains("mission") {
        "mission"
    } else if ext == "con" || route.contains("car") {
        "vehicle-config"
    } else if matches!(
        ext,
        "pag" | "scr" | "prj"
    ) || route.contains("scrooby")
    {
        "frontend-ui"
    } else if ext == "cho" || route.contains("chars") {
        "character-animation"
    } else if ext == "typ" || route.contains("sound") {
        "sound-metadata"
    } else if matches!(
        ext,
        "txt" | "e" | "f" | "g" | "i" | "s" | "x"
    ) {
        "localization"
    } else if ext == "err" {
        "build-artifact"
    } else {
        "loose-game-file"
    }
}

/// Extracted duplicate wav exists.
fn extracted_duplicate_wav_exists(
    extracted_root: &Path,
    output: &Path,
    wav: &[u8],
) -> PipelineOutcome<bool> {
    let digest = Sha256::digest(wav).hex();
    for existing in collect_files(extracted_root)? {
        if existing == output
            || existing.starts_with(extracted_root.join("game"))
        {
            continue;
        }
        let is_wav = existing
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case("wav"));
        if !is_wav {
            continue;
        }
        let existing_length =
            local_file_len(&existing).map_err(io_error(&existing))?;
        if existing_length != u64::try_from(wav.len()).unwrap_or(u64::MAX) {
            continue;
        }
        let bytes = local_read_bytes(&existing).map_err(io_error(&existing))?;
        if Sha256::digest(&bytes).hex() == digest {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Rsd to wav.
fn rsd_to_wav(
    bytes: &[u8],
    relative: &Path,
) -> PipelineOutcome<Vec<u8>> {
    let source = relative
        .display()
        .to_string();
    let audio = RsdAudio::parse(bytes)
        .map_err(|error| PipelineError::new(format!("{source}: {error}")))?;
    let wav = audio
        .to_wav()
        .map_err(|error| PipelineError::new(format!("{source}: {error}")))?;
    wav.to_bytes()
        .map_err(|error| PipelineError::new(format!("{source}: {error}")))
}

/// Extension of.
fn extension_of(path: &Path) -> String {
    path.extension()
        .and_then(|value| value.to_str())
        .map_or_else(
            || "error".to_owned(),
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
    use std::path::Path;

    use super::decode_straggler_text;

    #[test]
    fn decodes_windows_1252_text_stragglers() {
        let result = decode_straggler_text(
            b"Logitech\xae Force",
            Path::new("synthetic/era.txt"),
            "txt",
        );
        assert!(
            result.as_deref() == Ok("Logitech\u{ae} Force"),
            "era Windows-1252 bytes must decode deterministically"
        );
    }

    #[test]
    fn rejects_undefined_windows_1252_text_stragglers() {
        let result = decode_straggler_text(
            &[0x81_u8],
            Path::new("synthetic/invalid.txt"),
            "txt",
        );
        assert!(
            result.is_err(),
            "bytes Windows-1252 leaves undefined must fail closed"
        );
    }
}
