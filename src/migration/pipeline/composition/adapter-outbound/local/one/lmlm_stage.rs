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
//   - Lmlm stage outbound adapter.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Lmlm stage outbound adapter.
// - Description:
//   - Implements the declared outbound adapter responsibility for pipeline.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Lmlm stage outbound adapter.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use lmlm::{FileEntry, entry_bytes, parse as parse_lmlm};
use rmv::Sha256;
use rsd::RsdAudio;
use schoenwald_filesystem::adapters::driving::local::{
    create_dir_all as local_create_dir_all, read_bytes as local_read_bytes,
    write_bytes as local_write_bytes,
};

use super::media_dependencies::{ensure_ffmpeg_dependency, media_tool_path};
use crate::adapters::driven::check_cancellation;
use crate::adapters::driven::local::progress::StageProgress;
use crate::domain::{PipelineError, StageReport, escape_json as json_escape};

#[rustfmt::skip]
#[path = "lmlm_stage/optional_mods.rs"]
mod optional_mods;
#[rustfmt::skip]
#[path = "lmlm_stage/preview.rs"]
mod preview;

pub(in crate::adapters::driven::local) use preview::preview_optional_mods;

use optional_mods::{
    OptionalModCounts, OptionalModRole, apply_remaster,
    claim_normalized_output, create_optional_mod_work_root,
    discover_optional_mods, existing_file_index, is_latino_audio_path,
    is_latino_movie_path, read_optional_mod_bytes,
    require_package_byte_approval,
};

/// Rejects optional package application without caller approval.
pub(in crate::adapters::driven::local) fn require_optional_mod_approval(
    game_root: &Path,
    approved: Option<&str>,
) -> PipelineOutcome<Option<String>> {
    optional_mods::require_optional_mod_approval(game_root, approved)
}

/// Current optional-package extraction manifest schema.
const OPTIONAL_MOD_EXTRACT_SCHEMA: &str =
    "shar-schoenwald.optional-mod-extract.v3";

/// Rejects non-clean transitions from an unknown or different package set.
pub(in crate::adapters::driven::local) fn ensure_optional_mod_transition(
    extracted_root: &Path,
    current_token: Option<&str>,
) -> PipelineOutcome<()> {
    let output_root = extracted_root.join("lmlm");
    if !output_root.exists() {
        return Ok(());
    }
    let output_metadata =
        fs::symlink_metadata(&output_root).map_err(|error| {
            PipelineError::new(format!(
                "failed to inspect existing optional output ({:?})",
                error.kind()
            ))
        })?;
    if !output_metadata.is_dir() || output_metadata.file_type().is_symlink() {
        return Err(PipelineError::new(concat!(
            "existing optional output must be a real directory; ",
            "run clean extract-game"
        )));
    }
    let manifest_path = output_root.join("manifest.json");
    let manifest_metadata =
        fs::symlink_metadata(&manifest_path).map_err(|_error| {
            PipelineError::new(concat!(
                "existing optional output has no verifiable manifest; ",
                "run clean extract-game"
            ))
        })?;
    if !manifest_metadata.is_file()
        || manifest_metadata.file_type().is_symlink()
    {
        return Err(PipelineError::new(concat!(
            "existing optional manifest must be a real file; ",
            "run clean extract-game"
        )));
    }
    let bytes = local_read_bytes(&manifest_path).map_err(|error| {
        PipelineError::new(format!(
            "failed to read existing optional manifest ({:?})",
            error.kind()
        ))
    })?;
    let document: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|_error| {
            PipelineError::new(
                "existing optional manifest is invalid; run clean extract-game",
            )
        })?;
    if document.get("schema").and_then(serde_json::Value::as_str)
        != Some(OPTIONAL_MOD_EXTRACT_SCHEMA)
    {
        return Err(PipelineError::new(concat!(
            "existing optional manifest cannot verify package continuity; ",
            "run clean extract-game"
        )));
    }
    let previous = match document.get("approval_token") {
        Some(value) if value.is_null() => None,
        Some(value) => value.as_str(),
        None => {
            return Err(PipelineError::new(concat!(
                "existing optional manifest omits package identity; ",
                "run clean extract-game"
            )));
        }
    };
    if previous == current_token {
        Ok(())
    } else {
        Err(PipelineError::new(concat!(
            "optional package set changed; run clean extract-game ",
            "before continuing"
        )))
    }
}

/// Result.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Applies zero, one, or both supported local packages.
pub(super) fn extract_lmlm(
    game_root: &Path,
    extracted_root: &Path,
    approved: Option<&str>,
) -> PipelineOutcome<StageReport> {
    let output_root = extracted_root.join("lmlm");
    let archives = discover_optional_mods(game_root)?;
    if archives.is_empty() {
        let actual =
            require_package_byte_approval(std::iter::empty(), approved)?;
        ensure_optional_mod_transition(extracted_root, actual.as_deref())?;
        if output_root.exists() {
            fs::remove_dir_all(&output_root).map_err(io_error(&output_root))?;
        }
        return Ok(StageReport {
            name: "lmlm",
            files: 0,
            bytes: 0,
            note: "no supported optional LMLM packages present".to_owned(),
        });
    }
    let loaded_archives = archives
        .into_iter()
        .map(|archive| {
            check_cancellation()?;
            let data = read_optional_mod_bytes(&archive)?;
            Ok((archive, data))
        })
        .collect::<PipelineOutcome<Vec<_>>>()?;
    let actual_token = require_package_byte_approval(
        loaded_archives
            .iter()
            .map(|(archive, data)| (archive.role, data.as_slice())),
        approved,
    )?;
    ensure_optional_mod_transition(extracted_root, actual_token.as_deref())?;
    let parsed_archives = loaded_archives
        .into_iter()
        .map(|(archive, data)| {
            check_cancellation()?;
            let entries = parse_lmlm(&data).map_err(|error| {
                PipelineError::new(format!("{}: {error}", archive.role.alias()))
            })?;
            Ok((archive, data, entries))
        })
        .collect::<PipelineOutcome<Vec<_>>>()?;

    if output_root.exists() {
        fs::remove_dir_all(&output_root).map_err(io_error(&output_root))?;
    }
    local_create_dir_all(&output_root).map_err(io_error(&output_root))?;
    let base_files =
        existing_file_index(game_root, extracted_root, &output_root)?;

    let work_root = create_optional_mod_work_root("extract")?;

    let mut files_written = 0_usize;
    let mut bytes_written = 0_u64;
    let mut package_records = Vec::new();
    let mut member_records = Vec::new();
    for (archive, data, entries) in parsed_archives {
        check_cancellation()?;
        let counts = match archive.role {
            OptionalModRole::Remaster => apply_remaster(
                &data,
                &entries,
                extracted_root,
                &base_files,
                &mut member_records,
            )?,
            OptionalModRole::Latino => extract_latino_media(
                &data,
                &entries,
                work_root.path(),
                &output_root,
                extracted_root,
                &mut member_records,
            )?,
        };
        if counts.written == 0 {
            return Err(PipelineError::new(format!(
                "{} contains no supported {} payload",
                archive.role.alias(),
                archive.role.label()
            )));
        }
        files_written = files_written.saturating_add(counts.written);
        bytes_written = bytes_written.saturating_add(counts.bytes);
        package_records.push(package_record(archive.role, counts));
    }
    work_root.cleanup()?;

    let manifest = format!(
        concat!(
            "{{\"schema\":\"{}\",",
            "\"approval_token\":\"{}\",",
            "\"aliases\":{{\"m.lmlm\":\"remaster\",",
            "\"j.lmlm\":\"latino\"}},",
            "\"remaster_policy\":\"replace existing base files only; ",
            "skip every additional member\",",
            "\"latino_policy\":\"add voice WAV and cinematic-audio WAV ",
            "only; never replace base output\",",
            "\"packages\":[{}],",
            "\"records\":[{}]}}\n"
        ),
        OPTIONAL_MOD_EXTRACT_SCHEMA,
        actual_token.as_deref().ok_or_else(|| {
            PipelineError::new("approved package set omitted its identity")
        })?,
        package_records.join(","),
        member_records.join(",")
    );
    let manifest_path = output_root.join("manifest.json");
    write_bytes(&manifest_path, manifest.as_bytes())?;
    files_written = files_written.saturating_add(1);
    bytes_written = bytes_written
        .saturating_add(u64::try_from(manifest.len()).unwrap_or(u64::MAX));
    Ok(StageReport {
        name: "lmlm",
        files: files_written,
        bytes: bytes_written,
        note: "supported optional packages applied by alias and role policy"
            .to_owned(),
    })
}

/// Renders one deterministic package summary.
fn package_record(role: OptionalModRole, counts: OptionalModCounts) -> String {
    format!(
        concat!(
            "{{\"alias\":\"{}\",",
            "\"role\":\"{}\",",
            "\"written\":{},",
            "\"skipped\":{},",
            "\"bytes\":{}}}"
        ),
        role.alias(),
        role.label(),
        counts.written,
        counts.skipped,
        counts.bytes
    )
}

/// Converts only Latino dialogue and cinematic audio into isolated WAV output.
fn extract_latino_media(
    data: &[u8],
    entries: &[FileEntry],
    work_root: &Path,
    output_root: &Path,
    extracted_root: &Path,
    records: &mut Vec<String>,
) -> PipelineOutcome<OptionalModCounts> {
    let latino_root = output_root.join("latino");
    let mut claimed_outputs = BTreeMap::new();
    for entry in entries {
        check_cancellation()?;
        let _bytes = entry_bytes(data, entry).ok_or_else(|| {
            PipelineError::new(format!(
                "{}: LMLM entry out of bounds",
                entry.path
            ))
        })?;
        let destination = if is_latino_audio_path(&entry.path) {
            Some(
                lmlm_entry_path(&latino_root, &entry.path)
                    .with_extension("wav"),
            )
        } else if is_latino_movie_path(&entry.path) {
            Some(latino_movie_destination(&latino_root, &entry.path))
        } else {
            None
        };
        if let Some(destination) = destination {
            let output = relative_output(extracted_root, &destination)?;
            claim_normalized_output(
                &mut claimed_outputs,
                &output,
                &entry.path,
                "Latino",
            )?;
        }
    }
    local_create_dir_all(&latino_root).map_err(io_error(&latino_root))?;
    let mut counts = OptionalModCounts::default();
    let mut progress = StageProgress::begin("latino members", entries.len());
    for (index, entry) in entries.iter().enumerate() {
        check_cancellation()?;
        progress.advance(&format!("member {}", index.saturating_add(1)));
        let bytes = entry_bytes(data, entry).ok_or_else(|| {
            PipelineError::new(format!(
                "{}: LMLM entry out of bounds",
                entry.path
            ))
        })?;
        if is_latino_audio_path(&entry.path) {
            let wav = rsd_bytes_to_wav(bytes, &entry.path)?;
            let destination = lmlm_entry_path(&latino_root, &entry.path)
                .with_extension("wav");
            write_lmlm_wav(
                &destination,
                &wav,
                &entry.path,
                "latino_voice_audio",
                None,
                extracted_root,
                records,
            )?;
            counts.written = counts.written.saturating_add(1);
            counts.bytes = counts
                .bytes
                .saturating_add(u64::try_from(wav.len()).unwrap_or(u64::MAX));
        } else if is_latino_movie_path(&entry.path) {
            let (movie_files, movie_bytes) = export_lmlm_movie_audio(
                work_root,
                &latino_root,
                &entry.path,
                bytes,
                extracted_root,
                records,
            )?;
            counts.written = counts.written.saturating_add(movie_files);
            counts.bytes = counts.bytes.saturating_add(movie_bytes);
        } else {
            counts.skipped = counts.skipped.saturating_add(1);
        }
    }
    progress.finish();
    Ok(counts)
}

/// Build a normalized output path for one LMLM entry.
fn lmlm_entry_path(root: &Path, entry_path: &str) -> PathBuf {
    let mut destination = root.to_path_buf();
    for component in entry_path.split('/') {
        destination.push(component);
    }
    destination
}

/// Public-safe generated path relative to the extraction root.
fn relative_output(root: &Path, path: &Path) -> PipelineOutcome<String> {
    path.strip_prefix(root)
        .map(|relative| {
            relative
                .to_string_lossy()
                .replace('\\', "/")
                .to_ascii_lowercase()
        })
        .map_err(|_error| PipelineError::new("output escaped extraction root"))
}

/// Rsd bytes to wav.
fn rsd_bytes_to_wav(bytes: &[u8], source: &str) -> PipelineOutcome<Vec<u8>> {
    let audio = RsdAudio::parse(bytes)
        .map_err(|error| PipelineError::new(format!("{source}: {error}")))?;
    let wav = audio
        .to_wav()
        .map_err(|error| PipelineError::new(format!("{source}: {error}")))?;
    wav.to_bytes()
        .map_err(|error| PipelineError::new(format!("{source}: {error}")))
}

/// Builds the deterministic output for one Latino cinematic audio stream.
fn latino_movie_destination(output_root: &Path, entry_path: &str) -> PathBuf {
    let stem = Path::new(entry_path)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("movie");
    output_root
        .join("movies")
        .join(stem)
        .join("audio_track_01.wav")
}

/// Exports only the first audio stream from one Latino cinematic.
fn export_lmlm_movie_audio(
    work_root: &Path,
    output_root: &Path,
    entry_path: &str,
    movie_bytes: &[u8],
    extracted_root: &Path,
    records: &mut Vec<String>,
) -> PipelineOutcome<(usize, u64)> {
    let Some(wav) =
        decode_lmlm_movie_audio(work_root, entry_path, movie_bytes)?
    else {
        records.push(format!(
            "{{\"kind\":\"latino_cinematic_audio\",\"source\":\"{}\",\
             \"status\":\"skipped_no_audio\"}}",
            json_escape(entry_path)
        ));
        return Ok((0, 0));
    };
    let destination = latino_movie_destination(output_root, entry_path);
    write_lmlm_wav(
        &destination,
        &wav,
        entry_path,
        "latino_cinematic_audio",
        Some(1),
        extracted_root,
        records,
    )?;
    Ok((1, u64::try_from(wav.len()).unwrap_or(u64::MAX)))
}

/// Decodes the first cinematic audio stream without publishing output.
fn decode_lmlm_movie_audio(
    work_root: &Path,
    entry_path: &str,
    movie_bytes: &[u8],
) -> PipelineOutcome<Option<Vec<u8>>> {
    ensure_ffmpeg_dependency().map_err(PipelineError::new)?;
    let source_path = Path::new(entry_path);
    let stem = source_path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("movie");
    let extension = source_path
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("bik");
    let temp_movie = work_root.join(format!("{stem}.{extension}"));
    write_bytes(&temp_movie, movie_bytes)?;
    if ffprobe_audio_stream_count(&temp_movie, entry_path)? == 0 {
        return Ok(None);
    }
    let temp_wav = work_root.join(format!("{stem}_audio_track_01.wav"));
    let status = Command::new(media_tool_path("ffmpeg"))
        .args(["-y", "-hide_banner", "-loglevel", "error"])
        .arg("-i")
        .arg(&temp_movie)
        .args(["-vn", "-map", "0:a:0", "-acodec", "pcm_s16le"])
        .arg(&temp_wav)
        .status()
        .map_err(|error| {
            optional_movie_tool_error(
                "decode Latino cinematic audio",
                entry_path,
                &error,
            )
        })?;
    if !status.success() {
        return Err(PipelineError::new(format!(
            "ffmpeg failed to decode Latino cinematic audio for {entry_path}"
        )));
    }
    local_read_bytes(&temp_wav).map(Some).map_err(|error| {
        optional_movie_tool_error(
            "read decoded Latino cinematic audio",
            entry_path,
            &error,
        )
    })
}

/// Writes one normalized Latino WAV and its public-safe evidence record.
fn write_lmlm_wav(
    destination: &Path,
    wav: &[u8],
    source: &str,
    kind: &str,
    source_audio_stream_ordinal: Option<usize>,
    extracted_root: &Path,
    records: &mut Vec<String>,
) -> PipelineOutcome<()> {
    if destination.exists() {
        return Err(PipelineError::new(
            "Latino package would overwrite generated output",
        ));
    }
    write_bytes(destination, wav)?;
    records.push(format!(
        concat!(
            "{{\"kind\":\"{}\",",
            "\"source\":\"{}\",",
            "\"output\":\"{}\",",
            "\"status\":\"written\",",
            "\"bytes\":{},",
            "\"sha256\":\"{}\",",
            "\"language\":\"spanish_latam\",",
            "\"source_audio_stream_ordinal\":{}}}"
        ),
        kind,
        json_escape(source),
        json_escape(&relative_output(extracted_root, destination)?),
        wav.len(),
        Sha256::digest(wav).hex(),
        source_audio_stream_ordinal
            .map_or_else(|| "null".to_owned(), |value| value.to_string())
    ));
    Ok(())
}

/// Builds one public-safe optional-movie tool diagnostic.
fn optional_movie_tool_error(
    action: &str,
    public_source: &str,
    error: &std::io::Error,
) -> PipelineError {
    PipelineError::new(format!(
        "{action} for {public_source} failed ({:?})",
        error.kind(),
    ))
}

/// Ffprobe audio stream count.
fn ffprobe_audio_stream_count(
    input: &Path,
    public_source: &str,
) -> PipelineOutcome<usize> {
    let output = Command::new(media_tool_path("ffprobe"))
        .args([
            "-v",
            "error",
            "-select_streams",
            "a",
            "-show_entries",
            "stream=index",
            "-of",
            "csv=p=0",
        ])
        .arg(input)
        .output()
        .map_err(|error| {
            optional_movie_tool_error("run ffprobe", public_source, &error)
        })?;
    if !output.status.success() {
        return Err(PipelineError::new(format!(
            "ffprobe failed for {public_source}"
        )));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count())
}

/// Write bytes.
fn write_bytes(path: &Path, bytes: &[u8]) -> PipelineOutcome<()> {
    local_write_bytes(path, bytes, true).map_err(io_error(path))
}

/// Io error.
fn io_error(path: &Path) -> impl FnOnce(std::io::Error) -> PipelineError + '_ {
    move |error| PipelineError::new(format!("{}: {error}", path.display()))
}

#[cfg(test)]
#[rustfmt::skip]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/one/lmlm_stage/tests.rs"]
mod tests;
