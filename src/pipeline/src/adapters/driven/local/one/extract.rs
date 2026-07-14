// File:
//   - extract.rs
// Path:
//   - src/pipeline/src/adapters/driven/local/one/extract.rs
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
//   - The extract contract for pipeline phase one.
// - Must-Not:
//   - Violate repository architecture, path, provenance, or output rules.
// - Allows:
//   - Operations required to validate and execute extract.
// - Split-When:
//   - Split when extract contains two independently testable contracts.
// - Merge-When:
//   - Another pipeline module owns the same module boundary with no distinct
//   - invariant.
// - Summary:
//   - Returns an error when validation, filesystem access, or output
//   - writing.
// - Description:
//   - Defines extract data and behavior for pipeline phase one.
// - Usage:
//   - Used by pipeline phase one code that needs extract.
// - Defaults:
//   - No implicit output outside the repository is allowed.
//
// ADRs:
// - docs/adr/pipeline/minor-unit-taxonomy-value-case.md
//
// Large file:
//   - true
//   - Reason: Returns an error when validation, filesystem access, or output
//   - writing keeps tightly coupled validation, ordering, and deterministic
//   - transformation invariants together; split when a stable independently
//   - testable sub-boundary is identified.
//

//! This code defines pipeline config for phase one extract.
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use game_manifest::{DirCount, count_by_dir_ext};
use rcf::Extractor;
use rcf::adapters::{FileArchiveSource, FileEntrySink};
use rcf::domain::{ArchiveError, IndexRecord};
use rcf::ports::ExtractionObserver;
use rmv::{
    FilesystemMovieAuditor, MovieAuditor, MovieKind, MovieRecord,
    UnrealHapPackagePlan,
};
use rsd::RsdAudio;
use schoenwald_filesystem::adapters::driving::local::{
    file_len as local_file_len, write_bytes as local_write_bytes,
};
use schoenwald_filesystem::resolve_under;

use super::super::two::units::manifest_minor_unit as mum;
use super::cleanup::remove_generated_tree;
use super::json_output::validate_generated_text_file;
use super::lmlm_stage::extract_lmlm;
use super::media_dependencies::{ensure_ffmpeg_dependency, media_tool_path};
use super::{rms, spt};
use crate::adapters::driven::local::filesystem::collect_files;
use crate::adapters::driven::local::progress::StageProgress;
use crate::domain::{
    PipelineConfig, PipelineError, PipelineOutcome, PipelineReport,
    StageReport, escape_json as json_escape,
};

/// Number of ordered stages in one complete extraction run.
const FULL_EXTRACTION_STAGE_COUNT: usize = 10;

/// Reports RCF entry progress without exposing archive entry names.
#[derive(Debug)]
struct RcfProgressObserver {
    /// Unknown-total entry progress for one archive.
    progress: StageProgress,
}

impl RcfProgressObserver {
    /// Start entry reporting for one archive ordinal.
    fn begin(archive_ordinal: usize) -> Self {
        Self {
            progress: StageProgress::begin_unknown(
                format!("rcf archive {archive_ordinal} entries"),
            ),
        }
    }

    /// Finish entry reporting after the extractor returns.
    fn finish(self) {
        self.progress
            .finish();
    }
}

impl ExtractionObserver for RcfProgressObserver {
    fn entry_extracted(
        &mut self,
        entry: &IndexRecord,
        _output_path: &Path,
    ) -> Result<(), ArchiveError> {
        self.progress
            .advance(
                &format!(
                    "entry {:08x} ({} bytes)",
                    entry.hash, entry.length,
                ),
            );
        Ok(())
    }
}

/// Extractgameassets.
#[derive(Clone, Copy, Debug, Default)]
pub(in crate::adapters::driven::local) struct ExtractGameAssets;

impl ExtractGameAssets {
    /// Run.
    ///
    /// # Errors
    ///
    /// Returns an error when validation, filesystem access, or output writing
    /// fails.
    // One visible transaction preserves the exact fail-closed stage order.
    #[expect(
        clippy::too_many_lines,
        reason = "The fail-closed extraction stage order remains visible in \
                  one transaction."
    )]
    pub(in crate::adapters::driven::local) fn run(
        config: &PipelineConfig
    ) -> PipelineOutcome<PipelineReport> {
        guard_paths(
            &config.game_root,
            &config.extracted_root,
        )?;
        if config.clean_extracted
            && config
                .extracted_root
                .exists()
        {
            remove_generated_tree(&config.extracted_root).map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to clean {}: {error}",
                            config
                                .extracted_root
                                .display()
                        ),
                    )
                },
            )?;
        }
        fs::create_dir_all(&config.extracted_root)
            .map_err(io_error(&config.extracted_root))?;

        let mut progress = StageProgress::begin(
            "pipeline stages",
            FULL_EXTRACTION_STAGE_COUNT,
        );
        let mut report = PipelineReport::default();

        progress.advance("verify game manifest");
        report
            .stages
            .push(
                verify_manifest(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        progress.advance("convert readme");
        report
            .stages
            .push(
                convert_readme(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        progress.advance("extract rcf archives");
        report
            .stages
            .push(
                extract_rcf(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        progress.advance("convert rsd audio");
        report
            .stages
            .push(
                convert_rsd(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        progress.advance("normalize sound scripts");
        report
            .stages
            .push(normalize_sound_scripts(&config.extracted_root)?);
        progress.advance("export movies");
        report
            .stages
            .push(
                extract_movies(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        progress.advance("extract optional language package");
        report
            .stages
            .push(
                extract_lmlm(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        progress.advance("decode p3d packages");
        report
            .stages
            .push(
                extract_p3d(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        progress.advance("verify normalized outputs");
        report
            .stages
            .push(assert_normalized(&config.extracted_root)?);
        progress.advance("write minor-unit manifest");
        report
            .stages
            .push(
                mum::write_manifest_minor_units(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        write_pipeline_report(
            &config.extracted_root,
            &report,
        )?;
        progress.finish();
        Ok(report)
    }

    /// Export movies only.
    ///
    /// # Errors
    ///
    /// Returns an error when validation, filesystem access, or output writing
    /// fails.
    pub(in crate::adapters::driven::local) fn export_movies_only(
        config: &PipelineConfig
    ) -> PipelineOutcome<PipelineReport> {
        guard_paths(
            &config.game_root,
            &config.extracted_root,
        )?;
        fs::create_dir_all(&config.extracted_root)
            .map_err(io_error(&config.extracted_root))?;
        let mut report = PipelineReport::default();
        report
            .stages
            .push(
                extract_movies(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        Ok(report)
    }

    /// Export lmlm only.
    ///
    /// # Errors
    ///
    /// Returns an error when validation, filesystem access, or output writing
    /// fails.
    pub(in crate::adapters::driven::local) fn export_lmlm_only(
        config: &PipelineConfig
    ) -> PipelineOutcome<PipelineReport> {
        guard_paths(
            &config.game_root,
            &config.extracted_root,
        )?;
        fs::create_dir_all(&config.extracted_root)
            .map_err(io_error(&config.extracted_root))?;
        let mut report = PipelineReport::default();
        report
            .stages
            .push(
                extract_lmlm(
                    &config.game_root,
                    &config.extracted_root,
                )?,
            );
        Ok(report)
    }
}

/// Guard paths.
fn guard_paths(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<()> {
    if !game_root.exists() {
        return Err(
            PipelineError::new(
                format!(
                    "game root does not exist: {}",
                    game_root.display()
                ),
            ),
        );
    }
    if extracted_root == game_root || extracted_root.starts_with(game_root) {
        return Err(
            PipelineError::new(
                "refusing to write inside game/; use the root extracted/ \
                 output directory",
            ),
        );
    }
    Ok(())
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

/// Verify manifest.
fn verify_manifest(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    let manifest = game_root.join("manifest.jsonl");
    let text = fs::read_to_string(&manifest).map_err(io_error(&manifest))?;
    let rules = text
        .lines()
        .skip(1)
        .filter_map(DirCount::parse)
        .collect::<Vec<_>>();
    let counts = count_by_dir_ext(game_root).map_err(io_error(game_root))?;
    let mut failures = 0usize;
    let mut out = String::from(
        "{\"schema\":\"shar-schoenwald.pipeline.manifest-check.v1\"}\n",
    );
    let mut progress = StageProgress::begin(
        "manifest rules",
        rules.len(),
    );
    for rule in &rules {
        progress.advance(
            &format!(
                "{}/.{}",
                rule.dir, rule.extension,
            ),
        );
        let actual = counts
            .get(
                &(
                    rule.dir
                        .clone(),
                    rule.extension
                        .clone(),
                ),
            )
            .copied()
            .unwrap_or_default();
        let ok = actual >= rule.min_count;
        if !ok {
            failures = failures.saturating_add(1);
        }
        let row = format!(
            "{{\"dir\":\"{}\",\"ext\":\"{}\",\"min\":{},\"actual\":{},\"ok\":\
             {}}}\n",
            json_escape(&rule.dir),
            json_escape(&rule.extension),
            rule.min_count,
            actual,
            ok
        );
        out.push_str(&row);
    }
    progress.finish();
    let report_path = extracted_root.join("manifest-check.jsonl");
    fs::write(
        &report_path,
        out,
    )
    .map_err(io_error(&report_path))?;
    if failures > 0 {
        return Err(
            PipelineError::new(
                format!(
                    "manifest verification failed: {failures} rule(s) below \
                     minimum"
                ),
            ),
        );
    }
    Ok(
        StageReport {
            name: "manifest",
            files: rules.len(),
            bytes: fs::metadata(&manifest)
                .map_err(io_error(&manifest))?
                .len(),
            note: "game/manifest.jsonl verified".to_owned(),
        },
    )
}

/// Convert readme.
fn convert_readme(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    let input = game_root.join("README.rtf");
    let bytes = fs::read(&input).map_err(io_error(&input))?;
    let markdown = format!(
        "# Original README\n\nGenerated from `game/README.rtf`.\n\n---\n\n{}",
        rtf::rtf_to_markdown(&bytes)
    );
    let output = extracted_root.join("README.md");
    write_bytes(
        &output,
        markdown.as_bytes(),
    )?;
    Ok(
        StageReport {
            name: "readme",
            files: 1,
            bytes: u64::try_from(markdown.len()).unwrap_or(u64::MAX),
            note: "README.rtf normalized to README.md".to_owned(),
        },
    )
}

/// Extract rcf.
fn extract_rcf(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    if extracted_root
        .join("scripts")
        .exists()
        && (extracted_root
            .join("dialog")
            .exists()
            || extracted_root
                .join("art")
                .exists())
    {
        return Ok(
            StageReport {
                name: "rcf",
                files: collect_files(extracted_root)?.len(),
                bytes: sum_file_lengths(extracted_root)?,
                note: "resume: existing RCF-expanded tree reused".to_owned(),
            },
        );
    }
    let archives = files_with_extension(
        game_root, "rcf",
    )?;
    let mut progress = StageProgress::begin(
        "rcf archives",
        archives.len(),
    );
    let mut files = 0usize;
    let mut bytes = 0u64;
    for (index, archive) in archives
        .into_iter()
        .enumerate()
    {
        let archive_ordinal = index.saturating_add(1);
        progress.advance(&format!("archive {archive_ordinal}"));
        let source = FileArchiveSource::new(&archive);
        let mut sink = FileEntrySink::new(extracted_root);
        let mut observer = RcfProgressObserver::begin(archive_ordinal);
        let report = Extractor::extract(
            &source,
            &mut sink,
            &mut observer,
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "{}: {error}",
                        archive.display()
                    ),
                )
            },
        )?;
        observer.finish();
        files = files.saturating_add(report.entry_count);
        bytes = bytes.saturating_add(report.extracted_bytes);
    }
    progress.finish();
    Ok(
        StageReport {
            name: "rcf",
            files,
            bytes,
            note: "RCF archives expanded into extracted/<archive-stem>/..."
                .to_owned(),
        },
    )
}

/// Convert rsd.
fn convert_rsd(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    let game_inputs = files_with_extension(
        game_root, "rsd",
    )?;
    let extracted_inputs = files_with_extension(
        extracted_root,
        "rsd",
    )?;
    let mut progress = StageProgress::begin(
        "rsd audio",
        game_inputs
            .len()
            .saturating_add(extracted_inputs.len()),
    );
    let mut files = 0usize;
    let mut bytes = 0u64;
    for input in game_inputs {
        progress.advance(
            &progress_item(
                game_root, &input,
            ),
        );
        let relative = input
            .strip_prefix(game_root)
            .map_err(
                |_error| PipelineError::new("failed to relativize game RSD"),
            )?;
        let output = extracted_root
            .join(relative)
            .with_extension("wav");
        let written = convert_one_rsd(
            &input, &output,
        )?;
        files = files.saturating_add(1);
        bytes = bytes.saturating_add(written);
    }
    for input in extracted_inputs {
        progress.advance(
            &progress_item(
                extracted_root,
                &input,
            ),
        );
        if !input.exists() {
            continue;
        }
        let output = input.with_extension("wav");
        if !input.is_file() && output.is_file() {
            continue;
        }
        let written = convert_one_rsd(
            &input, &output,
        )?;
        if input.is_file() {
            fs::remove_file(&input).map_err(io_error(&input))?;
        }
        files = files.saturating_add(1);
        bytes = bytes.saturating_add(written);
    }
    progress.finish();
    Ok(
        StageReport {
            name: "rsd",
            files,
            bytes,
            note: concat!(
                "RSD normalized to WAV in-place; ",
                "extracted .rsd sources removed"
            )
            .to_owned(),
        },
    )
}

/// Convert one rsd.
fn convert_one_rsd(
    input: &Path,
    output: &Path,
) -> PipelineOutcome<u64> {
    let bytes = match fs::read(input) {
        Ok(bytes) => bytes,
        Err(error)
            if error.kind() == std::io::ErrorKind::NotFound
                && output.is_file() =>
        {
            return fs::metadata(output)
                .map(|metadata| metadata.len())
                .map_err(io_error(output));
        }
        Err(error) => return Err(io_error(input)(error)),
    };
    let audio = RsdAudio::parse(&bytes).map_err(
        |error| {
            PipelineError::new(
                format!(
                    "{}: {error}",
                    input.display()
                ),
            )
        },
    )?;
    let wav = audio
        .to_wav()
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "{}: {error}",
                        input.display()
                    ),
                )
            },
        )?;
    let wav_bytes = wav
        .to_bytes()
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "{}: {error}",
                        input.display()
                    ),
                )
            },
        )?;
    write_bytes(
        output, &wav_bytes,
    )?;
    Ok(u64::try_from(wav_bytes.len()).unwrap_or(u64::MAX))
}

/// Is movie decodable by ffmpeg.
const fn is_movie_decodable_by_ffmpeg(kind: MovieKind) -> bool {
    matches!(
        kind,
        MovieKind::BinkV1 | MovieKind::BinkV2
    )
}

/// Require ffmpeg.
fn require_ffmpeg() -> PipelineOutcome<()> {
    ensure_ffmpeg_dependency().map_err(PipelineError::new)
}

/// Exports one decoded movie package through `FFmpeg` and validates outputs.
// The complete media transaction must remain ordered and fail closed.
#[expect(
    clippy::too_many_lines,
    reason = "Movie export is one fail-closed decode and publication \
              transaction."
)]
fn export_movie_with_ffmpeg(
    record: &MovieRecord,
    plan: &UnrealHapPackagePlan,
) -> PipelineOutcome<()> {
    let legacy_frames = plan
        .movie_directory
        .join("frames");
    if legacy_frames.exists() {
        fs::remove_dir_all(&legacy_frames).map_err(io_error(&legacy_frames))?;
    }
    for track in 1_i32..=32_i32 {
        let path = plan
            .movie_directory
            .join(format!("audio_track_{track:02}.wav"));
        if path.exists() {
            fs::remove_file(&path).map_err(io_error(&path))?;
        }
    }
    if plan
        .hap_video_path
        .exists()
    {
        fs::remove_file(&plan.hap_video_path)
            .map_err(io_error(&plan.hap_video_path))?;
    }
    let source_probe = ffprobe_video_stream_json(&record.source_path)?;
    write_bytes(
        &plan
            .movie_directory
            .join("source-video.ffprobe.json"),
        source_probe.as_bytes(),
    )?;
    let video_status = Command::new(media_tool_path("ffmpeg"))
        .args(
            [
                "-y",
                "-hide_banner",
                "-loglevel",
                "error",
            ],
        )
        .arg("-i")
        .arg(&record.source_path)
        .args(
            [
                "-map",
                "0:v:0",
                "-an",
                "-c:v",
                "hap",
                "-format",
                plan.hap_format,
            ],
        )
        .arg(&plan.hap_video_path)
        .status()
        .map_err(
            |error| {
                PipelineError::new(
                    format!(
                        "failed to run ffmpeg HAP export for {}: {error}",
                        record
                            .source_path
                            .display()
                    ),
                )
            },
        )?;
    if !video_status.success() {
        return Err(
            PipelineError::new(
                format!(
                    "ffmpeg failed to export HAP video from {}",
                    record
                        .source_path
                        .display()
                ),
            ),
        );
    }
    let audio_tracks = ffprobe_audio_stream_count(&record.source_path)?;
    if audio_tracks == 0 {
        return Err(
            PipelineError::new(
                format!(
                    "ffprobe found no audio streams in {}",
                    record
                        .source_path
                        .display()
                ),
            ),
        );
    }
    for index in 0..audio_tracks {
        let output_wav = plan
            .movie_directory
            .join(
                format!(
                    "audio_track_{:02}.wav",
                    index.saturating_add(1)
                ),
            );
        let stream = format!("0:a:{index}");
        let track_status = Command::new(media_tool_path("ffmpeg"))
            .args(
                [
                    "-y",
                    "-hide_banner",
                    "-loglevel",
                    "error",
                ],
            )
            .arg("-i")
            .arg(&record.source_path)
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
            .arg(&output_wav)
            .status()
            .map_err(
                |error| {
                    PipelineError::new(
                        format!(
                            "failed to run ffmpeg audio export for {}: {error}",
                            record
                                .source_path
                                .display()
                        ),
                    )
                },
            )?;
        if !track_status.success() {
            return Err(
                PipelineError::new(
                    format!(
                        "ffmpeg failed to export audio stream {} from {}",
                        index,
                        record
                            .source_path
                            .display()
                    ),
                ),
            );
        }
    }
    let (_fps, frame_rate_fraction) = ffprobe_video_fps(&record.source_path)?;
    let report = format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.rmv-hap-export.v1\",",
            "\"video_codec\":\"hap\",",
            "\"hap_format\":\"{}\",",
            "\"video_path\":\"{}\",",
            "\"audio_tracks\":{},",
            "\"source_probe\":\"{}\",",
            "\"decoder\":\"ffmpeg\"}}
"
        ),
        plan.hap_format,
        json_escape(
            &plan
                .hap_video_path
                .display()
                .to_string()
        ),
        audio_tracks,
        "source-video.ffprobe.json"
    );
    write_bytes(
        &plan
            .movie_directory
            .join("decode-report.json"),
        report.as_bytes(),
    )?;
    write_movie_timing(
        &plan.movie_directory,
        &frame_rate_fraction,
    )?;
    Ok(())
}

/// Returns whether all required runtime movie outputs are non-empty.
fn movie_outputs_complete(plan: &UnrealHapPackagePlan) -> bool {
    let required = [
        &plan.hap_video_path,
        &plan.manifest_path,
        &plan.source_probe_path,
        &plan.decode_report_path,
        &plan.timing_manifest_path,
    ];
    required
        .iter()
        .all(|path| local_file_len(path).is_ok_and(|length| length > 0))
        && local_file_len(
            &plan
                .movie_directory
                .join("audio_track_01.wav"),
        )
        .is_ok_and(|length| length > 0)
}

/// Ffprobe video stream json.
fn ffprobe_video_stream_json(input: &Path) -> PipelineOutcome<String> {
    let output = Command::new(media_tool_path("ffprobe"))
        .args(
            [
                "-v",
                "error",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=codec_name,codec_type,width,height,pix_fmt,\
                 r_frame_rate,avg_frame_rate,bit_rate,nb_frames,duration",
                "-show_entries",
                "format=format_name,duration,size,bit_rate",
                "-of",
                "json",
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
    String::from_utf8(output.stdout).map_err(
        |error| {
            PipelineError::new(
                format!(
                    "ffprobe returned non-UTF-8 video metadata for {}: {error}",
                    input.display()
                ),
            )
        },
    )
}

/// Ffprobe video fps.
fn ffprobe_video_fps(
    input: &Path
) -> PipelineOutcome<(
    f64,
    String,
)> {
    let output = Command::new(media_tool_path("ffprobe"))
        .args(
            [
                "-v",
                "error",
                "-select_streams",
                "v:0",
                "-show_entries",
                "stream=r_frame_rate",
                "-of",
                "default=nokey=1:noprint_wrappers=1",
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
    let fraction = String::from_utf8_lossy(&output.stdout)
        .lines()
        .next()
        .unwrap_or("0/1")
        .trim()
        .to_owned();
    let fps = parse_frame_rate_fraction(&fraction);
    Ok(
        (
            fps, fraction,
        ),
    )
}

/// Parse frame rate fraction.
fn parse_frame_rate_fraction(value: &str) -> f64 {
    let Some((numerator_text, denominator_text)) = value.split_once('/') else {
        return value
            .parse::<f64>()
            .unwrap_or_default();
    };
    let numerator = numerator_text
        .parse::<f64>()
        .unwrap_or_default();
    let denominator = denominator_text
        .parse::<f64>()
        .unwrap_or(1.0_f64);
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
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

/// Write movie timing.
fn write_movie_timing(
    output: &Path,
    frame_rate_fraction: &str,
) -> PipelineOutcome<()> {
    let timing = format!(
        concat!(
            "schema	shar-schoenwald.rmv-timing.v2
",
            "status	hap-video-cadence
",
            "video	movie.mov
",
            "frame_rate_fraction	{}
"
        ),
        frame_rate_fraction,
    );
    write_bytes(
        &output.join("timing.tsv"),
        timing.as_bytes(),
    )
}

/// Extract movies.
fn extract_movies(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    let roots = vec![game_root.to_path_buf()];
    let auditor = FilesystemMovieAuditor;
    let report = auditor
        .audit_roots(
            &roots,
            extracted_root,
        )
        .map_err(
            |error| PipelineError::new(format!("movie audit failed: {error}")),
        )?;
    let selected = select_logical_movies(
        game_root,
        &report.records,
    )?;
    let stale_user_root = extracted_root
        .join("movies")
        .join("user");
    if stale_user_root.exists() {
        fs::remove_dir_all(&stale_user_root)
            .map_err(io_error(&stale_user_root))?;
    }
    if !selected.is_empty() {
        require_ffmpeg()?;
    }
    let mut progress = StageProgress::begin(
        "movies",
        selected.len(),
    );
    let mut files = 0usize;
    let mut bytes = 0u64;
    for (logical_relative, record) in selected {
        progress.advance(&logical_relative.to_string_lossy());
        let movie_stem = movie_package_stem(&logical_relative)?;
        let plan = UnrealHapPackagePlan::for_movie(
            &extracted_root.join("movies"),
            &movie_stem,
        )
        .map_err(
            |error| {
                PipelineError::new(
                    format!("movie plan failed for {movie_stem}: {error}"),
                )
            },
        )?;
        write_movie_package_plan(
            game_root,
            record,
            &logical_relative,
            &plan,
        )?;
        if is_movie_decodable_by_ffmpeg(record.kind) {
            export_movie_with_ffmpeg(
                record, &plan,
            )?;
            if !movie_outputs_complete(&plan) {
                return Err(
                    PipelineError::new(
                        format!(
                            "movie export produced incomplete outputs for \
                             {movie_stem}"
                        ),
                    ),
                );
            }
        }
        files = files.saturating_add(1);
        bytes = bytes.saturating_add(record.bytes);
    }
    progress.finish();
    Ok(
        StageReport {
            name: "movies",
            files,
            bytes,
            note: "movie packages written through rmv::UnrealHapPackagePlan; \
                   Bink/RMV movies exported through ffmpeg to HAP video and \
                   WAV tracks; movies/user inputs supersede placeholders"
                .to_owned(),
        },
    )
}

/// Writes one deterministic Unreal HAP package plan and its evidence files.
// One serializer owns canonical field order and obsolete-artifact cleanup.
#[expect(
    clippy::too_many_lines,
    reason = "The plan writer preserves one canonical manifest field order \
              and               cleans obsolete package artifacts atomically."
)]
fn write_movie_package_plan(
    game_root: &Path,
    record: &MovieRecord,
    logical_relative: &Path,
    plan: &UnrealHapPackagePlan,
) -> PipelineOutcome<()> {
    fs::create_dir_all(&plan.movie_directory)
        .map_err(io_error(&plan.movie_directory))?;
    let legacy_metadata = plan
        .movie_directory
        .join("metadata.json");
    if legacy_metadata.exists() {
        fs::remove_file(&legacy_metadata)
            .map_err(io_error(&legacy_metadata))?;
    }
    let legacy_audio_dir = plan
        .movie_directory
        .join("audio");
    if legacy_audio_dir.exists() {
        fs::remove_dir_all(&legacy_audio_dir)
            .map_err(io_error(&legacy_audio_dir))?;
    }
    let legacy_frames = plan
        .movie_directory
        .join("frames");
    if legacy_frames.exists() {
        fs::remove_dir_all(&legacy_frames).map_err(io_error(&legacy_frames))?;
    }
    let legacy_mezzanine = plan
        .movie_directory
        .join("mezzanine.mov");
    if legacy_mezzanine.exists() {
        fs::remove_file(&legacy_mezzanine)
            .map_err(io_error(&legacy_mezzanine))?;
    }
    let (fps, frame_rate_fraction) = ffprobe_video_fps(&record.source_path)?;
    let manifest = format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.rmv-unreal-hap-package.v1\",",
            "\"logical_path\":\"{}\",",
            "\"selected_source\":\"{}\",",
            "\"kind\":\"{}\",",
            "\"fps\":{},",
            "\"frame_rate_fraction\":\"{}\",",
            "\"bytes\":{},",
            "\"sha256\":\"{}\",",
            "\"provenance\":\"{}\",",
            "\"target\":\"{}\",",
            "\"transcode_policy\":\"preserve source resolution and cadence; \
             encode HAP Q for runtime media\",",
            "\"supersede_rule\":\"movies/user/<name>.rmv supersedes \
             movies/<name>.rmv\",",
            "\"movie_directory\":\"{}\",",
            "\"hap_video_path\":\"{}\",",
            "\"video_extension\":\"{}\",",
            "\"video_codec\":\"hap\",",
            "\"hap_format\":\"{}\",",
            "\"audio_track_pattern\":\"{}\",",
            "\"timing_manifest_path\":\"{}\",",
            "\"source_probe_path\":\"{}\",",
            "\"optional_bk2_path\":\"{}\"}}
"
        ),
        json_escape(&logical_relative.to_string_lossy()),
        json_escape(
            &relative_manifest_path_text(
                game_root,
                game_root,
                &record.source_path
            )?
        ),
        record
            .kind
            .label(),
        fps,
        json_escape(&frame_rate_fraction),
        record.bytes,
        record
            .hash
            .hex(),
        json_escape(
            &record
                .provenance
                .summary()
        ),
        plan.target
            .label(),
        json_escape(
            &plan
                .movie_directory
                .to_string_lossy()
        ),
        json_escape(
            &plan
                .hap_video_path
                .to_string_lossy()
        ),
        plan.video_extension,
        plan.hap_format,
        json_escape(
            &plan
                .audio_track_pattern
                .to_string_lossy()
        ),
        json_escape(
            &plan
                .timing_manifest_path
                .to_string_lossy()
        ),
        "source-video.ffprobe.json",
        json_escape(
            &plan
                .optional_bk2_path
                .to_string_lossy()
        )
    );
    write_bytes(
        &plan.manifest_path,
        manifest.as_bytes(),
    )?;
    let timing = format!(
        concat!(
            "schema	shar-schoenwald.rmv-timing.v2
",
            "status	pending-hap-export
",
            "source_sha256	{}
",
            "video	movie.mov
",
            "frame_rate_fraction	{}
"
        ),
        record
            .hash
            .hex(),
        frame_rate_fraction,
    );
    write_bytes(
        &plan.timing_manifest_path,
        timing.as_bytes(),
    )?;
    Ok(())
}

/// Movie package stem.
fn movie_package_stem(logical_relative: &Path) -> PipelineOutcome<String> {
    logical_relative
        .file_stem()
        .and_then(|value| value.to_str())
        .map(ToOwned::to_owned)
        .ok_or_else(
            || {
                PipelineError::new(
                    format!(
                        "invalid movie path: {}",
                        logical_relative.display()
                    ),
                )
            },
        )
}

/// Select logical movies.
fn select_logical_movies<'a>(
    game_root: &Path,
    records: &'a [MovieRecord],
) -> PipelineOutcome<BTreeMap<PathBuf, &'a MovieRecord>> {
    let mut selected = BTreeMap::<PathBuf, &'a MovieRecord>::new();
    for record in records {
        let logical = logical_movie_relative(
            game_root,
            &record.source_path,
        )?;
        let replace = selected
            .get(&logical)
            .is_none_or(
                |existing| movie_priority(record) > movie_priority(existing),
            );
        if replace {
            let _previous_record = selected.insert(
                logical, record,
            );
        }
    }
    Ok(selected)
}

/// Logical movie relative.
fn logical_movie_relative(
    game_root: &Path,
    input: &Path,
) -> PipelineOutcome<PathBuf> {
    let relative = input
        .strip_prefix(game_root)
        .map_err(|_error| PipelineError::new("failed to relativize movie"))?;
    let parts = relative
        .components()
        .collect::<Vec<_>>();
    if let [
        movies,
        user,
        name,
    ] = parts.as_slice()
        && movies
            .as_os_str()
            .to_str()
            == Some("movies")
        && user
            .as_os_str()
            .to_str()
            == Some("user")
    {
        return Ok(Path::new("movies").join(name.as_os_str()));
    }
    Ok(relative.to_path_buf())
}

/// Movie priority.
fn movie_priority(
    record: &MovieRecord
) -> (
    u8,
    u64,
) {
    let user_supersede = record
        .relative_path
        .components()
        .any(
            |component| {
                component
                    .as_os_str()
                    .to_str()
                    == Some("user")
            },
        );
    let kind_score = match record.kind {
        MovieKind::BinkV1
        | MovieKind::BinkV2
        | MovieKind::XboxXmvLike
        | MovieKind::RadicalMovieHeader => 2,
        MovieKind::OggNamedRmv => 0,
        MovieKind::Unknown => 1,
    };
    (
        u8::from(user_supersede)
            .saturating_mul(4)
            .saturating_add(kind_score),
        record.bytes,
    )
}

/// Extract p3d.
fn extract_p3d(
    game_root: &Path,
    extracted_root: &Path,
) -> PipelineOutcome<StageReport> {
    let game_inputs = files_with_extension(
        game_root, "p3d",
    )?;
    let extracted_inputs = files_with_extension(
        extracted_root,
        "p3d",
    )?;
    let mut progress = StageProgress::begin(
        "p3d packages",
        game_inputs
            .len()
            .saturating_add(extracted_inputs.len()),
    );
    let mut files = 0usize;
    let mut bytes = 0u64;
    for input in game_inputs {
        progress.advance(
            &progress_item(
                game_root, &input,
            ),
        );
        let relative = input
            .strip_prefix(game_root)
            .map_err(
                |_error| PipelineError::new("failed to relativize game P3D"),
            )?;
        let output = extracted_root.join(path_without_extension(relative));
        if p3d_package_complete(&output) {
            files = files.saturating_add(1);
            let input_bytes = fs::metadata(&input)
                .map_err(io_error(&input))?
                .len();
            bytes = bytes.saturating_add(input_bytes);
            continue;
        }
        normalize_p3d_file(
            &input, &output,
        )?;
        files = files.saturating_add(1);
        let input_bytes = fs::metadata(&input)
            .map_err(io_error(&input))?
            .len();
        bytes = bytes.saturating_add(input_bytes);
    }
    for input in extracted_inputs {
        progress.advance(
            &progress_item(
                extracted_root,
                &input,
            ),
        );
        if !input.exists() {
            continue;
        }
        if input
            .file_name()
            .and_then(|value| value.to_str())
            == Some("source.p3d")
        {
            // Fully decoded component outputs are the extraction artifact; a
            // copied source package would leave undecompiled bytes behind.
            fs::remove_file(&input).map_err(io_error(&input))?;
            continue;
        }
        let output = input.with_extension("");
        normalize_p3d_file(
            &input, &output,
        )?;
        fs::remove_file(&input).map_err(io_error(&input))?;
        files = files.saturating_add(1);
    }
    progress.finish();
    Ok(
        StageReport {
            name: "p3d",
            files,
            bytes,
            note: concat!(
                "P3D/P3DZ normalized to component directories ",
                "at their original relative paths"
            )
            .to_owned(),
        },
    )
}

/// P3d package complete.
fn p3d_package_complete(output: &Path) -> bool {
    let manifest = output.join("components.jsonl");
    let components = output.join("components");
    manifest.exists()
        && components.is_dir()
        && !has_raw_component_output(&components)
        && !has_incomplete_component_json(&components)
        && component_ledger_files_exist(output)
}

/// Has incomplete component JSON.
fn has_incomplete_component_json(components: &Path) -> bool {
    let Ok(files) = collect_files(components) else {
        return true;
    };
    files
        .iter()
        .filter(
            |path| {
                path.extension()
                    .and_then(|value| value.to_str())
                    .is_some_and(
                        |extension| extension.eq_ignore_ascii_case("json"),
                    )
            },
        )
        .any(
            |path| {
                validate_generated_text_file(path).is_err()
                    || fs::read_to_string(path).map_or(
                        true,
                        |text| {
                            text.contains("\"leading_u32\"")
                                || text.contains("\"other_children\"")
                                || text.contains("\"extra_chunks\"")
                                || text.contains("\"metadata\"")
                        },
                    )
            },
        )
}

/// Component ledger files exist.
fn component_ledger_files_exist(output: &Path) -> bool {
    let manifest = output.join("components.jsonl");
    let components = output.join("components");
    if validate_generated_text_file(&manifest).is_err() {
        return false;
    }
    let Ok(text) = fs::read_to_string(&manifest) else {
        return false;
    };
    let mut lines = text.lines();
    let Some(header) = lines.next() else {
        return false;
    };
    if extract_json_string_field(
        header, "schema",
    )
    .as_deref()
        != Some("p3d.package.v1")
    {
        return false;
    }
    let component_lines = lines.collect::<Vec<_>>();
    let Some(component_count) = extract_json_usize_field(
        header,
        "component_count",
    ) else {
        return false;
    };
    if component_count != component_lines.len() || component_lines.is_empty() {
        return false;
    }
    let mut identities = BTreeSet::new();
    for line in component_lines {
        let Some(path_text) = extract_json_string_field(
            line, "path",
        ) else {
            return false;
        };
        if path_text.is_empty() || path_text.contains(char::from(92)) {
            return false;
        }
        let relative = Path::new(&path_text);
        let Ok(resolved) = resolve_under(
            &components,
            relative,
        ) else {
            return false;
        };
        let identity = relative
            .components()
            .filter_map(
                |component| match component {
                    Component::Normal(value) => Some(
                        value
                            .to_string_lossy()
                            .to_ascii_lowercase(),
                    ),
                    _ => None,
                },
            )
            .collect::<Vec<_>>()
            .join("/");
        if identity.is_empty() || !identities.insert(identity) {
            return false;
        }
        if !resolved.is_file() {
            return false;
        }
    }
    true
}

/// Extract one required nonnegative top-level integer field.
fn extract_json_usize_field(
    line: &str,
    key: &str,
) -> Option<usize> {
    let value = find_unique_top_level_json_value(
        line, key,
    )?
    .trim();
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_digit())
    {
        return None;
    }
    value
        .parse()
        .ok()
}

/// Extract one unique top-level JSON string field.
fn extract_json_string_field(
    line: &str,
    key: &str,
) -> Option<String> {
    let value = find_unique_top_level_json_value(
        line, key,
    )?;
    let (decoded, cursor) = parse_component_json_string(
        value, 0,
    )?;
    (cursor == value.len()).then_some(decoded)
}

/// Locate one unique top-level JSON value without searching nested objects.
fn find_unique_top_level_json_value<'a>(
    line: &'a str,
    key: &str,
) -> Option<&'a str> {
    let trimmed_line = line.trim();
    let bytes = trimmed_line.as_bytes();
    if bytes.first() != Some(&b'{') || bytes.last() != Some(&b'}') {
        return None;
    }
    let mut cursor = 1usize;
    let mut found = None;
    loop {
        cursor = skip_json_whitespace(
            trimmed_line,
            cursor,
        );
        if bytes.get(cursor) == Some(&b'}') {
            return (cursor
                == bytes
                    .len()
                    .saturating_sub(1))
            .then_some(found)?;
        }
        let (field, next) = parse_component_json_string(
            trimmed_line,
            cursor,
        )?;
        cursor = skip_json_whitespace(
            trimmed_line,
            next,
        );
        if bytes.get(cursor) != Some(&b':') {
            return None;
        }
        let value_start = skip_json_whitespace(
            trimmed_line,
            cursor.saturating_add(1),
        );
        let value_end = skip_component_json_value(
            trimmed_line,
            value_start,
        )?;
        if field == key {
            if found.is_some() {
                return None;
            }
            found = trimmed_line.get(value_start..value_end);
        }
        cursor = skip_json_whitespace(
            line, value_end,
        );
        match bytes.get(cursor) {
            Some(b',') => {
                cursor = skip_json_whitespace(
                    trimmed_line,
                    cursor.saturating_add(1),
                );
                if bytes.get(cursor) == Some(&b'}') {
                    return None;
                }
            }
            Some(b'}')
                if cursor
                    == bytes
                        .len()
                        .saturating_sub(1) =>
            {
                return found;
            }
            _ => return None,
        }
    }
}

/// Decode one JSON string and return its next byte cursor.
fn parse_component_json_string(
    line: &str,
    start: usize,
) -> Option<(
    String,
    usize,
)> {
    let bytes = line.as_bytes();
    if bytes.get(start) != Some(&b'"') {
        return None;
    }
    let mut cursor = start.saturating_add(1);
    let mut output = String::new();
    while let Some(byte) = bytes
        .get(cursor)
        .copied()
    {
        match byte {
            b'"' => {
                return Some(
                    (
                        output,
                        cursor.saturating_add(1),
                    ),
                );
            }
            b'\\' => {
                cursor = cursor.saturating_add(1);
                let escaped = bytes
                    .get(cursor)
                    .copied()?;
                match escaped {
                    b'"' => output.push('"'),
                    b'\\' => output.push(char::from(92)),
                    b'/' => output.push('/'),
                    b'b' => output.push(char::from(8)),
                    b'f' => output.push(char::from(12)),
                    b'n' => output.push('\n'),
                    b'r' => output.push('\r'),
                    b't' => output.push('\t'),
                    _ => return None,
                }
            }
            control if control <= 0x1f => return None,
            _ if byte.is_ascii() => output.push(char::from(byte)),
            _ => {
                let tail = line.get(cursor..)?;
                let character = tail
                    .chars()
                    .next()?;
                output.push(character);
                cursor = cursor.saturating_add(character.len_utf8());
                continue;
            }
        }
        cursor = cursor.saturating_add(1);
    }
    None
}

/// Skip one unrelated JSON value without searching inside it.
fn skip_component_json_value(
    line: &str,
    start: usize,
) -> Option<usize> {
    if line
        .as_bytes()
        .get(start)
        == Some(&b'"')
    {
        return parse_component_json_string(
            line, start,
        )
        .map(|(_, next)| next);
    }
    let bytes = line.as_bytes();
    let mut cursor = start;
    let container_value = matches!(
        bytes.get(start),
        Some(b'{' | b'[')
    );
    let mut object_depth = 0usize;
    let mut array_depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    while let Some(byte) = bytes
        .get(cursor)
        .copied()
    {
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            cursor = cursor.saturating_add(1);
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' => object_depth = object_depth.saturating_add(1),
            b'}' if object_depth > 0 => {
                object_depth = object_depth.saturating_sub(1);
            }
            b'[' => array_depth = array_depth.saturating_add(1),
            b']' if array_depth > 0 => {
                array_depth = array_depth.saturating_sub(1);
            }
            b',' | b'}' if object_depth == 0 && array_depth == 0 => {
                if container_value {
                    return Some(cursor);
                }
                let value = line
                    .get(start..cursor)?
                    .trim();
                return json_primitive_is_valid(value).then_some(cursor);
            }
            _ => {}
        }
        cursor = cursor.saturating_add(1);
    }
    None
}

/// Return whether one scalar token is valid JSON.
fn json_primitive_is_valid(value: &str) -> bool {
    matches!(
        value,
        "true" | "false" | "null"
    ) || json_number_is_valid(value)
}

/// Validate one complete JSON number without accepting prefix variants.
fn json_number_is_valid(value: &str) -> bool {
    let bytes = value.as_bytes();
    let mut cursor = usize::from(bytes.first() == Some(&b'-'));
    match bytes.get(cursor) {
        Some(b'0') => {
            cursor = cursor.saturating_add(1);
            if matches!(
                bytes.get(cursor),
                Some(b'0'..=b'9')
            ) {
                return false;
            }
        }
        Some(b'1'..=b'9') => {
            cursor = cursor.saturating_add(1);
            while matches!(
                bytes.get(cursor),
                Some(b'0'..=b'9')
            ) {
                cursor = cursor.saturating_add(1);
            }
        }
        _ => return false,
    }
    if bytes.get(cursor) == Some(&b'.') {
        cursor = cursor.saturating_add(1);
        let start = cursor;
        while matches!(
            bytes.get(cursor),
            Some(b'0'..=b'9')
        ) {
            cursor = cursor.saturating_add(1);
        }
        if cursor == start {
            return false;
        }
    }
    if matches!(
        bytes.get(cursor),
        Some(b'e' | b'E')
    ) {
        cursor = cursor.saturating_add(1);
        if matches!(
            bytes.get(cursor),
            Some(b'+' | b'-')
        ) {
            cursor = cursor.saturating_add(1);
        }
        let start = cursor;
        while matches!(
            bytes.get(cursor),
            Some(b'0'..=b'9')
        ) {
            cursor = cursor.saturating_add(1);
        }
        if cursor == start {
            return false;
        }
    }
    cursor == bytes.len()
}

/// Advance over JSON whitespace bytes.
fn skip_json_whitespace(
    line: &str,
    mut cursor: usize,
) -> usize {
    while matches!(
        line.as_bytes()
            .get(cursor),
        Some(b' ' | b'\n' | b'\r' | b'\t')
    ) {
        cursor = cursor.saturating_add(1);
    }
    cursor
}

/// Has raw component output.
fn has_raw_component_output(components: &Path) -> bool {
    let Ok(files) = collect_files(components) else {
        return true;
    };
    files
        .iter()
        .any(|path| is_raw_component_path(path))
}

/// Is raw component path.
fn is_raw_component_path(path: &Path) -> bool {
    let has_raw_segment = path
        .components()
        .any(
            |component| {
                component
                    .as_os_str()
                    .to_str()
                    .is_some_and(|value| value.eq_ignore_ascii_case("raw"))
            },
        );
    let is_bin = path
        .extension()
        .and_then(|value| value.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case("bin"));
    has_raw_segment || is_bin
}

/// Normalize p3d file.
fn normalize_p3d_file(
    input: &Path,
    output: &Path,
) -> PipelineOutcome<()> {
    if is_p3d_diff_path(input) {
        // Metadata-only preservation is forbidden for package payloads because
        // it proves the source exists but loses the bytes needed to rebuild it.
        return Err(
            PipelineError::new(
                "P3D diff payloads require a real decoder before extraction",
            ),
        );
    }
    p3d::write_lossless_package(
        input, output,
    )
    .map_err(
        |error| {
            PipelineError::new(
                format!(
                    "{}: {error}",
                    input.display()
                ),
            )
        },
    )?;
    let source_copy = output.join("source.p3d");
    if source_copy.exists() {
        // Keeping the source package would make extraction look complete while
        // still carrying undecompiled bytes; decoded components are the output.
        fs::remove_file(&source_copy).map_err(io_error(&source_copy))?;
    }
    Ok(())
}

/// Is p3d diff path.
fn is_p3d_diff_path(path: &Path) -> bool {
    path.components()
        .any(
            |component| {
                component
                    .as_os_str()
                    .to_str()
                    .is_some_and(
                        |value| value.eq_ignore_ascii_case("P3D_Diffs"),
                    )
            },
        )
}

/// Assert normalized.
fn assert_normalized(extracted_root: &Path) -> PipelineOutcome<StageReport> {
    let forbidden = [
        "p3d", "rsd", "rcf", "rtf", "rmv", "bik", "bk2", "spt", "rms",
    ];
    let extracted_files = collect_files(extracted_root)?;
    let mut progress = StageProgress::begin(
        "normalized output audit",
        extracted_files.len(),
    );
    let mut bad = Vec::new();
    for file in &extracted_files {
        progress.advance(
            &progress_item(
                extracted_root,
                file,
            ),
        );
        validate_generated_text_file(file)?;
        if let Some(extension) = file
            .extension()
            .and_then(|value| value.to_str())
            && forbidden
                .iter()
                .any(|item| extension.eq_ignore_ascii_case(item))
        {
            bad.push(file.clone());
        }
    }
    progress.finish();
    if !bad.is_empty() {
        return Err(
            PipelineError::new(
                format!(
                    concat!(
                        "extracted output still contains proprietary source ",
                        "extensions; first example: {}"
                    ),
                    bad.first()
                        .map_or_else(
                            || Path::new("<none>")
                                .display()
                                .to_string(),
                            |path| path
                                .display()
                                .to_string()
                        )
                ),
            ),
        );
    }
    Ok(
        StageReport {
            name: "normalized",
            files: extracted_files.len(),
            bytes: sum_file_lengths(extracted_root)?,
            note: concat!(
                "no .p3d/.rsd/.rcf/.rtf/.rmv/.bik/.bk2 files ",
                "remain under extracted"
            )
            .to_owned(),
        },
    )
}

/// Write pipeline report.
fn write_pipeline_report(
    extracted_root: &Path,
    report: &PipelineReport,
) -> PipelineOutcome<()> {
    let mut jsonl =
        String::from("{\"schema\":\"shar-schoenwald.pipeline.report.v1\"}\n");
    for stage in &report.stages {
        let row = format!(
            "{{\"stage\":\"{}\",\"files\":{},\"bytes\":{},\"note\":\"{}\"}}\n",
            stage.name,
            stage.files,
            stage.bytes,
            json_escape(&stage.note)
        );
        jsonl.push_str(&row);
    }
    let path = extracted_root.join("pipeline-report.jsonl");
    fs::write(
        &path, jsonl,
    )
    .map_err(io_error(&path))
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

/// Render one root-relative progress item without machine-specific prefixes.
fn progress_item(
    root: &Path,
    path: &Path,
) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace(
            char::from(92),
            "/",
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

/// Path without extension.
fn path_without_extension(path: &Path) -> PathBuf {
    let mut output = PathBuf::new();
    for component in path.components() {
        if let Component::Normal(part) = component {
            let item = Path::new(part);
            if item
                .extension()
                .is_some()
            {
                output.push(item.with_extension(""));
            } else {
                output.push(item);
            }
        }
    }
    output
}

/// Keeps manifest paths root-relative so generated ledgers are deterministic
/// across machines.
fn relative_manifest_path_text(
    game_root: &Path,
    extracted_root: &Path,
    input: &Path,
) -> PipelineOutcome<String> {
    let relative = if input.starts_with(game_root) {
        input
            .strip_prefix(game_root)
            .map_err(
                |_error| PipelineError::new("failed to relativize game path"),
            )?
    } else {
        input
            .strip_prefix(extracted_root)
            .map_err(
                |_error| {
                    PipelineError::new("failed to relativize extracted path")
                },
            )?
    };
    Ok(
        relative
            .to_string_lossy()
            .replace(
                char::from(92),
                "/",
            ),
    )
}

/// Sums file lengths at stage boundaries so reports never infer size from
/// platform-specific directory metadata.
fn sum_file_lengths(root: &Path) -> PipelineOutcome<u64> {
    let mut bytes = 0u64;
    for path in collect_files(root)? {
        bytes = bytes
            .saturating_add(local_file_len(&path).map_err(io_error(&path))?);
    }
    Ok(bytes)
}

/// Normalize sound scripts.
fn normalize_sound_scripts(
    extracted_root: &Path
) -> PipelineOutcome<StageReport> {
    let spt_inputs = files_with_extension(
        extracted_root,
        "spt",
    )?;
    let rms_inputs = files_with_extension(
        extracted_root,
        "rms",
    )?;
    let mut progress = StageProgress::begin(
        "sound scripts",
        spt_inputs
            .len()
            .saturating_add(rms_inputs.len()),
    );
    let mut files = 0usize;
    let mut bytes = 0u64;
    for input in spt_inputs {
        progress.advance(
            &progress_item(
                extracted_root,
                &input,
            ),
        );
        if !input.exists() {
            continue;
        }
        let output = input.with_extension("json");
        let json = spt::to_json(&input).map_err(io_error(&input))?;
        write_bytes(
            &output,
            json.as_bytes(),
        )?;
        fs::remove_file(&input).map_err(io_error(&input))?;
        files = files.saturating_add(1);
        bytes =
            bytes.saturating_add(u64::try_from(json.len()).unwrap_or(u64::MAX));
    }
    for input in rms_inputs {
        progress.advance(
            &progress_item(
                extracted_root,
                &input,
            ),
        );
        if !input.exists() {
            continue;
        }
        let output = input.with_extension("json");
        let json = rms::to_json(&input).map_err(io_error(&input))?;
        write_bytes(
            &output,
            json.as_bytes(),
        )?;
        fs::remove_file(&input).map_err(io_error(&input))?;
        files = files.saturating_add(1);
        bytes =
            bytes.saturating_add(u64::try_from(json.len()).unwrap_or(u64::MAX));
    }
    progress.finish();
    Ok(
        StageReport {
            name: "sound_scripts",
            files,
            bytes,
            note: concat!(
                "SPT and RMS sound scripts normalized to JSON; ",
                "original extracted sources removed"
            )
            .to_owned(),
        },
    )
}

#[cfg(test)]
mod component_ledger_tests {
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use super::{
        UnrealHapPackagePlan, component_ledger_files_exist,
        movie_outputs_complete,
    };

    static CASE_ID: AtomicUsize = AtomicUsize::new(0);

    fn case_dir(label: &str) -> Result<PathBuf, String> {
        let case = std::env::temp_dir().join(
            format!(
                "shar-pipeline-{label}-{}-{}",
                std::process::id(),
                CASE_ID.fetch_add(
                    1,
                    Ordering::Relaxed
                ),
            ),
        );
        if case.exists() {
            fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        }
        fs::create_dir_all(case.join("components"))
            .map_err(|error| error.to_string())?;
        Ok(case)
    }

    #[test]
    fn rejects_nested_or_duplicate_component_counts() -> Result<(), String> {
        let case = case_dir("component-count-ownership")?;
        fs::write(
            case.join("components/component.json"),
            "{}",
        )
        .map_err(|error| error.to_string())?;
        for (label, header) in [
            (
                "nested",
                concat!(
                    "{\"schema\":\"p3d.package.v1\",",
                    "\"metadata\":{\"component_count\":1}}",
                ),
            ),
            (
                "duplicate",
                concat!(
                    "{\"schema\":\"p3d.package.v1\",",
                    "\"component_count\":1,\"component_count\":2}",
                ),
            ),
        ] {
            fs::write(
                case.join("components.jsonl"),
                format!(
                    "{header}
{{\"path\":\"component.json\"}}
"
                ),
            )
            .map_err(|error| error.to_string())?;
            if component_ledger_files_exist(&case) {
                return Err(format!("{label} count field must be rejected"));
            }
        }
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        Ok(())
    }

    #[test]
    fn rejects_nested_or_duplicate_component_path_fields() -> Result<(), String>
    {
        let case = case_dir("component-path-ownership")?;
        fs::write(
            case.join("components/component.json"),
            "{}",
        )
        .map_err(|error| error.to_string())?;
        for (label, row) in [
            (
                "nested",
                "{\"metadata\":{\"path\":\"component.json\"}}",
            ),
            (
                "duplicate",
                concat!(
                    "{\"path\":\"component.json\",",
                    "\"path\":\"other.json\"}",
                ),
            ),
        ] {
            fs::write(
                case.join("components.jsonl"),
                format!(
                    "{{\"schema\":\"p3d.package.v1\",\"component_count\":1}}
{row}
"
                ),
            )
            .map_err(|error| error.to_string())?;
            if component_ledger_files_exist(&case) {
                return Err(format!("{label} path field must be rejected"));
            }
        }
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        Ok(())
    }

    #[test]
    fn rejects_empty_movie_outputs() -> Result<(), String> {
        let case = case_dir("empty-movie-outputs")?;
        let plan = UnrealHapPackagePlan::for_movie(
            &case, "intro",
        )
        .map_err(|error| error.to_string())?;
        fs::create_dir_all(&plan.movie_directory)
            .map_err(|error| error.to_string())?;
        fs::write(
            &plan.hap_video_path,
            [],
        )
        .map_err(|error| error.to_string())?;
        fs::write(
            &plan.manifest_path,
            [],
        )
        .map_err(|error| error.to_string())?;
        for required_path in [
            &plan.source_probe_path,
            &plan.decode_report_path,
            &plan.timing_manifest_path,
        ] {
            fs::write(
                required_path,
                [],
            )
            .map_err(|error| error.to_string())?;
        }
        fs::write(
            plan.movie_directory
                .join("audio_track_01.wav"),
            [],
        )
        .map_err(|error| error.to_string())?;
        let complete = movie_outputs_complete(&plan);
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        if complete {
            return Err(
                "zero-byte movie outputs must be incomplete".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_invalid_unrelated_json_scalars() -> Result<(), String> {
        let case = case_dir("invalid-json-scalars")?;
        fs::write(
            case.join("components/component.json"),
            "{}",
        )
        .map_err(|error| error.to_string())?;
        for invalid in [
            "garbage", "01", "1.", "1e",
        ] {
            let header = format!(
                "{{\"schema\":\"p3d.package.v1\",\"component_count\":1,\"\
                 broken\":{invalid}}}"
            );
            fs::write(
                case.join("components.jsonl"),
                format!(
                    "{header}
{{\"path\":\"component.json\"}}
"
                ),
            )
            .map_err(|error| error.to_string())?;
            if component_ledger_files_exist(&case) {
                return Err(
                    format!("invalid scalar {invalid} must be rejected"),
                );
            }
        }
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        Ok(())
    }

    #[test]
    fn rejects_trailing_component_json_commas() -> Result<(), String> {
        let case = case_dir("trailing-json-commas")?;
        fs::write(
            case.join("components/component.json"),
            "{}",
        )
        .map_err(|error| error.to_string())?;
        for (label, header, row) in [
            (
                "header",
                concat!(
                    "{\"schema\":\"p3d.package.v1\",",
                    "\"component_count\":1,}",
                ),
                "{\"path\":\"component.json\"}",
            ),
            (
                "row",
                concat!(
                    "{\"schema\":\"p3d.package.v1\",",
                    "\"component_count\":1}",
                ),
                "{\"path\":\"component.json\",}",
            ),
        ] {
            fs::write(
                case.join("components.jsonl"),
                format!(
                    "{header}
{row}
"
                ),
            )
            .map_err(|error| error.to_string())?;
            if component_ledger_files_exist(&case) {
                return Err(format!("trailing {label} comma must be rejected"));
            }
        }
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        Ok(())
    }

    #[test]
    fn rejects_unframed_or_trailing_component_json() -> Result<(), String> {
        let case = case_dir("malformed-component-json")?;
        fs::write(
            case.join("components/component.json"),
            "{}",
        )
        .map_err(|error| error.to_string())?;
        for (label, row) in [
            (
                "unframed",
                "\"path\":\"component.json\"",
            ),
            (
                "trailing",
                "{\"path\":\"component.json\"}garbage",
            ),
        ] {
            fs::write(
                case.join("components.jsonl"),
                format!(
                    "{{\"schema\":\"p3d.package.v1\",\"component_count\":1}}
{row}
"
                ),
            )
            .map_err(|error| error.to_string())?;
            if component_ledger_files_exist(&case) {
                return Err(format!("{label} JSON row must be rejected"));
            }
        }
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        Ok(())
    }

    #[test]
    fn rejects_component_paths_outside_the_package() -> Result<(), String> {
        let case = case_dir("outside-paths")?;
        let outside = case.join("outside.json");
        fs::write(
            &outside, "{}",
        )
        .map_err(|error| error.to_string())?;
        let absolute = outside
            .to_string_lossy()
            .replace(
                '\\', "\\\\",
            );
        for (label, path) in [
            (
                "parent",
                "../outside.json".to_owned(),
            ),
            (
                "absolute", absolute,
            ),
        ] {
            fs::write(
                case.join("components.jsonl"),
                format!(
                    "{{\"schema\":\"p3d.package.v1\",\"component_count\":1}}
                     {{\"path\":\"{path}\"}}
"
                ),
            )
            .map_err(|error| error.to_string())?;
            if component_ledger_files_exist(&case) {
                return Err(format!("{label} path must be rejected"));
            }
        }
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        Ok(())
    }

    #[test]
    fn rejects_case_aliases_for_component_paths() -> Result<(), String> {
        let case = case_dir("case-aliases")?;
        fs::write(
            case.join("components/component.json"),
            "{}",
        )
        .map_err(|error| error.to_string())?;
        fs::write(
            case.join("components.jsonl"),
            concat!(
                "{\"schema\":\"p3d.package.v1\",",
                "\"component_count\":2}
",
                "{\"path\":\"component.json\"}
",
                "{\"path\":\"COMPONENT.JSON\"}
",
            ),
        )
        .map_err(|error| error.to_string())?;
        let complete = component_ledger_files_exist(&case);
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        if complete {
            return Err("case aliases must not count twice".to_owned());
        }
        Ok(())
    }

    #[test]
    fn rejects_duplicate_component_paths() -> Result<(), String> {
        let case = case_dir("duplicate-paths")?;
        fs::write(
            case.join("components/component.json"),
            "{}",
        )
        .map_err(|error| error.to_string())?;
        let row = "{\"path\":\"component.json\"}";
        fs::write(
            case.join("components.jsonl"),
            format!(
                "{{\"schema\":\"p3d.package.v1\",\"component_count\":2}}
{row}
{row}
"
            ),
        )
        .map_err(|error| error.to_string())?;
        let complete = component_ledger_files_exist(&case);
        fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
        if complete {
            return Err(
                "duplicate component paths must be rejected".to_owned(),
            );
        }
        Ok(())
    }

    #[test]
    fn rejects_invalid_component_ledger_counts() -> Result<(), String> {
        for (label, count_field) in [
            (
                "mismatch",
                "\"component_count\":2",
            ),
            (
                "missing",
                "\"other_count\":1",
            ),
            (
                "nonnumeric",
                "\"component_count\":\"1\"",
            ),
        ] {
            let case = case_dir(label)?;
            fs::write(
                case.join("components/component.json"),
                "{}",
            )
            .map_err(|error| error.to_string())?;
            let header =
                format!("{{\"schema\":\"p3d.package.v1\",{count_field}}}");
            fs::write(
                case.join("components.jsonl"),
                format!(
                    "{header}
{{\"path\":\"component.json\"}}
"
                ),
            )
            .map_err(|error| error.to_string())?;
            let complete = component_ledger_files_exist(&case);
            fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
            if complete {
                return Err(format!("{label} count must be rejected"));
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_invalid_component_ledger_headers() -> Result<(), String> {
        for (label, header) in [
            (
                "wrong-schema",
                "{\"schema\":\"wrong\"}",
            ),
            (
                "missing-schema",
                "{\"component_count\":1}",
            ),
        ] {
            let case = case_dir(label)?;
            fs::write(
                case.join("components/component.json"),
                "{}",
            )
            .map_err(|error| error.to_string())?;
            fs::write(
                case.join("components.jsonl"),
                format!(
                    "{header}
{{\"path\":\"component.json\"}}
"
                ),
            )
            .map_err(|error| error.to_string())?;
            let complete = component_ledger_files_exist(&case);
            fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
            if complete {
                return Err(format!("{label} header must be rejected"));
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_empty_component_ledgers() -> Result<(), String> {
        for (label, contents) in [
            (
                "empty", "",
            ),
            (
                "header-only",
                concat!(
                    "{\"schema\":\"p3d.package.v1\",",
                    "\"byte_len\":0,\"chunk_count\":0,",
                    "\"component_count\":0}\n",
                ),
            ),
        ] {
            let case = case_dir(label)?;
            fs::write(
                case.join("components.jsonl"),
                contents,
            )
            .map_err(|error| error.to_string())?;
            let complete = component_ledger_files_exist(&case);
            fs::remove_dir_all(&case).map_err(|error| error.to_string())?;
            if complete {
                return Err(format!("{label} ledger must remain incomplete"));
            }
        }
        Ok(())
    }
}
