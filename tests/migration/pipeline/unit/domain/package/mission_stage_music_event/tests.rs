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
//   - Unit evidence for stage-start mission music event bindings.
// - Must-Not:
//   - Assert a numeric legacy key or audio playback policy.
// - Allows:
//   - Verify source ownership, tokens, channel, and transform identity.
// - Split-When:
//   - Numeric key compatibility requires an independent fixture surface.
// - Merge-When:
//   - Complete mission presentation tests own this exact contract.
// - Summary:
//   - Stage-start mission music event binding tests.
// - Description:
//   - Locks reviewed transport identity without synthesizing audio behavior.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - Contradictory ordinals and nonportable tokens fail closed.
//

//! Unit evidence for stage-start mission music event bindings.

use super::*;

fn stages(
    directives: Vec<MissionStageDirective>,
) -> MissionStageSemanticReport {
    MissionStageSemanticReport::from_topology_entries_for_tests(vec![(
        2, 0, false, directives,
    )])
}

#[test]
fn binds_event_to_reviewed_runtime_channel() -> Result<(), String> {
    let report = preflight_mission_stage_music_events(&stages(vec![
        MissionStageDirective::StageStartMusicEvent {
            source_ordinal: 5,
            event_id: "L7_drama".to_owned(),
        },
    ]))?;
    let [binding] = report.bindings() else {
        return Err("stage music event binding count drifted".to_owned());
    };
    assert_eq!(binding.stage_source_ordinal(), 2);
    assert_eq!(binding.stage_sequence_ordinal(), 0);
    assert_eq!(binding.source_ordinal(), 5);
    assert_eq!(binding.event_id(), "L7_drama");
    assert_eq!(
        binding.channel(),
        MissionStageMusicEventChannel::MissionDrama
    );
    assert_eq!(
        binding.key_transform(),
        MissionStageMusicEventKeyTransform::LegacyCaseInsensitiveKey32
    );
    Ok(())
}

#[test]
fn rejects_event_before_owning_stage() {
    let error = preflight_mission_stage_music_events(&stages(vec![
        MissionStageDirective::StageStartMusicEvent {
            source_ordinal: 1,
            event_id: "M4_start".to_owned(),
        },
    ]))
    .expect_err("event before stage must fail");
    assert!(error.contains("precedes its owning stage"));
}

#[test]
fn rejects_nonportable_event_token() {
    let error = preflight_mission_stage_music_events(&stages(vec![
        MissionStageDirective::StageStartMusicEvent {
            source_ordinal: 5,
            event_id: "drama\n".to_owned(),
        },
    ]))
    .expect_err("control-bearing event token must fail");
    assert!(error.contains("not portable"));
}
