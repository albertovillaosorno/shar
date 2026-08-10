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
//   - Unit evidence for reviewed stage transition-policy classification.
// - Must-Not:
//   - Invent successor stages or treat presentation flags as terminal outcomes.
// - Allows:
//   - Lock reviewed iris/fade and game/level terminal precedence.
// - Split-When:
//   - Corpus transition-policy tests need physical source fixtures.
// - Merge-When:
//   - Transition policy tests move into a complete mission graph suite.
// - Summary:
//   - Stage transition-policy unit tests.
// - Description:
//   - Proves visual, presentation, and terminal markers stay distinct.
// - Usage:
//   - Compiled with the package-domain unit suite.
// - Defaults:
//   - No marker means no terminal or visual override.
//

use super::*;

fn policy(directives: &[MissionStageDirective]) -> MissionStageTransitionPolicy {
    classify_stage(7, 2, directives)
}

#[test]
fn no_markers_have_no_transition_overrides() {
    let stage = policy(&[]);
    assert_eq!(stage.source_ordinal(), 7);
    assert_eq!(stage.sequence_ordinal(), 2);
    assert_eq!(stage.visual(), MissionStageVisualTransition::None);
    assert_eq!(stage.terminal(), MissionStageTerminalOutcome::None);
    assert!(!stage.stay_in_black());
    assert!(!stage.show_stage_complete());
}

#[test]
fn iris_wins_when_fade_and_iris_are_both_authored() {
    let stage = policy(&[
        MissionStageDirective::FadeOutLegacyArgument {
            source_ordinal: 8,
            source_value: "1.0".to_owned(),
        },
        MissionStageDirective::IrisWipeLegacyArgument {
            source_ordinal: 9,
            source_value: "0.1".to_owned(),
        },
    ]);
    assert_eq!(stage.visual(), MissionStageVisualTransition::Iris);
}

#[test]
fn game_completion_subsumes_level_completion() {
    let stage = policy(&[
        MissionStageDirective::LevelOver { source_ordinal: 8 },
        MissionStageDirective::GameOver { source_ordinal: 9 },
    ]);
    assert_eq!(
        stage.terminal(),
        MissionStageTerminalOutcome::GameCompletion
    );
}

#[test]
fn presentation_flags_do_not_become_terminal_outcomes() {
    let stage = policy(&[
        MissionStageDirective::StayInBlack { source_ordinal: 8 },
        MissionStageDirective::ShowStageComplete { source_ordinal: 9 },
    ]);
    assert!(stage.stay_in_black());
    assert!(stage.show_stage_complete());
    assert_eq!(stage.terminal(), MissionStageTerminalOutcome::None);
    assert_eq!(stage.visual(), MissionStageVisualTransition::None);
}
