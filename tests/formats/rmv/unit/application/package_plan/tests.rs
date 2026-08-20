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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

//! Tests unit tests.

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
    let plan_result = UnrealHapPackagePlan::for_movie(&output_root, "intro");
    assert!(plan_result.is_ok(), "safe movie stem should be accepted");
    let Ok(plan) = plan_result else {
        return;
    };
    assert_eq!(
        plan.audio_track_pattern,
        output_root.join("intro").join("audio_track_%02d.wav")
    );
}

#[test]
fn rejects_movie_stems_longer_than_a_windows_component() {
    let stem = "a".repeat(256);
    assert!(UnrealHapPackagePlan::for_movie(Path::new("out"), &stem,).is_err());
}

#[test]
fn rejects_windows_superscript_port_device_names() {
    for stem in ["COM¹", "COM²", "COM³", "LPT¹", "LPT²", "LPT³"] {
        assert!(
            UnrealHapPackagePlan::for_movie(Path::new("out"), stem,).is_err(),
            "Windows superscript device name was accepted: {stem}"
        );
    }
}

#[test]
fn rejects_windows_console_device_names() {
    for stem in ["CONIN$", "CONOUT$"] {
        assert!(
            UnrealHapPackagePlan::for_movie(Path::new("out"), stem,).is_err(),
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
            UnrealHapPackagePlan::for_movie(Path::new("out"), stem,).is_err(),
            "unsafe Windows movie stem was accepted: {stem}"
        );
    }
}

#[test]
fn rejects_movie_stems_that_escape_the_output_root() {
    assert!(
        UnrealHapPackagePlan::for_movie(Path::new("out"), "../escape",)
            .is_err()
    );
}

#[test]
fn plans_hap_audio_manifest_and_optional_bk2_under_movie_root() {
    let plan_result =
        UnrealHapPackagePlan::for_movie(Path::new("out"), "intro");
    assert!(plan_result.is_ok(), "safe movie stem should be accepted");
    let Ok(plan) = plan_result else {
        return;
    };
    assert_eq!(plan.target, CinematicTarget::UnrealHapMovie);
    assert_eq!(plan.movie_directory, Path::new("out").join("intro"));
    assert_eq!(
        plan.hap_video_path,
        Path::new("out").join("intro").join("movie.mov")
    );
    assert_eq!(plan.video_extension, "mov");
    assert_eq!(plan.hap_format, "hap_q");
    assert_eq!(
        plan.audio_track_pattern,
        Path::new("out").join("intro").join("audio_track_%02d.wav")
    );
    assert_eq!(
        plan.source_probe_path,
        Path::new("out")
            .join("intro")
            .join("source-video.ffprobe.json")
    );
    assert_eq!(
        plan.decode_report_path,
        Path::new("out").join("intro").join("decode-report.json")
    );
    assert_eq!(
        plan.manifest_path,
        Path::new("out").join("intro").join("manifest.json")
    );
    assert_eq!(
        plan.timing_manifest_path,
        Path::new("out").join("intro").join("timing.tsv")
    );
    assert_eq!(
        plan.optional_bk2_path,
        Path::new("out").join("intro").join("movie.bk2")
    );
}
