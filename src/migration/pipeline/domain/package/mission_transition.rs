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
//   - Pure classification of reviewed stage transition and presentation
//     markers.
// - Must-Not:
//   - Infer successor stages, recovery topology, or presentation completion.
//   - Treat visual presentation as authoritative mission success or failure.
// - Allows:
//   - Preserve source stage identity and classify effective terminal/visual
//     policy.
//   - Apply reviewed source precedence when multiple visual markers coexist.
// - Split-When:
//   - Successor/recovery graph compilation becomes independently reusable.
// - Merge-When:
//   - Stage transition policy becomes intrinsic to the stage semantic report.
// - Summary:
//   - Reviewed stage transition-policy classification.
// - Description:
//   - Separates terminal mission intent from visual and HUD presentation flags.
// - Usage:
//   - Compile after typed stage semantics and before final mission graph
//     emission.
// - Defaults:
//   - No marker means no terminal override and no visual transition request.
//

//! Source-backed mission stage transition-policy classification.

use super::{MissionStageDirective, MissionStageSemanticReport};

/// Effective visual transition requested when a stage completes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionStageVisualTransition {
    /// No authored iris or fade request.
    None,
    /// Iris close; this wins when both iris and fade markers are authored.
    Iris,
    /// Fade to black when no iris marker is authored.
    Fade,
}

/// Explicit terminal override authored for a completed stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionStageTerminalOutcome {
    /// No explicit level/game terminal override.
    None,
    /// Complete the current level/chapter flow.
    ChapterTransition,
    /// Complete the game; this also subsumes level completion.
    GameCompletion,
}

/// Exact source marker kind retained before effective classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionStageTransitionMarkerKind {
    /// Authored iris-wipe completion marker.
    IrisWipe,
    /// Authored fade-out completion marker.
    FadeOut,
    /// Authored level-over terminal marker.
    LevelOver,
    /// Authored game-over terminal marker.
    GameOver,
    /// Authored stay-in-black presentation marker.
    StayInBlack,
    /// Authored stage-complete presentation marker.
    ShowStageComplete,
}

/// One exact authored transition or presentation marker occurrence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MissionStageTransitionMarker {
    source_ordinal: usize,
    kind: MissionStageTransitionMarkerKind,
}

impl MissionStageTransitionMarker {
    /// Return the exact source statement ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the typed authored marker kind.
    #[must_use]
    pub const fn kind(&self) -> MissionStageTransitionMarkerKind {
        self.kind
    }
}

/// Effective reviewed policy for one typed stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageTransitionPolicy {
    source_ordinal: usize,
    sequence_ordinal: usize,
    visual: MissionStageVisualTransition,
    terminal: MissionStageTerminalOutcome,
    stay_in_black: bool,
    show_stage_complete: bool,
    markers: Vec<MissionStageTransitionMarker>,
}

impl MissionStageTransitionPolicy {
    /// Return the source `AddStage` ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return dense source stage order.
    #[must_use]
    pub const fn sequence_ordinal(&self) -> usize {
        self.sequence_ordinal
    }

    /// Return the effective visual completion transition.
    #[must_use]
    pub const fn visual(&self) -> MissionStageVisualTransition {
        self.visual
    }

    /// Return the explicit terminal override.
    #[must_use]
    pub const fn terminal(&self) -> MissionStageTerminalOutcome {
        self.terminal
    }

    /// Return whether the stage keeps the screen black while active.
    #[must_use]
    pub const fn stay_in_black(&self) -> bool {
        self.stay_in_black
    }

    /// Return whether successful completion requests stage-complete HUD output.
    #[must_use]
    pub const fn show_stage_complete(&self) -> bool {
        self.show_stage_complete
    }

    /// Return exact authored marker occurrences in source order.
    #[must_use]
    pub fn markers(&self) -> &[MissionStageTransitionMarker] {
        &self.markers
    }
}

/// Reviewed transition policies for all typed stages.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageTransitionReport {
    stages: Vec<MissionStageTransitionPolicy>,
}

impl MissionStageTransitionReport {
    /// Return policies in the stage report's deterministic order.
    #[must_use]
    pub fn stages(&self) -> &[MissionStageTransitionPolicy] {
        &self.stages
    }
}

/// Classify typed stage markers without inventing graph successors.
#[must_use]
pub fn preflight_mission_stage_transitions(
    report: &MissionStageSemanticReport,
) -> MissionStageTransitionReport {
    let stages = report.stages().iter().map(|stage| {
        classify_stage(
            stage.source_ordinal(),
            stage.sequence_ordinal(),
            stage.directives(),
        )
    }).collect();
    MissionStageTransitionReport { stages }
}

fn classify_stage(
    source_ordinal: usize,
    sequence_ordinal: usize,
    directives: &[MissionStageDirective],
) -> MissionStageTransitionPolicy {
    let mut has_iris = false;
    let mut has_fade = false;
    let mut has_level_over = false;
    let mut has_game_over = false;
    let mut stay_in_black = false;
    let mut show_stage_complete = false;
    let mut markers = Vec::new();
    for directive in directives {
        match directive {
            MissionStageDirective::IrisWipeLegacyArgument {
                source_ordinal, ..
            } => {
                has_iris = true;
                markers.push(MissionStageTransitionMarker {
                    source_ordinal: *source_ordinal,
                    kind: MissionStageTransitionMarkerKind::IrisWipe,
                });
            },
            MissionStageDirective::FadeOutLegacyArgument {
                source_ordinal, ..
            } => {
                has_fade = true;
                markers.push(MissionStageTransitionMarker {
                    source_ordinal: *source_ordinal,
                    kind: MissionStageTransitionMarkerKind::FadeOut,
                });
            },
            MissionStageDirective::LevelOver { source_ordinal } => {
                has_level_over = true;
                markers.push(MissionStageTransitionMarker {
                    source_ordinal: *source_ordinal,
                    kind: MissionStageTransitionMarkerKind::LevelOver,
                });
            },
            MissionStageDirective::GameOver { source_ordinal } => {
                has_game_over = true;
                markers.push(MissionStageTransitionMarker {
                    source_ordinal: *source_ordinal,
                    kind: MissionStageTransitionMarkerKind::GameOver,
                });
            },
            MissionStageDirective::StayInBlack { source_ordinal } => {
                stay_in_black = true;
                markers.push(MissionStageTransitionMarker {
                    source_ordinal: *source_ordinal,
                    kind: MissionStageTransitionMarkerKind::StayInBlack,
                });
            },
            MissionStageDirective::ShowStageComplete { source_ordinal } => {
                show_stage_complete = true;
                markers.push(MissionStageTransitionMarker {
                    source_ordinal: *source_ordinal,
                    kind: MissionStageTransitionMarkerKind::ShowStageComplete,
                });
            }
            _ => {}
        }
    }
    let visual = if has_iris {
        MissionStageVisualTransition::Iris
    } else if has_fade {
        MissionStageVisualTransition::Fade
    } else {
        MissionStageVisualTransition::None
    };
    let terminal = if has_game_over {
        MissionStageTerminalOutcome::GameCompletion
    } else if has_level_over {
        MissionStageTerminalOutcome::ChapterTransition
    } else {
        MissionStageTerminalOutcome::None
    };
    MissionStageTransitionPolicy {
        source_ordinal,
        sequence_ordinal,
        visual,
        terminal,
        stay_in_black,
        show_stage_complete,
        markers,
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact syntax is indivisible
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_transition/tests.rs"]
mod tests;
