// File:
//   - package_plan.rs
// Path:
//   - src/rmv/src/application/package_plan.rs
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
//   - rmv use-case orchestration for application package plan.
// - Must-Not:
//   - Depend on driven adapters, parse local routes, or encode writer-specific
//   - syntax.
// - Allows:
//   - Use-case orchestration, planning, reporting, and calls through declared
//   - ports.
// - Split-When:
//   - Split when package plan contains two independently testable contracts.
// - Merge-When:
//   - Another rmv module owns the same application boundary with no distinct
//   - invariant.
// - Summary:
//   - HAP cinematic package planning.
// - Description:
//   - Defines package plan data and behavior for rmv application.
// - Usage:
//   - Called by entrypoints after ports and adapters are selected by the
//   - caller.
// - Defaults:
//   - No concrete adapter is selected unless the caller supplies one through a
//   - port.
//
// ADRs:
// - docs/adr/pipeline/extraction/extraction-provenance-and-manifest-linkage.md
//
// Large file:
//   - false
//

//! HAP cinematic package planning.
//!
//! This boundary keeps hap cinematic package planning explicit and returns
//! deterministic results to rmv callers.
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
        if components
            .next()
            .is_some()
            || !is_windows_safe_component(movie_stem)
        {
            return Err(RmvError::InvalidMovieStem(movie_stem.to_owned()));
        }
        let movie_root = output_root.join(movie_name);
        Ok(
            Self {
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
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::UnrealHapPackagePlan;
    use crate::domain::CinematicTarget;

    #[cfg(windows)]
    #[test]
    fn preserves_non_unicode_audio_pattern_paths() {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt as _;
        use std::path::PathBuf;

        let output_root = PathBuf::from(OsString::from_wide(&[0xd800]));
        let plan_result = UnrealHapPackagePlan::for_movie(
            &output_root,
            "intro",
        );
        assert!(
            plan_result.is_ok(),
            "safe movie stem should be accepted"
        );
        let Ok(plan) = plan_result else {
            return;
        };
        assert_eq!(
            plan.audio_track_pattern,
            output_root
                .join("intro")
                .join("audio_track_%02d.wav")
        );
    }

    #[test]
    fn rejects_movie_stems_longer_than_a_windows_component() {
        let stem = "a".repeat(256);
        assert!(
            UnrealHapPackagePlan::for_movie(
                Path::new("out"),
                &stem,
            )
            .is_err()
        );
    }

    #[test]
    fn rejects_windows_superscript_port_device_names() {
        for stem in [
            "COM¹", "COM²", "COM³", "LPT¹", "LPT²", "LPT³",
        ] {
            assert!(
                UnrealHapPackagePlan::for_movie(
                    Path::new("out"),
                    stem,
                )
                .is_err(),
                "Windows superscript device name was accepted: {stem}"
            );
        }
    }

    #[test]
    fn rejects_windows_console_device_names() {
        for stem in [
            "CONIN$", "CONOUT$",
        ] {
            assert!(
                UnrealHapPackagePlan::for_movie(
                    Path::new("out"),
                    stem,
                )
                .is_err(),
                "Windows console device name was accepted: {stem}"
            );
        }
    }

    #[test]
    fn rejects_windows_unsafe_movie_directory_names() {
        for stem in [
            "CON",
            "aux.txt",
            "LPT1",
            "movie.",
            "movie ",
            "movie?alt",
            "movie:alt",
        ] {
            assert!(
                UnrealHapPackagePlan::for_movie(
                    Path::new("out"),
                    stem,
                )
                .is_err(),
                "unsafe Windows movie stem was accepted: {stem}"
            );
        }
    }

    #[test]
    fn rejects_movie_stems_that_escape_the_output_root() {
        assert!(
            UnrealHapPackagePlan::for_movie(
                Path::new("out"),
                "../escape",
            )
            .is_err()
        );
    }

    #[test]
    fn plans_hap_audio_manifest_and_optional_bk2_under_movie_root() {
        let plan_result = UnrealHapPackagePlan::for_movie(
            Path::new("out"),
            "intro",
        );
        assert!(
            plan_result.is_ok(),
            "safe movie stem should be accepted"
        );
        let Ok(plan) = plan_result else {
            return;
        };
        assert_eq!(
            plan.target,
            CinematicTarget::UnrealHapMovie
        );
        assert_eq!(
            plan.movie_directory,
            Path::new("out").join("intro")
        );
        assert_eq!(
            plan.hap_video_path,
            Path::new("out")
                .join("intro")
                .join("movie.mov")
        );
        assert_eq!(
            plan.video_extension,
            "mov"
        );
        assert_eq!(
            plan.hap_format,
            "hap_q"
        );
        assert_eq!(
            plan.audio_track_pattern,
            Path::new("out")
                .join("intro")
                .join("audio_track_%02d.wav")
        );
        assert_eq!(
            plan.source_probe_path,
            Path::new("out")
                .join("intro")
                .join("source-video.ffprobe.json")
        );
        assert_eq!(
            plan.decode_report_path,
            Path::new("out")
                .join("intro")
                .join("decode-report.json")
        );
        assert_eq!(
            plan.manifest_path,
            Path::new("out")
                .join("intro")
                .join("manifest.json")
        );
        assert_eq!(
            plan.timing_manifest_path,
            Path::new("out")
                .join("intro")
                .join("timing.tsv")
        );
        assert_eq!(
            plan.optional_bk2_path,
            Path::new("out")
                .join("intro")
                .join("movie.bk2")
        );
    }
}
