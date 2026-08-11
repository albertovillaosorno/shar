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
//   - Unit evidence for mission music-state metadata binding helpers.
// - Must-Not:
//   - Assign runtime RADMusic semantics to structural named-asset evidence.
// - Allows:
//   - Verify exact level parsing, metadata intake, and unique symbol windows.
// - Split-When:
//   - Event metadata or decoded state machines require independent fixtures.
// - Merge-When:
//   - Music-context integration tests own the same helper invariants.
// - Summary:
//   - Mission music-state context unit tests.
// - Description:
//   - Proves same-level named-asset windows remain strict and source-backed.
// - Usage:
//   - Compiled with the local outbound adapter unit suite.
// - Defaults:
//   - Schema drift, duplicate windows, and invalid level paths fail closed.
//

//! Unit evidence for mission music-state metadata context.

use super::*;

fn metadata(named_assets: &str) -> String {
    format!(
        concat!(
            "{{\"schema\":\"shar-schoenwald.radmusic-compiled.v3\",",
            "\"named_assets\":[{}]}}"
        ),
        named_assets,
    )
}

#[test]
fn binds_reviewed_state_value_window() -> Result<(), String> {
    let text = metadata(concat!(
        "{\"offset\":100,\"value\":\"Mission2\"},",
        "{\"offset\":109,\"value\":\"Stage1\"},",
        "{\"offset\":116,\"value\":\"Stage2\"},",
        "{\"offset\":151,\"value\":\"M2_start\"}"
    ));
    let assets = parse_named_assets(&text).map_err(|error| error.to_string())?;
    assert_eq!(
        resolve_named_asset_window(&assets, "Mission2", "Stage1")
            .map_err(|error| error.to_string())?,
        (100, 109)
    );
    assert_eq!(
        resolve_named_asset_window(&assets, "Mission2", "Stage2")
            .map_err(|error| error.to_string())?,
        (100, 116)
    );
    Ok(())
}

#[test]
fn rejects_ambiguous_or_distant_value_window() -> Result<(), String> {
    let ambiguous = metadata(concat!(
        "{\"offset\":100,\"value\":\"Mission2\"},",
        "{\"offset\":109,\"value\":\"Stage1\"},",
        "{\"offset\":200,\"value\":\"Mission2\"},",
        "{\"offset\":209,\"value\":\"Stage1\"}"
    ));
    let assets =
        parse_named_assets(&ambiguous).map_err(|error| error.to_string())?;
    assert!(
        resolve_named_asset_window(&assets, "Mission2", "Stage1").is_err()
    );

    let distant = metadata(concat!(
        "{\"offset\":100,\"value\":\"Mission2\"},",
        "{\"offset\":109,\"value\":\"OtherA\"},",
        "{\"offset\":116,\"value\":\"OtherB\"},",
        "{\"offset\":123,\"value\":\"Stage1\"}"
    ));
    let assets = parse_named_assets(&distant)
        .map_err(|error| error.to_string())?;
    assert!(
        resolve_named_asset_window(&assets, "Mission2", "Stage1").is_err()
    );
    Ok(())
}

#[test]
fn validates_metadata_schema_offsets_and_source_levels() -> Result<(), String> {
    assert_eq!(
        source_level("game/scripts/missions/level02/m2i.mfk.json")
            .map_err(|error| error.to_string())?,
        2
    );
    assert_eq!(
        source_level(r"game\scripts\missions\level07\m4i.mfk.json")
            .map_err(|error| error.to_string())?,
        7
    );
    assert!(source_level("game/scripts/missions/demo/d1i.mfk.json").is_err());

    let stale = "{\"schema\":\"old\",\"named_assets\":[]}";
    assert!(parse_named_assets(stale).is_err());
    let descending = metadata(concat!(
        "{\"offset\":20,\"value\":\"Mission2\"},",
        "{\"offset\":10,\"value\":\"Stage1\"}"
    ));
    assert!(parse_named_assets(&descending).is_err());
    Ok(())
}

#[test]
fn binding_accessors_preserve_exact_provenance() {
    let binding = MissionMusicStateBinding {
        source_path: "game/scripts/missions/level02/m2i.mfk.json".to_owned(),
        owner_stage_source_ordinal: 8,
        owner_stage_sequence_ordinal: 1,
        source_ordinal: 10,
        level: 2,
        state_name: "Mission2".to_owned(),
        state_value: "Stage1".to_owned(),
        package_id: "score-library".to_owned(),
        script_id: "script-l2".to_owned(),
        script_path: "extracted/music/l2_music.json".to_owned(),
        state_offset: 100,
        value_offset: 109,
    };
    let report = MissionMusicStateReport {
        bindings: vec![binding],
    };
    let [binding] = report.bindings() else {
        panic!("music binding count changed");
    };
    assert_eq!(
        binding.source_path(),
        "game/scripts/missions/level02/m2i.mfk.json"
    );
    assert_eq!(binding.owner_stage_source_ordinal(), 8);
    assert_eq!(binding.owner_stage_sequence_ordinal(), 1);
    assert_eq!(binding.source_ordinal(), 10);
    assert_eq!(binding.level(), 2);
    assert_eq!(binding.state_name(), "Mission2");
    assert_eq!(binding.state_value(), "Stage1");
    assert_eq!(binding.package_id(), "score-library");
    assert_eq!(binding.script_id(), "script-l2");
    assert_eq!(binding.script_path(), "extracted/music/l2_music.json");
    assert_eq!(binding.offsets(), (100, 109));
}
