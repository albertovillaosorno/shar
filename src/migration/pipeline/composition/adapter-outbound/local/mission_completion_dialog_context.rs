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
//   - Canonical conversation-package grouping for mission completion dialogue.
// - Must-Not:
//   - Infer speaker, listener, line order, playback, or completion behavior.
// - Allows:
//   - Bind exact level/dialogue identity and optional character independently.
// - Split-When:
//   - Dialogue playback or line sequencing gains independent source authority.
// - Merge-When:
//   - Final mission presentation compilation owns this exact catalog binding.
// - Summary:
//   - Mission completion-dialog catalog context.
// - Description:
//   - Groups all canonical participant packages for one authored conversation.
// - Usage:
//   - Runs after mission snapshots, stage semantics, and package-index intake.
// - Defaults:
//   - Missing, ambiguous, malformed, or audio-less conversation groups fail.
//

//! Canonical completion-dialog conversation grouping.

use std::collections::BTreeSet;

use super::mission_locator_context::MissionLocatorScriptSnapshot;
use super::mission_music_context::source_level;
use crate::domain::{
    MissionCharacterCatalogReference, MissionReferenceCatalog,
    MissionStageDirective, PackageRole, PhaseThreePackageIndex,
    PhaseThreePackageRow, PipelineError, PipelineOutcome,
    compile_mission_scope_graphs, preflight_mission_stage_semantics,
};

/// One participant package contributing audio to a completion conversation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionCompletionDialogPackageBinding {
    participant_id: String,
    package_id: String,
    package_subcategory: String,
    audio_ids: Vec<String>,
    audio_paths: Vec<String>,
}

/// One authored completion dialogue bound to one canonical conversation group.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct MissionCompletionDialogBinding {
    source_path: String,
    owner_stage_source_ordinal: usize,
    owner_stage_sequence_ordinal: usize,
    source_ordinal: usize,
    level: u8,
    dialogue_id: String,
    mode: String,
    conversation_id: String,
    character: Option<MissionCharacterCatalogReference>,
    packages: Vec<MissionCompletionDialogPackageBinding>,
}

/// Reviewed completion-dialog bindings in source order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct MissionCompletionDialogReport {
    bindings: Vec<MissionCompletionDialogBinding>,
}

impl MissionCompletionDialogReport {
    #[cfg(test)]
    pub(super) fn bindings(&self) -> &[MissionCompletionDialogBinding] {
        &self.bindings
    }
}

impl MissionCompletionDialogBinding {
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
    pub(super) fn mode(&self) -> &str {
        &self.mode
    }

    #[cfg(test)]
    pub(super) fn conversation_id(&self) -> &str {
        &self.conversation_id
    }

    #[cfg(test)]
    pub(super) fn character(
        &self,
    ) -> Option<&MissionCharacterCatalogReference> {
        self.character.as_ref()
    }

    #[cfg(test)]
    pub(super) fn packages(&self) -> &[MissionCompletionDialogPackageBinding] {
        &self.packages
    }
}

impl MissionCompletionDialogPackageBinding {
    #[cfg(test)]
    pub(super) fn participant_id(&self) -> &str {
        &self.participant_id
    }

    #[cfg(test)]
    pub(super) fn package_id(&self) -> &str {
        &self.package_id
    }

    #[cfg(test)]
    pub(super) fn package_subcategory(&self) -> &str {
        &self.package_subcategory
    }

    #[cfg(test)]
    pub(super) fn audio_ids(&self) -> &[String] {
        &self.audio_ids
    }

    #[cfg(test)]
    pub(super) fn audio_paths(&self) -> &[String] {
        &self.audio_paths
    }
}

/// Bind all completion-dialog directives to canonical conversation packages.
///
/// # Errors
///
/// Returns an error when source levels, character references, conversation
/// groups, participant taxonomy, or audio membership are not exact.
pub(super) fn preflight_mission_completion_dialogs(
    index: &PhaseThreePackageIndex,
    characters: &MissionReferenceCatalog,
    snapshots: &[MissionLocatorScriptSnapshot],
) -> PipelineOutcome<MissionCompletionDialogReport> {
    let mut bindings = Vec::new();
    for snapshot in snapshots {
        let scopes = compile_mission_scope_graphs(snapshot.evidence())
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission completion-dialog scope failed: {error}"
                ))
            })?;
        let stages = preflight_mission_stage_semantics(&scopes)
            .map_err(|error| {
                PipelineError::new(format!(
                    "mission completion-dialog stage failed: {error}"
                ))
            })?;
        let mut directives = Vec::new();
        for stage in stages.stages() {
            for directive in stage.directives() {
                if let MissionStageDirective::CompletionDialog {
                    source_ordinal,
                    dialogue_id,
                    character_id,
                } = directive
                {
                    directives.push((
                        stage.source_ordinal(),
                        stage.sequence_ordinal(),
                        *source_ordinal,
                        dialogue_id,
                        character_id,
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
            source_ordinal,
            dialogue_id,
            character_id,
        ) in directives
        {
            let character = character_id
                .as_deref()
                .map(|id| characters.resolve_character(id))
                .transpose()
                .map_err(|error| {
                    PipelineError::new(format!(
                        "mission completion-dialog character failed: {error}"
                    ))
                })?;
            let (mode, conversation_id, packages) =
                resolve_conversation(index, level, dialogue_id)?;
            bindings.push(MissionCompletionDialogBinding {
                source_path: snapshot.source_path().to_owned(),
                owner_stage_source_ordinal,
                owner_stage_sequence_ordinal,
                source_ordinal,
                level,
                dialogue_id: dialogue_id.to_owned(),
                mode,
                conversation_id,
                character,
                packages,
            });
        }
    }
    Ok(MissionCompletionDialogReport { bindings })
}

fn resolve_conversation(
    index: &PhaseThreePackageIndex,
    level: u8,
    dialogue_id: &str,
) -> PipelineOutcome<(
    String,
    String,
    Vec<MissionCompletionDialogPackageBinding>,
)> {
    resolve_conversation_with_hint(index, level, dialogue_id, None)
}

pub(super) fn resolve_conversation_with_hint(
    index: &PhaseThreePackageIndex,
    level: u8,
    dialogue_id: &str,
    conversation_hint: Option<&str>,
) -> PipelineOutcome<(
    String,
    String,
    Vec<MissionCompletionDialogPackageBinding>,
)> {
    let level_id = format!("level-{level:02}");
    let suffix = format!("-{}/default", dialogue_id.to_ascii_lowercase());
    let mut groups = BTreeSet::<(String, String)>::new();
    let mut packages = Vec::new();
    for package in index.packages() {
        if package.category() != "dialog" {
            continue;
        }
        let Some((participant, candidate_level, mode, conversation)) =
            conversation_identity(package.subcategory())?
        else {
            continue;
        };
        if candidate_level != level_id
            || !package
                .subcategory()
                .to_ascii_lowercase()
                .ends_with(&suffix)
            || conversation_hint
                .is_some_and(|hint| !conversation.eq_ignore_ascii_case(hint))
        {
            continue;
        }
        let _was_new = groups.insert((
            mode.to_owned(),
            conversation.to_owned(),
        ));
        packages.push(compile_package_binding(package, participant)?);
    }
    let group_values = groups.into_iter().collect::<Vec<_>>();
    let [(mode, conversation_id)] = group_values.as_slice() else {
        return Err(PipelineError::new(format!(
            concat!(
                "completion dialogue `{}` has no unique ",
                "conversation group"
            ),
            dialogue_id
        )));
    };
    if packages.is_empty() {
        return Err(PipelineError::new(
            "completion dialogue conversation has no participant packages",
        ));
    }
    packages.sort_by(|left, right| {
        left.participant_id
            .cmp(&right.participant_id)
            .then_with(|| left.package_id.cmp(&right.package_id))
    });
    Ok((mode.clone(), conversation_id.clone(), packages))
}

fn conversation_identity(
    subcategory: &str,
) -> PipelineOutcome<Option<(&str, &str, &str, &str)>> {
    if !subcategory.contains("/conversation/mission/") {
        return Ok(None);
    }
    let segments = subcategory.split('/').collect::<Vec<_>>();
    let [
        "dialog",
        participant,
        "conversation",
        "mission",
        level,
        mode,
        conversation,
        "default",
    ] = segments.as_slice()
    else {
        return Err(PipelineError::new(
            "mission dialogue package taxonomy drifted",
        ));
    };
    if participant.is_empty()
        || conversation.is_empty()
        || !matches!(*mode, "convinit" | "noboxconv")
        || !level.starts_with("level-")
    {
        return Err(PipelineError::new(
            "mission dialogue package identity is malformed",
        ));
    }
    Ok(Some((participant, level, mode, conversation)))
}

fn compile_package_binding(
    package: &PhaseThreePackageRow,
    participant: &str,
) -> PipelineOutcome<MissionCompletionDialogPackageBinding> {
    let audio_ids = package.ids_for_role(PackageRole::Audio);
    if audio_ids.is_empty() {
        return Err(PipelineError::new(
            "completion dialogue participant package has no audio ids",
        ));
    }
    let id_set = audio_ids.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let mut audio_paths = package
        .members()
        .iter()
        .filter(|member| {
            member.role == PackageRole::Audio
                && id_set.contains(member.id.as_str())
        })
        .map(|member| member.path.clone())
        .collect::<Vec<_>>();
    if audio_paths.len() != audio_ids.len() {
        return Err(PipelineError::new(
            "completion dialogue audio ids do not match physical members",
        ));
    }
    audio_paths.sort();
    Ok(MissionCompletionDialogPackageBinding {
        participant_id: participant.to_owned(),
        package_id: package.package_id.clone(),
        package_subcategory: package.subcategory().to_owned(),
        audio_ids: audio_ids.to_vec(),
        audio_paths,
    })
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../../tests/migration/pipeline/unit/adapter-outbound/local/mission_completion_dialog_context/tests.rs"]
mod tests;
