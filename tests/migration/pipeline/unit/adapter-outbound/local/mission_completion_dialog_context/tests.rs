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
//   - Unit evidence for completion-dialog conversation grouping.
// - Must-Not:
//   - Treat the optional character as a speaker selector.
// - Allows:
//   - Verify participant package grouping and independent character binding.
// - Split-When:
//   - Dialogue line sequencing gains independently testable authority.
// - Merge-When:
//   - Final mission presentation tests own the same catalog invariants.
// - Summary:
//   - Completion-dialog context unit tests.
// - Description:
//   - Proves multi-participant and convinit groups resolve without heuristics.
// - Usage:
//   - Compiled with local outbound adapter unit tests.
// - Defaults:
//   - Multiple conversation groups or missing audio membership fail closed.
//

//! Unit evidence for completion-dialog conversation grouping.

use super::*;

fn dialog_row(
    package_id: &str,
    participant: &str,
    mode: &str,
    conversation: &str,
    audio_id: &str,
) -> String {
    let root = format!("extracted/dialog/{package_id}");
    let canonical_package_id = format!("extracted-dialog-{package_id}");
    let subcategory = format!(
        concat!(
            "dialog/{}/conversation/mission/level-07/",
            "{}/{}/default"
        ),
        participant,
        mode,
        conversation,
    );
    format!(
        concat!(
            "{{\"package_id\":\"{}\",\"package_root\":\"{}\",",
            "\"package_category\":\"dialog\",",
            "\"package_subcategory\":\"{}\",",
            "\"unit_count\":1,\"text_key_count\":0,",
            "\"unit_ids\":[\"{}\"],\"world_ids\":[],",
            "\"texture_ids\":[],\"material_ids\":[],\"model_ids\":[],",
            "\"physics_ids\":[],\"animation_ids\":[],\"scene_ids\":[],",
            "\"locator_ids\":[],\"camera_ids\":[],\"light_ids\":[],",
            "\"particle_ids\":[],\"controller_ids\":[],",
            "\"audio_ids\":[\"{}\"],\"movie_ids\":[],",
            "\"script_ids\":[],\"text_ids\":[],\"ui_ids\":[],",
            "\"metadata_ids\":[],\"error_ids\":[],",
            "\"source_unit_ids\":[],\"text_key_ids\":[],",
            "\"members\":[{{\"id\":\"{}\",\"role\":\"audio\",",
            "\"path\":\"{}/{}.wav\",\"type\":\"audio\",",
            "\"kind\":\"runtime-asset\",",
            "\"source_chunk_kind\":\"none\"}}],\"text_keys\":[]}}"
        ),
        canonical_package_id,
        root,
        subcategory,
        audio_id,
        audio_id,
        audio_id,
        root,
        audio_id,
    )
}

#[test]
fn package_binding_preserves_audio_id_path_alignment() -> Result<(), String> {
    let row_text = dialog_row(
        "order-homer",
        "homer",
        "noboxconv",
        "l7m3-order",
        "audio-z",
    )
    .replace("\"unit_count\":1", "\"unit_count\":2")
    .replace(
        "\"unit_ids\":[\"audio-z\"]",
        "\"unit_ids\":[\"audio-z\",\"audio-a\"]",
    )
    .replace(
        "\"audio_ids\":[\"audio-z\"]",
        "\"audio_ids\":[\"audio-z\",\"audio-a\"]",
    )
    .replace(
        concat!(
            "{\"id\":\"audio-z\",\"role\":\"audio\",",
            "\"path\":\"extracted/dialog/order-homer/audio-z.wav\",",
            "\"type\":\"audio\",\"kind\":\"runtime-asset\",",
            "\"source_chunk_kind\":\"none\"}"
        ),
        concat!(
            "{\"id\":\"audio-z\",\"role\":\"audio\",",
            "\"path\":\"extracted/dialog/order-homer/z.wav\",",
            "\"type\":\"audio\",\"kind\":\"runtime-asset\",",
            "\"source_chunk_kind\":\"none\"},",
            "{\"id\":\"audio-a\",\"role\":\"audio\",",
            "\"path\":\"extracted/dialog/order-homer/a.wav\",",
            "\"type\":\"audio\",\"kind\":\"runtime-asset\",",
            "\"source_chunk_kind\":\"none\"}"
        ),
    );
    let row = PhaseThreePackageRow::from_json_line(&row_text)
        .map_err(|error| error.to_string())?;
    let binding = compile_package_binding(&row, "homer")
        .map_err(|error| error.to_string())?;
    if binding.audio_ids() != ["audio-z", "audio-a"]
        || binding.audio_paths()
            != [
                "extracted/dialog/order-homer/z.wav",
                "extracted/dialog/order-homer/a.wav",
            ]
    {
        return Err(
            "completion dialogue detached audio ids from paths".to_owned(),
        );
    }
    Ok(())
}

#[test]
fn groups_participant_packages_for_one_conversation() -> Result<(), String> {
    let homer = dialog_row(
        "toxic-homer",
        "homer",
        "noboxconv",
        "l7m3-toxic",
        "audio-homer",
    );
    let frink = dialog_row(
        "toxic-frink",
        "frink",
        "noboxconv",
        "l7m3-toxic",
        "audio-frink",
    );
    let rows = format!("{frink}
{homer}");
    let index = PhaseThreePackageIndex::from_jsonl(&rows)
        .map_err(|error| error.to_string())?;
    let (mode, conversation, packages) =
        resolve_conversation(&index, 7, "toxic").map_err(|e| e.to_string())?;
    assert_eq!(mode, "noboxconv");
    assert_eq!(conversation, "l7m3-toxic");
    let [frink_package, homer_package] = packages.as_slice() else {
        return Err("conversation package count changed".to_owned());
    };
    assert_eq!(frink_package.participant_id(), "frink");
    assert_eq!(homer_package.participant_id(), "homer");
    assert_eq!(frink_package.audio_ids(), &["audio-frink"]);
    assert_eq!(homer_package.audio_ids(), &["audio-homer"]);
    Ok(())
}

#[test]
fn resolves_convinit_without_forcing_nobox_mode() -> Result<(), String> {
    let cletus = dialog_row(
        "pappy-cletus",
        "cletus",
        "convinit",
        "l7m3-pappy",
        "audio-cletus",
    );
    let index = PhaseThreePackageIndex::from_jsonl(&cletus)
        .map_err(|error| error.to_string())?;
    let (mode, conversation, packages) =
        resolve_conversation(&index, 7, "pappy").map_err(|e| e.to_string())?;
    assert_eq!(mode, "convinit");
    assert_eq!(conversation, "l7m3-pappy");
    assert_eq!(packages.len(), 1);
    Ok(())
}

#[test]
fn rejects_multiple_groups_for_same_dialogue() -> Result<(), String> {
    let first = dialog_row(
        "first",
        "homer",
        "noboxconv",
        "l7m3-toxic",
        "audio-first",
    );
    let second = dialog_row(
        "second",
        "frink",
        "convinit",
        "l7m3-toxic",
        "audio-second",
    );
    let rows = format!("{first}
{second}");
    let index = PhaseThreePackageIndex::from_jsonl(&rows)
        .map_err(|error| error.to_string())?;
    assert!(resolve_conversation(&index, 7, "toxic").is_err());
    Ok(())
}

#[test]
fn keeps_character_independent_from_audio_packages() -> Result<(), String> {
    let catalog = MissionReferenceCatalog::from_character_entries_for_tests(&[(
        "frink_m",
        "frink",
        "character-frink",
        "characters/frink/base-model",
    )]);
    let character = catalog.resolve_character("frink_m")?;
    let package = MissionCompletionDialogPackageBinding {
        participant_id: "homer".to_owned(),
        package_id: "toxic-homer".to_owned(),
        package_subcategory: concat!(
            "dialog/homer/conversation/mission/level-07/noboxconv/",
            "l7m3-toxic/default"
        )
        .to_owned(),
        audio_ids: vec!["audio-homer".to_owned()],
        audio_paths: vec![
            "extracted/dialog/toxic-homer/audio-homer.wav".to_owned(),
        ],
    };
    let binding = MissionCompletionDialogBinding {
        source_path: "game/scripts/missions/level07/m3i.mfk.json".to_owned(),
        owner_stage_source_ordinal: 60,
        owner_stage_sequence_ordinal: 2,
        source_ordinal: 65,
        level: 7,
        dialogue_id: "toxic".to_owned(),
        mode: "noboxconv".to_owned(),
        conversation_id: "l7m3-toxic".to_owned(),
        character: Some(character),
        packages: vec![package],
    };
    let report = MissionCompletionDialogReport {
        bindings: vec![binding],
    };
    let [binding] = report.bindings() else {
        return Err("completion-dialog binding count changed".to_owned());
    };
    assert_eq!(
        binding.source_path(),
        "game/scripts/missions/level07/m3i.mfk.json"
    );
    assert_eq!(binding.owner_stage_source_ordinal(), 60);
    assert_eq!(binding.owner_stage_sequence_ordinal(), 2);
    assert_eq!(binding.source_ordinal(), 65);
    assert_eq!(binding.level(), 7);
    assert_eq!(binding.dialogue_id(), "toxic");
    assert_eq!(binding.mode(), "noboxconv");
    assert_eq!(binding.conversation_id(), "l7m3-toxic");
    assert_eq!(
        binding
            .character()
            .map(MissionCharacterCatalogReference::participant_id),
        Some("frink")
    );
    let [package] = binding.packages() else {
        return Err("completion-dialog package count changed".to_owned());
    };
    assert_eq!(package.participant_id(), "homer");
    assert_eq!(package.package_id(), "toxic-homer");
    assert!(package
        .package_subcategory()
        .ends_with("l7m3-toxic/default"));
    assert_eq!(package.audio_ids(), &["audio-homer"]);
    assert_eq!(
        package.audio_paths(),
        &["extracted/dialog/toxic-homer/audio-homer.wav"]
    );
    Ok(())
}
