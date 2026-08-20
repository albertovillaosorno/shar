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
//   - Authored countdown start and ordered display-entry relationships.
// - Must-Not:
//   - Interpret countdown tokens, timing UX, or gameplay start behavior.
// - Allows:
//   - Bind ordered entries to one prior countdown start in the same stage.
//   - Reject orphan entries or duplicate starts before mission emission.
// - Split-When:
//   - Countdown runtime playback gains an independently authoritative model.
// - Merge-When:
//   - Final stage graph compilation owns this exact authored sequence.
// - Summary:
//   - Authored countdown-sequence semantic preflight.
// - Description:
//   - Preserves countdown blocks without assigning display-token meaning.
// - Usage:
//   - Runs after typed stage semantic compilation.
// - Defaults:
//   - Orphan entries and duplicate starts fail closed.
//

//! Source-backed authored countdown sequence blocks.

use super::{MissionStageDirective, MissionStageSemanticReport};

/// One ordered countdown display entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionCountdownEntryBinding {
    source_ordinal: usize,
    token: String,
    duration_milliseconds: u32,
}

impl MissionCountdownEntryBinding {
    /// Return the `AddToCountdownSequence` source ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact authored display token.
    #[must_use]
    pub fn token(&self) -> &str {
        &self.token
    }

    /// Return the exact positive authored duration in milliseconds.
    #[must_use]
    pub const fn duration_milliseconds(&self) -> u32 {
        self.duration_milliseconds
    }
}

/// One stage's complete authored countdown block.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionCountdownBinding {
    stage_source_ordinal: usize,
    stage_sequence_ordinal: usize,
    start_source_ordinal: usize,
    sequence_id: String,
    character_id: Option<String>,
    entries: Vec<MissionCountdownEntryBinding>,
}

impl MissionCountdownBinding {
    /// Return the source `AddStage` ordinal.
    #[must_use]
    pub const fn stage_source_ordinal(&self) -> usize {
        self.stage_source_ordinal
    }

    /// Return the dense owning stage ordinal.
    #[must_use]
    pub const fn stage_sequence_ordinal(&self) -> usize {
        self.stage_sequence_ordinal
    }

    /// Return the `StartCountdown` source ordinal.
    #[must_use]
    pub const fn start_source_ordinal(&self) -> usize {
        self.start_source_ordinal
    }

    /// Return the exact authored countdown sequence identity.
    #[must_use]
    pub fn sequence_id(&self) -> &str {
        &self.sequence_id
    }

    /// Return the optional exact authored character identity.
    #[must_use]
    pub fn character_id(&self) -> Option<&str> {
        self.character_id.as_deref()
    }

    /// Return display entries in authored order.
    #[must_use]
    pub fn entries(&self) -> &[MissionCountdownEntryBinding] {
        &self.entries
    }
}

/// All authored countdown blocks in one selected mission source.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionCountdownReport {
    countdowns: Vec<MissionCountdownBinding>,
}

impl MissionCountdownReport {
    /// Return countdown blocks in source stage order.
    #[must_use]
    pub fn countdowns(&self) -> &[MissionCountdownBinding] {
        &self.countdowns
    }
}

/// Bind countdown entries to one prior start in the same stage.
///
/// # Errors
///
/// Fails when entries appear without a prior start or a stage starts more than
/// one countdown block.
pub fn preflight_mission_countdowns(
    stages: &MissionStageSemanticReport,
) -> Result<MissionCountdownReport, String> {
    let mut countdowns = Vec::new();
    for stage in stages.stages() {
        let mut countdown: Option<MissionCountdownBinding> = None;
        for directive in stage.directives() {
            match directive {
                MissionStageDirective::StartCountdown {
                    source_ordinal,
                    sequence_id,
                    character_id,
                } => {
                    if countdown.is_some() {
                        return Err(
                            "mission stage starts more than one countdown"
                                .to_owned(),
                        );
                    }
                    countdown = Some(MissionCountdownBinding {
                        stage_source_ordinal: stage.source_ordinal(),
                        stage_sequence_ordinal: stage.sequence_ordinal(),
                        start_source_ordinal: *source_ordinal,
                        sequence_id: sequence_id.clone(),
                        character_id: character_id.clone(),
                        entries: Vec::new(),
                    });
                }
                MissionStageDirective::CountdownSequenceEntry {
                    source_ordinal,
                    token,
                    duration_milliseconds,
                } => {
                    let Some(active) = countdown.as_mut() else {
                        return Err(
                            "mission countdown entry precedes countdown start"
                                .to_owned(),
                        );
                    };
                    if *source_ordinal <= active.start_source_ordinal {
                        return Err(
                            "mission countdown entry order is contradictory"
                                .to_owned(),
                        );
                    }
                    active.entries.push(MissionCountdownEntryBinding {
                        source_ordinal: *source_ordinal,
                        token: token.clone(),
                        duration_milliseconds: *duration_milliseconds,
                    });
                }
                _ => {}
            }
        }
        if let Some(countdown) = countdown {
            countdowns.push(countdown);
        }
    }
    Ok(MissionCountdownReport { countdowns })
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_countdown/tests.rs"]
mod tests;
