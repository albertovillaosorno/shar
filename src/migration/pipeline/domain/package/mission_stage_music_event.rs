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
//   - Source-backed stage-start music event channel and key-transform identity.
// - Must-Not:
//   - Compute legacy key values or infer playback, mix, or transition policy.
// - Allows:
//   - Bind authored event tokens to their reviewed runtime delivery channel.
// - Split-When:
//   - A deterministic legacy-key implementation gains independent consumers.
// - Merge-When:
//   - Final mission presentation compilation owns this exact evidence.
// - Summary:
//   - Stage-start mission music event semantic binding.
// - Description:
//   - Preserves the authored token and reviewed runtime transport contract.
// - Usage:
//   - Runs after typed mission stage semantic compilation.
// - Defaults:
//   - Nonportable or contradictory source evidence fails closed.
//

//! Source-backed stage-start mission music event bindings.

use super::{MissionStageDirective, MissionStageSemanticReport};

/// Reviewed runtime channel receiving one stage-start music event key.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionStageMusicEventChannel {
    /// Mission-drama event emitted when the owning stage initializes.
    MissionDrama,
}

/// Reviewed transform required before the event token reaches its channel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MissionStageMusicEventKeyTransform {
    /// Legacy case-insensitive 32-bit key transform.
    LegacyCaseInsensitiveKey32,
}

/// One authored stage-start music event bound to its reviewed runtime route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageMusicEventBinding {
    stage_source_ordinal: usize,
    stage_sequence_ordinal: usize,
    source_ordinal: usize,
    event_id: String,
    channel: MissionStageMusicEventChannel,
    key_transform: MissionStageMusicEventKeyTransform,
}

impl MissionStageMusicEventBinding {
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

    /// Return the source `StageStartMusicEvent` ordinal.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the exact authored event token.
    #[must_use]
    pub fn event_id(&self) -> &str {
        &self.event_id
    }

    /// Return the reviewed runtime delivery channel.
    #[must_use]
    pub const fn channel(&self) -> MissionStageMusicEventChannel {
        self.channel
    }

    /// Return the reviewed pre-delivery key transform identity.
    #[must_use]
    pub const fn key_transform(&self) -> MissionStageMusicEventKeyTransform {
        self.key_transform
    }
}

/// All authored stage-start music events in one selected mission source.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionStageMusicEventReport {
    bindings: Vec<MissionStageMusicEventBinding>,
}

impl MissionStageMusicEventReport {
    /// Return bindings in authored stage and source order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionStageMusicEventBinding] {
        &self.bindings
    }
}

/// Bind authored stage-start music events to the reviewed runtime channel.
///
/// # Errors
///
/// Fails when an event precedes its owning stage or uses a nonportable token.
pub fn preflight_mission_stage_music_events(
    stages: &MissionStageSemanticReport,
) -> Result<MissionStageMusicEventReport, String> {
    let mut bindings = Vec::new();
    for stage in stages.stages() {
        for directive in stage.directives() {
            let MissionStageDirective::StageStartMusicEvent {
                source_ordinal,
                event_id,
            } = directive
            else {
                continue;
            };
            if *source_ordinal <= stage.source_ordinal() {
                return Err(
                    "mission stage music event precedes its owning stage"
                        .to_owned(),
                );
            }
            if event_id.is_empty()
                || !event_id.is_ascii()
                || event_id.bytes().any(|byte| byte.is_ascii_control())
            {
                return Err(
                    "mission stage music event token is not portable"
                        .to_owned(),
                );
            }
            bindings.push(MissionStageMusicEventBinding {
                stage_source_ordinal: stage.source_ordinal(),
                stage_sequence_ordinal: stage.sequence_ordinal(),
                source_ordinal: *source_ordinal,
                event_id: event_id.clone(),
                channel: MissionStageMusicEventChannel::MissionDrama,
                key_transform: MissionStageMusicEventKeyTransform::
                    LegacyCaseInsensitiveKey32,
            });
        }
    }
    Ok(MissionStageMusicEventReport { bindings })
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_stage_music_event/tests.rs"]
mod tests;
