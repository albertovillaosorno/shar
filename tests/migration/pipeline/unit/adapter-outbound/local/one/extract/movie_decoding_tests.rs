// Copyright:
//   - Copyright © 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE-MIT
//
// Boundary-Contract:
// - Owns:
//   - Movie decoder policy unit tests.
// - Must-Not:
//   - Own production behavior or execute external media tools.
// - Allows:
//   - Pure assertions over supported movie classifications.
// - Split-When:
//   - Split when decoder capabilities gain independent ownership.
// - Merge-When:
//   - Merge when another module owns the identical evidence.
// - Summary:
//   - Movie decoder policy unit tests.
// - Description:
//   - Proves FFmpeg-decodable Xbox XMV inputs are exported to HAP.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Unsupported or unknown movie kinds remain excluded.
//

//! Movie decoder policy unit tests.

use std::path::{Path, PathBuf};

use rmv::domain::ProvenanceEvidence;
use rmv::{MovieKind, MovieRecord, Sha256, UnrealHapPackagePlan};

use super::{
    is_movie_decodable_by_ffmpeg, movie_tool_io_error,
    render_movie_decode_report, render_movie_package_manifest,
};

#[test]
fn xbox_xmv_movies_are_exported_through_ffmpeg() {
    assert!(is_movie_decodable_by_ffmpeg(MovieKind::XboxXmvLike));
    assert!(is_movie_decodable_by_ffmpeg(MovieKind::BinkV1));
    assert!(is_movie_decodable_by_ffmpeg(MovieKind::BinkV2));
    assert!(!is_movie_decodable_by_ffmpeg(MovieKind::Unknown));
    assert!(!is_movie_decodable_by_ffmpeg(MovieKind::OggNamedRmv));
}

#[test]
fn movie_manifests_publish_only_package_relative_paths() -> Result<(), String> {
    let sensitive = PathBuf::from("operator-private-root")
        .join(".accepted.pipeline-staging");
    let game_root = PathBuf::from("game-root");
    let plan = UnrealHapPackagePlan::for_movie(
        &sensitive.join("movies"),
        "intro",
    )
    .map_err(|error| error.to_string())?;
    let record = MovieRecord {
        source_root: game_root.clone(),
        source_path: game_root.join("movies/intro.rmv"),
        relative_path: PathBuf::from("movies/intro.rmv"),
        output_path: sensitive.join("movies/intro/movie.bk2"),
        bytes: 5,
        kind: MovieKind::BinkV1,
        hash: Sha256::digest(b"movie"),
        provenance: ProvenanceEvidence {
            embedded_source_names: Vec::new(),
        },
    };
    let logical_path = PathBuf::from("movies").join("intro.rmv");
    let manifest = render_movie_package_manifest(
        &game_root,
        &record,
        &logical_path,
        &plan,
        30.0,
        "30/1",
    )
    .map_err(|error| error.to_string())?;
    let report = render_movie_decode_report(&plan, 2)
        .map_err(|error| error.to_string())?;
    for document in [&manifest, &report] {
        if document.contains("operator-private-root")
            || document.contains("pipeline-staging")
        {
            return Err("movie evidence leaked its output root".to_owned());
        }
    }
    let manifest_json: serde_json::Value =
        serde_json::from_str(&manifest).map_err(|error| error.to_string())?;
    for (field, expected) in [
        ("logical_path", "movies/intro.rmv"),
        ("movie_directory", "."),
        ("hap_video_path", "movie.mov"),
        ("audio_track_pattern", "audio_track_%02d.wav"),
        ("timing_manifest_path", "timing.tsv"),
        ("source_probe_path", "source-video.ffprobe.json"),
        ("optional_bk2_path", "movie.bk2"),
    ] {
        if manifest_json.get(field).and_then(|value| value.as_str())
            != Some(expected)
        {
            return Err(format!("unexpected {field} in movie manifest"));
        }
    }
    let report_json: serde_json::Value =
        serde_json::from_str(&report).map_err(|error| error.to_string())?;
    if report_json.get("video_path").and_then(|value| value.as_str())
        != Some("movie.mov")
    {
        return Err(
            "decode report video path is not package-relative".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn movie_relative_paths_reject_artifacts_outside_the_package(
) -> Result<(), String> {
    let plan = UnrealHapPackagePlan::for_movie(Path::new("out"), "intro")
        .map_err(|error| error.to_string())?;
    let mut escaped = plan;
    escaped.hap_video_path = PathBuf::from("other/movie.mov");
    if render_movie_decode_report(&escaped, 1).is_ok() {
        return Err("escaped movie artifact path was accepted".to_owned());
    }
    Ok(())
}

#[test]
fn movie_tool_diagnostics_hide_physical_error_paths() -> Result<(), String> {
    let private_fragment = "private-workstation/movie-source.rmv";
    let error = std::io::Error::other(private_fragment);
    let rendered = movie_tool_io_error(
        "run ffprobe",
        "movies/intro.rmv",
        &error,
    )
    .to_string();
    if rendered.contains(private_fragment)
        || !rendered.contains("movies/intro.rmv")
        || !rendered.contains("Other")
    {
        return Err(format!("movie diagnostic was not public-safe: {rendered}"));
    }
    Ok(())
}
