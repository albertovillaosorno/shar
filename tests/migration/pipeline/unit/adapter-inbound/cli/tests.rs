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

//! Tests unit tests.

use std::fs;

use schoenwald_cli::{CliProgram, ExitStatus, OutputStream};

use crate::domain::PipelineReport;
use crate::manifest_paths::FBX_MANIFEST_PATH;
use crate::workspace::{
    EXTRACTED_WORKSPACE_ROOT, FBX_WORKSPACE_ROOT, UNREAL_STAGING_WORKSPACE_ROOT,
};

use super::{PipelineCli, USAGE, render_success};

#[test]
fn generated_workspace_defaults_are_cached() {
    assert_eq!(EXTRACTED_WORKSPACE_ROOT, ".cache/pipeline/extracted");
    assert_eq!(FBX_WORKSPACE_ROOT, ".cache/pipeline/fbx-assets");
    assert_eq!(FBX_MANIFEST_PATH, "game/manifest/fbx.jsonl");
    assert_eq!(
        UNREAL_STAGING_WORKSPACE_ROOT,
        ".cache/pipeline/unreal-staging",
    );
}

#[test]
fn complete_fbx_catalog_uses_default_output_when_omitted() {
    let outcome = super::run_complete_fbx_catalog(&[
        "missing-index.jsonl".to_owned(),
    ]);
    assert_eq!(outcome.status(), ExitStatus::Failure);
    assert!(outcome.output().iter().all(|chunk| {
        !chunk.text().contains("missing required output directory")
    }));
}

#[test]
fn manifest_rejects_extra_positionals() -> Result<(), String> {
    let outcome = super::run_fbx_manifest(&[
        "missing-index.jsonl".to_owned(),
        "type:model".to_owned(),
        "output".to_owned(),
        "extra".to_owned(),
    ]);
    if outcome.status() != ExitStatus::Failure {
        return Err("extra manifest positional must fail".to_owned());
    }
    let [diagnostic] = outcome.output() else {
        return Err("extra positional must emit one diagnostic".to_owned());
    };
    let expected = "unexpected positional argument: extra
";
    if diagnostic.text() != expected {
        return Err(format!(
            "unexpected extra-position diagnostic: {:?}",
            diagnostic.text()
        ));
    }
    Ok(())
}

#[test]
fn missing_command_returns_usage_on_stderr() -> Result<(), String> {
    let outcome = PipelineCli.execute(&[]);
    if outcome.status() != ExitStatus::Failure {
        return Err("missing command must fail".to_owned());
    }
    let [chunk] = outcome.output() else {
        return Err("missing command must emit one usage chunk".to_owned());
    };
    if chunk.stream() != OutputStream::Stderr {
        return Err("usage must be written to stderr".to_owned());
    }
    let expected = format!(
        "{USAGE}
"
    );
    if chunk.text() != expected {
        return Err(format!("unexpected usage output: {:?}", chunk.text()));
    }
    Ok(())
}

#[test]
fn unknown_command_returns_name_and_usage() -> Result<(), String> {
    let outcome = PipelineCli.execute(&["unknown".to_owned()]);
    if outcome.status() != ExitStatus::Failure {
        return Err("unknown command must fail".to_owned());
    }
    let [unknown, usage] = outcome.output() else {
        return Err("unknown command must emit diagnostic and usage".to_owned());
    };
    if unknown.text()
        != "unknown command: unknown
"
    {
        return Err(format!(
            "unexpected command diagnostic: {:?}",
            unknown.text()
        ));
    }
    let expected = format!(
        "{USAGE}
"
    );
    if usage.text() != expected {
        return Err(format!("unexpected usage output: {:?}", usage.text()));
    }
    Ok(())
}

#[test]
fn retired_mod_commands_are_not_base_pipeline_commands() {
    for command in [
        "export-lmlm",
        "preview-optional-mods",
        "dry-run-optional-mods",
    ] {
        assert!(!super::is_known_command(command));
        assert!(!USAGE.contains(command));
    }
}

#[test]
fn prepare_unreal_is_a_known_pipeline_command() -> Result<(), String> {
    if !super::is_known_command("prepare-unreal") {
        return Err("prepare-unreal must be recognized by the CLI".to_owned());
    }
    if !USAGE.contains("prepare-unreal") {
        return Err("prepare-unreal must appear in canonical usage".to_owned());
    }
    Ok(())
}

#[test]
fn successful_output_summary_hides_the_physical_root() -> Result<(), String> {
    let private_fragment = "private-workstation-output-summary";
    let root = std::env::temp_dir().join(format!(
        "{private_fragment}-{}",
        std::process::id(),
    ));
    if root.exists() {
        fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    }
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    fs::write(root.join("evidence.bin"), b"evidence")
        .map_err(|error| error.to_string())?;
    let outcome = render_success(PipelineReport::default(), &root);
    fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
    let rendered = outcome
        .output()
        .iter()
        .map(schoenwald_cli::OutputChunk::text)
        .collect::<String>();
    if rendered.contains(private_fragment)
        || !rendered.contains("output: files=1 bytes=8")
    {
        return Err(format!("output summary leaked its root: {rendered}"));
    }
    Ok(())
}
