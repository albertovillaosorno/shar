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

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{
    decode_straggler_text, normalize_game_stragglers,
    publish_generated_directory, recover_generated_transaction,
};

static TRANSACTION_CASE_ID: AtomicUsize = AtomicUsize::new(0);

fn transaction_case_root(label: &str) -> Result<PathBuf, String> {
    let ordinal = TRANSACTION_CASE_ID.fetch_add(1, Ordering::Relaxed);
    let root = repository_root()?.join(".temp").join("tests").join(format!(
        "stragglers-{label}-{}-{ordinal}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    Ok(root)
}

fn remove_transaction_case(root: &Path) -> Result<(), String> {
    if root.exists() {
        fs::remove_dir_all(root).map_err(|error| error.to_string())?;
    }
    Ok(())
}

#[test]
fn failed_straggler_normalization_preserves_accepted_root() -> Result<(), String>
{
    let root = transaction_case_root("failure-preserves")?;
    let game = root.join("game-source");
    let extracted = root.join("extracted");
    fs::create_dir_all(&game).map_err(|error| error.to_string())?;
    fs::create_dir_all(extracted.join("game"))
        .map_err(|error| error.to_string())?;
    fs::write(game.join("broken.rsd"), b"not-rsd")
        .map_err(|error| error.to_string())?;
    fs::write(extracted.join("game/accepted.txt"), b"accepted")
        .map_err(|error| error.to_string())?;

    let result = normalize_game_stragglers(&game, &extracted);
    if result.is_ok() {
        return Err("invalid RSD unexpectedly normalized".to_owned());
    }
    if fs::read(extracted.join("game/accepted.txt"))
        .map_err(|error| error.to_string())?
        != b"accepted"
    {
        return Err("failed normalization changed accepted evidence".to_owned());
    }
    if extracted.join(".game.straggler-staging").exists()
        || extracted.join(".game.straggler-backup").exists()
    {
        return Err("failed normalization left transaction residue".to_owned());
    }
    remove_transaction_case(&root)
}

#[test]
fn straggler_publication_replaces_root_and_removes_backup() -> Result<(), String>
{
    let root = transaction_case_root("publish")?;
    let output = root.join("game");
    let staging = root.join(".game.straggler-staging");
    let backup = root.join(".game.straggler-backup");
    fs::create_dir_all(&output).map_err(|error| error.to_string())?;
    fs::create_dir_all(&staging).map_err(|error| error.to_string())?;
    fs::write(output.join("old.txt"), b"old")
        .map_err(|error| error.to_string())?;
    fs::write(staging.join("new.txt"), b"new")
        .map_err(|error| error.to_string())?;

    publish_generated_directory(&staging, &output, &backup)
        .map_err(|error| error.to_string())?;
    if output.join("old.txt").exists()
        || fs::read(output.join("new.txt"))
            .map_err(|error| error.to_string())?
            != b"new"
        || staging.exists()
        || backup.exists()
    {
        return Err("straggler publication inventory is not exact".to_owned());
    }
    remove_transaction_case(&root)
}

#[test]
fn failed_straggler_publication_restores_accepted_root() -> Result<(), String> {
    let root = transaction_case_root("publish-rollback")?;
    let output = root.join("game");
    let staging = root.join(".game.straggler-staging");
    let backup = root.join(".game.straggler-backup");
    fs::create_dir_all(&output).map_err(|error| error.to_string())?;
    fs::write(output.join("accepted.txt"), b"accepted")
        .map_err(|error| error.to_string())?;

    let result = publish_generated_directory(&staging, &output, &backup);
    if result.is_ok() {
        return Err(
            "missing staging directory unexpectedly published".to_owned()
        );
    }
    if fs::read(output.join("accepted.txt"))
        .map_err(|error| error.to_string())?
        != b"accepted"
        || backup.exists()
    {
        return Err(
            "failed publication did not restore accepted root".to_owned()
        );
    }
    remove_transaction_case(&root)
}

#[test]
fn straggler_recovery_restores_backup_before_staging_cleanup()
-> Result<(), String> {
    let root = transaction_case_root("recover")?;
    let output = root.join("game");
    let staging = root.join(".game.straggler-staging");
    let backup = root.join(".game.straggler-backup");
    fs::create_dir_all(&backup).map_err(|error| error.to_string())?;
    fs::create_dir_all(&staging).map_err(|error| error.to_string())?;
    fs::write(backup.join("accepted.txt"), b"accepted")
        .map_err(|error| error.to_string())?;
    fs::write(staging.join("partial.txt"), b"partial")
        .map_err(|error| error.to_string())?;

    recover_generated_transaction(&output, &staging, &backup)
        .map_err(|error| error.to_string())?;
    if fs::read(output.join("accepted.txt"))
        .map_err(|error| error.to_string())?
        != b"accepted"
        || staging.exists()
        || backup.exists()
    {
        return Err(
            "straggler transaction recovery did not restore accepted root"
                .to_owned(),
        );
    }
    remove_transaction_case(&root)
}

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

fn repository_root() -> Result<PathBuf, String> {
    let mut current = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
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

fn mission_sources(root: &Path) -> Result<Vec<PathBuf>, String> {
    let mut pending = vec![root.join("game/scripts/missions")];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in
            fs::read_dir(&directory).map_err(|error| error.to_string())?
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
    let mut mission_graph_count = 0usize;
    let mut unscoped_command_count = 0usize;
    let mut mission_command_count = 0usize;
    let mut stage_command_count = 0usize;
    let mut objective_command_count = 0usize;
    let mut condition_command_count = 0usize;
    let mut stage_count = 0usize;
    let mut stage_condition_count = 0usize;
    let mut objective_condition_count = 0usize;
    let mut unavailable_objective_count = 0usize;
    let mut empty_placeholder_count = 0usize;
    for source_path in &sources {
        let bytes = fs::read(source_path).map_err(|error| error.to_string())?;
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
        let scopes = crate::domain::compile_mission_scope_graphs(&evidence)
            .map_err(|error| format!("{}: {error}", relative.display()))?;
        mission_graph_count =
            mission_graph_count.saturating_add(scopes.missions().len());
        unscoped_command_count = unscoped_command_count
            .saturating_add(scopes.unscoped_commands().len());
        for mission in scopes.missions() {
            mission_command_count =
                mission_command_count.saturating_add(mission.commands().len());
            stage_count = stage_count.saturating_add(mission.stages().len());
            for stage in mission.stages() {
                stage_command_count =
                    stage_command_count.saturating_add(stage.commands().len());
                objective_command_count = objective_command_count
                    .saturating_add(stage.objective().commands().len());
                if !stage.objective().binding().is_mapped() {
                    unavailable_objective_count =
                        unavailable_objective_count.saturating_add(1);
                }
                for condition in stage.conditions() {
                    condition_command_count = condition_command_count
                        .saturating_add(condition.commands().len());
                    match condition.scope() {
                        crate::domain::MissionConditionScope::Stage => {
                            stage_condition_count =
                                stage_condition_count.saturating_add(1);
                        },
                        crate::domain::MissionConditionScope::Objective => {
                            objective_condition_count =
                                objective_condition_count.saturating_add(1);
                        },
                    }
                }
            }
        }
    }
    if objective_count != 611
        || condition_count != 408
        || adaptation_count != 2
        || mission_graph_count != 154
        || unscoped_command_count != 7705
        || mission_command_count != 811
        || stage_command_count != 2454
        || objective_command_count != 3605
        || condition_command_count != 375
        || stage_count != 611
        || stage_condition_count != 402
        || objective_condition_count != 6
        || unavailable_objective_count != 2
        || empty_placeholder_count != 8
    {
        return Err(format!(
            concat!(
                "mission corpus inventory changed: files={} objectives={} ",
                "conditions={} adaptations={} missions={} unscoped_commands={} ",
                "mission_commands={} stage_commands={} ",
                "objective_commands={} condition_commands={} stages={} ",
                "stage_conditions={} objective_conditions={} ",
                "unavailable_objectives={} empty_placeholders={}"
            ),
            sources.len(),
            objective_count,
            condition_count,
            adaptation_count,
            mission_graph_count,
            unscoped_command_count,
            mission_command_count,
            stage_command_count,
            objective_command_count,
            condition_command_count,
            stage_count,
            stage_condition_count,
            objective_condition_count,
            unavailable_objective_count,
            empty_placeholder_count,
        ));
    }
    Ok(())
}
