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
//   - Unit evidence for objective dialogue-info catalog context.
// - Must-Not:
//   - Infer dialogue playback or participant speaking order.
// - Allows:
//   - Verify street-race disambiguation and exact binding projection.
// - Split-When:
//   - Another ambiguous mission family needs independent source mapping.
// - Merge-When:
//   - Dialogue presentation tests own the same catalog invariants.
// - Summary:
//   - Objective dialogue-info context unit tests.
// - Description:
//   - Proves only street-race source ids add a conversation identity hint.
// - Usage:
//   - Compiled with local outbound adapter unit tests.
// - Defaults:
//   - Invalid street-race source ids fail closed.
//

//! Unit evidence for objective dialogue-info catalog context.

use super::*;

#[test]
fn derives_only_reviewed_street_race_hints() -> Result<(), String> {
    assert_eq!(
        street_race_hint(
            "game/scripts/missions/level02/sr2i.mfk.json",
            2,
            "success",
        )
        .map_err(|error| error.to_string())?
        .as_deref(),
        Some("l2r2-success")
    );
    assert_eq!(
        street_race_hint(
            "game/scripts/missions/level02/m2i.mfk.json",
            2,
            "frog",
        )
        .map_err(|error| error.to_string())?,
        None
    );
    assert!(street_race_hint("level02/sr4i.mfk.json", 2, "success").is_err());
    Ok(())
}

#[test]
fn preserves_participants_and_identity() -> Result<(), String> {
    let catalog = MissionReferenceCatalog::from_character_entries_for_tests(&[
        (
            "patty",
            "patty",
            "character-patty",
            "characters/patty/base-model",
        ),
        (
            "homer",
            "homer",
            "character-homer",
            "characters/homer/base-model",
        ),
    ]);
    let player = catalog
        .resolve_character("patty")
        .map_err(|error| error.to_string())?;
    let npc = catalog
        .resolve_character("homer")
        .map_err(|error| error.to_string())?;
    let binding = MissionDialogueInfoBinding {
        source_path: "game/scripts/missions/level02/sr2i.mfk.json".to_owned(),
        source_ordinal: 67,
        level: 2,
        dialogue_id: "success".to_owned(),
        legacy_zero: "0".to_owned(),
        player,
        npc,
        mode: "convinit".to_owned(),
        conversation_id: "l2r2-success".to_owned(),
        packages: Vec::new(),
    };
    let report = MissionDialogueInfoReport {
        bindings: vec![binding],
    };
    let [binding] = report.bindings() else {
        return Err("dialogue-info binding count changed".to_owned());
    };
    assert!(binding.source_path().ends_with("level02/sr2i.mfk.json"));
    assert_eq!(binding.source_ordinal(), 67);
    assert_eq!(binding.level(), 2);
    assert_eq!(binding.dialogue_id(), "success");
    assert_eq!(binding.legacy_zero(), "0");
    assert_eq!(binding.player().participant_id(), "patty");
    assert_eq!(binding.npc().participant_id(), "homer");
    assert_eq!(binding.mode(), "convinit");
    assert_eq!(binding.conversation_id(), "l2r2-success");
    assert_eq!(binding.package_count(), 0);
    Ok(())
}
