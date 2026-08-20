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
//   - Canonical conversation grouping for objective `SetDialogueInfo`.
// - Must-Not:
//   - Infer speaker order, listener roles, playback, cameras, or progression.
// - Allows:
//   - Bind both authored characters and one exact conversation package group.
// - Split-When:
//   - Dialogue playback or line sequencing gains independent source authority.
// - Merge-When:
//   - Final mission presentation compilation owns this exact catalog binding.
// - Summary:
//   - Objective dialogue-info catalog context.
// - Description:
//   - Resolves authored dialogue setup into participant and audio provenance.
// - Usage:
//   - Runs after mission snapshots, objective semantics, and package intake.
// - Defaults:
//   - Missing characters or ambiguous conversation identities fail closed.
//

//! Canonical objective `SetDialogueInfo` conversation grouping.

use super::mission_completion_dialog_context::{
    MissionCompletionDialogPackageBinding, resolve_conversation_with_hint,
};
use super::mission_locator_context::MissionLocatorScriptSnapshot;
use super::mission_music_context::source_level;
use crate::domain::{
    MissionCharacterCatalogReference, MissionObjectiveDirective,
    MissionReferenceCatalog, PhaseThreePackageIndex, PipelineError,
    PipelineOutcome, compile_mission_scope_graphs,
    preflight_mission_objective_semantics,
};

/// One objective dialogue setup bound to canonical participants and audio
/// group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionDialogueInfoBinding {
    source_path: String,
    owner_stage_source_ordinal: usize,
    owner_stage_sequence_ordinal: usize,
    owner_objective_source_ordinal: usize,
    source_ordinal: usize,
    level: u8,
    dialogue_id: String,
    legacy_zero: String,
    player: MissionCharacterCatalogReference,
    npc: MissionCharacterCatalogReference,
    mode: String,
    conversation_id: String,
    packages: Vec<MissionCompletionDialogPackageBinding>,
}

/// Reviewed objective dialogue-info bindings in source order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct MissionDialogueInfoReport {
    bindings: Vec<MissionDialogueInfoBinding>,
}

impl MissionDialogueInfoReport {
    #[cfg(test)]
    pub(super) fn bindings(&self) -> &[MissionDialogueInfoBinding] {
        &self.bindings
    }
}

impl MissionDialogueInfoBinding {
    #[cfg(test)]
    pub(super) fn source_path(&self) -> &str {
        &self.source_path
    }

    #[cfg(test)]
    pub(super) const fn owner_stage_source_ordinal(&self) -> usize {
        self.owner_stage_source_ordinal
    }

    #[cfg(test)]
    pub(super) const fn owner_stage_sequence_ordinal(&self) -> usize {
        self.owner_stage_sequence_ordinal
    }

    #[cfg(test)]
    pub(super) const fn owner_objective_source_ordinal(&self) -> usize {
        self.owner_objective_source_ordinal
    }

    #[cfg(test)]
    pub(super) const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    #[cfg(test)]
    pub(super) const fn level(&self) -> u8 {
        self.level
    }

    #[cfg(test)]
    pub(super) fn dialogue_id(&self) -> &str {
        &self.dialogue_id
    }

    #[cfg(test)]
    pub(super) fn legacy_zero(&self) -> &str {
        &self.legacy_zero
    }

    #[cfg(test)]
    pub(super) const fn player(&self) -> &MissionCharacterCatalogReference {
        &self.player
    }

    #[cfg(test)]
    pub(super) const fn npc(&self) -> &MissionCharacterCatalogReference {
        &self.npc
    }

    #[cfg(test)]
    pub(super) fn mode(&self) -> &str {
        &self.mode
    }

    #[cfg(test)]
    pub(super) fn conversation_id(&self) -> &str {
        &self.conversation_id
    }

    #[cfg(test)]
    pub(super) const fn package_count(&self) -> usize {
        self.packages.len()
    }
}

/// Bind every typed objective `SetDialogueInfo` to canonical catalog evidence.
///
/// # Errors
///
/// Returns an error when participant references, source level, or conversation
/// grouping cannot be resolved exactly.
pub(super) fn preflight_mission_dialogue_info(
    index: &PhaseThreePackageIndex,
    characters: &MissionReferenceCatalog,
    snapshots: &[MissionLocatorScriptSnapshot],
) -> PipelineOutcome<MissionDialogueInfoReport> {
    let mut bindings = Vec::new();
    for snapshot in snapshots {
        let scopes = compile_mission_scope_graphs(snapshot.evidence())
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission dialogue-info scope failed: {error}"
                ))
            })?;
        let objectives = preflight_mission_objective_semantics(&scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission dialogue-info objective failed: {error}"
                ))
            })?;
        let mut directives = Vec::new();
        for objective in objectives.objectives() {
            for directive in objective.directives() {
                if let MissionObjectiveDirective::DialogueInfo {
                    source_ordinal,
                    player_character_id,
                    npc_character_id,
                    dialogue_id,
                    legacy_zero,
                } = directive
                {
                    directives.push((
                        objective.owner_stage_source_ordinal(),
                        objective.owner_stage_sequence_ordinal(),
                        objective.source_ordinal(),
                        *source_ordinal,
                        player_character_id,
                        npc_character_id,
                        dialogue_id,
                        legacy_zero,
                    ));
                }
            }
        }
        if directives.is_empty() {
            continue;
        }
        let level = source_level(snapshot.source_path())?;
        for (
            owner_stage_source_ordinal,
            owner_stage_sequence_ordinal,
            owner_objective_source_ordinal,
            source_ordinal,
            player_character_id,
            npc_character_id,
            dialogue_id,
            legacy_zero,
        ) in directives
        {
            let player = resolve_character(
                characters,
                player_character_id,
                "player",
            )?;
            let npc = resolve_character(characters, npc_character_id, "NPC")?;
            let hint = street_race_hint(
                snapshot.source_path(),
                level,
                dialogue_id,
            )?;
            let (mode, conversation_id, packages) =
                resolve_conversation_with_hint(
                    index,
                    level,
                    dialogue_id,
                    hint.as_deref(),
                )?;
            bindings.push(MissionDialogueInfoBinding {
                source_path: snapshot.source_path().to_owned(),
                owner_stage_source_ordinal,
                owner_stage_sequence_ordinal,
                owner_objective_source_ordinal,
                source_ordinal,
                level,
                dialogue_id: dialogue_id.to_owned(),
                legacy_zero: legacy_zero.to_owned(),
                player,
                npc,
                mode,
                conversation_id,
                packages,
            });
        }
    }
    Ok(MissionDialogueInfoReport { bindings })
}

fn resolve_character(
    catalog: &MissionReferenceCatalog,
    source_id: &str,
    role: &str,
) -> PipelineOutcome<MissionCharacterCatalogReference> {
    catalog.resolve_character(source_id).map_err(|error| {
        PipelineError::new(format!(
            "mission dialogue-info {role} character failed: {error}"
        ))
    })
}

fn street_race_hint(
    source_path: &str,
    level: u8,
    dialogue_id: &str,
) -> PipelineOutcome<Option<String>> {
    let normalized = source_path.replace(char::from(92), "/");
    let file = normalized.rsplit('/').next().ok_or_else(|| {
        PipelineError::new("mission dialogue-info source path has no filename")
    })?;
    let Some(stem) = file.strip_suffix(".mfk.json") else {
        return Err(PipelineError::new(
            "mission dialogue-info source filename is not canonical",
        ));
    };
    let Some(race) = stem
        .strip_prefix("sr")
        .and_then(|value| value.strip_suffix('i'))
    else {
        return Ok(None);
    };
    if !matches!(race, "1" | "2" | "3") {
        return Err(PipelineError::new(
            "mission dialogue-info street-race source id is invalid",
        ));
    }
    Ok(Some(format!(
        "l{level}r{race}-{}",
        dialogue_id.to_ascii_lowercase()
    )))
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/mission_dialogue_info_context/tests.rs"]
mod tests;
