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
//   - Unit evidence for canonical objective FMV package binding.
// - Must-Not:
//   - Assert playback, audio, transition, or music-stop semantics.
// - Allows:
//   - Verify exact story-movie identity and opaque argument preservation.
// - Split-When:
//   - Another source movie namespace requires independent fixtures.
// - Merge-When:
//   - Final mission asset tests own these exact FMV references.
// - Summary:
//   - Mission FMV reference unit tests.
// - Description:
//   - Proves typed RMV paths bind to one converted package movie member.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Missing, ambiguous, or malformed story movie evidence fails closed.
//

//! Unit evidence for canonical mission FMV references.

use super::*;

fn movie_row(package_id: &str, stem: &str, movie_id: &str) -> String {
    format!(
        concat!(
            "{{\"package_id\":\"{0}\",",
            "\"package_root\":\"extracted/movies/{1}\",",
            "\"package_category\":\"movies\",",
            "\"package_subcategory\":\"movies/story/{1}\",",
            "\"unit_count\":1,\"text_key_count\":0,",
            "\"unit_ids\":[\"{2}\"],",
            "\"world_ids\":[],\"texture_ids\":[],",
            "\"material_ids\":[],\"model_ids\":[],",
            "\"physics_ids\":[],\"animation_ids\":[],",
            "\"scene_ids\":[],\"locator_ids\":[],",
            "\"camera_ids\":[],\"light_ids\":[],",
            "\"particle_ids\":[],\"controller_ids\":[],",
            "\"audio_ids\":[],\"movie_ids\":[\"{2}\"],",
            "\"script_ids\":[],\"text_ids\":[],\"ui_ids\":[],",
            "\"metadata_ids\":[],\"error_ids\":[],",
            "\"source_unit_ids\":[],\"text_key_ids\":[],",
            "\"members\":[{{\"id\":\"{2}\",\"role\":\"movie\",",
            "\"path\":\"extracted/movies/{1}/movie.mov\",",
            "\"type\":\"movie-video\",\"kind\":\"runtime-asset\",",
            "\"source_chunk_kind\":\"none\"}}],\"text_keys\":[]}}"
        ),
        package_id, stem, movie_id,
    )
}

fn objectives(
    rmv_path: &str,
    legacy: Option<&str>,
) -> MissionObjectiveSemanticReport {
    MissionObjectiveSemanticReport::from_route_entries_for_tests(vec![(
        19,
        0,
        20,
        "fmv".to_owned(),
        vec![MissionObjectiveDirective::FmvInfo {
            source_ordinal: 21,
            rmv_path: rmv_path.to_owned(),
            legacy_argument: legacy.map(str::to_owned),
        }],
    )])
}

#[test]
fn binds_story_movie_and_preserves_opaque_argument() -> Result<(), String> {
    let index = PhaseThreePackageIndex::from_jsonl(&movie_row(
        "extracted-movies-fmv7",
        "fmv7",
        "movie-video-fmv7",
    ))
    .map_err(|error| error.to_string())?;
    let report = preflight_mission_fmv_references(
        &index,
        &objectives("fmv7.rmv", Some("stopmusic")),
    )?;
    let [binding] = report.bindings() else {
        return Err("mission FMV binding count drifted".to_owned());
    };
    assert_eq!(binding.owner_stage_source_ordinal(), 19);
    assert_eq!(binding.owner_stage_sequence_ordinal(), 0);
    assert_eq!(binding.objective_source_ordinal(), 20);
    assert_eq!(binding.source_ordinal(), 21);
    assert_eq!(binding.rmv_path(), "fmv7.rmv");
    assert_eq!(binding.legacy_argument(), Some("stopmusic"));
    assert_eq!(binding.package_id(), "extracted-movies-fmv7");
    assert_eq!(binding.package_root(), "extracted/movies/fmv7");
    assert_eq!(binding.package_subcategory(), "movies/story/fmv7");
    assert_eq!(binding.movie_id(), "movie-video-fmv7");
    assert_eq!(binding.movie_path(), "extracted/movies/fmv7/movie.mov");
    Ok(())
}

#[test]
fn rejects_missing_story_movie() -> Result<(), String> {
    let row = movie_row(
        "extracted-movies-fmv7",
        "fmv7",
        "movie-video-fmv7",
    );
    let index = PhaseThreePackageIndex::from_jsonl(&row)
        .map_err(|error| error.to_string())?;
    assert!(
        preflight_mission_fmv_references(
            &index,
            &objectives("fmv6.rmv", None),
        )
        .is_err()
    );
    Ok(())
}

#[test]
fn rejects_non_basename_and_non_rmv_paths() {
    assert!(story_movie_stem("movies/fmv7.rmv").is_err());
    assert!(story_movie_stem("fmv7.mov").is_err());
    assert_eq!(story_movie_stem("FMV7.RMV").as_deref(), Ok("fmv7"));
}
