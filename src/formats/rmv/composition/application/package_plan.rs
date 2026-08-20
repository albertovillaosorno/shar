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
//   - Package plan application service.
// - Must-Not:
//   - Own unrelated policy, persistence, or external effects.
// - Allows:
//   - Inputs and outputs required by this module boundary.
// - Split-When:
//   - Split when one responsibility gains an independent lifecycle.
// - Merge-When:
//   - Merge when another module owns the identical responsibility.
// - Summary:
//   - Package plan application service.
// - Description:
//   - Implements the declared application service responsibility for rmv.
// - Usage:
//   - Used through the owning function boundary.
// - Defaults:
//   - Invalid or missing inputs fail explicitly.
//

//! Package plan application service.

use std::path::{Component, Path, PathBuf};

use crate::domain::{
    CinematicTarget, RmvError, TargetDecision, is_windows_safe_component,
};

#[derive(Debug, Clone, PartialEq, Eq)]
/// Unrealhappackageplan.
pub struct UnrealHapPackagePlan {
    /// Target.
    pub target: CinematicTarget,
    /// Movie directory.
    pub movie_directory: PathBuf,
    /// HAP video path.
    pub hap_video_path: PathBuf,
    /// Video extension.
    pub video_extension: &'static str,
    /// HAP format.
    pub hap_format: &'static str,
    /// Audio track pattern.
    pub audio_track_pattern: PathBuf,
    /// Source probe path.
    pub source_probe_path: PathBuf,
    /// Decode report path.
    pub decode_report_path: PathBuf,
    /// Manifest path.
    pub manifest_path: PathBuf,
    /// Timing manifest path.
    pub timing_manifest_path: PathBuf,
    /// Optional BK2 path.
    pub optional_bk2_path: PathBuf,
}

impl UnrealHapPackagePlan {
    /// For movie.
    ///
    /// # Errors
    ///
    /// Returns an error when the movie stem is not one normal path component.
    pub fn for_movie(
        output_root: &Path,
        movie_stem: &str,
    ) -> Result<Self, RmvError> {
        let movie_path = Path::new(movie_stem);
        let mut components = movie_path.components();
        let Some(Component::Normal(movie_name)) = components.next() else {
            return Err(RmvError::InvalidMovieStem(movie_stem.to_owned()));
        };
        if components.next().is_some() || !is_windows_safe_component(movie_stem)
        {
            return Err(RmvError::InvalidMovieStem(movie_stem.to_owned()));
        }
        let movie_root = output_root.join(movie_name);
        Ok(Self {
            target: TargetDecision::without_official_bink2_encoder()
                .primary_target,
            movie_directory: movie_root.clone(),
            hap_video_path: movie_root.join("movie.mov"),
            video_extension: "mov",
            hap_format: "hap_q",
            audio_track_pattern: movie_root.join("audio_track_%02d.wav"),
            source_probe_path: movie_root.join("source-video.ffprobe.json"),
            decode_report_path: movie_root.join("decode-report.json"),
            manifest_path: movie_root.join("manifest.json"),
            timing_manifest_path: movie_root.join("timing.tsv"),
            optional_bk2_path: movie_root.join("movie.bk2"),
        })
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact test module path is indivisible
#[path = "../../../../../tests/formats/rmv/unit/application/package_plan/tests.rs"]
mod tests;
