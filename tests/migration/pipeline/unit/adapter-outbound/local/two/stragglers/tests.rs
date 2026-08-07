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
//   - Tests unit tests.
// - Must-Not:
//   - Own production behavior or broaden the tested API surface.
// - Allows:
//   - Private test fixtures and assertions for the owning source module.
// - Split-When:
//   - Split when an independent fixture family gains separate ownership.
// - Merge-When:
//   - Merge when another test module owns the identical evidence.
// - Summary:
//   - Tests unit tests.
// - Description:
//   - Preserves unit-test access through a test-only path module.
// - Usage:
//   - Included only by the owning source module under cfg(test).
// - Defaults:
//   - Test setup and assertions fail explicitly.
//

// cspell:ignore selectmission closemission addstage closestage addobjective
// cspell:ignore closeobjective
//! Tests unit tests.

use std::path::Path;

use super::decode_straggler_text;

#[test]
fn decodes_windows_1252_text_stragglers() {
    let result = decode_straggler_text(
        b"Logitech\xae Force",
        Path::new("synthetic/era.txt"),
        "txt",
    );
    assert!(
        result.as_deref() == Ok("Logitech\u{ae} Force"),
        "era Windows-1252 bytes must decode deterministically"
    );
}

#[test]
fn rejects_undefined_windows_1252_text_stragglers() {
    let result = decode_straggler_text(
        &[0x81_u8],
        Path::new("synthetic/invalid.txt"),
        "txt",
    );
    assert!(
        result.is_err(),
        "bytes Windows-1252 leaves undefined must fail closed"
    );
}

#[test]
fn mission_v3_renderer_matches_semantic_preflight() -> Result<(), String> {
    let source = concat!(
        "SelectMission(\"m1\");\n",
        "AddStage(0);\n",
        "AddObjective(\"goto\");\n",
        "CloseObjective();\n",
        "CloseStage();\n",
        "CloseMission();\n",
    );
    let rendered = super::semantic_json_from_text(
        Path::new("scripts/missions/level01/m1i.mfk"),
        "mfk",
        source.as_bytes(),
        source,
    );
    let evidence = crate::domain::preflight_mission_script(&rendered)?;
    if evidence.statement_count() != 6 || evidence.invocations().len() != 6 {
        return Err(
            "rendered mission evidence changed during preflight".to_owned()
        );
    }
    Ok(())
}

fn repository_root() -> Result<std::path::PathBuf, String> {
    let mut current = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    loop {
        if current.join("TODO.md").is_file()
            && current.join("game/scripts/missions").is_dir()
        {
            return Ok(current);
        }
        if !current.pop() {
            return Err("repository root was not found".to_owned());
        }
    }
}

fn mission_sources(root: &Path) -> Result<Vec<std::path::PathBuf>, String> {
    let mut pending = vec![root.join("game/scripts/missions")];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in
            std::fs::read_dir(&directory).map_err(|error| error.to_string())?
        {
            let path = entry.map_err(|error| error.to_string())?.path();
            if path.is_dir() {
                pending.push(path);
            } else if path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| extension.eq_ignore_ascii_case("mfk"))
            {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

#[test]
fn repository_mission_corpus_passes_semantic_registries() -> Result<(), String>
{
    let root = repository_root()?;
    let game_root = root.join("game");
    let sources = mission_sources(&root)?;
    if sources.is_empty() {
        return Err("repository mission corpus is empty".to_owned());
    }
    let mut objective_count = 0usize;
    let mut condition_count = 0usize;
    let mut adaptation_count = 0usize;
    let mut empty_placeholder_count = 0usize;
    for source_path in &sources {
        let bytes =
            std::fs::read(source_path).map_err(|error| error.to_string())?;
        if bytes.is_empty() {
            empty_placeholder_count = empty_placeholder_count.saturating_add(1);
            continue;
        }
        let relative = source_path
            .strip_prefix(&game_root)
            .map_err(|_error| "mission source escaped game root".to_owned())?;
        let text = decode_straggler_text(&bytes, relative, "mfk")
            .map_err(|error| error.to_string())?;
        let rendered = super::semantic_json_from_text(
            relative,
            "mfk",
            &bytes,
            text.as_ref(),
        );
        let evidence = crate::domain::preflight_mission_script(&rendered)
            .map_err(|error| format!("{}: {error}", relative.display()))?;
        let objectives = crate::domain::preflight_mission_objectives(&evidence)
            .map_err(|error| format!("{}: {error}", relative.display()))?;
        drop(
            crate::domain::preflight_mission_objective_commands(&evidence)
                .map_err(|error| format!("{}: {error}", relative.display()))?,
        );
        let conditions = crate::domain::preflight_mission_conditions(&evidence)
            .map_err(|error| format!("{}: {error}", relative.display()))?;
        drop(
            crate::domain::preflight_mission_condition_commands(&evidence)
                .map_err(|error| format!("{}: {error}", relative.display()))?,
        );
        objective_count =
            objective_count.saturating_add(objectives.objectives().len());
        condition_count =
            condition_count.saturating_add(conditions.conditions().len());
        adaptation_count =
            adaptation_count.saturating_add(evidence.adaptations().len());
    }
    if objective_count != 611
        || condition_count != 408
        || adaptation_count != 2
        || empty_placeholder_count != 8
    {
        return Err(format!(
            concat!(
                "mission corpus inventory changed: files={} objectives={} ",
                "conditions={} adaptations={} empty_placeholders={}"
            ),
            sources.len(),
            objective_count,
            condition_count,
            adaptation_count,
            empty_placeholder_count,
        ));
    }
    Ok(())
}
