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
//   - Canonical localization-key binding for authored stage message indices.
// - Must-Not:
//   - Read localized strings or infer presentation, timing, or unlock policy.
// - Allows:
//   - Resolve reviewed message namespaces through package-index text mirrors.
// - Split-When:
//   - Localized text payload compilation gains an independent lifecycle.
// - Merge-When:
//   - Final mission asset compilation owns these exact key references.
// - Summary:
//   - Mission stage-message localization reference preflight.
// - Description:
//   - Converts typed stage message indices into exact derived text-key
//     evidence.
// - Usage:
//   - Runs after stage semantic compilation and package-index intake.
// - Defaults:
//   - Missing or multiply-published localization keys fail closed.
//

//! Canonical localization-key binding for mission stage messages.

use std::collections::BTreeMap;

use super::{
    MissionStageDirective, MissionStageMessageKind, MissionStageSemanticReport,
    PhaseThreePackageIndex,
};

#[derive(Clone, Debug, Eq, PartialEq)]
struct TextKeyCatalogEntry {
    id: String,
    source_unit_id: String,
    package_id: String,
    subcategory: String,
}

/// One authored stage message bound to one canonical derived text key.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissionStageMessageReferenceBinding {
    stage_source_ordinal: usize,
    stage_sequence_ordinal: usize,
    source_ordinal: usize,
    kind: MissionStageMessageKind,
    index: u16,
    key: String,
    text_key_id: String,
    source_unit_id: String,
    package_id: String,
    package_subcategory: String,
}

impl MissionStageMessageReferenceBinding {
    /// Return source ordinal of the owning `AddStage` command.
    #[must_use]
    pub const fn stage_source_ordinal(&self) -> usize {
        self.stage_source_ordinal
    }

    /// Return dense authored stage order.
    #[must_use]
    pub const fn stage_sequence_ordinal(&self) -> usize {
        self.stage_sequence_ordinal
    }

    /// Return source ordinal of `SetStageMessageIndex`.
    #[must_use]
    pub const fn source_ordinal(&self) -> usize {
        self.source_ordinal
    }

    /// Return the reviewed stage-dependent message namespace.
    #[must_use]
    pub const fn kind(&self) -> MissionStageMessageKind {
        self.kind
    }

    /// Return exact authored numeric message index.
    #[must_use]
    pub const fn index(&self) -> u16 {
        self.index
    }

    /// Return exact localization key resolved from the authored index.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Return canonical derived text-key id.
    #[must_use]
    pub fn text_key_id(&self) -> &str {
        &self.text_key_id
    }

    /// Return physical source unit that published this text key.
    #[must_use]
    pub fn source_unit_id(&self) -> &str {
        &self.source_unit_id
    }

    /// Return package containing the canonical derived text key.
    #[must_use]
    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    /// Return exact package subcategory for the text key.
    #[must_use]
    pub fn package_subcategory(&self) -> &str {
        &self.package_subcategory
    }
}

/// Canonical stage-message text-key bindings for one mission source.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct MissionStageMessageReferenceReport {
    bindings: Vec<MissionStageMessageReferenceBinding>,
}

impl MissionStageMessageReferenceReport {
    /// Return bindings in authored stage/directive order.
    #[must_use]
    pub fn bindings(&self) -> &[MissionStageMessageReferenceBinding] {
        &self.bindings
    }
}

/// Bind every typed stage message index to one exact package-index text key.
///
/// # Errors
///
/// Returns an error when a required key is absent or published more than once.
pub fn preflight_mission_stage_message_references(
    index: &PhaseThreePackageIndex,
    stages: &MissionStageSemanticReport,
) -> Result<MissionStageMessageReferenceReport, String> {
    let catalog = build_text_key_catalog(index);
    let mut bindings = Vec::new();
    for stage in stages.stages() {
        for directive in stage.directives() {
            let MissionStageDirective::MessageIndex {
                source_ordinal,
                kind,
                index,
                ..
            } = directive
            else {
                continue;
            };
            let key = message_key(*kind, *index);
            let Some(entries) = catalog.get(&key) else {
                return Err(format!(
                    "stage message localization key is missing: {key}"
                ));
            };
            let [entry] = entries.as_slice() else {
                return Err(format!(
                    "stage message localization key is ambiguous: {key}"
                ));
            };
            bindings.push(MissionStageMessageReferenceBinding {
                stage_source_ordinal: stage.source_ordinal(),
                stage_sequence_ordinal: stage.sequence_ordinal(),
                source_ordinal: *source_ordinal,
                kind: *kind,
                index: *index,
                key,
                text_key_id: entry.id.clone(),
                source_unit_id: entry.source_unit_id.clone(),
                package_id: entry.package_id.clone(),
                package_subcategory: entry.subcategory.clone(),
            });
        }
    }
    Ok(MissionStageMessageReferenceReport { bindings })
}

fn build_text_key_catalog(
    index: &PhaseThreePackageIndex,
) -> BTreeMap<String, Vec<TextKeyCatalogEntry>> {
    let mut catalog = BTreeMap::<String, Vec<TextKeyCatalogEntry>>::new();
    for package in index.packages() {
        for text_key in package.text_keys() {
            catalog
                .entry(text_key.key.clone())
                .or_default()
                .push(TextKeyCatalogEntry {
                    id: text_key.id.clone(),
                    source_unit_id: text_key.source_unit_id.clone(),
                    package_id: package.package_id.clone(),
                    subcategory: text_key.subcategory.clone(),
                });
        }
    }
    catalog
}

fn message_key(kind: MissionStageMessageKind, index: u16) -> String {
    match kind {
        MissionStageMessageKind::Objective => {
            format!("MISSION_OBJECTIVE_{index:02}")
        },
        MissionStageMessageKind::Locked => format!("INGAME_MESSAGE_{index:02}"),
    }
}

#[cfg(test)]
// jig-ignore-next-line: exact Rust test-module path is indivisible.
#[path = "../../../../../tests/migration/pipeline/unit/domain/package/mission_stage_message_reference/tests.rs"]
mod tests;
