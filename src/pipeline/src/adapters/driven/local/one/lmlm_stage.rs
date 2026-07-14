// File:
//   - lmlm_stage.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/one/lmlm_stage.rs
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
//   - The lmlm stage contract for pipeline phase one.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute lmlm stage.
// - Split-When:
//   - Split when lmlm stage contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Lmlm stage for pipeline phase one.
// - Description:
//   - Defines lmlm stage data and behavior for pipeline phase one.
// - Usage:
//   - Used by pipeline phase one code that needs lmlm stage.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Lmlm stage for pipeline phase one keeps tightly coupled
//   - validation, ordering, and deterministic transformation invariants
//   - together; split when a stable independently testable sub-boundary is
//   - identified.
//

//! Lmlm stage for pipeline phase one.
//!
//! This boundary keeps lmlm stage for pipeline phase one explicit and returns
//! deterministic results to pipeline callers.
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use lmlm::{entry_bytes, parse as parse_lmlm};
use rmv::Sha256;
use rsd::RsdAudio;
use schoenwald_filesystem::adapters::driving::local::{
    create_dir_all as local_create_dir_all, file_len as local_file_len,
    read_bytes as local_read_bytes, write_bytes as local_write_bytes,
};

use super::media_dependencies::{ensure_ffmpeg_dependency, media_tool_path};
use crate::adapters::driven::local::filesystem::collect_files;
use crate::domain::{PipelineError, StageReport, escape_json as json_escape};

/// Result.
type PipelineOutcome<T> = Result<T, PipelineError>;

/// Extract lmlm.
// One transaction validates, deduplicates, and stages the archive output.
#[expect(
    clippy::too_many_lines,
    reason = "LMLM extraction preserves archive validation, deduplication, \
              and staged output ordering in one transaction."
)]
pub(super) fn extract_lmlm(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    let output_root = extracted_root.join("lmlm");
    if output_root.exists() {
        fs::remove_dir_all(&output_root).map_err(io_error(&output_root))?;
    }
    local_create_dir_all(&output_root).map_err(io_error(&output_root))?;
    let mut known_wav_hashes = BTreeSet::new();
    let work_root = std::env::temp_dir()
        .join("shar-schoenwald")
        .join("lmlm-pipeline");
    if work_root.exists() {
        fs::remove_dir_all(&work_root).map_err(io_error(&work_root))?;
    }
    local_create_dir_all(&work_root).map_err(io_error(&work_root))?;

    let mut files_written = 0usize;
    let mut bytes_written = 0u64;
    let mut records = Vec::new();
    for archive in files_with_extension(
        game_root, "lmlm",
    )? {
        let data = local_read_bytes(&archive).map_err(io_error(&archive))?;
        let entries = parse_lmlm(&data).map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "{}: {error}",
                        archive.display()
                    ),
                )
            },
        )?;
        for entry in &entries {
            let lower = entry
                .path
                .to_ascii_lowercase();
            let bytes = entry_bytes(
                &data, entry,
            )
            .ok_or_else(
                || {
                    PipelineError::new(
                        format!(
                            "{}: LMLM entry out of bounds",
                            entry.path
                        ),
                    )
                },
            )?;
            if entry_extension_is(
                &entry.path,
                "ini",
            ) {
                if lower == "customtext/customtext.ini" {
                    let destination = lmlm_entry_path(
                        &output_root,
                        &entry.path,
                    );
                    write_bytes(
                        &destination,
                        bytes,
                    )?;
                    files_written = files_written.saturating_add(1);
                    bytes_written = bytes_written.saturating_add(
                        u64::try_from(bytes.len()).unwrap_or(u64::MAX),
                    );
                    records.push(
                        format!(
                            concat!(
                                "{{\"kind\":\"ini\",",
                                "\"source\":\"{}\",",
                                "\"output\":\"{}\",",
                                "\"bytes\":{}}}",
                            ),
                            json_escape(&entry.path),
                            json_escape(
                                &destination
                                    .display()
                                    .to_string()
                            ),
                            bytes.len()
                        ),
                    );
                }
                continue;
            }
            if entry_extension_is(
                &entry.path,
                "rsd",
            ) {
                let wav = rsd_bytes_to_wav(
                    bytes,
                    &entry.path,
                )?;
                let destination = lmlm_entry_path(
                    &output_root,
                    &entry.path,
                )
                .with_extension("wav");
                write_lmlm_wav(
                    &destination,
                    &wav,
                    &entry.path,
                    "rsd_audio_override",
                    None,
                    &mut records,
                )?;
                files_written = files_written.saturating_add(1);
                bytes_written = bytes_written.saturating_add(
                    u64::try_from(wav.len()).unwrap_or(u64::MAX),
                );
                continue;
            }
            if !lmlm_is_fmv_or_intro(&lower) {
                continue;
            }
            if entry_extension_is(
                &entry.path,
                "rmv",
            ) {
                let (movie_files, movie_bytes) = export_lmlm_movie_audio(
                    &work_root,
                    &output_root,
                    &entry.path,
                    bytes,
                    &mut known_wav_hashes,
                    extracted_root,
                    &mut records,
                )?;
                files_written = files_written.saturating_add(movie_files);
                bytes_written = bytes_written.saturating_add(movie_bytes);
            }
        }
    }
    drop(fs::remove_dir_all(&work_root));
    let manifest = format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.lmlm-extract.v1\",",
            "\"scope\":\"jebano_latino_local_review_overrides\",",
            "\"audio_language_fields_apply_to\":\"audio_records_only\",",
            "\"dedupe_basis\":\"movie_audio_track_override_only_exact_",
            "match_against_non_lmlm_extracted_wavs\",",
            "\"records\":[{}]}}\n",
        ),
        records.join(",")
    );
    let manifest_path = output_root.join("manifest.json");
    write_bytes(
        &manifest_path,
        manifest.as_bytes(),
    )?;
    files_written = files_written.saturating_add(1);
    bytes_written = bytes_written
        .saturating_add(u64::try_from(manifest.len()).unwrap_or(u64::MAX));
    Ok(
        StageReport {
            name: "lmlm",
            files: files_written,
            bytes: bytes_written,
            note: "Jebano Latino LMLM inspected in-process; CustomText INI, \
                   all RSD WAV overrides, and unique Spanish LatAm FMV/intro \
                   movie WAV overrides written under extracted/lmlm; LMLM \
                   video frames are intentionally not exported"
                .to_owned(),
        },
    )
}

/// Lmlm is fmv or intro.
fn lmlm_is_fmv_or_intro(lower_path: &str) -> bool {
    lower_path.contains("fmv") || lower_path.contains("intro")
}

/// Lmlm entry path.
fn entry_extension_is(
    path: &str,
    expected: &str,
) -> bool {
    Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(expected))
}

/// Build a normalized output path for one LMLM entry.
fn lmlm_entry_path(
    root: &Path,
    entry_path: &str,
) -> PathBuf {
    let mut destination = root.to_path_buf();
    for component in entry_path.split('/') {
        destination.push(component);
    }
    destination
}

/// Non lmlm wav hash exists.
fn non_lmlm_wav_hash_exists(
    extracted_root: &Path,
    hash: &str,
    byte_len: usize,
) -> PipelineOutcome<bool> {
    let lmlm_root = extracted_root.join("lmlm");
    for file in collect_files(extracted_root)? {
        if file.starts_with(&lmlm_root) {
            continue;
        }
        let is_wav = file
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case("wav"));
        if !is_wav {
            continue;
        }
        let file_length = local_file_len(&file).map_err(io_error(&file))?;
        if file_length != u64::try_from(byte_len).unwrap_or(u64::MAX) {
            continue;
        }
        let bytes = local_read_bytes(&file).map_err(io_error(&file))?;
        if Sha256::digest(&bytes).hex() == hash {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Rsd bytes to wav.
fn rsd_bytes_to_wav(
    bytes: &[u8],
    source: &str,
) -> PipelineOutcome<Vec<u8>> {
    let audio = RsdAudio::parse(bytes)
        .map_err(|error| PipelineError::new(format!("{source}: {error}")))?;
    let wav = audio
        .to_wav()
        .map_err(|error| PipelineError::new(format!("{source}: {error}")))?;
    wav.to_bytes()
        .map_err(|error| PipelineError::new(format!("{source}: {error}")))
}

/// Export lmlm movie audio.
// Movie audio remains one ordered demux, deduplication, and staging flow.
#[expect(
    clippy::too_many_lines,
    reason = "Movie audio export preserves demux, hash deduplication, and \
              artifact staging order."
)]
fn export_lmlm_movie_audio(
    work_root: &Path,
    output_root: &Path,
    entry_path: &str,
    movie_bytes: &[u8],
    known_wav_hashes: &mut BTreeSet<String>,
    extracted_root: &Path,
    records: &mut Vec<String>,
) -> PipelineOutcome<(
    usize,
    u64,
)> {
    ensure_ffmpeg_dependency().map_err(PipelineError::new)?;
    let stem = Path::new(entry_path)
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("movie");
    let temp_movie = work_root.join(format!("{stem}.rmv"));
    write_bytes(
        &temp_movie,
        movie_bytes,
    )?;
    let audio_count = ffprobe_audio_stream_count(&temp_movie)?;
    if audio_count == 0 {
        records.push(
            format!(
                "{{\"kind\":\"movie_audio\",\"source\":\"{}\",\"status\":\"\
                 skipped_no_audio\"}}",
                json_escape(entry_path)
            ),
        );
        return Ok(
            (
                0, 0,
            ),
        );
    }
    let source_stream_index: usize = if audio_count >= 4 {
        3
    } else {
        0
    };
    let temp_wav = work_root.join(format!("{stem}_audio_track_04.wav"));
    let stream = format!("0:a:{source_stream_index}");
    let status = Command::new(media_tool_path("ffmpeg"))
        .args(
            [
                "-y",
                "-hide_banner",
                "-loglevel",
                "error",
            ],
        )
        .arg("-i")
        .arg(&temp_movie)
        .args(
            [
                "-vn", "-map",
            ],
        )
        .arg(&stream)
        .args(
            [
                "-acodec",
                "pcm_s16le",
            ],
        )
        .arg(&temp_wav)
        .status()
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "failed to run ffmpeg for LMLM movie audio \
                         {entry_path}: {error}"
                    ),
                )
            },
        )?;
    if !status.success() {
        return Err(
            PipelineError::new(
                format!(
                    "ffmpeg failed to export LMLM movie audio for {entry_path}"
                ),
            ),
        );
    }
    let wav = local_read_bytes(&temp_wav).map_err(io_error(&temp_wav))?;
    let destination = output_root
        .join("movies")
        .join(stem)
        .join("audio_track_04.wav");
    let written = write_lmlm_unique_wav(
        &destination,
        &wav,
        known_wav_hashes,
        extracted_root,
        entry_path,
        "movie_audio_track_override",
        Some(source_stream_index.saturating_add(1)),
        records,
    )?;
    if written {
        Ok(
            (
                1,
                u64::try_from(wav.len()).unwrap_or(u64::MAX),
            ),
        )
    } else {
        Ok(
            (
                0, 0,
            ),
        )
    }
}

/// Write lmlm wav.
fn write_lmlm_wav(
    destination: &Path,
    wav: &[u8],
    source: &str,
    kind: &str,
    source_audio_stream_ordinal: Option<usize>,
    records: &mut Vec<String>,
) -> PipelineOutcome<()> {
    write_bytes(
        destination,
        wav,
    )?;
    records.push(
        format!(
            "{{\"kind\":\"{}\",\"source\":\"{}\",\"output\":\"{}\",\"status\":\
             \"written\",\"bytes\":{},\"language\":\"spanish_latam\",\"\
             game_track_number\":4,\"source_audio_stream_ordinal\":{}}}",
            kind,
            json_escape(source),
            json_escape(
                &destination
                    .display()
                    .to_string()
            ),
            wav.len(),
            source_audio_stream_ordinal.map_or_else(
                || "null".to_owned(),
                |value| value.to_string()
            )
        ),
    );
    Ok(())
}

/// Write lmlm unique wav.
// This scoped expectation preserves a documented boundary with explicit
// validation.
#[expect(
    clippy::too_many_arguments,
    reason = "Each argument is a separate provenance boundary for extracted \
              files; grouping would obscure the audit contract."
)]
fn write_lmlm_unique_wav(
    destination: &Path,
    wav: &[u8],
    known_wav_hashes: &mut BTreeSet<String>,
    extracted_root: &Path,
    source: &str,
    kind: &str,
    source_audio_stream_ordinal: Option<usize>,
    records: &mut Vec<String>,
) -> PipelineOutcome<bool> {
    let hash = Sha256::digest(wav).hex();
    let duplicate_in_lmlm = !known_wav_hashes.insert(hash.clone());
    let duplicate_in_extracted = non_lmlm_wav_hash_exists(
        extracted_root,
        &hash,
        wav.len(),
    )?;
    let is_unique = !duplicate_in_lmlm && !duplicate_in_extracted;
    let status = if is_unique {
        "written"
    } else {
        "skipped_duplicate"
    };
    if is_unique {
        write_bytes(
            destination,
            wav,
        )?;
    }
    records.push(
        format!(
            "{{\"kind\":\"{}\",\"source\":\"{}\",\"output\":\"{}\",\"status\":\
             \"{}\",\"bytes\":{},\"language\":\"spanish_latam\",\"\
             game_track_number\":4,\"source_audio_stream_ordinal\":{}}}",
            kind,
            json_escape(source),
            json_escape(
                &destination
                    .display()
                    .to_string()
            ),
            status,
            wav.len(),
            source_audio_stream_ordinal.map_or_else(
                || "null".to_owned(),
                |value| value.to_string()
            )
        ),
    );
    Ok(is_unique)
}

/// Ffprobe audio stream count.
fn ffprobe_audio_stream_count(input: &Path) -> PipelineOutcome<usize> {
    let output = Command::new(media_tool_path("ffprobe"))
        .args(
            [
                "-v",
                "error",
                "-select_streams",
                "a",
                "-show_entries",
                "stream=index",
                "-of",
                "csv=p=0",
            ],
        )
        .arg(input)
        .output()
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "failed to run ffprobe for {}: {error}",
                        input.display()
                    ),
                )
            },
        )?;
    if !output
        .status
        .success()
    {
        return Err(
            PipelineError::new(
                format!(
                    "ffprobe failed for {}",
                    input.display()
                ),
            ),
        );
    }
    Ok(
        String::from_utf8_lossy(&output.stdout)
            .lines()
            .filter(
                |line| {
                    !line
                        .trim()
                        .is_empty()
                },
            )
            .count(),
    )
}

/// Files with extension.
fn files_with_extension(
    root: &Path,
    extension: &str,
) -> PipelineOutcome<Vec<PathBuf>> {
    Ok(
        collect_files(root)?
            .into_iter()
            .filter(
                |path| {
                    path.extension()
                        .and_then(|value| value.to_str())
                        .is_some_and(
                            |value| value.eq_ignore_ascii_case(extension),
                        )
                },
            )
            .collect(),
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
