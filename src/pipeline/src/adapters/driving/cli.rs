// File:
//   - cli.rs
// Path:
//   - src/pipeline/src/adapters/driving/cli.rs
//
// Copyright:
//   - Copyright (c) 2026 Alberto Villa Osorno.
// SPDX-License-Identifier:
//   - MIT
// Confidential:
//   - false
// License-File:
//   - LICENSE
// Path-Rule:
//   - All paths in this header are repository-root relative.
//
// Boundary-Contract:
// - Owns:
//   - Pipeline command names, argument meaning, defaults, and presentation.
// - Must-Not:
//   - Implement process streams, recursive storage traversal, or phase
//   - behavior.
// - Allows:
//   - Compose pipeline use cases with shared CLI and driven provider
//   - adapters.
// - Split-When:
//   - Split when one command family needs an independent inbound adapter.
// - Merge-When:
//   - Another adapter owns the same pipeline command contract.
// - Summary:
//   - Pipeline command-line driving adapter.
// - Description:
//   - Converts process-neutral arguments into pipeline application
//   - invocations.
// - Usage:
//   - Called by the thin binary through shared process composition.
// - Defaults:
//   - Game and extracted roots default to `game` and `extracted`.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - true
//   - Reason: Pipeline command-line driving adapter keeps tightly coupled
//   - validation, ordering, and deterministic transformation invariants
//   - together; split when a stable independently testable sub-boundary is
//   - identified.
//

//! Driving CLI adapter for pipeline application use cases.
//!
//! Shared process mechanisms come from `schoenwald-cli`.
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use schoenwald_cli::{CliProgram, CommandOutcome, run_process};

use self::options::{ParsedArguments, parse_common_arguments};
use crate::adapters::driven::{
    FilesystemOutputInventory, LocalPipeline, RunRegistry, RunStartError,
    check_cancellation, install_progress,
};
use crate::application::{PipelineService, SummarizeOutput};
use crate::domain::{
    PhaseThreePackageSelector, PipelineConfig, PipelineError, PipelineReport,
    StageReport,
};
use crate::ports::FbxExportOptions;

mod options;

/// Complete pipeline command usage text.
const USAGE: &str = concat!(
    "usage: pipeline active | pipeline cancel <run-id|all> | pipeline ",
    "extract-game|extract-game-resume|export-movies|",
    "export-lmlm|manifest-minor-units|",
    "metadata-fill-minor-units|edit-minor-unit-metadata|",
    "index-minor-units|audit-minor-units [game-root] [extracted-root] | ",
    "plan-fbx-package [index-jsonl] [selector] [output-dir] | ",
    "fbx-export-characters [index-jsonl] [output-dir] [base-root] | ",
    "fbx-export-wasp-camera [index-jsonl] [output-dir] [base-root] | ",
    "fbx-export-wrench [index-jsonl] [output-dir] [base-root] | ",
    "fbx-export-props [index-jsonl] [game-root] [output-dir] | ",
    "fbx-export-vehicles [index-jsonl] [game-root] [output-dir] | ",
    "fbx-export-world-props [index-jsonl] [game-root] [output-dir] | ",
    "fbx-export-world [index-jsonl] [game-root] ",
    "[coordinate-p3d-root] [output-dir] | ",
    "fbx-export [index-jsonl] [selector] [output-dir] [base-root] ",
    "[--embed-textures (legacy compatibility)] ",
    "[--verbosity detailed|minimal] ",
    "[--log <path>|--no-log] ",
    "[--run-label <portable-label>] [--allow-concurrent]",
);

/// Pipeline command-line program.
#[derive(Debug, Default, Clone, Copy)]
pub struct PipelineCli;

impl CliProgram for PipelineCli {
    fn execute(
        &self,
        arguments: &[String],
    ) -> CommandOutcome {
        let Some(command) = arguments.first() else {
            return CommandOutcome::failure().stderr_line(USAGE);
        };
        let remaining = arguments
            .get(1..)
            .unwrap_or_default();
        if matches!(
            command.as_str(),
            "active" | "--active"
        ) {
            return run_active_command(remaining);
        }
        if matches!(
            command.as_str(),
            "cancel" | "--cancel"
        ) {
            return run_cancel_command(remaining);
        }
        if !is_known_command(command) {
            return CommandOutcome::failure()
                .stderr_line(format!("unknown command: {command}"))
                .stderr_line(USAGE);
        }
        let parsed = match parse_common_arguments(remaining) {
            Ok(parsed) => parsed,
            Err(error) => {
                return CommandOutcome::failure()
                    .stderr_line(format!("invalid arguments: {error}"))
                    .stderr_line(USAGE);
            }
        };
        if parsed.embed_textures && command != "fbx-export" {
            return CommandOutcome::failure().stderr_line(
                "invalid arguments: --embed-textures requires fbx-export",
            );
        }
        run_registered_command(
            command, &parsed,
        )
    }
}

/// List every active pipeline process with progress and timing evidence.
fn run_active_command(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 0,
    ) {
        return outcome;
    }
    match RunRegistry::current_workspace().active_lines() {
        Ok(lines) if lines.is_empty() => {
            CommandOutcome::success().stdout_line("no active pipeline runs")
        }
        Ok(lines) => lines
            .into_iter()
            .fold(
                CommandOutcome::success(),
                CommandOutcome::stdout_line,
            ),
        Err(error) => CommandOutcome::failure()
            .stderr_line(format!("active-run inspection failed: {error}")),
    }
}

/// Request cooperative cancellation for one run identity or every active run.
fn run_cancel_command(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 1,
    ) {
        return outcome;
    }
    let Some(target) = arguments.first() else {
        return missing_argument("run id or all");
    };
    match RunRegistry::current_workspace().request_cancel(target) {
        Ok(requested) if requested.is_empty() => {
            CommandOutcome::success().stdout_line("no active pipeline runs")
        }
        Ok(requested) => requested
            .into_iter()
            .fold(
                CommandOutcome::success(),
                |outcome, run_id| {
                    outcome.stdout_line(
                        format!("cancellation requested: {run_id}"),
                    )
                },
            ),
        Err(error) => CommandOutcome::failure()
            .stderr_line(format!("cancellation request failed: {error}")),
    }
}

/// Execute one normal command inside a cooperative active-run transaction.
fn run_registered_command(
    command: &str,
    parsed: &ParsedArguments,
) -> CommandOutcome {
    let registry = RunRegistry::current_workspace();
    let guard = match registry.start(
        command,
        parsed.run_label(),
        parsed.run_mode(),
    ) {
        Ok(guard) => guard,
        Err(error) => return render_run_start_error(&error),
    };
    let run_id = guard
        .run_id()
        .to_owned();
    let log_file = parsed.log_file_for_run(&run_id);
    if let Err(error) = install_progress(
        parsed.verbosity,
        log_file.as_deref(),
    ) {
        let cleanup = guard.finish();
        return render_setup_failure(
            &run_id,
            format!("failed to configure progress: {error}"),
            cleanup,
        );
    }
    let outcome = match check_cancellation() {
        Ok(()) => dispatch_known_command(
            command, parsed,
        ),
        Err(error) => CommandOutcome::failure()
            .stderr_line(format!("pipeline failed: {error}")),
    };
    match guard.finish() {
        Ok(()) => outcome.stderr_line(format!("pipeline run: {run_id}")),
        Err(error) => CommandOutcome::failure()
            .stderr_line(format!("pipeline run cleanup failed: {error}"))
            .stderr_line(format!("pipeline run: {run_id}")),
    }
}

/// Dispatch one already validated normal command.
fn dispatch_known_command(
    command: &str,
    parsed: &ParsedArguments,
) -> CommandOutcome {
    if command == "plan-fbx-package" {
        return run_fbx_manifest(&parsed.positionals);
    }
    if command == "fbx-export-characters" {
        return run_character_catalog(&parsed.positionals);
    }
    if command == "fbx-export-wasp-camera" {
        return run_wasp_camera(&parsed.positionals);
    }
    if command == "fbx-export-wrench" {
        return run_wrench(&parsed.positionals);
    }
    if command == "fbx-export-props" {
        return run_prop_catalog(&parsed.positionals);
    }
    if command == "fbx-export-vehicles" {
        return run_vehicle_catalog(&parsed.positionals);
    }
    if command == "fbx-export-world-props" {
        return run_world_prop_catalog(&parsed.positionals);
    }
    if command == "fbx-export-world" {
        return run_world_master(&parsed.positionals);
    }
    if command == "fbx-export" {
        return run_fbx_export(
            &parsed.positionals,
            FbxExportOptions {
                embed_textures: parsed.embed_textures,
            },
        );
    }
    run_pipeline_command(
        command,
        &parsed.positionals,
    )
}

/// Render one blocked start with active-run evidence and safe next commands.
fn render_run_start_error(error: &RunStartError) -> CommandOutcome {
    let mut outcome = CommandOutcome::failure().stderr_line(
        format!(
            "pipeline start blocked: {}",
            error.message()
        ),
    );
    for line in error.active_lines() {
        outcome = outcome.stderr_line(line);
    }
    if !error
        .active_lines()
        .is_empty()
    {
        outcome = outcome
            .stderr_line("inspect active runs: pipeline active")
            .stderr_line("request cancellation: pipeline cancel <run-id>")
            .stderr_line(
                "explicit concurrent execution: add --allow-concurrent",
            );
    }
    outcome
}

/// Render setup failure while preserving any lease-cleanup evidence.
fn render_setup_failure(
    run_id: &str,
    message: String,
    cleanup: Result<(), String>,
) -> CommandOutcome {
    let outcome = CommandOutcome::failure()
        .stderr_line(message)
        .stderr_line(format!("pipeline run: {run_id}"));
    match cleanup {
        Ok(()) => outcome,
        Err(error) => {
            outcome.stderr_line(format!("pipeline run cleanup failed: {error}"))
        }
    }
}

/// Runs the pipeline CLI in the current process.
#[must_use]
pub fn run_env() -> ExitCode {
    run_process(&PipelineCli)
}

/// Returns whether one exact command name is supported.
fn is_known_command(command: &str) -> bool {
    matches!(
        command,
        "extract-game"
            | "extract-game-resume"
            | "export-movies"
            | "export-lmlm"
            | "manifest-minor-units"
            | "metadata-fill-minor-units"
            | "edit-minor-unit-metadata"
            | "index-minor-units"
            | "audit-minor-units"
            | "plan-fbx-package"
            | "fbx-export-characters"
            | "fbx-export-wasp-camera"
            | "fbx-export-wrench"
            | "fbx-export-props"
            | "fbx-export-vehicles"
            | "fbx-export-world-props"
            | "fbx-export-world"
            | "fbx-export"
    )
}

/// Rejects the first positional argument beyond one command's contract.
fn reject_extra_positionals(
    arguments: &[String],
    maximum: usize,
) -> Option<CommandOutcome> {
    let argument = arguments.get(maximum)?;
    Some(
        CommandOutcome::failure()
            .stderr_line(format!("unexpected positional argument: {argument}")),
    )
}

/// Runs one command that uses the standard game and extracted roots.
fn run_pipeline_command(
    command: &str,
    arguments: &[String],
) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 2,
    ) {
        return outcome;
    }
    let game_root = arguments
        .first()
        .map_or_else(
            || PathBuf::from("game"),
            PathBuf::from,
        );
    let extracted_root = arguments
        .get(1)
        .map_or_else(
            || PathBuf::from("extracted"),
            PathBuf::from,
        );
    let summary_root = extracted_root.clone();
    let config = PipelineConfig {
        game_root,
        extracted_root,
        clean_extracted: command == "extract-game",
    };
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = match command {
        "export-movies" => application.export_movies(&config),
        "export-lmlm" => application.export_lmlm(&config),
        "manifest-minor-units" => one_stage(
            application.manifest_minor_units(
                &config.game_root,
                &config.extracted_root,
            ),
        ),
        "metadata-fill-minor-units" => one_stage(
            application.fill_minor_unit_metadata(&config.extracted_root),
        ),
        "edit-minor-unit-metadata" => one_stage(
            application.edit_minor_unit_metadata(&config.extracted_root),
        ),
        "index-minor-units" => {
            one_stage(application.index_minor_units(&config.extracted_root))
        }
        "audit-minor-units" => {
            one_stage(application.audit_minor_units(&config.extracted_root))
        }
        _ => application.run(&config),
    };
    render_result(
        result,
        &summary_root,
    )
}

/// Runs the phase-three FBX manifest planning command.
fn run_fbx_manifest(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 3,
    ) {
        return outcome;
    }
    let Some(index_path) = arguments.first() else {
        return missing_argument("package index path");
    };
    let Some(raw_selector) = arguments.get(1) else {
        return missing_argument("package selector");
    };
    let Some(output_dir) = arguments.get(2) else {
        return missing_argument("output directory");
    };
    let selector = match PhaseThreePackageSelector::parse(raw_selector) {
        Ok(selector) => selector,
        Err(error) => {
            return CommandOutcome::failure()
                .stderr_line(format!("invalid selector: {error}"));
        }
    };
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = one_stage(
        application.write_fbx_manifest(
            Path::new(index_path),
            &selector,
            Path::new(output_dir),
        ),
    );
    render_result(
        result,
        Path::new(output_dir),
    )
}

/// Runs the complete package-index-driven character FBX catalog export.
fn run_character_catalog(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 3,
    ) {
        return outcome;
    }
    let Some(index_path) = arguments.first() else {
        return missing_argument("package index path");
    };
    let Some(output_dir) = arguments.get(1) else {
        return missing_argument("output directory");
    };
    let base_root = arguments
        .get(2)
        .map_or(
            ".",
            String::as_str,
        );
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = one_stage(
        application.export_character_catalog(
            Path::new(index_path),
            Path::new(output_dir),
            Path::new(base_root),
        ),
    );
    render_result(
        result,
        Path::new(output_dir),
    )
}

/// Runs the canonical standalone Wasp Camera FBX export.
fn run_wasp_camera(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 3,
    ) {
        return outcome;
    }
    let Some(index_path) = arguments.first() else {
        return missing_argument("package index path");
    };
    let Some(output_dir) = arguments.get(1) else {
        return missing_argument("output directory");
    };
    let base_root = arguments
        .get(2)
        .map_or(
            ".",
            String::as_str,
        );
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = one_stage(
        application.export_wasp_camera(
            Path::new(index_path),
            Path::new(output_dir),
            Path::new(base_root),
        ),
    );
    render_result(
        result,
        Path::new(output_dir),
    )
}

/// Runs the canonical standalone Wrench model FBX export.
fn run_wrench(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 3,
    ) {
        return outcome;
    }
    let Some(index_path) = arguments.first() else {
        return missing_argument("package index path");
    };
    let Some(output_dir) = arguments.get(1) else {
        return missing_argument("output directory");
    };
    let base_root = arguments
        .get(2)
        .map_or(
            ".",
            String::as_str,
        );
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = one_stage(
        application.export_wrench(
            Path::new(index_path),
            Path::new(output_dir),
            Path::new(base_root),
        ),
    );
    render_result(
        result,
        Path::new(output_dir),
    )
}

/// Runs the complete non-world card and mission prop catalog export.
fn run_prop_catalog(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 3,
    ) {
        return outcome;
    }
    let Some(index_path) = arguments.first() else {
        return missing_argument("package index path");
    };
    let Some(game_root) = arguments.get(1) else {
        return missing_argument("game root");
    };
    let Some(output_dir) = arguments.get(2) else {
        return missing_argument("output directory");
    };
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = one_stage(
        application.export_prop_catalog(
            Path::new(index_path),
            Path::new(game_root),
            Path::new(output_dir),
        ),
    );
    render_result(
        result,
        Path::new(output_dir),
    )
}

/// Run the complete standalone world-prop FBX catalog command.
fn run_world_prop_catalog(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 3,
    ) {
        return outcome;
    }
    let Some(index_path) = arguments.first() else {
        return missing_argument("package index path");
    };
    let Some(game_root) = arguments.get(1) else {
        return missing_argument("game root");
    };
    let Some(output_dir) = arguments.get(2) else {
        return missing_argument("output directory");
    };
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = one_stage(
        application.export_world_prop_catalog(
            Path::new(index_path),
            Path::new(game_root),
            Path::new(output_dir),
        ),
    );
    render_result(
        result,
        Path::new(output_dir),
    )
}

/// Run the complete semantically separated vehicle FBX catalog export.
fn run_vehicle_catalog(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 3,
    ) {
        return outcome;
    }
    let Some(index_path) = arguments.first() else {
        return missing_argument("package index path");
    };
    let Some(game_root) = arguments.get(1) else {
        return missing_argument("game root");
    };
    let Some(output_dir) = arguments.get(2) else {
        return missing_argument("output directory");
    };
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = one_stage(
        application.export_vehicle_catalog(
            Path::new(index_path),
            Path::new(game_root),
            Path::new(output_dir),
        ),
    );
    render_result(
        result,
        Path::new(output_dir),
    )
}

/// Run the complete separated master-world analysis export.
fn run_world_master(arguments: &[String]) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 4,
    ) {
        return outcome;
    }
    let Some(index_path) = arguments.first() else {
        return missing_argument("package index path");
    };
    let Some(game_root) = arguments.get(1) else {
        return missing_argument("game root");
    };
    let Some(coordinate_root) = arguments.get(2) else {
        return missing_argument("coordinate P3D root");
    };
    let Some(output_dir) = arguments.get(3) else {
        return missing_argument("output directory");
    };
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = one_stage(
        application.export_world_master(
            Path::new(index_path),
            Path::new(game_root),
            Path::new(coordinate_root),
            Path::new(output_dir),
        ),
    );
    render_result(
        result,
        Path::new(output_dir),
    )
}

/// Runs the phase-three package-driven FBX export command.
fn run_fbx_export(
    arguments: &[String],
    options: FbxExportOptions,
) -> CommandOutcome {
    if let Some(outcome) = reject_extra_positionals(
        arguments, 4,
    ) {
        return outcome;
    }
    let Some(index_path) = arguments.first() else {
        return missing_argument("package index path");
    };
    let Some(raw_selector) = arguments.get(1) else {
        return missing_argument("package selector");
    };
    let Some(output_dir) = arguments.get(2) else {
        return missing_argument("output directory");
    };
    let base_root = arguments
        .get(3)
        .map_or(
            ".",
            String::as_str,
        );
    let selector = match PhaseThreePackageSelector::parse(raw_selector) {
        Ok(selector) => selector,
        Err(error) => {
            return CommandOutcome::failure()
                .stderr_line(format!("invalid selector: {error}"));
        }
    };
    let provider = LocalPipeline;
    let application = PipelineService::new(&provider);
    let result = one_stage(
        application.export_fbx_package(
            Path::new(index_path),
            &selector,
            Path::new(output_dir),
            Path::new(base_root),
            options,
        ),
    );
    render_result(
        result,
        Path::new(output_dir),
    )
}

/// Returns a failed missing-argument outcome with usage.
fn missing_argument(name: &str) -> CommandOutcome {
    CommandOutcome::failure()
        .stderr_line(format!("missing {name}"))
        .stderr_line(USAGE)
}

/// Wraps one stage result as a complete pipeline report.
fn one_stage(
    result: Result<StageReport, PipelineError>
) -> Result<PipelineReport, PipelineError> {
    result.map(
        |stage| PipelineReport {
            stages: vec![stage],
        },
    )
}

/// Renders one pipeline result and optional output inventory.
fn render_result(
    result: Result<PipelineReport, PipelineError>,
    output_root: &Path,
) -> CommandOutcome {
    match result {
        Ok(report) => render_success(
            report,
            output_root,
        ),
        Err(error) => CommandOutcome::failure()
            .stderr_line(format!("pipeline failed: {error}")),
    }
}

/// Renders one successful pipeline report and output summary.
fn render_success(
    report: PipelineReport,
    output_root: &Path,
) -> CommandOutcome {
    let mut outcome = CommandOutcome::success().stderr_line(
        format!(
            "pipeline completed: {} stages",
            report
                .stages
                .len()
        ),
    );
    for stage in report.stages {
        outcome = outcome.stderr_line(
            format!(
                "{}: files={} bytes={} note={}",
                stage.name, stage.files, stage.bytes, stage.note
            ),
        );
    }
    if let Ok(summary) = SummarizeOutput::execute(
        &FilesystemOutputInventory,
        output_root,
    ) {
        outcome = outcome.stderr_line(
            format!(
                "output: {} files={} bytes={}",
                summary
                    .root
                    .display(),
                summary.files,
                summary.bytes
            ),
        );
        for directory in summary.directories {
            outcome = outcome.stderr_line(
                format!(
                    "output/{}: files={}",
                    directory.name, directory.files
                ),
            );
        }
    }
    outcome
}

#[cfg(test)]
mod tests {
    use schoenwald_cli::{CliProgram, ExitStatus, OutputStream};

    use super::{PipelineCli, USAGE};

    #[test]
    fn manifest_rejects_extra_positionals() -> Result<(), String> {
        let outcome = super::run_fbx_manifest(
            &[
                "missing-index.jsonl".to_owned(),
                "type:model".to_owned(),
                "output".to_owned(),
                "extra".to_owned(),
            ],
        );
        if outcome.status() != ExitStatus::Failure {
            return Err("extra manifest positional must fail".to_owned());
        }
        let [diagnostic] = outcome.output() else {
            return Err("extra positional must emit one diagnostic".to_owned());
        };
        let expected = "unexpected positional argument: extra
";
        if diagnostic.text() != expected {
            return Err(
                format!(
                    "unexpected extra-position diagnostic: {:?}",
                    diagnostic.text()
                ),
            );
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
            return Err(
                format!(
                    "unexpected usage output: {:?}",
                    chunk.text()
                ),
            );
        }
        Ok(())
    }

    #[test]
    fn unknown_command_returns_name_and_usage() -> Result<(), String> {
        let outcome = PipelineCli.execute(&["unknown".to_owned()]);
        if outcome.status() != ExitStatus::Failure {
            return Err("unknown command must fail".to_owned());
        }
        let [
            unknown,
            usage,
        ] = outcome.output()
        else {
            return Err(
                "unknown command must emit diagnostic and usage".to_owned(),
            );
        };
        if unknown.text()
            != "unknown command: unknown
"
        {
            return Err(
                format!(
                    "unexpected command diagnostic: {:?}",
                    unknown.text()
                ),
            );
        }
        let expected = format!(
            "{USAGE}
"
        );
        if usage.text() != expected {
            return Err(
                format!(
                    "unexpected usage output: {:?}",
                    usage.text()
                ),
            );
        }
        Ok(())
    }
}
