// File:
//   - options.rs
// Path:
//   - src/pipeline/src/adapters/driving/cli/options.rs
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
//   - Common pipeline verbosity and diagnostic-log command options.
// - Must-Not:
//   - Execute pipeline stages or infer machine-specific filesystem roots.
// - Allows:
//   - Separate process options from command-specific positional arguments.
// - Split-When:
//   - Split when another option family gains independent parsing policy.
// - Merge-When:
//   - The pipeline CLI can own these options without obscuring commands.
// - Summary:
//   - Parses portable pipeline progress and logging options.
// - Description:
//   - Resolves detailed or minimal progress, local log selection, and
//   - untouched positional arguments for the driving adapter.
// - Usage:
//   - Called once after the command name is validated.
// - Defaults:
//   - Detailed progress and `logs/pipeline/latest.jsonl` logging are enabled.
//
// ADRs:
// - docs/adr/pipeline/orchestration-cli-and-language-boundaries.md
//
// Large file:
//   - false
//

//! Parses portable progress and diagnostic-log options for pipeline commands.
use std::path::{Path, PathBuf};

use schoenwald_filesystem::resolve_under;

use crate::adapters::driven::ProgressVerbosity;

/// Stable latest-run log relative to the pipeline working directory.
const DEFAULT_LOG_FILE: &str = "logs/pipeline/latest.jsonl";

/// Common process options separated from command-specific positionals.
#[derive(Debug, Eq, PartialEq)]
pub(super) struct ParsedArguments {
    /// Command-specific positional values in their original order.
    pub(super) positionals: Vec<String>,
    /// Selected terminal detail level.
    pub(super) verbosity: ProgressVerbosity,
    /// Selected latest-run log, or `None` when logging is disabled.
    pub(super) log_file: Option<PathBuf>,
    /// Whether `fbx-export` should emit the experimental unsupported
    /// Blender helper.
    pub(super) blender_helper: bool,
    /// Whether `fbx-export` should emit the optional Maya import script.
    pub(super) maya: bool,
    /// Whether `fbx-export` should embed compatibility texture payloads.
    pub(super) embed_textures: bool,
}

impl Default for ParsedArguments {
    fn default() -> Self {
        Self {
            positionals: Vec::new(),
            verbosity: ProgressVerbosity::Detailed,
            log_file: Some(PathBuf::from(DEFAULT_LOG_FILE)),
            blender_helper: false,
            maya: false,
            embed_textures: false,
        }
    }
}

/// Select one explicit verbosity without order-dependent overrides.
fn select_verbosity(
    verbosity: &mut ProgressVerbosity,
    selected: &mut bool,
    value: ProgressVerbosity,
) -> Result<(), String> {
    if *selected {
        return Err(String::from("verbosity may be specified only once"));
    }
    *verbosity = value;
    *selected = true;
    Ok(())
}

/// One parsed logging selector and its consumed argument count.
struct ParsedLogOption {
    /// Selected log destination, or `None` when logging is disabled.
    log_file: Option<PathBuf>,
    /// Number of command arguments consumed by this selector.
    consumed: usize,
}

/// Parse one logging selector without applying order-dependent overrides.
fn parse_log_option(
    arguments: &[String],
    index: usize,
    argument: &str,
) -> Result<Option<ParsedLogOption>, String> {
    if argument == "--log" {
        let value = arguments
            .get(index.saturating_add(1))
            .ok_or_else(|| String::from("--log requires a path"))?;
        if value.starts_with('-') {
            return Err(format!("--log requires a path before {value}"));
        }
        return Ok(
            Some(
                ParsedLogOption {
                    log_file: Some(parse_log_path(value)?),
                    consumed: 2,
                },
            ),
        );
    }
    if argument == "--no-log" {
        return Ok(
            Some(
                ParsedLogOption {
                    log_file: None,
                    consumed: 1,
                },
            ),
        );
    }
    let Some(value) = argument.strip_prefix("--log=") else {
        return Ok(None);
    };
    Ok(
        Some(
            ParsedLogOption {
                log_file: Some(parse_log_path(value)?),
                consumed: 1,
            },
        ),
    )
}

/// Select one explicit logging mode without order-dependent overrides.
fn select_logging(
    log_file: &mut Option<PathBuf>,
    selected: &mut bool,
    value: Option<PathBuf>,
) -> Result<(), String> {
    if *selected {
        return Err(String::from("logging may be specified only once"));
    }
    *log_file = value;
    *selected = true;
    Ok(())
}

/// Parse progress and logging options without changing positional paths.
///
/// # Errors
///
/// Returns an error for unknown options, missing values, empty log paths, or
/// unsupported verbosity names.
pub(super) fn parse_common_arguments(
    arguments: &[String]
) -> Result<ParsedArguments, String> {
    let mut parsed = ParsedArguments::default();
    let mut verbosity_selected = false;
    let mut logging_selected = false;
    let mut parse_options = true;
    let mut index = 0usize;
    while let Some(argument) = arguments.get(index) {
        if parse_options && argument == "--" {
            parse_options = false;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument == "--verbosity" {
            let value = arguments
                .get(index.saturating_add(1))
                .ok_or_else(|| String::from("--verbosity requires a value"))?;
            if value.starts_with('-') {
                return Err(
                    format!("--verbosity requires a value before {value}"),
                );
            }
            select_verbosity(
                &mut parsed.verbosity,
                &mut verbosity_selected,
                parse_verbosity(value)?,
            )?;
            index = index.saturating_add(2);
            continue;
        }
        if parse_options {
            let parsed_log_option = parse_log_option(
                arguments, index, argument,
            )?;
            if let Some(log_option) = parsed_log_option {
                select_logging(
                    &mut parsed.log_file,
                    &mut logging_selected,
                    log_option.log_file,
                )?;
                index = index.saturating_add(log_option.consumed);
                continue;
            }
        }
        if parse_options && argument == "--minimal" {
            select_verbosity(
                &mut parsed.verbosity,
                &mut verbosity_selected,
                ProgressVerbosity::Minimal,
            )?;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument == "--detailed" {
            select_verbosity(
                &mut parsed.verbosity,
                &mut verbosity_selected,
                ProgressVerbosity::Detailed,
            )?;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument == "--blender-helper" {
            parsed.blender_helper = true;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument == "--maya" {
            parsed.maya = true;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument == "--embed-textures" {
            parsed.embed_textures = true;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options
            && let Some(value) = argument.strip_prefix("--verbosity=")
        {
            select_verbosity(
                &mut parsed.verbosity,
                &mut verbosity_selected,
                parse_verbosity(value)?,
            )?;
            index = index.saturating_add(1);
            continue;
        }
        if parse_options && argument.starts_with('-') {
            return Err(format!("unknown option: {argument}"));
        }
        parsed
            .positionals
            .push(argument.clone());
        index = index.saturating_add(1);
    }
    Ok(parsed)
}

/// Parse one exact supported verbosity value.
fn parse_verbosity(value: &str) -> Result<ProgressVerbosity, String> {
    match value {
        "detailed" => Ok(ProgressVerbosity::Detailed),
        "minimal" => Ok(ProgressVerbosity::Minimal),
        _ => Err(
            format!(
                "unsupported verbosity {value:?}; expected detailed or minimal"
            ),
        ),
    }
}

/// Parse one nonempty portable log path.
fn parse_log_path(value: &str) -> Result<PathBuf, String> {
    if value.is_empty() {
        return Err(String::from("--log path must not be empty"));
    }
    if value
        .trim()
        .is_empty()
    {
        return Err(String::from("--log path must not be blank"));
    }
    if value.ends_with('/') || value.ends_with(char::from(92)) {
        return Err(String::from("--log path must identify a file"));
    }
    let validation = resolve_under(
        Path::new("."),
        Path::new(value),
    );
    let validated = match validation {
        Ok(path) => path,
        Err(error) => {
            return Err(format!("invalid --log path {value:?}: {error}"));
        }
    };
    let Ok(relative) = validated.strip_prefix(Path::new(".")) else {
        return Err(String::from("failed to normalize --log path"));
    };
    Ok(relative.to_path_buf())
}

#[cfg(test)]
#[path = "options_tests.rs"]
mod tests;
